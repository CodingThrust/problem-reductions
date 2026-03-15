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

### Step 0: Load the Final-Review Context Bundle

Start from the skill-scoped bundle. The happy path should only consume:
- `CTX["selection"]`
- `CTX["pr"]`
- `CTX["prep"]`
- `CTX["review_context"]`

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
STATE_FILE=/tmp/problemreductions-final-review-selection.json
set -- python3 scripts/pipeline_skill_context.py final-review --repo "$REPO" --state-file "$STATE_FILE" --format json
if [ -n "${PR:-}" ]; then
  set -- "$@" --pr "$PR"
fi
CTX=$("$@")
STATUS=$(printf '%s\n' "$CTX" | python3 -c "import sys,json; print(json.load(sys.stdin)['status'])")
```

Branch on `STATUS`:
- `empty`: report `No items in the Final review column` and stop.
- `ready`: continue with the full common-path bundle.
- `ready-with-warnings`: continue only with the narrow warning fallback. Read `CTX["warnings"]` first and keep the fallback limited to whatever data could not be prepared mechanically.

Extract the common working objects:

```bash
ITEM_ID=$(printf '%s\n' "$CTX" | python3 -c "import sys,json; print(json.load(sys.stdin)['selection']['item_id'])")
PR=$(printf '%s\n' "$CTX" | python3 -c "import sys,json; print(json.load(sys.stdin)['selection']['pr_number'])")
ISSUE=$(printf '%s\n' "$CTX" | python3 -c "import sys,json; data=json.load(sys.stdin); print(data['selection'].get('issue_number') or '')")
TITLE=$(printf '%s\n' "$CTX" | python3 -c "import sys,json; print(json.load(sys.stdin)['selection']['title'])")
PR_CTX=$(printf '%s\n' "$CTX" | python3 -c "import sys,json; print(json.dumps(json.load(sys.stdin)['pr']))")
PREP=$(printf '%s\n' "$CTX" | python3 -c "import sys,json; print(json.dumps(json.load(sys.stdin)['prep']))")
REVIEW_CONTEXT=$(printf '%s\n' "$CTX" | python3 -c "import sys,json; print(json.dumps(json.load(sys.stdin).get('review_context')))")
```

If `PREP["checkout"]["worktree_dir"]` exists, `cd` into it for any merge-resolution or quick-fix work:

```bash
WORKTREE_DIR=$(printf '%s\n' "$CTX" | python3 -c "import sys,json; print((json.load(sys.stdin).get('prep', {}).get('checkout') or {}).get('worktree_dir', ''))")
if [ -n "$WORKTREE_DIR" ]; then
  cd "$WORKTREE_DIR"
fi
```

### Step 1: Use the Bundled Review Context

`PR_CTX` already includes the mechanical PR data:
- title, body, URL, mergeability, changed files, commits
- `comments`
- linked issue metadata and `issue_context_text`
- CI summary
- Codecov summary

`PREP` already includes the review worktree and merge attempt:
- `PREP["checkout"]`
- `PREP["merge"]`
- `PREP["ready"]`

`REVIEW_CONTEXT` is the deterministic review/check payload:
- `REVIEW_CONTEXT["subject"]`
- `REVIEW_CONTEXT["whitelist"]`
- `REVIEW_CONTEXT["completeness"]`
- `REVIEW_CONTEXT["changed_files"]`
- `REVIEW_CONTEXT["diff_stat"]`

Read the full diff as additional review input:

```bash
gh pr diff "$PR"
```

Run `pred list` (CLI tool, not MCP) to see the surrounding problem/reduction graph context before assessing usefulness.

If `PREP["ready"]` is false, inspect `PREP["merge"]`. The common case is still usable:
- if `PREP["merge"]["status"] == "conflicted"` but the worktree exists, you still have `REVIEW_CONTEXT`; decide whether to hold for manual resolution or resolve and continue
- if `STATUS == "ready-with-warnings"` and `REVIEW_CONTEXT` is `null`, treat that as a narrow prep failure path and prefer hold/manual follow-up over reassembling lots of mechanics inside the skill

### Step 1a: Comment Audit (REQUIRED)

Final review must check the comment history before recommending merge.

Read the following from `PR_CTX["comments"]`:
- `human_issue_comments`
- `inline_comments`
- `reviews`

Read the linked-issue discussion from `PR_CTX`:
- `linked_issue_number`
- `human_linked_issue_comments`
- `issue_context_text`

Build a list of every actionable comment and classify each as:
- `addressed`
- `superseded / no longer applicable`
- `still open`

Pay special attention to the `## Review Pipeline Report` comment. If it contains a `Remaining issues for final review` section, those items must be reviewed explicitly here.

Do **not** recommend merge until every actionable comment has been dispositioned.

Prepare a short summary for later steps:

> **Comment Audit**
>
> [N addressed, M superseded, K still open]
>
> Open items:
> - [comment / issue summary]
> - ...

If no actionable comments remain, report `No open actionable comments`.

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

Use `REVIEW_CONTEXT["whitelist"]` directly in the common path.

