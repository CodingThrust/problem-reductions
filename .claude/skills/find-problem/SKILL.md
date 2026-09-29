---
name: find-problem
description: Use when someone has (or plans) a solver for one model and wants to know which other problems it can handle through incoming reductions — ranks those problems by effective complexity against their best-known bounds and writes a solution doc
---

# Find Problem

Reverse of `find-solver`: given a solver for model `M` with complexity `T(M's parameters)`,
find the problems that reduce to `M`, compute what the solver costs on each, and document the
worthwhile ones in `docs/solutions/`.

Invocation: `/find-problem` or `/find-problem <Model>`.

<HARD-GATE>
No source edits, no Rust, no PRs. The only file written is the solution doc, after the user
confirms its filename. For contributions, point to `/propose`.
</HARD-GATE>

**Output visibility:** Bash output is hidden from the user. For every `pred` / `pred-sym`
command, say why, paste the full output in a fenced block, then interpret briefly. Never invent
output. Ask one question at a time and mark a recommendation.

`make cli` installs both `pred` and `pred-sym`; run it only if they are missing.

## 1. Solver and complexity

Validate the model with `pred show <Model>` and show its `parameters`. Get the solver's time
complexity written in those parameter names (e.g. `1.1996^num_vertices`); help formalize
informal answers ("exponential in n").

## 2. Discover and score sources

```bash
pred to <Model> --hops 3                    # incoming neighbors (default is 1 hop)
pred path <Source> <Model> --limit 5        # candidate paths with per-step and Overall: formulas
pred show <Source>                          # the source's own best-known complexity
```

- Use the exact variant-qualified names from `pred to` (e.g. `SpinGlass/SimpleGraph/f64`); bare
  names resolve to the default variant and can give false "no path" results.
- `pred path` returns a set of paths, not a single cheapest one; pick the path with the best
  composed overhead and say which. `unavailable` overhead formulas mean the effective
  complexity cannot be computed symbolically; report that rather than guess.
- **Effective complexity** = the solver's expression with each `M` parameter replaced by the
  path's `Overall:` formula in source parameters. Check every substitution:

```bash
pred-sym big-o "1.1996^(3 * num_clauses)"            # normal form
pred-sym eval --vars num_clauses=20 "1.1996^(3 * num_clauses)"
pred-sym compare "<effective>" "<best-known>"        # exits 1 when not Big-O equal
```

- Classify against the source's best-known bound: **Better**, **Similar**, or **Worse**. When
  the two bounds use different variables (e.g. `1.5^num_subsets` vs `2^universe_size`), evaluate
  both with `pred-sym eval` at representative sizes and state the crossover condition
  ("better when num_subsets ≤ c·universe_size").
- WebSearch real-world applications only for Better and Similar sources.

If more than ~15 sources come back, show the top 10 by effective complexity and offer the rest.

## 3. Rank and choose

Present one table (problem, hops, overhead, effective complexity, vs best-known, applications),
Better entries bolded. The user picks which go into the doc.

## 4. Solution doc

Propose `docs/solutions/problems-solvable-via-<Model>-<solver>.md` (`<solver>` a short label such
as `custom-1.1996` or `ILP`) and write it only after confirmation. Contents: overview and ranking
method, summary table, then per problem: what it is and where it appears, path, overhead,
effective complexity, comparison, and commands. Run `pred create <Source> --help` for each
selected source and use its real flags. Command pattern, verified against the current CLI:

```bash
pred create <Source> <flags> -o input.json
pred path <Source> <Model> --json | jq '.paths[0]' > route.json   # the entry you chose
pred reduce input.json --via route.json -o bundle.json            # bundle.target is the M instance
pred solve bundle.json --timeout 30                               # built-in solver, mapped back
```

For an external solver, feed it `bundle.target` and map its answer back with
`pred extract bundle.json --config '<target solution>'`. Whether a built-in solver exists for
`M` is shown by `pred inspect` on an instance (`solver_capabilities`); reachability alone does
not imply one.

After writing, summarize the doc, offer a live demo on one Better source when a built-in solver
applies (`--timeout 30`), and ask for changes.
