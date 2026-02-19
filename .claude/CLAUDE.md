# CLAUDE.md

## Project Overview
Rust library for NP-hard problem reductions. Implements computational problems with reduction rules for transforming between equivalent formulations.

## Skills
When resolving an issue with pull request, please use [issue-to-pr](skills/issue-to-pr.md) skill to convert the issue into a pull request with a plan.

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
make jl-testdata   # Regenerate Julia parity test data (requires julia)
make cli           # Build the pred CLI tool (release mode)
make cli-demo      # Run closed-loop CLI demo (exercises all commands)
make run-plan      # Execute a plan with Claude autorun
make release V=x.y.z  # Tag and push a new release (CI publishes to crates.io)
```

## Verify Changes
```bash
make test clippy  # Must pass before PR
```

## Git Safety
- **NEVER force push** (`git push --force`, `git push -f`, `git push --force-with-lease`). This is an absolute rule with no exceptions. Force push can silently destroy other people's work and stashed changes.

## Architecture

### Core Modules
- `src/models/` - Problem implementations (SAT, Graph, Set, Optimization)
- `src/rules/` - Reduction rules + inventory registration
- `src/solvers/` - BruteForce solver, ILP solver (feature-gated)
- `src/traits.rs` - `Problem`, `OptimizationProblem`, `SatisfactionProblem` traits
- `src/rules/traits.rs` - `ReduceTo<T>`, `ReductionResult` traits
- `src/registry/` - Compile-time reduction metadata collection
- `problemreductions-cli/` - `pred` CLI tool (separate crate in workspace)
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
├── fn variant() -> Vec<(&str, &str)>  // e.g., [("graph","SimpleGraph"), ("weight","i32")]
├── fn num_variables(&self) -> usize   // default: dims().len()
├── fn problem_size_names() -> &[&str] // static field names for size metrics
└── fn problem_size_values(&self) -> Vec<usize>  // instance-level size values

OptimizationProblem : Problem<Metric = SolutionSize<Self::Value>> (extension for optimization)
│
├── type Value: PartialOrd + Clone     // inner objective type (i32, f64, etc.)
└── fn direction(&self) -> Direction   // Maximize or Minimize

SatisfactionProblem : Problem<Metric = bool> (marker trait for decision problems)
```

**Satisfaction problems** (e.g., `Satisfiability`) use `Metric = bool` and implement `SatisfactionProblem`.

**Optimization problems** (e.g., `MaximumIndependentSet`) use `Metric = SolutionSize<W>` where:
```rust
enum SolutionSize<T> { Valid(T), Invalid }  // Invalid = infeasible config
enum Direction { Maximize, Minimize }
```

### Key Patterns
- Problems parameterized by graph type `G` and optionally weight type `W` (problem-dependent)
- `ReductionResult` provides `target_problem()` and `extract_solution()`
- `Solver::find_best()` → `Option<Vec<usize>>` for optimization problems; `Solver::find_satisfying()` → `Option<Vec<usize>>` for `Metric = bool`
- `BruteForce::find_all_best()` / `find_all_satisfying()` return `Vec<Vec<usize>>` for all optimal/satisfying solutions
- Graph types: HyperGraph, SimpleGraph, PlanarGraph, BipartiteGraph, UnitDiskGraph, KingsSubgraph, TriangularSubgraph
- Weight types: `One` (unit weight marker), `i32`, `f64` — all implement `WeightElement` trait
- `WeightElement` trait: `type Sum: NumericSize` + `fn to_sum(&self)` — converts weight to a summable numeric type
- Weight management via inherent methods (`weights()`, `set_weights()`, `is_weighted()`), not traits
- `NumericSize` supertrait bundles common numeric bounds (`Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static`)

### Problem Names
Problem types use explicit optimization prefixes:
- `MaximumIndependentSet`, `MaximumClique`, `MaximumMatching`, `MaximumSetPacking`
- `MinimumVertexCover`, `MinimumDominatingSet`, `MinimumSetCovering`
- No prefix: `MaxCut`, `SpinGlass`, `QUBO`, `ILP`, `Satisfiability`, `KSatisfiability`, `CircuitSAT`, `Factoring`, `MaximalIS`, `PaintShop`, `BicliqueCover`, `BMF`, `KColoring`, `TravelingSalesman`

### Problem Variant IDs
Reduction graph nodes use variant key-value pairs from `Problem::variant()`:
- Base: `MaximumIndependentSet` (empty variant = defaults)
- Graph variant: `MaximumIndependentSet {graph: "GridGraph", weight: "i32"}`
- Weight variant: `MaximumIndependentSet {graph: "SimpleGraph", weight: "f64"}`
- Nodes come exclusively from `#[reduction]` registrations; natural edges between same-name variants are inferred from the graph/weight subtype partial order

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

Unit tests in `src/unit_tests/` linked via `#[path]` (see Core Modules above). Integration tests in `tests/suites/`, consolidated through `tests/main.rs`. Example tests in `tests/suites/examples.rs` using `include!` for direct invocation.

## Documentation Locations
- `README.md` — Project overview and quickstart
- `.claude/` — Claude Code instructions and skills
- `docs/book/` — mdBook user documentation (built with `make doc`)
- `docs/paper/reductions.typ` — Typst paper with problem definitions and reduction theorems
- `examples/` — Reduction example code (also used in paper and tests)

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
