# Using Reductions

## Basic Usage

```rust
use problemreductions::prelude::*;

// Create source problem
let is = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// Reduce to target problem
let result = ReduceTo::<VertexCovering<i32>>::reduce_to(&is);
let vc = result.target_problem();

// Solve target problem
let solver = BruteForce::new();
let vc_solutions = solver.find_best(vc);

// Extract solution back to source problem
let is_solution = result.extract_solution(&vc_solutions[0]);

// Verify solution is valid
assert!(is.solution_size(&is_solution).is_valid);
```

## Chaining Reductions

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

## Type Safety

The reduction system is compile-time verified. Invalid reductions won't compile:

```rust,compile_fail
// This won't compile - no reduction from QUBO to SetPacking
let result = ReduceTo::<SetPacking<i32>>::reduce_to(&qubo);
```
