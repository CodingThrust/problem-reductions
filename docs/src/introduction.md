# Documentation

Use executable reductions to connect a hard problem to a solver. Explore the catalog, transform an instance, and check the recovered solution. These guides are organized around small tasks for agents and their operators.

## See what works

[Watch the CLI solve a graph problem →](cli-demo.md)

A real terminal run: discover a route, reduce to ILP, recover an independent set, and cross-check the optimum. Six commands, about half a minute.

## Choose a task

| You have… | Start here | You get… |
|---|---|---|
| A problem to solve | [Find a solver](agent-find-solver.md) | A model, reduction route, and solver recommendation |
| A solver to reuse | [Extend its reach](agent-find-problem.md) | Reachable source problems and size overheads |
| A candidate connection | [Propose a model or rule](agent-propose.md) | A precise research proposal |
| An approved issue | [Implement and review](agent-pipeline.md) | A tested implementation for review |
| An instance to run | [First solve](cli.md) | A solution checked against the original problem |

## Give an agent the right context

Start with [agent setup](skills.md). Every page has a **Markdown** link for direct reading; the [Markdown index](markdown/index.md) lists all tasks. Use [JSON exports](cli-automation.md) for registry data and [MCP](mcp.md) for tool access.

## Research scope

The long-term goal is autonomous discovery of new reduction rules. Today, this repository provides executable models, registered reductions, solver routing, and agent workflows for proposals, implementation, and review. A candidate rule needs a mathematical argument as well as tests; passing finite examples does not establish a general proof.

[Explore the atlas](index.html#atlas) · [Read the mathematical manual](reductions.pdf) · [Browse the Rust API](api.md)
