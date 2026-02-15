# Polish design.md Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rewrite `docs/src/design.md` following the "Follow the Data" structure: fill empty sections, update outdated content, and create a coherent contributor-oriented narrative.

**Architecture:** Single-file rewrite of `docs/src/design.md`. Organized as 11 sections tracing the lifecycle of a reduction. All diagrams are preserved as-is. No code changes, only documentation.

**Tech Stack:** Markdown (mdBook), existing SVG diagrams, code snippets from the Rust source.

---

### Task 1: Rewrite sections 1-2 (Module Overview + Problem Model)

**Files:**
- Modify: `docs/src/design.md:1-44`

**Step 1: Replace lines 1-44 with updated Module Overview and Problem Model**

Replace the entire file content from line 1 through line 44 (end of trait hierarchy diagram) with:

```markdown
# Design

This guide covers the library internals for contributors.

## Module Overview

<div class="theme-light-only">

![Module Overview](static/module-overview.svg)

</div>
<div class="theme-dark-only">

![Module Overview](static/module-overview-dark.svg)

</div>

| Module | Purpose |
|--------|---------|
| [`src/models/`](#problem-model) | Problem type implementations (SAT, Graph, Set, Optimization) |
| [`src/rules/`](#reduction-rules) | Reduction rules with `ReduceTo` implementations |
| [`src/registry/`](#reduction-graph) | Compile-time reduction graph metadata |
| [`src/solvers/`](#solvers) | BruteForce and ILP solvers |
| `src/traits.rs` | Core `Problem` and `OptimizationProblem` traits (see [Problem Model](#problem-model)) |
| `src/types.rs` | Shared types: `SolutionSize`, `Direction`, `ProblemSize` (see [Problem Model](#problem-model)) |
| `src/variant.rs` | Variant parameter system (see [Variant System](#variant-system)) |

## Problem Model

Every problem implements `Problem`. Optimization problems additionally implement `OptimizationProblem`; satisfaction problems implement `SatisfactionProblem`.

- **`Problem`** — the base trait. Every problem declares a `NAME` (e.g., `"MaximumIndependentSet"`). The solver explores the configuration space defined by `dims()` and scores each configuration with `evaluate()`. For example, a 4-vertex MIS has `dims() = [2, 2, 2, 2]` (each vertex is selected or not); `evaluate(&[1, 0, 1, 0])` returns `Valid(2)` if vertices 0 and 2 form an independent set, or `Invalid` if they share an edge.
- **`OptimizationProblem`** — extends `Problem` with a comparable `Value` type and a `direction()` (`Maximize` or `Minimize`).
- **`SatisfactionProblem`** — constrains `Metric = bool`: `true` if all constraints are satisfied, `false` otherwise.

<div class="theme-light-only">

![Trait Hierarchy](static/trait-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Trait Hierarchy](static/trait-hierarchy-dark.svg)

</div>
```

**Step 2: Verify the mdbook builds**

Run: `cd /Users/liujinguo/rcode/problemreductions && mdbook build`
Expected: Build succeeds, no broken links.

**Step 3: Commit**

```bash
git add docs/src/design.md
git commit -m "docs: rewrite design.md sections 1-2 (module overview + problem model)"
```

---

### Task 2: Rewrite section 3 (Variant System)

**Files:**
- Modify: `docs/src/design.md` — replace the old "Problem variants" subsection (lines 46-93 in the original) with the new "Variant System" section.

**Step 1: Write the Variant System section**

This section replaces everything from `### Problem variants` through the end of the `variant_params!` code block. It should appear immediately after the trait hierarchy diagram.

