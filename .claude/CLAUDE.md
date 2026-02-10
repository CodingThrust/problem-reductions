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
make export-graph   # Regenerate reduction graph JSON
make export-schemas # Regenerate problem schemas JSON
make qubo-testdata  # Regenerate QUBO ground truth JSON
make clean          # Clean build artifacts
```

## Verify Changes
```bash
make test clippy export-graph  # Must pass before PR
```

## Architecture

### Core Modules
- `src/models/` - Problem implementations (SAT, Graph, Set, Optimization)
- `src/rules/` - Reduction rules + inventory registration
- `src/solvers/` - BruteForce solver, ILP solver (feature-gated)
- `src/traits.rs` - `Problem`, `ConstraintSatisfactionProblem` traits
- `src/rules/traits.rs` - `ReduceTo<T>`, `ReductionResult` traits
- `src/registry/` - Compile-time reduction metadata collection
- `src/unit_tests/` - Unit test files (mirroring `src/` structure, referenced via `#[path]`)
- `tests/main.rs` - Integration tests (modules in `tests/suites/`)
- `tests/data/` - Ground truth JSON for integration tests
- `scripts/` - Python test data generation scripts (managed with `uv`)
- `docs/plans/` - Implementation plans

### Trait Hierarchy

```
Problem (core trait - all problems must implement)
│
├── const NAME: &'static str           // Problem name, e.g., "MaximumIndependentSet"
├── type GraphType: GraphMarker        // Graph topology marker
├── type Weight: NumericWeight         // Weight type (i32, f64, Unweighted)
├── type Size                          // Objective value type
│
├── fn num_variables(&self) -> usize
├── fn num_flavors(&self) -> usize     // Usually 2 for binary problems
├── fn problem_size(&self) -> ProblemSize
├── fn energy_mode(&self) -> EnergyMode
├── fn solution_size(&self, config) -> SolutionSize
└── ... (default methods: variables, flavors, is_valid_config)

ConstraintSatisfactionProblem : Problem (extension for CSPs)
│
├── fn constraints(&self) -> Vec<LocalConstraint>
├── fn objectives(&self) -> Vec<LocalSolutionSize>
├── fn weights(&self) -> Vec<Self::Size>
├── fn set_weights(&mut self, weights)
├── fn is_weighted(&self) -> bool
└── ... (default methods: is_satisfied, compute_objective)
```

### Key Patterns
- Problems parameterized by weight type `W` and graph type `G`
- `ReductionResult` provides `target_problem()` and `extract_solution()`
- Graph types: SimpleGraph, GridGraph, UnitDiskGraph, Hypergraph
- Weight types: `Unweighted` (marker), `i32`, `f64`

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
- Example files: `examples/reduction_<source>_to_<target>.rs`
- Test naming: `test_<source>_to_<target>_closed_loop`

### Paper (docs/paper/reductions.typ)
- `problem-def(name, title, body)` — defines a problem with auto-generated schema, reductions list, and label `<def:ProblemName>`
- `reduction-rule(source, target, ...)` — generates a theorem with label `<thm:Source-to-Target>` and registers in `covered-rules` state
- Completeness warnings auto-check that all JSON graph nodes/edges are covered in the paper
- `display-name` dict maps `ProblemName` to display text

## Contributing
See `.claude/rules/` for detailed guides:
- `adding-reductions.md` - How to add reduction rules
- `adding-models.md` - How to add problem types
- `testing.md` - Testing requirements and patterns
- `documentation.md` - Paper documentation patterns

Also see GitHub Issue #3 for coding rules.
