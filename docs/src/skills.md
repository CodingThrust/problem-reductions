# AI Agent Skills

AI coding assistants ([Claude Code](https://claude.ai/claude-code), [OpenCode](https://github.com/opencode-ai/opencode), [Codex](https://github.com/openai/codex)) can call `pred` CLI commands directly and use interactive skills to work with the reduction graph.

## Quick Start

Paste into Claude Code or Codex:

```
1. Clone https://github.com/CodingThrust/problem-reductions,
2. Build the pred CLI with `make cli` in the root directory,
3.1 Run `/find-solver` skill to help me find a solver for my scheduling problem.
3.2 Run `/find-problem` skill to help me find problems that my QUBO solver can solve.
3.3 Run `/propose` skill to help me propose a new problem or reduction to the project.
```

The prompt 3.1 is for users who have a real-world problem and need help finding a solver for it.
The prompt 3.2 is for users who have a solver and need help finding problems that it can solve.
The prompt 3.3 is for contributors who want to propose a new problem or reduction rule to the project.