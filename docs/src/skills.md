# Start with an agent

An agent can use the CLI directly from a repository checkout. Skills supply the task procedure; the CLI supplies current model and reduction data.

## Prepare the workspace

```bash
git clone https://github.com/CodingThrust/problem-reductions
cd problem-reductions
make cli
```

This installs `pred` into Cargo’s binary directory (usually `~/.cargo/bin`). Confirm it is on your shell’s `PATH` with `pred --version`.

## Give the agent this context

```text
Work in this repository. Read AGENTS.md and .claude/CLAUDE.md.
Use the built pred CLI to inspect current models, variants, and paths.
Read the SKILL.md for the task before following its workflow.
Report the exact model variant, commands, results, and unresolved assumptions.
```

Then choose [find a solver](agent-find-solver.md), [extend a solver's reach](agent-find-problem.md), or [propose a rule](agent-propose.md).

## Read without a browser

Use the [Markdown index](markdown/index.md) to retrieve only the relevant task pages. Code includes are expanded in those files. The [graph and schemas](cli-automation.md#registry-exports) provide structured data.

An agent with MCP support can instead [connect to `pred mcp`](mcp.md). Repository skills still require access to the checkout.
