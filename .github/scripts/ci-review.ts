import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';

// ==========================================
// 1. 类型定义
// ==========================================

export type ReviewMode = 'FRESH' | 'CONTINUE';

export interface ReviewStrategy {
  templatePath: string;
  getPiArgs: (hasSession: boolean) => string[];
}

export const STATUS_CONTEXT = 'AI Review Gate';

// ==========================================
// 2. 策略表与纯函数
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

export function parseVerdictFromReport(reportText: string): { hasP0: boolean; p0Count: number } {
  // 匹配类似 "- P0: 0" 或 "P0: 2" 等标准报告头部
  const p0Match = reportText.match(/P0:\s*(\d+)/i);
  if (p0Match) {
    const count = parseInt(p0Match[1], 10);
    return { hasP0: count > 0, p0Count: count };
  }
  // 兜底：若文中显式出现未修复的 P0 阻断描述
  if (/\[P0\]/i.test(reportText)) {
    return { hasP0: true, p0Count: 1 };
  }
  return { hasP0: false, p0Count: 0 };
}

// ==========================================
// 3. GitHub CLI / Status API 客户端
// ==========================================

export class GitHubClient {
  static getHeadSha(prNumber: string): string {
    const proc = Bun.spawnSync([
      'gh', 'pr', 'view', prNumber,
      '--json', 'headRefOid',
      '--jq', '.headRefOid'
    ]);
    if (proc.exitCode !== 0) {
      throw new Error(`[GitHubClient] Failed to get HEAD SHA for PR #${prNumber}: ${proc.stderr.toString()}`);
    }
    const sha = proc.stdout.toString().trim();
    if (!sha) {
      throw new Error(`[GitHubClient] Empty HEAD SHA returned for PR #${prNumber}`);
    }
    return sha;
  }

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

  static setCommitStatus(
    repo: string,
    sha: string,
    state: 'pending' | 'success' | 'failure',
    description: string,
    targetUrl?: string
  ) {
    console.log(`[GitHubClient] Setting Commit Status for ${sha.substring(0, 8)} -> ${state} (${description})`);
    const args = [
      'gh', 'api', `repos/${repo}/statuses/${sha}`,
      '-X', 'POST',
      '-f', `state=${state}`,
      '-f', `context=${STATUS_CONTEXT}`,
      '-f', `description=${description.substring(0, 140)}`
    ];
    if (targetUrl) {
      args.push('-f', `target_url=${targetUrl}`);
    }
    const proc = Bun.spawnSync(args);
    if (proc.exitCode !== 0) {
      console.warn(`[GitHubClient] Warning: Failed to set Commit Status: ${proc.stderr.toString()}`);
    }
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

  static getCommentBody(repo: string, commentId: string): string {
    const proc = Bun.spawnSync([
      'gh', 'api', `repos/${repo}/issues/comments/${commentId}`,
      '--jq', '.body'
    ]);
    if (proc.exitCode !== 0) {
      console.warn(`[GitHubClient] Warning: Failed to fetch comment body: ${proc.stderr.toString()}`);
      return '';
    }
    return proc.stdout.toString().trim();
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

  // 1. 获取 PR 的 HEAD Commit SHA
  const headSha = GitHubClient.getHeadSha(prNumber);
  const prUrl = `https://github.com/${repo}/pull/${prNumber}`;

  // 2. 核心门禁锁：立即向 Commit Status API 发送 Pending，死锁 Merge 按钮与 Merge Queue
  GitHubClient.setCommitStatus(repo, headSha, 'pending', 'AI 正在深度审查代码与架构规范...', prUrl);

  // 3. 获取当前 PR 属性，选择审查角色与规范
  const initialLabels = GitHubClient.getPrLabels(prNumber);
  const isArchitecturePr = initialLabels.includes('type/architecture');
  const skillFile = isArchitecturePr
    ? '.agents/skills/artifact-reviewer/SKILL.md'
    : '.agents/skills/code-reviewer/SKILL.md';
  const role = isArchitecturePr ? '架构与契约审查员' : 'Feature 代码审查员';

  // 4. 发布极简占位评论
  const commentId = GitHubClient.createPlaceholderComment(repo, prNumber);
  console.log(`[CI Review] Created placeholder comment ID: ${commentId}`);

  // 5. 解析模式与模板策略
  const mode = resolveReviewMode(eventName, commentBody);
  const strategy = REVIEW_STRATEGIES[mode];
  const hasValidSession = existsSync(sessionDir) && readdirSync(sessionDir).length > 0;

  if (mode === 'FRESH' || !hasValidSession) {
    rmSync(sessionDir, { recursive: true, force: true });
    mkdirSync(sessionDir, { recursive: true });
  }

  const dynamicPiArgs = strategy.getPiArgs(hasValidSession);
  console.log(`[CI Review] Selected mode: ${mode}, template: ${strategy.templatePath}, session reuse: ${hasValidSession}`);

  // 6. 渲染 Prompt 模板
  const prompt = renderPrompt(strategy.templatePath, {
    role,
    skill_file: skillFile,
    pr_number: prNumber,
    comment_id: commentId,
    repo,
  });

  console.log(`[CI Review] Running Pi CLI with model deepseek/deepseek-v4-flash...`);

  // 7. 执行 Pi CLI 进行 Tool Calling 深度审查
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

  // 8. 读取最终覆写的审查报告并进行硬性门禁判定
  const latestReport = GitHubClient.getCommentBody(repo, commentId);
  const verdict = parseVerdictFromReport(latestReport);
  console.log(`[CI Review] Final Verdict -> hasP0: ${verdict.hasP0}, p0Count: ${verdict.p0Count}`);

  if (verdict.hasP0) {
    // 发现阻断级 P0：向 Commit Status 回传 failure，物理锁死合并！
    GitHubClient.setCommitStatus(
      repo,
      headSha,
      'failure',
      `拦截: 发现 ${verdict.p0Count} 项 P0 阻断缺陷，请修复后复核`,
      prUrl
    );
    console.error(`[CI Review] Hard Gate BLOCKED: ${verdict.p0Count} P0 blockers detected.`);
    process.exit(1);
  } else {
    // 审查通过（0 项 P0）：向 Commit Status 回传 success，绿灯放行合并！
    GitHubClient.setCommitStatus(
      repo,
      headSha,
      'success',
      '通过: AI 审查通过 (0 项 P0 阻断)',
      prUrl
    );
    console.log(`[CI Review] Hard Gate PASSED: No P0 blockers.`);
    process.exit(0);
  }
}

if (import.meta.main) {
  runReview().catch((err) => {
    console.error(`[CI Review] Fatal error:`, err);
    process.exit(1);
  });
}
