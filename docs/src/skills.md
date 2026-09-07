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
| `check-issue` | Quality gate for `[Model]` and `[Rule]` issues: usefulness, non-triviality, literature, and writing. Posts a report. |
| `fix-issue` | Fixes problems found by `check-issue`, then re-checks and moves the issue to Ready. |
| `issue-to-pr` | Converts an approved issue into a pull request with an implementation plan. |
| `add-model` | Adds a problem model: source, variants, tests, canonical example, and paper entry. |
| `add-rule` | Adds a reduction rule with the same artifacts, verified mathematically by default. |
| `verify-reduction` | Standalone verification of a rule: Typst proof, a constructor script, and an adversary script with thousands of independent checks. |
| `fix-pr` | Resolves review comments, CI failures, and coverage gaps on a pull request. |
| `write-model-in-paper`, `write-rule-in-paper` | Write or improve an entry in the Typst paper. |

For a rule, distinguish a construction supported by literature from a new conjecture, and record proof gaps and counterexamples explicitly.

## Maintain

| Skill | What it produces |
|---|---|
| `run-pipeline` | Takes one Ready issue from the project board through implementation to the Review pool. |
| `review-pipeline` | Agentic review of a pull request: structural check, quality check, and feature tests. Moves it to Final review. |
| `review-structural`, `review-quality` | The two read-only sub-reviews, usable on their own. |
| `final-review` | Interactive maintainer review, then merge or hold. |
| `auto-pipeline` | Chains the steps above from a Backlog issue to Final review. |
| `topology-sanity-check` | Detects isolated problems, missing NP-hardness chains from 3-SAT, and dominated rules. |
| `review-paper` | Reviews ten paper entries for mechanical and critical issues. |
| `release` | Determines the version bump, verifies tests, and tags a release. |
| `dev-setup` | Installs and configures the development tools. |
| `update-papers` | Downloads referenced papers and regenerates the collection index. |

The corresponding make targets run in a configured maintainer checkout:

```bash
make run-issue N=42      # implement one issue
make run-pipeline        # pick the next Ready issue
make run-review N=570    # review one pull request
```

A passing test suite is evidence for the tested instances, not a proof for all inputs. Keep the mathematical argument, the constructor and adversary checks, closed-loop tests, and review findings with the work.

## Reading without a browser

The [Markdown index](markdown/index.md) lists every page of this guide with code includes expanded. [reduction_graph.json](reductions/reduction_graph.json) and [problem_schemas.json](reductions/problem_schemas.json) provide the registry as structured data.

## Authorship

Contributors of ten non-trivial reduction rules are added to the author list of the [paper](reductions.pdf). The software is MIT licensed.
