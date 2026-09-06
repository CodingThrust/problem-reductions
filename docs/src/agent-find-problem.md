# Extend a solver's reach

Find source problems that can reduce to a solver you already have.

**Before you start:** complete [agent setup](skills.md). Specify the solver's accepted model, topology, weights, and practical size limit.

## Prompt

```text
Read .claude/skills/find-problem/SKILL.md and follow it.
My solver accepts: [exact model and variant].
Its time complexity and practical size limit: [details].
Find incoming reduction routes and rank useful source problems.
Show the assumptions and composed size overhead for each recommendation.
Write the findings to docs/solutions/.
```

## Check the direction

A solver for a target can handle sources that reduce **to** it. For example:

```bash
pred to QUBO
pred path MIS QUBO
```

A route in the opposite direction does not establish that capability. Confirm each candidate's exact variants and measure the constructed target size on a small instance.

Next: [reduce an instance](cli-reduce.md).
