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

Collect the scripted candidate list first:

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
CANDIDATES=$(python3 scripts/pipeline_board.py list review --repo "$REPO" --format json)
```

Inspect `CANDIDATES["items"]`. Each item includes:
- `item_id`, `title`, `issue_number`, `pr_number`, `eligibility`, `reason`
- for ambiguous cards: `linked_repo_prs` and `recommendation`

The scripted `eligibility` values are:
- `eligible`
- `waiting-for-copilot`
- `stale-closed-pr`
- `ambiguous-linked-prs`
- `no-open-pr`

#### 0b. Print the List

Print the candidate list using the scripted eligibility:

```
Review pool PRs:
  #570  Fix #117: [Model] GraphPartitioning     [eligible]
  #571  Fix #97: [Rule] BinPacking to ILP       [waiting-for-copilot]
  #170  Fix #108: [Model] LongestCommonSubsequence [stale-closed-pr]
  issue #108 [Model] LongestCommonSubsequence   [ambiguous-linked-prs]
```

**If a specific PR number was provided:** find the matching entry in `CANDIDATES["items"]` by `pr_number`. If its `eligibility` is:
- `eligible`: continue
- `waiting-for-copilot`: STOP with `PR #N is waiting for Copilot review. Re-run after Copilot has reviewed.`
- `stale-closed-pr`: STOP with `PR #N is closed and not eligible for review-pipeline.`
- `ambiguous-linked-prs`: this specific PR number counts as the user's disambiguation choice. Use that PR number, but keep the card marked as ambiguous until stale links are cleaned up.
- missing from the list: STOP with `PR #N is not currently in the Review pool candidate set.`

If the matched entry has `eligibility == "ambiguous-linked-prs"`, keep `PR` set to the chosen PR number, extract `ITEM_ID`, `TITLE`, and `ISSUE` from the matching candidate entry, and use the manual claim fallback in Step 0g because the scripted queue intentionally skips ambiguous cards.

Otherwise, claim the item into `Under review` through the scripted bundle:

```bash
STATE_FILE=/tmp/problemreductions-review-selection.json
CLAIM=$(python3 scripts/pipeline_board.py claim-next review "$STATE_FILE" --repo "$REPO" --number "$PR" --format json)
```

If the command exits with status 1, STOP with: `PR #N is not currently in the Review pool candidate set.`

Extract `ITEM_ID`, `TITLE`, and `ISSUE` from `CLAIM`, and keep `PR` set to the chosen PR number.

**If `--all`:** process only entries where `eligibility == "eligible"` in order (lowest PR number first). Skip waiting, stale, no-open-pr, and ambiguous items.

**Otherwise:** claim the next eligible item through the scripted queue:

```bash
STATE_FILE=/tmp/problemreductions-review-selection.json
CLAIM=$(python3 scripts/pipeline_board.py claim-next review "$STATE_FILE" --repo "$REPO" --format json)
```

If the command exits with status 1, STOP with: `No Review pool PRs are currently eligible for review-pipeline.`

Extract the board item and PR metadata from `CLAIM`:

```bash
ITEM_ID=$(printf '%s\n' "$CLAIM" | python3 -c "import sys,json; print(json.load(sys.stdin)['item_id'])")
PR=$(printf '%s\n' "$CLAIM" | python3 -c "import sys,json; data=json.load(sys.stdin); print(data['pr_number'] or data['number'])")
TITLE=$(printf '%s\n' "$CLAIM" | python3 -c "import sys,json; print(json.load(sys.stdin)['title'])")
ISSUE=$(printf '%s\n' "$CLAIM" | python3 -c "import sys,json; data=json.load(sys.stdin); print(data['issue_number'] or '')")
```

**If `CANDIDATES["items"]` contains an ambiguous card that you need to resolve manually:** STOP and ask the user which PR to process. Show short options from `linked_repo_prs` and recommend `recommendation` when present:

```text
Review pool card [Model] LongestCommonSubsequence links multiple repo PRs:
1. PR #170 — CLOSED — Superseded: Add LongestCommonSubsequence model
2. PR #173 — OPEN — Fix #109: Add LCS to MaximumIndependentSet reduction  (Recommended)

Recommendation rule:
- Prefer the only OPEN repo PR when all others are closed or merged.
- If multiple repo PRs are OPEN, prefer the one whose title/body most directly matches the card's current work, and say that the card still needs stale-link cleanup.
```

### 0g. Claim Result

If you used `claim-next review`, the scripted bundle has already moved the chosen item to `Under review`, which prevents another agent from picking the same PR.

