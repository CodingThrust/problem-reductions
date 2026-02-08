# CLAUDE.md

## Project Overview
Rust library for NP-hard problem reductions. Implements computational problems with reduction rules for transforming between equivalent formulations.

## Commands
```bash
make test          # Run all tests
make clippy        # Lint
make export-graph  # Regenerate reduction graph
make paper         # Build Typst paper
make coverage      # Generate coverage report (>95% required)
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

### Trait Hierarchy

```
Problem (core trait - all problems must implement)
│
├── const NAME: &'static str           // Problem name, e.g., "IndependentSet"
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

### Problem Variant IDs
Reduction graph nodes use variant IDs: `ProblemName[/GraphType][/Weighted]`
- Base: `IndependentSet` (SimpleGraph, unweighted)
- Graph variant: `IndependentSet/GridGraph`
- Weighted variant: `IndependentSet/Weighted`
- Both: `IndependentSet/GridGraph/Weighted`

## Conventions

### File Naming
- Reduction files: `src/rules/<source>_<target>.rs`
- Model files: `src/models/<category>/<name>.rs`
- Test naming: `test_<source>_to_<target>_closed_loop`

## Contributing
See `.claude/rules/` for detailed guides:
- `adding-reductions.md` - How to add reduction rules
- `adding-models.md` - How to add problem types
- `testing.md` - Testing requirements and patterns
- `documentation.md` - Paper documentation patterns

Also see GitHub Issue #3 for coding rules.
