# Design: Review-Implementation as Parallel Subagents

**Date:** 2026-02-27
**Status:** Approved

## Problem

The `review-implementation` skill runs inline in the main agent's context after implementation. This causes:
1. **Context bias** — the agent that just wrote the code is reviewing it, anchored to its own decisions
2. **No fresh perspective** — all implementation history pollutes the review
3. **No automatic trigger** — executing-plans has no review step after batches

## Design

### Split into Two Subagent Prompts

One skill (`review-implementation`) dispatches two parallel subagents via `superpowers:code-reviewer`:

| Subagent | Prompt file | Scope | When |
|----------|------------|-------|------|
| Structural reviewer | `structural-reviewer-prompt.md` | Model/rule checklists (16/14 items) + semantic review | If `src/models/` or `src/rules/` in diff |
| Quality reviewer | `quality-reviewer-prompt.md` | DRY, KISS, HC/LC, HCI, test quality | Always |

### File Structure

```
.claude/skills/review-implementation/
├── SKILL.md                          # Orchestrator: how main agent dispatches
├── structural-reviewer-prompt.md     # Self-contained checklist for subagent
└── quality-reviewer-prompt.md        # Self-contained quality review for subagent
```

### Integration with executing-plans

New steps after each batch:

```
Step 2: Execute Batch (3 tasks)
  ↓
Step 2.5: Dispatch Review Subagents (parallel)
  ├── structural-reviewer (if model/rule files in diff)
  └── quality-reviewer (always)
  ↓
Step 2.6: Main Agent Addresses Findings
  - Fix FAIL items automatically
  - Report unfixable/ambiguous items to user
  ↓
Step 3: Report (implementation + review results + fixes)
```

Main agent determines diff via `git diff --name-only` against batch start SHA.

### Standalone / add-model / add-rule Integration

`/review-implementation` invocation:
1. Auto-detect what changed (git diff)
2. Dispatch structural + quality subagents in parallel
3. Collect results
4. Fix what it can automatically
5. Present consolidated report to user

### Prompt Template Design

**structural-reviewer-prompt.md** (self-contained):
- Full model checklist table (16 items) with Grep/Glob verification methods
- Full rule checklist table (14 items)
- Semantic review: evaluate() correctness, dims(), overhead accuracy, extract_solution
- `make test clippy` build check
- Placeholders: `{REVIEW_TYPE}`, `{PROBLEM_NAME}`, `{CATEGORY}`, `{FILE_STEM}`, `{SOURCE}`, `{TARGET}`, `{RULE_STEM}`
- Output: structured table with PASS/FAIL per item

**quality-reviewer-prompt.md** (self-contained):
- DRY, KISS, HC/LC design principles with detection criteria
- HCI checks (error messages, discoverability, consistency, least surprise, feedback)
- Naive test detection (types-only, mirrors-impl, no-adversarial, trivial-only, etc.)
- Placeholders: `{DIFF_SUMMARY}`, `{CHANGED_FILES}`, `{PLAN_STEP}`
- Output: structured findings with severity (Critical/Important/Minor)

### Main Agent Fix Strategy

| Finding type | Action |
|-------------|--------|
| Missing file/registration (structural FAIL) | Fix automatically |
| Missing test case | Fix automatically |
| Semantic correctness issue (clear) | Fix automatically |
| Semantic correctness issue (ambiguous) | Report to user |
| Code quality (Important+) | Fix automatically |
| Code quality (Minor) | Report to user |

### CLAUDE.md Changes

- Update `review-implementation` skill description to mention subagent dispatch
- Add note about parallel subagent dispatch to executing-plans integration section
- Keep single `/review-implementation` entry point

### What Doesn't Change

- SDD (subagent-driven-development) keeps its own two-stage review (spec + generic code quality)
- The review-implementation invocation syntax stays the same
- The structured output format stays the same (tables with PASS/FAIL)
