# Create an instance

**Input:** a model name or exact variant, plus fields from `pred show <model>`.
**Output:** a JSON problem file containing `type`, `variant`, and `data`.

```bash
pred create MIS --graph 0-1,1-2,2-3 -o problem.json
pred create MIS/SimpleGraph/i32 --graph 0-1,1-2,2-3 --weights 2,1,3,1 -o weighted.json
pred inspect weighted.json
```

Vertices use zero-based indices. `--graph` is a comma-separated edge list; weights must match the model's expected vertices or edges.

## Use a canonical example

```bash
pred create --example MIS/SimpleGraph/i32 -o model.json
pred create --example MVC/SimpleGraph/i32 --to MIS/SimpleGraph/i32 -o source.json
pred create --example MVC/SimpleGraph/i32 --to MIS/SimpleGraph/i32 --example-side target -o target.json
```

The first command loads a model fixture. The next two load the source and target of a documented reduction example.

CLI fields follow schema names: `universe_size` becomes `--universe-size`, and `subsets` becomes `--subsets`.

Next: [input examples](cli-examples.md), [random instances](cli-random.md), or [inspect and evaluate](cli-inspect.md).
