---
name: review-pipeline
description: Agentic review for PRs in the Review pool — runs structural, quality, and agentic-test sub-reviews (no code changes), posts combined verdict, moves to Final review
---

# Review Pipeline

Pick PRs from the `Review pool` column on the [GitHub Project board](https://github.com/orgs/CodingThrust/projects/8/views/1). For each PR: claim it into `Under review`, run three read-only sub-reviews in parallel (structural check, quality check, agentic feature tests), post a combined verdict as a PR comment, then move to `Final review`.

**This skill does NOT modify the PR.** No commits, no pushes, no merging main. It only evaluates and reports.

## Invocation

- `/review-pipeline` -- pick the next Review pool item
- `/review-pipeline 570` -- process a specific PR number

For Codex, open this `SKILL.md` directly and treat the slash-command forms above as aliases. The Makefile `run-review` target already does this translation.

## Constants

GitHub Project board IDs (for `gh project item-edit`):

| Constant | Value |
|----------|-------|
| `PROJECT_ID` | `PVT_kwDOBrtarc4BRNVy` |
| `STATUS_FIELD_ID` | `PVTSSF_lADOBrtarc4BRNVyzg_GmQc` |
| `STATUS_REVIEW_POOL` | `7082ed60` |
| `STATUS_UNDER_REVIEW` | `f04790ca` |
| `STATUS_FINAL_REVIEW` | `51a3d8bb` |

## Prerequisites

- **agentic-tests** must be installed (`~/.claude/commands/agentic-tests:test-feature.md` must exist). If missing, STOP with: `agentic-tests not installed. Run: gh clone GiggleLiu/agentic-tests ~/.claude/agentic-tests && mkdir -p ~/.claude/commands && ln -s ~/.claude/agentic-tests/skills/test-feature/SKILL.md ~/.claude/commands/agentic-tests:test-feature.md`

## Autonomous Mode

This skill runs **fully autonomously** except for one case: if a Review pool card links multiple repo PRs and the intended target is unclear, STOP and ask the user which PR is the intended target.

## Steps

### 0a. Triage Review Pool

Before spending the expensive full-context packet, do one lightweight Review-pool scan:

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
QUEUE=$(python3 scripts/pipeline_board.py list review --repo "$REPO" --format json)
```

If `PR` was explicitly supplied (for example `/review-pipeline 570`), do **not** pick a different item from the queue. Triage only that PR.

Pick one candidate with a lightweight heuristic:
- prefer direct PR cards over issue cards
- any open PR in Review pool is eligible
- if multiple candidates are tied, pick one at random (e.g., use the current minute mod candidate count) to avoid always picking the same item on retries

**Review-ready criteria** — a PR is ready for review if all of these hold:
- PR state is `OPEN` (not draft, not closed)
- The diff contains at least one model file (`src/models/`) or rule file (`src/rules/`) or other substantive code
- A test file exists for the new code
- The PR body does not say "WIP" or "DO NOT REVIEW"

If the PR is not review-ready, post a diagnostic comment and move to Final review for human triage:

```bash
gh pr comment <PR_NUMBER> --body "review-pipeline: PR not review-ready. <brief concrete reason>. Skipping full review, moving to Final review for human triage."
python3 scripts/pipeline_board.py move <ITEM_ID> final-review
```

For untargeted runs, then return to Step 0a to pick another item.
For explicit `PR` runs, STOP after reporting.

If no candidate is both open and ready for review, STOP with `No Review pool PRs are currently ready for review-pipeline processing.`

### 0b. Create Worktree and Generate Reports

Only after Step 0a has identified a review-ready PR should you create a worktree and spend the expensive context packets.

**Create worktree and check out the PR branch:**

```bash
REPO_ROOT=$(pwd)
WORKTREE_JSON=$(python3 scripts/pipeline_worktree.py enter --name "review-pr-$PR" --format json)
WORKTREE_DIR=$(printf '%s\n' "$WORKTREE_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['worktree_dir'])")
cd "$WORKTREE_DIR"
gh pr checkout "$PR"
```

**Generate both reports** (inside the worktree):

The two expensive context calls are allowed exactly once each per top-level `review-pipeline` invocation:

```bash
# 1. Review-pipeline context (selection, comments, CI, linked issue)
set -- python3 scripts/pipeline_skill_context.py review-pipeline --repo "$REPO" --pr "$PR" --format text
REPORT=$("$@")
printf '%s\n' "$REPORT"

# 2. Review-implementation context (scope, checklists, diff, issue compliance)
IMPL_REPORT=$(python3 scripts/pipeline_skill_context.py review-implementation --repo-root . --format text)
printf '%s\n' "$IMPL_REPORT"
```

The review-pipeline report should already include:
- Selection: board item, PR number, linked issue, title, URL
- Recommendation Seed: suggested mode and deterministic blockers
- Comment Summary
- CI / Coverage
- PR head branch
- Linked Issue Context

The review-implementation report should already include:
- Review Range: base SHA, head SHA
- Scope: review type (model/rule/generic), subject metadata
- Deterministic Checks: whitelist + completeness status
- Changed Files and Diff Stat
- Linked Issue Context

Both reports are reused for the rest of the skill — do not regenerate either.

Branch from the review-pipeline report:
- `Bundle status: empty` => the selected PR is no longer eligible; run `cd "$REPO_ROOT" && python3 scripts/pipeline_worktree.py cleanup --worktree "$WORKTREE_DIR"`, then for untargeted runs return to Step 0a, for explicit `PR` runs STOP
- `Bundle status: needs-user-choice` => run `cd "$REPO_ROOT" && python3 scripts/pipeline_worktree.py cleanup --worktree "$WORKTREE_DIR"`, STOP and ask the user which PR is intended
- `Bundle status: ready` => continue

The bundle already handled the mechanical claim step (moved to Under review). Use the identifiers from the report for all subsequent operations.

All subsequent steps run inside the worktree and should read facts from the reports instead of re-fetching them.

### 1. Run Three Sub-Reviews (Parallel)

Run three independent sub-reviews. All three are **read-only** — they evaluate the PR but do NOT commit or push anything. Dispatch them as parallel subagents where possible.

**Pass `IMPL_REPORT` to both structural and quality subagents** so they skip their own context generation step. Include the full text of `IMPL_REPORT` in each subagent prompt with a prefix like:

> The review-implementation context has already been generated. Use this report instead of running `pipeline_skill_context.py review-implementation` yourself:
>
> ```
> <IMPL_REPORT content>
> ```

#### 1a. Structural Check (project-specific)

Invoke `/review-structural` (file: `.claude/skills/review-structural/SKILL.md`) with the pre-generated `IMPL_REPORT`. This runs the model/rule checklists, build checks, semantic review, and issue compliance checks.

**Mathematical correctness is critical.** In addition to the standard structural checks, verify:
- **For rules**: Is the reduction mathematically correct? Trace through the `reduce_to()` logic with a small example and confirm the target instance encodes the same problem. Check that `extract_solution` correctly inverts the mapping. Verify the paper proof sketch is sound — not just present, but logically valid.
- **For models**: Does `evaluate()` correctly compute the objective for the mathematical definition? Are edge cases handled (empty graph, zero weights, infeasible configs)?
- **Overhead expressions**: Manually count the sizes in `reduce_to()` output and verify they match the `overhead = { ... }` formulas.

**Do NOT auto-fix anything.** Collect the output report for Step 2.

#### 1b. Quality Check (generic)

Invoke `/review-quality` (file: `.claude/skills/review-quality/SKILL.md`) with the pre-generated `IMPL_REPORT`. This runs DRY/KISS/HC-LC checks, test quality review, and HCI checks (if CLI changed).

**Do NOT auto-fix anything.** Collect the output report for Step 2.

#### 1c. Agentic Feature Tests

**This step is mandatory — do NOT skip.**

1. **Identify the feature** from the PR title and changed files:
   - `[Model]` PRs: the new problem model name
   - `[Rule]` PRs: the new reduction rule (source -> target)

2. **Invoke `/agentic-tests:test-feature`** (file: `~/.claude/commands/agentic-tests:test-feature.md`) with the identified feature. This simulates a downstream user exercising the feature from docs and examples.

   **Minimum test checklist** for the agentic tester:
   - `pred list` — verify the new model/rule appears in the catalog
   - `pred show <Name>` — verify details display correctly
   - `pred create --example <Name>` — verify example instance creation works
   - `pred solve <instance>` — verify solving works on the example
   - For rules: `pred reduce <source-instance>` — verify reduction produces valid target

3. **Collect the test report.** For each issue found:
   - Reproduce it from the current PR worktree to confirm it's real
   - Classify as: `confirmed` / `not reproducible in current worktree`
   - For confirmed issues, note severity and recommended fix

**Do NOT fix any issues.** Only report them.

### 2. Compose Combined Review Comment

Merge the results from all three sub-reviews into one structured PR comment.

Paste the **structured report section** from each subagent — the formatted output (checklist tables, issue lists, test results), not raw transcripts or internal reasoning. The human in final-review reads these reports to make merge/hold decisions, so every finding matters.

If the report's `Merge Prep` section indicates merge conflicts with main, include a note at the top:

> **Note:** This PR has merge conflicts with `main`. These must be resolved before merging (handled in final-review Step 1).

```bash
COMMENT_FILE=$(mktemp)
cat > "$COMMENT_FILE" <<'EOF'
## Agentic Review Report

### Structural Check

[Paste structured report from `/review-structural` here — full checklist table, build status, semantic review, issue compliance. Do not include internal reasoning.]

---

### Quality Check

[Paste structured report from `/review-quality` here — design principles review, HCI (if applicable), test quality, all issues with severity and file:line references.]

---

### Agentic Feature Tests

[Paste structured report from `/agentic-tests:test-feature` here — test results with all findings, reproduction results, and classifications.]

---

Generated by review-pipeline
EOF
python3 scripts/pipeline_pr.py comment --repo "$REPO" --pr "$PR" --body-file "$COMMENT_FILE"
rm -f "$COMMENT_FILE"
```

The review stage does not judge pass/fail — it reports findings. The human in final-review decides.

### 3. Move PR to Final Review

Always move to Final review — the human decides what to do with the findings:

```bash
python3 scripts/pipeline_board.py move <ITEM_ID> final-review
```

### 4. Clean Up Worktree

```bash
cd "$REPO_ROOT"
python3 scripts/pipeline_worktree.py cleanup --worktree "$WORKTREE_DIR"
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| PR not in Review pool column | Verify status before processing; STOP if not Review pool |
| Processing a closed PR from a stale issue card | Require PR state `OPEN`; skip stale closed PRs |
| Guessing on an issue card with multiple linked repo PRs | Stop, show options to the user, and recommend the most likely correct OPEN PR |
| Committing or pushing changes | This skill is read-only — evaluate and report only, never modify the PR |
| Moving items backward to Ready | Never move backward — always forward to Final review |
| Missing project scopes | Run `gh auth refresh -s read:project,project` |
| Skipping structural check | Always run `/review-structural` — it catches gaps in paper entries, registrations, tests |
| Skipping agentic tests | Always run `/agentic-tests:test-feature` even if CI is green |
| Not checking out the right branch | Use `gh pr checkout <PR_NUMBER>` after `pipeline_worktree.py enter` |
| Worktree left behind on failure | Always run `pipeline_worktree.py cleanup` in Step 4 |
| Working in main checkout | All work happens in the worktree — never modify the main checkout |
| Fixing issues instead of reporting them | The review stage judges, it does not fix. Report findings for human/final-review to act on |
| Pasting raw agent transcripts | Paste the structured report sections only — checklist tables, issue lists, test results — not internal reasoning or scratch work |
| Regenerating context in subagents | Pass `IMPL_REPORT` to structural/quality subagents so they skip `pipeline_skill_context.py review-implementation` |
| Always picking the same PR on retry | Use randomized tie-breaking when multiple candidates are eligible |
