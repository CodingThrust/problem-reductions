# Problem Reductions

[![CI](https://github.com/CodingThrust/problem-reductions/actions/workflows/ci.yml/badge.svg)](https://github.com/CodingThrust/problem-reductions/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/CodingThrust/problem-reductions/branch/main/graph/badge.svg)](https://codecov.io/gh/CodingThrust/problem-reductions)
[![Crates.io](https://img.shields.io/crates/v/problemreductions.svg)](https://crates.io/crates/problemreductions)
[![Documentation](https://docs.rs/problemreductions/badge.svg)](https://docs.rs/problemreductions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for NP-hard problem definitions and reductions.

## Features

- **18+ Problem Types**: Implementations of classic NP-hard problems
- **Type-Safe Reductions**: Compile-time verified problem transformations
- **BruteForce Solver**: For testing and verification on small instances
- **Topology Types**: HyperGraph and UnitDiskGraph support
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
let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// Solve with brute force
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);

// Apply a reduction
let result = ReduceTo::<VertexCovering<i32>>::reduce_to(&problem);
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

## License

MIT License - see [LICENSE](LICENSE) for details.
