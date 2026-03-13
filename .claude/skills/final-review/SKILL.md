---
name: final-review
description: Interactive maintainer review for PRs in "In review" column — assess usefulness, safety, completeness, quality ranking, then merge or hold
---

# Final Review

Interactive review with the maintainer for PRs in the `In review` column on the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1). The goal is to decide whether to **merge**, put **OnHold** (with reason), or **quick fix** before merging.

**Rule: Every `AskUserQuestion` must include your recommendation** (e.g., "My recommendation: **Merge** — clean implementation with full coverage").

## Invocation

- `/final-review` -- pick the first PR from "In review" column
- `/final-review 42` -- review a specific PR number

## Constants

GitHub Project board IDs (for `gh project item-edit`):

| Constant | Value |
|----------|-------|
| `PROJECT_ID` | `PVT_kwDOBrtarc4BRNVy` |
| `STATUS_FIELD_ID` | `PVTSSF_lADOBrtarc4BRNVyzg_GmQc` |
| `STATUS_IN_REVIEW` | `df73e18b` |
| `STATUS_ON_HOLD` | `29244783` |
| `STATUS_DONE` | `98236657` |

## Workflow

### Step 0: Discover "In review" PRs

If a specific PR number was given, use it directly. Otherwise:

1. Fetch all project board items:
   ```bash
   gh project item-list 8 --owner CodingThrust --limit 500 --format json
   ```
2. Filter items where `Status == "In review"`. Items may be Issues (with linked PRs) or PRs directly.
3. If none found, report "No items in the In review column" and stop.
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

### Step 4: Completeness check

Verify the PR includes all required components. Check:

**For [Model] PRs:**
- [ ] Model implementation (`src/models/...`)
- [ ] Unit tests (`src/unit_tests/models/...`)
- [ ] `declare_variants!` macro with complexity
- [ ] Paper section in `docs/paper/reductions.typ` (`problem-def` entry)
- [ ] `display-name` entry in paper

**For [Rule] PRs:**
- [ ] Reduction implementation (`src/rules/...`)
- [ ] Unit tests (`src/unit_tests/rules/...`)
- [ ] `#[reduction(overhead = {...})]` with correct expressions
- [ ] Example file (`examples/reduction_...`)
- [ ] Example test in `tests/suites/examples.rs`
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
> Weaknesses:
> - [bullet points]
>
> Comparable to: [name a similar-quality existing model/rule for reference]

### Step 6: Final decision

Summarize all findings and ask the reviewer for a decision.

Present a summary table:

| Aspect | Result |
|--------|--------|
| Usefulness | [Useful/Marginal/Not useful] |
| Safety | [Safe/Concerns found] |
| Completeness | [Complete/Missing: X, Y] |
| Quality | [N%] |
| PR URL | [link] |

Use `AskUserQuestion`:

> **What would you like to do with this PR?**
> - "Merge" — approve and show merge link for browser
> - "OnHold" — move to OnHold column with a reason comment
> - "Quick fix" — fix specific issues before merging (describe what to fix)
> - "Reject" — close the PR with explanation

### Step 7: Execute decision

**If Merge:**
1. Print the PR URL prominently: `https://github.com/CodingThrust/problem-reductions/pull/<number>`
2. Say: "Please merge this PR in your browser. After merging, I'll move the linked issue to Done."
3. Wait for user confirmation, then move the project board item to `Done` (`98236657`).

**If OnHold:**
1. Ask the reviewer for the reason (use `AskUserQuestion` with free text).
2. Post a comment on the PR (or linked issue) with the reason:
   ```bash
   gh pr comment <number> --body "**On Hold**: <reason>"
   ```
3. Move the project board item to `OnHold` (`29244783`):
   ```bash
   gh project item-edit --project-id PVT_kwDOBrtarc4BRNVy --id <ITEM_ID> --field-id PVTSSF_lADOBrtarc4BRNVyzg_GmQc --single-select-option-id 29244783
   ```

**If Quick fix:**
1. Ask the reviewer what needs fixing (use `AskUserQuestion`).
2. Checkout the PR branch in a worktree, apply fixes, commit, push.
3. After push, go back to Step 6 to re-confirm the decision.

**If Reject:**
1. Ask the reviewer for the reason.
2. Post a comment explaining the rejection.
3. Close the PR: `gh pr close <number> --comment "<reason>"`
4. Move the project board item to `OnHold` (`29244783`).
