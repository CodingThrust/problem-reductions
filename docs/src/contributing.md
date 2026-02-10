# Contributing

Contributions are welcome! See [CLAUDE.md](./claude.md) for the full list of commands and architecture details.

## Finding Issues

Browse [GitHub Issues](https://github.com/CodingThrust/problem-reductions/issues) to find tasks. Issues labeled `good first issue` are a great starting point.

## Authorship Recognition

**Contribute 10 non-trivial reduction rules and you will be automatically added to the author list of the paper.**

## Workflow

1. Find or create a GitHub issue describing your proposal.
2. Write a detailed plan in `docs/plans/issue-<number>-<slug>.md`.
3. Implement, test, and submit a PR.

If you use [Claude Code](https://github.com/anthropics/claude-code), the [`/issue-to-pr`](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/skills/issue-to-pr.md) skill automates the brainstorming, planning, and PR creation workflow.

## Before Submitting

- `cargo fmt --all` — format code
- `cargo clippy --all-targets --all-features -- -D warnings` — no warnings
- `cargo test --all-features -- --include-ignored` — all tests pass
- New code must have >95% test coverage

## Adding Problems and Reductions

See the developer guides in `.claude/rules/`:
- [`adding-models.md`](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-models.md) — how to add problem types
- [`adding-reductions.md`](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/adding-reductions.md) — how to add reduction rules
- [`testing.md`](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/rules/testing.md) — testing requirements and patterns
