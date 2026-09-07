# Getting started

```bash
cargo add problemreductions
```

HiGHS is enabled by default through the `ilp-highs` feature. Add `--no-default-features` to use the library without an ILP backend.

## Solve a small instance

```rust
use problemreductions::prelude::*;

fn main() {
    let problem = MaximumSetPacking::<i32>::new(vec![
        vec![0, 1], vec![1, 2], vec![2, 3], vec![4, 5],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    println!("{:?}: {}", solution, problem.evaluate(&solution));
}
```

The optimal packing selects sets 0, 2, and 3: the witness is `[1, 0, 1, 1]` and evaluates to `Max(3)`. `Problem::evaluate` scores a configuration; `BruteForce` enumerates the configuration space, so keep exhaustive examples small.

## Apply a reduction

Reduce the same instance to binary ILP, solve the target, and recover the original configuration:

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

The target has one binary variable per set and one constraint per overlapping pair. `extract_solution` maps a target witness back to the source configuration space. For any type implementing `ReduceTo<ILP<bool>>`, `ILPSolver::new().solve_reduced(&problem)` combines these steps and returns an optional source configuration.

## Discover and run a path

Search uses exact variants. This discovers a route from `Factoring` to `SpinGlass` and executes it:

```rust,ignore
use problemreductions::prelude::*;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::topology::SimpleGraph;
use problemreductions::types::ProblemSize;
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step1}}

let factoring = Factoring::new(2, 2, 6);
let reduction = graph.reduce_along_path(&rpath, &factoring).unwrap();
let target: &SpinGlass<SimpleGraph, f64> = reduction.target_problem();
// Solve `target` with a compatible solver, then recover source bits:
// let source_config = reduction.extract_solution(&target_config);
```

`extract_solution` walks the intermediate mappings in reverse. The full [example](https://github.com/CodingThrust/problem-reductions/blob/main/examples/chained_reduction_factoring_to_spinglass.rs) also solves factoring through a direct ILP reduction and checks that the recovered factors multiply to 6:

```bash
cargo run --example chained_reduction_factoring_to_spinglass
```

## Solver contracts

| API | Result | Scope |
|---|---|---|
| `Solver::solve` with `BruteForce` | Aggregate value | Any problem whose value implements `Aggregate` |
| `BruteForce::find_witness` | Optional configuration | Values supporting representative witnesses |
| `ILPSolver::solve` | Optional ILP configuration | ILP instances; requires an ILP backend |
| `ILPSolver::solve_reduced` | Optional source configuration | Problems implementing `ReduceTo<ILP<bool>>` |
| `CustomizedSolver` | Model-specific exact solving | Selected models with specialized implementations |

Counting and other aggregate-only problems produce a value, not a representative configuration. For witness results, evaluate the recovered configuration against the source; for aggregate results, compare values through the reduction's extraction contract. Method signatures are in the [solver API](api/problemreductions/solvers/index.html).
