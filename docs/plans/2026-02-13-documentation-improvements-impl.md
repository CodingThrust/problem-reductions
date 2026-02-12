# Documentation Improvements Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Improve Getting Started and Architecture documentation with diagrams, updated API references, and clearer structure.

**Architecture:** Three Typst diagrams (using fletcher) compiled to SVG, then referenced in markdown docs. Getting Started focuses on workflow; Architecture focuses on internals and contribution paths.

**Tech Stack:** Typst with fletcher, mdBook markdown, SVG output

---

## Task 1: Create Reduction Workflow Diagram

**Files:**
- Create: `docs/src/static/reduction-workflow.typ`
- Output: `docs/src/static/reduction-workflow.svg`, `docs/src/static/reduction-workflow-dark.svg`

**Step 1: Create the Typst diagram**

Create `docs/src/static/reduction-workflow.typ`:

```typst
#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Noto Sans CJK SC")

#let reduction-workflow(dark: false) = {
  let (fg, box-fill) = if dark {
    (rgb("#e2e8f0"), rgb("#1e293b"))
  } else {
    (rgb("#1e293b"), rgb("#f8fafc"))
  }

  set text(fill: fg, size: 10pt)

  diagram(
    node-stroke: 1.5pt + fg,
    edge-stroke: 1.5pt,
    spacing: (20mm, 10mm),

    let accent = rgb("#3b82f6"),
    let success = rgb("#22c55e"),

    // Nodes
    node((0, 0), box(width: 28mm, align(center)[*Problem A*\ #text(size: 8pt)[source problem]]), fill: box-fill, corner-radius: 6pt, inset: 10pt, name: <a>),
    node((1, 0), box(width: 28mm, align(center)[*Problem B*\ #text(size: 8pt)[target problem]]), fill: box-fill, corner-radius: 6pt, inset: 10pt, name: <b>),
    node((2, 0), box(width: 28mm, align(center)[*Solution B*\ #text(size: 8pt)[solver output]]), fill: box-fill, corner-radius: 6pt, inset: 10pt, name: <sol-b>),
    node((1, 1), box(width: 28mm, align(center)[*Solution A*\ #text(size: 8pt)[extracted result]]), fill: rgb("#dcfce7"), stroke: 1.5pt + success, corner-radius: 6pt, inset: 10pt, name: <sol-a>),

    // Edges with labels
    edge(<a>, <b>, "->", stroke: 1.5pt + accent, label: text(size: 9pt)[`reduce_to()`], label-pos: 0.5, label-side: center),
    edge(<b>, <sol-b>, "->", stroke: 1.5pt + accent, label: text(size: 9pt)[`find_best()`], label-pos: 0.5, label-side: center),
    edge(<sol-b>, <sol-a>, "->", stroke: 1.5pt + success, label: text(size: 9pt)[`extract_solution()`], label-pos: 0.5, label-side: center),
  )
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#reduction-workflow(dark: standalone-dark)
```

**Step 2: Compile to SVG (light and dark)**

```bash
cd docs/src/static
typst compile reduction-workflow.typ --input dark=false reduction-workflow.svg
typst compile reduction-workflow.typ --input dark=true reduction-workflow-dark.svg
```

**Step 3: Verify output**

```bash
ls -la docs/src/static/reduction-workflow*.svg
```

Expected: Two SVG files created

**Step 4: Commit**

```bash
git add docs/src/static/reduction-workflow.typ docs/src/static/reduction-workflow.svg docs/src/static/reduction-workflow-dark.svg
git commit -m "docs: add reduction workflow diagram"
```

---

## Task 2: Create Module Overview Diagram

**Files:**
- Create: `docs/src/static/module-overview.typ`
- Output: `docs/src/static/module-overview.svg`, `docs/src/static/module-overview-dark.svg`

**Step 1: Create the Typst diagram**

Create `docs/src/static/module-overview.typ`:

```typst
#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Noto Sans CJK SC")

#let module-overview(dark: false) = {
  let (fg, box-fill) = if dark {
    (rgb("#e2e8f0"), rgb("#1e293b"))
  } else {
    (rgb("#1e293b"), rgb("#f8fafc"))
  }

  set text(fill: fg, size: 10pt)

  diagram(
    node-stroke: 1.5pt + fg,
    edge-stroke: 1.5pt,
    spacing: (25mm, 15mm),

    let model-color = rgb("#c8f0c8"),
    let rule-color = rgb("#c8c8f0"),
    let registry-color = rgb("#f0f0a0"),
    let solver-color = rgb("#f0c8c8"),

    // Module nodes
    node((0, 0), box(width: 30mm, align(center)[*models/*\ #text(size: 8pt)[Problem types]]), fill: model-color, corner-radius: 6pt, inset: 10pt, name: <models>),
    node((1, 0), box(width: 30mm, align(center)[*rules/*\ #text(size: 8pt)[Reductions]]), fill: rule-color, corner-radius: 6pt, inset: 10pt, name: <rules>),
    node((2, 0), box(width: 30mm, align(center)[*registry/*\ #text(size: 8pt)[Graph metadata]]), fill: registry-color, corner-radius: 6pt, inset: 10pt, name: <registry>),
    node((1, 1), box(width: 30mm, align(center)[*solvers/*\ #text(size: 8pt)[BruteForce, ILP]]), fill: solver-color, corner-radius: 6pt, inset: 10pt, name: <solvers>),

    // Relationships
    edge(<models>, <rules>, "<->", label: text(size: 8pt)[imports], label-side: center),
    edge(<rules>, <registry>, "->", label: text(size: 8pt)[registers], label-side: center),
    edge(<solvers>, <models>, "->", label: text(size: 8pt)[solves], label-side: center),
  )
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#module-overview(dark: standalone-dark)
```

**Step 2: Compile to SVG**

```bash
cd docs/src/static
typst compile module-overview.typ --input dark=false module-overview.svg
typst compile module-overview.typ --input dark=true module-overview-dark.svg
```

**Step 3: Verify output**

```bash
ls -la docs/src/static/module-overview*.svg
```

**Step 4: Commit**

```bash
git add docs/src/static/module-overview.typ docs/src/static/module-overview.svg docs/src/static/module-overview-dark.svg
git commit -m "docs: add module overview diagram"
```

---

## Task 3: Create Trait Hierarchy Diagram

**Files:**
- Create: `docs/src/static/trait-hierarchy.typ`
- Output: `docs/src/static/trait-hierarchy.svg`, `docs/src/static/trait-hierarchy-dark.svg`

**Step 1: Create the Typst diagram**

Create `docs/src/static/trait-hierarchy.typ`:

```typst
#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Noto Sans CJK SC")

#let trait-hierarchy(dark: false) = {
  let (fg, box-fill) = if dark {
    (rgb("#e2e8f0"), rgb("#1e293b"))
  } else {
    (rgb("#1e293b"), rgb("#f8fafc"))
  }

  set text(fill: fg, size: 9pt)

  diagram(
    node-stroke: 1.5pt + fg,
    edge-stroke: 1.5pt,
    spacing: (8mm, 12mm),

    let trait-fill = rgb("#e0e7ff"),
    let type-fill = rgb("#fef3c7"),

    // Problem trait (main)
    node((0, 0), box(width: 55mm, align(left)[
      *trait Problem*\
      #text(size: 8pt, fill: rgb("#6b7280"))[
        `const NAME: &str`\
        `type Metric: Clone`\
        `fn dims() -> Vec<usize>`\
        `fn evaluate(&config) -> Metric`\
        `fn variant() -> Vec<(&str, &str)>`
      ]
    ]), fill: trait-fill, corner-radius: 6pt, inset: 10pt, name: <problem>),

    // OptimizationProblem trait
    node((0, 1), box(width: 55mm, align(left)[
      *trait OptimizationProblem*\
      #text(size: 8pt, fill: rgb("#6b7280"))[
        `type Value: PartialOrd + Clone`\
        `fn direction() -> Direction`\
        #text(style: "italic")[requires `Metric = SolutionSize<Value>`]
      ]
    ]), fill: trait-fill, corner-radius: 6pt, inset: 10pt, name: <opt>),

    // Type boxes on the right
    node((1.3, 0), box(width: 38mm, align(left)[
      *SolutionSize\<T\>*\
      #text(size: 8pt, fill: rgb("#6b7280"))[`Valid(T) | Invalid`]
    ]), fill: type-fill, corner-radius: 6pt, inset: 8pt, name: <solsize>),

    node((1.3, 1), box(width: 38mm, align(left)[
      *Direction*\
      #text(size: 8pt, fill: rgb("#6b7280"))[`Maximize | Minimize`]
    ]), fill: type-fill, corner-radius: 6pt, inset: 8pt, name: <dir>),

    // Inheritance arrow
    edge(<opt>, <problem>, "->", stroke: 1.5pt + fg, label: text(size: 8pt)[extends], label-side: center),

    // Type associations (dashed)
    edge(<problem>, <solsize>, "-->", stroke: (paint: fg, dash: "dashed")),
    edge(<opt>, <dir>, "-->", stroke: (paint: fg, dash: "dashed")),
  )
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#trait-hierarchy(dark: standalone-dark)
```

**Step 2: Compile to SVG**

```bash
cd docs/src/static
typst compile trait-hierarchy.typ --input dark=false trait-hierarchy.svg
typst compile trait-hierarchy.typ --input dark=true trait-hierarchy-dark.svg
```

**Step 3: Verify output**

```bash
ls -la docs/src/static/trait-hierarchy*.svg
```

**Step 4: Commit**

```bash
git add docs/src/static/trait-hierarchy.typ docs/src/static/trait-hierarchy.svg docs/src/static/trait-hierarchy-dark.svg
git commit -m "docs: add trait hierarchy diagram"
```

---

## Task 4: Rewrite Getting Started

**Files:**
- Modify: `docs/src/getting-started.md`

**Step 1: Rewrite the file**

Replace contents of `docs/src/getting-started.md` with:

```markdown
# Getting Started

## What This Library Does

**problemreductions** transforms hard computational problems into forms that efficient solvers can handle. You define a problem, reduce it to another problem type (like QUBO or ILP), solve the reduced problem, and extract the solution back. The [interactive reduction graph](./introduction.html) shows all available problem types and transformations.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
problemreductions = "0.1"
```

## The Reduction Workflow

The core workflow is: **create** a problem, **reduce** it to a target, **solve** the target, and **extract** the solution back.

<div class="theme-light-only">

![Reduction Workflow](static/reduction-workflow.svg)

</div>
<div class="theme-dark-only">

![Reduction Workflow](static/reduction-workflow-dark.svg)

</div>

### Complete Example

```rust
use problemreductions::prelude::*;

// 1. Create: Independent Set on a path graph (4 vertices)
let problem = MaximumIndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// 2. Reduce: Transform to Minimum Vertex Cover
let reduction = ReduceTo::<MinimumVertexCover<i32>>::reduce_to(&problem);
let target = reduction.target_problem();

// 3. Solve: Find optimal solution to the target problem
let solver = BruteForce::new();
let target_solutions = solver.find_best(target);

// 4. Extract: Map solution back to original problem
let solution = reduction.extract_solution(&target_solutions[0]);

// Verify: solution is valid for the original problem
let metric = problem.evaluate(&solution);
assert!(metric.is_valid());
```

### Chaining Reductions

Reductions can be chained. Each step preserves the solution mapping:

