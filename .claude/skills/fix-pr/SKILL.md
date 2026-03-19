---
name: fix-pr
description: Use when a PR has review comments to address, CI failures to fix, or codecov coverage gaps to resolve
---

# Fix PR

Resolve PR review comments, fix CI failures, and address codecov coverage gaps for the current branch's PR.

## Step 1: Gather PR State

Step 1 should be a single report-generation step. Use the shared scripted helper to produce one skill-readable PR context packet. Do not rebuild this logic inline with `gh api | python3 -c` unless you are debugging the helper itself.

```bash
REPORT=$(python3 scripts/pipeline_pr.py context --current --format text)
printf '%s\n' "$REPORT"
```

The report should already include:
- repo, PR number, title, URL, head SHA
- comment counts
- CI summary
- Codecov summary
- linked issue context

Use the values printed in that report for the rest of this skill. If you absolutely need raw structured data for a corner case, rerun the same command with `--format json`, but do not rebuild Step 1 manually.

### 1a. Fetch Review Comments

**Check ALL four sources.** User inline comments are the most commonly missed — do not skip any.

Start from the report's `Comment Summary`. It should tell you whether any source is non-empty before you inspect raw threads.

If you need the raw comment arrays for detailed triage, rerun `python3 scripts/pipeline_pr.py context --current --format json` and inspect:
- `comments["inline_comments"]`
- `comments["reviews"]`
- `comments["human_issue_comments"]`
- `comments["human_linked_issue_comments"]`
- `comments["codecov_comments"]`

### 1b. Check CI Status

Read the report's `CI Summary`. The structured JSON fallback includes:
- `state` — `pending`, `failure`, or `success`
- `runs` — normalized check-run details
- `pending` / `failing` / `succeeding` counts

### 1c. Check Codecov Report

Read the report's `Codecov` section. The structured JSON fallback includes:
- `found` — whether a Codecov comment is present
- `patch_coverage`
- `project_coverage`
- `filepaths` — deduplicated paths referenced by Codecov links
- `body` — the raw latest Codecov comment body

## Step 2: Triage and Prioritize

Categorize all findings:

| Priority | Type | Action |
|----------|------|--------|
| 1 | CI failures (clippy/test/coverage) | Fix immediately — blocks merge |
| 2 | Inline/review comments | Address each one — evaluate validity, fix if correct |
| 3 | Codecov coverage gaps | Add tests for uncovered lines |

## Step 3: Fix CI Failures

For each failing check:

1. **Clippy**: Run `make clippy` locally, fix warnings
2. **Test**: Run `make test` locally, fix failures (build errors surface here too)
3. **Code Coverage**: See Step 5 (codecov-specific flow)

## Step 4: Address Review Comments

For each review comment:

1. Read the comment and the code it references
2. Evaluate if the suggestion is correct
3. If valid: make the fix, commit
4. If debatable: fix it anyway unless technically wrong
5. If wrong: prepare a response explaining why

**Do NOT respond on the PR** -- just fix and commit. The user will push and respond.

### Handling Suggestions

Review suggestions with `suggestion` blocks contain exact code. Evaluate each:
- **Correct**: Apply the suggestion
- **Partially correct**: Apply the spirit, adjust details
- **Wrong**: Skip, note why in commit message

## Step 5: Fix Codecov Coverage Gaps

**IMPORTANT: Do NOT run `cargo-llvm-cov` locally.** Use the `gh api` to read the codecov report instead.

### 5a. Identify Uncovered Lines

From the `CODECOV` JSON (fetched in Step 1c), extract:
- Files with missing coverage
- Patch coverage percentage
- Specific uncovered files referenced in `filepaths`

Then read the source files and identify which new/changed lines lack test coverage.

### 5b. Add Tests for Uncovered Lines

1. Read the uncovered file and identify the untested code paths
2. Write tests targeting those specific paths (error branches, edge cases, etc.)
3. Run `make test` to verify tests pass
4. Commit the new tests

### 5c. Verify Coverage Improvement

After pushing, CI will re-run coverage. Check the updated codecov comment on the PR.

## Step 6: Commit and Report

After all fixes:

```bash
# Verify everything passes locally
make check  # fmt + clippy + test
```

Commit with a descriptive message referencing the PR:

```bash
git commit -m "fix: address PR #$PR review comments

- [summary of fixes applied]
"
```

Report to user:
- List of review comments addressed (with what was done)
- CI fixes applied
- Coverage gaps filled
- Any comments left unresolved (with reasoning)

## Integration

### With review-implementation

Run `/review-implementation` first to catch issues before push. Then `/fix-pr` after push to address CI and reviewer feedback.

### With executing-plans / finishing-a-development-branch

After creating a PR, use `/fix-pr` to address review feedback and CI failures.
