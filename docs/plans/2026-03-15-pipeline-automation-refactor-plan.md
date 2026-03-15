# Pipeline Automation Refactor Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extract deterministic pipeline orchestration out of repo-local skills into tested Python helpers and thin `Makefile` entry points, while preserving agent skills for judgment-heavy decisions.

**Architecture:** Introduce a small `scripts/pipeline_*.py` layer for board selection, PR context, worktree lifecycle, and review checklists. `Makefile` stays the human-facing interface. Skills shrink to the parts that require reasoning: prioritization, research, code changes, semantic review, and maintainer judgment.

**Tech Stack:** Python 3, `gh` CLI, `unittest`, `Makefile`, repo-local `SKILL.md` docs

---

## Recommended Approach

### Option A: Keep skills as-is, add a few shell helpers

- Lowest implementation cost
- Keeps most logic duplicated across `SKILL.md`
- Hard to test and easy to drift

### Option B: Hybrid scripted orchestration + skill judgment (**Recommended**)

- Scripts own deterministic logic
- Skills call scripts, interpret results, and make decisions
- Best trade-off between reliability and flexibility

### Option C: Full scripted pipeline with minimal skills

- Maximum determinism
- Too rigid for research-heavy issue planning and nuanced final review
- Higher risk of encoding unstable heuristics into scripts

The rest of this plan assumes **Option B**.

---

## File Structure

### New scripts

- Create: `scripts/pipeline_board.py`
- Create: `scripts/pipeline_pr.py`
- Create: `scripts/pipeline_worktree.py`
- Create: `scripts/pipeline_checks.py`

### Shared helpers

- Create: `scripts/pipeline_common.py`

### Tests

- Create: `scripts/test_pipeline_board.py`
- Create: `scripts/test_pipeline_pr.py`
- Create: `scripts/test_pipeline_worktree.py`
- Create: `scripts/test_pipeline_checks.py`

### Existing files to shrink

- Modify: `scripts/project_board_poll.py`
- Modify: `scripts/project_board_recover.py`
- Modify: `scripts/make_helpers.sh`
- Modify: `Makefile`
- Modify: `.claude/skills/project-pipeline/SKILL.md`
- Modify: `.claude/skills/review-pipeline/SKILL.md`
- Modify: `.claude/skills/final-review/SKILL.md`
- Modify: `.claude/skills/issue-to-pr/SKILL.md`
- Modify: `.claude/skills/fix-pr/SKILL.md`
- Modify: `.claude/skills/review-implementation/SKILL.md`

---

## Script APIs

### `scripts/pipeline_board.py`

Purpose: all project-board reads, selection, and status transitions.

Subcommands:

- `next-ready --repo <repo> [--issue <n>] [--format json]`
- `next-review --repo <repo> [--pr <n>] [--allow-ambiguous skip|report] [--format json]`
- `next-final-review --repo <repo> [--pr <n>] [--format json]`
- `move --item-id <id> --status <ready|in-progress|review-pool|under-review|final-review|done|on-hold>`
- `claim-review --item-id <id>`
- `recover --repo <repo> [--apply]`

Output contract:

- Always emit stable JSON in machine mode.
- Include `item_id`, `status`, `title`, `issue_number`, `pr_number`, `reason`, `ambiguity`, and `recommendation` when relevant.

Notes:

- Fold the useful logic from `project_board_poll.py` and `project_board_recover.py` into this file.
- Keep `project_board_poll.py` as a thin compatibility wrapper or delete it after migration.

### `scripts/pipeline_pr.py`

Purpose: one-shot PR snapshots and repeated PR-related queries.

Subcommands:

- `snapshot --repo <repo> --pr <n> --format json`
- `comments --repo <repo> --pr <n> --format json`
- `ci --repo <repo> --pr <n> --format json`
- `wait-ci --repo <repo> --pr <n> [--timeout 900] [--interval 30] --format json`
- `codecov --repo <repo> --pr <n> --format json`
- `linked-issue --repo <repo> --pr <n> --format json`
- `edit-body --repo <repo> --pr <n> --body-file <path>`
- `comment --repo <repo> --pr <n> --body-file <path>`
- `close --repo <repo> --pr <n> --comment-file <path>`

Output contract:

- `snapshot` should include title, body, state, mergeability, linked issue, changed files, review counts, CI summary, and Codecov summary.

### `scripts/pipeline_worktree.py`

Purpose: worktree creation, PR checkout, merge-from-main, and cleanup.

Subcommands:

- `create-issue --issue <n> --slug <slug> --base origin/main --format json`
- `checkout-pr --repo <repo> --pr <n> --format json`
- `merge-main --worktree <path> --format json`
- `cleanup --worktree <path>`

Output contract:

- Return `worktree_dir`, `branch`, `base_sha`, `head_sha`.
- `merge-main` returns `status=clean|conflicted|aborted`, plus `conflicts` and a `likely_complex` boolean.

### `scripts/pipeline_checks.py`

Purpose: deterministic review and completeness checks that skills currently describe in prose.

Subcommands:

- `detect-scope --base <sha> --head <sha> --format json`
- `file-whitelist --kind model|rule --files-file <path> --format json`
- `completeness --kind model|rule --name <name> [--source <src> --target <dst>] --format json`
- `issue-guards --repo <repo> --issue <n> --format json`

Output contract:

- `detect-scope` returns review type, problem/rule identifiers, and changed file lists.
- `file-whitelist` returns out-of-policy files with reasons.
- `completeness` returns pass/fail checks for registrations, tests, paper entries, and trait consistency.

### `scripts/pipeline_common.py`

Purpose: shared `gh` runner, JSON decoding, project constants, and common parsing.

Keep this small:

- `run_gh_json(...)`
- status option IDs
- PR/issue/title parsing helpers
- common error formatting

---

## Makefile Changes

Keep `Makefile` as the user entrypoint, but stop encoding workflow logic in shell prompts.

### New targets

- `make board-next MODE=ready`
- `make board-next MODE=review`
- `make board-move ITEM=<id> STATUS=<status>`
- `make pr-context PR=<n>`
- `make pr-wait-ci PR=<n>`
- `make worktree-pr PR=<n>`

### Existing targets to keep

- `make run-pipeline`
- `make run-review`
- `make run-review-forever`

### Change in responsibility

- `run-pipeline` and `run-review` should call scripts first, then pass structured context into the agent prompt.
- `run-review-forever` should continue using queue state, but the eligibility logic should live in `pipeline_board.py`, not in shell.

---

## Skill Shrink Map

### `project-pipeline`

Shrink these sections:

- Replace discovery and raw filtering from `.claude/skills/project-pipeline/SKILL.md:39`
- Replace board move snippets from `.claude/skills/project-pipeline/SKILL.md:123`
- Replace worktree boilerplate from `.claude/skills/project-pipeline/SKILL.md:107`

Keep:

- C1/C2 scoring judgment from `.claude/skills/project-pipeline/SKILL.md:65`
- decision to proceed with a ranked issue
- delegation to `issue-to-pr --execute`

New skill shape:

- Call `pipeline_board.py next-ready`
- Read JSON candidates
- Apply judgment-only ranking
- Call `pipeline_worktree.py create-issue`
- Call `pipeline_board.py move`
- Continue with `issue-to-pr`

### `review-pipeline`

Shrink these sections:

- Replace candidate discovery and stale/ambiguity filtering from `.claude/skills/review-pipeline/SKILL.md:41`
- Replace claim/move snippets from `.claude/skills/review-pipeline/SKILL.md:106`
- Replace worktree checkout and merge boilerplate from `.claude/skills/review-pipeline/SKILL.md:120`
- Replace comment harvesting shell blocks from `.claude/skills/review-pipeline/SKILL.md:187`
- Replace CI polling loop later in the file with `pipeline_pr.py wait-ci`

Keep:

- how to interpret Copilot and human comments
- whether a suggested fix is technically correct
- invoking `fix-pr`, `review-implementation`, and `agentic-tests:test-feature`

New skill shape:

- Call `pipeline_board.py next-review`
- If `ambiguity.kind != none`, ask the user and present the script's recommendation
- Call `pipeline_worktree.py checkout-pr`
- Call `pipeline_worktree.py merge-main`
- Call `pipeline_pr.py comments`
- Make judgment calls and edit code

### `final-review`

Shrink these sections:

- Replace final-review PR discovery from `.claude/skills/final-review/SKILL.md:31`
- Replace PR metadata gathering from `.claude/skills/final-review/SKILL.md:43`
- Replace file whitelist checking from `.claude/skills/final-review/SKILL.md:105`
- Replace board move snippets from `.claude/skills/final-review/SKILL.md:232`

Keep:

- usefulness assessment
- safety interpretation
- completeness judgment
- quality percentile and merge/on-hold recommendation

New skill shape:

- Call `pipeline_board.py next-final-review`
- Call `pipeline_pr.py snapshot`
- Call `pipeline_checks.py file-whitelist`
- Present structured context to the maintainer

