# MaxCut/One to Maximum2Satisfiability

Implement issue #1102 as a witness-preserving reduction from
`MaxCut<SimpleGraph, One>` to `Maximum2Satisfiability`. The branch is stacked on
commit `a9067297`, where the exact source variant is introduced.

Reference: Gramm, Hirsch, Niedermeier, and Rossmanith, “Worst-case upper bounds
for MAX-2-SAT with an application to MAX-CUT,” *Discrete Applied Mathematics*
130(2), 2003, DOI `10.1016/S0166-218X(02)00402-X`; open preprint ECCC
TR00-037, Section 5.

## Batch 1: verify and implement the reduction

Follow `.agents/skills/add-rule/SKILL.md` Steps 0–5 and 7, using the repository
copy of that skill from the parent checkout when the stacked worktree does not
contain `.agents/`.

1. Run the full `verify-reduction` workflow before editing Rust code. Verify the
   pointwise identity

   `satisfied_clauses(x) = num_edges + cut_edges(x)`

   with a standalone Typst proof, an independent constructor validator, and an
   adversary validator. Cover all graphs through five vertices, direct witness
   extraction, exact overhead, empty and isolated graphs, self-loops, parallel
   edges, the five-edge worked example, and a three-or-more-vertex negative
   threshold example. Keep all verification artifacts outside the repository.

2. Add `src/rules/maxcut_maximum2satisfiability.rs`. Implement
   `ReduceTo<Maximum2Satisfiability>` specifically for
   `MaxCut<SimpleGraph, One>`. For each edge occurrence `(u, v)`, append
   `CNFClause::new(vec![(u + 1) as i32, (v + 1) as i32])` and
   `CNFClause::new(vec![-((u + 1) as i32), -((v + 1) as i32)])`. Preserve the
   target assignment directly in `extract_solution`. Register exact overhead
   `num_vars = num_vertices` and `num_clauses = 2 * num_edges`.

3. Register the module directly in `src/rules/mod.rs`. Do not add adapters,
   alternate variants, or compatibility paths.

4. Add `src/unit_tests/rules/maxcut_maximum2satisfiability.rs` with focused
   semantic tests:

   - `test_maxcut_to_maximum2satisfiability_closed_loop` checks both optima and
     every extracted optimum on the four-cycle-plus-diagonal example.
   - A structure/pointwise test checks all ten clauses, exact metrics, direct
     extraction, and `target_value = 5 + source_value` for every assignment.
   - Boundary tests cover empty, isolated, disconnected, self-loop, and
     parallel-edge inputs, including repeated literals and repeated clause
     pairs.
   - An exhaustive small-graph test checks the affine identity and round trip
     without introducing a golden fixture.

5. Add the canonical four-cycle-plus-diagonal example to
   `canonical_rule_example_specs()` in the new rule module. Use source config
   `[0, 1, 0, 1]` and the identical target config, with source optimum `4` and
   target optimum `9`.

6. Regenerate the reduction graph, schemas, and example fixtures. Run focused
   tests, `make test`, `make clippy`, `make fmt-check`, and `make coverage`.
   Keep tracked generated data required by the rule; do not commit ignored
   exports or temporary verification artifacts.

## Batch 2: document the rule with fresh context

After Batch 1 and fixture regeneration, follow `.agents/skills/add-rule/SKILL.md`
Step 6.

1. Add the BibTeX entry for Gramm et al. if it is not already present.
2. Add a `MaxCut` to `Maximum2Satisfiability` `reduction-rule` entry in
   `docs/paper/reductions.typ`, loading the canonical rule fixture and deriving
   the source-side `pred create --example` command from the loaded variant.
3. State the construction, prove both directions independently via the
   per-edge truth table, state direct witness extraction, and explain why loops
   and parallel edges preserve the identity.
4. Include a tutorial-style walkthrough of the four-cycle-plus-diagonal
   fixture: five edges, ten clauses, cut witness `[0,1,0,1]`, cut value `4`, and
   target value `9`. State that the fixture stores one canonical optimum.
5. Run `make paper`, then rerun formatting, linting, tests, and coverage after
   all documentation and fixture changes.

## Completion

Commit the implementation and documentation in coherent commits, remove this
plan file, post a PR implementation summary including any deviations, push the
stacked branch, and leave the issue ready for the review pipeline.
