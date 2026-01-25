# Solvers

The library provides multiple solvers for finding optimal solutions to computational problems.

## BruteForce Solver

A complete solver that enumerates all configurations. Guaranteed to find all optimal solutions but only practical for small instances.

```rust
use problemreductions::prelude::*;

let problem: IndependentSetT = IndependentSetT::new(4, vec![(0, 1), (1, 2), (2, 3)]);
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);

// Get solutions with their sizes
let solutions_with_size = solver.find_best_with_size(&problem);
```

### Configuration

```rust
use problemreductions::prelude::*;

let solver = BruteForce::new()
    .valid_only(true);   // Only return valid solutions (default: true)

// For floating-point problems
let solver = BruteForce::with_tolerance(1e-6, 1e-6);
```

### Performance

The brute-force solver enumerates all `num_flavors^num_variables` configurations. Use only for small instances (typically < 20 variables).

## ILP Solver (Optional)

An Integer Linear Programming solver using HiGHS. Enables exact solving for much larger instances than brute force.

### Enabling the ILP Feature

Add the `ilp` feature to your `Cargo.toml`:

```toml
[dependencies]
problemreductions = { version = "0.1", features = ["ilp"] }
```

### Usage

```rust,ignore
use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;

let problem: IndependentSetT = IndependentSetT::new(100, edges);
let solver = ILPSolver::new();

if let Some(solution) = solver.solve(&problem) {
    println!("Found solution: {:?}", solution);
}

// With time limit
let solver = ILPSolver::with_time_limit(60.0);  // 60 seconds
```

### Working with ILP

The ILP solver works with the `ILP` problem type directly. Problems that implement `ReduceTo<ILP>` can be solved using the `solve_reduced` method.

```rust,ignore
use problemreductions::models::optimization::{ILP, LinearConstraint, ObjectiveSense};
use problemreductions::solvers::ILPSolver;

// Create an ILP directly
let ilp = ILP::binary(
    2,
    vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
    vec![(0, 1.0), (1, 2.0)],
    ObjectiveSense::Maximize,
);

let solver = ILPSolver::new();
if let Some(solution) = solver.solve(&ilp) {
    println!("Solution: {:?}", solution);
}
```

### Solving Problems via Reduction

For problems that implement `ReduceTo<ILP>`, use `solve_reduced`:

```rust,ignore
use problemreductions::solvers::ILPSolver;

let problem = SomeProblem::new(...); // Problem that implements ReduceTo<ILP>
let solver = ILPSolver::new();

if let Some(solution) = solver.solve_reduced(&problem) {
    println!("Solution: {:?}", solution);
}
```

## Solver Trait

All solvers implement the `Solver` trait:

```rust,ignore
pub trait Solver {
    /// Find the best solution(s) for a problem.
    fn find_best<P: Problem>(&self, problem: &P) -> Vec<Vec<usize>>;

    /// Find the best solution(s) with their sizes.
    fn find_best_with_size<P: Problem>(
        &self,
        problem: &P,
    ) -> Vec<(Vec<usize>, SolutionSize<P::Size>)>;
}
```

## Comparing Solvers

```rust,ignore
use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;

let problem: IndependentSetT = IndependentSetT::new(15, edges);

// Brute force (complete but slow)
let bf = BruteForce::new();
let bf_solutions = bf.find_best(&problem);

// ILP (fast but returns single solution)
let ilp = ILPSolver::new();
let ilp_solution = ilp.solve(&problem);

// Verify both find the same optimal value
let bf_size = problem.solution_size(&bf_solutions[0]).size;
let ilp_size = problem.solution_size(&ilp_solution.unwrap()).size;
assert_eq!(bf_size, ilp_size);
```
