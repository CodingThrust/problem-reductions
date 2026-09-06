# JSON and automation

Use structured output for agent tools and scripts.

```bash
pred list --json
pred show MIS --json
pred path MIS QUBO --json
pred solve problem.json -o solution.json
```

| Global flag | Effect |
|---|---|
| `--json` | Request JSON output for data commands |
| `-o, --output <FILE>` | Save JSON to a file |
| `-q, --quiet` | Suppress informational messages on stderr |

Check the process exit status before consuming a result. Preserve `type` and `variant` with each instance; an alias alone is insufficient to replay an exact endpoint.

## Pipe instances

```bash
pred create MIS --graph 0-1,1-2,2-3 | pred solve - --json
pred create MIS --graph 0-1,1-2,2-3 | pred reduce - --to QUBO | pred solve - --solver brute-force --json
```

`-` reads a problem or bundle from stdin. In shell automation, enable `set -o pipefail` so a failed earlier command fails the pipeline.

## Registry exports

```bash
pred export-graph -o reduction_graph.json
```

The docs build publishes [reduction_graph.json](reductions/reduction_graph.json) and [problem_schemas.json](reductions/problem_schemas.json). The graph includes exact variants, directed edges, capabilities, and symbolic overheads; schemas describe model fields.

Use the [Markdown index](markdown/index.md) for task instructions and these JSON files for structured registry queries. The exports describe the website's build; local CLI output describes the installed version.
