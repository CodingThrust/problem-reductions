---
name: review-implementation
description: Use after implementing a model, rule, or any code change to verify completeness and correctness before committing
---

# Review Implementation

Dispatches two parallel review subagents with fresh context (no implementation history bias):
- **Structural reviewer** -- model/rule checklists + semantic correctness (only for new models/rules)
- **Quality reviewer** -- DRY, KISS, HC/LC, HCI, test quality (always)

## Invocation

- `/review-implementation` -- auto-detect from git diff
- `/review-implementation model MaximumClique` -- review a specific model
- `/review-implementation rule mis_qubo` -- review a specific rule
- `/review-implementation generic` -- code quality only (no structural checklist)

## Step 1: Detect What Changed

Determine whether new model/rule files were added:

```bash
# Check for NEW files across the entire branch
git diff --name-only --diff-filter=A main..HEAD
```

Detection rules:
- New file in `src/models/` (not `mod.rs`) -> **model review** (structural + quality)
- New file in `src/rules/` (not `mod.rs`, `traits.rs`, `cost.rs`, `graph.rs`, `registry.rs`) -> **rule review** (structural + quality)
- Only modified files (no new model/rule) -> **quality review only**
- Both new model and rule files -> dispatch structural for both + quality
- Explicit argument overrides auto-detection

Extract the problem name(s) and rule source/target from the file paths.

## Step 2: Prepare Subagent Context

Get the git SHAs for the review range:

```bash
BASE_SHA=$(git merge-base main HEAD)  # or HEAD~N for batch reviews
HEAD_SHA=$(git rev-parse HEAD)
```

Get the diff summary and changed file list:

```bash
git diff --stat $BASE_SHA..$HEAD_SHA
git diff --name-only $BASE_SHA..$HEAD_SHA
```

### Detect Linked Issue

Check if the current branch has a PR linked to an issue:

```bash
# Get PR number for current branch
PR_NUM=$(gh pr view --json number -q .number 2>/dev/null)

# If PR exists, extract the linked issue number from the body
if [ -n "$PR_NUM" ]; then
  ISSUE_NUM=$(gh pr view $PR_NUM --json body -q .body | grep -oE '#[0-9]+' | head -1 | tr -d '#')
fi

# Fetch the issue body if found
if [ -n "$ISSUE_NUM" ]; then
  ISSUE_BODY=$(gh issue view $ISSUE_NUM --json title,body -q '"# " + .title + "\n\n" + .body')
fi
```

If an issue is found, pass it as `{ISSUE_CONTEXT}` to both subagents. If not, set `{ISSUE_CONTEXT}` to "No linked issue found."

## Step 3: Dispatch Subagents in Parallel

### Structural Reviewer (if new model/rule detected)

Dispatch using `Agent` tool with `subagent_type="superpowers:code-reviewer"`:

- Read `structural-reviewer-prompt.md` from this skill directory
- Fill placeholders:
  - `{REVIEW_TYPE}` -> "model", "rule", or "model + rule"
  - `{REVIEW_PARAMS}` -> summary of what's being reviewed
  - `{PROBLEM_NAME}`, `{CATEGORY}`, `{FILE_STEM}` -> for model reviews
  - `{SOURCE}`, `{TARGET}`, `{RULE_STEM}`, `{EXAMPLE_STEM}` -> for rule reviews
  - `{ISSUE_CONTEXT}` -> full issue title + body (or "No linked issue found.")
- Prompt = filled template

### Quality Reviewer (always)

Dispatch using `Agent` tool with `subagent_type="superpowers:code-reviewer"`:

- Read `quality-reviewer-prompt.md` from this skill directory
- Fill placeholders:
  - `{DIFF_SUMMARY}` -> output of `git diff --stat`
  - `{CHANGED_FILES}` -> list of changed files
  - `{PLAN_STEP}` -> description of what was implemented (or "standalone review")
  - `{BASE_SHA}`, `{HEAD_SHA}` -> git range
  - `{ISSUE_CONTEXT}` -> full issue title + body (or "No linked issue found.")
- Prompt = filled template

**Both subagents must be dispatched in parallel** (single message with two Agent tool calls — use `run_in_background: true` on one, foreground on the other, then read the background result with `TaskOutput`).

## Step 4: Collect and Address Findings

When both subagents return:

1. **Parse results** -- identify FAIL/ISSUE items from both reports
2. **Fix automatically** -- structural FAILs (missing registration, missing file), clear semantic issues, Important+ quality issues
3. **Update complexity review table** -- for any new/modified model or rule, add a row to `docs/src/complexity-review.md` with result PASS/FAIL/UNVERIFIED and review date
4. **Report to user** -- ambiguous semantic issues, Minor quality items, anything you're unsure about
5. **Present consolidated report** combining both reviews

## Step 5: Present Consolidated Report

Merge both subagent outputs into a single report:

```
## Review: [Model/Rule/Generic] [Name]

### Structural Completeness (from structural reviewer)
| # | Check | Status |
|---|-------|--------|
...

### Build Status (from structural reviewer)
- `make test`: PASS / FAIL
- `make clippy`: PASS / FAIL

### Semantic Review (from structural reviewer)
...

### Issue Compliance (from structural reviewer, if linked issue found)
...

### Code Quality (from quality reviewer)
- DRY: OK / ...
- KISS: OK / ...
- HC/LC: OK / ...

### HCI (from quality reviewer, if CLI/MCP changed)
...

### Test Quality (from quality reviewer)
...

### Complexity & Overhead Review
- Update `docs/src/complexity-review.md` with review results for any new or modified models/rules
- Models table: verify `declare_variants!` complexity against best known algorithm
- Rules table: verify `#[reduction(overhead)]` against actual `reduce_to()` code
- Result: PASS / FAIL / UNVERIFIED

### Fixes Applied
- [list of issues automatically fixed by main agent]

### Remaining Items (needs user decision)
- [list of issues that need user input]
```

## Integration

### With executing-plans

After each batch in the executing-plans flow, the main agent should:
1. Record `BASE_SHA` before the batch starts
2. After batch completes, follow Steps 1-5 above
3. Fix findings before reporting to user
4. Include review results in the batch report

### Copilot Review (after PR creation)

After creating a PR (from any flow), run `make copilot-review` to request GitHub Copilot code review on the PR.

### With add-model / add-rule

At the end of these skills (after their verify step), invoke `/review-implementation` which dispatches subagents as described above.

### Standalone

Invoke directly via `/review-implementation` for any code change.
