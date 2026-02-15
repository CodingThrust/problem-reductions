# Design

This guide covers the library internals for contributors and developers. See [Getting Started](./getting-started.md) for usage examples.

## Module Overview

<div class="theme-light-only">

![Module Overview](static/module-overview.svg)

</div>
<div class="theme-dark-only">

![Module Overview](static/module-overview-dark.svg)

</div>

| Module | Purpose |
|--------|---------|
| [`src/models/`](#models) | Problem type implementations (SAT, Graph, Set, Optimization) |
| [`src/rules/`](#rules) | Reduction rules with `ReduceTo` implementations |
| [`src/registry/`](#registry) | Compile-time reduction graph metadata |
| [`src/solvers/`](#solvers) | BruteForce and ILP solvers |
| `src/traits.rs` | Core `Problem` and `OptimizationProblem` traits (see [Models](#models)) |
| `src/types.rs` | Shared types: `SolutionSize`, `Direction`, `ProblemSize` (see [Models](#models)) |

## Models

Every problem implements `Problem`. Optimization problems additionally implement `OptimizationProblem`.

<div class="theme-light-only">

![Trait Hierarchy](static/trait-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Trait Hierarchy](static/trait-hierarchy-dark.svg)

</div>

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

Problems are parameterized by graph type and weight type:

- `MaximumIndependentSet<G, W>` — graph type `G`, weight type `W`
- `Satisfiability` — CNF formula (concrete type, no parameters)
- `QUBO<W>` — parameterized by weight type only

**Graph types:**

| Type | Description |
|------|-------------|
| `SimpleGraph` | Standard adjacency-based graph |
| `UnitDiskGraph` | Edges connect vertices within a distance threshold |
| `KingsSubgraph` | King's subgraph on a square grid (subtype of UnitDiskGraph) |
| `TriangularSubgraph` | Triangular lattice subgraph (subtype of UnitDiskGraph) |
| `HyperGraph` | Edges connecting any number of vertices |

All problem types support JSON serialization via serde:

```rust
use problemreductions::io::{to_json, from_json};

let json = to_json(&problem)?;
let restored: MaximumIndependentSet<SimpleGraph, i32> = from_json(&json)?;
```

See [adding-models.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-models.md) for the full implementation guide.

## Variant System

A single problem name like `MaximumIndependentSet` can have multiple **variants** — concrete instantiations that differ in graph topology, weight type, or other parameters. The variant system tracks these distinctions in the reduction graph so that reductions between specific instantiations are represented precisely.

<div class="theme-light-only">

![Variant Hierarchy](static/variant-hierarchy.svg)

</div>
<div class="theme-dark-only">

![Variant Hierarchy](static/variant-hierarchy-dark.svg)

</div>

Each variant is identified by a set of key-value pairs returned by `Problem::variant()`:

```rust
// MaximumIndependentSet<UnitDiskGraph, One>
fn variant() -> Vec<(&'static str, &'static str)> {
    vec![("graph", "UnitDiskGraph"), ("weight", "One")]
}

// KSatisfiability<3>
fn variant() -> Vec<(&'static str, &'static str)> {
    vec![("k", "3")]
}
```

Variant nodes in the reduction graph are discovered automatically from `#[reduction]` registrations — each reduction's source and target types become nodes. Natural edges between same-name variants are inferred from the graph/weight subtype partial order (e.g., `MIS/KingsSubgraph → MIS/SimpleGraph`). In the visualization, nodes are labeled with only the non-default fields for brevity (e.g. `MaximumIndependentSet (KingsSubgraph)` omits the default `One`).

### Graph Hierarchy

Graph types form a subtype hierarchy declared in `src/graph_types.rs`:

```
HyperGraph          (most general)
└── SimpleGraph
    ├── PlanarGraph
    ├── BipartiteGraph
    └── UnitDiskGraph
        ├── KingsSubgraph
        └── TriangularSubgraph
```

A problem on a more specific graph type can always be treated as a problem on a more general one — a `KingsSubgraph` *is* a `SimpleGraph`. This subtype relationship is registered at compile time:

```rust
declare_graph_subtype!(KingsSubgraph => UnitDiskGraph);
declare_graph_subtype!(UnitDiskGraph => SimpleGraph);
// ...
```

The runtime builds a transitive closure: `KingsSubgraph` is a subtype of `UnitDiskGraph`, `SimpleGraph`, and `HyperGraph`.

**Example: natural edge for TriangularSubgraph MIS.** Suppose we have a `MaximumIndependentSet<TriangularSubgraph, i32>` instance — an independent set problem on a triangular lattice. Because `TriangularSubgraph` is a subtype of `SimpleGraph` in the graph hierarchy, the reduction graph contains a natural edge:

```
MIS<TriangularSubgraph, i32>  →  MIS<SimpleGraph, i32>
```

This edge has identity overhead (the problem size is unchanged) and requires no code — the triangular lattice graph *is* a simple graph, so any MIS algorithm for general graphs applies directly. Combined with the explicit reduction `MIS<SimpleGraph, i32> → MIS<KingsSubgraph, i32>` (unit disk mapping), the system can automatically chain:

```
MIS<TriangularSubgraph, i32>  →  MIS<SimpleGraph, i32>  →  MIS<KingsSubgraph, i32>
     (natural edge)                  (explicit reduction)
```

### Weight Hierarchy

Weight types form a linear promotion chain:

```
One → i32 → f64
```

An unweighted problem (using `One`, the unit-weight type) is a special case of a weighted one (all weights equal to 1), and an integer-weighted problem embeds naturally into real-weighted. This is declared in `src/graph_types.rs`:

```rust
declare_weight_subtype!("One" => "i32");
declare_weight_subtype!("i32" => "f64");
```

### K Parameter

`KSatisfiability<K>` and `KColoring<K, G>` use type-level K values:

| Rust type | Variant `k` |
|-----------|-------------|
| `KSatisfiability<K2>` | `"K2"` |
| `KSatisfiability<K3>` | `"K3"` |
| Generic `KSatisfiability<KN>` | `"KN"` |

K values form a **flat hierarchy**: each specific K value (K1, K2, K3, K4, K5) is a direct child of the generic KN, with no chain between them. This reflects the fact that k-SAT and k-coloring problems with different k are independent problem classes — a 2-SAT instance is not a 3-SAT instance, and vice versa.

### Natural Edges

When two variants of the same problem differ only in that one is "more specific" than the other, a **natural edge** is auto-generated in the reduction graph. The edge represents the trivial identity reduction — the problem instance doesn't change, only its type annotation relaxes.

A variant A is reducible to variant B when every field of A is at least as specific as the corresponding field of B:

- **graph:** `is_graph_subtype(A.graph, B.graph)` — e.g. `UnitDiskGraph` ≤ `SimpleGraph`
- **weight:** `is_weight_subtype(A.weight, B.weight)` — e.g. `Unweighted` ≤ `i32`
- **k:** a concrete value is a subtype of `"N"`

Natural edges have identity overhead: the output size equals the input size.

### Example: Unweighted MIS on UnitDiskGraph → Weighted MIS on SimpleGraph

Consider reducing `MaximumIndependentSet<UnitDiskGraph, Unweighted>` to `MaximumIndependentSet<SimpleGraph, i32>`. These are two variants of the same problem, so the reduction graph connects them via natural edges:

```
MIS (UnitDiskGraph, Unweighted)
  │
  │  graph relaxation: UnitDiskGraph ≤ SimpleGraph
  ▼
MIS (SimpleGraph, Unweighted)
  │
  │  weight promotion: Unweighted ≤ i32
  ▼
MIS (SimpleGraph, i32)
```

**Step 1 — Graph relaxation.** A unit disk graph is a simple graph (it just happens to have geometric structure). The MIS instance is unchanged; we simply forget the geometric embedding and treat it as a generic graph.

**Step 2 — Weight promotion.** An unweighted MIS asks for the largest independent set (all vertices have equal value). This is equivalent to a weighted MIS where every vertex has weight 1. The instance gains uniform weights and becomes `MaximumIndependentSet<SimpleGraph, i32>`.

Both steps are identity reductions with zero overhead — no new variables or constraints are introduced. The variant system generates these edges automatically from the declared hierarchies.

### Variant-Aware Path Resolution

The `ReductionGraph` performs path-finding at the **name level** — nodes are `"MaximumIndependentSet"`, not `"MaximumIndependentSet<GridGraph, i32>"`. This keeps path discovery fast (one node per problem name), but it means a `ReductionPath` like `["KSatisfiability", "QUBO"]` carries no variant information. Two issues follow:

1. **Overhead ambiguity.** `KSatisfiability<2> → QUBO` and `KSatisfiability<3> → QUBO` have different overheads (k=3 introduces auxiliary variables via Rosenberg quadratization), but a name-level path can't distinguish them.

2. **Natural edge execution.** The path `MIS(KingsSubgraph) → VC(SimpleGraph)` needs an implicit graph-relaxation step, but the name-level path only says `["MaximumIndependentSet", "MinimumVertexCover"]`.

The solution is **two-phase resolution**: name-level discovery followed by variant-level resolution.

#### `resolve_path`

```rust
pub fn resolve_path(
    &self,
    path: &ReductionPath,                       // name-level plan
    source_variant: &BTreeMap<String, String>,   // caller's concrete variant
    target_variant: &BTreeMap<String, String>,   // desired target variant
) -> Option<ResolvedPath>
```

The resolver walks the name-level path, threading variant state through each step:

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

#### Execution Model

`ResolvedPath` is a **plan**, not an executor. Callers dispatch each step themselves:

- `EdgeKind::Reduction` → call `ReduceTo::reduce_to()`
- `EdgeKind::NaturalCast` → call `GraphCast::cast_graph()` or equivalent weight cast

This avoids type-erasure complexity while giving callers precise variant and overhead information at each step.

## Rules

A reduction requires two pieces:

**1. Result struct** — holds the target problem and extraction logic:

```rust
#[derive(Clone)]
pub struct ReductionAToB {
    target: B,
    // ... mapping data for extraction
}

impl ReductionResult for ReductionAToB {
    type Source = A;
    type Target = B;

    fn target_problem(&self) -> &B { &self.target }
    fn extract_solution(&self, target_sol: &[usize]) -> Vec<usize> { /* ... */ }
}
```

**2. `ReduceTo<T>` impl** with the `#[reduction]` macro:

```rust
#[reduction(A -> B)]
impl ReduceTo<B> for A {
    type Result = ReductionAToB;
    fn reduce_to(&self) -> Self::Result { /* ... */ }
}
```

The macro generates `inventory::submit!` calls for compile-time reduction graph registration.

See [adding-reductions.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-reductions.md) for the full implementation guide.

## Registry

The reduction graph is built at compile time using the `inventory` crate:

```rust
#[reduction(A -> B)]
impl ReduceTo<B> for A { /* ... */ }

// Expands to include:
// inventory::submit! { ReductionMeta { source: "A", target: "B", ... } }
```

**JSON exports:**
- [reduction_graph.json](reductions/reduction_graph.json) — all problem variants and reduction edges
- [problem_schemas.json](reductions/problem_schemas.json) — field definitions for each problem type

Regenerate exports:

```bash
cargo run --example export_graph                # docs/src/reductions/reduction_graph.json (default)
cargo run --example export_graph -- output.json # custom output path
cargo run --example export_schemas  # docs/src/reductions/problem_schemas.json
```

## Solvers

Solvers implement the `Solver` trait:

```rust
pub trait Solver {
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>>;
    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>>;
}
```

`ILPSolver` additionally provides `solve_reduced()` for problems implementing `ReduceTo<ILP>`.

## Contributing

See [Call for Contributions](./introduction.md#call-for-contributions) for the recommended issue-based workflow (no coding required).

For manual implementation:

- **Adding a problem:** See [adding-models.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-models.md)
- **Adding a reduction:** See [adding-reductions.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-reductions.md)
- **Testing requirements:** See [testing.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/testing.md)

Run `make test clippy` before submitting PRs.
