# Reduce an instance

**Before you start:** [create a problem file](cli-create.md) and [find a route](cli-paths.md).

## Choose a target

```bash
pred reduce problem.json --to QUBO -o reduced.json
pred inspect reduced.json
```

The reduction bundle contains the source instance, target instance, and variant-level path. Keep the whole bundle to preserve solution recovery.

## Use a saved path

```bash
pred path MIS QUBO -o path.json
pred reduce problem.json --via path.json -o reduced.json
```

The path determines the target; `--to` is unnecessary. The input must match the path's source variant. For a weighted input, find a path from its exact weighted variant.

## Solve and recover

```bash
pred solve reduced.json --solver brute-force
```

The solver solves the target and maps the result back through the bundle. Keep targets small when using brute force. For JSON output, the `intermediate` field records the target result alongside the recovered source solution.

Next: [solver options](cli-solve.md) or [pipeline commands](cli-automation.md).
