---
name: project-pipeline
description: Pick a Ready issue from the GitHub Project board, move it through In Progress -> issue-to-pr -> Review pool
---

# Project Pipeline

Pick a "Ready" issue from the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1), move it to "In Progress", run `issue-to-pr --execute`, then move it to "Review pool". The separate `review-pipeline` handles Copilot comments, CI fixes, and agentic testing.

## Invocation

- `/project-pipeline` -- pick the highest-ranked Ready issue (ranked by importance, relatedness, pending rules)
- `/project-pipeline 97` -- process a specific issue number from the Ready column
- `/project-pipeline --all` -- batch-process all Ready issues in ranked order

For Codex, open this `SKILL.md` directly and treat the slash-command forms above as aliases. The Makefile `run-pipeline` target already does this translation.

## Constants

GitHub Project board IDs (for `gh project item-edit`):

| Constant | Value |
|----------|-------|
| `PROJECT_ID` | `PVT_kwDOBrtarc4BRNVy` |
| `STATUS_FIELD_ID` | `PVTSSF_lADOBrtarc4BRNVyzg_GmQc` |
| `STATUS_READY` | `f37d0d80` |
| `STATUS_IN_PROGRESS` | `a12cfc9c` |
| `STATUS_REVIEW_POOL` | `7082ed60` |
| `STATUS_UNDER_REVIEW` | `f04790ca` |
| `STATUS_FINAL_REVIEW` | `51a3d8bb` |
| `STATUS_DONE` | `6aca54fa` |

## Autonomous Mode

This skill runs **fully autonomously** — no confirmation prompts, no user questions. It picks the next issue and processes it end-to-end. All sub-skills (`issue-to-pr`, `check-issue`, `add-model`, `add-rule`, etc.) should also auto-approve any confirmation prompts.

## Steps

### 0. Discover and Rank Ready Issues

#### 0a. Fetch Ready Issues

```bash
gh project item-list 8 --owner CodingThrust --format json --limit 500
```

Filter items where `status == "Ready"`. Partition into `[Model]` and `[Rule]` buckets.

#### 0b. Gather Context for Ranking

1. **Existing problems:** Grep for problem struct definitions in the codebase: `grep -r "^pub struct" src/models/ | sed 's/.*pub struct \([A-Za-z]*\).*/\1/'` to get all problem names currently implemented on `main`.
2. **Pending rules:** From the full project board JSON, collect all `[Rule]` issues that are in "Ready" or "In Progress" status. Parse their source/target problem names (e.g., `[Rule] BinPacking to ILP` → source=BinPacking, target=ILP).

#### 0c. Check Eligibility

**Rule issues require both source and target models to exist on `main`.** For each `[Rule]` issue, parse the source and target problem names (e.g., `[Rule] BinPacking to ILP` → source=BinPacking, target=ILP). Check that both appear in the existing problems list (from Step 0b grep).

- If both models exist in the codebase → **eligible**
- If either model is missing from the codebase → **ineligible**, mark it `[blocked]` with reason (e.g., "model X not yet implemented on main")

Do NOT consider pending `[Model]` issues as satisfying the dependency — only models already merged to `main` count. This prevents bundling model + rule in the same PR.

All `[Model]` issues are always eligible (no dependency check needed).

#### 0d. Score Eligible Issues

Score only **eligible** issues on three criteria. For `[Model]` issues, extract the problem name. For `[Rule]` issues, extract both source and target problem names.

| Criterion | Weight | How to Assess |
|-----------|--------|---------------|
| **C1: Industrial/Theoretical Importance** | 3 | Read the issue body. Score 0-2: **2** = widely used in industry or foundational in complexity theory (e.g., ILP, SAT, MaxFlow, TSP, GraphColoring); **1** = moderately important or well-studied (e.g., SubsetSum, SetCover, Knapsack); **0** = niche or primarily academic |
| **C2: Related to Existing Problems** | 2 | Check if the problem connects to problems already in the reduction graph (via `list_problems`). Score 0-2: **2** = directly related (shares input structure or has known reductions to/from ≥2 existing problems, but is NOT a trivial variant of an existing one); **1** = loosely related (same domain, connects to 1 existing problem); **0** = isolated or is essentially a variant/renaming of an existing problem |
| **C3: Unblocks Pending Rules** | 2 | Check if this issue is a dependency for pending `[Rule]` issues. Score 0-2: **2** = unblocks ≥2 pending rules (a `[Model]` issue whose problem appears as source or target in ≥2 pending rules); **1** = unblocks 1 pending rule; **0** = does not unblock any pending rule |

