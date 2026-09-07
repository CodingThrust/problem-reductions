# Implement and review

Move a prepared issue through the repository's agent workflow. These commands act on GitHub issues, project status, and pull requests; run them in a configured maintainer checkout.

## Implement one issue

```bash
make run-issue N=42
```

Replace `42` with the issue number. The task follows `.claude/skills/issue-to-pr/SKILL.md`. Model and rule skills define the source, tests, examples, and paper changes required.

To pick one eligible Ready issue from the project board:

```bash
make run-pipeline
```

## Review one pull request

```bash
make run-review N=570
```

Replace `570` with the PR number. The review workflow checks structure, quality, and user-facing behavior, then moves the PR to Final review. Maintainer review controls acceptance.

## Evidence to retain

Keep the mathematical argument, constructor and adversarial checks, closed-loop tests, canonical example, and review findings with the work. A passing test suite is evidence for tested instances, not a proof for all inputs.

For exact workflow rules, read [the repository instructions](https://github.com/CodingThrust/problem-reductions/blob/main/.claude/CLAUDE.md).
