# ThreeDimensionalMatching to ExactCoverBy3Sets

Implement issue #1103 as a witness-preserving `Or -> Or` reduction on top of commit `a9067297`. The construction maps each indexed source triple `(w, x, y)` over three size-`q` coordinate domains to `[w, q + x, 2 * q + y]` in a tagged universe of size `3q`; target and source configuration vectors have identical indexing.

References: Richard M. Karp, “Reducibility Among Combinatorial Problems” (1972), DOI `10.1007/978-1-4684-2001-2_9`; Garey and Johnson, *Computers and Intractability* (1979), Appendix A, SP1–SP2.

## Batch 1 — Reduction, registration, tests, and canonical example

Follow `.claude/skills/add-rule/SKILL.md` Steps 1–5.

1. Carry forward the completed mathematical verification: the tagged blocks make every target subset a valid three-element set; a source perfect matching selects `q` coordinate-disjoint triples iff the corresponding sets form an exact cover of all `3q` tagged elements. Solution extraction is the identity vector. Preserve duplicate triple indices. The verified feasible example is `q=3` with triples `[(0,0,0),(1,1,1),(2,2,2),(0,1,2),(1,2,0)]`; the verified infeasible example is `q=3` with `[(0,0,0),(0,1,1),(1,2,2)]`.
2. Add `src/rules/threedimensionalmatching_exactcoverby3sets.rs` with a direct `ReductionResult` implementation and `ReduceTo<ExactCoverBy3Sets> for ThreeDimensionalMatching`. Construct the target as `ExactCoverBy3Sets::new(3 * self.universe_size(), tagged_subsets)`. Use exact overhead metadata `universe_size = "3 * universe_size"`, `num_subsets = "num_triples"`, and `num_sets = "num_triples"`. Do not introduce adapters, registries, compatibility paths, or mapping state that identity extraction does not need.
3. Register the module directly in `src/rules/mod.rs` in the existing set-rule section.
4. Add `src/unit_tests/rules/threedimensionalmatching_exactcoverby3sets.rs`. Include the semantically named closed-loop test, exact target tagging and overhead assertions, infeasible/no-witness behavior, empty `q=0`, duplicate triples, unused coordinates, equal numeric coordinates across domains, and identity extraction. Keep every test under five seconds and use focused assertions rather than snapshots.
5. Add the canonical rule example using the current per-rule `canonical_rule_example_specs()` pattern in the rule module. Use the issue’s five-triple `q=3` instance and canonical witness `[1,1,1,0,0]`; ensure it is discovered through the existing example-db aggregation.
6. Run focused formatting and tests for the new rule, then `cargo run --example export_graph` to confirm the primitive edge and overhead metadata appear exactly once.

## Batch 2 — Paper documentation, generated fixtures, and final verification

Follow `.claude/skills/add-rule/SKILL.md` Steps 6–7 after Batch 1 is complete.

1. Add `load-example("ThreeDimensionalMatching", "ExactCoverBy3Sets")` bindings and a mandatory `reduction-rule("ThreeDimensionalMatching", "ExactCoverBy3Sets", ...)` entry near the existing ThreeDimensionalMatching reductions in `docs/paper/reductions.typ`.
2. Make the theorem self-contained: define `q`, the indexed triple list, the three tagged blocks, and the target sets; prove both directions independently; state identity extraction and exact size overhead. Cite Karp/Garey–Johnson using existing bibliography keys when available.
3. Add a tutorial-style `extra:` block starting with `pred-commands()` derived from `problem-spec()` and `target-spec()` on the loaded fixture. Walk through the exact five tagged subsets, verify the diagonal witness end-to-end, and explain that the two cross triples are mutually disjoint but cannot be extended by any available third triple. State that the fixture stores one canonical witness.
4. Regenerate the graph/schema exports and canonical example fixture with the repository commands. Commit only tracked artifacts required by the rule and paper; do not include ignored exports or temporary verification files.
5. Run `make paper`, `make test`, `make clippy`, formatting checks, and the repository’s coverage command. Inspect the final diff and working tree, remove the temporary plan file as required by `issue-to-pr`, and report any deviation from this plan in the PR implementation summary.
