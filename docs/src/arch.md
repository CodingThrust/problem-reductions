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

Every computational problem implements the `Problem` trait. A problem defines:

- **Variables** — the unknowns to be solved (e.g., vertex assignments, boolean values)
- **Flavors** — possible values each variable can take (usually 2 for binary problems)
- **Solution size** — the objective value for a given configuration

Each problem type has its own parameters. For example:

- `MaximumIndependentSet<G, W>` — parameterized by graph type `G` and weight type `W`
- `Satisfiability<W>` — CNF formula with optional clause weights
- `QUBO<W>` — parameterized by weight type only

Graph-based problems support multiple topologies:

| Graph Type | Description |
|------------|-------------|
| `SimpleGraph` | Standard adjacency-based graph |
| `GridGraph` | Vertices on a regular grid |
| `UnitDiskGraph` | Edges connect vertices within a distance threshold |
| `HyperGraph` | Edges connecting any number of vertices |

Problem variants appear as separate nodes in the reduction graph when they have distinct reductions:

```
MaximumIndependentSet           # base variant
MaximumIndependentSet/GridGraph # different graph topology
MaximumIndependentSet/Weighted  # weighted objective
```

Evaluating a configuration returns both validity and objective value:

```rust
let config = vec![1, 0, 1, 0];  // Variable assignments
let result = problem.evaluate(&config);
// result.is_valid() -> bool
// result.size() -> Option<&T>
```

### Implementation

Implement the `Problem` trait. Key methods:

| Method | Purpose |
|--------|---------|
| `NAME` | Problem identifier (e.g., `"MaximumIndependentSet"`) |
| `Metric` | Result type of evaluation (`SolutionSize<W>` or `bool`) |
| `dims()` | Configuration space dimensions |
| `evaluate()` | Evaluate a configuration |
| `variant()` | Key-value pairs identifying this variant |

See [Adding Models](claude.md) for the full guide.

## Rules

A **reduction** transforms one problem into another while preserving solutions. Given a source problem A and target problem B:

1. **Reduce** — convert A to B
2. **Solve** — find solution to B
3. **Extract** — map B's solution back to A

```rust
// Reduce: MaximumIndependentSet → QUBO
let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&problem);
let qubo = reduction.target_problem();

// Solve the target
let qubo_solution = solver.find_best(qubo);

// Extract back to source
let original_solution = reduction.extract_solution(&qubo_solution[0]);
```

Reductions track size overhead for complexity analysis:

```rust
let source_size = reduction.source_size();  // ProblemSize
let target_size = reduction.target_size();  // ProblemSize
```

The reduction graph shows all available transformations:

```
Satisfiability ──→ MaximumIndependentSet ──→ QUBO
                          │
                          ▼
                   MinimumVertexCover
```

Not all reductions preserve optimality — some only preserve satisfiability. The graph encodes this metadata.

### Implementation

A reduction requires two pieces:

1. **Result struct** — holds the target problem and extraction logic
2. **`ReduceTo<T>` impl** — performs the reduction

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

Use the `#[reduction]` macro to register in the global inventory:

```rust
#[reduction(A -> B)]
impl ReduceTo<B> for A {
    type Result = ReductionAToB;

    fn reduce_to(&self) -> Self::Result { /* ... */ }
}
```

The macro generates `inventory::submit!` calls, making the reduction discoverable at compile time for the reduction graph.

See [Adding Reductions](claude.md) for the full guide.

## Registry

The **reduction graph** is a directed graph where:

- **Nodes** — problem variants (e.g., `MaximumIndependentSet/GridGraph`)
- **Edges** — available reductions between variants

Variant IDs follow the pattern `ProblemName[/GraphType][/Weighted]`:

| Variant ID | Meaning |
|------------|---------|
| `MaximumIndependentSet` | Base variant (SimpleGraph, unweighted) |
| `MaximumIndependentSet/GridGraph` | GridGraph topology |
| `MaximumIndependentSet/Weighted` | Weighted objective |
| `MaximumIndependentSet/GridGraph/Weighted` | Both |

