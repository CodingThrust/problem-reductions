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

This skill runs **fully autonomously** except for one case: if the scripted `review-pipeline` context bundle returns `status == "needs-user-choice"`, STOP and ask the user which PR is the intended target.

## Steps

### 0. Generate the Review-Pipeline Report

Step 0 should be a single report-generation step. Do not manually unpack board selection, worktree prep, or PR context with shell snippets.

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
STATE_FILE=/tmp/problemreductions-review-selection.json
set -- python3 scripts/pipeline_skill_context.py review-pipeline --repo "$REPO" --state-file "$STATE_FILE" --format text
if [ -n "${PR:-}" ]; then
  set -- "$@" --pr "$PR"
fi
REPORT=$("$@")
printf '%s\n' "$REPORT"
```

The report is the Step 0 packet. It should already include:
- Selection: board item, PR number, linked issue, title, URL
- Recommendation Seed: suggested mode and deterministic blockers
- Comment Summary
- CI / Coverage
- Merge Prep
- Linked Issue Context

Branch from the report:
- `Bundle status: empty` => STOP with `No Review pool PRs are currently eligible for review-pipeline.`
- `Bundle status: needs-user-choice` => STOP and ask the user which PR is intended
- `Bundle status: ready` => continue with the already-claimed item and prepared worktree

For ambiguous cards, the report should print short options and the recommendation. Format the prompt like:

```text
Review pool card links multiple repo PRs:
1. PR #170 — CLOSED — Superseded LCS model
2. PR #173 — OPEN — Fix #109: Add LCS reduction  (Recommended)
```

The bundle already handled the mechanical claim step:
- normal eligible PRs are claimed through the review queue
- explicit `--pr` matches on ambiguous cards are treated as deterministic disambiguation and claimed automatically

When you need to take actions later, use the identifiers already printed in the report (`Board item`, `PR`, worktree path). If you absolutely need raw structured data for a corner case, rerun the same command with `--format json`, but do not rebuild Step 0 manually.

All subsequent steps run inside the prepared worktree and should read facts from the report instead of re-fetching them by default.

### 1a. Resolve Conflicts with Main

**IMPORTANT:** The `add-model` and `add-rule` skills evolve frequently. When merging main into a PR branch, conflicts in skill-generated code are common. Before resolving conflicts:

1. Run `git diff origin/main...HEAD -- .claude/skills/add-model/ .claude/skills/add-rule/` to see if these skills changed on main since the PR was created.
2. If they changed, read the current versions on main (`git show origin/main:.claude/skills/add-model/SKILL.md` and `git show origin/main:.claude/skills/add-rule/SKILL.md`) to understand what's different.
3. When resolving conflicts in model/rule implementation files, prefer the patterns from main's current skills — the PR's implementation may be based on outdated skill instructions.

Read the merge result from the report's `Merge Prep` section.

- If the report says the merge status is `clean`: push the merge commit and continue.
- If there are conflicts:
  1. Inspect the conflicting files listed in the report.
  2. Compare the current skill versions on main vs the PR branch to understand which patterns are current.
  3. Resolve conflicts (prefer main's patterns for skill-generated code, the PR branch for problem-specific logic, main for regenerated artifacts like JSON).
  4. Stage resolved files, commit, and push.
- If the report says the merge status is `conflicted` and there are too many conflicts (>3 files, or conflicts in core implementation files beyond just JSON/skill artifacts):
  1. Abort the merge: `git merge --abort` if a merge is still in progress
  2. Post a comment on the PR explaining the situation:
     ```bash
     gh pr comment <PR_NUMBER> --body "This PR has significant merge conflicts with main ($(N) conflicting files). Moving back to Ready for rework.

     Conflicting files:
     $(list of files)

     The PR needs to be rebased on current main and conflicts resolved before it can proceed through the review pipeline."
     ```
  3. Close the PR:
     ```bash
     gh pr close <PR_NUMBER>
     ```
  4. Move the project item back to `Ready`:
     ```bash
     python3 scripts/pipeline_board.py move <ITEM_ID> ready
     ```
  5. Report: `PR #N has too many merge conflicts — closed and moved back to Ready for rework.`
  6. STOP processing this PR.

### 2. Fix Copilot Review Comments

Use the report as the primary mechanical context:
- `Comment Summary`
- `CI / Coverage`
- `Linked Issue Context`

Inspect the report's Copilot comment count and linked issue context. If there are actionable comments: invoke `/fix-pr` to address them, then push:

```bash
git push
```

If Copilot approved with no actionable comments: skip to next step.

### 2a. Check Issue Comments and Human PR Reviews

Reuse the report's `Comment Summary` and `Linked Issue Context` sections. If you need the raw structured comment objects for a corner case, rerun the bundle with `--format json`.

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
| Picking a PR before Copilot has reviewed | Check `pulls/$PR/reviews` for copilot-pull-request-reviewer[bot]; if absent, request with `gh copilot-review <PR>` and wait |
| Missing project scopes | Run `gh auth refresh -s read:project,project` |
| Skipping review-implementation | Always run structural completeness check in Step 2b — it catches gaps Copilot misses (paper entries, CLI registration, trait_consistency) |
| Skipping agentic tests | Always run test-feature even if CI is green |
| Not checking out the right branch | Use `gh pr view` to get the exact branch name |
| Worktree left behind on failure | Always clean up with `git worktree remove` in Step 5 |
| Working in main checkout | All work happens in `.worktrees/` — never modify the main checkout |
| Skipping merge with main | Always merge origin/main in Step 1a to catch conflicts before fixing comments |
| Wasting time on heavy conflicts | If >3 files conflict or core impl files are affected, close PR, move to Ready, and let project-pipeline rework it |
| Ignoring issue comments | Always check the linked issue (`Fix #N`) for human feedback in Step 2a |
| Only checking Copilot comments | Step 2a checks human PR reviews and linked issue comments too — bot-only review is insufficient |
| Saying "passed" while deferring issues | If anything remains for maintainer judgment, list it explicitly under `Remaining issues for final review` and mark the agentic result accordingly |
