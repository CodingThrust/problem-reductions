# Solvers

## BruteForce Solver

The library includes a brute-force solver for testing and verification.

```rust
use problemreductions::prelude::*;

let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);
```

## Configuration

```rust
let solver = BruteForce::new()
    .valid_only(true)     // Only return valid solutions
    .tolerance(1e-6);     // Tolerance for floating-point comparisons
```

## Solver Trait

```rust
pub trait Solver<P: Problem> {
    fn find_best(&self, problem: &P) -> Vec<Vec<usize>>;
}
```

## Performance Notes

The brute-force solver enumerates all `num_flavors^num_variables` configurations. Use only for small instances (typically < 20 variables).
