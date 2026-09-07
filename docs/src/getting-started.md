# Getting started

```bash
cargo add problemreductions
```

The library includes the HiGHS ILP backend.

## Solve a small instance

```rust
use problemreductions::prelude::*;

fn main() {
    let problem = MaximumSetPacking::<i64>::new(vec![
        vec![0, 1], vec![1, 2], vec![2, 3], vec![4, 5],
    ]);
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap().unwrap();
    println!("{:?}: {}", solution, problem.evaluate(&solution).unwrap());
}
```

The optimal packing selects sets 0, 2, and 3: the witness is `[true, false, true, true]` and evaluates to `Max(3)`. `Problem::evaluate` scores a configuration; `BruteForce` enumerates the configuration space, so keep exhaustive examples small.

## Apply a reduction

Reduce the same instance to binary ILP, solve the target, and recover the original configuration:

```rust
use problemreductions::prelude::*;
use problemreductions::models::algebraic::ILP;
use problemreductions::solvers::ILPSolver;

fn main() {
    let problem = MaximumSetPacking::<i64>::new(vec![
        vec![0, 1], vec![1, 2], vec![2, 3], vec![4, 5],
    ]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).unwrap();
    let target = reduction.target_problem();
    assert_eq!(target.num_vars(), 4);
    assert_eq!(target.num_constraints(), 2);

    let target_solution = ILPSolver::new().solve(target).unwrap();
    let solution = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(solution, vec![true, false, true, true]);
    println!("{}", problem.evaluate(&solution).unwrap()); // Max(3)
}
```

The target has one binary variable per set and a constraint for each element shared by multiple sets. `extract_solution` maps a target solution back to the source solution type. `ILPSolver::new().solve(&problem)` executes the exact variant’s registered ILP pipeline and returns its source solution.

## Discover and run a path

Search uses exact variants. This discovers a route from `Factoring` to `SpinGlass` and executes it:

```rust,ignore
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:imports}}
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step1}}
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step2}}

let reduction = graph.reduce_along_path(rpath, &factoring).unwrap().unwrap();
let target: &SpinGlass<SimpleGraph, f64> = reduction.target_problem();
// Solve `target`, then call reduction.extract_solution(&target_solution).
```

`extract_solution` walks the intermediate mappings in reverse. The full [example](https://github.com/CodingThrust/problem-reductions/blob/main/examples/chained_reduction_factoring_to_spinglass.rs) also solves factoring through a direct ILP reduction and checks that the recovered factors multiply to 6:

```bash
cargo run --example chained_reduction_factoring_to_spinglass
```

## Solver contracts

| API | Result | Scope |
|---|---|---|
| `BruteForce::solve` | `Result<Option<P::Solution>, SolveError>` | Registered finite search spaces; `None` proves infeasibility |
| `ILPSolver::solve` | `Result<P::Solution, ILPSolveError>` | Exact variants with registered ILP pipelines |

Every successful solve returns the problem's `Solution`. Evaluate it against the source with `Problem::evaluate`, which returns `Result<P::Value, EvaluationError>`. Path discovery enumerates routes; it does not rank them or register a solver capability. See the [solver API](api/problemreductions/solvers/index.html).
