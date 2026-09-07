# Find reduction paths

Choose source and target [variants](cli-variants.md), then query their direction of reachability.

```bash
pred path MIS ILP
pred from MIS
pred to QUBO
```

`from` explores outgoing routes; `to` explores incoming routes. `path` finds a route between the supplied endpoints. Search defaults to witness-capable reductions.

## Save and compare routes

```bash
pred path MIS QUBO -o path.json
pred path MIS QUBO --all --max-paths 50
pred path MIS QUBO --cost minimize-steps
pred path MIS QUBO --cost minimize:num_variables
```

`--all` is capped (20 paths by default); inspect the truncation indicator. A saved single path can be used with `pred reduce --via path.json`.

The default cost minimizes steps. Size-based costs use reduction overhead metadata; inspect available size fields with `pred show`. Overhead formulas describe scaling bounds, not exact constructed instance sizes.

## Multi-step example

```text
{{#include generated/pred-path-factoring-spinglass.txt}}
```

A discovered path does not imply the target will be inexpensive to solve. Measure the target size and solver behavior on representative instances.

Next: [apply a path](cli-reduce.md) or [understand overhead](design-paths.md).
