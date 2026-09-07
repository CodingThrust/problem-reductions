# Command reference

Every command accepts `--json` for structured output, `-o FILE` to save JSON, and `-q` to silence informational messages. `pred <command> --help` lists all flags; this page shows one example per command.

## Catalog

```bash
pred list
pred show MIS
pred list --rules
```

`list` reports every problem with its aliases, variants, and reduction counts. `show` describes the resolved variant, its parameter fields, input schema, and incoming and outgoing reductions. Read that schema before constructing an instance.

<details>
<summary>Example: <code>pred show MIS</code></summary>

```text
{{#include generated/pred-show-mis.txt}}
```

</details>

## Names and variants

```bash
pred show MIS/SimpleGraph/i64
pred path MIS/SimpleGraph/i64 ILP/bool
```

Aliases such as `MIS` resolve to full names, and a bare name selects the declared default variant: `MIS` is `MaximumIndependentSet/SimpleGraph/One`. Slash-separated parameters select graph, weight, or other variant values. `One` means unit weights; passing non-unit `--weights` to `create` upgrades a default instance to `i64`. Name the exact variant when a reproducible endpoint matters.

{{#include generated/pred-aliases.txt}}

## Paths

```bash
pred path MIS ILP
pred path MIS QUBO --limit 50
pred path MIS QUBO --json -o paths.json
pred from MIS --hops 2
pred to QUBO
```

`path` enumerates witness-capable simple routes between exact endpoints, without ranking. `--limit` accepts 1 through 999, or `all` for 999; the default is 20. JSON output contains `paths` and `truncated`. `from` and `to` explore outgoing and incoming neighbors.

<details>
<summary>Example: a multi-step path from <code>Factoring</code> to <code>SpinGlass</code></summary>

```text
{{#include generated/pred-path-factoring-spinglass.txt}}
```

</details>

Parameter transforms declare exact equalities, upper bounds, or unavailable relations. A discovered route does not imply the target is cheap to solve; inspect the constructed target on representative instances.

## Create

```bash
pred create MIS --graph 0-1,1-2,2-3 -o problem.json
pred create MIS/SimpleGraph/i64 --graph 0-1,1-2,2-3 --weights 2,1,3,1 -o weighted.json
pred create --example MVC/SimpleGraph/i64 --to MIS/SimpleGraph/i64 -o source.json
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
pred evaluate problem.json --config '[true,false,true,false]'
pred create MIS --graph 0-1,1-2,2-3 | pred evaluate - --config '[true,false,true,false]'
```

`inspect` reports the resolved variant and sizes of a problem file or reduction bundle. `evaluate` scores one configuration: selecting vertices 0 and 2 returns `Max(2)`, while selecting adjacent vertices returns `Max(None)`. Configurations follow each problem's variable domains and are not always binary. `-` reads from stdin.

For a problem file, JSON inspection includes `parameter_values`, the model's actual named instance parameters. These are separate from the `parameters` list of parameter names.

## Reduce

```bash
pred path MIS QUBO --json -o paths.json
python3 -c 'import json; print(json.dumps(json.load(open("paths.json"))["paths"][0]))' > path.json
pred reduce problem.json --via path.json -o reduced.json
pred extract reduced.json --config '[1,0,1,0]'
```

The bundle contains the source instance, the target instance, and the variant-level path; keep it whole to preserve solution recovery. `--via` replays one route extracted from the `paths` envelope, whose source variant must match the input. `extract` maps a target-space configuration back to the source.

## Solve

```bash
pred solve problem.json
pred solve problem.json --solver brute-force
pred solve reduced.json --timeout 30 --json
```

| Solver | Behavior |
|---|---|
| `ilp` | Executes the exact variant’s registered fixed ILP pipeline and recovers its source solution |
| `brute-force` | Enumerates all configurations; for tiny instances and cross-checks |
| `customized` | Exact structure-exploiting backends for selected models; see `pred solve --help` |

Default dispatch tries registered customized, ILP, then brute-force capabilities in that order. `pred inspect` lists the capabilities available for the exact variant. Solving a bundle solves its target and maps the result back; every successful solve returns a solution. A discovered path does not by itself provide a registered solver. Evaluate the returned solution on the original instance to verify its value.

## JSON and pipes

```bash
pred create MIS --graph 0-1,1-2,2-3 | pred reduce - --via path.json | pred solve - --json
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
