# Command reference

Every command accepts `--json` for structured output, `-o FILE` to save JSON, and `-q` to silence informational messages. `pred <command> --help` lists all flags; this page shows one example per command.

## Catalog

```bash
pred list
pred show MIS
pred list --rules
```

`list` reports every problem with its aliases, variants, and reduction counts. `show` describes the resolved variant, its size fields, input schema, and incoming and outgoing reductions. Read that schema before constructing an instance.

<details>
<summary>Example: <code>pred show MIS</code></summary>

```text
{{#include generated/pred-show-mis.txt}}
```

</details>

## Names and variants

```bash
pred show MIS/SimpleGraph/i32
pred path MIS/SimpleGraph/i32 ILP/bool
```

Aliases such as `MIS` resolve to full names, and a bare name selects the declared default variant: `MIS` is `MaximumIndependentSet/SimpleGraph/One`. Slash-separated parameters select graph, weight, or other variant values. `One` means unit weights; passing non-unit `--weights` to `create` upgrades a default instance to `i32`. Name the exact variant when a reproducible endpoint matters.

{{#include generated/pred-aliases.txt}}

## Paths

```bash
pred path MIS ILP
pred path MIS QUBO --all --max-paths 50
pred path MIS QUBO --cost minimize:num_variables -o path.json
pred from MIS --hops 2
pred to QUBO
```

`path` finds the cheapest route between two endpoints; `--all` enumerates alternatives up to `--max-paths`. The default cost minimizes steps; `minimize:<field>` uses the overhead metadata of a size field from `pred show`. `from` and `to` explore outgoing and incoming neighbors. Search defaults to reductions that can map a solution back.

<details>
<summary>Example: a multi-step path from <code>Factoring</code> to <code>SpinGlass</code></summary>

```text
{{#include generated/pred-path-factoring-spinglass.txt}}
```

</details>

Overhead formulas describe scaling bounds, not exact target sizes. A discovered route does not imply the target is cheap to solve; inspect the constructed target on representative instances.

## Create

```bash
pred create MIS --graph 0-1,1-2,2-3 -o problem.json
pred create MIS/SimpleGraph/i32 --graph 0-1,1-2,2-3 --weights 2,1,3,1 -o weighted.json
pred create --example MVC/SimpleGraph/i32 --to MIS/SimpleGraph/i32 -o source.json
pred create MIS --random --num-vertices 10 --edge-prob 0.3 --seed 42 -o random.json
```

Flags follow the schema field names in kebab-case: `universe_size` becomes `--universe-size`. Vertices use zero-based indices and `--graph` is a comma-separated edge list. `--example` loads a canonical model fixture, or with `--to` the source side of a documented reduction example (`--example-side target` for the other side). `--random` generates a graph instance; save the JSON so the instance can be reproduced.

Other input structures:

```bash
pred create SAT --num-vars 3 --clauses '1,2;-1,3' -o sat.json          # signed one-based literals; ';' separates clauses
pred create QUBO --matrix '1,0.5;0.5,2' -o qubo.json                    # ';' separates rows
pred create X3C --universe-size 6 --subsets '0,1,2;3,4,5;0,3,4' -o x3c.json
pred create Factoring --target 6 --m 2 --n 2 -o factoring.json
```

## Inspect and evaluate

```bash
pred inspect problem.json
pred evaluate problem.json --config 1,0,1,0
pred create MIS --graph 0-1,1-2,2-3 | pred evaluate - --config 1,0,1,0
```

`inspect` reports the resolved variant and sizes of a problem file or reduction bundle. `evaluate` scores one configuration: selecting vertices 0 and 2 returns `Max(2)`, while selecting adjacent vertices returns `Max(None)`. Configurations follow each problem's variable domains and are not always binary. `-` reads from stdin.

## Reduce

```bash
pred reduce problem.json --to QUBO -o reduced.json
pred reduce problem.json --via path.json -o reduced.json
pred extract reduced.json --config 1,0,1,0
```

The bundle contains the source instance, the target instance, and the variant-level path; keep it whole to preserve solution recovery. `--via` replays a path saved by `pred path -o`, whose source variant must match the input. `extract` maps a target-space configuration back to the source.

## Solve

```bash
pred solve problem.json
pred solve problem.json --solver brute-force
pred solve reduced.json --timeout 30 --json
```

| Solver | Behavior |
|---|---|
| `ilp` (default) | Finds a route to ILP, solves the target, and recovers a source configuration |
| `brute-force` | Enumerates all configurations; for tiny instances and cross-checks |
| `customized` | Exact structure-exploiting backends for selected models; see `pred solve --help` |

Solving a bundle solves the target and maps the result back; JSON output records the target result under `intermediate`. If the ILP solver reports no route, check `pred path <exact-variant> ILP`. Aggregate-only problems return a value without a configuration. To verify a result, evaluate the returned configuration on the original instance, or compare the value with an exhaustive solve on a small example.

## JSON and pipes

```bash
pred create MIS --graph 0-1,1-2,2-3 | pred reduce - --to QUBO | pred solve - --json
pred export-graph -o reduction_graph.json
```

Check the exit status before consuming a result, and enable `set -o pipefail` in scripts. Keep `type` and `variant` with each instance; an alias alone does not identify an exact endpoint. The site publishes [reduction_graph.json](reductions/reduction_graph.json) and [problem_schemas.json](reductions/problem_schemas.json) from the same registry; the local `export-graph` describes the installed version.

## Shell completions

```bash
eval "$(pred completions bash)"        # ~/.bashrc
eval "$(pred completions zsh)"         # ~/.zshrc
pred completions fish | source         # ~/.config/fish/config.fish
```

Without an argument, `pred completions` detects the current shell.
