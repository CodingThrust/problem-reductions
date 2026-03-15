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

This skill runs **fully autonomously** -- no confirmation prompts, no user questions.

## Steps

### 0. Discover Review pool Items

```bash
gh project item-list 8 --owner CodingThrust --format json --limit 500
```

Filter items where `status == "Review pool"`. Each item should have an associated PR. Extract the PR number from the item title or linked issue.

#### 0a. Check Copilot Review Status

For each candidate PR, check whether Copilot has already submitted a review:

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
gh api repos/$REPO/pulls/$PR/reviews --jq '[.[] | select(.user.login == "copilot-pull-request-reviewer[bot]")] | length'
```

A PR is **eligible** only if the count is ≥ 1 (Copilot has submitted at least one review). PRs without a Copilot review yet are marked `[waiting for Copilot]` and skipped.

#### 0b. Print the List

Print all Review pool items with their Copilot status:

```
Review pool PRs:
  #570  Fix #117: [Model] GraphPartitioning     [copilot reviewed]
  #571  Fix #97: [Rule] BinPacking to ILP       [waiting for Copilot]
```

**If a specific PR number was provided:** verify it is in the Review pool column. If it is waiting for Copilot, STOP with a message: `PR #N is waiting for Copilot review. Re-run after Copilot has reviewed.`

**If `--all`:** process only eligible (Copilot-reviewed) items in order (lowest PR number first). Skip waiting items.

**Otherwise:** pick the first eligible item. If no items are eligible, STOP with: `No Review pool PRs have been reviewed by Copilot yet.`

### 0g. Claim: Move to "Under review"

**Immediately** move the chosen PR to the `Under review` column to signal that an agent is actively working on it. This prevents other agents from picking the same PR:

```bash
gh project item-edit \
  --id <ITEM_ID> \
  --project-id PVT_kwDOBrtarc4BRNVy \
  --field-id PVTSSF_lADOBrtarc4BRNVyzg_GmQc \
  --single-select-option-id f04790ca
```

In `--all` mode, claim each PR right before processing it (not all at once).

### 1. Create Worktree and Checkout PR Branch

Create an isolated git worktree so the main working directory stays clean:

```bash
REPO_ROOT=$(git rev-parse --show-toplevel)
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
BRANCH=$(gh pr view $PR --json headRefName --jq .headRefName)
WORKTREE_DIR=".worktrees/review-$BRANCH"
mkdir -p .worktrees
git fetch origin $BRANCH
git worktree add "$WORKTREE_DIR" $BRANCH
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
git fetch origin main
git merge origin/main --no-edit
```

- If the merge succeeds cleanly: push the merge commit and continue.
- If there are conflicts:
  1. Inspect the conflicting files with `git diff --name-only --diff-filter=U`.
  2. Compare the current skill versions on main vs the PR branch to understand which patterns are current.
  3. Resolve conflicts (prefer main's patterns for skill-generated code, the PR branch for problem-specific logic, main for regenerated artifacts like JSON).
  4. Stage resolved files, commit, and push.
- If conflicts are too complex to resolve automatically (e.g., overlapping logic changes in the same function):
  1. Abort the merge: `git merge --abort`
  2. Move the project item back to `Review pool`:
     ```bash
     gh project item-edit \
       --id <ITEM_ID> \
       --project-id PVT_kwDOBrtarc4BRNVy \
       --field-id PVTSSF_lADOBrtarc4BRNVyzg_GmQc \
       --single-select-option-id 7082ed60
     ```
  3. Report: `PR #N has complex merge conflicts with main — needs manual resolution.`
  4. STOP processing this PR.

### 2. Fix Copilot Review Comments

Copilot review is guaranteed to exist (verified in Step 0). Fetch the comments:

```bash
COMMENTS=$(gh api repos/$REPO/pulls/$PR/comments --jq '[.[] | select(.user.login == "copilot-pull-request-reviewer[bot]")]')
```

If there are actionable comments: invoke `/fix-pr` to address them, then push:

```bash
git push
```

If Copilot approved with no actionable comments: skip to next step.

### 2a. Check Issue Comments and Human PR Reviews

Extract the linked issue number from the PR title (pattern: `Fix #N:`):

```bash
ISSUE=$(gh pr view $PR --json title --jq .title | grep -oP '(?<=Fix #)\d+')
```

Fetch all comment sources:

```bash
# 1. Linked issue comments (from contributors, excluding bots)
if [ -n "$ISSUE" ]; then
    gh api repos/$REPO/issues/$ISSUE/comments | python3 -c "
import sys,json
comments = [c for c in json.load(sys.stdin) if not c['user']['login'].endswith('[bot]')]
print(f'=== Issue #{sys.argv[1]} comments: {len(comments)} ===')
for c in comments:
    print(f'[{c[\"user\"][\"login\"]}] {c[\"body\"][:300]}')
    print('---')
" "$ISSUE"
fi

# 2. Human PR review comments (inline, excluding Copilot)
gh api repos/$REPO/pulls/$PR/comments | python3 -c "
import sys,json
comments = [c for c in json.load(sys.stdin) if not c['user']['login'].endswith('[bot]')]
print(f'=== Human PR inline comments: {len(comments)} ===')
for c in comments:
    line = c.get('line') or c.get('original_line') or '?'
    print(f'[{c[\"user\"][\"login\"]}] {c[\"path\"]}:{line} — {c[\"body\"][:300]}')
"

# 3. Human PR conversation comments (general discussion, excluding bots)
gh api repos/$REPO/issues/$PR/comments | python3 -c "
import sys,json
comments = [c for c in json.load(sys.stdin) if not c['user']['login'].endswith('[bot]')]
print(f'=== Human PR conversation comments: {len(comments)} ===')
for c in comments:
    print(f'[{c[\"user\"][\"login\"]}] {c[\"body\"][:300]}')
"

# 4. Human review-level comments (top-level review body)
gh api repos/$REPO/pulls/$PR/reviews | python3 -c "
import sys,json
reviews = [r for r in json.load(sys.stdin) if not r['user']['login'].endswith('[bot]') and r.get('body')]
print(f'=== Human reviews: {len(reviews)} ===')
for r in reviews:
    print(f'[{r[\"user\"][\"login\"]}] {r[\"state\"]}: {r[\"body\"][:300]}')
"
```

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
   for i in $(seq 1 30); do
       sleep 30
       HEAD_SHA=$(gh api repos/$REPO/pulls/$PR | python3 -c "import sys,json; print(json.load(sys.stdin)['head']['sha'])")
       STATUS=$(gh api repos/$REPO/commits/$HEAD_SHA/check-runs | python3 -c "
   import sys,json
   runs = json.load(sys.stdin)['check_runs']
   if not runs:
       print('PENDING')
   else:
       failed = [r['name'] for r in runs if r.get('conclusion') not in ('success', 'skipped', None)]
       pending = [r['name'] for r in runs if r.get('conclusion') is None and r['status'] != 'completed']
       if pending:
           print('PENDING')
       elif failed:
           print('FAILED')
       else:
           print('GREEN')
   ")
       if [ "$STATUS" != "PENDING" ]; then break; fi
   done
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
gh project item-edit \
  --id <ITEM_ID> \
  --project-id PVT_kwDOBrtarc4BRNVy \
  --field-id PVTSSF_lADOBrtarc4BRNVyzg_GmQc \
  --single-select-option-id 51a3d8bb
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