**Final score** = C1 × 3 + C2 × 2 + C3 × 2 (max = 12)

**Tie-breaking:** Models before Rules, then by lower issue number.

**Important for C2:** A problem that is merely a weighted/unweighted variant or a graph-subtype specialization of an existing problem scores **0** on C2, not 2. The goal is to add genuinely new problem types that expand the graph's reach.

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

**If `--all`:** proceed with all eligible issues in ranked order (highest score first). Models before Rules at same score. Blocked rules are skipped. After each issue is processed, re-check eligibility for remaining rules (a just-merged Model may unblock them).

**Otherwise (no args):** pick the highest-scored eligible (non-blocked) issue and proceed immediately (no confirmation).

### 1. Create Worktree

Create an isolated git worktree for this issue so the main working directory stays clean:

```bash
WORKTREE=$(python3 scripts/pipeline_worktree.py create-issue --issue <number> --slug <slug> --base origin/main --format json)
BRANCH=$(printf '%s\n' "$WORKTREE" | python3 -c "import sys,json; print(json.load(sys.stdin)['branch'])")
WORKTREE_DIR=$(printf '%s\n' "$WORKTREE" | python3 -c "import sys,json; print(json.load(sys.stdin)['worktree_dir'])")
cd "$WORKTREE_DIR"
```

All subsequent steps run inside the worktree. This ensures the user's main checkout is never modified.

### 2. Move to "In Progress"

Extract the project item ID for the chosen issue from the JSON output (the `id` field of the matching item).

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> in-progress
```

### 3. Run issue-to-pr --execute

Invoke the `issue-to-pr` skill with `--execute` (working directory is the worktree):

```
/issue-to-pr <number> --execute
```

This handles the full pipeline: fetch issue, verify Good label, research, write plan, create PR, implement, review, fix CI.

**If `issue-to-pr` fails after creating a PR:** record the failure, but still move the issue to "Final review" so it's visible for human triage. Report the failure to the user.

### 4. Move to "Review pool"

After `issue-to-pr` succeeds, move the issue to the `Review pool` column for the second-stage review pipeline:

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> review-pool
```

**If `issue-to-pr` failed after creating a PR:** move the issue to `Final review` instead so a human can take over:

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> final-review
```

**If no PR was created** (issue-to-pr failed before creating a PR): move the issue back to "Ready" instead:

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> ready
```

### 5. Clean Up Worktree

After the issue is processed (success or failure), clean up the worktree:

```bash
cd "$REPO_ROOT"
git worktree remove "$WORKTREE_DIR" --force
```

### 6. Report (single issue)

Print a summary:

```
Pipeline complete:
  Issue:  #97 [Rule] BinPacking to ILP
  PR:     #200
  Status: Awaiting agentic review
  Board:  Moved Ready -> In Progress -> Review pool
```

### 7. Batch Mode (`--all`)

If `--all` was specified, repeat Steps 1-6 for each issue in order. Each issue gets its own worktree (created and cleaned up per issue).

After all issues, print a batch report:

```
=== Project Pipeline Batch Report ===

| Issue | Title                              | PR   | Status      | Board       |
|-------|------------------------------------|------|-------------|-------------|
| #129  | [Model] MultivariateQuadratic      | #201 | CI green    | Review pool |
| #97   | [Rule] BinPacking to ILP           | #202 | CI green    | Review pool |
| #110  | [Rule] LCS to ILP                  | #203 | fix failed  | Review pool |
| #126  | [Rule] KSat to SubsetSum           | -    | plan failed | Ready       |

Completed: 2/4 | Review pool: 3 | Returned to Ready: 1
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Issue not in Ready column | Verify status before processing; STOP if not Ready |
| Picking a Rule whose model doesn't exist | Hard constraint: both source and target models must exist on `main` — pending Model issues do NOT count |
| Missing project scopes | Run `gh auth refresh -s read:project,project` |
| Forgetting to move back to Ready on total failure | Only move to Review pool if a PR exists |
| Processing Rules before their Model dependencies | In `--all` mode, re-check eligibility after each issue — a just-merged Model may unblock rules |
| Scoring a variant as "related" | Weighted/unweighted variants or graph-subtype specializations of existing problems score 0 on C2 |
| Not syncing main between batch issues | Each issue gets a fresh worktree from `origin/main` |
| Worktree left behind on failure | Always clean up with `git worktree remove` in Step 5 |
| Working in main checkout | All work happens in `.worktrees/` — never modify the main checkout |
| Missing items from project board | `gh project item-list` defaults to 30 items — always use `--limit 500` |
