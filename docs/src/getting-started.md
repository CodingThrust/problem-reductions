# Getting Started

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
problemreductions = "0.1"
```

## Basic Usage

### Creating a Problem

```rust
use problemreductions::prelude::*;

// Independent Set on a path graph
let is = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// Vertex Cover on the same graph
let vc = VertexCovering::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// QUBO problem
let qubo = QUBO::from_matrix(vec![
    vec![1.0, -2.0],
    vec![0.0, 1.0],
]);
```

### Solving a Problem

```rust
use problemreductions::prelude::*;

let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);

println!("Found {} optimal solutions", solutions.len());
for sol in &solutions {
    println!("  Solution: {:?}", sol);
}
```

### Applying Reductions

```rust
use problemreductions::prelude::*;

// Create an Independent Set problem
let is = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2)]);

// Reduce to Vertex Cover
let result = ReduceTo::<VertexCovering<i32>>::reduce_to(&is);
let vc = result.target_problem();

// Solve the reduced problem
let solver = BruteForce::new();
let vc_solutions = solver.find_best(vc);

// Extract solution back to original problem
let is_solution = result.extract_solution(&vc_solutions[0]);
```

### Chaining Reductions

```rust
use problemreductions::prelude::*;

let sp = SetPacking::<i32>::new(vec![
    vec![0, 1],
    vec![1, 2],
    vec![2, 3],
]);

// SetPacking -> IndependentSet -> VertexCovering
let sp_to_is = ReduceTo::<IndependentSet<i32>>::reduce_to(&sp);
let is = sp_to_is.target_problem();

let is_to_vc = ReduceTo::<VertexCovering<i32>>::reduce_to(is);
let vc = is_to_vc.target_problem();

// Solve and extract back through the chain
let solver = BruteForce::new();
let vc_solutions = solver.find_best(vc);
let is_solution = is_to_vc.extract_solution(&vc_solutions[0]);
let sp_solution = sp_to_is.extract_solution(&is_solution);
```

### Type Safety

The reduction system is compile-time verified. Invalid reductions won't compile:

```rust,compile_fail
// This won't compile - no reduction from QUBO to SetPacking
let result = ReduceTo::<SetPacking<i32>>::reduce_to(&qubo);
```

## Next Steps

- Explore the [interactive reduction graph](./introduction.html) to discover available reductions
- Browse the [API Reference](./api.md) for full documentation
- Check out the [Solvers](./solvers.md) for different solving strategies
