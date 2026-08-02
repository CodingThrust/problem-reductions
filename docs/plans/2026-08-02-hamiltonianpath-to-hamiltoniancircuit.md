# HamiltonianPath to HamiltonianCircuit

Implement issue #1097 as a witness-preserving reduction from
`HamiltonianPath<SimpleGraph>` to `HamiltonianCircuit<SimpleGraph>`, following
the `add-rule` workflow.

## Batch 1: verify and implement the rule

1. Run the full `verify-reduction` workflow before writing Rust code. Use a
   standalone Typst proof and two independent Python implementations to verify
   at least 5,000 cases each, including exhaustive undirected simple graphs
   through five vertices, solution extraction from every feasible target
   witness, the stated overhead bounds, and positive/negative examples. Treat
   the verified construction and extraction as the implementation
   specification.
2. Confirm the source and target both use `Or`, then add
   `src/rules/hamiltonianpath_hamiltoniancircuit.rs` and register it in
   `src/rules/mod.rs`.
3. For source graphs with at least two vertices, copy every source edge, add a
   new vertex `x = n`, and add `{x, v}` for every old vertex. For empty and
   singleton sources, produce a fixed triangle so the target remains feasible.
   Store enough source-size state to extract a witness: return `[]` or `[0]`
   for the two small cases; otherwise rotate the target circuit so `x` is
   first, remove `x`, and preserve the remaining order.
4. Register the safe symbolic bounds `num_vertices = "num_vertices + 3"` and
   `num_edges = "num_edges + num_vertices + 3"`. Keep the primitive edge
   specific to the `SimpleGraph` variants.
5. Add focused tests at
   `src/unit_tests/rules/hamiltonianpath_hamiltoniancircuit.rs`: a positive
   closed loop, an infeasible graph, target structure and exact large-branch
   sizes, both target cycle orientations/rotations, empty and singleton source
   semantics, two isolated vertices, one edge, a disconnected graph, a star,
   and inputs containing self-loops and parallel edges. Avoid snapshot fixtures
   and redundant helper tests.
6. Add the issue's five-vertex worked example to the canonical rule example
   database using the repository's current per-rule example-spec pattern, and
   verify the target circuit and extracted source path end to end.

## Batch 2: paper and generated integration

1. With a fresh context, add a mandatory `reduction-rule` entry for
   `HamiltonianPath` to `HamiltonianCircuit` in
   `docs/paper/reductions.typ`. Cite Waggoner's course notes for the
   universal-vertex mapping and retain Garey--Johnson only for the classical
   problem definitions. Give a self-contained construction, both correctness
   directions, witness extraction, small-instance branch, overhead, and a
   tutorial-style worked example driven from canonical fixture data.
2. Add the Waggoner BibTeX record to `docs/paper/references.bib` if it is not
   already present. Regenerate the reduction graph, schemas, and example
   fixtures required by the paper, then run `make paper`.
3. Run formatting, tests, clippy, and coverage in accordance with repository
   requirements. Inspect generated changes, keep only files required by this
   rule, and ensure the final worktree is clean.

