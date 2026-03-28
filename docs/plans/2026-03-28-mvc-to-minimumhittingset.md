# Plan: MinimumVertexCover -> MinimumHittingSet Reduction

**Issue:** #200
**Skill:** add-rule

## Summary

Implement a reduction from `MinimumVertexCover<SimpleGraph, One>` (unit-weight) to `MinimumHittingSet`. Each vertex maps to a universe element, each edge maps to a 2-element subset. This connects the currently-orphaned MinimumHittingSet to the reduction graph.

**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1, p.64

## Prerequisite: Add MinimumVertexCover<SimpleGraph, One> variant

The existing `declare_variants!` only registers `MinimumVertexCover<SimpleGraph, i32>`. We need to add a `MinimumVertexCover<SimpleGraph, One>` variant for the unit-weight case. Complexity: same `1.1996^num_vertices`.

## Batch 1: Implementation (add-rule Steps 1-4)

### Step 1: Add `MinimumVertexCover<SimpleGraph, One>` variant

In `src/models/graph/minimum_vertex_cover.rs`, add to `declare_variants!`:
```rust
crate::declare_variants! {
    default MinimumVertexCover<SimpleGraph, i32> => "1.1996^num_vertices",
    MinimumVertexCover<SimpleGraph, One> => "1.1996^num_vertices",
}
```

### Step 2: Implement the reduction rule

Create `src/rules/minimumvertexcover_minimumhittingset.rs`:

- **Source:** `MinimumVertexCover<SimpleGraph, One>`
- **Target:** `MinimumHittingSet`
- **Algorithm:** Universe = vertex set (size n), collection = edge set as 2-element subsets (m subsets)
- **Solution extraction:** Identity mapping (hitting set elements = vertex cover vertices)
- **Overhead:** `universe_size = "num_vertices"`, `num_sets = "num_edges"`

### Step 3: Register in mod.rs

Add `pub(crate) mod minimumvertexcover_minimumhittingset;` to `src/rules/mod.rs`.

### Step 4: Write unit tests

Create `src/unit_tests/rules/minimumvertexcover_minimumhittingset.rs`:

- `test_minimumvertexcover_to_minimumhittingset_closed_loop`: Build MVC<SimpleGraph, One> on a small graph, reduce, solve target with BruteForce, extract solution, verify valid vertex cover with optimal value.
- `test_reduction_structure`: Verify target universe_size = num_vertices, num_sets = num_edges, each set has exactly 2 elements.
- `test_empty_graph`: Edge case with no edges.
- `test_single_edge`: Minimal non-trivial case.

### Step 5: Add canonical example to example_db

Add builder function in the rule file's `canonical_rule_example_specs()`. Use the issue's example: 6 vertices, 8 edges, optimal VC = {0,3,4} (size 3). Register in `src/rules/mod.rs` `canonical_rule_example_specs()`.

## Batch 2: Paper entry (add-rule Step 5) and exports (Step 6)

### Step 6: Document in paper

Add `reduction-rule("MinimumVertexCover", "MinimumHittingSet", ...)` entry in `docs/paper/reductions.typ`:
- Rule statement: restriction proof from Garey & Johnson
- Proof: construction, correctness, solution extraction
- Worked example with JSON data from fixture

### Step 7: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
make paper
```
