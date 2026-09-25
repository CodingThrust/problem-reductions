# Skills

Skills are task procedures under `.claude/skills/<name>/SKILL.md` in the repository. An agent reads one and follows it, using the `pred` CLI for current model, variant, and path data. They work with Claude Code as slash commands (`/find-solver`) and with any agent that can read files.

## Setup

```bash
git clone https://github.com/CodingThrust/problem-reductions
cd problem-reductions
make cli
```

This installs `pred` into Cargo's binary directory. Give the agent this context:

```text
Work in this repository. Read AGENTS.md and .claude/CLAUDE.md.
Use the built pred CLI to inspect current models, variants, and paths.
Read the SKILL.md for the task before following its workflow.
Report the exact model variant, commands, results, and unresolved assumptions.
```

Every skill is invoked the same way. Name the skill, then describe the task:

```text
Read .claude/skills/find-solver/SKILL.md and follow it.
My problem: [inputs, constraints, and objective].
Typical size: [counts and ranges].
```

## Use the atlas

| Skill | What it produces |
|---|---|
| `find-solver` | Matches a real-world problem to a library model, explores reduction routes, and recommends solvers. Writes a solution document to `docs/solutions/`. |
| `find-problem` | The reverse: given a solver for one model, lists the source problems it can handle through incoming reductions, ranked by effective complexity. |

Ask the agent to construct a small instance, solve it, and evaluate the recovered configuration on the original problem before scaling up. A solver for a target handles sources that reduce **to** it; a route in the other direction establishes nothing.

## Contribute

| Skill | What it produces |
|---|---|
| `propose` | Turns a definition or a candidate source-to-target connection into a precise proposal and files a GitHub issue. |

The remaining workflow has no queue or project board. Point an agent at an issue ("implement #42") and it follows these guides on its own:

| Guide | Covers |
|---|---|
| `how-to-triage-issue` | Quality check of `[Model]` and `[Rule]` issues: usefulness, non-triviality, literature, completeness, writing. Fixes mechanical problems and discusses substantive ones. |
| `how-to-code` | Adding a problem model or reduction rule: source, variants, tests, canonical example. |
| `how-to-verify` | Certifying a rule: type check, Typst proof, and a constructor and an adversary script with thousands of independent checks, posted as a verification certificate on the pull request. Also reduction-graph topology checks. |
| `how-to-write-manual` | Writing or auditing entries in the Typst manual and these docs. |
| `how-to-review` | Fresh-context review of a pull request: structural check, quality check, and a feature test through `pred`. |
| `how-to-ship` | From issue to a merge-ready pull request: gates, review, CI, review comments, and coverage. A maintainer merges. |

For a rule, distinguish a construction supported by literature from a new conjecture, and record proof gaps and counterexamples explicitly.

A passing test suite is evidence for the tested instances, not a proof for all inputs. Keep the mathematical argument, the constructor and adversary checks, closed-loop tests, and review findings with the work.

## Maintain

| Skill | What it produces |
|---|---|
| `release` | Checks the tree, proposes the version bump, and tags a release after confirmation. |
| `dev-setup` | Installs and configures the development tools. |
| `update-papers` | Downloads referenced papers and regenerates the collection index. |

## Reading without a browser

The [Markdown index](markdown/index.md) lists every page of this guide with code includes expanded. [reduction_graph.json](reductions/reduction_graph.json) and [problem_schemas.json](reductions/problem_schemas.json) provide the registry as structured data.

## Authorship

Contributors of ten non-trivial reduction rules are added to the author list of the [paper](reductions.pdf). The software is MIT licensed.
