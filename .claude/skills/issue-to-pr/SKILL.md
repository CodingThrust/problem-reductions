---
name: issue-to-pr
description: Use when you have a GitHub issue and want to create a PR with an implementation plan that triggers automated execution
---

# Issue to PR

Convert a GitHub issue into an actionable PR with a plan. Optionally execute the plan immediately using subagent-driven-development.

## Invocation

- `/issue-to-pr 42` — create PR with plan only
- `/issue-to-pr 42 --execute` — create PR, then execute the plan and review

## Workflow

```
Receive issue number [+ --execute flag]
    -> Fetch issue with gh
    -> Verify Good label (from check-issue)
    -> If not Good: STOP
    -> If Good: research references, write plan, create PR
    -> If --execute: run plan via subagent-driven-development, then review-implementation
```

## Steps

### 1. Parse Input

Extract issue number and flags from arguments:
- `123` -> issue #123, plan only
- `123 --execute` -> issue #123, plan + execute
- `https://github.com/owner/repo/issues/123` -> issue #123
- `owner/repo#123` -> issue #123 in owner/repo

### 2. Fetch Issue

```bash
gh issue view <number> --json title,body,labels,assignees,comments
```

Present issue summary to user. **Also review all comments** — contributors and maintainers may have posted clarifications, corrections, additional context, or design decisions that refine or override parts of the original issue body. Incorporate relevant comment content when writing the plan.

### 3. Verify Issue Has Passed check-issue

The issue must have already passed the `check-issue` quality gate (Stage 1 validation). Do NOT re-validate the issue here.

**Gate condition:** The issue must have the `Good` label (added by `check-issue` when all checks pass).

```bash
LABELS=$(gh issue view <number> --json labels --jq '[.labels[].name] | join(",")')
```

- If `Good` is NOT in the labels → **STOP**: "Issue #N has not passed check-issue. Please run `/check-issue <N>` first."
- If `Good` is present → continue to step 4.

### 3.5. Model-Existence Guard (for `[Rule]` issues only)

For `[Rule]` issues, parse the source and target problem names from the title (e.g., `[Rule] BinPacking to ILP` → source=BinPacking, target=ILP). Verify that **both** models already exist in the codebase on `main`:

```bash
grep -r "struct SourceName" src/models/
grep -r "struct TargetName" src/models/
```

- If **both** models exist → continue to step 4.
- If either model is missing → **STOP**. Comment on the issue: "Blocked: model `<name>` does not exist in main yet. Please implement it first (or file a `[Model]` issue)."

**One item per PR:** Do NOT implement a missing model as part of a `[Rule]` PR. Each PR should contain exactly one model or one rule, never both. This avoids bloated PRs and repeated implementation when the model is needed by multiple rules.

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

**Plan batching:** The paper writing step (add-model Step 6 / add-rule Step 5) MUST be in a **separate batch** from the implementation steps, so it gets its own subagent with fresh context. It depends on the implementation being complete (needs exports). Example batch structure for a `[Model]` plan:
- Batch 1: Steps 1-5.5 (implement model, register, CLI, tests, trait_consistency)
- Batch 2: Step 6 (write paper entry — depends on batch 1 for exports)

**Solver rules:**
- Ensure at least one solver is provided in the issue template. Check if the solving strategy is valid. If not, reply under issue to ask for clarification.
- If the solver uses integer programming, implement the model and ILP reduction rule together.
- Otherwise, ensure the information provided is enough to implement a solver.

**Example rules:**
- Implement the user-provided example instance as an example program in `examples/`.
- Run the example; verify JSON output against user-provided information.
- Present in `docs/paper/reductions.typ` in tutorial style with clear intuition (see KColoring->QUBO section for reference).

### 6. Create PR (or Resume Existing)

**Check for existing PR first:**
```bash
EXISTING_PR=$(gh pr list --search "Fixes #<number>" --state open --json number,headRefName --jq '.[0].number // empty')
```

**If a PR already exists** (`EXISTING_PR` is non-empty):
- Switch to its branch: `git checkout <headRefName>`
- Capture `PR=$EXISTING_PR`
- Skip plan creation — jump directly to Step 7 (execute)

**If no existing PR** — create one with only the plan file:

