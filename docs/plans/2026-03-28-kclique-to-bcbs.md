# Plan: KClique -> BalancedCompleteBipartiteSubgraph Reduction

Fixes #231

## Overview

Implement the classical reduction from KClique to BalancedCompleteBipartiteSubgraph (GT24, Johnson 1987). This is a satisfaction-to-satisfaction (Or -> Or) reduction using a non-incidence gadget construction.

## Reduction Algorithm

Given KClique instance (G=(V,E), k) with n=|V|, m=|E|:

1. Pad vertex set: add C(k,2) = k(k-1)/2 isolated vertices. n' = n + C(k,2).
2. Part A (left): n' vertices (padded vertex set).
3. Part B (right): m edge elements + (n-k) padding elements. |B| = m + n - k.
4. Bipartite edges (non-incidence + full padding):
   - For edge element e_j = {u,w}: connect v to e_j iff v is NOT an endpoint of e_j.
   - For padding element w_i: connect v to w_i always.
5. Target parameter K' = n' - k = n + C(k,2) - k.
6. Solution extraction: clique S = {v in V : v NOT in A'} (vertices not selected on left side). source_config[v] = 1 - target_config[v] for v in 0..n.

## Overhead

- left_size = num_vertices + k * (k - 1) / 2
- right_size = num_edges + num_vertices - k
- k = num_vertices + k * (k - 1) / 2 - k

## Batch 1: Implementation (add-rule Steps 1-4)

### Step 1: Implement reduction rule
- File: `src/rules/kclique_balancedcompletebipartitesubgraph.rs`
- ReductionResult struct: `ReductionKCliqueToBCBS` storing target, source n, and source k
- extract_solution: invert left-side selection for first n vertices
- #[reduction(overhead = {...})] macro with the overhead formulas above

### Step 2: Register in mod.rs
- Add `pub(crate) mod kclique_balancedcompletebipartitesubgraph;` to `src/rules/mod.rs`

### Step 3: Write unit tests
- File: `src/unit_tests/rules/kclique_balancedcompletebipartitesubgraph.rs`
- test_kclique_to_balancedcompletebipartitesubgraph_closed_loop (4 vertices, k=3, triangle+tail)
- test_kclique_to_bcbs_no_clique (bipartite graph, no triangle)
- test_kclique_to_bcbs_structure (verify left_size, right_size, target k, edge count)
- test_kclique_to_bcbs_complete_graph (K4, k=3)
- test_kclique_to_bcbs_extract_solution

### Step 4: Add canonical example to example_db
- Add builder in `src/example_db/rule_builders.rs`
- Use the issue example: 4 vertices, 4 edges, k=3
- Register in `canonical_rule_example_specs()` in `src/rules/mod.rs`

### Verify Batch 1
```bash
make test clippy
```

## Batch 2: Paper entry + exports (add-rule Steps 5-6)

### Step 5: Document in paper
- Add reduction-rule entry in `docs/paper/reductions.typ`
- Include worked example from the issue

### Step 6: Regenerate exports and verify
```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
make paper
```
