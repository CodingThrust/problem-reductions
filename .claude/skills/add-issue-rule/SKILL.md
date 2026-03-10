---
name: add-issue-rule
description: Use when filing a GitHub issue for a new reduction rule, ensuring all template sections are complete with citations, worked examples, and correctness verification
---

# Add Issue — Rule

File a `[Rule]` GitHub issue on CodingThrust/problem-reductions using the upstream "Rule" issue template. Ensures all sections are complete, with citations, a worked example, and a validation method.

## Input

The caller (zero-to-infinity or user) provides:
- Source problem name
- Target problem name
- Reference URLs (if available)

## Step 1: Verify Non-Existence

Before anything else, confirm the rule doesn't already exist:

```bash
# Check implemented rules (filename pattern: source_target.rs)
ls src/rules/ | grep -i "<source_lowercase>.*<target_lowercase>"

# Check open issues
gh issue list --state open --limit 200 --json title,number | grep -i "<source>.*<target>"

# Check closed issues
gh issue list --state closed --limit 200 --json title,number | grep -i "<source>.*<target>"
```

**If found:** STOP. Report to caller that this rule already exists.

**Also verify both source and target models exist or have open issues:**
```bash
ls src/models/*/ | grep -i "<source_lowercase>"
ls src/models/*/ | grep -i "<target_lowercase>"
```

If a model doesn't exist and has no open issue, report it. The caller should file model issues first.

## Step 2: Research and Fill Template Sections

Use `WebSearch` and `WebFetch` to fill all sections from the upstream template (`.github/ISSUE_TEMPLATE/rule.md`):

| Section | What to fill | Guidance |
|---------|-------------|----------|
| **Source** | Source problem name | Must exist in repo or have open issue. Browse: https://codingthrust.github.io/problem-reductions/ |
| **Target** | Target problem name | Must exist in repo or have open issue |
| **Motivation** | One sentence: why is this reduction useful? | E.g. "Enables solving MIS on quantum annealers via QUBO formulation." |
| **Reference** | URL, paper, or textbook citation | Must be a real, accessible reference |
| **Reduction Algorithm** | Three parts: (1) Define notation — list ALL symbols for source and target instances. (2) Variable mapping — how source variables map to target variables. (3) Constraint/objective transformation — formulas, penalty terms, etc. Use LaTeX math (`$...$` inline, `$$...$$` display). | Solution extraction follows from variable mapping, no need to describe separately |
| **Size Overhead** | Table: `\| Target metric (code name) \| Polynomial (using symbols) \|` | Code names must match the target problem's getter methods (e.g., `num_vertices`, `num_edges`) |
| **Validation Method** | How to verify correctness beyond closed-loop testing | E.g. compare with ProblemReductions.jl, external solver, known results |
| **Example** | Small but non-trivial source instance for the paper illustration | Must be small enough for brute-force but large enough to exercise the reduction meaningfully. Provide as many details as possible — this appears in the paper and is used by AI to generate example code. |

**Citation rule:** Every claim MUST include a URL.

**Formatting rule:** All mathematical expressions MUST use GitHub LaTeX rendering: `$...$` for inline math (e.g., $G=(V,E)$, $x_i$, $Q_{ij}$) and `$$...$$` for display equations. Never use plain text for math.

## Step 3: Verify Example Correctness

For the Example section:
- Walk through the reduction step-by-step
- Show: source instance → apply reduction → target instance → solve target → verify solution maps back
- The example must be small enough to verify by hand (e.g., Petersen graph for graph problems)
- Provide concrete numbers, not just descriptions

## Step 4: Verify Nontriviality

The rule must be **nontrivial** (per issue #127 standards):
- NOT a simple identity mapping or type cast
- NOT a trivial embedding (just copying data)
- NOT a weight type conversion (i32 → f64)
- MUST involve meaningful structural transformation

If the rule is trivial, STOP and report to caller.

## Step 5: Draft and File Issue

Draft the issue body matching the upstream template format exactly:

```bash
gh issue create --repo CodingThrust/problem-reductions \
  --title "[Rule] Source to Target" \
  --label "rule" \
  --body "$(cat <<'ISSUE_EOF'
**Source:** SourceProblem
**Target:** TargetProblem
**Motivation:** <one sentence>
**Reference:** [citation](url)

## Reduction Algorithm

**Notation:**
- Source instance: $G=(V,E)$, $n=|V|$, $m=|E|$
- Target instance: ...

**Variable mapping:**
<how source variables map to target variables, using LaTeX: $x_i$, $Q_{ij}$, etc.>

**Constraint/objective transformation:**
<formulas in LaTeX, penalty terms, proof of correctness>

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | $n = |V|$ |
| `num_edges` | $m + \ldots$ |

## Validation Method

<how to verify correctness beyond closed-loop testing>

## Example

<small but non-trivial source instance, worked step-by-step>

Source: <describe instance>
Reduction: <show transformation>
Target: <describe resulting instance>
Solution: <solve and verify>
ISSUE_EOF
)"
```

Report the created issue number and URL.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Using custom format instead of template | Must match `.github/ISSUE_TEMPLATE/rule.md` sections exactly |
| Filing trivial reductions | Check nontriviality in Step 4 |
| Missing model dependency | Verify both source and target exist in Step 1 |
| Example too complex or too trivial | Small enough for brute-force, large enough to be meaningful (e.g., Petersen graph) |
| Undefined symbols in algorithm | Define ALL notation before using it |
| Missing validation method | Must describe how to cross-check beyond closed-loop |
| Wrong overhead code names | Must match actual getter methods on target type |
| Missing label | Use `--label "rule"` to match template metadata |
| Plain text math | Use LaTeX: `$G=(V,E)$` not `G=(V,E)`, `$\sum w_{ij}$` not `sum w_ij` |
