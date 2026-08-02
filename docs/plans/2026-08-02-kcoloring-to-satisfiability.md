# Implement KColoring to Satisfiability reduction

Issue: #1099

Base commit: `a9067297c9b7759b4f1139692553a465de49fed3`

Reference: Daniel Faber, Adalat Jabrayilov, and Petra Mutzel, “SAT Encoding of Partial Ordering Models for Graph Coloring Problems,” SAT 2024, Section 2.2, DOI `10.4230/LIPIcs.SAT.2024.12`.

## Required behavior

For `KColoring<KN, SimpleGraph>` with `n` vertices, `m` stored edges, and runtime color count `k`, construct a `Satisfiability` instance with Boolean variable `x[v,c]` at one-based SAT index `v * k + c + 1`:

1. For every vertex `v`, add the at-least-one clause `(x[v,0] ∨ ... ∨ x[v,k-1])`.
2. For every vertex `v` and pair `a < b`, add `(¬x[v,a] ∨ ¬x[v,b])`.
3. For every stored edge `(u,v)` and color `c`, add `(¬x[u,c] ∨ ¬x[v,c])`.
4. Extract one source color per vertex by finding the true variable in its `k`-wide block.

Register only the runtime `KN` / `SimpleGraph` endpoint as the primitive reduction. The exact target sizes are:

- `num_vars = num_vertices * num_colors`
- `num_clauses = num_vertices + num_vertices * num_colors * (num_colors - 1) / 2 + num_edges * num_colors`
- `num_literals = num_vertices * num_colors + num_vertices * num_colors * (num_colors - 1) + 2 * num_edges * num_colors`

The construction must preserve the repository's complete legal domain: empty graphs, `k = 0`, isolated vertices, disconnected graphs, self-loops, parallel edges, and `k > n`.

## Batch 1: verification and Rust implementation

Follow `.claude/skills/add-rule/SKILL.md` Steps 0–5 and Step 7, including the default mathematical verification in `.claude/skills/verify-reduction/SKILL.md`.

1. Produce an ephemeral standalone Typst proof with independent forward and backward correctness directions, extraction, exact overhead, a feasible five-cycle example with `k = 3`, and an infeasible five-cycle example with `k = 2`.
2. Produce and run an ephemeral constructor verifier with symbolic overhead checks, exhaustive/simple-graph feasibility agreement through five vertices, extraction checks for every satisfying assignment tested, exact target-size checks, structural checks, and at least 5,000 checks.
3. Independently adversarially verify the proof with a separately written constructor/extractor, exhaustive tests through five vertices, two Hypothesis strategies, at least 5,000 checks, and cross-comparison against the constructor verifier.
4. Add `src/rules/kcoloring_satisfiability.rs` containing the direct `ReductionResult`, `ReduceTo<Satisfiability>` implementation, exact `#[reduction(overhead = ...)]` metadata, construction, extraction, and the linked test module. Use `CNFClause` and the target's signed one-based literal convention directly.
5. Register the module in `src/rules/mod.rs` with no new dispatch or compatibility layer.
6. Add `src/unit_tests/rules/kcoloring_satisfiability.rs`. Cover the semantic closed loop, exact clause families and sizes, extraction, five-cycle `k=3`/`k=2`, empty `n=0,k=0`, nonempty `k=0`, isolated vertices, self-loop infeasibility, parallel-edge clause duplication, and `k>n`. Keep each test below five seconds.
7. Add the canonical C5 `k=3` rule example through the rule module's `canonical_rule_example_specs()`, using source colors `[0,1,0,1,2]` and the corresponding 15-bit one-hot SAT assignment.
8. Run focused tests, format checks, graph/schema exports, fixture regeneration, and `make test clippy`. Do not commit ephemeral verification scripts or generated ignored documentation exports.

## Batch 2: paper documentation and final verification

Follow `.claude/skills/add-rule/SKILL.md` Step 6 with fresh context after Batch 1 is complete.

1. Add the Faber–Jabrayilov–Mutzel SAT 2024 BibTeX entry to `docs/paper/references.bib` if it is not already present.
2. Add a `reduction-rule("KColoring", "Satisfiability", ...)` entry to `docs/paper/reductions.typ` based on the verified proof. State the construction, both correctness directions, variable mapping, extraction, and exact scaling. Explicitly distinguish the standard positive-`k`, loop-free literature domain from the repository edge cases handled by the same clauses.
3. Build a fixture-driven tutorial example for C5 with `k=3`. Start the extra block with `pred-commands()` derived from the loaded example, show the 15 variables and 35 clauses, verify `[0,1,0,1,2]` end to end, and explain witness multiplicity without relying on fixture solution counts.
4. Regenerate graph/schema exports and example fixtures after the documentation is connected, then run `make paper`, `make fmt-check`, `make test`, and `make clippy`.
5. Inspect the final diff and working tree. Keep only files directly required by #1099, delete this plan file before the final push, and leave the branch clean.
