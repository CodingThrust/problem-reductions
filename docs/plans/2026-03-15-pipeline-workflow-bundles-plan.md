# Pipeline Workflow Bundles Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Collapse overly fine-grained pipeline helper calls into a few workflow-stage bundles so the pipeline skills mostly consume one JSON payload per mechanical stage.

**Architecture:** Keep the existing low-level primitives in `pipeline_board.py`, `pipeline_pr.py`, `pipeline_worktree.py`, and `pipeline_checks.py` for testability and debugging, but add higher-level subcommands at the points where the skills naturally branch. Skills should read a bundled payload for selection, branch/worktree prep, PR context, and deterministic review checks, then do only judgment-heavy work in prose.

**Tech Stack:** Python 3, `gh` CLI, `git`, `unittest`, repo-local `SKILL.md` docs, Makefile shell helpers

---

## Chunk 1: PR Context Bundle

### Task 1: Add `pipeline_pr.py context`

**Files:**
- Modify: `scripts/pipeline_pr.py`
- Modify: `scripts/test_pipeline_pr.py`
- Modify: `.claude/skills/fix-pr/SKILL.md`
- Modify: `.claude/skills/final-review/SKILL.md`

- [x] **Step 1: Write the failing tests for the context builder**

Add tests that prove `context` returns one consumer-friendly JSON object containing:
- repo + PR identity
- snapshot-derived metadata (`title`, `body`, `files`, `mergeable`, `head_sha`)
- comment summary
- CI summary
- Codecov summary
- linked issue data and `issue_context_text`

- [x] **Step 2: Run the focused PR helper tests to verify failure**

Run: `python3 scripts/test_pipeline_pr.py`
Expected: FAIL because `context` helpers / CLI are not implemented yet.

- [x] **Step 3: Implement the minimal `context` command**

In `scripts/pipeline_pr.py`:
- add a pure builder that merges the existing `snapshot`, `comments`, `ci`, `codecov`, and `linked-issue` data into one payload
- add `context --repo <repo> --pr <n> --format json`
- add `context --current --format json`

- [x] **Step 4: Run the PR helper tests to verify they pass**

Run: `python3 scripts/test_pipeline_pr.py`
Expected: PASS

- [x] **Step 5: Rewrite the highest-value skill consumers**

Update:
- `fix-pr` Step 1 to use `CONTEXT=$(python3 scripts/pipeline_pr.py context --current --format json)`
- `final-review` Step 1 to use `CONTEXT=$(python3 scripts/pipeline_pr.py context --repo "$REPO" --pr <number> --format json)` for mechanical data loading

- [x] **Step 6: Re-run targeted verification**

Run:
- `python3 scripts/test_pipeline_pr.py`
- `python3 scripts/pipeline_pr.py context --help`

- [x] **Step 7: Commit**

```bash
git add scripts/pipeline_pr.py scripts/test_pipeline_pr.py .claude/skills/fix-pr/SKILL.md .claude/skills/final-review/SKILL.md
git commit -m "refactor(pipeline): bundle PR context loading"
```

## Chunk 2: Review Context Bundle

### Task 2: Add `pipeline_checks.py review-context`

**Files:**
- Modify: `scripts/pipeline_checks.py`
- Modify: `scripts/test_pipeline_checks.py`
- Modify: `.claude/skills/review-implementation/SKILL.md`
- Modify: `.claude/skills/final-review/SKILL.md`

- [x] **Step 1: Write the failing tests for `review-context`**

Cover:
- `detect-scope` output inclusion
- changed file list + diff stat inclusion
- whitelist result inclusion when subject kind is known
- completeness result inclusion when subject kind is known

- [x] **Step 2: Run the focused checks tests to verify failure**

Run: `python3 scripts/test_pipeline_checks.py`
Expected: FAIL because `review-context` does not exist yet.

- [x] **Step 3: Implement the minimal bundle**

In `scripts/pipeline_checks.py`:
- add `review-context --repo-root . --base <sha> --head <sha> --format json`
- support either inferred subject (`auto`) or explicit `--kind/--name/--source/--target`
- include `scope`, `diff_stat`, `changed_files`, `whitelist`, and `completeness`

- [x] **Step 4: Run the checks tests to verify they pass**

Run: `python3 scripts/test_pipeline_checks.py`
Expected: PASS

- [x] **Step 5: Rewrite the skill consumers**

Update:
- `review-implementation` Step 1 / Step 2 to use `review-context`
- `final-review` deterministic whitelist/completeness steps to use `review-context`

- [x] **Step 6: Re-run targeted verification**

Run:
- `python3 scripts/test_pipeline_checks.py`
- `python3 scripts/pipeline_checks.py review-context --help`

- [x] **Step 7: Commit**

```bash
git add scripts/pipeline_checks.py scripts/test_pipeline_checks.py .claude/skills/review-implementation/SKILL.md .claude/skills/final-review/SKILL.md
git commit -m "refactor(pipeline): bundle deterministic review checks"
```

