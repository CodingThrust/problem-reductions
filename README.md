# Problem Reductions

A Rust library for NP-hard problem definitions and reductions.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
problemreductions = "0.1.0"
```

## Quick Start

```rust
use problemreductions::prelude::*;
use problemreductions::models::graph::IndependentSet;

// Create an Independent Set problem on a triangle graph
let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);

// Solve with brute force
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);

// Maximum independent set in a triangle has size 1
assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
```

## Supported Problems

| Category | Problems |
|----------|----------|
| **Satisfiability** | SAT, K-SAT, CircuitSAT, Factoring |
| **Graph** | IndependentSet, MaximalIS, VertexCovering, DominatingSet, Coloring, MaxCut, Matching |
| **Set** | SetCovering, SetPacking |
| **Optimization** | SpinGlass, QUBO |
| **Specialized** | Paintshop, BicliqueCover, BMF |

## Development

```bash
# Build
cargo build

# Test
cargo test

# Run clippy
make clippy

# Generate docs
cargo doc --open
```

## License

MIT
