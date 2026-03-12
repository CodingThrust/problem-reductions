---
name: project-pipeline
description: Pick a Ready issue from the GitHub Project board, move it through In Progress -> issue-to-pr -> review-agentic
---

# Project Pipeline

Pick a "Ready" issue from the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1), move it to "In Progress", run `issue-to-pr --execute`, then move it to "review-agentic". The separate `review-pipeline` handles Copilot comments, CI fixes, and agentic testing.

## Invocation

- `/project-pipeline` -- pick the next Ready issue (first Model, then Rule, by issue number)
- `/project-pipeline 97` -- process a specific issue number from the Ready column
- `/project-pipeline --all` -- batch-process all Ready issues (Models first, then Rules)

## Constants

GitHub Project board IDs (for `gh project item-edit`):

| Constant | Value |
|----------|-------|
| `PROJECT_ID` | `PVT_kwDOBrtarc4BRNVy` |
| `STATUS_FIELD_ID` | `PVTSSF_lADOBrtarc4BRNVyzg_GmQc` |
| `STATUS_READY` | `61e4505c` |
| `STATUS_IN_PROGRESS` | `47fc9ee4` |
| `STATUS_REVIEW_AGENTIC` | `b2f16561` |
| `STATUS_IN_REVIEW` | `df73e18b` |
| `STATUS_DONE` | `98236657` |

## Autonomous Mode

This skill runs **fully autonomously** — no confirmation prompts, no user questions. It picks the next issue and processes it end-to-end. All sub-skills (`issue-to-pr`, `check-issue`, `add-model`, `add-rule`, etc.) should also auto-approve any confirmation prompts.

## Steps

### 0. Discover Ready Issues

```bash
gh project item-list 8 --owner CodingThrust --format json
```

Filter items where `status == "Ready"`. Partition into `[Model]` and `[Rule]` buckets, sort each by issue number ascending. Final order: **all Models first, then all Rules** (so dependencies are satisfied).

Print the list for visibility (no confirmation needed):

```
Ready issues:
  Models:
    #129  [Model] MultivariateQuadratic
    #117  [Model] GraphPartitioning
  Rules:
    #97   [Rule] BinPacking to ILP
    #110  [Rule] LCS to ILP
    #126  [Rule] KSatisfiability to SubsetSum
    #130  [Rule] MultivariateQuadratic to ILP
```

**If a specific issue number was provided:** verify it is in the Ready column. If not, STOP with a message.

**If `--all`:** proceed immediately with all Ready issues in order (no confirmation).

**Otherwise (no args):** pick the first issue in the ordered list (Models before Rules, lowest number first) and proceed immediately (no confirmation).

### 1. Create Worktree

Create an isolated git worktree for this issue so the main working directory stays clean:

```bash
git fetch origin main
BRANCH="issue-<number>-<slug>"
WORKTREE_DIR=".worktrees/$BRANCH"
mkdir -p .worktrees
git worktree add "$WORKTREE_DIR" -b "$BRANCH" origin/main
cd "$WORKTREE_DIR"
```

All subsequent steps run inside the worktree. This ensures the user's main checkout is never modified.

### 2. Move to "In Progress"

Extract the project item ID for the chosen issue from the JSON output (the `id` field of the matching item).

```bash
gh project item-edit \
  --id <ITEM_ID> \
  --project-id PVT_kwDOBrtarc4BRNVy \
  --field-id PVTSSF_lADOBrtarc4BRNVyzg_GmQc \
  --single-select-option-id 47fc9ee4
```

### 3. Run issue-to-pr --execute

Invoke the `issue-to-pr` skill with `--execute` (working directory is the worktree):

```
/issue-to-pr <number> --execute
```

This handles the full pipeline: fetch issue, verify Good label, research, write plan, create PR, implement, review, fix CI.

**If `issue-to-pr` fails:** record the failure, but still move the issue to "In Review" so it's visible for human triage. Report the failure to the user.

### 4. Move to "review-agentic"

After `issue-to-pr` completes (success or failure with a PR created), move the issue to the `review-agentic` column for the second-stage review pipeline:

```bash
gh project item-edit \
  --id <ITEM_ID> \
  --project-id PVT_kwDOBrtarc4BRNVy \
  --field-id PVTSSF_lADOBrtarc4BRNVyzg_GmQc \
  --single-select-option-id b2f16561
```

**If no PR was created** (issue-to-pr failed before creating a PR): move the issue back to "Ready" instead:

```bash
gh project item-edit \
  --id <ITEM_ID> \
  --project-id PVT_kwDOBrtarc4BRNVy \
  --field-id PVTSSF_lADOBrtarc4BRNVyzg_GmQc \
  --single-select-option-id 61e4505c
```

### 5. Clean Up Worktree

After the issue is processed (success or failure), clean up the worktree:

```bash
cd /Users/liujinguo/rcode/problemreductions
git worktree remove "$WORKTREE_DIR" --force
```

### 6. Report (single issue)

Print a summary:

```
Pipeline complete:
  Issue:  #97 [Rule] BinPacking to ILP
  PR:     #200
  Status: Awaiting agentic review
  Board:  Moved Ready -> In Progress -> review-agentic
```

### 7. Batch Mode (`--all`)

If `--all` was specified, repeat Steps 1-6 for each issue in order. Each issue gets its own worktree (created and cleaned up per issue).

After all issues, print a batch report:

```
=== Project Pipeline Batch Report ===

| Issue | Title                              | PR   | Status      | Board       |
|-------|------------------------------------|------|-------------|-------------|
| #129  | [Model] MultivariateQuadratic      | #201 | CI green    | review-agentic |
| #97   | [Rule] BinPacking to ILP           | #202 | CI green    | review-agentic |
| #110  | [Rule] LCS to ILP                  | #203 | fix failed  | review-agentic |
| #126  | [Rule] KSat to SubsetSum           | -    | plan failed | Ready       |

Completed: 2/4 | In Review: 3 | Returned to Ready: 1
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Issue not in Ready column | Verify status before processing; STOP if not Ready |
| Missing project scopes | Run `gh auth refresh -s read:project,project` |
| Forgetting to move back to Ready on total failure | Only move to In Review if a PR exists |
| Processing Rules before Models | Always sort Models first — Rules may depend on them |
| Not syncing main between batch issues | Each issue gets a fresh worktree from `origin/main` |
| Worktree left behind on failure | Always clean up with `git worktree remove` in Step 5 |
| Working in main checkout | All work happens in `.worktrees/` — never modify the main checkout |
