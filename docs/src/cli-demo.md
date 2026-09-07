# CLI in action

From a graph to a checked solution in six commands. This recording runs the real CLI: discover a route, transform the instance, solve, and verify the result.

<iframe class="cli-cast" src="static/cli-demo.html" title="Terminal recording: reduce Maximum Independent Set to ILP and verify the solution" loading="lazy" allowfullscreen></iframe>

[Open the player](static/cli-demo.html) · [Download the cast](static/cli-demo.cast) · [Download the offline HTML](static/cli-demo.html)

## Run it yourself

[Install `pred`](install.md), then run in a fresh directory. The recording uses `./pred`, a local build of the same executable.

```bash
pred path MIS ILP
pred create MIS --graph 0-1,1-2,2-3,3-4,4-0 -o cycle.json
pred reduce cycle.json --to ILP -o reduced.json
pred solve reduced.json
pred evaluate cycle.json --config 1,0,1,0,0
pred solve cycle.json --solver brute-force
```

## What to check

The recorded route goes through Maximum Set Packing and a weight cast before reaching binary ILP. `reduced.json` preserves the source instance and path so the solver can recover a source solution.

| Check | Recorded result |
|---|---|
| Solve the reduction bundle | `Max(2)`, configuration `[1,0,1,0,0]` |
| Evaluate that source configuration | `Max(2)` |
| Exhaustive source solve | `Max(2)`, configuration `[0,0,1,0,1]` |
| Command exit codes | All six returned `0` |

Different configurations can have the same optimum. This cross-check validates this instance; a reduction's general correctness requires a mathematical argument.

Next: [JSON output for agents](cli-automation.md) or [find another route](cli-paths.md).
