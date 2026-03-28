# Plan: KColoring(K3) -> TwoDimensionalConsecutiveSets

Fixes #437

## Overview

Add a reduction from `KColoring<K3, SimpleGraph>` to `TwoDimensionalConsecutiveSets`.
This is the Garey & Johnson SR19 / Lipsky 1977 reduction that proves NP-completeness
of 2-Dimensional Consecutive Sets via Graph 3-Colorability.

## Reduction Algorithm

Given graph G = (V, E) with |V| = n, |E| = m:

1. **Alphabet:** Sigma = V union {d_e : e in E}, size n + m.
   - Vertex symbols: 0..n-1 (same indices as vertices)
   - Edge dummy symbols: n..n+m-1 (one per edge)

2. **Subsets:** For each edge e_i = {u, v}, define subset S_i = {u, v, n + i}.
   - Each subset has size 3.
   - Total: m subsets.

3. **Solution extraction:** A valid partition into k=3 groups maps to a coloring:
   - For each vertex v, its group index is its color.
   - The consecutiveness + intersection constraints ensure adjacent vertices get different colors.

## Overhead

- `alphabet_size = "num_vertices + num_edges"`
- `num_subsets = "num_edges"`

## Batch 1: Implementation (add-rule Steps 1-4, 6)

### Step 1: Implement reduction rule

File: `src/rules/kcoloring_twodimensionalconsecutivesets.rs`

- `ReductionKColoringToTwoDCS` struct holding target + num_vertices
- `ReductionResult` impl with `extract_solution` mapping group assignment -> coloring
- `#[reduction(overhead = { alphabet_size = "num_vertices + num_edges", num_subsets = "num_edges" })]`
- `ReduceTo<TwoDimensionalConsecutiveSets> for KColoring<K3, SimpleGraph>`

### Step 2: Register in mod.rs

Add `pub(crate) mod kcoloring_twodimensionalconsecutivesets;` to `src/rules/mod.rs`.

### Step 3: Write unit tests

File: `src/unit_tests/rules/kcoloring_twodimensionalconsecutivesets.rs`

Tests:
- `test_kcoloring_to_twodimensionalconsecutivesets_closed_loop` - triangle graph (K3)
- `test_reduction_structure` - verify alphabet_size and num_subsets
- `test_non_3colorable_graph` - K4 should have no valid partition
- `test_empty_graph` - graph with vertices but no edges
- `test_bipartite_graph` - 2-colorable graph (also 3-colorable)

### Step 4: Add canonical example to example_db

Add builder in `src/example_db/rule_builders.rs`:
- Source: KColoring<K3, SimpleGraph> with a small graph (e.g., triangle + path)
- Pre-computed solution pair

### Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
```

## Batch 2: Paper entry (add-rule Step 5)

### Step 5: Document in paper

Add `reduction-rule("KColoring", "TwoDimensionalConsecutiveSets", ...)` entry in
`docs/paper/reductions.typ` with:
- Rule statement with O(n+m) complexity and Lipski 1977 citation
- Proof: Construction, Correctness, Solution extraction
- Worked example from canonical fixture data

```bash
make paper
```
