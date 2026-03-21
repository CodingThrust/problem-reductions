---
name: final-review
description: Interactive maintainer review for PRs in "Final review" column — assess usefulness, safety, completeness, quality ranking, then merge or hold
---

# Final Review

Interactive review with the maintainer for PRs in the `Final review` column on the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1). The goal is to decide whether to **merge**, put **OnHold** (with reason), or **quick fix** before merging.

**Rules:**
- Every `AskUserQuestion` must include your recommendation (e.g., "My recommendation: **Merge** — clean implementation with full coverage").
- **Skip questions when no issues found.** If a check (usefulness, safety, completeness) finds no concerns, report the positive result and continue to the next step without asking the reviewer. Only use `AskUserQuestion` when there are findings that need the reviewer's input or when the recommendation is not clearly positive.
- **Issue presentation format** — whenever reporting an issue (in any step), use this format:
  > **N. [Short title]** (`file:line`)
  > ```rust
  > // 5-15 lines showing the problem
  > ```
  > - **Why**: What's wrong and why it matters — assume the reviewer hasn't seen the code
  > - **Suggested fix**: Concrete action or code sketch

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

### Step 0: Select PR and Create Worktree

**0a. Select the PR** from the Final review column:

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
```

If a specific PR number was provided (`/final-review 42`), use it directly. Otherwise, find the first item from the Final review column:

```bash
# List items in Final review column (returns issues, not PRs)
python3 scripts/pipeline_board.py list final-review --repo "$REPO" --format json
```

The board tracks **issues**, not PRs. For each issue in the list, find the associated **open** PR:

```bash
gh pr list --repo "$REPO" --state open --search "Fix #<ISSUE_NUMBER>" --json number,title,headRefName
```

**Skip stale items**: if an issue has no open PR (the PR was already merged or doesn't exist), move it to Done (`python3 scripts/pipeline_board.py move <ITEM_ID> done`) and try the next issue. Do not spend time investigating why the board item is stale.

Extract the `PR` number from the first matched open PR and its `ITEM_ID` from the board list.

**Claim the item** by moving it to OnHold immediately, so no other reviewer picks it up:

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> on-hold
```

If the review completes successfully the item moves to Done; if abandoned, move it back to Final review (`python3 scripts/pipeline_board.py move <ITEM_ID> final-review`).

**0b. Create worktree and check out the PR branch:**

```bash
REPO_ROOT=$(pwd)
WORKTREE_JSON=$(python3 scripts/pipeline_worktree.py enter --name "final-review-pr-$PR" --format json)
WORKTREE_DIR=$(printf '%s\n' "$WORKTREE_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['worktree_dir'])")
cd "$WORKTREE_DIR"
gh pr checkout "$PR"
```

**0c. Merge main** (commit only, do not push yet — push happens in Step 8 after the review):

```bash
git fetch origin main
git merge origin/main --no-edit
```

