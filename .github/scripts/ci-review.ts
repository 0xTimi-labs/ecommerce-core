import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';

// ==========================================
// 1. 类型定义
// ==========================================

export enum ReviewState {
  IDLE = 'IDLE',
  REVIEWING = 'REVIEWING',
  APPROVED = 'APPROVED',
  CHANGES_REQUESTED = 'CHANGES_REQUESTED',
}

export type ReviewMode = 'FRESH' | 'CONTINUE';

export interface StateTransitionResult {
  nextState: ReviewState;
  labelsToAdd: string[];
  labelsToRemove: string[];
}

export interface ReviewStrategy {
  templatePath: string;
  getPiArgs: (hasSession: boolean) => string[];
}

// ==========================================
// 2. 纯函数状态机与策略表
// ==========================================

export const REVIEW_STRATEGIES: Record<ReviewMode, ReviewStrategy> = {
  FRESH: {
    templatePath: '.github/templates/prompts/initial_review.md',
    getPiArgs: () => [],
  },
  CONTINUE: {
    templatePath: '.github/templates/prompts/continue_review.md',
    getPiArgs: (hasSession: boolean) => (hasSession ? ['-c'] : []),
  },
};

export function transition(
  currentState: ReviewState,
  event: { type: 'START_REVIEW' } | { type: 'EVALUATE_AI_VERDICT'; currentLabels: string[] }
): StateTransitionResult {
  switch (event.type) {
    case 'START_REVIEW':
      return {
        nextState: ReviewState.REVIEWING,
        labelsToAdd: ['review/in-progress'],
        labelsToRemove: ['review/approved', 'review/changes-requested'],
      };

    case 'EVALUATE_AI_VERDICT': {
      const hasChangesRequested = event.currentLabels.includes('review/changes-requested');

      if (hasChangesRequested) {
        return {
          nextState: ReviewState.CHANGES_REQUESTED,
          labelsToAdd: [],
          labelsToRemove: ['review/in-progress', 'review/approved'],
        };
      }

      return {
        nextState: ReviewState.APPROVED,
        labelsToAdd: ['review/approved'],
        labelsToRemove: ['review/in-progress', 'review/changes-requested'],
      };
    }
  }
}

export function resolveReviewMode(eventName: string, commentBody: string): ReviewMode {
  if (eventName === 'issue_comment' && /^\/review\s+(-c|--continue)/.test(commentBody)) {
    return 'CONTINUE';
  }
  return 'FRESH';
}

export function renderPrompt(
  templatePath: string,
  variables: Record<string, string>
): string {
  let content = readFileSync(templatePath, 'utf-8');
  for (const [key, value] of Object.entries(variables)) {
    const placeholder = new RegExp(`\\{\\{${key}\\}\\}`, 'g');
    content = content.replace(placeholder, value);
  }
  return content;
}

// ==========================================
// 3. GitHub CLI 客户端
// ==========================================

export class GitHubClient {
  static getPrLabels(prNumber: string): string[] {
    const proc = Bun.spawnSync([
      'gh', 'pr', 'view', prNumber,
      '--json', 'labels',
      '--jq', '.labels[].name'
    ]);
    if (proc.exitCode !== 0) {
      console.warn(`[GitHubClient] Warning: Failed to query PR labels: ${proc.stderr.toString()}`);
      return [];
    }
    return proc.stdout.toString().trim().split('\n').map((l) => l.trim()).filter(Boolean);
  }

  static createPlaceholderComment(repo: string, prNumber: string): string {
    const proc = Bun.spawnSync([
      'gh', 'api', `repos/${repo}/issues/${prNumber}/comments`,
      '-X', 'POST',
      '-f', 'body=AI 评审中...',
      '--jq', '.id'
    ]);
    if (proc.exitCode !== 0) {
      throw new Error(`[GitHubClient] Failed to create placeholder comment: ${proc.stderr.toString()}`);
    }
    return proc.stdout.toString().trim();
  }

  static syncLabels(prNumber: string, toAdd: string[], toRemove: string[]) {
    for (const label of toRemove) {
      console.log(`[GitHubClient] Removing label: ${label}`);
      Bun.spawnSync(['gh', 'pr', 'edit', prNumber, '--remove-label', label]);
    }
    for (const label of toAdd) {
      console.log(`[GitHubClient] Adding label: ${label}`);
      Bun.spawnSync(['gh', 'pr', 'edit', prNumber, '--add-label', label]);
    }
  }
}

