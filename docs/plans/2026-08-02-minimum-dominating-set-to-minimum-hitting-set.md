# Implement MinimumDominatingSet/One to MinimumHittingSet

Issue: #1096

Base: `1075-growth-domain` at `a9067297`

## Scope

Add one witness-preserving reduction from
`MinimumDominatingSet<SimpleGraph, One>` to `MinimumHittingSet`. The source
vertices become target universe elements, and every source vertex contributes
its closed neighborhood as one target set. Target configurations map back by
identity, preserving feasibility and cardinality.

The construction follows Bannach and Tantau, *Computing Hitting Set Kernels By
AC^0-Circuits* (STACS 2018, DOI 10.4230/LIPIcs.STACS.2018.9), which states the
closed-neighborhood equivalence directly. Garey and Johnson (1979) supplies the
endpoint definitions already cited by the issue.

## Batch 1: Verify and implement the reduction

Follow `add-rule` Steps 1-5 and 7 for the code, tests, and canonical example.

1. Run the full `verify-reduction` workflow before editing Rust:
   - prove that `D subset.eq V` dominates `G` iff it hits every closed
     neighborhood `N[v]`;
   - verify identity extraction and exact objective preservation;
   - verify `universe_size = num_vertices` and
     `num_sets = num_vertices` symbolically and concretely;
   - exercise all simple graphs through five vertices with at least 5,000
     constructor checks and an independent adversary implementation;
   - include a five-vertex path as the feasible/optimal example and a
     three-vertex instance with an impossible fixed candidate configuration as
     the negative feasibility example.
2. Add
   `src/rules/minimumdominatingset_minimumhittingset.rs`:
   - build one sorted closed-neighborhood vector per vertex;
   - construct `MinimumHittingSet::new(num_vertices, sets)`;
   - implement identity `extract_solution`;
   - register the exact endpoint pair with overhead fields
     `universe_size = "num_vertices"` and
     `num_sets = "num_vertices"`.
3. Register the module and its canonical example specs in `src/rules/mod.rs`.
4. Add focused tests in
   `src/unit_tests/rules/minimumdominatingset_minimumhittingset.rs`:
   - required optimization closed loop on the five-vertex path;
   - exact target closed-neighborhood structure;
   - identity extraction;
   - empty, isolated, disconnected, star, and complete graph behavior;
   - exhaustive objective/feasibility equivalence for all simple graphs
     through four vertices.
5. Add the issue's path example to the rule's canonical example specs with
   source and target witness `[0, 1, 0, 1, 0]`.
6. Run focused tests and formatting, then regenerate the graph, schemas, and
   example fixture required by the paper.

## Batch 2: Document the rule in the paper

Follow `add-rule` Step 6 with fresh context after Batch 1 has produced the
canonical fixture.

1. Add the Bannach--Tantau citation to `docs/paper/references.bib` if it is not
   already present.
2. Add a `reduction-rule("MinimumDominatingSet", "MinimumHittingSet", ...)`
   entry to `docs/paper/reductions.typ`, selecting the source variant
   `(graph: "SimpleGraph", weight: "One")`.
3. Derive the `pred create --example` command from the loaded fixture via the
   existing `problem-spec`/`target-spec` helpers.
4. Explain construction, both correctness directions, exact overhead,
   identity extraction, and the five-vertex path round trip in tutorial form.
5. Run `make paper` and correct any paper or fixture mismatch.

## Final verification

Run `cargo run --example export_graph`,
`cargo run --example export_schemas`, `make regenerate-fixtures`,
`make test`, `make clippy`, `make fmt-check`, and `make paper`. Inspect the
working tree so only issue-required tracked changes are committed and the plan
file is removed before the final push.
