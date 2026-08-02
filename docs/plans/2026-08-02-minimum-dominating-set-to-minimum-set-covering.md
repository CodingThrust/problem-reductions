# Minimum Dominating Set to Minimum Set Covering

Issue: #1095

Base: `a9067297`

Implement the witness-preserving reduction
`MinimumDominatingSet<SimpleGraph, i32> -> MinimumSetCovering<i32>` by following
the repository's `add-rule` skill.

## Batch 1: Verify and implement

1. Follow `verify-reduction` before writing Rust. Prove that selecting vertices
   is equivalent to selecting their closed-neighborhood sets, verify the exact
   overhead, and independently test at least 5,000 instances/checks in both the
   constructor and adversary implementations. Include the weighted path
   `0-1-2-3-4`, an empty graph, isolated vertices, signed weights, self-loops,
   and repeated edges.
2. Add `src/rules/minimumdominatingset_minimumsetcovering.rs`. Construct universe
   `V`, one deduplicated set `N[v]` per vertex, and copy vertex weights. Extract
   the source witness coordinate-for-coordinate. Register exact overhead
   `universe_size = num_vertices` and `num_sets = num_vertices`.
3. Register the module directly in `src/rules/mod.rs`.
4. Add focused tests in
   `src/unit_tests/rules/minimumdominatingset_minimumsetcovering.rs` for the
   canonical weighted closed loop, exact target structure, signed-weight
   optimality/extraction, empty and isolated graphs, and deduplication of
   self-loops/repeated edges.
5. Add the canonical five-vertex weighted path to the rule's example-db specs,
   using source witness `[0, 1, 0, 1, 0]` and the identical target witness.
6. Run focused tests, formatting, and clippy before handing off to Batch 2.

## Batch 2: Paper, fixtures, and final verification

1. Add a self-contained `reduction-rule("MinimumDominatingSet",
   "MinimumSetCovering", ...)` entry to `docs/paper/reductions.typ`, adapting
   the verified construction and proof rather than rewriting it. Include the
   canonical example via loaded fixture data and a `pred-commands()` block.
2. Cite Garey--Johnson for the problem definitions and the UMass COMPSCI 311
   solution for the explicit closed-neighborhood reduction.
3. Regenerate the reduction graph, schemas, and example fixtures. Stage only
   tracked artifacts required by this rule.
4. Run `make paper`, `make test`, `make clippy`, and the relevant coverage check.
   Confirm the tree is clean except for the temporary plan file, which the
   pipeline removes after implementation.