If you reached this point through the ambiguous-card path above, manually claim it now:

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> under-review
```

In `--all` mode, claim each PR right before processing it (not all at once).

### 1. Prepare Review Worktree

Create an isolated git worktree and attempt the `origin/main` merge in one scripted step so the main working directory stays clean:

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
PREP=$(python3 scripts/pipeline_worktree.py prepare-review --repo "$REPO" --pr "$PR" --format json)
WORKTREE_DIR=$(printf '%s\n' "$PREP" | python3 -c "import sys,json; print(json.load(sys.stdin)['checkout']['worktree_dir'])")
cd "$WORKTREE_DIR"
```

All subsequent steps run inside the worktree.

### 1a. Resolve Conflicts with Main

**IMPORTANT:** The `add-model` and `add-rule` skills evolve frequently. When merging main into a PR branch, conflicts in skill-generated code are common. Before resolving conflicts:

1. Run `git diff origin/main...HEAD -- .claude/skills/add-model/ .claude/skills/add-rule/` to see if these skills changed on main since the PR was created.
2. If they changed, read the current versions on main (`git show origin/main:.claude/skills/add-model/SKILL.md` and `git show origin/main:.claude/skills/add-rule/SKILL.md`) to understand what's different.
3. When resolving conflicts in model/rule implementation files, prefer the patterns from main's current skills — the PR's implementation may be based on outdated skill instructions.

Read the bundled merge result:

```bash
MERGE_STATUS=$(printf '%s\n' "$PREP" | python3 -c "import sys,json; print(json.load(sys.stdin)['merge']['status'])")
LIKELY_COMPLEX=$(printf '%s\n' "$PREP" | python3 -c "import sys,json; print(str(json.load(sys.stdin)['merge']['likely_complex']).lower())")
```

- If `MERGE_STATUS == clean`: push the merge commit and continue.
- If there are conflicts:
  1. Inspect the conflicting files from the `conflicts` array in `PREP["merge"]`.
  2. Compare the current skill versions on main vs the PR branch to understand which patterns are current.
  3. Resolve conflicts (prefer main's patterns for skill-generated code, the PR branch for problem-specific logic, main for regenerated artifacts like JSON).
  4. Stage resolved files, commit, and push.
- If `MERGE_STATUS == conflicted` and `LIKELY_COMPLEX == true` (or the overlap is otherwise too complex to resolve automatically):
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

3. **If test-feature reports issues:** treat every reported issue as real until you have checked it in the **current PR worktree/branch**.
   - Reproduce each issue from the current PR branch/worktree before acting. If it does not reproduce there, classify it as `not reproducible in current worktree`.
   - Auto-fix every objective issue you reasonably can: code bugs, tests, docs, help text, examples, discoverability gaps, and validation/error-message problems. Do **not** leave "minor docs issues" unfixed by default.
   - If you changed user-facing behavior, docs, or CLI help, re-run `/agentic-tests:test-feature`.
   - Classify every reported issue as exactly one of:
     - `fixed`
     - `not reproducible in current worktree`
     - `needs human decision`

4. **Only `needs human decision` issues may remain unresolved.** If any remain:
   - continue to the next step only after you have written them down for the final PR report
   - include why they were not auto-fixed
   - include your recommended maintainer decision

5. **If test-feature passes with no remaining issues:** continue to next step.

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
COMMENT_FILE=$(mktemp)
cat > "$COMMENT_FILE" <<'EOF'
## Review Pipeline Report

| Check | Result |
|-------|--------|
| Copilot comments | 3 fixed |
| Issue/human comments | 2 checked, 1 fixed |
| Structural review | 17/17 passed |
| CI | green |
| Agentic test | passed |
| Needs human decision | none |
| Board | Review pool → Under review → Final review |

### Remaining issues for final review

- None.

🤖 Generated by review-pipeline
EOF
python3 scripts/pipeline_pr.py comment --repo "$REPO" --pr "$PR" --body-file "$COMMENT_FILE"
rm -f "$COMMENT_FILE"
```

Adapt the table values to match the actual results for the PR. If CI failed after 3 retries, report `failed (3 retries)` instead of `green`.

This section is **mandatory**:
- `Needs human decision` must be `none` or `N item(s)`
- `### Remaining issues for final review` must always be present
- If there are unresolved issues, list each one as a bullet with:
  - the concrete problem
  - why it was not auto-fixed
  - the recommended maintainer decision

If unresolved issues remain, do **not** write `Agentic test | passed`. Use `passed with notes` or another accurate status.

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
| Saying "passed" while deferring issues | If anything remains for maintainer judgment, list it explicitly under `Remaining issues for final review` and mark the agentic result accordingly |
