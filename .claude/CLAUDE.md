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
- `src/traits/` - `Problem`, `ConstraintSatisfactionProblem`, `ReduceTo<T>` traits
- `src/registry/` - Compile-time reduction metadata collection

### Key Patterns
- Problems parameterized by weight type `W` and graph type `G`
- `ReductionResult` provides `target_problem()` and `extract_solution()`
- Graph types: SimpleGraph, GridGraph, UnitDiskGraph, Hypergraph

## Conventions

### File Naming
- Reduction files: `src/rules/<source>_<target>.rs`
- Model files: `src/models/<category>/<name>.rs`
- Test naming: `test_<source>_to_<target>_closed_loop`

### Reduction Pattern
```rust
impl ReduceTo<TargetProblem> for SourceProblem {
    type Result = ReductionSourceToTarget;
    fn reduce_to(&self) -> Self::Result { ... }
}

inventory::submit! { ReductionEntry { source_name, target_name, ... } }
```

## Anti-patterns
- Don't create reductions without closed-loop tests
- Don't forget `inventory::submit!` registration (graph won't update)
- Don't hardcode weights - use generic `W` parameter
- Don't skip `make clippy` before PR

## Contributing
See `.claude/rules/` for detailed guides:
- `adding-reductions.md` - How to add reduction rules
- `adding-models.md` - How to add problem types
- `testing.md` - Testing requirements and patterns

Also see GitHub Issue #3 for coding rules.
