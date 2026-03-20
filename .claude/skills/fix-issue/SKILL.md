---
name: fix-issue
description: Fix quality issues found by check-issue — auto-fixes mechanical problems, brainstorms substantive issues with human, then re-checks and moves to Ready
---

# Fix Issue

Fix errors and warnings from a `check-issue` report. Auto-fixes mechanical issues, brainstorms substantive ones with the human, edits the issue body, re-checks once, then asks the human to approve or iterate.

**This skill only edits GitHub issues via `gh`.** No git operations, no commits, no pushes.

## Invocation

```bash
/fix-issue <issue-number>
```

## Constants

GitHub Project board IDs:

| Constant | Value |
|----------|-------|
| `PROJECT_ID` | `PVT_kwDOBrtarc4BRNVy` |
| `STATUS_FIELD_ID` | `PVTSSF_lADOBrtarc4BRNVyzg_GmQc` |
| `STATUS_READY` | `f37d0d80` |
| `STATUS_ON_HOLD` | `48dfe446` |

## Autonomous Mode

This skill is **interactive** — it requires human input for substantive issues and final approval. Mechanical auto-fixes are applied silently, but always presented for review before pushing to GitHub.

## Steps

### 0. Fetch Issue and Check Comment

```bash
gh issue view <NUMBER> --json title,body,labels,comments
```

Find the **most recent** comment starting with `## Issue Quality Check`. If none exists, run `/check-issue <NUMBER>` first, then re-fetch.

### 1. Load Context

#### 1a. Parse check report

Extract from the check comment's summary table:

| Field | How to extract |
|-------|---------------|
| Check name | First column (Usefulness, Non-trivial, Correctness, Well-written) |
| Result | Second column (Pass / Fail / Warn) |
| Details | Third column (one-line summary) |

Then parse each `### <Check Name>` section for full explanations and `#### Recommendations` for suggestions.

Build a structured list of **all issues to fix** — both `Fail` and `Warn`. Warnings are not ignorable.

#### 1b. Codebase check

Only for `[Rule]` issues, run `pred show` on source and target problems to understand existing size fields, variants, and getters — needed for fixing metric names and overhead expressions.

### 2. Classify & Draft Fixes

