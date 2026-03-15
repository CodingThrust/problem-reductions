---
name: issue-to-pr
description: Use when you have a GitHub issue and want to create a PR with an implementation plan that triggers automated execution
---

# Issue to PR

Convert a GitHub issue into an actionable PR with a plan. Optionally execute the plan immediately using subagent-driven-development.

## Invocation

- `/issue-to-pr 42` — create PR with plan only
- `/issue-to-pr 42 --execute` — create PR, then execute the plan and review

For Codex, open this `SKILL.md` directly and treat the slash-command forms above as aliases. The Makefile `run-issue` target already does this translation.

## Workflow

```
Receive issue number [+ --execute flag]
    -> Fetch structured issue preflight report
    -> Verify Good label and rule-model guards
    -> If guards fail: STOP
    -> If guards pass: research references, write plan, create or resume PR
    -> If --execute: run plan via subagent-driven-development, then review-implementation
```

## Steps

### 1. Parse Input

Extract issue number, repo, and flags from arguments:
- `123` -> issue #123, plan only
- `123 --execute` -> issue #123, plan + execute
- `https://github.com/owner/repo/issues/123` -> issue #123
- `owner/repo#123` -> issue #123 in owner/repo

Normalize to:
- `ISSUE=<number>`
- `REPO=<owner/repo>` (default `CodingThrust/problem-reductions`)
- `EXECUTE=true|false`

### 2. Fetch Issue + Preflight Guards

```bash
ISSUE_JSON=$(python3 scripts/pipeline_checks.py issue-guards \
  --repo "$REPO" \
  --issue "$ISSUE" \
  --format json)
```

Treat `ISSUE_JSON` as the source of truth for the deterministic preflight data:
- `title`, `body`, `labels`, and `comments` provide the issue summary and comment thread
- `kind`, `source_problem`, and `target_problem` provide parsed issue metadata
- `checks.good_label`, `checks.source_model`, and `checks.target_model` provide guard outcomes
- `existing_prs`, `resume_pr`, and `action` tell you whether to resume an open PR instead of creating a new one

Present the issue summary to the user. **Also review all comments** — contributors and maintainers may have posted clarifications, corrections, additional context, or design decisions that refine or override parts of the original issue body. Incorporate relevant comment content when writing the plan.

### 3. Verify Issue Has Passed check-issue

The issue must have already passed the `check-issue` quality gate (Stage 1 validation). Do NOT re-validate the issue here.

Use `ISSUE_JSON.checks.good_label`:
- If it is `fail` → **STOP**: "Issue #N has not passed check-issue. Please run `/check-issue <N>` first."
- If it is `pass` → continue.

### 3.5. Model-Existence Guard (for `[Rule]` issues only)

For `[Rule]` issues, `ISSUE_JSON` already includes `source_problem`, `target_problem`, and the deterministic model-existence checks.

- If both `checks.source_model` and `checks.target_model` are `pass` → continue to step 4.
- If either is `fail` → **STOP**. Comment on the issue: "Blocked: model `<name>` does not exist in main yet. Please implement it first (or file a `[Model]` issue)."

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

Use the `ISSUE_JSON.action` and `ISSUE_JSON.resume_pr` fields from Step 2.

**If an open PR already exists** (`action == "resume-pr"`):
- Switch to its branch: `git checkout <resume_pr.head_ref_name>`
- Capture `PR=<resume_pr.number>`
- Skip plan creation — jump directly to Step 7 (execute)

**If no open PR exists** (`action == "create-pr"`) — create one with only the plan file:

```bash
# Prepare or reuse the issue branch (this enforces a clean working tree)
BRANCH_JSON=$(python3 scripts/pipeline_worktree.py prepare-issue-branch \
  --issue <number> \
  --slug <slug> \
  --base main \
  --format json)
BRANCH=$(printf '%s\n' "$BRANCH_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['branch'])")

# Stage the plan file
git add docs/plans/<plan-file>.md

# Commit
git commit -m "Add plan for #<number>: <title>"

# Push
git push -u origin "$BRANCH"

# Create PR body
PR_BODY_FILE=$(mktemp)
cat > "$PR_BODY_FILE" <<'EOF'
## Summary
<Brief description>

Fixes #<number>
EOF

# Create PR and capture the created PR number
PR_JSON=$(python3 scripts/pipeline_pr.py create \
  --repo "$REPO" \
  --title "Fix #<number>: <title>" \
  --body-file "$PR_BODY_FILE" \
  --base main \
  --head "$BRANCH" \
  --format json)
PR=$(printf '%s\n' "$PR_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['pr_number'])")
rm -f "$PR_BODY_FILE"
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
COMMENT_FILE=$(mktemp)
cat > "$COMMENT_FILE" <<'EOF'
## Implementation Summary

### Changes
- [list of files added/modified and what they do]

### Deviations from Plan
- [any design changes, accidents, or workarounds — or "None"]

### Open Questions
- [any trade-offs or items needing review — or "None"]
EOF
python3 scripts/pipeline_pr.py comment --repo "$REPO" --pr "$PR" --body-file "$COMMENT_FILE"
rm -f "$COMMENT_FILE"

git push
make copilot-review
```

#### 7e. Done

Report final status:
- PR URL and number
- Implementation summary

The PR is **not merged** and CI/review fixes are **not** handled here. The separate `review-pipeline` skill picks up PRs from the `Review pool` board column to handle Copilot review comments, CI fixes, and agentic testing.

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
| Branch already exists on retry | Use `pipeline_worktree.py prepare-issue-branch` — it will reuse the existing branch instead of failing on `git checkout -b` |
| Dirty working tree | Use `pipeline_worktree.py prepare-issue-branch` — it stops before branching if the worktree is dirty |
| Bundling model + rule in one PR | Each PR must contain exactly one model or one rule — STOP and block if model is missing (Step 3.5) |
| Plan files left in PR | Delete plan files before final push (Step 7c) |
