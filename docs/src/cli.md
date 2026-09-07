# CLI Tool

Create and solve your first problem with `pred`. [Install the CLI](install.md) before running these commands in a fresh working directory.

## Create a graph problem

```bash
pred create MIS --graph 0-1,1-2,2-3,3-4,4-0 -o cycle.json
```

`MIS` means Maximum Independent Set: select as many pairwise non-adjacent vertices as possible. This graph is a cycle with five vertices.

## Solve and check

```bash
pred solve cycle.json
pred evaluate cycle.json --config 1,0,1,0,0
pred solve cycle.json --solver brute-force
```

Both solvers return `Max(2)`. The configuration `1,0,1,0,0` selects vertices 0 and 2 and evaluates to the same value. Several optimal configurations exist, so the solvers may return different selections.

The default solver discovers a route to ILP and maps the solution back. Brute force checks all configurations; keep instances small.

Next: [watch the full run](cli-demo.md), [inspect the path](cli-paths.md), or [save a reduction bundle](cli-reduce.md).
