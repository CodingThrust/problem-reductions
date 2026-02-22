---
name: issue-to-pr
description: Use when you have a GitHub issue and want to create a PR with an implementation plan that triggers automated execution
---

# Issue to PR

Convert a GitHub issue into an actionable PR with a plan. This skill validates the issue, then dispatches to the appropriate implementation skill.

## Workflow

```
Receive issue number
    -> Fetch issue with gh
    -> Classify: [Model] or [Rule]?
    -> Validate against checklist (from add-model or add-rule)
    -> If incomplete: comment on issue, STOP
    -> If complete: research references, write plan, create PR
```

## Steps

### 1. Parse Input

Extract issue number from argument:
- `123` -> issue #123
- `https://github.com/owner/repo/issues/123` -> issue #123
- `owner/repo#123` -> issue #123 in owner/repo

### 2. Fetch Issue

```bash
gh issue view <number> --json title,body,labels,assignees
```

Present issue summary to user.

### 3. Classify and Validate

Determine the issue type from its title/labels:
- **`[Model]`** issues -> validate against the checklist in [add-model](../add-model/SKILL.md) Step 0
- **`[Rule]`** issues -> validate against the checklist in [add-rule](../add-rule/SKILL.md) Step 0

Check every item in the relevant checklist against the issue body. Verify facts provided by the user -- feel free to use `WebSearch` and `WebFetch` to cross-check claims.

**If any item is missing or unclear:** comment on the issue via `gh issue comment <number> --body "..."` listing what's missing. Then STOP -- do NOT proceed until the issue is complete.

**If all items are present:** continue to step 4.

### 4. Research References

Use `WebSearch` and `WebFetch` to look up the reference URL provided in the issue. This helps:
- Clarify the formal problem definition and notation
- Understand the reduction algorithm in detail (variable mapping, penalty terms, proof of correctness)
- Resolve any ambiguities in the issue description without bothering the contributor

If the reference is a paper or textbook, search for accessible summaries, lecture notes, or Wikipedia articles on the same reduction.

### 5. Write Plan

Write plan to `docs/plans/YYYY-MM-DD-<slug>.md` using `superpowers:writing-plans`.

The plan MUST reference the appropriate implementation skill and follow its steps:

- **For `[Model]` issues:** Follow [add-model](../add-model/SKILL.md) Steps 1-7 as the action pipeline
- **For `[Rule]` issues:** Follow [add-rule](../add-rule/SKILL.md) Steps 1-6 as the action pipeline

Include the concrete details from the issue (problem definition, reduction algorithm, example, etc.) mapped onto each step.

**Solver rules:**
- Ensure at least one solver is provided in the issue template. Check if the solving strategy is valid. If not, reply under issue to ask for clarification.
- If the solver uses integer programming, implement the model and ILP reduction rule together.
- Otherwise, ensure the information provided is enough to implement a solver.

**Example rules:**
- Implement the user-provided example instance as an example program in `examples/`.
- Run the example; verify JSON output against user-provided information.
- Present in `docs/paper/reductions.typ` in tutorial style with clear intuition (see KColoring->QUBO section for reference).

### 6. Create PR

Create a pull request with only the plan file.

```bash
# Create branch
git checkout -b issue-<number>-<slug>

# Stage the plan file
git add docs/plans/<plan-file>.md

# Commit
git commit -m "Add plan for #<number>: <title>"

# Push
git push -u origin issue-<number>-<slug>

# Create PR
gh pr create --title "Fix #<number>: <title>" --body "
## Summary
<Brief description>

Closes #<number>"
```

## Example

```
User: /issue-to-pr 42

Claude: Let me fetch issue #42...

[Fetches issue: "[Rule] IndependentSet to QUBO"]
[Classifies as [Rule] issue]
[Validates against add-rule checklist -- all items present]

All required info is present. I'll create the plan...

[Writes docs/plans/2026-02-09-independentset-to-qubo.md]
[Creates branch, commits, pushes]
[Creates PR]

Created PR #45: Fix #42: Add IndependentSet -> QUBO reduction
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Issue template incomplete | Comment listing missing items, then STOP |
| Including implementation code in initial PR | First PR: plan only |
| Generic plan | Use specifics from the issue, mapped to add-model/add-rule steps |
| Skipping CLI registration in plan | add-model requires CLI dispatch updates -- include in plan |
| Not verifying facts from issue | Use WebSearch/WebFetch to cross-check claims |
