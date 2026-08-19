import { describe, expect, it } from 'bun:test';
import {
  ReviewState,
  renderPromptTemplate,
  transition,
} from './ci-review-orchestrator';

describe('Review Orchestrator State Machine', () => {
  it('should transition to REVIEWING and clean legacy verdict labels on START_REVIEW', () => {
    const res = transition(ReviewState.IDLE, { type: 'START_REVIEW' });
    expect(res.nextState).toBe(ReviewState.REVIEWING);
    expect(res.labelsToRemove).toContain('review/approved');
    expect(res.labelsToRemove).toContain('review/changes-requested');
    expect(res.labelsToAdd).toHaveLength(0);
  });

  it('should transition to APPROVED and clean review/ready when approved label is present', () => {
    const res = transition(ReviewState.REVIEWING, {
      type: 'EVALUATE_AI_VERDICT',
      currentLabels: ['type/architecture', 'review/approved', 'review/ready'],
    });
    expect(res.nextState).toBe(ReviewState.APPROVED);
    expect(res.labelsToRemove).toContain('review/ready');
    expect(res.labelsToRemove).toContain('review/changes-requested');
  });

  it('should transition to CHANGES_REQUESTED and clean review/ready when changes-requested label is present', () => {
    const res = transition(ReviewState.REVIEWING, {
      type: 'EVALUATE_AI_VERDICT',
      currentLabels: ['type/architecture', 'review/changes-requested', 'review/ready'],
    });
    expect(res.nextState).toBe(ReviewState.CHANGES_REQUESTED);
    expect(res.labelsToRemove).toContain('review/ready');
    expect(res.labelsToRemove).toContain('review/approved');
  });

  it('should retain current state if no verdict label is detected', () => {
    const res = transition(ReviewState.REVIEWING, {
      type: 'EVALUATE_AI_VERDICT',
      currentLabels: ['type/architecture', 'review/ready'],
    });
    expect(res.nextState).toBe(ReviewState.REVIEWING);
    expect(res.labelsToRemove).toHaveLength(0);
    expect(res.labelsToAdd).toHaveLength(0);
  });
});

describe('Prompt Template Renderer', () => {
  it('should render template variables accurately', () => {
    const rendered = renderPromptTemplate('.github/templates/prompts/initial_review.md', {
      role: '架构与契约审查员',
      skill_file: '.agents/skills/artifact-reviewer/SKILL.md',
      pr_number: '14',
      comment_id: '998877',
      repo: '0xTimi2233/ecommerce-core',
    });

    expect(rendered).toContain('你是 架构与契约审查员');
    expect(rendered).toContain('PR #14');
    expect(rendered).toContain('.agents/skills/artifact-reviewer/SKILL.md');
    expect(rendered).toContain('Comment ID: 998877');
    expect(rendered).toContain('repos/0xTimi2233/ecommerce-core/issues/comments/998877');
  });
});
