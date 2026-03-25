---
name: check-issue
description: Use when reviewing a [Rule] or [Model] GitHub issue for quality before implementation — checks usefulness, non-triviality, correctness of literature claims, and writing quality
---

# Check Issue

Quality gate for `[Rule]` and `[Model]` GitHub issues. Runs 4 checks (adapted per issue type) and posts a structured report as a GitHub comment. Adds labels for failures but does NOT close issues.

## Invocation

```
/check-issue <issue-number>
```

## Process

```dot
digraph check_issue {
    rankdir=TB;
    "Fetch issue" [shape=box];
    "Detect issue type" [shape=diamond];
    "Run Rule checks" [shape=box];
    "Run Model checks" [shape=box];
    "Unknown type: stop" [shape=box, style=filled, fillcolor="#ffcccc"];
    "Compose report" [shape=box];
    "Post comment + add labels" [shape=box];

    "Fetch issue" -> "Detect issue type";
    "Detect issue type" -> "Run Rule checks" [label="[Rule]"];
    "Detect issue type" -> "Run Model checks" [label="[Model]"];
    "Detect issue type" -> "Unknown type: stop" [label="other"];
    "Run Rule checks" -> "Compose report";
    "Run Model checks" -> "Compose report";
    "Compose report" -> "Post comment + add labels";
}
```

### Prerequisites

This skill uses the `pred` CLI tool. If `pred` is not available, build it first:

```bash
pred --version 2>/dev/null || make cli
```

### Step 0: Fetch and Parse Issue

```bash
gh issue view <NUMBER> --json title,body,labels,comments
```

- Detect issue type from title: `[Rule]` or `[Model]`
- If neither, stop with message: "This skill only checks [Rule] and [Model] issues."

#### Duplicate Check Detection

Before running checks, scan existing comments for a previous `## Issue Quality Check` heading. If found:
- **Default:** Skip and report: "Already checked (comment from YYYY-MM-DD). Use `/check-issue <NUMBER> --force` to re-check."
- **`--force` flag:** Proceed with re-check. Post the new report as "Re-check" and note any changes from the previous report (e.g., "Previously: 2 warnings → Now: 0 warnings after issue edits").

---

# Part A: Rule Issue Checks

Applies when the title contains `[Rule]`.

## Rule Check 1: Usefulness (fail label: `Useless`)

**Goal:** Is this reduction novel or does it improve on an existing one?

1. Parse **Source** and **Target** problem names from the issue body.

2. Resolve problem aliases (the issue may say "MIS" but `pred` needs "MaximumIndependentSet"):
   ```bash
   pred show <source> --json 2>/dev/null
   pred show <target> --json 2>/dev/null
   ```
   If either fails, try the full name. If still fails, **Warn** (unknown problem — may be a new model).

3. Check existing path:
   ```bash
   pred path <source> <target> --json
   ```

4. Decision (principle: new rule must reduce the reduction overhead):
   - **No path exists** → **Pass** (novel reduction)
   - **Path exists** → run `/topology-sanity-check redundancy <source> <target>` to perform a full overhead dominance analysis against all composite paths. Use its verdict:
     - **Not Redundant** → **Pass** ("improves existing reduction — not dominated by any composite path")
     - **Redundant** (dominated by a composite path) → **Fail** — include the dominating path from the redundancy report
     - **Inconclusive** → **Warn** with the details from the redundancy report

5. Check **Motivation** field: if empty, placeholder, or just "enables X" without explaining *why this path matters* → **Warn**

---

## Rule Check 2: Non-trivial (fail label: `Trivial`)

**Goal:** Does the reduction involve genuine structural transformation?

Read the "Reduction Algorithm" section and flag as **Fail** if:

- **Variable substitution only:** The mapping is a 1-to-1 relabeling (e.g., `x_i → 1 - x_i` for complement problems). A valid reduction must construct new constraints, objectives, or graph structure.
- **Subtype coercion:** The reduction merely casts to a more general type within an existing variant hierarchy (e.g., UnitDiskGraph → SimpleGraph) with no structural change to the problem instance.
- **Same-problem identity:** Reducing between variants of the same problem with no insight (e.g., `MIS<SimpleGraph, One>` → `MIS<SimpleGraph, i32>` by setting all weights to 1).
- **Insufficient detail:** The algorithm is a hand-wave ("map variables accordingly", "follows from the definition") — not a step-by-step procedure a programmer could implement. This is also a **Fail**.

