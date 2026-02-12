# CLAUDE.md

## Project Overview
Rust library for NP-hard problem reductions. Implements computational problems with reduction rules for transforming between equivalent formulations.

## Commands
```bash
make help           # Show all available targets
make build          # Build the project
make test           # Run all tests
make fmt            # Format code with rustfmt
make fmt-check      # Check code formatting
make clippy         # Run clippy lints
make doc            # Build mdBook documentation (includes reduction graph export)
make mdbook         # Build and serve mdBook with live reload
make paper          # Build Typst paper (runs examples + exports first)
make coverage       # Generate coverage report (>95% required)
make check          # Quick pre-commit check (fmt + clippy + test)
make rust-export    # Generate Rust mapping JSON exports
make export-schemas # Regenerate problem schemas JSON
make qubo-testdata  # Regenerate QUBO ground truth JSON
make clean          # Clean build artifacts
make release V=x.y.z  # Tag and push a new release (CI publishes to crates.io)
```

## Verify Changes
```bash
make test clippy  # Must pass before PR
```

## Architecture

### Core Modules
- `src/models/` - Problem implementations (SAT, Graph, Set, Optimization)
- `src/rules/` - Reduction rules + inventory registration
- `src/solvers/` - BruteForce solver, ILP solver (feature-gated)
- `src/traits.rs` - `Problem`, `OptimizationProblem` traits
- `src/rules/traits.rs` - `ReduceTo<T>`, `ReductionResult` traits
- `src/registry/` - Compile-time reduction metadata collection
- `src/unit_tests/` - Unit test files (mirroring `src/` structure, referenced via `#[path]`)
- `tests/main.rs` - Integration tests (modules in `tests/suites/`); example tests use `include!` for direct invocation (no subprocess)
- `tests/data/` - Ground truth JSON for integration tests
- `scripts/` - Python test data generation scripts (managed with `uv`)
- `docs/plans/` - Implementation plans

### Trait Hierarchy

```
Problem (core trait — all problems must implement)
│
├── const NAME: &'static str           // e.g., "MaximumIndependentSet"
├── type Metric: Clone                 // SolutionSize<W> for optimization, bool for satisfaction
├── fn dims(&self) -> Vec<usize>       // config space: [2, 2, 2] for 3 binary variables
├── fn evaluate(&self, config) -> Metric
├── fn variant() -> Vec<(&str, &str)>  // [("graph","SimpleGraph"), ("weight","i32")]
└── fn num_variables(&self) -> usize   // default: dims().len()

OptimizationProblem : Problem<Metric = SolutionSize<Self::Value>> (extension for optimization)
│
├── type Value: PartialOrd + Clone     // inner objective type (i32, f64, etc.)
└── fn direction(&self) -> Direction   // Maximize or Minimize
```

**Satisfaction problems** (e.g., `Satisfiability`) use `Metric = bool` and do not implement `OptimizationProblem`.

**Optimization problems** (e.g., `MaximumIndependentSet`) use `Metric = SolutionSize<W>` where:
```rust
enum SolutionSize<T> { Valid(T), Invalid }  // Invalid = infeasible config
enum Direction { Maximize, Minimize }
```

### Key Patterns
- Problems parameterized by weight type `W` and graph type `G`
- `ReductionResult` provides `target_problem()` and `extract_solution()`
- `Solver::find_best()` for optimization problems, `Solver::find_satisfying()` for `Metric = bool`
- Graph types: SimpleGraph, GridGraph, UnitDiskGraph, Hypergraph
- Weight types: `Unweighted` (marker), `i32`, `f64`
- Weight management via inherent methods (`weights()`, `set_weights()`, `is_weighted()`), not traits
- `NumericSize` supertrait bundles common numeric bounds (`Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static`)

### Problem Names
Problem types use explicit optimization prefixes:
- `MaximumIndependentSet`, `MaximumClique`, `MaximumMatching`, `MaximumSetPacking`
- `MinimumVertexCover`, `MinimumDominatingSet`, `MinimumSetCovering`
- No prefix: `MaxCut`, `SpinGlass`, `QUBO`, `ILP`, `Satisfiability`, `KSatisfiability`, `CircuitSAT`, `Factoring`, `MaximalIS`

### Problem Variant IDs
Reduction graph nodes use variant IDs: `ProblemName[/GraphType][/Weighted]`
- Base: `MaximumIndependentSet` (SimpleGraph, unweighted)
- Graph variant: `MaximumIndependentSet/GridGraph`
- Weighted variant: `MaximumIndependentSet/Weighted`
- Both: `MaximumIndependentSet/GridGraph/Weighted`

## Conventions

### File Naming
- Reduction files: `src/rules/<source>_<target>.rs` (e.g., `maximumindependentset_qubo.rs`)
- Model files: `src/models/<category>/<name>.rs` (e.g., `maximum_independent_set.rs`)
- Example files: `examples/reduction_<source>_to_<target>.rs` (must have `pub fn run()` + `fn main() { run() }`)
- Test naming: `test_<source>_to_<target>_closed_loop`

### Paper (docs/paper/reductions.typ)
- `problem-def(name)[body]` — defines a problem with auto-generated schema, reductions list, and label `<def:ProblemName>`. Title comes from `display-name` dict.
- `reduction-rule(source, target, example: bool, ...)[rule][proof]` — generates a theorem with label `<thm:Source-to-Target>` and registers in `covered-rules` state. Overhead auto-derived from JSON edge data.
- Every directed reduction needs its own `reduction-rule` entry
- Completeness warnings auto-check that all JSON graph nodes/edges are covered in the paper
- `display-name` dict maps `ProblemName` to display text

## Contributing
See `.claude/rules/` for detailed guides:
- `adding-reductions.md` - How to add reduction rules
- `adding-models.md` - How to add problem types
- `testing.md` - Testing requirements and patterns
- `documentation.md` - Paper documentation patterns

Also see GitHub Issue #3 for coding rules.