```markdown
## Variant System

A single problem name like `MaximumIndependentSet` can have multiple **variants** — carrying weights on vertices, or defined on a grid. Some variants are more specific than others: the grid graph is a special case of the unit-disk graph, which is a special case of the simple graph.

In **set** language, variants form **subsets**: independent sets on grid graphs are a subset of independent sets on unit-disk graphs. The reduction from a more specific variant to a less specific one is a **natural reduction** (identity mapping). To avoid repeating the same rule for each variant pair, the library provides an auto-casting mechanism.

<div class="theme-light-only">

![Variant Hierarchy](static/variant-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Variant Hierarchy](static/variant-hierarchy-dark.svg)

</div>

Arrows indicate the **subset** (subtype) direction. Variant types fall into three categories:

- **Graph type** — e.g., `SimpleGraph`, `UnitDiskGraph`, `KingsSubgraph`. Available graph variants:
- **Weight type** — `One` (unweighted), `i32`, `f64`.
- **K value** — e.g., `K3` for 3-SAT, `KN` for arbitrary K.

<div class="theme-light-only">

![Lattices](static/lattices.svg)

</div>
<div class="theme-dark-only">

![Lattices](static/lattices-dark.svg)

</div>

### VariantParam trait

Each variant parameter type implements `VariantParam`, which declares its category, value, and optional parent:

```rust
pub trait VariantParam: 'static {
    const CATEGORY: &'static str;     // e.g., "graph", "weight", "k"
    const VALUE: &'static str;        // e.g., "SimpleGraph", "i32"
    const PARENT_VALUE: Option<&'static str>;  // None for root types
}
```

Types with a parent also implement `CastToParent`, providing the runtime conversion for natural casts:

```rust
pub trait CastToParent: VariantParam {
    type Parent: VariantParam;
    fn cast_to_parent(&self) -> Self::Parent;
}
```

### Registration with `impl_variant_param!`

The `impl_variant_param!` macro implements `VariantParam` (and optionally `CastToParent` / `KValue`) and registers a `VariantTypeEntry` via `inventory` for compile-time hierarchy discovery:

```rust
// Root type (no parent):
impl_variant_param!(SimpleGraph, "graph");

// Type with parent (cast closure required):
impl_variant_param!(UnitDiskGraph, "graph",
    parent: SimpleGraph,
    cast: |g| SimpleGraph::new(g.num_vertices(), g.edges()));

// K root (arbitrary K):
impl_variant_param!(KN, "k", k: None);

// Specific K with parent:
impl_variant_param!(K3, "k", parent: KN, cast: |_| KN, k: Some(3));
```

At startup, the `ReductionGraph` collects all `VariantTypeEntry` registrations and computes the **transitive closure** of the parent relationships, so `KingsSubgraph` is recognized as a subtype of `SimpleGraph` even though it declares `UnitDiskGraph` as its direct parent.

### Composing `Problem::variant()`

The `variant_params!` macro composes the `Problem::variant()` body from type parameter names:

```rust
// MaximumIndependentSet<G: VariantParam, W: VariantParam>
fn variant() -> Vec<(&'static str, &'static str)> {
    crate::variant_params![G, W]
    // e.g., MaximumIndependentSet<UnitDiskGraph, One>
    //     → vec![("graph", "UnitDiskGraph"), ("weight", "One")]
}
```
```

**Step 2: Verify mdbook builds**

Run: `cd /Users/liujinguo/rcode/problemreductions && mdbook build`

**Step 3: Commit**

```bash
git add docs/src/design.md
git commit -m "docs: rewrite design.md section 3 (variant system)"
```

---

### Task 3: Rewrite section 4 (Reduction Rules)

**Files:**
- Modify: `docs/src/design.md` — replace the old "Reduction Rules" section.

**Step 1: Write the Reduction Rules section**

This replaces the old section (lines 95-129 in the original). Place it after the Variant System section.

```markdown
## Reduction Rules

A reduction requires two pieces: a **result struct** and a **`ReduceTo<T>` impl**.

### Result struct

Holds the target problem and the logic to map solutions back:

```rust
#[derive(Clone)]
pub struct ReductionISToVC<W> {
    target: MinimumVertexCover<SimpleGraph, W>,
}

impl<W: WeightElement + VariantParam> ReductionResult for ReductionISToVC<W> {
    type Source = MaximumIndependentSet<SimpleGraph, W>;
    type Target = MinimumVertexCover<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target { &self.target }
    fn extract_solution(&self, target_sol: &[usize]) -> Vec<usize> {
        target_sol.iter().map(|&x| 1 - x).collect()  // complement
    }
}
```

### `ReduceTo<T>` impl with the `#[reduction]` macro

```rust
#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices)),
            ("num_edges", poly!(num_edges)),
        ])
    }
)]
impl ReduceTo<MinimumVertexCover<SimpleGraph, i32>>
    for MaximumIndependentSet<SimpleGraph, i32>
{
    type Result = ReductionISToVC<i32>;
    fn reduce_to(&self) -> Self::Result { /* ... */ }
}
```

### What the macro generates

The `#[reduction]` attribute expands to the original `impl` block plus an `inventory::submit!` call:

```rust
inventory::submit! {
    ReductionEntry {
        source_name: "MaximumIndependentSet",
        target_name: "MinimumVertexCover",
        source_variant_fn: || <MIS<SimpleGraph, i32> as Problem>::variant(),
        target_variant_fn: || <MVC<SimpleGraph, i32> as Problem>::variant(),
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices)),
            ("num_edges", poly!(num_edges)),
        ]),
        module_path: module_path!(),
    }
}
```