```rust
use problemreductions::prelude::*;

// SetPacking -> IndependentSet -> VertexCover
let sp = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

let r1 = ReduceTo::<MaximumIndependentSet<i32>>::reduce_to(&sp);
let r2 = ReduceTo::<MinimumVertexCover<i32>>::reduce_to(r1.target_problem());

// Solve final target, extract back through chain
let solver = BruteForce::new();
let vc_sol = solver.find_best(r2.target_problem());
let is_sol = r2.extract_solution(&vc_sol[0]);
let sp_sol = r1.extract_solution(&is_sol);
```

## Solvers

Two solvers are available:

| Solver | Use Case | Notes |
|--------|----------|-------|
| [`BruteForce`](api/problemreductions/solvers/struct.BruteForce.html) | Small instances (<20 variables) | Enumerates all configurations |
| [`ILPSolver`](api/problemreductions/solvers/ilp/struct.ILPSolver.html) | Larger instances | Requires `ilp` feature flag |

Enable ILP support:

```toml
[dependencies]
problemreductions = { version = "0.1", features = ["ilp"] }
```

**Future:** Automated reduction path optimization will find the best route between any two connected problems.

## JSON Resources

The library exports machine-readable metadata useful for tooling and research:

| File | Contents | Use Case |
|------|----------|----------|
| [`reduction_graph.json`](reductions/reduction_graph.json) | All problem variants and reduction edges | Visualization, path finding, research |
| [`problem_schemas.json`](reductions/problem_schemas.json) | Field definitions for each problem type | Code generation, validation |

Generate locally:

```bash
cargo run --example export_graph    # reduction_graph.json
cargo run --example export_schemas  # problem_schemas.json
```

## Next Steps

- Explore the [interactive reduction graph](./introduction.html) to discover available reductions
- Read the [Architecture](./arch.md) guide for implementation details
- Browse the [API Reference](./api.html) for full documentation
```

**Step 2: Verify markdown renders**

```bash
cd docs && mdbook build && echo "Build successful"
```

**Step 3: Commit**

```bash
git add docs/src/getting-started.md
git commit -m "docs: rewrite Getting Started with workflow focus"
```

---

## Task 5: Update Architecture - Module Overview Section

**Files:**
- Modify: `docs/src/arch.md` (partial update)

**Step 1: Read current file to get line numbers**

```bash
head -60 docs/src/arch.md
```

**Step 2: Replace the beginning of arch.md (lines 1-58) with new Module Overview**

Replace the beginning of `docs/src/arch.md` up through the Problems section header with:

```markdown
# Architecture

This guide covers the library internals for contributors and developers extending the library.

## Module Overview

<div class="theme-light-only">

![Module Overview](static/module-overview.svg)

</div>
<div class="theme-dark-only">

![Module Overview](static/module-overview-dark.svg)

</div>

| Module | Purpose |
|--------|---------|
| `src/models/` | Problem type implementations (SAT, Graph, Set, Optimization) |
| `src/rules/` | Reduction rules with `ReduceTo` implementations |
| `src/registry/` | Compile-time reduction graph metadata |
| `src/solvers/` | BruteForce and ILP solvers |
| `src/traits.rs` | Core `Problem` and `OptimizationProblem` traits |
| `src/types.rs` | Shared types (`SolutionSize`, `Direction`, `ProblemSize`) |

## Trait Hierarchy

<div class="theme-light-only">

![Trait Hierarchy](static/trait-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Trait Hierarchy](static/trait-hierarchy-dark.svg)

</div>

Every problem implements `Problem`. Optimization problems additionally implement `OptimizationProblem`.

```rust
pub trait Problem: Clone {
    const NAME: &'static str;          // e.g., "MaximumIndependentSet"
    type Metric: Clone;                // SolutionSize<W> or bool
    fn dims(&self) -> Vec<usize>;      // config space: [2, 2, 2] for 3 binary vars
    fn evaluate(&self, config: &[usize]) -> Self::Metric;
    fn variant() -> Vec<(&'static str, &'static str)>;
}

pub trait OptimizationProblem: Problem<Metric = SolutionSize<Self::Value>> {
    type Value: PartialOrd + Clone;    // i32, f64, etc.
    fn direction(&self) -> Direction;  // Maximize or Minimize
}
```

