---
name: find-solver
description: Use when someone has a real-world or formally named problem and wants to know how to solve it with this library — matches it to a model, explores reduction paths, checks which built-in solvers actually apply, surveys external solvers, and writes a solution doc
---

# Find Solver

Take the user from a problem ("assign jobs to machines, minimize makespan", or "MIS on unit-disk
graphs") to a concrete solving recipe, ending in one doc under `docs/solutions/`.

Invocation: `/find-solver` (start by clarifying the problem) or `/find-solver <Problem>`
(validate with `pred show` and go straight to reductions).

<HARD-GATE>
No source edits, no Rust, no PRs. The only file written is the solution doc, after the user
confirms its filename. For contributions, point to `/propose`.
</HARD-GATE>

**Output visibility:** Bash output is hidden from the user. For every `pred` command, say why
you run it, then paste its full output in a fenced block, then 1-3 sentences of interpretation.
Every command runs for real; never invent output.

Ask one question at a time and always mark a recommended choice.

## 1. Pin down the problem

For a fuzzy description, clarify only what is missing: input structure (graph, sets, formula,
numbers), objective (min, max, feasibility), typical instance size. Build `pred` first if
absent (`command -v pred || make cli`).

## 2. Match to models

WebSearch the clarified problem ("… NP-hard", "… reduction") before proposing matches; don't
guess from memory. Search the catalog with `pred list <keyword>` (`pred list --category graph`,
`pred list --all` for more). Present 3-5 candidates with why/caveat, run `pred show <model>` on
each so the user sees the required input fields and complexity, and let them pick.

**Optimization vs decision mismatch.** If the user wants to minimize/maximize but the model is a
feasibility problem (value `Or`, a `bound`/`deadline` field), say so: the optimum needs a binary
search over the bound, and the doc gets a "Finding the optimum" section.

## 3. Explore reduction paths

```bash
pred from <Model> --hops 3                  # reachable targets
pred path <Model> <Target> --limit 5        # a set of candidate paths with composed parameters
pred show <Target>                          # target complexity
```

- Use the exact variant-qualified names printed by `pred from` (e.g. `SpinGlass/SimpleGraph/f64`).
  Bare names resolve to the default variant and can produce false "no path" results.
- `pred path` returns several paths, not a ranked best. Compare them yourself by hops and the
  `Overall:` parameter formulas; some formulas are `unavailable`, say so rather than guess.
- If more than ~15 targets come back, show the top 10 and offer the rest.

Present a table (target, hops, composed overhead, target complexity) with a recommendation and
note blowups (e.g. quadratic QUBO variables only pay off on annealing hardware).

## 4. Solvers

**Reachability does not imply a built-in solver.** Solver dispatch uses only registered
customized solvers and fixed ILP pipelines. Check with a concrete instance:

```bash
pred create --example <Model/Variant> -o ex.json    # or: pred create <Model> <flags>
pred inspect ex.json        # solver_capabilities: brute_force, customized, ilp (with its fixed path)
```

`pred solve --solver` accepts `customized`, `ilp`, `brute-force`; omit it for the default
(customized, else ILP, else brute force). Brute force is exact but only for small instances
(~25 variables). ILP uses HiGHS. Also inspect the final target of the chosen path.

Then WebSearch "<target> solver benchmark open source" for external tools, and present built-in
and external options together; the user picks which go into the doc.

## 5. Solution doc

Propose `docs/solutions/<problem>-via-<model>-<solver>.md` (kebab-case problem description) and
write it only after confirmation. Sections: problem description; matched model (variant, why,
complexity); input schema from `pred show <model> --json` plus an example instance; the chosen
reduction path with per-step overhead; solving and interpreting the output (`Max(3)`,
`Or(true)`); finding the optimum (decision models only); external alternatives (if chosen); a
quick-reference command block. Use real flags from `pred create <Model> --help`.

The explicit-path workflow, verified against the current CLI:

```bash
pred create <Model> <flags> -o input.json
pred path <Model> <Target> --json | jq '.paths[0]' > route.json   # pick the entry you chose
pred reduce input.json --via route.json -o bundle.json
pred solve bundle.json --timeout 30        # solves the target, maps the solution back
pred evaluate input.json --config '<solution>'
```

`--via` takes exactly one path entry, not the whole `{paths, truncated}` set. If the model has a
registered ILP pipeline, `pred solve input.json --solver ilp` is the one-step alternative.

After writing, summarize the doc, offer a live end-to-end demo when a built-in solver applies
(always `--timeout 30`), and ask for changes.