This `ReductionEntry` is collected at compile time by `inventory`, making the reduction discoverable by the `ReductionGraph` without any manual registration.

See [adding-reductions.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-reductions.md) for the full implementation guide.
```

**Step 2: Verify mdbook builds**

Run: `cd /Users/liujinguo/rcode/problemreductions && mdbook build`

**Step 3: Commit**

```bash
git add docs/src/design.md
git commit -m "docs: rewrite design.md section 4 (reduction rules)"
```

---

### Task 4: Rewrite section 5 (Reduction Graph)

**Files:**
- Modify: `docs/src/design.md` — replace the old "Reduction" H2 and "Reduction Graph" H3 (lines 132-149 in the original).

**Step 1: Write the Reduction Graph section**

Place after Reduction Rules. This replaces the old "Reduction" section header and its "Reduction Graph" subsection.

```markdown
## Reduction Graph

The `ReductionGraph` is the central runtime data structure. It collects all registered reductions and variant hierarchies to enable path finding and overhead evaluation.

### Construction

`ReductionGraph::new()` performs two `inventory` scans:

1. **`ReductionEntry` items** — each registered reduction becomes a directed edge in a `petgraph::DiGraph`. Nodes are type-erased base names (e.g., `"MaxCut"`, not `"MaxCut<SimpleGraph, i32>"`), so path finding works regardless of type parameters.

2. **`VariantTypeEntry` items** — parent declarations are collected per category and transitively closed, building a `variant_hierarchy: HashMap<category, HashMap<value, Set<supertypes>>>`.

### Natural edges

When exporting the graph (via `to_json()`), the graph auto-generates **natural edges** between same-name variant nodes. A natural edge from variant A to variant B exists when every field of A is at least as restrictive as B's (i.e., A is a subtype of B). Natural edges carry **identity overhead** — the problem size is unchanged.

For example, `MaximumIndependentSet{KingsSubgraph, i32}` gets a natural edge to `MaximumIndependentSet{SimpleGraph, i32}` because `KingsSubgraph` is a subtype of `SimpleGraph`.

### JSON export

`ReductionGraph::to_json()` produces a `ReductionGraphJson` with fully expanded variant nodes and both reduction + natural edges:

- [reduction_graph.json](reductions/reduction_graph.json) — all problem variants and reduction edges
- [problem_schemas.json](reductions/problem_schemas.json) — field definitions for each problem type
```

**Step 2: Verify mdbook builds**

Run: `cd /Users/liujinguo/rcode/problemreductions && mdbook build`

**Step 3: Commit**

```bash
git add docs/src/design.md
git commit -m "docs: rewrite design.md section 5 (reduction graph)"
```

---

### Task 5: Rewrite section 6 (Path Finding)

**Files:**
- Modify: `docs/src/design.md` — replace the old "Path Finding" H3 (lines 153-208 in the original).

**Step 1: Write the Path Finding section**

Place after Reduction Graph. Keep the existing resolve_path content and examples, add the Dijkstra/cost-function content.

```markdown
## Path Finding

Path finding operates at two levels: **name-level** paths (which problem types to traverse) and **variant-level** resolved paths (with concrete variant and overhead at each step).

### Name-level paths

`find_paths_by_name(src, dst)` enumerates all simple paths in the type-erased graph. `find_shortest_path_by_name()` returns the one with fewest hops.

For cost-aware routing, `find_cheapest_path()` uses **Dijkstra's algorithm** with set-theoretic validation:

```rust
pub fn find_cheapest_path<C: PathCostFn>(
    &self,
    source: (&str, &str),        // (problem_name, graph_type)
    target: (&str, &str),
    input_size: &ProblemSize,
    cost_fn: &C,
) -> Option<ReductionPath>
```

At each edge, Dijkstra checks `rule_applicable()` — the source graph must be a subtype of the rule's expected source, and the rule's target graph must be a subtype of the desired target. This ensures the chosen path respects variant constraints.

### Cost functions

The `PathCostFn` trait computes edge cost from overhead and current problem size:

```rust
pub trait PathCostFn {
    fn edge_cost(&self, overhead: &ReductionOverhead, current_size: &ProblemSize) -> f64;
}
```

Built-in implementations:

| Cost function | Strategy |
|--------------|----------|
| `Minimize("field")` | Minimize a single output field |
| `MinimizeWeighted([(field, w)])` | Weighted sum of output fields |
| `MinimizeMax([fields])` | Minimize the maximum of fields |
| `MinimizeLexicographic([fields])` | Lexicographic: minimize first, break ties with rest |
| `MinimizeSteps` | Minimize number of hops (unit edge cost) |
| `CustomCost(closure)` | User-defined cost function |

