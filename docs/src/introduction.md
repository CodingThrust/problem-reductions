# Problem Reductions

A Rust library for reducing NP-hard problems.

## Overview

**problemreductions** provides implementations of various NP-hard computational problems and reduction rules between them. It is designed for algorithm research, education, and quantum optimization studies.

## Features

- **18+ Problem Types**: Implementations of classic NP-hard problems
- **Type-Safe Reductions**: Compile-time verified problem transformations
- **BruteForce Solver**: For testing and verification on small instances
- **Topology Types**: HyperGraph and UnitDiskGraph support
- **File I/O**: JSON serialization for all problem types
- **Truth Tables**: Utilities for logic gadget construction

## Quick Example

```rust
use problemreductions::prelude::*;

// Create an Independent Set problem on a triangle graph
let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);

// Solve with brute force
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);

// Maximum independent set in a triangle has size 1
assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
```

## Problem Categories

| Category | Problems |
|----------|----------|
| **Satisfiability** | SAT, K-SAT, CircuitSAT, Factoring |
| **Graph** | IndependentSet, VertexCovering, MaxCut, Coloring, DominatingSet, MaximalIS, Matching |
| **Set** | SetCovering, SetPacking |
| **Optimization** | SpinGlass, QUBO |
| **Specialized** | Paintshop, BicliqueCover, BMF |

## License

MIT License
