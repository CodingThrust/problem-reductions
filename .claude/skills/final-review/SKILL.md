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

### Step 0: Generate the Final-Review Report

Step 0 should be a single report-generation step. Do not manually unpack board selection, PR metadata, merge prep, or deterministic checks with shell snippets.

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
STATE_FILE=/tmp/problemreductions-final-review-selection.json
set -- python3 scripts/pipeline_skill_context.py final-review --repo "$REPO" --state-file "$STATE_FILE" --format text
if [ -n "${PR:-}" ]; then
  set -- "$@" --pr "$PR"
fi
REPORT=$("$@")
printf '%s\n' "$REPORT"
```

The report is the Step 0 packet. It already includes **all** mechanical context:
- Selection: board item, PR number, linked issue, title, URL
- Recommendation Seed: suggested mode and deterministic blockers
- Subject
- Comment Summary (with linked issue context)
- Merge Prep
- Deterministic Checks
- Changed Files
- Diff Stat
- Full Diff

Branch from the report:
- `Bundle status: empty` => stop with `No items in the Final review column`
- `Bundle status: ready` => continue normally (check warnings — a self-review warning means the reviewer is the PR author; flag it but do not block)
- `Bundle status: ready-with-warnings` => continue only with the narrow warning fallback described in the report

When you need to take actions later, use the identifiers already printed in the report (`Board item`, `PR`, URL). If you absolutely need raw structured data for a corner case, rerun the same command with `--format json`, but do not rebuild Step 0 manually.

### Step 1: Push the Merge with Main

The context script already merged `origin/main` into the PR branch in the worktree. Read the report's `Merge Prep` section:

- **Merge status: clean** — push the merge commit from the worktree:
  ```bash
  cd <worktree path from report>
  git push
  ```
- **Merge status: conflicted** — note the conflicts. You can still continue with the review steps below and decide whether to resolve or hold in Step 6.
- **Merge prep failed** — skip this step; the warning fallback applies.

### Step 1a: Use the Bundled Review Context

**Trust the report.** The Step 0 report already contains all mechanical context — selection, comments, linked issue, merge prep, deterministic checks, changed files, diff stat, full diff, and `pred list` output. Do NOT re-fetch any of this data with separate tool calls (e.g., `gh api` for comments, `gh pr diff`, `gh pr view`, `pred list`). Extract everything directly from the report text.

If the report is in the warning fallback path, keep the fallback narrow. Prefer hold/manual follow-up over reconstructing the whole pipeline inside the skill.

### Step 1b: Comment Audit (REQUIRED)

Final review must check the comment history before recommending merge.

Use the report's `Comment Summary` and `Linked Issue Context` sections as the starting point. If you need to inspect the underlying comment threads in detail, do that only after reading the report.

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

Use the report's `Deterministic Checks` section directly in the common path.

If the report says whitelist is unavailable because of the warning fallback, call that out explicitly and keep the fallback narrow: either fix the prep problem first or hold the PR instead of rebuilding the deterministic pipeline manually inside the skill.

If the report says files fall outside the whitelist, flag it:

> **File Whitelist Check**
>
> Found N file(s) outside expected whitelist:
> - `path/to/file` — [what it does, why it may not belong]
>
> These should be reviewed — they may follow a deprecated pattern or be unrelated to this PR.

If all files are whitelisted, report "All files within expected whitelist" and continue.

### Step 4: Completeness check

Use the report's `Deterministic Checks` section as the baseline checklist for files, paper entries, examples, variants/overhead forms, and trait-consistency coverage. Then apply maintainer judgment on anything the script cannot prove.

Read the review subject from the report's `Subject` section to understand whether the PR is being reviewed as a model, rule, or generic change. If the deterministic checks are unavailable because of the warning fallback, that should usually push you toward hold/manual follow-up rather than a full merge recommendation.

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
- [ ] Aliases: if provided, verify they are standard literature abbreviations (not made up); if empty, confirm no well-known abbreviation is missing; check no conflict with existing aliases

**For [Rule] PRs:**
- [ ] Reduction implementation (`src/rules/...`)
- [ ] `src/rules/mod.rs` registration
- [ ] Unit tests (`src/unit_tests/rules/...`)
- [ ] `#[reduction(overhead = {...})]` with correct expressions
- [ ] Uses only the `overhead` form of `#[reduction]`
- [ ] Canonical rule example function in the rule file
- [ ] Paper section in `docs/paper/reductions.typ` (`reduction-rule` entry)

