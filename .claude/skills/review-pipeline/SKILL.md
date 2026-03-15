---
name: review-pipeline
description: Pick a PR from the Review pool board column, fix Copilot review comments, check issue/human comments, fix CI, run agentic feature tests, then move to Final review
---

# Review Pipeline

Pick PRs from the `Review pool` column on the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1). For each PR: wait for Copilot review, fix Copilot comments, check and address issue/human comments, fix CI, run agentic feature tests, then move to `Final review`.

## Invocation

- `/review-pipeline` -- pick the next Review pool item
- `/review-pipeline 570` -- process a specific PR number
- `/review-pipeline --all` -- batch-process all Review pool items

For Codex, open this `SKILL.md` directly and treat the slash-command forms above as aliases. The Makefile `run-review` target already does this translation.

## Constants

GitHub Project board IDs (for `gh project item-edit`):

| Constant | Value |
|----------|-------|
| `PROJECT_ID` | `PVT_kwDOBrtarc4BRNVy` |
| `STATUS_FIELD_ID` | `PVTSSF_lADOBrtarc4BRNVyzg_GmQc` |
| `STATUS_REVIEW_POOL` | `7082ed60` |
| `STATUS_UNDER_REVIEW` | `f04790ca` |
| `STATUS_FINAL_REVIEW` | `51a3d8bb` |
| `STATUS_READY` | `f37d0d80` |

## Prerequisites

- **agentic-tests** must be installed (`~/.claude/commands/agentic-tests:test-feature.md` must exist). If missing, STOP with: `agentic-tests not installed. Run: gh clone GiggleLiu/agentic-tests ~/.claude/agentic-tests && mkdir -p ~/.claude/commands && ln -s ~/.claude/agentic-tests/skills/test-feature/SKILL.md ~/.claude/commands/agentic-tests:test-feature.md`

## Autonomous Mode

This skill runs **fully autonomously** except for one case: if a Review pool card links multiple PRs in `CodingThrust/problem-reductions` and the user did not specify which PR to process, STOP and ask the user which PR is the intended target.

## Steps

### 0. Discover Review pool Items

```bash
gh project item-list 8 --owner CodingThrust --format json --limit 500
```

Filter items where `status == "Review pool"`. Each item should have an associated PR. Extract the PR number from the item title or linked issue.

Before treating an item as a candidate:

- Only consider PRs in `CodingThrust/problem-reductions` whose GitHub state is `OPEN`.
- If a Review pool **PR card** points to a PR that is `CLOSED` or `MERGED`, mark it stale and skip it.
- If a Review pool **issue card** has multiple linked repo PRs, treat the card as **ambiguous**. Do not guess which PR to process.
- If a Review pool issue card has exactly one linked repo PR, require that PR to be `OPEN`.
- If a Review pool issue card has no linked repo PRs, resolve the current open PR from the issue number as before.

Stale links must be cleaned up at the PR level (for example, remove outdated `Closes #...` references from superseded PRs) rather than by moving the board card.

#### 0a. Check Copilot Review Status

For each candidate PR that passed the open/ambiguity checks above, check whether Copilot has already submitted a review:

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
gh api repos/$REPO/pulls/$PR/reviews --jq '[.[] | select(.user.login == "copilot-pull-request-reviewer[bot]")] | length'
```

A PR is **eligible** only if:

1. The PR state is `OPEN`
2. The Review pool card is unambiguous, or the user explicitly chose the PR from an ambiguous card
3. The Copilot review count is ≥ 1

PRs without a Copilot review yet are marked `[waiting for Copilot]` and skipped. Closed/merged PRs are marked `[stale closed PR]` and skipped. Mixed-status or multi-PR issue cards are marked `[ambiguous linked PRs]` and require user confirmation before proceeding.

#### 0b. Print the List

Print all Review pool items with their Copilot status:

```
Review pool PRs:
  #570  Fix #117: [Model] GraphPartitioning     [copilot reviewed]
  #571  Fix #97: [Rule] BinPacking to ILP       [waiting for Copilot]
  #170  Fix #108: [Model] LongestCommonSubsequence [stale closed PR]
  issue #108 [Model] LongestCommonSubsequence   [ambiguous linked PRs]
```

**If a specific PR number was provided:** verify it is in the Review pool column and is `OPEN`. If it is waiting for Copilot, STOP with a message: `PR #N is waiting for Copilot review. Re-run after Copilot has reviewed.` If it is closed, STOP with: `PR #N is closed and not eligible for review-pipeline.` A specific PR number counts as the user's disambiguation choice for a multi-PR card.

**If `--all`:** process only eligible (open, unambiguous, Copilot-reviewed) items in order (lowest PR number first). Skip waiting, stale, and ambiguous items.

**Otherwise:** pick the first eligible item. If no items are eligible, STOP with: `No Review pool PRs have been reviewed by Copilot yet.`

**If the first candidate card is ambiguous:** STOP and ask the user which PR to process. Show short options and recommend the most likely correct one:

```text
Review pool card [Model] LongestCommonSubsequence links multiple repo PRs:
1. PR #170 — CLOSED — Superseded: Add LongestCommonSubsequence model
2. PR #173 — OPEN — Fix #109: Add LCS to MaximumIndependentSet reduction  (Recommended)

Recommendation rule:
- Prefer the only OPEN repo PR when all others are closed or merged.
- If multiple repo PRs are OPEN, prefer the one whose title/body most directly matches the card's current work, and say that the card still needs stale-link cleanup.
```

### 0g. Claim: Move to "Under review"

**Immediately** move the chosen PR to the `Under review` column to signal that an agent is actively working on it. This prevents other agents from picking the same PR:

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> under-review
```

In `--all` mode, claim each PR right before processing it (not all at once).

### 1. Create Worktree and Checkout PR Branch

Create an isolated git worktree so the main working directory stays clean:

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
WORKTREE=$(python3 scripts/pipeline_worktree.py checkout-pr --repo "$REPO" --pr "$PR" --format json)
WORKTREE_DIR=$(printf '%s\n' "$WORKTREE" | python3 -c "import sys,json; print(json.load(sys.stdin)['worktree_dir'])")
cd "$WORKTREE_DIR"
```

All subsequent steps run inside the worktree.

### 1a. Resolve Conflicts with Main

**IMPORTANT:** The `add-model` and `add-rule` skills evolve frequently. When merging main into a PR branch, conflicts in skill-generated code are common. Before resolving conflicts:

1. Run `git diff origin/main...HEAD -- .claude/skills/add-model/ .claude/skills/add-rule/` to see if these skills changed on main since the PR was created.
2. If they changed, read the current versions on main (`git show origin/main:.claude/skills/add-model/SKILL.md` and `git show origin/main:.claude/skills/add-rule/SKILL.md`) to understand what's different.
3. When resolving conflicts in model/rule implementation files, prefer the patterns from main's current skills — the PR's implementation may be based on outdated skill instructions.

Check if the branch has merge conflicts with main:

```bash
MERGE=$(python3 scripts/pipeline_worktree.py merge-main --worktree "$WORKTREE_DIR" --format json)
MERGE_STATUS=$(printf '%s\n' "$MERGE" | python3 -c "import sys,json; print(json.load(sys.stdin)['status'])")
```

