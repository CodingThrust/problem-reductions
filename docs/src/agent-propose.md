# Propose a model or rule

Turn a mathematical idea or published construction into a precise proposal for the atlas.

**Before you start:** complete [agent setup](skills.md). Bring a definition, a source reference, or a candidate source → target connection.

## Prompt

```text
Read .claude/skills/propose/SKILL.md and follow it.
I want to propose: [model or source-to-target reduction].
Reference or construction: [details].
Check the current catalog for existing models and exact variant endpoints.
Clarify the objective, solution mapping, correctness argument, and overhead.
Prepare the proposal for the issue workflow.
```

## Expected result

A model or rule proposal with explicit assumptions and enough mathematical detail to assess usefulness and correctness. The skill guides issue creation.

For a rule, distinguish a construction supported by literature from a new conjecture. Record proof gaps and counterexamples explicitly.

Next: [implementation and review](agent-pipeline.md) once the issue is ready.