**Paper-example consistency check (both Model and Rule PRs):**

The paper example must use data from the canonical fixture JSON (`src/example_db/fixtures/examples.json`), not hand-written data. To verify:
1. If the PR changes example builders/specs, run `make regenerate-fixtures` on the PR branch.
2. For **[Rule] PRs**: the paper's `reduction-rule` entry must call `load-example(source, target, ...)` (defined in `reductions.typ`) to load the canonical example from `examples.json`, and derive all concrete values from the loaded data using Typst array operations — no hand-written instance data.
3. For **[Model] PRs**: read the problem's entry in `examples.json` under `models` and compare its `instance` field against the paper's `problem-def` example. The paper example must use the same instance (allowing 0-indexed JSON vs 1-indexed math notation). If they differ, flag: "Paper example does not match `example_db` canonical instance in `examples.json`."

**Issue–test round-trip consistency check (both Model and Rule PRs):**

The unit test's example instance and expected solution must match the issue's example. Compare using the report's `Linked Issue Context` and `Full Diff`:

1. **Instance match**: The unit test's `example_instance()` (or equivalent setup) must construct the same graph/weights/parameters as described in the issue's "Example Instance" section. Check vertex count, edge list, weights, and any problem-specific fields (e.g., terminals, clauses).
2. **Solution match**: The expected optimal value in the test (e.g., `SolutionSize::Valid(6)`) must equal the issue's stated optimal. For rules, the closed-loop test must verify that reducing and solving the target gives the same optimum as solving the source directly.
3. **Brute-force verification**: A brute-force test must exist that independently confirms the expected optimum, not just assert a hardcoded value.

If any mismatch is found, flag it:

> **Issue–Test Consistency**
>
> Mismatch: [describe what differs — e.g., "Issue says optimal cost = 6 but test asserts 7"]

If all match, report "Issue example and unit tests are consistent."

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

> **How should we proceed?** (select all that apply)
> - "Approve & Merge" — approve the PR, then squash-merge and move to Done
> - "Record 1: [short description]" — record for follow-up fix (does not block merge)
> - "Record 2: [short description]" — record for follow-up fix
> - ...
> - "Quick fix 1: [short description]" — fix now before merging
> - "Quick fix 2: [short description]" — fix now before merging
> - ...
> - "OnHold" — move to OnHold column with a reason

"Record" items are non-blocking — they get posted as a follow-up comment on the PR/issue but do not prevent merging. "Quick fix" items are applied immediately before merging.

If any actionable PR / issue comment from Step 1b is still open, `Approve & Merge` must **not** be your recommendation. Recommend either **Quick fix** or **OnHold** instead.

### Step 7: Execute decision

**If Approve & Merge:**
1. If any "Record" items were selected, post them as a follow-up comment:
   ```bash
   COMMENT_FILE=$(mktemp)
   cat > "$COMMENT_FILE" <<'EOF'
   **Follow-up items** (recorded during final review):
   - [item 1]
   - [item 2]
   EOF
   python3 scripts/pipeline_pr.py comment --repo "$REPO" --pr "<number>" --body-file "$COMMENT_FILE"
   rm -f "$COMMENT_FILE"
   ```
2. Approve and merge the PR (approve may fail if you are the PR author — that's OK, continue to merge):
   ```bash
   gh pr review <number> --approve || true
   gh pr merge <number> --squash --delete-branch
   ```
3. Move the project board item to `Done`:
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
2. Work in the worktree from Step 0, apply fixes, commit, push.
3. After push, go back to Step 6 to re-confirm the decision.

**If Reject:**
1. Ask the reviewer for the reason.
2. Post a comment explaining the rejection.
3. Close the PR: `gh pr close <number> --comment "<reason>"`
4. Move the project board item to `OnHold`:
   ```bash
   python3 scripts/pipeline_board.py move <ITEM_ID> on-hold
   ```
