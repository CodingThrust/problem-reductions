# HamiltonianCircuit to Satisfiability Implementation Plan

Issue: #1098, `[Rule] HamiltonianCircuit to Satisfiability`

Base commit: `a9067297c9b7759b4f1139692553a465de49fed3`

The implementation follows the repository's `add-rule` Steps 1-7. Mathematical verification has established the vertex-position encoding against every simple graph through five vertices, including solution extraction, overhead bounds, the fixed contradiction for `n < 3`, and the issue's YES/NO examples.

## Batch 1: Implement and test the reduction

1. Add `src/rules/hamiltoniancircuit_satisfiability.rs`.
   - Implement `ReduceTo<Satisfiability>` for `HamiltonianCircuit<SimpleGraph>` and a witness-preserving `ReductionResult`.
   - For `n < 3`, emit one SAT variable with clauses `(z)` and `(not z)`.
   - For `n >= 3`, map `(vertex, position)` to the 1-indexed SAT literal `vertex * n + position + 1`.
   - Emit exactly-one constraints for every position and every vertex, followed by cyclic forbidden-successor clauses for equal vertices and non-edges.
   - Decode a satisfying assignment by reading the unique true vertex at each position.
   - Register the safe overhead bounds from the issue: `num_vars = num_vertices * num_vertices + 1`, `num_clauses = 2 * num_vertices + num_vertices * num_vertices * (num_vertices - 1) + num_vertices^3 + 2`, and `num_literals = 4 * num_vertices^3 + 2`.

2. Register the module and example provider in `src/rules/mod.rs`.

3. Add focused tests in `src/unit_tests/rules/hamiltoniancircuit_satisfiability.rs`.
   - Closed loop on a triangle through the SAT brute-force solver.
   - Exact structure and representative successor clauses on the five-cycle-plus-chord example.
   - Extraction for both orientations of a circuit.
   - Unsatisfiability of the five-vertex path and of every `n < 3` source instance.
   - Semantics for isolated vertices, self-loops, and parallel edges.
   - Assert the registered overhead evaluates to bounds that cover the constructed target.

4. Add `hamiltoniancircuit_to_satisfiability` to the canonical rule example database from the issue's five-cycle-plus-chord source and its 25-bit diagonal position assignment. Regenerate `src/example_db/fixtures/examples.json` and verify the stored source witness, SAT witness, extracted circuit, and target metrics.

5. Run focused Rust tests, formatter, and the standard non-paper verification needed before documentation.

## Batch 2: Paper entry and final verification

1. With fresh context after Batch 1, add the Velev-Gao SARA 2009 reference to `docs/paper/references.bib` if it is not already present.

2. Add the mandatory `reduction-rule("HamiltonianCircuit", "Satisfiability", ...)` entry to `docs/paper/reductions.typ`.
   - Load the canonical example fixture and derive the `pred create --example`, reduce, solve, and evaluate commands from it.
   - Explain the `n x n` position matrix, the two exactly-one families, cyclic forbidden-successor clauses, both directions of correctness, and witness extraction.
   - Walk through the five-cycle-plus-chord example using fixture data, including a representative forbidden pair and the recovered circuit.
   - State that the fixture contains one canonical SAT witness while rotated/reversed Hamiltonian circuits produce multiple satisfying assignments.

3. Regenerate the reduction graph and schemas, regenerate fixtures, and run `make paper`.

4. Run `make test clippy`, `make coverage`, and inspect the final diff/status. Commit only files required for issue #1098; remove this plan file before the final push.