Tag each issue from Step 1a as `mechanical` or `substantive` using the [Classification Reference](#classification-reference) below.

**For each mechanical issue:**

1. Identify the exact section in the issue body that needs editing
2. Apply the fix to a local draft (do NOT edit GitHub yet)
3. Record what was changed

Use `pred show <problem> --json` to look up valid problem names, `size_fields`, existing variants.

**Collect substantive issues** for discussion in Step 3.

### 3. Present to Human

#### 3a. Show auto-fixes

```
## Auto-fixes applied

| # | Section | Issue | Fix |
|---|---------|-------|-----|
| 1 | Size Overhead | Symbol `m` undefined | Added ... |
```

#### 3b. Brainstorm substantive issues (one at a time)

For each substantive issue:

1. State the problem clearly
2. Offer 2-3 concrete options with your recommendation
3. Wait for the human's response
4. Apply the chosen fix to the draft

Use web search if needed for complexity bounds, algorithm claims, or references.

After all issues are resolved, show the complete updated issue body (or a diff summary if long).

### 4. Re-check & Approve Loop

#### 4a. Re-check locally

Re-run the 4 quality checks (Usefulness, Non-trivial, Correctness, Well-written) against the **draft issue body**. Use `pred show`, `pred path`, web search as needed. Do NOT post a GitHub comment.

Print results as a summary table.

#### 4b. Ask human for decision

Show the draft issue body, then use `AskUserQuestion`:

> The issue has been re-checked locally. What would you like to do?
>
> 1. **Looks good** — I'll push the edits to GitHub, update labels, and move to Ready
> 2. **Modify again** — tell me what else you'd like to change

**If human picks 2:** Ask what to change (free-form), apply changes, return to 4a.

### 5. Finalize

Only reached when the human approves.

#### 5a. Edit the issue body

Save the updated body to `/tmp/fix_issue_body.md` via the Write tool, then:

```bash
gh issue edit <NUMBER> --body-file /tmp/fix_issue_body.md
```

#### 5b. Post changelog comment

```bash
gh issue comment <NUMBER> --body "$(cat <<'EOF'
## Fix-issue changelog

- <bullet for each change made>
- ...

Applied by `/fix-issue`.
EOF
)"
```

#### 5c. Update labels and board

```bash
gh issue edit <NUMBER> --remove-label "Useless,Trivial,Wrong,PoorWritten" 2>/dev/null
gh issue edit <NUMBER> --add-label "Good"
```

To move on the project board, first look up the **project item ID** (not the issue number):

```bash
gh project item-list 8 --owner CodingThrust --limit 200 | grep "<NUMBER>"
# Output: Issue	[Rule] Foo to Bar	<NUMBER>	CodingThrust/problem-reductions	PVTI_...
```

Then move using the `PVTI_...` item ID:

```bash
uv run --project scripts scripts/pipeline_board.py move <PVTI_ITEM_ID> Ready
```

#### 5d. Confirm

```text
Done! Issue #<NUMBER>:
  - Body updated on GitHub
  - Labels: removed failure labels, added "Good"
  - Board: moved to Ready
```

---

## Classification Reference

### Mechanical (auto-fixable)

| Issue pattern | Fix strategy |
|--------------|-------------|
| Undefined symbol in overhead/algorithm | Add definition derived from context (e.g., "let n = \|V\|") |
| Inconsistent notation across sections | Standardize to the most common usage in the issue |
| Missing/wrong code metric names | Look up correct names via `pred show <target> --json` -> `size_fields` |
| Formatting issues (broken tables, missing headers) | Reformat to match issue template |
| Incomplete `(TBD)` in fields derivable from other sections | Fill from context |
| Incorrect DOI format (link syntax only) | Reformat to `https://doi.org/...` |

### Substantive (brainstorm with human)

| Issue pattern | Why human input needed |
|--------------|----------------------|
| DOI resolves to wrong paper or paper doesn't contain cited claim | Requires finding a replacement reference via web search |
| Naming decisions (optimization prefix, CamelCase choice, too long name) | Codebase convention judgment call |
| Missing or incorrect complexity bounds | Requires literature verification |
| Missing type dependencies | Architectural decision about codebase |
| Incorrect mathematical claims | Domain expertise needed |
| Incomplete reduction algorithm | Core technical content |
| Incomplete or trivial example | Needs meaningful design, provide 3 options for the human to choose from |
| Decision vs optimization framing | Check associated `[Rule]` issues first — if a rule targets the decision version, implement that; if it targets optimization, implement that; if both exist, split into two separate model issues. Problem modeling choice |
| Ambiguous overhead expressions | Requires understanding the reduction |

---

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Pushing to GitHub before human approves | All edits stay local until human picks "Looks good" |
| Hallucinating paper content for complexity bounds | Use web search; if not found, say so and ask human |
| Using `pred show` on a problem that doesn't exist yet | Check existence first; for new problems, skip metric lookup |
| Overwriting human's original content | Preserve original text; only modify the specific sections flagged |
| Not preserving `<!-- Unverified -->` markers | Keep existing provenance markers; add new ones for AI-filled content |
| Running check-issue more than once per iteration | Re-check exactly once after edits, then ask human |
| Closing the issue | Never close. Labels and board status only |
| Force-pushing or modifying git | This skill only edits GitHub issues via `gh`. No git operations |
| Inventing `pipeline_board.py` subcommands | Only `next`, `claim-next`, `ack`, `list`, `move`, `backlog` exist |
| Skipping code-grounded verification for rule issues | Always read model source files and run round-trip verification before fixing |
| Verifying only forward direction (source->target) | Must also verify reverse: brute-force target optimum maps back to source optimum |
| Using issue's example without checking canonical examples | Always start from `canonical_model_example_specs()` in the source model file |