### Variant-level resolution: `resolve_path`

Given a name-level `ReductionPath`, `resolve_path` threads variant state through each step to produce a `ResolvedPath`:

```rust
pub fn resolve_path(
    &self,
    path: &ReductionPath,                       // name-level plan
    source_variant: &BTreeMap<String, String>,   // caller's concrete variant
    target_variant: &BTreeMap<String, String>,   // desired target variant
) -> Option<ResolvedPath>
```

The algorithm:

1. **Find candidates** — all `ReductionEntry` items matching `(src_name, dst_name)`.
2. **Filter compatible** — keep entries where the current variant is equal-or-more-specific than the entry's source variant on every axis.
3. **Pick most specific** — among compatible entries, choose the tightest fit.
4. **Insert natural cast** — if the current variant is more specific than the chosen entry's source, emit a `NaturalCast` edge.
5. **Advance** — update current variant to the entry's target variant, emit a `Reduction` edge with the correct overhead.

The result is a `ResolvedPath`:

```rust
pub struct ResolvedPath {
    pub steps: Vec<ReductionStep>,  // (name, variant) at each node
    pub edges: Vec<EdgeKind>,       // Reduction{overhead} | NaturalCast
}
```

#### Example: MIS on KingsSubgraph to MinimumVertexCover

Resolving `MIS(KingsSubgraph, i32) → VC(SimpleGraph, i32)` through name-path `["MIS", "VC"]`:

```
steps:  MIS{KingsSubgraph,i32}  →  MIS{SimpleGraph,i32}  →  VC{SimpleGraph,i32}
edges:       NaturalCast                  Reduction{overhead}
```

The resolver finds that the `MIS → VC` reduction expects `SimpleGraph`, so it inserts a `NaturalCast` to relax `KingsSubgraph` to `SimpleGraph` first.

#### Example: KSat Disambiguation

Resolving `KSat(k=3) → QUBO` through name-path `["KSatisfiability", "QUBO"]`:

- Candidates: `KSat<2> → QUBO` (overhead: `num_vars`) and `KSat<3> → QUBO` (overhead: `num_vars + num_clauses`).
- Filter with `k=3`: only `KSat<3>` is compatible (`3` is not a subtype of `2`).
- Result: the k=3-specific overhead is returned.
```

**Step 2: Verify mdbook builds**

Run: `cd /Users/liujinguo/rcode/problemreductions && mdbook build`

**Step 3: Commit**

```bash
git add docs/src/design.md
git commit -m "docs: rewrite design.md section 6 (path finding)"
```

---

### Task 6: Write sections 7-8 (Overhead Evaluation + Reduction Execution)

**Files:**
- Modify: `docs/src/design.md` — replace the empty "Overhead Evaluation" and "Reduction Execution" headers (lines 210-212 in the original).

**Step 1: Write the Overhead Evaluation and Reduction Execution sections**

Place after Path Finding.

```markdown
## Overhead Evaluation

Each reduction declares how the output problem size relates to the input size, expressed as polynomials.

### ProblemSize

A `ProblemSize` holds named size components — the dimensions that characterize a problem instance:

```rust
let size = ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 15)]);
assert_eq!(size.get("num_vertices"), Some(10));
```

### Polynomials

Output size formulas use `Polynomial` (a sum of `Monomial` terms). The `poly!` macro provides a concise syntax:

```rust
poly!(num_vertices)              // p(x) = num_vertices
poly!(num_vertices ^ 2)          // p(x) = num_vertices²
poly!(3 * num_edges)             // p(x) = 3 · num_edges
poly!(num_vertices * num_edges)  // p(x) = num_vertices · num_edges
```

A `ReductionOverhead` pairs output field names with their polynomials:

```rust
ReductionOverhead::new(vec![
    ("num_vars", poly!(num_vertices) + poly!(num_edges)),
    ("num_clauses", poly!(3 * num_edges)),
])
```

### Evaluating overhead

`ReductionOverhead::evaluate_output_size(input)` substitutes input values into the polynomials and returns a new `ProblemSize`:

```
Input:  ProblemSize { num_vertices: 10, num_edges: 15 }
Output: ProblemSize { num_vars: 25, num_clauses: 45 }
```

### Composing through a path

For a multi-step reduction path, overhead composes: the output of step $N$ becomes the input of step $N+1$. Each `ResolvedPath` edge carries its own `ReductionOverhead` (or `NaturalCast` with identity overhead), so the total output size is computed by chaining `evaluate_output_size` calls through the path.

## Reduction Execution

A `ResolvedPath` is a **plan**, not an executor. It provides variant and overhead information at each step, but callers dispatch the actual transformations themselves.

