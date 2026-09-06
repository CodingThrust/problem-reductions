# Inspect and evaluate

Use `inspect` to identify a problem file or reduction bundle. Use `evaluate` to check a specific configuration against the original instance.

## Inspect a file

```bash
pred create MIS --graph 0-1,1-2,2-3 -o problem.json
pred inspect problem.json
pred inspect problem.json --json
```

Inspect the resolved variant and sizes before choosing a solver.

## Evaluate a configuration

```bash
pred evaluate problem.json --config 1,0,1,0
```

This selects vertices 0 and 2 and returns `Max(2)`. Selecting adjacent vertices is invalid and returns `Max(None)`.

Configurations follow each problem's variable domains; they are not always binary. Check the model definition before constructing one.

## Read from stdin

```bash
pred create MIS --graph 0-1,1-2,2-3 | pred evaluate - --config 1,0,1,0
```

Evaluation checks one candidate. It does not establish optimality; compare with [a solver](cli-solve.md).
