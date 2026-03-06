---
name: meta-power
description: Use when you want to batch-resolve all open [Model] and [Rule] GitHub issues autonomously — plans, implements, reviews, fixes, and merges each one in dependency order
---

# Meta-Power

Batch-process open `[Model]` and `[Rule]` issues end-to-end: plan, implement, review, fix CI, and merge — fully autonomous.

## Overview

You are the **outer orchestrator**. For each issue you invoke `issue-to-pr --execute`, which handles the full pipeline (plan, implement, review, fix). You never implement code directly.

**Batch context:** When invoking sub-skills (like `issue-to-pr`), you are running in batch mode. Auto-approve any confirmation prompts from sub-skills — do not wait for user input mid-batch.

## Step 0: Discover and Order Issues

```bash
# Fetch all open issues
gh issue list --state open --limit 50 --json number,title
```

Filter to issues whose title contains `[Model]` or `[Rule]`. Partition into two buckets, sort each by issue number ascending. Final order: **all Models first, then all Rules**.

**Filter to checked issues only:** Only include issues with the `Good` label (added by `check-issue` when all checks pass):

```bash
# Only process issues that have the "Good" label
LABELS=$(gh issue view <number> --json labels --jq '[.labels[].name] | join(",")')
# Skip if "Good" is not in LABELS
```

Issues without the `Good` label are excluded from the batch with status `skipped (not checked)`.

**Check for existing PRs:** For each issue, check if a PR already exists:
```bash
gh pr list --search "Fixes #<number>" --state open --json number,headRefName
```
If a PR exists, mark the issue as `resume` — `issue-to-pr --execute` will detect the existing PR and continue from where it left off.

Present the ordered list to the user for confirmation before starting:

```
Batch plan:
  Models:
    #108  [Model] LongestCommonSubsequence
    #103  [Model] SubsetSum          (has open PR #115 — will resume)
  Rules:
    #109  [Rule] LCS → MIS
    #110  [Rule] LCS → ILP
    #97   [Rule] BinPacking → ILP
    #91   [Rule] CVP → QUBO

Proceed? (user confirms)
```

Initialize a results table to track status for each issue.

## Step 1: Plan, Execute, Review, Fix (issue-to-pr --execute)

For the current issue:

```bash
git checkout main && git pull origin main
```

**Check for stale branches:** If a branch `issue-<number>-*` exists with no open PR, delete it to start fresh:
```bash
STALE=$(git branch --list "issue-<number>-*" | head -1 | xargs)
if [ -n "$STALE" ]; then
    git branch -D "$STALE"
    git push origin --delete "$STALE" 2>/dev/null || true
fi
```

Invoke `issue-to-pr --execute` with the issue number. This single skill call handles:
- Creating branch and PR with plan
- Executing the plan via subagent-driven-development
- Running review-implementation
- Pushing and requesting Copilot review
- Fix loop (up to 3 retries of fix-pr)

```
/issue-to-pr <number> --execute
```

**If `issue-to-pr` fails** at any stage: record the failure status, skip Steps 2-3, move to next issue.

Capture the PR number for the merge step:
```bash
PR=$(gh pr view --json number --jq .number 2>/dev/null)
if [ -z "$PR" ]; then
    # issue-to-pr failed before creating a PR
    # Record status and move to next issue
fi
```

## Step 2: Merge (skip if Step 1 failed)

Only attempt merge if `issue-to-pr --execute` completed successfully (CI green or fix loop passed).

```bash
gh pr merge $PR --squash --delete-branch --auto
```

The `--auto` flag tells GitHub to merge once all required checks pass, avoiding a race between CI completion and the merge command.

**If merge fails** (e.g., conflict): record status as `merge failed`, leave PR open, move to next issue.

Wait for the auto-merge to complete before proceeding:
```bash
for i in $(seq 1 20); do
    sleep 15
    STATE=$(gh pr view $PR --json state --jq .state)
    if [ "$STATE" = "MERGED" ]; then break; fi
    if [ "$STATE" = "CLOSED" ]; then break; fi  # merge conflict closed it
done
```

## Step 3: Sync

Return to main for the next issue:

```bash
git checkout main && git pull origin main
```

This ensures the next issue (especially a Rule that depends on a just-merged Model) sees all prior work.

## Step 4: Report

After all issues are processed, print the summary table:

```
=== Meta-Power Batch Report ===

| Issue | Title                              | Status                    |
|-------|------------------------------------|---------------------------|
| #108  | [Model] LCS                        | merged                    |
| #103  | [Model] SubsetSum                  | merged (resumed PR #115)  |
| #109  | [Rule] LCS → MIS                  | merged                    |
| #110  | [Rule] LCS → ILP                  | fix-pr failed (3 retries) |
| #97   | [Rule] BinPacking → ILP           | merged                    |
| #91   | [Rule] CVP → QUBO                 | skipped (plan failed)     |

Completed: 4/6 | Skipped: 1 | Failed: 1
```

## Constants

| Name | Value | Rationale |
|------|-------|-----------|
| `MERGE_POLL_INTERVAL` | 15s | Wait for auto-merge to land |
| `MERGE_POLL_MAX` | 5 min | Upper bound for merge completion |

CI polling and retry constants are defined in `issue-to-pr` Step 7d.

## Context Budget

Each `issue-to-pr --execute` call consumes significant context (research, planning, implementation, review, fix loop). In a long batch, the session may hit context limits after 2-3 issues. If this happens, the current issue's status is recorded as `context exhausted` and the batch report is printed with remaining issues marked `not started`.

## Common Failure Modes

| Symptom | Cause | Mitigation |
|---------|-------|------------|
| `issue-to-pr` comments and stops | Issue template incomplete | Skip; user must fix the issue |
| `issue-to-pr --execute` fails | Implementation too complex or subagent error | Skip; needs manual work |
| CI red after 3 retries | Deep bug or flaky test | Leave PR open for human review |
| Merge conflict | Concurrent push to main | Leave PR open; manual rebase needed |
| Rule fails because model missing | Model issue was skipped earlier | Expected; skip rule too |
| Stale branch from previous run | Previous meta-power run failed mid-issue | Auto-cleaned in Step 1 |
| PR already exists for issue | Previous partial attempt | Resumed from existing PR |
| Context exhausted mid-batch | Too many issues in one session | Print report with remaining issues as `not started` |
