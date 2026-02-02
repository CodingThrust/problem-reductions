# CLAUDE.md

## Project Overview
Rust library for NP-hard problem reductions. Implements computational problems with reduction rules for transforming between equivalent formulations.

## Build & Test
```bash
make test        # Run all tests
make clippy      # Lint
make export-graph  # Regenerate reduction graph
make paper         # Build Typst paper
make mdbook        # Build and serve mdBook documentation
make coverage      # Generate coverage report
```

## Architecture

### Core Modules
- `src/models/` - Problem implementations (SAT, Graph, Set, Optimization categories)
- `src/rules/` - Reduction rules + inventory registration
- `src/solvers/` - BruteForce solver, ILP solver (feature-gated)
- `src/traits/` - `Problem`, `ConstraintSatisfactionProblem`, `ReduceTo<T>` traits
- `src/registry/` - Compile-time reduction metadata collection

### Reduction System
```rust
// Implement ReduceTo<TargetProblem> for SourceProblem
impl ReduceTo<TargetProblem> for SourceProblem {
    type Result = ReductionSourceToTarget;
    fn reduce_to(&self) -> Self::Result { ... }
}

// Register with inventory for automatic discovery
inventory::submit! { ReductionEntry { source_name, target_name, ... } }
```

### Key Patterns
- Problems parameterized by weight type `W` (i32, f64)
- `ReductionResult` provides `target_problem()` and `extract_solution()`
- Graph types: SimpleGraph, GridGraph, UnitDiskGraph, Hypergraph

## Adding Reductions
See GitHub Issue #3 for detailed coding rules:
1. Create `src/rules/<source>_<target>.rs` with inventory registration
2. Add closed-loop test (create instance -> reduce -> solve -> extract -> verify)
3. Document in `docs/paper/reductions.typ`
4. Regenerate graph with `cargo run --example export_graph --all-features`

## Coverage Requirement
New code must have >95% test coverage:
```bash
cargo tarpaulin --all-features --skip-clean --ignore-tests -- <module_name>
```