// ==========================================
// 4. 主流程编排
// ==========================================

export async function runReview() {
  const deepseekApiKey = process.env.DEEPSEEK_API_KEY;
  if (!deepseekApiKey) {
    console.log('[CI Review] DEEPSEEK_API_KEY not configured. Skipping review.');
    return;
  }

  const prNumber = process.env.PR_NUMBER;
  const repo = process.env.REPO || process.env.GITHUB_REPOSITORY;
  const eventName = process.env.EVENT_NAME || '';
  const commentBody = process.env.COMMENT_BODY || '';
  const sessionDir = join(process.cwd(), '.pi_session');

  if (!prNumber || !repo) {
    throw new Error('[CI Review] Missing PR_NUMBER or REPO environment variables.');
  }

  console.log(`[CI Review] Starting Review for PR #${prNumber} in ${repo} (Event: ${eventName})`);

  // 1. 获取当前 PR 标签，选择审查角色与规范
  const initialLabels = GitHubClient.getPrLabels(prNumber);
  const isArchitecturePr = initialLabels.includes('type/architecture');
  const skillFile = isArchitecturePr
    ? '.agents/skills/artifact-reviewer/SKILL.md'
    : '.agents/skills/code-reviewer/SKILL.md';
  const role = isArchitecturePr ? '架构与契约审查员' : 'Feature 代码审查员';

  // 2. 状态机：进入 REVIEWING 状态，打上 review/in-progress 并清理旧终态标
  let state = ReviewState.IDLE;
  const startTransition = transition(state, { type: 'START_REVIEW' });
  state = startTransition.nextState;
  GitHubClient.syncLabels(prNumber, startTransition.labelsToAdd, startTransition.labelsToRemove);

  // 3. 发布极简占位评论
  const commentId = GitHubClient.createPlaceholderComment(repo, prNumber);
  console.log(`[CI Review] Created placeholder comment ID: ${commentId}`);

  // 4. 解析模式与模板策略
  const mode = resolveReviewMode(eventName, commentBody);
  const strategy = REVIEW_STRATEGIES[mode];
  const hasValidSession = existsSync(sessionDir) && readdirSync(sessionDir).length > 0;

  if (mode === 'FRESH' || !hasValidSession) {
    rmSync(sessionDir, { recursive: true, force: true });
    mkdirSync(sessionDir, { recursive: true });
  }

  const dynamicPiArgs = strategy.getPiArgs(hasValidSession);
  console.log(`[CI Review] Selected mode: ${mode}, template: ${strategy.templatePath}, session reuse: ${hasValidSession}`);

  // 5. 渲染 Prompt 模板
  const prompt = renderPrompt(strategy.templatePath, {
    role,
    skill_file: skillFile,
    pr_number: prNumber,
    comment_id: commentId,
    repo,
  });

  console.log(`[CI Review] Running Pi CLI with model deepseek/deepseek-v4-flash...`);

  // 6. 执行 Pi CLI 进行 Tool Calling 深度审查
  const piArgs = [
    'pi',
    '-p', prompt,
    '--session-dir', sessionDir,
    ...dynamicPiArgs,
    '--model', 'deepseek/deepseek-v4-flash',
    '--thinking', 'max',
  ];

  const piProc = Bun.spawnSync(piArgs, {
    stdout: 'inherit',
    stderr: 'inherit',
    env: {
      ...process.env,
      PATH: `${process.env.HOME}/.cargo/bin:${process.env.HOME}/.local/bin:${process.env.PATH}`,
    },
  });

  if (piProc.exitCode !== 0) {
    console.warn(`[CI Review] Pi CLI exited with code ${piProc.exitCode}`);
  }

  // 7. 终态收敛：检测 AI 结果标签，状态机流转并自动清理 review/in-progress
  const postReviewLabels = GitHubClient.getPrLabels(prNumber);
  console.log(`[CI Review] Post-review labels: ${JSON.stringify(postReviewLabels)}`);

  const endTransition = transition(state, {
    type: 'EVALUATE_AI_VERDICT',
    currentLabels: postReviewLabels,
  });
  state = endTransition.nextState;

  console.log(`[CI Review] Final Review State: ${state}`);
  GitHubClient.syncLabels(prNumber, endTransition.labelsToAdd, endTransition.labelsToRemove);
}

if (import.meta.main) {
  runReview().catch((err) => {
    console.error(`[CI Review] Fatal error:`, err);
    process.exit(1);
  });
}
