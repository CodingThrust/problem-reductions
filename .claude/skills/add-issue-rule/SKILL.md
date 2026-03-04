---
name: add-issue-rule
description: Use when filing a GitHub issue for a new reduction rule, ensuring all 9 checklist items from add-rule are complete with citations and worked examples
---

# Add Issue — Rule

File a well-formed `[Rule]` GitHub issue that passes the `issue-to-pr` validation. This skill ensures all 9 checklist items are complete, with citations, a worked example, and a correctness argument.

## Input

The caller (zero-to-infinity or user) provides:
- Source problem name
- Target problem name
- Reference URLs (if available)

## Step 1: Verify Non-Existence

Before anything else, confirm the rule doesn't already exist:

```bash
# Check implemented rules (filename pattern: source_target.rs)
ls src/rules/ | grep -i "<source_lowercase>.*<target_lowercase>"

# Check open issues
gh issue list --state open --limit 200 --json title,number | grep -i "<source>.*<target>"

# Check closed issues
gh issue list --state closed --limit 200 --json title,number | grep -i "<source>.*<target>"
```

**If found:** STOP. Report to caller that this rule already exists.

**Also verify both source and target models exist:**
```bash
ls src/models/*/ | grep -i "<source_lowercase>"
ls src/models/*/ | grep -i "<target_lowercase>"
```

If source or target model doesn't exist, report which model(s) are missing. The caller should file model issues first.

## Step 2: Research and Fill Checklist

Use `WebSearch` and `WebFetch` to fill all 9 items from the [add-rule](../add-rule/SKILL.md) Step 0 checklist:

| # | Item | How to fill |
|---|------|-------------|
| 1 | **Source problem** | Full type with generics: `ProblemName<SimpleGraph, i32>` |
| 2 | **Target problem** | Full type with generics |
| 3 | **Reduction algorithm** | Step-by-step: how to transform source instance to target instance |
| 4 | **Solution extraction** | How to map target solution back to source solution |
| 5 | **Correctness argument** | Why the reduction preserves optimality/satisfiability |
| 6 | **Size overhead** | Expressions for target size in terms of source size getters |
| 7 | **Concrete example** | Small worked instance, tutorial style, step-by-step |
| 8 | **Solving strategy** | How to solve the target (BruteForce, existing solver) |
| 9 | **Reference** | Paper/textbook citation with URL |

**Citation rule:** Every claim MUST include a URL.

## Step 3: Verify Example Correctness

For item 7 (concrete example):
- Walk through the reduction step-by-step on paper
- Show: source instance -> reduction -> target instance -> solve target -> extract source solution
- Verify the extracted solution is valid and optimal for the source
- The example must be small enough to verify by hand (3-5 vertices/variables)

## Step 4: Verify Nontriviality

The rule must be **nontrivial** (per issue #127 standards):
- NOT a simple identity mapping or type cast
- NOT a trivial embedding (just copying data)
- NOT a weight type conversion (i32 -> f64)
- MUST involve meaningful structural transformation

If the rule is trivial, STOP and report to caller.

## Step 5: Draft and File Issue

```bash
gh issue create --repo CodingThrust/problem-reductions \
  --title "[Rule] Source to Target" \
  --body "$(cat <<'ISSUE_EOF'
## Reduction Definition

**1. Source problem:** `SourceProblem<SimpleGraph, i32>`

**2. Target problem:** `TargetProblem<...>`

**3. Reduction algorithm:**
- Step 1: ...
- Step 2: ...

**4. Solution extraction:** ...

**5. Correctness argument:** ...

**6. Size overhead:**
```
field1 = "expression1"
field2 = "expression2"
```

**7. Concrete example:**
Source: ...
-> Reduction: ...
-> Target: ...
-> Solve: ...
-> Extract: ...

**8. Solving strategy:** BruteForce / existing solver

**9. Reference:**
- [Source](url)

## References
- [Source 1](url1)
ISSUE_EOF
)"
```

Report the created issue number and URL.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Filing trivial reductions | Check nontriviality in Step 4 |
| Missing model dependency | Verify both source and target exist in Step 1 |
| Example too complex | Keep to 3-5 vertices/variables, verifiable by hand |
| Missing correctness argument | Must explain WHY, not just HOW |
| Wrong overhead expressions | Must reference getter methods that exist on source type |