## Chunk 3: Review Worktree Bundle

### Task 3: Add `pipeline_worktree.py prepare-review`

**Files:**
- Modify: `scripts/pipeline_worktree.py`
- Modify: `scripts/test_pipeline_worktree.py`
- Modify: `.claude/skills/review-pipeline/SKILL.md`
- Modify: `.claude/skills/final-review/SKILL.md`

- [x] **Step 1: Write the failing tests for `prepare-review`**

Cover:
- `checkout-pr` + `merge-main` combined success path
- conflict path preserves `conflicts` and `likely_complex`
- result contains `ready=true|false`

- [x] **Step 2: Run the worktree tests to verify failure**

Run: `python3 scripts/test_pipeline_worktree.py`
Expected: FAIL because `prepare-review` does not exist yet.

- [x] **Step 3: Implement the minimal bundle**

In `scripts/pipeline_worktree.py`:
- add `prepare-review --repo <repo> --pr <n> --format json`
- call existing `checkout-pr`
- call existing `merge-main`
- return one payload with checkout + merge result

- [x] **Step 4: Run the worktree tests to verify they pass**

Run: `python3 scripts/test_pipeline_worktree.py`
Expected: PASS

- [x] **Step 5: Rewrite skill consumers**

Update:
- `review-pipeline` checkout + merge sections
- `final-review` merge-conflict preflight

- [x] **Step 6: Re-run targeted verification**

Run:
- `python3 scripts/test_pipeline_worktree.py`
- `python3 scripts/pipeline_worktree.py prepare-review --help`

- [x] **Step 7: Commit**

```bash
git add scripts/pipeline_worktree.py scripts/test_pipeline_worktree.py .claude/skills/review-pipeline/SKILL.md .claude/skills/final-review/SKILL.md
git commit -m "refactor(pipeline): bundle review worktree preparation"
```

## Chunk 4: Queue Claim Bundle

### Task 4: Add `pipeline_board.py claim-next`

**Files:**
- Modify: `scripts/pipeline_board.py`
- Modify: `scripts/test_pipeline_board.py`
- Modify: `.claude/skills/project-pipeline/SKILL.md`
- Modify: `.claude/skills/review-pipeline/SKILL.md`
- Modify: `Makefile`
- Modify: `scripts/make_helpers.sh`

- [x] **Step 1: Write the failing tests for `claim-next`**

Cover:
- ready mode claims and moves to `in-progress`
- review mode claims and moves to `under-review`
- structured result preserves the selected item metadata

- [x] **Step 2: Run the board tests to verify failure**

Run: `python3 scripts/test_pipeline_board.py`
Expected: FAIL because `claim-next` does not exist yet.

- [x] **Step 3: Implement the minimal bundle**

In `scripts/pipeline_board.py`:
- add `claim-next ready <state_file> ...`
- add `claim-next review <state_file> --repo <repo> ...`
- internally call the existing selector and move logic in one place

- [x] **Step 4: Run the board tests to verify they pass**

Run: `python3 scripts/test_pipeline_board.py`
Expected: PASS

- [x] **Step 5: Rewrite skill and Makefile consumers**

Update:
- `project-pipeline` to consume `claim-next ready`
- `review-pipeline` to consume `claim-next review`
- shell helpers if needed for future automation

- [x] **Step 6: Re-run targeted verification**

Run:
- `python3 scripts/test_pipeline_board.py`
- `python3 scripts/test_make_helpers.py`

- [x] **Step 7: Commit**

```bash
git add scripts/pipeline_board.py scripts/test_pipeline_board.py scripts/test_make_helpers.py .claude/skills/project-pipeline/SKILL.md .claude/skills/review-pipeline/SKILL.md Makefile scripts/make_helpers.sh
git commit -m "refactor(pipeline): bundle queue claiming"
```

## Chunk 5: Issue Context Alias

### Task 5: Rename or alias `issue-guards` to `issue-context`

**Files:**
- Modify: `scripts/pipeline_checks.py`
- Modify: `scripts/test_pipeline_checks.py`
- Modify: `.claude/skills/issue-to-pr/SKILL.md`
- Modify: `Makefile`
- Modify: `scripts/make_helpers.sh`

- [ ] **Step 1: Add failing tests for the alias / richer output contract**
- [ ] **Step 2: Run `python3 scripts/test_pipeline_checks.py` and verify failure**
- [ ] **Step 3: Implement `issue-context` as an alias or replacement for `issue-guards`**
- [ ] **Step 4: Rewrite `issue-to-pr` to use the final command name**
- [ ] **Step 5: Re-run `python3 scripts/test_pipeline_checks.py` and `python3 scripts/test_make_helpers.py`**
- [ ] **Step 6: Commit**

```bash
git add scripts/pipeline_checks.py scripts/test_pipeline_checks.py .claude/skills/issue-to-pr/SKILL.md Makefile scripts/make_helpers.sh
git commit -m "refactor(pipeline): standardize issue context loading"
```
