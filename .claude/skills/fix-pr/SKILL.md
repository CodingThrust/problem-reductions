---
name: fix-pr
description: Use when a PR has review comments to address, CI failures to fix, or codecov coverage gaps to resolve
---

# Fix PR

Resolve PR review comments, fix CI failures, and address codecov coverage gaps for the current branch's PR.

## Step 1: Gather PR State

```bash
# Get PR number
PR=$(gh pr view --json number --jq .number)

# Get PR head SHA (on remote)
HEAD_SHA=$(gh api repos/{owner}/{repo}/pulls/$PR --jq '.head.sha')
```

### 1a. Fetch Review Comments

Three sources of feedback to check:

```bash
# Copilot and user inline review comments (on code lines)
gh api repos/{owner}/{repo}/pulls/$PR/comments --jq '.[] | "[\(.user.login)] \(.path):\(.line // .original_line) — \(.body)"'

# Review-level comments (top-level review body)
gh api repos/{owner}/{repo}/pulls/$PR/reviews --jq '.[] | select(.body != "") | "[\(.user.login)] \(.state): \(.body)"'

# Issue-level comments (general discussion)
gh api repos/{owner}/{repo}/issues/$PR/comments --jq '.[] | select(.user.login | test("codecov|copilot") | not) | "[\(.user.login)] \(.body)"'
```

### 1b. Check CI Status

```bash
# All check runs on the PR head
gh api repos/{owner}/{repo}/commits/$HEAD_SHA/check-runs \
  --jq '.check_runs[] | "\(.name): \(.conclusion // .status)"'
```

### 1c. Check Codecov Report

```bash
# Codecov bot comment with coverage diff
gh api repos/{owner}/{repo}/issues/$PR/comments \
  --jq '.[] | select(.user.login == "codecov[bot]") | .body'
```

## Step 2: Triage and Prioritize

Categorize all findings:

| Priority | Type | Action |
|----------|------|--------|
| 1 | CI failures (test/clippy/build) | Fix immediately -- blocks merge |
| 2 | User review comments | Address each one -- respond on PR |
| 3 | Copilot review comments | Evaluate validity, fix if correct |
| 4 | Codecov coverage gaps | Add tests for uncovered lines |

## Step 3: Fix CI Failures

For each failing check:

1. **Clippy**: Run `make clippy` locally, fix warnings
2. **Test**: Run `make test` locally, fix failures
3. **Build**: Run `make build` locally, fix errors
4. **Coverage**: See Step 5 (codecov-specific flow)

## Step 4: Address Review Comments

For each review comment:

1. Read the comment and the code it references
2. Evaluate if the suggestion is correct
3. If valid: make the fix, commit
4. If debatable: fix it anyway unless technically wrong
5. If wrong: prepare a response explaining why

**Do NOT respond on the PR** -- just fix and commit. The user will push and respond.

### Handling Copilot Suggestions

Copilot suggestions with `suggestion` blocks contain exact code. Evaluate each:
- **Correct**: Apply the suggestion
- **Partially correct**: Apply the spirit, adjust details
- **Wrong**: Skip, note why in commit message

## Step 5: Fix Codecov Coverage Gaps

**IMPORTANT: Do NOT run `cargo-llvm-cov` locally.** Use the `gh api` to read the codecov report instead.

### 5a. Identify Uncovered Lines

From the codecov bot comment (fetched in Step 1c), extract:
- Files with missing coverage
- Patch coverage percentage
- Specific uncovered lines (linked in the report)

For detailed line-by-line coverage, use the Codecov API:

```bash
# Get file-level coverage for the PR
gh api repos/{owner}/{repo}/pulls/$PR/comments \
  --jq '.[] | select(.user.login == "codecov[bot]") | .body' \
  | grep -oP 'filepath=\K[^&]+'
```

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

After creating a PR and running `make copilot-review`, use `/fix-pr` to address the resulting feedback.
