# Solve an instance

**Input:** a problem JSON file or a complete reduction bundle.
**Output:** the evaluation and, for witness-capable problems, a solution configuration.

```bash
pred solve problem.json
pred solve problem.json --solver brute-force
pred solve problem.json --timeout 30
pred solve problem.json -o solution.json
```

## Choose a solver

| Solver | Behavior |
|---|---|
| `ilp` (default) | Finds a witness-capable ILP route, solves the target, and recovers a source configuration |
| `brute-force` | Enumerates configurations; use for tiny instances and cross-checks |
| `customized` | Uses exact backends for selected models; inspect `pred solve --help` for supported models |

If ILP reports no route, inspect `pred path <exact-variant> ILP` and try brute force on a small instance. A model's presence in the catalog does not guarantee every solver supports it.

## Solve a bundle

```bash
pred solve reduced.json --json
```

A bundle requires a witness-capable target and path. Its result includes the recovered source solution and the intermediate target result.

Aggregate-only problems produce values without representative configurations. Do not assume every successful solve contains a witness.

## Verify the result

Evaluate the returned configuration on the original instance using `pred evaluate`. For a tiny example, compare its objective with an exhaustive solve. Different optimal configurations are acceptable when their evaluations agree.

Next: [JSON and automation](cli-automation.md).
