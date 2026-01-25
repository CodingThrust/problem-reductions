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

## Next Steps

- Learn about [Problem Types](./problems/index.md)
- Explore [Available Reductions](./reductions/index.md)
- Check out the [API Reference](./api.md)
