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
gh issue view <number> --json title,body,labels,assignees
```

Present issue summary to user.

### 3. Verify Issue Has Passed check-issue

The issue must have already passed the `check-issue` quality gate (Stage 1 validation). Do NOT re-validate the issue here.

**Gate condition:** The issue must have the `Good` label (added by `check-issue` when all checks pass).

```bash
LABELS=$(gh issue view <number> --json labels --jq '[.labels[].name] | join(",")')
```

- If `Good` is NOT in the labels → **STOP**: "Issue #N has not passed check-issue. Please run `/check-issue <N>` first."
- If `Good` is present → continue to step 4.

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

#### 7c. Push, Post Summary, and Request Copilot Review

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

#### 7d. Fix Loop (max 3 retries)

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
```

For each retry:

1. **Wait for CI to complete** (poll every 30s, up to 15 minutes):
   ```bash
   for i in $(seq 1 30); do
       sleep 30
       HEAD_SHA=$(gh api repos/$REPO/pulls/$PR | python3 -c "import sys,json; print(json.load(sys.stdin)['head']['sha'])")
       STATUS=$(gh api repos/$REPO/commits/$HEAD_SHA/check-runs | python3 -c "
   import sys,json
   runs = json.load(sys.stdin)['check_runs']
   if not runs:
       print('PENDING')  # CI hasn't registered yet
   else:
       failed = [r['name'] for r in runs if r.get('conclusion') not in ('success', 'skipped', None)]
       pending = [r['name'] for r in runs if r.get('conclusion') is None and r['status'] != 'completed']
       if pending:
           print('PENDING')
       elif failed:
           print('FAILED')
       else:
           print('GREEN')
   ")
       if [ "$STATUS" != "PENDING" ]; then break; fi
   done
   ```

   - If `GREEN` on the **first** iteration (before any fix-pr): skip the fix loop, done.
   - If `GREEN` after a fix-pr pass: break, done.
   - If `FAILED`: continue to step 2.
   - If still `PENDING` after 15 min: treat as `FAILED`.

2. **Invoke `/fix-pr`** to address review comments, CI failures, and coverage gaps.

3. **Push fixes:**
   ```bash
   git push
   ```

4. Increment retry counter. If `< 3`, go back to step 1. If `= 3`, give up.

**After 3 failed retries:** leave PR open, report to user.

#### 7e. Done

Report final status:
- PR URL
- CI status (green / failed after retries)
- Any unresolved review items

The PR is **not merged** — the user or `meta-power` decides when to merge.

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
[Polls CI... GREEN on first pass]

PR #45: CI green, ready for merge.
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Issue not checked | Run `/check-issue <N>` first — issue-to-pr requires it |
| Issue has failure labels | Fix the issue, re-run `/check-issue`, then retry |
| Including implementation code in initial PR | First PR: plan only |
| Generic plan | Use specifics from the issue, mapped to add-model/add-rule steps |
| Skipping CLI registration in plan | add-model requires CLI dispatch updates -- include in plan |
| Not verifying facts from issue | Use WebSearch/WebFetch to cross-check claims |
| Branch already exists on retry | Check with `git rev-parse --verify` before `git checkout -b` |
| Dirty working tree | Verify `git status --porcelain` is empty before branching |
