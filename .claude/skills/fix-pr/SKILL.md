---
name: fix-pr
description: Use when a PR has review comments to address, CI failures to fix, or codecov coverage gaps to resolve
---

# Fix PR

Resolve PR review comments, fix CI failures, and address codecov coverage gaps for the current branch's PR.

## Step 1: Gather PR State

**IMPORTANT:** Do NOT use `gh api --jq` for extracting data — it uses a built-in jq that
chokes on response bodies containing backslashes (common in Copilot code suggestions).
Always pipe to `python3 -c` instead. (`gh pr view --jq` is fine — only `gh api --jq` is affected.)

```bash
# Get repo identifiers
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)  # e.g., "owner/repo"

# Get PR number
PR=$(gh pr view --json number --jq .number)

# Get PR head SHA (on remote)
HEAD_SHA=$(gh api repos/$REPO/pulls/$PR | python3 -c "import sys,json; print(json.load(sys.stdin)['head']['sha'])")
```

### 1a. Fetch Review Comments

**Check ALL four sources.** User inline comments are the most commonly missed — do not skip any.

```bash
# 1. Inline review comments on code lines (from ALL reviewers: users AND Copilot)
gh api repos/$REPO/pulls/$PR/comments | python3 -c "
import sys,json
comments = json.load(sys.stdin)
print(f'=== Inline comments: {len(comments)} ===')
for c in comments:
    line = c.get('line') or c.get('original_line') or '?'
    print(f'[{c[\"user\"][\"login\"]}] {c[\"path\"]}:{line} — {c[\"body\"][:200]}')
"

# 2. Review-level comments (top-level review body from formal reviews)
gh api repos/$REPO/pulls/$PR/reviews | python3 -c "
import sys,json
reviews = json.load(sys.stdin)
print(f'=== Reviews: {len(reviews)} ===')
for r in reviews:
    if r.get('body'):
        print(f'[{r[\"user\"][\"login\"]}] {r[\"state\"]}: {r[\"body\"][:200]}')
"

# 3. Issue-level comments (general discussion)
gh api repos/$REPO/issues/$PR/comments | python3 -c "
import sys,json
comments = [c for c in json.load(sys.stdin) if 'codecov' not in c['user']['login']]
print(f'=== Issue comments: {len(comments)} ===')
for c in comments:
    print(f'[{c[\"user\"][\"login\"]}] {c[\"body\"][:200]}')
"
```

**Verify counts:** If any source returns 0, confirm it's genuinely empty — don't assume no feedback exists.

### 1b. Check CI Status

```bash
# All check runs on the PR head
gh api repos/$REPO/commits/$HEAD_SHA/check-runs | python3 -c "
import sys,json
for cr in json.load(sys.stdin)['check_runs']:
    print(f'{cr[\"name\"]}: {cr.get(\"conclusion\") or cr[\"status\"]}')
"
```

### 1c. Check Codecov Report

```bash
# Codecov bot comment with coverage diff
gh api repos/$REPO/issues/$PR/comments | python3 -c "
import sys,json
for c in json.load(sys.stdin):
    if c['user']['login'] == 'codecov[bot]':
        print(c['body'])
"
```

## Step 2: Triage and Prioritize

Categorize all findings:

| Priority | Type | Action |
|----------|------|--------|
| 1 | CI failures (clippy/test/coverage) | Fix immediately — blocks merge |
| 2 | User inline/review comments | Address each one — highest review priority |
| 3 | Copilot inline suggestions | Evaluate validity, fix if correct |
| 4 | Codecov coverage gaps | Add tests for uncovered lines |

**User comments always take priority over bot comments.** A user inline comment requesting file deletion is just as important as a user review requesting a code change.

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
gh api repos/$REPO/issues/$PR/comments | python3 -c "
import sys,json,re
for c in json.load(sys.stdin):
    if c['user']['login'] == 'codecov[bot]':
        for m in re.findall(r'filepath=([^&\"]+)', c['body']):
            print(m)
"
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