The graph data is stored in [`reduction_graph.json`](reductions/reduction_graph.json), it stores the reduction graph topology (nodes and edges):

```json
{
  "nodes": [
    {
      "name": "Satisfiability",
      "variant": {},
      "category": "satisfiability",
      "doc_path": "..."
    },
    {
      "name": "MaximumIndependentSet",
      "variant": {"graph": "GridGraph"},
      "category": "graph",
      "doc_path": "..."
    }
  ],
  "edges": [
    {
      "source": {"name": "Satisfiability", "variant": {}},
      "target": {"name": "MaximumIndependentSet", "variant": {}}
    }
  ]
}
```

</details>

Problem schemas (`problem_schemas.json`) describe each problem's structure:

<details>
<summary><code>problem_schemas.json</code> schema</summary>

```json
[
  {
    "name": "Satisfiability",
    "category": "satisfiability",
    "description": "Find satisfying assignment for CNF formula",
    "fields": [
      {
        "name": "num_vars",
        "type_name": "usize",
        "description": "Number of Boolean variables"
      },
      {
        "name": "clauses",
        "type_name": "Vec<CNFClause>",
        "description": "Clauses in conjunctive normal form"
      },
      {
        "name": "weights",
        "type_name": "Vec<W>",
        "description": "Clause weights for MAX-SAT"
      }
    ]
  }
]
```

</details>

Use the interactive diagram in the [mdBook documentation](https://codingthrust.github.io/problem-reductions/) to explore available reductions.

### Implementation

Reductions are collected at compile time using the `inventory` crate. The `#[reduction]` macro registers metadata:

```rust
#[reduction(A -> B)]
impl ReduceTo<B> for A { /* ... */ }

// Expands to include:
// inventory::submit! { ReductionMeta { source: "A", target: "B", ... } }
```

To regenerate the exports after adding rules or problems:

```bash
cargo run --example export_graph    # writes docs/src/reductions/reduction_graph.json
cargo run --example export_schemas  # writes docs/src/reductions/problem_schemas.json
```

## Solvers

Solvers find optimal solutions to problems. The library provides:

| Solver | Description | Use case |
|--------|-------------|----------|
| `BruteForce` | Enumerates all configurations | Small instances (< 20 variables) |
| `ILPSolver` | Integer Linear Programming (HiGHS) | Larger instances, requires `ilp` feature |

All solvers implement the `Solver` trait:

```rust
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);           // Best solution(s)
let with_size = solver.find_best_with_size(&problem); // With objective values
```

Solvers work with reductions — solve the target problem, then extract:

```rust
let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&problem);
let qubo_solutions = solver.find_best(reduction.target_problem());
let original = reduction.extract_solution(&qubo_solutions[0]);
```

### Implementation

The `Solver` trait:

```rust
pub trait Solver {
    fn find_best<P: Problem>(&self, problem: &P) -> Vec<Vec<usize>>;
    fn find_best_with_size<P: Problem>(&self, problem: &P)
        -> Vec<(Vec<usize>, SolutionSize<P::Size>)>;
}
```

`ILPSolver` additionally provides `solve_reduced()` for problems implementing `ReduceTo<ILP>`.

Enable with:

```toml
[dependencies]
problemreductions = { version = "0.1", features = ["ilp"] }
```

## File I/O

All problem types support JSON serialization for persistence and interoperability.

```rust
use problemreductions::io::{write_problem, read_problem, FileFormat};

// Write
write_problem(&problem, "problem.json", FileFormat::Json)?;

// Read
let problem: MaximumIndependentSet<i32> = read_problem("problem.json", FileFormat::Json)?;
```

String serialization:

```rust
use problemreductions::io::{to_json, from_json};

let json = to_json(&problem)?;
let restored: MaximumIndependentSet<i32> = from_json(&json)?;
```

| Format | Description |
|--------|-------------|
| `Json` | Pretty-printed |
| `JsonCompact` | No whitespace |

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