**Pre-flight checks** (before creating the branch):
1. Verify clean working tree: `git status --porcelain` must be empty. If not, STOP and ask user to stash or commit.
2. Check if branch already exists: `git rev-parse --verify issue-<number>-<slug> 2>/dev/null`. If it exists, switch to it with `git checkout` (no `-b`) instead of creating a new one.

```bash
# Create branch (from main)
git checkout main
git rev-parse --verify issue-<number>-<slug> 2>/dev/null && git checkout issue-<number>-<slug> || git checkout -b issue-<number>-<slug>

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

Fixes #<number>"

# Capture PR number
PR=$(gh pr view --json number --jq .number)
```

### 7. Execute Plan (only with `--execute`)

Skip this step if `--execute` was not provided.

#### 7a. Implement

Execute the plan using `superpowers:subagent-driven-development`:

1. **Read the plan** from `docs/plans/<plan-file>.md`
2. **Clear context** — summarize only the plan content and essential file paths, then invoke subagent-driven-development with a clean prompt. Do not carry forward research notes, issue comments, or other accumulated context from prior steps.
3. **Invoke subagent-driven-development** with the plan as input — this dispatches parallel subagents for independent tasks in the plan

If execution fails, leave the PR open with the plan commit only — the user can run `make run-plan` manually later. Skip remaining sub-steps.

#### 7b. Review

Run review-implementation to verify the code:

```
/review-implementation
```

Auto-fix any issues found. If unfixable issues remain, report them to the user.

**Commit all changes** (implementation + review fixes):
```bash
git add -A
git commit -m "Implement #<number>: <title>"
```

#### 7c. Clean Up Plan File

Delete the plan file from the branch — it served its purpose during implementation and should not be merged into main:

```bash
git rm docs/plans/<plan-file>.md
git commit -m "chore: remove plan file after implementation"
```

#### 7d. Push, Post Summary, and Request Copilot Review

Post an implementation summary comment on the PR **before** pushing. This comment should:
- Summarize what was implemented (files added/changed)
- Highlight any **deviations from the plan** — design changes, unexpected issues, or workarounds discovered during implementation
- Note any open questions or trade-offs made

```bash
gh pr comment $PR --body "$(cat <<'EOF'
## Implementation Summary

### Changes
- [list of files added/modified and what they do]

### Deviations from Plan
- [any design changes, accidents, or workarounds — or "None"]

### Open Questions
- [any trade-offs or items needing review — or "None"]
EOF
)"

git push
make copilot-review
```

#### 7e. Done

Report final status:
- PR URL and number
- Implementation summary

The PR is **not merged** and CI/review fixes are **not** handled here. The separate `review-pipeline` skill picks up PRs from the `review-agentic` board column to handle Copilot review comments, CI fixes, and agentic testing.

## Example

```
User: /issue-to-pr 42

Claude: Let me fetch issue #42...

[Fetches issue: "[Rule] IndependentSet to QUBO"]
[Verifies Good label — passed]
[Researches references]
[Writes docs/plans/2026-02-09-independentset-to-qubo.md]
[Creates branch, commits, pushes]
[Creates PR]

Created PR #45: Fix #42: Add IndependentSet -> QUBO reduction
```

```
User: /issue-to-pr 42 --execute

Claude: [Same as above, then continues...]

Executing plan via subagent-driven-development...
[Subagents implement the plan steps]
[Runs review-implementation — all checks pass, auto-fixes applied]
[Pushes + requests Copilot review]

PR #45 created and pushed. Copilot review requested.
Run /review-pipeline to process Copilot comments, fix CI, and run agentic tests.
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Issue not checked | Run `/check-issue <N>` first — issue-to-pr requires it |
| Issue has failure labels | Fix the issue, re-run `/check-issue`, then retry |
| Including implementation code in initial PR | First PR: plan only |
| Generic plan | Use specifics from the issue, mapped to add-model/add-rule steps |
| Skipping CLI registration in plan | add-model still requires alias/create/example-db planning, but not manual CLI dispatch-table edits |
| Not verifying facts from issue | Use WebSearch/WebFetch to cross-check claims |
| Branch already exists on retry | Check with `git rev-parse --verify` before `git checkout -b` |
| Dirty working tree | Verify `git status --porcelain` is empty before branching |
| Bundling model + rule in one PR | Each PR must contain exactly one model or one rule — STOP and block if model is missing (Step 3.5) |
| Plan files left in PR | Delete plan files before final push (Step 7c) |