- If `MERGE_STATUS == clean`: push the merge commit and continue.
- If there are conflicts:
  1. Inspect the conflicting files from the `conflicts` array in `MERGE`.
  2. Compare the current skill versions on main vs the PR branch to understand which patterns are current.
  3. Resolve conflicts (prefer main's patterns for skill-generated code, the PR branch for problem-specific logic, main for regenerated artifacts like JSON).
  4. Stage resolved files, commit, and push.
- If `MERGE_STATUS == conflicted` and `likely_complex == true` (or the overlap is otherwise too complex to resolve automatically):
  1. Abort the merge: `git merge --abort` if a merge is still in progress
  2. Move the project item back to `Review pool`:
     ```bash
     python3 scripts/pipeline_board.py move <ITEM_ID> review-pool
     ```
  3. Report: `PR #N has complex merge conflicts with main — needs manual resolution.`
  4. STOP processing this PR.

### 2. Fix Copilot Review Comments

Copilot review is guaranteed to exist (verified in Step 0). Fetch the comments:

```bash
COMMENTS=$(python3 scripts/pipeline_pr.py comments --repo "$REPO" --pr "$PR" --format json)
```

Inspect the `copilot_inline_comments` array in `COMMENTS`. If there are actionable comments: invoke `/fix-pr` to address them, then push:

```bash
git push
```

If Copilot approved with no actionable comments: skip to next step.

### 2a. Check Issue Comments and Human PR Reviews

Reuse the `COMMENTS` JSON from Step 2. It already includes:
- `linked_issue_number`
- `human_linked_issue_comments`
- `human_inline_comments`
- `human_issue_comments`
- `human_reviews`

For each actionable comment found:

1. **Read the relevant source files** referenced by the comment.
2. **Check if the comment's feedback is already addressed** in the current code.
3. **If not addressed:** fix the code to respect the comment, commit, and push.
4. **If already addressed:** move on.

Actionable comments include: code suggestions, bug reports, requests for additional tests, naming feedback, algorithmic corrections, and missing edge cases. Ignore comments that are purely informational or questions that have already been answered.

If there are no actionable unaddressed comments: skip to next step.

### 2b. Structural Completeness Check (REQUIRED)

Run `/review-implementation` to catch structural gaps (missing paper entries, missing registrations, missing tests) that Copilot and human reviewers may not flag:

```
/review-implementation
```

This dispatches structural + quality subagents with fresh context. If findings include FAIL items:

1. **Auto-fix** structural FAILs (missing registrations, missing test files, etc.)
2. **For missing paper entries** (checks #15/#16 for models, check #14 for rules): invoke `/write-model-in-paper` or `/write-rule-in-paper` as appropriate — do NOT skip these as "unfixable"
3. **Commit and push** all fixes before proceeding

If all structural checks pass: continue to next step.

### 3. Agentic Feature Test (REQUIRED)

**This step is mandatory — do NOT skip or substitute with manual testing.**

Run agentic feature tests on the modified feature:

1. **Identify the feature** from the PR title and changed files:
   - `[Model]` PRs: the new problem model name
   - `[Rule]` PRs: the new reduction rule (source -> target)

2. **Invoke `/agentic-tests:test-feature`** with the identified feature. This simulates a downstream user exercising the feature from docs and examples. You MUST use the Skill tool to invoke `agentic-tests:test-feature`.

3. **If test-feature reports issues:** fix them, commit, and push.

4. **If test-feature passes:** continue to next step.

### 4. Fix Loop (max 3 retries)

For each retry:

1. **Wait for CI to complete** (poll every 30s, up to 15 minutes):
   ```bash
   CI=$(python3 scripts/pipeline_pr.py wait-ci --repo "$REPO" --pr "$PR" --timeout 900 --interval 30 --format json)
   STATUS=$(printf '%s\n' "$CI" | python3 -c "
import sys,json
state = json.load(sys.stdin)['state']
mapping = {'success': 'GREEN', 'failure': 'FAILED', 'timeout': 'FAILED', 'pending': 'PENDING'}
print(mapping.get(state, 'FAILED'))
")
   ```

   - If `GREEN` on the **first** iteration (before any fix-pr): skip the fix loop, done.
   - If `GREEN` after a fix-pr pass: break, done.
   - If `FAILED`: continue to step 2.
   - If still `PENDING` after 15 min: treat as `FAILED`.

2. **Invoke `/fix-pr`** to address CI failures and coverage gaps.

3. **Push fixes:**
   ```bash
   git push
   ```

4. Increment retry counter. If `< 3`, go back to step 1. If `= 3`, give up.

**After 3 failed retries:** leave PR open, still move to Final review for human triage.

### 5. Clean Up Worktree

```bash
cd "$REPO_ROOT"
git worktree remove "$WORKTREE_DIR" --force
```

### 6. Move to "Final review"

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> final-review
```

### 7. Report

Post the review summary as a PR comment so it's visible to human reviewers:

```bash
gh pr comment $PR --body "$(cat <<'EOF'
## Review Pipeline Report

| Check | Result |
|-------|--------|
| Copilot comments | 3 fixed |
| Issue/human comments | 2 checked, 1 fixed |
| Structural review | 17/17 passed |
| CI | green |
| Agentic test | passed |
| Board | Review pool → Under review → Final review |

🤖 Generated by review-pipeline
EOF
)"
```

Adapt the table values to match the actual results for the PR. If CI failed after 3 retries, report `failed (3 retries)` instead of `green`.

### 8. Batch Mode (`--all`)

If `--all` was specified, repeat Steps 1-7 for each PR (including posting a PR comment per Step 7). After all PRs, print a batch summary to console:

```
=== Review Pipeline Batch Report ===

| PR   | Title                              | Copilot | Issue/Human | Structural | CI      | Agentic Test | Board      |
|------|------------------------------------|---------|-------------|------------|---------|--------------|------------|
| #570 | Fix #117: [Model] GraphPartitioning| 3 fixed | 1 fixed     | 17/17      | green   | passed       | Final review  |
| #571 | Fix #97: [Rule] BinPacking to ILP  | 0       | 0           | 14/14      | green   | passed       | Final review  |

Completed: 2/2 | All moved to Final review
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| PR not in Review pool column | Verify status before processing; STOP if not Review pool |
| Processing a closed PR from a stale issue card | Require PR state `OPEN`; skip stale closed PRs |
| Guessing on an issue card with multiple linked repo PRs | Stop, show options to the user, and recommend the most likely correct OPEN PR |
| Picking a PR before Copilot has reviewed | Check `pulls/$PR/reviews` for copilot-pull-request-reviewer[bot]; skip if absent |
| Missing project scopes | Run `gh auth refresh -s read:project,project` |
| Skipping review-implementation | Always run structural completeness check in Step 2b — it catches gaps Copilot misses (paper entries, CLI registration, trait_consistency) |
| Skipping agentic tests | Always run test-feature even if CI is green |
| Not checking out the right branch | Use `gh pr view` to get the exact branch name |
| Worktree left behind on failure | Always clean up with `git worktree remove` in Step 5 |
| Working in main checkout | All work happens in `.worktrees/` — never modify the main checkout |
| Skipping merge with main | Always merge origin/main in Step 1a to catch conflicts before fixing comments |
| Ignoring issue comments | Always check the linked issue (`Fix #N`) for human feedback in Step 2a |
| Only checking Copilot comments | Step 2a checks human PR reviews and linked issue comments too — bot-only review is insufficient |
