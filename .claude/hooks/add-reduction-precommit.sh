#!/bin/bash
# Pre-commit hook for add-reduction: blocks commits missing required files.
#
# This hook fires on any commit that stages a new rule file under src/rules/.
# It checks that the companion files (example_db test, paper entry, mod.rs)
# are also staged. If any are missing, the commit is blocked.
#
# Install: symlink or copy to .git/hooks/pre-commit
#   ln -sf ../../.claude/hooks/add-reduction-precommit.sh .git/hooks/pre-commit

STAGED=$(git diff --cached --name-only)

# Only run for commits that add/modify rule files (not mod.rs or traits.rs)
RULE_FILES=$(echo "$STAGED" | grep "^src/rules/" | grep -v mod.rs | grep -v traits.rs | grep '\.rs$')
if [ -z "$RULE_FILES" ]; then
  exit 0
fi

ERRORS=0

# Check example_db.rs is staged
if ! echo "$STAGED" | grep -q "example_db.rs"; then
  echo "BLOCKED: src/unit_tests/example_db.rs not staged (Check 10 from #974)"
  ERRORS=$((ERRORS + 1))
fi

# Check reductions.typ is staged
if ! echo "$STAGED" | grep -q "reductions.typ"; then
  echo "BLOCKED: docs/paper/reductions.typ not staged (Check 11 from #974)"
  ERRORS=$((ERRORS + 1))
fi

# Check mod.rs is staged
if ! echo "$STAGED" | grep -q "mod.rs"; then
  echo "BLOCKED: src/rules/mod.rs not staged"
  ERRORS=$((ERRORS + 1))
fi

# Check no verification artifacts are staged
if echo "$STAGED" | grep -q "docs/paper/verify-reductions/"; then
  echo "BLOCKED: verification artifacts still staged — run git rm first"
  ERRORS=$((ERRORS + 1))
fi

if [ $ERRORS -gt 0 ]; then
  echo ""
  echo "Fix the above and re-commit. See /add-reduction skill for details."
  exit 1
fi
