---
name: final-review
description: Interactive maintainer review for PRs in "Final review" column — assess usefulness, safety, completeness, quality ranking, then merge or hold
---

# Final Review

Interactive review with the maintainer for PRs in the `Final review` column on the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1). The goal is to decide whether to **merge**, put **OnHold** (with reason), or **quick fix** before merging.

**Rule: Every `AskUserQuestion` must include your recommendation** (e.g., "My recommendation: **Merge** — clean implementation with full coverage").

## Invocation

- `/final-review` -- pick the first PR from "Final review" column
- `/final-review 42` -- review a specific PR number

## Constants

GitHub Project board IDs (for `gh project item-edit`):

| Constant | Value |
|----------|-------|
| `PROJECT_ID` | `PVT_kwDOBrtarc4BRNVy` |
| `STATUS_FIELD_ID` | `PVTSSF_lADOBrtarc4BRNVyzg_GmQc` |
| `STATUS_FINAL_REVIEW` | `51a3d8bb` |
| `STATUS_ON_HOLD` | `48dfe446` |
| `STATUS_DONE` | `6aca54fa` |

## Workflow

### Step 0: Discover "Final review" PRs

If a specific PR number was given, use it directly. Otherwise:

1. Fetch all project board items:
   ```bash
   gh project item-list 8 --owner CodingThrust --limit 500 --format json
   ```
2. Filter items where `Status == "Final review"`. Items may be Issues (with linked PRs) or PRs directly.
3. If none found, report "No items in the Final review column" and stop.
4. Pick the first one. If the item is an Issue, find the linked PR by searching open PRs for `Fix #<issue_number>` in the title. Print title, PR number, issue number, and URL.

### Step 1: Gather PR context

Collect all information needed for the review:

1a. **PR metadata**: `gh pr view <number> --json title,body,labels,files,additions,deletions,commits,headRefName,baseRefName,url,state`

1b. **PR diff**: `gh pr diff <number>` — read the full diff to understand all changes.

1c. **Linked issue**: Extract the linked issue number from PR body (look for `Fixes #N`, `Closes #N`, or `#N` references). Fetch issue body: `gh issue view <N> --json title,body,labels`

1d. **Determine PR type**: From labels and title, classify as `[Model]` or `[Rule]`.
  - For `[Model]`: identify the problem name being added
  - For `[Rule]`: identify the source and target problem names

1e. **Existing problems**: Run `pred list` (CLI tool, not MCP) to show all currently registered problems and reductions. This provides context for evaluating usefulness.

1f. **Check for conflicts with main**: Run `gh pr view <number> --json mergeable`. If there are merge conflicts, launch a subagent to merge `origin/main` into the PR branch (in a worktree) and push the merge commit.

### Step 2: Usefulness assessment

Think critically about whether this model/rule is genuinely useful. Consider:

- **For models**: Is this problem well-known in the literature? Does it connect to existing problems via reductions? Is it a trivial variant of something already implemented? Would researchers or practitioners actually use this?
- **For rules**: Is this reduction well-known? Is it non-trivial (not just a relabeling)? Does it strengthen the reduction graph connectivity? Is the overhead reasonable?

Present your assessment to the reviewer:

> **Usefulness Assessment**
>
> [Your reasoning — 2-3 sentences with specific justification]
>
> Verdict: [Useful / Marginal / Not useful]

Use `AskUserQuestion` to ask the reviewer:

> **Do you agree with this usefulness assessment?**
> - "Agree" — continue review
> - "Disagree" — let me explain why (reviewer provides reasoning)
> - "Skip" — skip this check

### Step 3: Safety check

Scan the PR diff for dangerous actions:

- **Removed features**: Any existing model, rule, test, or example deleted?
- **Unrelated changes**: Files modified that don't belong to this PR (e.g., changes to unrelated models/rules, CI config, Cargo.toml dependency changes not needed for this PR)
- **Force push indicators**: Any sign of history rewriting
- **Broad modifications**: Changes to core traits, macros, or shared infrastructure that could affect other features

Report findings:

> **Safety Check**
>
> [List any concerns, or "No safety issues found"]

Use `AskUserQuestion` to confirm:

> **Any safety concerns with this PR?**
> - "Looks safe" — continue
> - "I see an issue" — reviewer describes the problem
> - "Skip" — skip this check

### Step 3b: File whitelist check

Check that the PR only touches files expected for its type. Any file outside the whitelist is flagged for review — it may be a legacy pattern or an unrelated change.

**Whitelist for [Model] PRs:**
- `src/models/<category>/<name>.rs` — model implementation
- `src/unit_tests/models/<category>/<name>.rs` — unit tests
- `src/example_db/model_builders.rs` — canonical example registration
- `src/example_db/rule_builders.rs` — only if updating nonempty-style assertions
- `docs/paper/reductions.typ` — paper entry
- `docs/src/reductions/problem_schemas.json` — schema export
- `docs/src/reductions/reduction_graph.json` — graph export
- `tests/suites/trait_consistency.rs` — trait consistency entry

**Whitelist for [Rule] PRs:**
- `src/rules/<source>_<target>.rs` — reduction implementation
- `src/rules/mod.rs` — module registration
- `src/unit_tests/rules/<source>_<target>.rs` — unit tests
- `src/example_db/rule_builders.rs` — canonical example registration
- `src/models/<category>/<name>.rs` — only if adding getters needed for overhead expressions
- `docs/paper/reductions.typ` — paper entry
- `docs/src/reductions/reduction_graph.json` — graph export
- `docs/src/reductions/problem_schemas.json` — only if updating field descriptions