### `issue-to-pr`

Shrink these sections:

- Replace input normalization and issue guards from `.claude/skills/issue-to-pr/SKILL.md:30`
- Replace model-existence guard from `.claude/skills/issue-to-pr/SKILL.md:59`
- Replace PR resume/create mechanics from `.claude/skills/issue-to-pr/SKILL.md:107`
- Replace plan-file cleanup mechanics from `.claude/skills/issue-to-pr/SKILL.md:180`

Keep:

- reference research
- implementation-plan authoring
- deciding how to batch plan steps
- subagent-driven execution

New skill shape:

- Call `pipeline_checks.py issue-guards`
- Call `pipeline_worktree.py create-issue`
- Call `pipeline_pr.py snapshot` when resuming an existing PR
- Keep the agent focused on research and plan quality

### `fix-pr`

Shrink these sections:

- Replace PR snapshot gathering from `.claude/skills/fix-pr/SKILL.md:10`
- Replace comments aggregation from `.claude/skills/fix-pr/SKILL.md:27`
- Replace CI and Codecov extraction from `.claude/skills/fix-pr/SKILL.md:64`

Keep:

- triage order
- whether a review comment is correct
- what code/test changes to make

### `review-implementation`

Shrink these sections:

- Replace change-scope detection from `.claude/skills/review-implementation/SKILL.md:19`
- Replace linked-issue context fetch from `.claude/skills/review-implementation/SKILL.md:53`

Keep:

- dispatching subagents
- interpreting structural and semantic findings
- deciding which findings to auto-fix

---

## Migration Sequence

### Task 1: Consolidate board logic

- [x] Move reusable logic from `project_board_poll.py` and `project_board_recover.py` into `pipeline_board.py`
- [x] Keep old scripts as wrappers during migration
- [x] Add tests for ready/review/final-review selection and status moves

### Task 2: Add PR snapshot script

- [x] Implement `pipeline_pr.py snapshot/comments/ci/wait-ci/codecov`
- [x] Add fixtures for PR metadata and comment parsing
- [x] Update `fix-pr` and `review-pipeline` docs to call the script

### Task 3: Add worktree script

- [x] Implement PR checkout from forks using pull refs
- [x] Implement merge-main with structured conflict output
- [x] Add tests around path/branch naming and result formatting

### Task 4: Add deterministic review checks

- [x] Implement `detect-scope`
- [x] Implement `file-whitelist`
- [x] Implement `completeness`
- [x] Point `final-review` and `review-implementation` at the new checks

### Task 5: Thin the skills

- [x] Rewrite `project-pipeline` to call scripts for board/worktree actions
- [x] Rewrite `review-pipeline` to call scripts for board selection, PR context, CI wait, and cleanup
- [x] Rewrite `final-review`, `issue-to-pr`, and `fix-pr` similarly
- [x] Remove long shell snippets that duplicate script behavior

### Task 6: Makefile cleanup

- [x] Add board/pr/worktree helper targets
- [x] Keep existing public targets stable
- [x] Make prompt generation consume script output instead of duplicating selection logic in prose

---

## Testing Strategy

- `scripts/test_pipeline_board.py`
  Covers ready/review/final selection, stale PRs, ambiguous cards, status transitions.
- `scripts/test_pipeline_pr.py`
  Covers comment parsing, linked-issue extraction, CI summary, Codecov parsing.
- `scripts/test_pipeline_worktree.py`
  Covers branch naming, pull-ref checkout planning, merge-main result formatting.
- `scripts/test_pipeline_checks.py`
  Covers scope detection, whitelist results, completeness checklist generation.

Verification commands:

- `python3 -m unittest scripts/test_pipeline_board.py`
- `python3 -m unittest scripts/test_pipeline_pr.py`
- `python3 -m unittest scripts/test_pipeline_worktree.py`
- `python3 -m unittest scripts/test_pipeline_checks.py`

---

## Risks and Constraints

- Do not encode subjective ranking or usefulness heuristics too aggressively in scripts.
- Keep script output JSON stable so skills and `Makefile` targets do not drift.
- Preserve current public commands (`make run-pipeline`, `make run-review`) so maintainers do not have to relearn the interface.
- Maintain a strict boundary: scripts gather facts, skills make judgments.

---

## Expected End State

- Skills are shorter, easier to read, and focused on reasoning.
- Python scripts are testable and reusable from both `Makefile` and skills.
- `Makefile` remains the single ergonomic entrypoint for humans.
- Board selection and PR context gathering stop being duplicated across multiple `SKILL.md` files.
