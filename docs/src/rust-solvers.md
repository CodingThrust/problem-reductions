# Solver contracts

Choose a solver by its input and result contract.

| API | Result | Scope |
|---|---|---|
| `Solver::solve` with `BruteForce` | Aggregate value | Any problem whose value implements `Aggregate` |
| `BruteForce::find_witness` | Optional configuration | Values supporting representative witnesses |
| `ILPSolver::solve` | Optional ILP configuration | ILP instances; requires an ILP backend |
| `ILPSolver::solve_reduced` | Optional source configuration | Witness-capable problems implementing `ReduceTo<ILP<bool>>` |
| `CustomizedSolver` | Model-specific exact solving | Selected models with specialized implementations |

The aggregate `Solver::solve` contract differs from the inherent witness-returning ILP methods. Counting and other aggregate-only problems need a value, not a representative configuration.

## Feature selection

HiGHS is enabled by default through `ilp-highs`. To use the library without it:

```bash
cargo add problemreductions --no-default-features
```

See the [solver API](api/problemreductions/solvers/index.html) for exact method signatures and supported specialized backends.

## Verification

For witness results, evaluate the recovered configuration against the source. For aggregate results, compare values according to the reduction's extraction contract. Use small exhaustive solves as independent checks.
