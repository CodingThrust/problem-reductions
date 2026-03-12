---
name: project-pipeline
description: Pick a Ready issue from the GitHub Project board, move it through In Progress -> issue-to-pr -> review-agentic
---

# Project Pipeline

Pick a "Ready" issue from the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1), move it to "In Progress", run `issue-to-pr --execute`, then move it to "review-agentic". The separate `review-pipeline` handles Copilot comments, CI fixes, and agentic testing.

## Invocation

- `/project-pipeline` -- pick the highest-ranked Ready issue (ranked by importance, relatedness, pending rules)
- `/project-pipeline 97` -- process a specific issue number from the Ready column
- `/project-pipeline --all` -- batch-process all Ready issues in ranked order

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

### 0. Discover and Rank Ready Issues

#### 0a. Fetch Ready Issues

```bash
gh project item-list 8 --owner CodingThrust --format json
```

Filter items where `status == "Ready"`. Partition into `[Model]` and `[Rule]` buckets.

#### 0b. Gather Context for Ranking

1. **Existing problems:** Call `list_problems` (MCP tool) to get all problems currently in the reduction graph.
2. **Pending rules:** From the full project board JSON, collect all `[Rule]` issues that are in "Ready" or "In Progress" status. Parse their source/target problem names (e.g., `[Rule] BinPacking to ILP` → source=BinPacking, target=ILP).

#### 0c. Score Each Issue

Score each Ready issue on three criteria. For `[Model]` issues, extract the problem name. For `[Rule]` issues, extract both source and target problem names.

| Criterion | Weight | How to Assess |
|-----------|--------|---------------|
| **C1: Industrial/Theoretical Importance** | 3 | Read the issue body. Score 0-2: **2** = widely used in industry or foundational in complexity theory (e.g., ILP, SAT, MaxFlow, TSP, GraphColoring); **1** = moderately important or well-studied (e.g., SubsetSum, SetCover, Knapsack); **0** = niche or primarily academic |
| **C2: Related to Existing Problems** | 2 | Check if the problem connects to problems already in the reduction graph (via `list_problems`). Score 0-2: **2** = directly related (shares input structure or has known reductions to/from ≥2 existing problems, but is NOT a trivial variant of an existing one); **1** = loosely related (same domain, connects to 1 existing problem); **0** = isolated or is essentially a variant/renaming of an existing problem |
| **C3: Unblocks Pending Rules** | 2 | Check if this issue is a dependency for pending `[Rule]` issues. Score 0-2: **2** = unblocks ≥2 pending rules (a `[Model]` issue whose problem appears as source or target in ≥2 pending rules); **1** = unblocks 1 pending rule; **0** = does not unblock any pending rule |

**Final score** = C1 × 3 + C2 × 2 + C3 × 2 (max = 12)

**Tie-breaking:** Models before Rules, then by lower issue number.

**Important for C2:** A problem that is merely a weighted/unweighted variant or a graph-subtype specialization of an existing problem scores **0** on C2, not 2. The goal is to add genuinely new problem types that expand the graph's reach.

#### 0d. Apply Hard Constraints

**Rule issues require both source and target models to exist.** For each `[Rule]` issue, check that both its source and target problem names appear in the `list_problems` output (existing models) OR in a `[Model]` issue in the current Ready/In Progress columns.

- If both models exist → eligible
- If a missing model has a `[Model]` issue in Ready → eligible only in `--all` mode (the Model will be processed first); in single-issue mode, **skip this Rule** and mark it `[blocked]`
- If a missing model has no `[Model]` issue at all → **ineligible**, mark it `[blocked]` with reason

#### 0e. Print Ranked List

Print all Ready issues with their scores for visibility (no confirmation needed). Blocked rules appear at the bottom with their reason:

```
Ready issues (ranked):
  Score  Issue  Title                              C1  C2  C3
  ─────────────────────────────────────────────────────────────
    10   #117   [Model] GraphPartitioning           2   2   2
     8   #129   [Model] MultivariateQuadratic       2   1   1
     7   #97    [Rule] BinPacking to ILP            1   2   1
     6   #110   [Rule] LCS to ILP                   1   1   1
     4   #126   [Rule] KSatisfiability to SubsetSum  0   2   0

  Blocked:
     3   #130   [Rule] MultivariateQuadratic to ILP  -- model "MultivariateQuadratic" not yet implemented
```

#### 0f. Pick Issues

**If a specific issue number was provided:** verify it is in the Ready column. If it is blocked, STOP with a message explaining which model is missing.

**If `--all`:** proceed with all eligible issues in ranked order (highest score first). **Dependency ordering override:** if a `[Model]` issue and a `[Rule]` that depends on it are both eligible, the Model MUST be processed before that Rule regardless of score. Blocked rules are skipped.

**Otherwise (no args):** pick the highest-scored eligible (non-blocked) issue and proceed immediately (no confirmation).

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
| Picking a Rule whose model doesn't exist | Hard constraint: both source and target models must exist in codebase or be in a Ready Model issue (for `--all` mode) |
| Missing project scopes | Run `gh auth refresh -s read:project,project` |
| Forgetting to move back to Ready on total failure | Only move to In Review if a PR exists |
| Processing Rules before their Model dependencies | In `--all` mode, ensure Models that unblock pending rules come before those rules regardless of score |
| Scoring a variant as "related" | Weighted/unweighted variants or graph-subtype specializations of existing problems score 0 on C2 |
| Not syncing main between batch issues | Each issue gets a fresh worktree from `origin/main` |
| Worktree left behind on failure | Always clean up with `git worktree remove` in Step 5 |
| Working in main checkout | All work happens in `.worktrees/` — never modify the main checkout |