- **Merge clean** — merge commit is ready locally. Run `make check && make paper` to verify the merge didn't break anything (API incompatibilities, formatting, test failures, paper compilation). If any step fails, fix the errors before proceeding. For `.bib` conflicts, also check for duplicate BibTeX keys (`grep '^@' docs/paper/references.bib | sed 's/@[a-z]*{//' | sed 's/,$//' | sort | uniq -d`) and remove duplicates.
- **Merge conflicted** — most conflicts in this codebase are "both sides added new entries in ordered lists" (in `mod.rs`, `lib.rs`, `create.rs`, `dispatch.rs`, `reductions.typ`). These are mechanical: keep both sides, maintain alphabetical order. Delegate to a subagent for resolution, then run `make check && make paper` to verify. Continue with the review; if conflicts are too complex, decide whether to hold in Step 5.
- **Merge failed** — note the error and continue.

**0d. Sanity check**: verify the diff touches `src/models/` or `src/rules/` (for model/rule PRs). If the diff only contains unrelated files, STOP and flag the mismatch.

### Step 1: Gather Context

Use `gh` commands to get the PR's actual data — always from GitHub, never from local git state:

```bash
# PR diff (always correct, regardless of local state)
gh pr diff "$PR"

# PR metadata
gh pr view "$PR" --json title,body,author,headRefName,baseRefName,labels,comments,reviews

# Linked issue (if any — extract from PR body "Fix #N" / "Close #N")
gh issue view <ISSUE_NUMBER> --json title,body,labels
```

Also run the review-implementation context inside the worktree for deterministic checks:

```bash
IMPL_REPORT=$(python3 scripts/pipeline_skill_context.py review-implementation --repo-root . --format text)
printf '%s\n' "$IMPL_REPORT"
```

If any deterministic check shows `fail` or `skipped` without clear explanation, fall back to `gh pr diff "$PR"` for the actual PR file list and perform the corresponding check manually against those files only (not the full merge range). The `review-implementation` script may scope to base→HEAD across the full merge, which includes unrelated changes from main. Do not silently accept partial results.

### Step 1b: Walk Through Agentic Review Findings

The `review-pipeline` skill already posted a structured **Review Pipeline Report** as a PR comment (structural check, quality check, agentic feature tests). Read it:

```bash
gh pr view "$PR" --json comments --jq '.comments[] | select(.body | contains("Review Pipeline Report")) | .body'
```

**Do not re-evaluate from scratch.** Walk through each finding from the agentic review report with the reviewer:
- For each issue flagged: is it still present? Was it already fixed? Is it a false positive?
- For each "Remaining issues for final review" item: disposition it explicitly.

Prepare a short summary:

> **Agentic Review Walk-Through**
>
> [N findings reviewed: X addressed, Y still open, Z false positive]
>
> Still open: [list each using the issue presentation format]

If no agentic review report exists (or the report is poorly structured / missing key sections), note this to the reviewer and perform the checks in Steps 2–5 yourself from scratch — read the full diff, verify correctness, check completeness, and assess quality as if no prior review had been done.

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

Use `AskUserQuestion` with your recommendation:

> My recommendation: **[Useful / Marginal / Not useful]** — [one-sentence justification]
>
> **Do you agree with this usefulness assessment?**
> - "Agree" — continue review
> - "Not useful, hold" — keep OnHold, post reason (skip remaining steps, go to Step 8)
> - Reviewer provides their own reasoning — agent updates verdict accordingly and continues
> - "Skip" — skip this check

### Step 3: Safety check

Scan the PR diff for dangerous actions:

- **Blacklisted files**: If the diff touches `docs/src/reductions/reduction_graph.json`, `docs/src/reductions/problem_schemas.json`, or `src/example_db/fixtures/examples.json` (legacy, no longer exists), **block merge**. These files are auto-generated and must not be committed in PRs — they are rebuilt by CI/`make doc`/`make paper`. Flag immediately and recommend OnHold.
- **Removed features**: Any existing model, rule, test, or example deleted?
- **Unrelated changes**: Files modified that don't belong to this PR (e.g., changes to unrelated models/rules, CI config, Cargo.toml dependency changes not needed for this PR)
- **Force push indicators**: Any sign of history rewriting
- **Broad modifications**: Changes to core traits, macros, or shared infrastructure that could affect other features
- **No committed `examples.json`**: The example database is generated on demand by `make paper` (via `export_examples`). PRs should not commit `src/example_db/fixtures/examples.json` (legacy path, deleted) or `docs/paper/data/examples.json` (current output path) — both are gitignored build artifacts.

Report findings with fix options for each concern:

> **Safety Check**
>
> [If no concerns: "No safety issues found."]
>
> [If concerns found, for each one:]
> **1. [Short title]** (`file:line`)
> - **What**: [Describe the concern and why it matters]
> - **Suggested fix**: [Concrete action — e.g., "revert this file", "split into separate PR", "remove unrelated hunk"]
> - **Recommendation**: [Block merge / Quick fix / Acceptable — with reasoning]

Use `AskUserQuestion` with your recommendation:

> My recommendation: **[Safe / Fix needed before merge]** — [one-sentence justification]
>
> **Do you agree with the safety assessment?**
> - "Agree" — continue
> - "Unsafe, hold" — keep OnHold, post reason (skip remaining steps, go to Step 8)
> - Reviewer flags additional concerns — agent adds them to the fix plan
> - "Skip" — skip this check

### Step 3b: File whitelist check

Use the `IMPL_REPORT`'s deterministic checks section. If files fall outside the whitelist, flag it:

> **File Whitelist Check**
>
> Found N file(s) outside expected whitelist:
> - `path/to/file` — [what it does, why it may not belong]

If all files are whitelisted, report "All files within expected whitelist" and continue.

### Step 4: Completeness and correctness check

Use the `IMPL_REPORT`'s deterministic checks as the baseline checklist. Then apply maintainer judgment.

Verify the PR includes all required components:

**For [Model] PRs:**
- [ ] Model implementation (`src/models/...`)
- [ ] Unit tests (`src/unit_tests/models/...`)
- [ ] `declare_variants!` macro with explicit `opt`/`sat` solver-kind markers and intended default variant
- [ ] Schema / registry entry for CLI-facing model creation (`ProblemSchemaEntry`)
- [ ] `canonical_model_example_specs()` function in the model file (gated by `#[cfg(feature = "example-db")]`) and registered in the category `mod.rs` example chain
- [ ] Paper section in `docs/paper/reductions.typ` (`problem-def` entry)
- [ ] `display-name` entry in paper
- [ ] Aliases: if provided, verify they are standard literature abbreviations (not made up); if empty, confirm no well-known abbreviation is missing; check no conflict with existing aliases
- [ ] After merge-with-main, verify no stale manual dispatch arms remain in `dispatch.rs` or `problem_name.rs` (main uses registry-based dispatch via `load_dyn` / `find_problem_type_by_alias`)

**For [Rule] PRs:**
- [ ] Reduction implementation (`src/rules/...`)
- [ ] `src/rules/mod.rs` registration
- [ ] Unit tests (`src/unit_tests/rules/...`)
- [ ] `#[reduction(overhead = {...})]` with correct expressions
- [ ] Uses only the `overhead` form of `#[reduction]`
- [ ] Canonical rule example function in the rule file
- [ ] Paper section in `docs/paper/reductions.typ` (`reduction-rule` entry)

**Paper-example consistency check (both Model and Rule PRs):**

The paper example must use data from the canonical example database (generated on demand by `make paper` via `export_examples`), not hand-written data. To verify:
1. If the PR changes example specs, run `make paper` to regenerate `docs/paper/data/examples.json`.
2. For **[Rule] PRs**: the paper's `reduction-rule` entry must call `load-example(source, target, ...)` (defined in `reductions.typ`) to load the canonical example from `examples.json`, and derive all concrete values from the loaded data using Typst array operations — no hand-written instance data.
3. For **[Model] PRs**: run the export and read the problem's entry in the generated `examples.json` under `models`, compare its `instance` field against the paper's `problem-def` example. The paper example must use the same instance (allowing 0-indexed JSON vs 1-indexed math notation). If they differ, flag: "Paper example does not match `example_db` canonical instance."

**Issue–test round-trip consistency check (both Model and Rule PRs):**

The unit test's example instance and expected solution must match the issue's example:

1. **Instance match**: The unit test's `example_instance()` (or equivalent setup) must construct the same graph/weights/parameters as described in the issue's "Example Instance" section.
2. **Solution match**: The expected optimal value in the test must equal the issue's stated optimal.
3. **Brute-force verification**: A brute-force test must exist that independently confirms the expected optimum, not just assert a hardcoded value.

Report missing items:

> **Completeness Check**
>
> [Checklist with pass/fail for each item]
> Missing: [list missing items, or "None — all complete"]

For each missing item, describe what's missing, why it matters, and propose a concrete fix (e.g., "add a `test_evaluate_optimal` test", "register in `mod.rs`").

Use `AskUserQuestion` with your recommendation:

> My recommendation: **[Complete / Fixable during review / Incomplete]** — [one-sentence justification]
>
> **Is the completeness acceptable?**
> - "Agree" — continue
> - "Incomplete, hold" — keep OnHold, post reason (skip remaining steps, go to Step 8)
> - Reviewer flags additional gaps or overrides — agent updates the fix plan accordingly
> - "Skip" — skip this check

### Step 4b: Issue consistency check

**This is a critical step.** Read the linked issue and the implementation side by side, verifying each of the following. Do NOT delegate this to a subagent — read the actual PR files yourself.

| Check | What to compare |
|-------|-----------------|
| **Mathematical definition** | Does `evaluate()` implement exactly the condition stated in the issue? Watch for subtle differences (e.g., "at least 2 of 3 edges" vs "all 3 edges"). |
| **Variable encoding** | Does `dims()` match the issue's `dims()` specification? |
| **Complexity string** | Does `declare_variants!` use the same complexity expression as the issue? |
| **Test YES instance** | Does a unit test construct the same graph/weights/parameters as the issue's YES example? |
| **Test NO instance** | Does a unit test construct the same instance as the issue's NO example (if provided)? |
| **Solution match** | Does the test verify the same partition/solution as the issue's stated answer? |
| **Paper definition** | Does the paper's `problem-def` match the issue's mathematical definition? |
| **Brute-force verification** | Does a brute-force test independently confirm the expected answer? |

Report results:

> **Issue Consistency Check**
>
> | Check | Result |
> |-------|--------|
> | Mathematical definition | [Match / Mismatch — details] |
> | Variable encoding | [Match / Mismatch] |
> | Complexity string | [Match / Mismatch] |
> | Test YES instance | [Match / Mismatch] |
> | Test NO instance | [Match / Mismatch / N/A] |
> | Solution match | [Match / Mismatch] |
> | Paper definition | [Match / Mismatch] |
> | Brute-force verification | [Present / Missing] |

If any mismatch is found, use `AskUserQuestion` with your recommendation and the specific discrepancy. If all checks pass, report the positive result and continue.

### Step 5: Quality review

Review the PR's code quality. Focus on issues that matter, not percentile scores.

**Do not assume the reviewer knows the context.** The reviewer may not have looked at this PR before. Each issue must be self-contained and actionable.

Present to reviewer:

> **Quality Review — Merge confidence: [High / Medium / Low]**
>
> **Reason**: [1-2 sentences explaining why this confidence level, referencing specific strengths or concerns]
>
> Based on: mathematical/algorithmic correctness, code quality, and writing quality (paper, docs, comments).
> - **High**: Correct, clean, well-written — ready to merge as-is or with minor follow-ups.
> - **Medium**: Mostly correct but has quality or writing issues fixable during this review.
> - **Low**: Correctness concerns or significant quality problems that may need rework.
>
> Strengths:
> - [bullet points]
>
> Issues: [list each using the issue presentation format, plus for each:]
> - **Pros/cons**: Tradeoffs of fixing now vs deferring
> - **Recommendation**: Quick fix / Record for follow-up / Informational only
>
> Notable observations: [optional — unusual design choices, clever techniques, or patterns that diverge from the codebase. Omit if nothing stands out.]

### Step 6: Triage and fix

Classify each issue from Steps 1b–5 into two categories:

**Mechanical fixes** — issues with an obvious, unambiguous fix (e.g., alphabetical ordering, `cargo fmt`, removing blacklisted files, updating a renamed API call, fixing a merge conflict). Apply these immediately without asking. Commit and report what was done.

**Uncertain fixes** — issues where the correct action is unclear, involves design judgment, or could change behavior (e.g., incorrect algorithm, missing edge case handling, questionable mathematical correctness, ambiguous issue compliance). Present these to the reviewer using `AskUserQuestion`.

Summarize all findings from Steps 1b–5:

| Aspect | Result |
|--------|--------|
| Agentic review | [N open / all addressed] |
| Usefulness | [Useful/Marginal/Not useful] |
| Safety | [Safe/Concerns found] |
| Completeness | [Complete/Missing: X, Y] |
| Merge confidence | [High/Medium/Low] |
| PR URL | [link] |

Then report:

> **Mechanical fixes applied:**
> - [list each fix already committed, or "None"]
>
> **Issues requiring reviewer input:**
> For each uncertain issue, present fix options with pros/cons and a recommendation:
> **N. [Short title]** (`file:line`)
> - **Option A**: [description] — Pros: ... / Cons: ...
> - **Option B**: [description] — Pros: ... / Cons: ...
> - **Recommendation**: Option [X] — [one-sentence justification]
>
> If no uncertain issues: "No uncertain issues."

If there are uncertain issues, use `AskUserQuestion` with your recommendations for the reviewer to confirm or override.

If all issues were mechanical and already fixed, skip the question and proceed to Step 7.

### Step 7: Final decision

If all issues were mechanical and already fixed (no uncertain issues remain), skip the question and proceed directly to Step 8 (Push and fix CI). Only ask the reviewer when there are uncertain issues or when the recommendation is OnHold.

Use `AskUserQuestion` only when needed:

> My recommendation: **[Push and fix CI / OnHold]** — [one-sentence justification]
>
> **Final decision:**
> - **1** — Push and fix CI: push all commits, fix any CI failures, then present the merge link
> - **2** — OnHold: keep in OnHold column, post reason

### Step 8: Execute decision

**If Push and fix CI:**
1. **Pre-push verification** — run `make check && make paper` locally before pushing. Both must pass. Fix any failures, commit, then push:
   ```bash
   cd <worktree path>
   make check && make paper
   git push
   ```
2. Wait for CI — but **do not poll excessively**. Check once after ~60 seconds. If CI hasn't triggered or is still pending, note "CI pending, local checks pass" when presenting the merge link (since pre-push already verified). The reviewer can admin-merge or wait for CI at their discretion. Do not spend more than 1–2 CI poll attempts. If CI does run and fails, fix the issues, commit, and push again.
3. If any follow-up items were noted during the review, post them as a comment:
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
4. Approve the PR (may fail if you are the PR author — that's OK):
   ```bash
   gh pr review <number> --approve || true
   ```
5. Post a community call validation checklist as a comment on the **linked issue** (not the PR). Substitute actual problem names from the PR diff (no angle-bracket placeholders). Example for a rule PR adding `Satisfiability` → `MaximumIndependentSet`:
   ````bash
   COMMENT_FILE=$(mktemp)
   cat > "$COMMENT_FILE" <<'EOF'
   Please kindly check the following items (PR #123):
   - [ ] **Paper** ([PDF](https://github.com/CodingThrust/problem-reductions/blob/main/docs/paper/reductions.pdf)): check definition, proof sketch, example figure, and reproducible `pred` commands
   - [ ] **Implementation (Optional)**: spot-check the source files changed in this PR for correctness

   💬 Join the discussion on [Zulip](https://julialang.zulipchat.com/#narrow/channel/365542-problem-reductions) — feel free to ask questions or leave feedback there.
   EOF
   gh issue comment <ISSUE_NUMBER> --body-file "$COMMENT_FILE"
   rm -f "$COMMENT_FILE"
   ````
   If there is no linked issue, post the checklist as a PR comment instead.
6. Present the PR link for the reviewer to merge:
   > CI green, commits pushed, PR approved. Community call checklist posted on #<ISSUE_NUMBER>. Please merge when ready:
   > **<PR URL>**
7. After the reviewer merges, use `AskUserQuestion` to confirm:
   > **Merged? (continue to move card & cleanup worktree)** Once confirmed, I will move the board item to Done and clean up the worktree.
   > - "Yes" — proceed with cleanup
8. Move the project board item to `Done` and clean up:
   ```bash
   python3 scripts/pipeline_board.py move <ITEM_ID> done
   cd "$REPO_ROOT"
   python3 scripts/pipeline_worktree.py cleanup --worktree "$WORKTREE_DIR"
   ```

**If OnHold:**
The item is already in the OnHold column (claimed in Step 0a), so no board move is needed.
1. Ask the reviewer for the reason (use `AskUserQuestion` with free text).
2. Post a comment on the PR (or linked issue) with the reason:
   ```bash
   COMMENT_FILE=$(mktemp)
   printf '**On Hold**: %s\n' "<reason>" > "$COMMENT_FILE"
   python3 scripts/pipeline_pr.py comment --repo "$REPO" --pr "<number>" --body-file "$COMMENT_FILE"
   rm -f "$COMMENT_FILE"
   ```
3. Clean up the worktree:
   ```bash
   cd "$REPO_ROOT"
   python3 scripts/pipeline_worktree.py cleanup --worktree "$WORKTREE_DIR"
   ```

## Pipeline Script Subcommands

Only use subcommands that exist. Available subcommands per script:

| Script | Subcommands |
|--------|-------------|
| `pipeline_board.py` | `next`, `claim-next`, `ack`, `list`, `move`, `backlog` |
| `pipeline_pr.py` | `context`, `current`, `snapshot`, `comments`, `ci`, `wait-ci`, `codecov`, `linked-issue`, `create`, `comment`, `edit-body` |
| `pipeline_worktree.py` | `enter`, `create-issue`, `prepare-issue-branch`, `checkout-pr`, `prepare-review`, `merge-main`, `cleanup` |
| `pipeline_skill_context.py` | `review-pipeline`, `final-review`, `review-implementation`, `project-pipeline` |
| `pipeline_checks.py` | `detect-scope`, `file-whitelist`, `completeness`, `review-context`, `issue-guards`, `issue-context` |
