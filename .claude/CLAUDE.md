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
make diagrams      # Generate SVG diagrams from Typst (light + dark)
make examples      # Generate example JSON for paper
make compare       # Generate and compare Rust mapping exports
make run-plan      # Execute a plan with Claude autorun
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
- `Solver::find_best()` → `Option<Vec<usize>>` for optimization problems; `Solver::find_satisfying()` → `Option<Vec<usize>>` for `Metric = bool`
- `BruteForce::find_all_best()` / `find_all_satisfying()` return `Vec<Vec<usize>>` for all optimal/satisfying solutions
- Graph types: SimpleGraph, GridGraph, UnitDiskGraph, Hypergraph
- Weight types: `Unweighted` (marker), `i32`, `f64`
- Weight management via inherent methods (`weights()`, `set_weights()`, `is_weighted()`), not traits
- `NumericSize` supertrait bundles common numeric bounds (`Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static`)

### Problem Names
Problem types use explicit optimization prefixes:
- `MaximumIndependentSet`, `MaximumClique`, `MaximumMatching`, `MaximumSetPacking`
- `MinimumVertexCover`, `MinimumDominatingSet`, `MinimumSetCovering`
- No prefix: `MaxCut`, `SpinGlass`, `QUBO`, `ILP`, `Satisfiability`, `KSatisfiability`, `CircuitSAT`, `Factoring`, `MaximalIS`, `PaintShop`, `BicliqueCover`, `BMF`

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

## Adding a Reduction Rule (A -> B)

**Reference implementations — read these first:**
- **Reduction rule:** `src/rules/minimumvertexcover_maximumindependentset.rs` — `ReductionResult` + `ReduceTo` + `#[reduction]` macro
- **Unit test:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs` — closed-loop + edge cases
- **Example program:** `examples/reduction_minimumvertexcover_to_maximumindependentset.rs` — create, reduce, solve, extract, verify, export
- **Paper entry:** `docs/paper/reductions.typ` (search for `MinimumVertexCover` `MaximumIndependentSet`)
- **Traits:** `src/rules/traits.rs` — `ReductionResult` and `ReduceTo` trait definitions

### 0. Before Writing Code

1. **Ensure you have enough information**
   - The reduction algorithm, from reliable source, e.g. a paper or a famous website.
   - Which example instance to use in `examples/`, example is expected for human reading.
   - The method to generate test data in `tests/data/<target>/` as json files.

   otherwise use `superpowers:brainstorming` to discuss with the user.
2. **Write plan** — save to `docs/plans/` using `superpowers:writing-plans`.

### 1. Implement

Create `src/rules/<source>_<target>.rs` following the reference. Key pieces:

- **`ReductionResult` struct + impl** — `target_problem()` + `extract_solution()` (see reference)
- **`ReduceTo` impl with `#[reduction(...)]` macro** — auto-generates `inventory::submit!`; only `overhead` attribute needed (graph/weight types are inferred, defaulting to `SimpleGraph`/`Unweighted`)
- **`#[cfg(test)] #[path = ...]`** linking to unit tests

Register in `src/rules/mod.rs`.

### 2. Test

- **Unit tests** in `src/unit_tests/rules/<source>_<target>.rs` — closed-loop + edge cases (see reference test).
- **Integration tests** in `tests/suites/reductions.rs` — compare against JSON ground truth.

### 3. Example Program

Add `examples/reduction_<source>_to_<target>.rs` — create, reduce, solve, extract, verify, export JSON (see reference example).

Examples must expose `pub fn run()` with `fn main() { run() }` so they can be tested directly via `include!` (no subprocess). Use regular comments (`//`) not inner doc comments (`//!`), and hardcode the example name instead of using `env!("CARGO_BIN_NAME")`.

Register the example in `tests/suites/examples.rs` by adding:
```rust
example_test!(reduction_<source>_to_<target>);
example_fn!(test_<source>_to_<target>, reduction_<source>_to_<target>);
```

### 4. Document

Update `docs/paper/reductions.typ` — add `reduction-rule("Source", "Target", ...)` with proof sketch (see Documentation Requirements section below).

### 5. Regenerate Graph

```bash
cargo run --example export_graph
```

## Adding a Model (Problem Type)

**Reference implementations — read these first:**
- **Optimization problem:** `src/models/graph/maximum_independent_set.rs` — `Problem` + `OptimizationProblem` with `Metric = SolutionSize<W>`
- **Satisfaction problem:** `src/models/satisfiability/sat.rs` — `Problem` with `Metric = bool`
- **Reference test:** `src/unit_tests/models/graph/maximum_independent_set.rs`

### Steps

1. **Create** `src/models/<category>/<name>.rs` — follow the reference for struct definition, `Problem` impl, and `OptimizationProblem` impl (if applicable).
2. **Register** in `src/models/<category>/mod.rs`.
3. **Add tests** in `src/unit_tests/models/<category>/<name>.rs` (linked via `#[path]`).
4. **Document** in `docs/paper/reductions.typ`: add `display-name` entry and `#problem-def("Name")[definition...]`.

### Trait Implementations

See Trait Hierarchy above for `Problem` and `OptimizationProblem` members. Weight management (`weights()`, `set_weights()`, `is_weighted()`) goes on inherent `impl` blocks, not traits. See the reference implementation for the pattern.

### Categories

- `src/models/satisfiability/` — Satisfiability, KSatisfiability
- `src/models/graph/` — MaximumIndependentSet, MinimumVertexCover, KColoring, etc.
- `src/models/set/` — MinimumSetCovering, MaximumSetPacking
- `src/models/optimization/` — SpinGlass, QUBO, ILP
- `src/models/specialized/` — CircuitSAT, Factoring, PaintShop, BicliqueCover, BMF

Naming convention: see Problem Names above.

## Testing Requirements

**Reference implementations — read these first:**
- **Reduction test:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs` — closed-loop pattern
- **Model test:** `src/unit_tests/models/graph/maximum_independent_set.rs` — evaluation, serialization
- **Solver test:** `src/unit_tests/solvers/brute_force.rs` — `find_best` + `find_satisfying`
- **Trait definitions:** `src/traits.rs` (`Problem`, `OptimizationProblem`), `src/solvers/mod.rs` (`Solver`)

### Coverage

New code must have >95% test coverage. Run `make coverage` to check.

### Naming

- Reduction tests: `test_<source>_to_<target>_closed_loop`
- Model tests: `test_<model>_basic`, `test_<model>_serialization`
- Solver tests: `test_<solver>_<problem>`

### Key Testing Patterns

See Key Patterns above for solver API signatures. Follow the reference files for exact usage.

### File Organization

Unit tests in `src/unit_tests/` linked via `#[path]` (see Core Modules above). Integration tests in `tests/suites/`, consolidated through `tests/main.rs`. Example tests in `tests/suites/examples.rs` (see Example Program in Adding a Reduction above).

## Documentation Requirements

**Reference:** search `docs/paper/reductions.typ` for `MinimumVertexCover` `MaximumIndependentSet` to see a complete problem-def + reduction-rule example.

### Adding a Problem Definition

```typst
#problem-def("ProblemName")[
  Mathematical definition...
]
```

Also add to the `display-name` dictionary:
```typst
"ProblemName": [Problem Name],
```

### Adding a Reduction Theorem

```typst
#reduction-rule("Source", "Target",
  example: true,
  example-caption: [caption text],
)[
  Rule statement...
][
  Proof sketch...
]
```

Every directed reduction in the graph needs its own `reduction-rule` entry. The paper auto-checks completeness against `reduction_graph.json`.
