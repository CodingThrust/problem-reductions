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

If a specific PR number was provided (`/final-review 42`), use it directly. Otherwise, pick the first open PR from the Final review column:

```bash
# List items in Final review column
python3 scripts/pipeline_board.py list final-review --repo "$REPO" --format json
```

Pick the first open PR from the list. Extract the `PR` number and `ITEM_ID` for later board moves.

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

- **Merge clean** — merge commit is ready locally. Continue with the review.
- **Merge conflicted** — note the conflicts. Continue with the review; decide whether to resolve or hold in Step 5.
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

### Step 1b: Walk Through Agentic Review Findings

The `review-pipeline` skill already posted a structured **Agentic Review Report** as a PR comment (structural check, quality check, agentic feature tests). Read it:

```bash
gh pr view "$PR" --json comments --jq '.comments[] | select(.body | contains("Agentic Review Report")) | .body'
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
> - "Not useful, hold" — move to OnHold (skip remaining steps, go to Step 8)
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
> - "Unsafe, hold" — move to OnHold (skip remaining steps, go to Step 8)
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
- [ ] Canonical model example function in the model file
- [ ] Paper section in `docs/paper/reductions.typ` (`problem-def` entry)
- [ ] `display-name` entry in paper
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
> - "Incomplete, hold" — move to OnHold (skip remaining steps, go to Step 8)
> - Reviewer flags additional gaps or overrides — agent updates the fix plan accordingly
> - "Skip" — skip this check

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

### Step 6: Confirm issues and fix plan

Summarize all findings from Steps 1b–5:

| Aspect | Result |
|--------|--------|
| Agentic review | [N open / all addressed] |
| Usefulness | [Useful/Marginal/Not useful] |
| Safety | [Safe/Concerns found] |
| Completeness | [Complete/Missing: X, Y] |
| Merge confidence | [High/Medium/Low] |
| PR URL | [link] |

Then list all issues found (from Steps 1b–5) with the fix plan for each. Use `AskUserQuestion` to confirm:

> **Issues found and proposed fixes:**
>
> [List each using the issue presentation format. If no issues: "No issues found."]
>
> **Should I proceed with these fixes?**
> - "Yes" — apply all fixes, commit (do not push yet)
> - "Adjust" — reviewer specifies which fixes to change, add, or drop; agent revises and re-presents
> - "Skip fixes" — continue to final decision without fixing
> - "Hold" — too many issues; move to OnHold (go to Step 8)

Quick fixes during final review are normal — they get squash-merged with the PR.

### Step 7: Final decision

After fixes are applied (or skipped), ask the reviewer for the final decision:

Use `AskUserQuestion`:

> My recommendation: **[Push and fix CI / OnHold]** — [one-sentence justification]
>
> **Final decision:**
> - "Push and fix CI" — I will push all commits, fix any CI failures, then present the merge link
> - "OnHold" — move to OnHold column with a reason

### Step 8: Execute decision

**If Push and fix CI:**
1. Push all commits (merge-with-main + any fixes) from the worktree:
   ```bash
   cd <worktree path>
   git push
   ```
2. Wait for CI to complete. If CI fails, fix the issues, commit, and push again. Repeat until CI passes. Common CI issues during final review are expected — just fix them.
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
5. Post a community call validation checklist as a comment on the **linked issue** (not the PR). All CLI commands must be copy-pastable — substitute actual problem names from the PR diff (no angle-bracket placeholders). Example for a rule PR adding `Satisfiability` → `MaximumIndependentSet`:
   ````bash
   COMMENT_FILE=$(mktemp)
   cat > "$COMMENT_FILE" <<'EOF'
   Please kindly check the following items (PR #123):
   - [ ] **Paper** ([PDF](https://github.com/CodingThrust/problem-reductions/blob/main/docs/paper/reductions.pdf)): check definition, proof sketch, and example figure
   - [ ] **CLI demo** (build from source: `cargo install --path problemreductions-cli`):
     ```bash
     pred show Satisfiability
     pred create --example Satisfiability -o instance.json
     pred reduce instance.json Satisfiability MaximumIndependentSet -o reduced.json
     pred solve reduced.json MaximumIndependentSet
     ```
   - [ ] **Implementation (Optional)**: spot-check the source files changed in this PR for correctness
   EOF
   gh issue comment <ISSUE_NUMBER> --body-file "$COMMENT_FILE"
   rm -f "$COMMENT_FILE"
   ````
   For model PRs, omit the `pred reduce` / `pred solve` lines. If there is no linked issue, post the checklist as a PR comment instead.
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
1. Ask the reviewer for the reason (use `AskUserQuestion` with free text).
2. Post a comment on the PR (or linked issue) with the reason:
   ```bash
   COMMENT_FILE=$(mktemp)
   printf '**On Hold**: %s\n' "<reason>" > "$COMMENT_FILE"
   python3 scripts/pipeline_pr.py comment --repo "$REPO" --pr "<number>" --body-file "$COMMENT_FILE"
   rm -f "$COMMENT_FILE"
   ```
3. Move the project board item to `OnHold` and clean up:
   ```bash
   python3 scripts/pipeline_board.py move <ITEM_ID> on-hold
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
