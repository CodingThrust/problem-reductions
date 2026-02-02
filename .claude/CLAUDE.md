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

### Reduction Pattern (Recommended: Using Macro)
```rust
use problemreductions::reduction;

#[reduction(
    overhead = { ReductionOverhead::new(vec![...]) }
)]
impl ReduceTo<TargetProblem<Unweighted>> for SourceProblem<Unweighted> {
    type Result = ReductionSourceToTarget;
    fn reduce_to(&self) -> Self::Result { ... }
}
```

The `#[reduction]` macro automatically:
- Extracts type names from the impl signature
- Detects weighted vs unweighted from type parameters (`Unweighted` vs `i32`/`f64`)
- Detects graph types from type parameters (e.g., `GridGraph`, `SimpleGraph`)
- Generates the `inventory::submit!` call

Optional macro attributes:
- `source_graph = "..."` - Override detected source graph type
- `target_graph = "..."` - Override detected target graph type
- `source_weighted = true/false` - Override weighted detection
- `target_weighted = true/false` - Override weighted detection
- `overhead = { ... }` - Specify reduction overhead

### Manual Registration (Alternative)
```rust
inventory::submit! {
    ReductionEntry {
        source_name: "SourceProblem",
        target_name: "TargetProblem",
        source_graph: "SimpleGraph",
        target_graph: "SimpleGraph",
        source_weighted: false,
        target_weighted: false,
        overhead_fn: || ReductionOverhead::new(...),
    }
}
```

### Weight Types
- `Unweighted` - Marker type for unweighted problems (all weights = 1)
- `i32`, `f64`, etc. - Concrete weight types for weighted problems

### Problem Variant IDs
Reduction graph nodes use variant IDs: `ProblemName[/GraphType][/Weighted]`
- Base: `IndependentSet` (SimpleGraph, unweighted)
- Graph variant: `IndependentSet/GridGraph`
- Weighted variant: `IndependentSet/Weighted`
- Both: `IndependentSet/GridGraph/Weighted`

## Anti-patterns
- Don't create reductions without closed-loop tests
- Don't forget `inventory::submit!` registration (graph won't update)
- Don't hardcode weights - use generic `W` parameter
- Don't skip `make clippy` before PR

## Documentation Requirements

The technical paper (`docs/paper/reductions.typ`) must include:

1. **Table of Contents** - Auto-generated outline of all sections
2. **Problem Data Structures** - For each problem definition, include the Rust struct with fields in a code block
3. **Reduction Examples** - For each reduction theorem, include a minimal working example showing:
   - Creating the source problem
   - Reducing to target problem
   - Solving and extracting solution back
   - Based on closed-loop tests from `tests/reduction_tests.rs`

### Documentation Pattern
```typst
#definition("Problem Name")[
  Mathematical definition...
]

// Rust data structure
```rust
pub struct ProblemName<W = i32> {
    field1: Type1,
    field2: Type2,
}
`` `

#theorem[
  *(Source → Target)* Reduction description...
]

// Minimal working example
```rust
let source = SourceProblem::new(...);
let reduction = ReduceTo::<TargetProblem>::reduce_to(&source);
let target = reduction.target_problem();
// ... solve and extract
`` `
```

## Contributing
See `.claude/rules/` for detailed guides:
- `adding-reductions.md` - How to add reduction rules
- `adding-models.md` - How to add problem types
- `testing.md` - Testing requirements and patterns

Also see GitHub Issue #3 for coding rules.