If the construction involves gadgets, penalty terms, auxiliary variables, or non-trivial structural transformation → **Pass**.

Note: if a trivial reduction make original disconnected problems connected, it is also **Pass**.

---

## Rule Check 3: Correctness (fail label: `Wrong`)

**Goal:** Are the cited references real and do they support the claims?

### 3a: Extract References

Parse all literature citations from the issue body — paper titles, author names, years, DOIs, arxiv IDs, textbook references.

### 3b: Cross-check Against Project Knowledge Base

First, read `references.md` (in this skill's directory) for quick fact-checking — it contains known complexity bounds, key results, and established reductions with BibTeX keys. If the issue's claims contradict known facts in this file → **Fail** immediately.

Then read `docs/paper/references.bib` for the full bibliography. If a cited paper is already in the bibliography:
- Verify the claim in the issue matches what the paper actually says
- If the issue cites a result from a known paper but misquotes it → **Fail**

### 3c: External Verification (best-effort fallback chain)

For each reference NOT in the bibliography:

1. **Try arxiv MCP** (if available): search by title/author/arxiv ID
2. **Try Semantic Scholar MCP** (if available): search by title/DOI
3. **Fall back to WebSearch + WebFetch**: search for the paper, fetch abstract

For each reference, verify:
- Paper exists with matching title, authors, and year
- The specific theorem/result cited actually appears in the paper
- Cross-check the claim against at least one other source (survey paper, textbook, or independent reference)

If any cited fact **cannot be verified** (paper not found, claim not in paper) → **Fail** with specifics.

### 3d: Better Algorithm Discovery (not fatal)

While searching, if you find:
- A **more recent paper** that supersedes the cited reference
- A **lower overhead** construction for the same reduction
- A **different approach** that achieves better bounds

→ Include in the report as a **Recommendation** (not a failure). Example: "Note: Smith et al. (2024) improve on this with O(n) overhead instead of O(n^2)."

---

## Rule Check 4: Well-written (fail label: `PoorWritten`)

**Goal:** Is the issue clear, complete, and implementable?

### 4a: Information Completeness

Check all template sections are present and substantive (not placeholder text):

| Section | Required content |
|---------|-----------------|
| Source | Valid problem name |
| Target | Valid problem name |
| Motivation | Why this reduction matters |
| Reference | At least one literature citation |
| Reduction Algorithm | Complete step-by-step procedure |
| Size Overhead | Table with code metric names and formulas |
| Validation Method | How to verify correctness |
| Example | Concrete worked instance |

Missing or placeholder sections → list them as **Fail** items.

### 4b: Algorithm Completeness

The reduction algorithm must be a **complete, step-by-step procedure** detailed enough for a programmer to implement:
- Every step numbered and unambiguous
- All intermediate values defined
- No gaps ("similarly for the remaining variables")
- Solution extraction must be clear from the variable mapping

If the algorithm is a high-level sketch rather than an implementable procedure → **Fail**.

### 4c: Symbol and Notation Consistency

- **All symbols must be defined before first use.** E.g., if the algorithm references `n`, there must be a prior line "let n = |V|".
- **Symbols must be consistent** between sections. If the algorithm defines `G = (V, E)` but the overhead table uses `N` for vertex count without defining it → **Fail**.
- **Code metric names** in the overhead table must match actual getter methods on the target problem (e.g., `num_vertices`, `num_edges`, `num_vars`, `num_clauses`). Check with `pred show <target> --json` → `size_fields`.

### 4d: Example Quality

- **Non-trivial**: Must have enough structure to exercise the reduction meaningfully (not just 2 vertices)
- **Brute-force solvable**: Small enough to verify by hand or with `pred solve`
- **Fully worked**: Shows the source instance, the reduction construction step by step, and the target instance — not just "apply the reduction to get..."
- **Round-trip testable**: The example must be complex enough to validate correctness via a closed-loop test: reduce the source instance → solve the target → extract the solution back → verify it is optimal for the source. A too-simple example (e.g., a single edge, a trivially satisfiable formula) can pass the round trip even with a buggy reduction. The example should have multiple feasible solutions with different objective values so that only a correct reduction maps to the true optimum. Rule of thumb: the source instance should have at least 2 suboptimal feasible solutions in addition to the optimal one.

---

# Part B: Model Issue Checks

Applies when the title contains `[Model]`.

## Model Check 1: Usefulness (fail label: `Useless`)

**Goal:** Does this problem add value to the reduction graph?

1. Parse the **problem name** from the issue body ("Name" field under Definition).

2. Check if the problem already exists:
   ```bash
   pred show <name> --json 2>/dev/null
   ```
   If it succeeds, the problem **already exists** → **Fail** ("Problem already implemented").

3. Check **planned reductions** — the issue must mention at least one concrete reduction rule connecting this problem to the existing graph:
   - Look for explicit statements like "reduces to/from X", "interreducible with Y", or references to planned `[Rule]` issues
   - If **no reduction is mentioned at all** → **Fail** ("Orphan node — a problem without any planned reduction rule has no value in the reduction graph. Add at least one planned reduction to/from an existing problem.")
   - If reductions are mentioned but vague ("can be connected to other problems") → **Warn**

4. Check **Motivation** field:
   - Is there a concrete use case? (quantum computing, network design, scheduling, etc.)
   - If motivation is empty, placeholder, or vague → **Warn**

5. Check **How to solve** section:
   - At least one solver method must be checked (brute-force, ILP reduction, or other)
   - If no solver path is identified → **Warn** ("No solver means reduction rules can't be verified")
   - If direct ILP solving is claimed, the issue must link a direct `[Rule] <ProblemName> to ILP` companion issue in the "Reduction Rule Crossref" section; otherwise → **Fail**

---

## Model Check 2: Non-trivial (fail label: `Trivial`)

**Goal:** Is this genuinely a distinct problem, not a repackaging of an existing one?

Flag as **Fail** if:

- **Isomorphic to existing problem:** The definition is mathematically equivalent to a problem already in the codebase under a different name (e.g., proposing "Maximum Weight Clique" when `MaximumClique` with weights already exists).
- **Trivial variant:** The proposed problem is just an existing problem restricted to a specific graph type or weight type that could be handled by adding a variant to the existing model (e.g., "MIS on bipartite graphs" is a variant, not a new problem).
- **Trivial renaming:** Same feasibility constraints and objective, different name.

Check against existing problems:
```bash
pred list --json
```

If the problem has a genuinely different feasibility constraint or objective function from all existing problems → **Pass**.

---

## Model Check 3: Correctness (fail label: `Wrong`)

**Goal:** Are the definition, complexity claims, and references accurate?

### 3a: Definition Correctness

- Verify the formal definition is mathematically well-formed
- Check that feasibility constraints and objective are clearly separated
- Verify the variable domain matches the problem semantics (binary for selection, k-ary for coloring, etc.)

### 3e: Representation Feasibility

Verify that the proposed data types in the Schema can represent the stated problem domain:
- If the Schema proposes a data type but the Definition or Variants mention domains that exceed that type's range (e.g., proposing integer coefficients for a finite field larger than any fixed-width integer can hold) → **Fail** ("Proposed data type cannot represent the stated domain")
- If multiple variants are listed, check that the proposed schema handles all of them or explicitly restricts scope
- If the issue acknowledges a limitation and restricts scope (e.g., "initial implementation targets small fields only"), this is acceptable → **Pass** with a note

### 3b: Complexity Verification

The issue claims a best-known exact algorithm with a specific time bound. Verify:
- The cited paper/algorithm actually exists (use same fallback chain as Rule Check 3)
- The time bound matches what the paper claims
- For polynomial-time problems: verify they are indeed polynomial (not NP-hard)
- For NP-hard problems: verify the exponential base is correct (e.g., 1.1996^n for MIS, not 2^n)
- **If a better algorithm is found** during search → add as a **Recommendation** in the comment

### 3c: Cross-check Against Project Knowledge Base

Same process as Rule Check 3b — first read `references.md` (in this skill's directory) for quick fact-checking against known complexity bounds and results. Then read `docs/paper/references.bib` and verify claims against known papers.

### 3d: External Verification

Same fallback chain as Rule Check 3c:
1. arxiv MCP → 2. Semantic Scholar MCP → 3. WebSearch + WebFetch

Verify each reference exists and supports the claims made in the issue.

---

## Model Check 4: Well-written (fail label: `PoorWritten`)

**Goal:** Is the issue clear, complete, and implementable?

### 4a: Information Completeness

Check all template sections are present and substantive:

| Section | Required content |
|---------|-----------------|
| Motivation | Concrete use case and graph connectivity |
| Name | Valid problem name following naming conventions |
| Reference | At least one literature citation |
| Definition | Formal: input, feasibility constraints, objective |
| Variables | Count, per-variable domain, semantic meaning |
| Schema | Type name, variants, field table |
| Complexity | Best known algorithm with citation **and** a concrete complexity expression in terms of problem parameters (e.g., `q^n`, `2^{0.8765n}`) |
| How to solve | At least one solver method checked; if ILP is claimed, a direct `[Rule] <ProblemName> to ILP` issue must be linked |
| Example Instance | Concrete instance that exercises the core structure |
| Expected Outcome | Satisfaction: one valid / satisfying solution with brief justification. Optimization: one optimal solution with the optimal objective value |

Missing or placeholder sections → list them as **Fail** items.

### 4b: Definition Completeness

The formal definition must be **precise and implementable**:
- Input structure clearly specified (graph, formula, matrix, etc.)
- Feasibility constraints stated as mathematical conditions
- Objective (if optimization) stated as what to maximize/minimize
- All quantifiers explicit ("for all edges (u,v) in E" not "adjacent vertices don't share colors")

### 4c: Symbol and Notation Consistency

- **All symbols defined before first use.** If the definition uses `G = (V, E)`, the Variables section should reference `V` consistently.
- **Symbols consistent across sections.** The Schema field descriptions must match symbols in the Definition.
- **Naming conventions:** optimization problems must use `Maximum`/`Minimum` prefix. Check against CLAUDE.md naming rules.

### 4d: Example Quality

- **Non-trivial**: Enough vertices/variables to exercise constraints meaningfully (not just a triangle)
- **Exercises core structure**: Examples must use the defining features of the problem. For instance, a "MultivariateQuadratic" example that only has linear terms does not exercise the quadratic structure → **Fail**. If the problem's name or definition highlights a specific structural feature (quadratic, k-colorable, bipartite, etc.), at least one example must exercise that feature.
- **Expected outcome provided**:
  - Satisfaction problems must include a concrete valid / satisfying solution and say why it is valid
  - Optimization problems must include a concrete optimal solution and the optimal objective value
- **Detailed enough for paper**: This example will appear in the paper — it needs to be illustrative
- **Round-trip testable**: The example must be complex enough that a round-trip test (construct instance → solve → verify) can catch implementation bugs. A too-simple instance (e.g., 2 vertices, a single clause) may have a trivially correct solution that passes even with a wrong implementation. The example should have multiple feasible configurations with different objective values (for optimization) or a mix of satisfying and non-satisfying configurations (for satisfaction problems), so that correctness is meaningfully tested. Rule of thumb: the instance should have at least 2 suboptimal feasible solutions in addition to the optimal one.
- **ILP-testable when claimed**: If the issue advertises a direct ILP path, the example should be rich enough to support strong ILP closed-loop tests rather than a degenerate "any formulation passes" case.

### 4e: Representation Feasibility

Same check as Correctness 3e — if the proposed data types cannot represent the stated domain, this is also a **Fail** here (the schema is not implementable as written).

---

# Step 2: Compose and Post Report

### Report Format

Post a single GitHub comment. The table adapts to the issue type:

**For [Rule] issues:**

````markdown
## Issue Quality Check — Rule

| Check | Result | Details |
|-------|--------|---------|
| Usefulness | ✅ Pass | No existing direct reduction Source → Target |
| Non-trivial | ✅ Pass | Gadget construction with penalty terms |
| Correctness | ❌ Fail | Paper "Smith 2020" not found on arxiv or Semantic Scholar |
| Well-written | ⚠️ Warn | Symbol `m` used in overhead table but not defined in algorithm |

**Overall: 2 passed, 1 failed, 1 warning**

---

### Usefulness
[Detailed explanation]

### Non-trivial
[Detailed explanation]

### Correctness
[Per-reference verification results, any better algorithms found]

### Well-written
[Specific items to fix]

#### Recommendations
- [Better algorithms or papers discovered]
- [Suggestions for improving the issue]
````

**For [Model] issues:**

````markdown
## Issue Quality Check — Model

| Check | Result | Details |
|-------|--------|---------|
| Usefulness | ✅ Pass | Novel problem not yet in reduction graph |
| Non-trivial | ✅ Pass | Distinct feasibility constraints from existing problems |
| Correctness | ⚠️ Warn | Complexity bound not independently verified |
| Well-written | ❌ Fail | Missing Variables section; symbol `K` undefined |

**Overall: 2 passed, 1 warning, 1 failed**

---

### Usefulness
[Detailed explanation]

### Non-trivial
[Detailed explanation]

### Correctness
[Per-reference verification, complexity check results, better algorithms found]

### Well-written
[Specific items to fix]

#### Recommendations
- [Better complexity bounds discovered]
- [Suggestions for improving the issue]
````

### Label Application

```bash
# Add labels for FAILED checks (not warnings)
gh issue edit <NUMBER> --add-label "Useless"     # if Check 1 failed
gh issue edit <NUMBER> --add-label "Trivial"      # if Check 2 failed
gh issue edit <NUMBER> --add-label "Wrong"        # if Check 3 failed
gh issue edit <NUMBER> --add-label "PoorWritten"  # if Check 4 failed

# "Good" label requires: zero failures AND zero warnings on Usefulness or Correctness.
# Warnings on Non-trivial or Well-written alone do NOT block "Good".
gh issue edit <NUMBER> --add-label "Good"

# If re-checking after fixes, remove stale failure labels and add "Good" if now passing
gh issue edit <NUMBER> --remove-label "Useless,Trivial,Wrong,PoorWritten" 2>/dev/null
gh issue edit <NUMBER> --add-label "Good"
```

**Never close the issue.** Labels and comments only.

### Comment Posting

```bash
gh issue comment <NUMBER> --body "$(cat <<'EOF'
<report content>
EOF
)"
```

---

## Tool Fallback Chain for Literature

| Priority | Tool | Use for |
|----------|------|---------|
| 1 | arxiv MCP | arxiv papers (search by ID, title, author) |
| 2 | Semantic Scholar MCP | DOI lookup, citation graphs, abstracts |
| 3 | WebSearch | General paper search, cross-referencing |
| 4 | WebFetch | Fetch specific paper pages for claim verification |

If an MCP tool is not available, skip to the next in the chain. All checks should be possible with just WebSearch + WebFetch as a baseline.

---

## Step 3: Offer to Fix (optional)

After posting the report, if there are **fixable failures** (not just warnings), ask the user:

> "Would you like me to help fix the issues found? I can update the issue body to address: [list fixable items]"

**Auto-fixable items** (if the user agrees):
- Missing or placeholder sections → fill with templates from the issue template
- Incorrect DOI format → reformat to standard `https://doi.org/...` form
- Inconsistent notation → standardize symbols across sections
- Missing symbol definitions → add definitions based on context

**NOT auto-fixable** (require the contributor's input):
- Missing reduction algorithm or proof details
- Incorrect mathematical claims
- Missing references (need the contributor to provide them)

If the user agrees, edit the issue body with `gh issue edit <NUMBER> --body "..."` and re-run the checks to verify the fixes.

---

## Common Mistakes

- **Don't fail on warnings.** Only add labels for definitive failures. Ambiguous cases get warnings.
- **Don't close issues.** This skill labels and comments only.
- **Don't hallucinate paper content.** If you can't find a paper, say "not found" — don't guess what it might contain.
- **Don't hallucinate issue references.** Do NOT reference other GitHub issues unless you have fetched them with `gh issue view` and verified their content. Do NOT reference file paths unless you have verified they exist.
- **Match problem names carefully.** Issues may use aliases (MIS, MVC, SAT) that need resolution via `pred show`.
- **Check the right template.** `[Rule]` and `[Model]` issues have different sections — don't check for "Reduction Algorithm" on a Model issue.
