# SetSplitting to NAESatisfiability implementation plan

Issue: #1100 `[Rule] SetSplitting to NAESatisfiability`

The implementation follows the repository `add-rule` workflow. Both models use
`Or`, so the reduction is witness-capable through `ReduceTo<NAESatisfiability>`.
The construction encodes element `u` as positive, one-indexed literal `u + 1`,
deduplicates each source subset while preserving first-occurrence order, and
emits `(x_u, x_u)` when a subset contains only repetitions of one element.

## Batch 1: verify and implement the rule

1. Run the `verify-reduction` workflow before writing Rust:
   - prove that a source subset is split exactly when its positive-literal NAE
     clause contains both truth values;
   - cover the all-repeated subset with the unsatisfiable repeated-literal
     clause;
   - verify direct witness extraction and the overhead bounds
     `num_vars = universe_size`, `num_clauses = num_subsets`, and
     `num_literals <= (universe_size + 1) * num_subsets`;
   - run independent constructor and adversary checks with at least 5,000
     checks each and cross-compare their target construction.
2. Add `src/rules/setsplitting_naesatisfiability.rs` with a direct
   `ReductionResult`, identity witness extraction, deterministic first-seen
   deduplication, `CNFClause` construction, and the required overhead metadata.
3. Register the module in `src/rules/mod.rs` without introducing any dispatch
   layer or compatibility path.
4. Add focused tests in
   `src/unit_tests/rules/setsplitting_naesatisfiability.rs`:
   - closed-loop feasibility and witness extraction for the issue example;
   - exact target structure and overhead bounds;
   - repeated members and the all-repeated infeasible case;
   - empty family, unused universe elements, and clauses of arity two through
     greater than three;
   - canonical example-db output.
5. Add the issue's four-element example to the rule's
   `canonical_rule_example_specs()` implementation and ensure the existing
   example-db collector discovers it.

## Batch 2: paper documentation and generated fixtures

1. With fresh context after Batch 1, add a self-contained
   `reduction-rule("SetSplitting", "NAESatisfiability", ...)` entry to
   `docs/paper/reductions.typ`, following the existing reverse rule and the
   KColoring-to-QUBO tutorial structure.
2. Describe the construction, both correctness directions, identity solution
   extraction, duplicate canonicalization, and the all-repeated NO case.
3. Build the worked example from the canonical example-db fixture and begin its
   `extra:` block with the `pred-commands()` create/reduce/solve/evaluate flow.
4. Regenerate the reduction graph, schemas, and example-db fixtures, then run
   `make paper`.

## Final verification

Run formatting, tests, and clippy (`make fmt-check`, `make test`, and
`make clippy`). Confirm that tracked changes are limited to the rule,
registration, tests, canonical fixture/export updates, and paper entry.
