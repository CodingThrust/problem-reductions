# Architecture

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

## Implementing Problems

Problems are parameterized by graph type and weight type:

- `MaximumIndependentSet<G, W>` — graph type `G`, weight type `W`
- `Satisfiability<W>` — CNF formula with optional clause weights
- `QUBO<W>` — parameterized by weight type only

**Graph types:**

| Type | Description |
|------|-------------|
| `SimpleGraph` | Standard adjacency-based graph |
| `GridGraph` | Vertices on a regular grid |
| `UnitDiskGraph` | Edges connect vertices within a distance threshold |
| `HyperGraph` | Edges connecting any number of vertices |

**Variant IDs** in the reduction graph follow `ProblemName[/GraphType][/Weighted]`:

```
MaximumIndependentSet           # base variant (SimpleGraph, unweighted)
MaximumIndependentSet/GridGraph # different graph topology
MaximumIndependentSet/Weighted  # weighted objective
```

See [adding-models.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-models.md) for the full implementation guide.

## Implementing Reductions

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
    fn source_size(&self) -> ProblemSize { /* ... */ }
    fn target_size(&self) -> ProblemSize { /* ... */ }
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

## Registry Internals

The reduction graph is built at compile time using the `inventory` crate:

```rust
#[reduction(A -> B)]
impl ReduceTo<B> for A { /* ... */ }

// Expands to include:
// inventory::submit! { ReductionMeta { source: "A", target: "B", ... } }
```

**JSON exports** (see [Getting Started](./getting-started.md#json-resources) for locations):

<details>
<summary><code>reduction_graph.json</code> schema</summary>

```json
{
  "nodes": [
    { "name": "Satisfiability", "variant": {}, "category": "satisfiability", "doc_path": "..." }
  ],
  "edges": [
    { "source": {"name": "A", "variant": {}}, "target": {"name": "B", "variant": {}} }
  ]
}
```

</details>

<details>
<summary><code>problem_schemas.json</code> schema</summary>

```json
[
  {
    "name": "Satisfiability",
    "category": "satisfiability",
    "description": "Find satisfying assignment for CNF formula",
    "fields": [
      { "name": "num_vars", "type_name": "usize", "description": "Number of Boolean variables" }
    ]
  }
]
```

</details>

Regenerate exports:

```bash
cargo run --example export_graph    # docs/src/reductions/reduction_graph.json
cargo run --example export_schemas  # docs/src/reductions/problem_schemas.json
```

## Implementing Solvers

Solvers implement the `Solver` trait:

```rust
pub trait Solver {
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Vec<Vec<usize>>;
    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>>;
}
```

`ILPSolver` additionally provides `solve_reduced()` for problems implementing `ReduceTo<ILP>`.

## File I/O

All problem types support JSON serialization via serde:

```rust
use problemreductions::io::{to_json, from_json};

let json = to_json(&problem)?;
let restored: MaximumIndependentSet<i32> = from_json(&json)?;
```

## Contributing

### Recommended: Issue-Based Workflow

The easiest way to contribute is through GitHub issues:

1. **Open an issue** using the [Problem](https://github.com/CodingThrust/problem-reductions/issues/new?template=problem.md) or [Rule](https://github.com/CodingThrust/problem-reductions/issues/new?template=rule.md) template
2. **Fill in all sections** — definition, algorithm, size overhead, example instance
3. **Review AI generated code** — AI generates code and you can review and comment on the pull request.
4. **Merge the pull request** — Once you are happy with the code, just ask maintainers' assistance to merge the pull request.

### Manual Implementation

When automation isn't suitable:

- **Adding a problem:** See [adding-models.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-models.md)
- **Adding a reduction:** See [adding-reductions.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-reductions.md)
- **Testing requirements:** See [testing.md](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/testing.md)

Run `make test clippy` before submitting PRs.