### Dispatching steps

Walk the `edges` array and dispatch based on `EdgeKind`:

- **`EdgeKind::Reduction`** — call `ReduceTo::reduce_to()` on the current problem to produce a `ReductionResult`, then call `target_problem()` to get the next problem.
- **`EdgeKind::NaturalCast`** — call `CastToParent::cast_to_parent()` (for graph casts) or the equivalent weight cast. The problem data is preserved; only the type changes.

### Extracting solutions

After solving the final target problem, walk the chain **in reverse**:

- At each `Reduction` edge, call `extract_solution(&target_solution)` on the corresponding `ReductionResult` to map the solution back to the source space.
- At each `NaturalCast` edge, the solution passes through unchanged (identity mapping).

### Why concrete types (no type erasure)

The library uses concrete types at each step rather than `dyn Problem`. This preserves full type safety and avoids boxing overhead, at the cost of requiring callers to know the types at each step. This design choice keeps the reduction pipeline zero-cost and makes the compiler verify correctness at each transformation boundary.
```

**Step 2: Verify mdbook builds**

Run: `cd /Users/liujinguo/rcode/problemreductions && mdbook build`

**Step 3: Commit**

```bash
git add docs/src/design.md
git commit -m "docs: write design.md sections 7-8 (overhead evaluation + execution)"
```

---

### Task 7: Rewrite sections 9-11 (Solvers + JSON + Contributing)

**Files:**
- Modify: `docs/src/design.md` — replace the old "Solvers", "JSON Serialization", and "Contributing" sections (lines 237-252 in the original).

**Step 1: Write the Solvers, JSON Serialization, and Contributing sections**

Place after Reduction Execution.

```markdown
## Solvers

Solvers implement the `Solver` trait:

```rust
pub trait Solver {
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>>;
    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>>;
}
```

### BruteForce

Enumerates every configuration in the space defined by `dims()`. Suitable for small instances (<20 variables). In addition to the `Solver` trait methods, provides:

- `find_all_best(problem)` — returns all tied-optimal configurations.
- `find_all_satisfying(problem)` — returns all satisfying configurations.

Primarily used for **testing and verification** of reductions via closed-loop tests.

### ILPSolver

Feature-gated behind `ilp`. Uses the HiGHS solver via the `good_lp` crate. Additionally provides `solve_reduced()` for problems that implement `ReduceTo<ILP>` — it reduces, solves the ILP, and extracts the solution in one call.

## JSON Serialization

All problem types support JSON serialization via serde:

```rust
use problemreductions::io::{to_json, from_json};

let json = to_json(&problem)?;
let restored: MaximumIndependentSet<SimpleGraph, i32> = from_json(&json)?;
```

**Exported JSON files:**
- [reduction_graph.json](reductions/reduction_graph.json) — all problem variants and reduction edges
- [problem_schemas.json](reductions/problem_schemas.json) — field definitions for each problem type

Regenerate exports:

```bash
cargo run --example export_graph                # docs/src/reductions/reduction_graph.json (default)
cargo run --example export_graph -- output.json # custom output path
cargo run --example export_schemas              # docs/src/reductions/problem_schemas.json
```

## Contributing

See [Call for Contributions](./introduction.md#call-for-contributions) for the recommended issue-based workflow (no coding required).
```

**Step 2: Verify mdbook builds**

Run: `cd /Users/liujinguo/rcode/problemreductions && mdbook build`

**Step 3: Commit**

```bash
git add docs/src/design.md
git commit -m "docs: rewrite design.md sections 9-11 (solvers + JSON + contributing)"
```

---

### Task 8: Update internal anchor links in module table

**Files:**
- Modify: `docs/src/design.md` — the Module Overview table links.

**Step 1: Verify all anchor links resolve correctly**

Check that the `#problem-model`, `#reduction-rules`, `#reduction-graph`, `#solvers`, `#variant-system` anchors match the actual section headers. mdBook generates anchors from headers by lowercasing and replacing spaces with hyphens.

Expected mappings:
- `## Problem Model` → `#problem-model`
- `## Variant System` → `#variant-system`
- `## Reduction Rules` → `#reduction-rules`
- `## Reduction Graph` → `#reduction-graph`
- `## Solvers` → `#solvers`

**Step 2: Fix any broken anchors**

If needed, update the table links in the Module Overview section.

**Step 3: Final mdbook build**

Run: `cd /Users/liujinguo/rcode/problemreductions && mdbook build`

**Step 4: Commit**

```bash
git add docs/src/design.md
git commit -m "docs: fix internal anchor links in design.md"
```
