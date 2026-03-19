---
name: run-pipeline
description: Pick a Ready issue from the GitHub Project board, move it through In Progress -> issue-to-pr -> Review pool
---

# Run Pipeline

Pick a "Ready" issue from the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1), claim it into "In Progress", run `issue-to-pr`, then move it to "Review pool". The separate `review-pipeline` handles agentic review (structural check, quality check, agentic feature tests).

## Invocation

- `/run-pipeline` -- pick the highest-ranked Ready issue (ranked by importance, relatedness, pending rules)
- `/run-pipeline 97` -- process a specific issue number from the Ready column

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

### 0. Generate the Project-Pipeline Report

Step 0 should be a single report-generation step. Do not manually list Ready items, list In-progress items, grep model declarations, or re-derive blocked rules with separate shell commands.
The expensive full-context call here is `python3 scripts/pipeline_skill_context.py project-pipeline ...` (backed by `build_project_pipeline_context()`). For a single top-level `run-pipeline` invocation, call it once and reuse the packet for scoring, ranking, and choosing the issue. Do not rerun it in the single-issue path after the packet exists.

```bash
set -- python3 scripts/pipeline_skill_context.py project-pipeline --repo CodingThrust/problem-reductions --repo-root . --format text

# If a specific issue number was provided, validate it through the same bundle:
# set -- "$@" --issue <number>

REPORT=$("$@")
printf '%s\n' "$REPORT"
```

The report is the Step 0 packet. It should already include:
- Queue Summary
- Eligible Ready Issues
- Blocked Ready Issues
- In Progress Issues
- Requested Issue validation when a specific issue was supplied

Branch from the report:
- `Bundle status: empty` => STOP with `No Ready issues are currently available.`
- `Bundle status: no-eligible-issues` => STOP with `Ready issues exist, but all current rule candidates are blocked by missing models on main.`
- `Bundle status: requested-missing` => STOP with `Issue #N is not currently in the Ready column.`
- `Bundle status: requested-blocked` => STOP with the blocking reason from the report
- `Bundle status: ready` => continue

The report already handled the deterministic setup:
- it loaded the Ready and In-progress issue sets
- it scanned existing problems on main
- it marked blocked `[Rule]` issues whose source or target model is still missing
- it computed the pending-rule unblock counts used for C3

#### 0a. Score Eligible Issues

**Short-circuit:** If there is only 1 eligible issue, skip scoring and pick it directly. Print "Only 1 eligible issue, picking it." and jump to Step 0c.

Score only **eligible** issues on three criteria. For `[Model]` issues, extract the problem name. For `[Rule]` issues, extract both source and target problem names.

| Criterion | Weight | How to Assess |
|-----------|--------|---------------|
| **C1: Industrial/Theoretical Importance** | 3 | Read the report's issue summary for each eligible issue. Score 0-2: **2** = widely used in industry or foundational in complexity theory (e.g., ILP, SAT, MaxFlow, TSP, GraphColoring); **1** = moderately important or well-studied (e.g., SubsetSum, SetCover, Knapsack); **0** = niche or primarily academic |
| **C2: Related to Existing Problems** | 2 | Use the report's Ready/In-progress context plus `pred list` if needed. Score 0-2: **2** = directly related (shares input structure or has known reductions to/from ≥2 existing problems, but is NOT a trivial variant of an existing one); **1** = loosely related (same domain, connects to 1 existing problem); **0** = isolated or is essentially a variant/renaming of an existing problem |
| **C3: Unblocks Pending Rules** | 2 | Read the `Pending rules unblocked` count already printed in the report for each eligible issue. Score 0-2: **2** = unblocks ≥2 pending rules; **1** = unblocks 1 pending rule; **0** = does not unblock any pending rule |

**Final score** = C1 × 3 + C2 × 2 + C3 × 2 (max = 12)

**Tie-breaking:** Models before Rules, then by lower issue number.

**Important for C2:** A problem that is merely a weighted/unweighted variant or a graph-subtype specialization of an existing problem scores **0** on C2, not 2. The goal is to add genuinely new problem types that expand the graph's reach.

#### 0b. Print Ranked List

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

#### 0c. Pick Issues

**If a specific issue number was provided:** validate and claim it through the scripted bundle:

```bash
STATE_FILE=/tmp/problemreductions-ready-selection.json
CLAIM=$(python3 scripts/pipeline_board.py claim-next ready "$STATE_FILE" --number <number> --format json)
```

The report should already have stopped you before this point if the requested issue was missing or blocked.

