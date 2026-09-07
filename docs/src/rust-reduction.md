# Apply a reduction

Reduce Maximum Set Packing to binary ILP, solve the target, and recover the original configuration. The default library features include the HiGHS backend.

```rust
use problemreductions::prelude::*;
use problemreductions::models::algebraic::ILP;
use problemreductions::solvers::ILPSolver;

fn main() {
    let problem = MaximumSetPacking::<i32>::new(vec![
        vec![0, 1], vec![1, 2], vec![2, 3], vec![4, 5],
    ]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let target = reduction.target_problem();
    assert_eq!(target.num_vars(), 4);
    assert_eq!(target.num_constraints(), 2);

    let target_solution = ILPSolver::new().solve(target).unwrap();
    let solution = reduction.extract_solution(&target_solution);
    assert_eq!(solution, vec![1, 0, 1, 1]);
    println!("{}", problem.evaluate(&solution)); // Max(3)
}
```

The target has one binary variable per set and excludes overlapping pairs. `extract_solution` maps a target witness to the source configuration space.

For types implementing `ReduceTo<ILP<bool>>`, `ILPSolver::new().solve_reduced(&problem)` combines these steps. It returns an optional source configuration; evaluate it against the original problem.

Next: [discover a multi-step path](rust-paths.md).
