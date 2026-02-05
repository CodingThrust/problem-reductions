# Problem Reductions

[![CI](https://github.com/CodingThrust/problem-reductions/actions/workflows/ci.yml/badge.svg)](https://github.com/CodingThrust/problem-reductions/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/CodingThrust/problem-reductions/graph/badge.svg?token=0CdEC8GHN0)](https://codecov.io/github/CodingThrust/problem-reductions)
[![Documentation](https://github.com/CodingThrust/problem-reductions/actions/workflows/docs.yml/badge.svg)](https://codingthrust.github.io/problem-reductions/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for NP-hard problem definitions and reductions.

## Features

- **18+ Problem Types**: Implementations of classic NP-hard problems
- **Type-Safe Reductions**: Compile-time verified problem transformations
- **Graph Abstraction**: Generic `Graph` trait with `SimpleGraph` and `UnitDiskGraph` implementations
- **Multiple Solvers**: BruteForce and ILP (HiGHS) solvers
- **Topology Types**: HyperGraph and UnitDiskGraph for specialized constraints
- **File I/O**: JSON serialization for all problem types

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
problemreductions = "0.1"
```

## Quick Start

```rust
use problemreductions::prelude::*;

// Create an Independent Set problem
let problem: IndependentSetT = IndependentSetT::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// Solve with brute force
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);

// Apply a reduction
let result = ReduceTo::<VertexCoverT>::reduce_to(&problem);
let vc = result.target_problem();
```

## Problem Types

| Category | Problems |
|----------|----------|
| **Satisfiability** | SAT, K-SAT, CircuitSAT, Factoring |
| **Graph** | IndependentSet, MaximalIS, VertexCovering, DominatingSet, Coloring, MaxCut, Matching |
| **Set** | SetCovering, SetPacking |
| **Optimization** | SpinGlass, QUBO |
| **Specialized** | Paintshop, BicliqueCover, BMF |

## Available Reductions

- IndependentSet ↔ VertexCovering
- IndependentSet ↔ SetPacking
- SpinGlass ↔ QUBO
- SpinGlass ↔ MaxCut

## Documentation

- [User Guide](https://CodingThrust.github.io/problem-reductions/)
- [API Reference](https://docs.rs/problemreductions)

## Development

### Using Make

```bash
make help      # Show all available targets
make build     # Build the project
make test      # Run all tests
make fmt       # Format code with rustfmt
make fmt-check # Check code formatting
make clippy    # Run clippy lints
make doc       # Build and open documentation
make coverage  # Generate coverage report (requires cargo-llvm-cov)
make clean     # Clean build artifacts
make check     # Quick check before commit (fmt + clippy + test)
```

### Using Cargo directly

```bash
cargo build --all-features
cargo test --all-features
cargo doc --all-features --no-deps --open
```

## Contributing

### Authorship Recognition

**Contribute 10 non-trivial reduction rules and you will be automatically added to the author list of the paper.**

### Using Claude Code (Recommended)

1. Find or create a GitHub issue describing your contribution
2. Run the issue-to-pr skill:
   ```
   /issue-to-pr <issue-number>
   ```
3. Brainstorm with Claude using `superpowers:brainstorming` to clarify requirements
4. The skill creates a PR starting with `[action]`, which automatically triggers Claude CI to implement the plan

### Manual Contribution

1. Follow guides in `.claude/rules/` for adding reductions or models
2. Run `make test clippy export-graph` before submitting
3. Ensure >95% test coverage for new code

## License

MIT License - see [LICENSE](LICENSE) for details.
