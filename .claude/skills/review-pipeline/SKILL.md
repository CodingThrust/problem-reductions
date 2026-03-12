---
name: review-pipeline
description: Pick a PR from the review-agentic board column, fix Copilot review comments, fix CI, run agentic feature tests, then move to In Review
---

# Review Pipeline

Pick PRs from the `review-agentic` column on the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1). For each PR: wait for Copilot review, fix comments, fix CI, run agentic feature tests, then move to `In Review`.

## Invocation

- `/review-pipeline` -- pick the next review-agentic item
- `/review-pipeline 570` -- process a specific PR number
- `/review-pipeline --all` -- batch-process all review-agentic items

## Constants

GitHub Project board IDs (for `gh project item-edit`):

| Constant | Value |
|----------|-------|
| `PROJECT_ID` | `PVT_kwDOBrtarc4BRNVy` |
| `STATUS_FIELD_ID` | `PVTSSF_lADOBrtarc4BRNVyzg_GmQc` |
| `STATUS_REVIEW_AGENTIC` | `b2f16561` |
| `STATUS_IN_REVIEW` | `df73e18b` |
| `STATUS_READY` | `61e4505c` |

## Autonomous Mode

This skill runs **fully autonomously** -- no confirmation prompts, no user questions.

## Steps

### 0. Discover review-agentic Items

```bash
gh project item-list 8 --owner CodingThrust --format json
```

Filter items where `status == "review-agentic"`. Each item should have an associated PR. Extract the PR number from the item title or linked issue.

Print the list for visibility:

```
review-agentic PRs:
  #570  Fix #117: [Model] GraphPartitioning
  #571  Fix #97: [Rule] BinPacking to ILP
```

**If a specific PR number was provided:** verify it is in the review-agentic column. If not, STOP with a message.

**If `--all`:** process all items in order (lowest PR number first).

**Otherwise:** pick the first item.

### 1. Create Worktree and Checkout PR Branch

Create an isolated git worktree so the main working directory stays clean:

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
BRANCH=$(gh pr view $PR --json headRefName --jq .headRefName)
WORKTREE_DIR=".worktrees/review-$BRANCH"
mkdir -p .worktrees
git fetch origin $BRANCH
git worktree add "$WORKTREE_DIR" $BRANCH
cd "$WORKTREE_DIR"
```

All subsequent steps run inside the worktree.

### 2. Fix Copilot Review Comments

Check for existing Copilot review comments (no waiting — Copilot review was already requested by `issue-to-pr`):

```bash
COMMENTS=$(gh api repos/$REPO/pulls/$PR/comments --jq '[.[] | select(.user.login == "copilot-pull-request-reviewer[bot]")]')
```

If there are actionable comments: invoke `/fix-pr` to address them, then push:

```bash
git push
```

If no comments (or Copilot hasn't reviewed yet): skip to next step.

### 3. Agentic Feature Test

Run agentic feature tests on the modified feature:

1. **Identify the feature** from the PR title and changed files:
   - `[Model]` PRs: the new problem model name
   - `[Rule]` PRs: the new reduction rule (source -> target)

2. **Invoke `/agentic-tests:test-feature`** with the identified feature. This simulates a downstream user exercising the feature from docs and examples.

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

**After 3 failed retries:** leave PR open, still move to In Review for human triage.

### 5. Clean Up Worktree

```bash
cd /Users/liujinguo/rcode/problemreductions
git worktree remove "$WORKTREE_DIR" --force
```

### 6. Move to "In Review"

```bash
gh project item-edit \
  --id <ITEM_ID> \
  --project-id PVT_kwDOBrtarc4BRNVy \
  --field-id PVTSSF_lADOBrtarc4BRNVyzg_GmQc \
  --single-select-option-id df73e18b
```

### 7. Report

```
Review pipeline complete:
  PR:     #570
  Copilot comments: 3 fixed
  CI:     green
  Agentic test: passed
  Board:  review-agentic -> In Review
```

### 8. Batch Mode (`--all`)

If `--all` was specified, repeat Steps 1-7 for each PR. After all PRs, print a batch report:

```
=== Review Pipeline Batch Report ===

| PR   | Title                              | Copilot | CI      | Agentic Test | Board      |
|------|------------------------------------|---------|---------|--------------|------------|
| #570 | Fix #117: [Model] GraphPartitioning| 3 fixed | green   | passed       | In Review  |
| #571 | Fix #97: [Rule] BinPacking to ILP  | 0       | green   | passed       | In Review  |

Completed: 2/2 | All moved to In Review
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| PR not in review-agentic column | Verify status before processing; STOP if not review-agentic |
| Missing project scopes | Run `gh auth refresh -s read:project,project` |
| Skipping agentic tests | Always run test-feature even if CI is green |
| Not checking out the right branch | Use `gh pr view` to get the exact branch name |
| Worktree left behind on failure | Always clean up with `git worktree remove` in Step 5 |
| Working in main checkout | All work happens in `.worktrees/` — never modify the main checkout |
