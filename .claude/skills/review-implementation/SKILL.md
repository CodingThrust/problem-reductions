---
name: review-implementation
description: Use after implementing a model, rule, or any code change to verify completeness and correctness before committing
---

# Review Implementation

Dispatches two parallel review subagents with fresh context (no implementation history bias):
- **Structural reviewer** -- model/rule checklists + semantic correctness (only for new models/rules)
- **Quality reviewer** -- DRY, KISS, HC/LC, HCI, test quality (always)

## Invocation

- `/review-implementation` -- auto-detect from git diff
- `/review-implementation model MaximumClique` -- review a specific model
- `/review-implementation rule mis_qubo` -- review a specific rule
- `/review-implementation generic` -- code quality only (no structural checklist)

## Step 1: Generate the Review-Implementation Report

Step 1 should be a single report-generation step. Do not manually rebuild git range detection, `review-context`, current PR lookup, or linked-issue loading with separate shell snippets.

```bash
set -- python3 scripts/pipeline_skill_context.py review-implementation --repo-root . --format text

# Explicit subject overrides still go through the same bundle. Examples:
# set -- "$@" --kind model --name MaximumClique
# set -- "$@" --kind rule --name mis_qubo --source MaximumIndependentSet --target QUBO
# set -- "$@" --kind generic

REPORT=$("$@")
printf '%s\n' "$REPORT"
```

The report is the Step 1 packet. It should already include:
- Review Range: base SHA, head SHA, repo root
- Scope: review type, subject, added model/rule files
- Deterministic Checks: whitelist + completeness status
- Changed Files
- Diff Stat
- Current PR
- Linked Issue Context

Use the report as the default source of truth for the rest of this skill. If you need structured data for a corner case, rerun the same command with `--format json`, but do not rebuild Step 1 manually.

## Step 2: Prepare Subagent Context

Read the packet directly:
- `Review Range` for `{BASE_SHA}` and `{HEAD_SHA}`
- `Scope` for `{REVIEW_TYPE}` and the concrete model/rule metadata
- `Changed Files` and `Diff Stat` for the quality-reviewer prompt
- `Linked Issue Context` for `{ISSUE_CONTEXT}`

If the report says `Current PR` is absent, set `{ISSUE_CONTEXT}` to `No linked issue found.` Comments often contain clarifications, corrections, or additional requirements from maintainers, so prefer the report text over re-fetching issue state.

## Step 3: Dispatch Subagents in Parallel

### Structural Reviewer (if new model/rule detected)

Dispatch using `Agent` tool with `subagent_type="superpowers:code-reviewer"`:

- Read `structural-reviewer-prompt.md` from this skill directory
- Fill placeholders:
  - `{REVIEW_TYPE}` -> from the report's `Scope` section (`model`, `rule`, `model + rule`, or `generic`)
  - `{REVIEW_PARAMS}` -> summary of what's being reviewed from the report
  - `{PROBLEM_NAME}`, `{CATEGORY}`, `{FILE_STEM}` -> for model reviews
  - `{SOURCE}`, `{TARGET}`, `{RULE_STEM}`, `{EXAMPLE_STEM}` -> for rule reviews
  - `{ISSUE_CONTEXT}` -> the report's `Linked Issue Context` section (or "No linked issue found.")
- Prompt = filled template

### Quality Reviewer (always)

Dispatch using `Agent` tool with `subagent_type="superpowers:code-reviewer"`:

- Read `quality-reviewer-prompt.md` from this skill directory
- Fill placeholders:
  - `{DIFF_SUMMARY}` -> the report's `Diff Stat`
  - `{CHANGED_FILES}` -> the report's `Changed Files`
  - `{PLAN_STEP}` -> description of what was implemented (or "standalone review")
  - `{BASE_SHA}`, `{HEAD_SHA}` -> the report's `Review Range`
  - `{ISSUE_CONTEXT}` -> the report's `Linked Issue Context` section (or "No linked issue found.")
- Prompt = filled template

**Both subagents must be dispatched in parallel** (single message with two Agent tool calls — use `run_in_background: true` on one, foreground on the other, then read the background result with `TaskOutput`).

## Step 4: Collect and Address Findings

When both subagents return:

1. **Parse results** -- identify FAIL/ISSUE items from both reports
2. **Fix automatically** -- structural FAILs (missing registration, missing file), clear semantic issues, Important+ quality issues
3. **For missing paper entries** -- these are NOT "unfixable". Handle as follows:
   - Model checks #15/#16 FAIL (missing `display-name` or `problem-def`): follow the paper writing instructions inlined in `add-model` Step 6 (register display name, write formal definition, write body with background + example + visualization, run `make paper`)
   - Rule check #14 FAIL (missing `reduction-rule`): follow the paper writing instructions inlined in `add-rule` Step 5 (load example data, write theorem body, write proof, write worked example, run `make paper`)
   - Reference the gold-standard examples: `problem-def("MaximumIndependentSet")` for models, `reduction-rule("KColoring", "QUBO"` for rules
   - Do NOT skip these or mark as "needs user decision"
4. **Report to user** -- ambiguous semantic issues, Minor quality items, anything you're unsure about
5. **Present consolidated report** combining both reviews

## Step 5: Present Consolidated Report

Merge both subagent outputs into a single report:

```
## Review: [Model/Rule/Generic] [Name]

### Structural Completeness (from structural reviewer)
| # | Check | Status |
|---|-------|--------|
...

### Build Status (from structural reviewer)
- `make test`: PASS / FAIL
- `make clippy`: PASS / FAIL

### Semantic Review (from structural reviewer)
...

### Issue Compliance (from structural reviewer, if linked issue found)
...

### Code Quality (from quality reviewer)
- DRY: OK / ...
- KISS: OK / ...
- HC/LC: OK / ...

### HCI (from quality reviewer, if CLI/MCP changed)
...

### Test Quality (from quality reviewer)
...

### Overhead Consistency Check
- Rules: verify `#[reduction(overhead)]` expressions match actual sizes constructed in `reduce_to()` code
- Rules: verify the impl uses only the `overhead` form and does not introduce a duplicate primitive exact endpoint pair
- Models: verify `dims()` and getter methods are consistent with struct fields
- Result: PASS / FAIL

### Fixes Applied
- [list of issues automatically fixed by main agent]

### Remaining Items (needs user decision)
- [list of issues that need user input]
```

## Integration

### With executing-plans

After each batch in the executing-plans flow, the main agent should:
1. Record `BASE_SHA` before the batch starts
2. After batch completes, follow Steps 1-5 above
3. Fix findings before reporting to user
4. Include review results in the batch report

### Copilot Review (after PR creation)

After creating a PR (from any flow), run `make copilot-review` to request GitHub Copilot code review on the PR.

### With add-model / add-rule

At the end of these skills (after their verify step), invoke `/review-implementation` which dispatches subagents as described above.

### Standalone

Invoke directly via `/review-implementation` for any code change.
