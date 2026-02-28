---
name: write-model-in-paper
description: Use when writing or improving a problem-def entry in the Typst paper (docs/paper/reductions.typ)
---

# Write Problem Model in Paper

Full authoring guide for writing a `problem-def` entry in `docs/paper/reductions.typ`. Covers formal definition, background, examples with visualization, and verification.

## Prerequisites

Before using this skill, ensure:
- The problem model is implemented (`src/models/<category>/<name>.rs`)
- The problem is registered with schema and variant metadata
- JSON exports are up to date (`make rust-export && make export-schemas`)

## Reference Example

**MaximumIndependentSet** in `docs/paper/reductions.typ` is the gold-standard model example. Search for `problem-def("MaximumIndependentSet")` to see the complete entry. Use it as a template for style, depth, and structure.

## The `problem-def` Function

```typst
#problem-def("ProblemName")[
  Formal definition...          // parameter 1: def
][
  Background, example, figure...  // parameter 2: body
]
```

**Three parameters:**
- `name` (string) — problem name matching `display-name` dictionary key
- `def` (content) — formal mathematical definition
- `body` (content) — background, examples, figures, algorithm list

**Auto-generated between `def` and `body`:**
- Variant complexity table (from Rust `declare_variants!` metadata)
- Reduction links (from reduction graph JSON)
- Schema field table (from problem schema JSON)

## Step 1: Register Display Name

Add to the `display-name` dictionary near the top of `reductions.typ`:

```typst
"ProblemName": [Display Name],
```

## Step 2: Write the Formal Definition (`def` parameter)

One self-contained sentence or short paragraph. Requirements:

1. **Introduce all inputs first** — graph, weights, sets, variables with their domains
2. **State the objective or constraint** — what is being optimized or satisfied
3. **Define all notation before use** — every symbol must be introduced before it appears

### Pattern for optimization problems

```typst
Given [inputs with domains], find [solution variable] [maximizing/minimizing] [objective] such that [constraints].
```

### Pattern for satisfaction problems

```typst
Given [inputs with domains], find [solution variable] such that [constraints].
```

### Example (MIS)

```typst
Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$
maximizing $sum_(v in S) w(v)$ such that no two vertices in $S$ are
adjacent: $forall u, v in S: (u, v) in.not E$.
```

## Step 3: Write the Body

The body goes AFTER the auto-generated sections (complexity, reductions, schema). It contains four parts in order:

### 3a. Background & Motivation

1-3 sentences covering:
- Historical context (e.g., "One of Karp's 21 NP-complete problems")
- Applications (e.g., "appears in wireless network scheduling, register allocation")
- Notable structural properties (e.g., "Solvable in polynomial time on bipartite graphs, interval graphs, chordal graphs")

If the user provides specific justification or motivation, incorporate it here.

### 3b. Best Known Algorithms (Structured)

List the best known algorithms with complexity and citation. Use this format:

```typst
The best known algorithm runs in $O^*(1.1996^n)$ time via measure-and-conquer
branching @xiao2017.
```

For problems with multiple notable algorithms or special cases, list them:

```typst
Best known: $O(n+m)$ for $k=2$ (bipartiteness testing); $O^*(1.3289^n)$ for
$k=3$ @beigel2005; $O^*(1.7159^n)$ for $k=4$ @wu2024; $O^*(2^n)$ in general
via inclusion-exclusion @bjorklund2009.
```

**Citation rules:**
- Every complexity claim MUST have a citation (`@key`)
- If no verifiable source exists, add footnote: `#footnote[Complexity not independently verified from literature.]`
- Include approximation results where relevant (e.g., "0.878-approximation @goemans1995")

**Consistency note:** The auto-generated complexity table (from `declare_variants!`) also shows complexity per variant. The written text and the auto-generated table may overlap. Keep both — the written text provides references and context; the auto-generated table provides per-variant detail. A future verification step will check consistency between them.

### 3c. Example with Visualization

A concrete small instance that illustrates the problem. Requirements:

1. **Small enough to verify by hand** — readers should be able to check the solution
2. **Include a diagram/graph** using the paper's visualization helpers
3. **Show a valid/optimal solution** and explain why it is valid/optimal
4. **Walk through evaluation** — show how the objective/verifier computes the solution value

Structure:

```typst
*Example.* Consider [instance description with concrete numbers].
[Describe the solution and why it's valid/optimal].

#figure({
  // visualization code — see MaximumIndependentSet for graph rendering pattern
},
caption: [Caption describing the figure with key parameters],
) <fig:problem-example>
```

**For graph problems**, use the paper's existing graph helpers:
- `petersen-graph()`, `house-graph()` or define custom vertex/edge lists
- `canvas(length: ..., { ... })` with `g-node()` and `g-edge()`
- Highlight solution elements with `graph-colors.at(0)` (blue) and use `white` fill for non-solution

Refer to the **MaximumIndependentSet** entry for the complete graph rendering pattern. Adapt it to your problem.

### 3d. Evaluation Explanation

Explain how a configuration is evaluated — this maps to the Rust `evaluate()` method:
- For optimization: show the cost function computation on the example solution
- For satisfaction: show the verifier check on the example solution

This can be woven into the example text (as MIS does: "$w(S) = sum_(v in S) w(v) = 4 = alpha(G)$").

## Step 4: Build and Verify

```bash
# Regenerate exports (if not already done)
make rust-export && make export-schemas

# Build the paper
make paper
```

### Verification Checklist

- [ ] **Display name registered**: entry exists in `display-name` dictionary
- [ ] **Notation self-contained**: every symbol in `def` is defined before first use
- [ ] **Background present**: historical context, applications, or structural properties
- [ ] **Algorithms cited**: every complexity claim has `@citation` or footnote warning
- [ ] **Example present**: concrete small instance with visualization
- [ ] **Evaluation shown**: objective/verifier computed on the example solution
- [ ] **Diagram included**: figure with caption and label for graph/matrix/set visualization
- [ ] **Paper compiles**: `make paper` succeeds without errors
- [ ] **Complexity consistency**: written complexity and auto-generated variant table are compatible (note any discrepancies for later review)