If `REVIEW_CONTEXT` is `null` because `STATUS == "ready-with-warnings"`, call that out explicitly and keep the fallback narrow: either fix the prep problem first or hold the PR instead of rebuilding the full deterministic pipeline manually inside the skill.

If any file falls outside `REVIEW_CONTEXT["whitelist"]`, flag it:

> **File Whitelist Check**
>
> Found N file(s) outside expected whitelist:
> - `path/to/file` — [what it does, why it may not belong]
>
> These should be reviewed — they may follow a deprecated pattern or be unrelated to this PR.

If all files are whitelisted, report "All files within expected whitelist" and continue.

### Step 4: Completeness check

Use `REVIEW_CONTEXT["completeness"]` as the deterministic baseline checklist for files, paper entries, examples, variants/overhead forms, and trait-consistency coverage. Then apply maintainer judgment on anything the script cannot prove.

Read the review subject from `REVIEW_CONTEXT["subject"]` to understand whether the PR is being reviewed as a model, rule, or generic change. If `REVIEW_CONTEXT` is `null`, that is a rare prep-failure path and should usually push you toward hold/manual follow-up rather than a full merge recommendation.

Verify the PR includes all required components. Check:

**For [Model] PRs:**
- [ ] Model implementation (`src/models/...`)
- [ ] Unit tests (`src/unit_tests/models/...`)
- [ ] `declare_variants!` macro with explicit `opt`/`sat` solver-kind markers and intended default variant
- [ ] Schema / registry entry for CLI-facing model creation (`ProblemSchemaEntry`)
- [ ] Canonical model example function in the model file
- [ ] Paper section in `docs/paper/reductions.typ` (`problem-def` entry)
- [ ] `display-name` entry in paper
- [ ] `trait_consistency.rs` entry in `src/unit_tests/trait_consistency.rs` (`test_all_problems_implement_trait_correctly`, plus `test_direction` for optimization)

**For [Rule] PRs:**
- [ ] Reduction implementation (`src/rules/...`)
- [ ] `src/rules/mod.rs` registration
- [ ] Unit tests (`src/unit_tests/rules/...`)
- [ ] `#[reduction(overhead = {...})]` with correct expressions
- [ ] Uses only the `overhead` form of `#[reduction]`
- [ ] Canonical rule example function in the rule file
- [ ] Paper section in `docs/paper/reductions.typ` (`reduction-rule` entry)

**Paper-example consistency check (both Model and Rule PRs):**

The paper example must use data from the generated JSON (`docs/paper/examples/generated/`), not hand-written data. To verify:
1. Run `make examples` on the PR branch to regenerate `docs/paper/examples/generated/models.json` and `rules.json`.
2. For **[Rule] PRs**: the paper's `reduction-rule` entry must call `load-example(source, target)` (defined in `reductions.typ`) to load the canonical example from `rules.json`, and derive all concrete values from the loaded data using Typst array operations — no hand-written instance data.
3. For **[Model] PRs**: read the problem's entry in `models.json` and compare its `instance` field against the paper's `problem-def` example. The paper example must use the same instance (allowing 0-indexed JSON vs 1-indexed math notation). If they differ, flag: "Paper example does not match `example_db` canonical instance in `models.json`."

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
| Comments | [All addressed / Open: X, Y] |
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

If any actionable PR / issue comment from Step 1g is still open, `Merge as-is` must **not** be your recommendation. Recommend either **Quick fix** or **OnHold** instead.

### Step 7: Execute decision

**If Merge:**
1. Print the PR URL prominently: `https://github.com/CodingThrust/problem-reductions/pull/<number>`
2. Say: "Please merge this PR in your browser. After merging, I'll move the linked issue to Done."
3. Wait for user confirmation, then move the project board item to `Done`:
   ```bash
   python3 scripts/pipeline_board.py move <ITEM_ID> done
   ```

**If OnHold:**
1. Ask the reviewer for the reason (use `AskUserQuestion` with free text).
2. Post a comment on the PR (or linked issue) with the reason:
   ```bash
   COMMENT_FILE=$(mktemp)
   printf '**On Hold**: %s\n' "<reason>" > "$COMMENT_FILE"
   python3 scripts/pipeline_pr.py comment --repo "$REPO" --pr "<number>" --body-file "$COMMENT_FILE"
   rm -f "$COMMENT_FILE"
   ```
3. Move the project board item to `OnHold`:
   ```bash
   python3 scripts/pipeline_board.py move <ITEM_ID> on-hold
   ```

**If Quick fix:**
1. Apply only the fixes the reviewer selected in Step 6.
2. Checkout the PR branch in a worktree, apply fixes, commit, push.
3. After push, go back to Step 6 to re-confirm the decision.

**If Reject:**
1. Ask the reviewer for the reason.
2. Post a comment explaining the rejection.
3. Close the PR: `gh pr close <number> --comment "<reason>"`
4. Move the project board item to `OnHold`:
   ```bash
   python3 scripts/pipeline_board.py move <ITEM_ID> on-hold
   ```