If any file falls outside these whitelists, flag it:

> **File Whitelist Check**
>
> Found N file(s) outside expected whitelist:
> - `path/to/file` — [what it does, why it may not belong]
>
> These should be reviewed — they may follow a deprecated pattern or be unrelated to this PR.

If all files are whitelisted, report "All files within expected whitelist" and continue.

### Step 4: Completeness check

Verify the PR includes all required components. Check:

**For [Model] PRs:**
- [ ] Model implementation (`src/models/...`)
- [ ] Unit tests (`src/unit_tests/models/...`)
- [ ] `declare_variants!` macro with explicit `opt`/`sat` solver-kind markers and intended default variant
- [ ] CLI `pred create` support / help text as needed
- [ ] Canonical model example in `src/example_db/model_builders.rs`
- [ ] Paper section in `docs/paper/reductions.typ` (`problem-def` entry)
- [ ] `display-name` entry in paper
- [ ] `trait_consistency.rs` entry in `test_all_problems_implement_trait_correctly` (+ `test_direction` for optimization)

**For [Rule] PRs:**
- [ ] Reduction implementation (`src/rules/...`)
- [ ] Unit tests (`src/unit_tests/rules/...`)
- [ ] `#[reduction(overhead = {...})]` with correct expressions
- [ ] Uses only the `overhead` form of `#[reduction]` and does not duplicate a primitive exact endpoint registration
- [ ] Canonical rule example in `src/example_db/rule_builders.rs`
- [ ] Paper section in `docs/paper/reductions.typ` (`reduction-rule` entry)

Report missing items:

> **Completeness Check**
>
> [Checklist with pass/fail for each item]
> Missing: [list missing items, or "None — all complete"]

Use `AskUserQuestion` to confirm:

> **Is the completeness acceptable?**
> - "Complete enough" — continue
> - "Missing items are blocking" — needs fix before merge
> - "Skip" — skip this check

### Step 5: Quality ranking

Rate this PR's quality relative to all existing models/rules in the codebase. Consider:

- **Code quality**: Clean implementation, good variable names, proper error handling
- **Test quality**: Meaningful test cases, good coverage, closed-loop reduction tests
- **Documentation**: Clear paper section, good examples, proper citations
- **Correctness**: Overhead expressions match implementation, complexity citations verified
- **Integration**: Proper use of traits, macros, naming conventions

Assign a **quality percentile** (0-100%):
- 0-20%: Poor — significant issues, bare minimum effort
- 20-40%: Below average — functional but lacking polish
- 40-60%: Average — meets requirements, nothing remarkable
- 60-80%: Good — clean code, thorough tests, well-documented
- 80-100%: Excellent — exemplary implementation, could serve as reference

Present to reviewer:

> **Quality Ranking: N%** (among all existing models/rules)
>
> Strengths:
> - [bullet points]
>
> Weaknesses (numbered):
> 1. [issue description — file:line if applicable]
> 2. [issue description — file:line if applicable]
> ...
>
> Comparable to: [name a similar-quality existing model/rule for reference]

### Step 6: Final decision

Summarize all findings and present the numbered issues as selectable options.

Present a summary table:

| Aspect | Result |
|--------|--------|
| Usefulness | [Useful/Marginal/Not useful] |
| Safety | [Safe/Concerns found] |
| Completeness | [Complete/Missing: X, Y] |
| Quality | [N%] |
| PR URL | [link] |

Then present all numbered issues from Step 5 as a multi-select `AskUserQuestion`:

> **Which issues should be fixed before merging?** (select all that apply, or "Merge as-is")
> - "Merge as-is" — no fixes needed
> - "Fix 1: [short description]" — [one-line summary]
> - "Fix 2: [short description]" — [one-line summary]
> - ...
> - "OnHold" — move to OnHold column with a reason

This lets the reviewer cherry-pick exactly which issues to fix. If the reviewer selects fixes, proceed to Step 7 Quick fix. If "Merge as-is", proceed to Step 7 Merge.

### Step 7: Execute decision

**If Merge:**
1. Print the PR URL prominently: `https://github.com/CodingThrust/problem-reductions/pull/<number>`
2. Say: "Please merge this PR in your browser. After merging, I'll move the linked issue to Done."
3. Wait for user confirmation, then move the project board item to `Done` (`6aca54fa`).

**If OnHold:**
1. Ask the reviewer for the reason (use `AskUserQuestion` with free text).
2. Post a comment on the PR (or linked issue) with the reason:
   ```bash
   gh pr comment <number> --body "**On Hold**: <reason>"
   ```
3. Move the project board item to `OnHold` (`48dfe446`):
   ```bash
   gh project item-edit --project-id PVT_kwDOBrtarc4BRNVy --id <ITEM_ID> --field-id PVTSSF_lADOBrtarc4BRNVyzg_GmQc --single-select-option-id 48dfe446
   ```

**If Quick fix:**
1. Apply only the fixes the reviewer selected in Step 6.
2. Checkout the PR branch in a worktree, apply fixes, commit, push.
3. After push, go back to Step 6 to re-confirm the decision.

**If Reject:**
1. Ask the reviewer for the reason.
2. Post a comment explaining the rejection.
3. Close the PR: `gh pr close <number> --comment "<reason>"`
4. Move the project board item to `OnHold` (`48dfe446`).