After successful validation, extract `ITEM_ID`, `ISSUE`, and `TITLE` from `CLAIM` using the same commands shown below.

**Otherwise (no args):** score the eligible issues from the report, pick the highest-scored one, and proceed immediately (no confirmation). After picking the issue number, claim it through the scripted bundle:

```bash
STATE_FILE=/tmp/problemreductions-ready-selection.json
CLAIM=$(python3 scripts/pipeline_board.py claim-next ready "$STATE_FILE" --number <chosen-issue-number> --format json)
```

Extract the board item metadata from `CLAIM`:

```bash
ITEM_ID=$(printf '%s\n' "$CLAIM" | python3 -c "import sys,json; print(json.load(sys.stdin)['item_id'])")
ISSUE=$(printf '%s\n' "$CLAIM" | python3 -c "import sys,json; data=json.load(sys.stdin); print(data['issue_number'] or data['number'])")
TITLE=$(printf '%s\n' "$CLAIM" | python3 -c "import sys,json; print(json.load(sys.stdin)['title'])")
```

### 1. Create Worktree

Create an isolated worktree for this issue:

```bash
REPO_ROOT=$(pwd)
WORKTREE_JSON=$(python3 scripts/pipeline_worktree.py enter --name "issue-$ISSUE" --format json)
WORKTREE_DIR=$(printf '%s\n' "$WORKTREE_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['worktree_dir'])")
cd "$WORKTREE_DIR"
```

All subsequent steps run inside the worktree. This ensures the user's main checkout is never modified.

`issue-to-pr` (Step 3) handles all PR detection and branch management — if an existing open PR exists, it checks out that branch and resumes; otherwise it creates a fresh branch from `origin/main`.

### 2. Claim Result

`claim-next ready` has already moved the selected issue from `Ready` to `In progress`. Keep using `ITEM_ID` from the `CLAIM` JSON payload for later board transitions.

### 3. Run issue-to-pr

Invoke the `issue-to-pr` skill (working directory is the worktree):

```
/issue-to-pr "$ISSUE"
```

This handles the full pipeline: fetch issue, verify Good label, research, write plan, create PR, implement. If an existing open PR is detected, `issue-to-pr` will resume it (skip plan creation, jump to execution).

**If `issue-to-pr` fails:** move the issue to OnHold with a diagnostic comment (see Step 4).

### 4. Move to "Review pool"

After `issue-to-pr` fully succeeds, move the issue to the `Review pool` column. "Fully succeeds" means the implementation work is committed, the temporary plan file has been deleted, the PR implementation summary comment has been posted, the branch has been pushed, and the working tree is clean aside from ignored/generated files:

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> review-pool
```

**If `issue-to-pr` failed (whether or not a PR was created):** move the issue to `OnHold` with a diagnostic comment explaining what went wrong:

```bash
gh issue comment <ISSUE> --body "run-pipeline: implementation failed. <brief reason>"
python3 scripts/pipeline_board.py move <ITEM_ID> on-hold
```

**Forward-only rule:** never move items backward (e.g., back to Ready). All failures go to OnHold for human triage.

### 5. Clean Up Worktree

After the issue is processed (success or failure), clean up the worktree:

```bash
cd "$REPO_ROOT"
python3 scripts/pipeline_worktree.py cleanup --worktree "$WORKTREE_DIR"
```

### 6. Report

Print a summary:

```
Pipeline complete:
  Issue:  #97 [Rule] BinPacking to ILP
  PR:     #200
  Status: Awaiting agentic review
  Board:  Moved Ready -> In Progress -> Review pool
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Issue not in Ready column | Verify status before processing; STOP if not Ready |
| Picking a Rule whose model doesn't exist | Hard constraint: both source and target models must exist on `main` — pending Model issues do NOT count |
| Missing project scopes | Run `gh auth refresh -s read:project,project` |
| Moving items backward to Ready | Never move backward — all failures go to OnHold with diagnostic comment |
| Scoring a variant as "related" | Weighted/unweighted variants or graph-subtype specializations of existing problems score 0 on C2 |
| Worktree left behind on failure | Always run `pipeline_worktree.py cleanup` in Step 5 |
| Working in main checkout | All work happens in the worktree — never modify the main checkout |
| Missing items from project board | `gh project item-list` defaults to 30 items — always use `--limit 500` |
| Inventing `pipeline_board.py` subcommands | Only `next`, `claim-next`, `ack`, `list`, `move`, `backlog` exist |