**Key types:**
- `SolutionSize<T>`: `Valid(T)` for feasible solutions, `Invalid` for constraint violations
- `Direction`: `Maximize` or `Minimize`

## Problems
```

**Step 3: Commit partial update**

```bash
git add docs/src/arch.md
git commit -m "docs: update Architecture with module overview and trait hierarchy"
```

---

## Task 6: Update Architecture - Problems and Rules Sections

**Files:**
- Modify: `docs/src/arch.md` (continue update)

**Step 1: Update the Problems section**

Find and update the Problems section to fix outdated API references. Replace `solution_size(&config)` with `evaluate(&config)`:

Old:
```rust
let config = vec![1, 0, 1, 0];
let result = problem.solution_size(&config);
// result.is_valid: bool
// result.size: objective value
```

New:
```rust
let config = vec![1, 0, 1, 0];
let result = problem.evaluate(&config);
// result.is_valid() -> bool
// result.size() -> Option<&T>
```

**Step 2: Update the Rules section**

Replace the outdated reduction example:

Old:
```rust
let reduction = problem.reduce_to::<QUBO<f64>>();
```

New:
```rust
let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&problem);
```

**Step 3: Remove reference to ConstraintSatisfactionProblem**

Delete the line: "For problems with explicit constraints, also implement `ConstraintSatisfactionProblem`."

**Step 4: Commit**

```bash
git add docs/src/arch.md
git commit -m "docs: fix outdated API references in Architecture"
```

---

## Task 7: Add Contributing Section to Architecture

**Files:**
- Modify: `docs/src/arch.md` (append)

**Step 1: Add Contributing section at the end of arch.md**

Append to `docs/src/arch.md`:

```markdown

## Contributing

### Recommended: Issue-Based Workflow

The easiest way to contribute is through GitHub issues:

1. **Open an issue** using the [Problem](https://github.com/CodingThrust/problem-reductions/issues/new?template=problem.md) or [Rule](https://github.com/CodingThrust/problem-reductions/issues/new?template=rule.md) template
2. **Fill in all sections** — definition, algorithm, size overhead, example instance
3. **AI handles implementation** — automated tools generate the code from your specification

### Optional: Plan + Automated PR

For more control over the implementation:

1. Use `superpowers:brainstorming` to create a detailed plan
2. Create a PR with `[action]` prefix in the description
3. Automated implementation is triggered from your plan

### Manual Implementation

When automation isn't suitable:

- **Adding a problem:** See [adding-models.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-models.md)
- **Adding a reduction:** See [adding-reductions.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-reductions.md)
- **Testing requirements:** See [testing.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/testing.md)

Run `make test clippy` before submitting PRs.
```

**Step 2: Commit**

```bash
git add docs/src/arch.md
git commit -m "docs: add Contributing section to Architecture"
```

---

## Task 8: Build and Verify Documentation

**Files:**
- None (verification only)

**Step 1: Build the mdBook**

```bash
make doc
```

**Step 2: Verify diagrams are included**

```bash
ls -la docs/book/static/*.svg | grep -E "(reduction-workflow|module-overview|trait-hierarchy)"
```

Expected: All 6 SVG files present (light + dark for each)

**Step 3: Open locally and visual check**

```bash
open docs/book/getting-started.html
open docs/book/arch.html
```

Verify:
- Diagrams render correctly
- Light/dark theme switching works
- Links work

**Step 4: Final commit if any fixes needed**

```bash
git status
# If clean, done. If changes needed, fix and commit.
```

---

## Summary

| Task | Description |
|------|-------------|
| 1 | Create reduction workflow diagram (Typst → SVG) |
| 2 | Create module overview diagram |
| 3 | Create trait hierarchy diagram |
| 4 | Rewrite Getting Started |
| 5 | Update Architecture - Module Overview |
| 6 | Update Architecture - Fix outdated API |
| 7 | Add Contributing section |
| 8 | Build and verify |
