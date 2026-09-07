# Find a solver

Map a concrete problem to a library model and a usable solver route.

**Before you start:** complete [agent setup](skills.md). Describe the inputs, constraints, objective, and expected instance sizes.

## Prompt

```text
Read .claude/skills/find-solver/SKILL.md and follow it.
My problem: [describe inputs, constraints, and objective].
Typical size: [counts and ranges].
Available solvers or hardware: [list, or no preference].
Check the exact model variant and current reduction paths with pred.
Separate implemented routes from suggestions that need new work.
```

## Expected result

A solution document in `docs/solutions/` with the proposed model, assumptions, reduction route, size overheads, and solver recommendation.

## Check the recommendation

Ask the agent to construct a small instance, solve it, and evaluate the recovered configuration on the original problem. If several models fit, resolve the modeling differences before scaling up.

Next: [first solve](cli.md) or [inspect a route](cli-paths.md).
