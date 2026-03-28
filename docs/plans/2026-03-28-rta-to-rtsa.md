# Plan: RootedTreeArrangement -> RootedTreeStorageAssignment reduction

**Issue:** #424
**Skill:** add-rule

## Summary

Implement the reduction from RootedTreeArrangement to RootedTreeStorageAssignment, establishing the connection described by Gavril (1977) and cataloged as SR5 in Garey & Johnson.

## Reduction Algorithm

Given RTA instance: graph G=(V,E), bound K:
1. Universe X = V (the vertex set), so universe_size = |V|
2. For each edge {u,v} in E, create subset {u,v}. Collection C has |E| subsets, each of size 2.
3. Bound K' = K - |E|

### Solution Extraction

RTSA config = parent array defining a rooted tree on X=V.
RTA config = [parent_array, identity_mapping] since the embedding f is the identity f(v)=v.

### Correctness

- Forward: if arrangement has total stretch <= K, each edge {u,v} lies on a root-to-leaf path with d(u,v) intermediate nodes. Extension cost for {u,v} = d(u,v) - 1. Total extension = sum(d(u,v)) - |E| <= K - |E| = K'.
- Reverse: if storage assignment has total extension <= K', total arrangement distance = extension + |E| <= K' + |E| = K.

### Overhead

- universe_size = num_vertices
- num_subsets = num_edges

## Batch 1: Implementation (Steps 1-4)

### Step 1: Create reduction file

File: `src/rules/rootedtreearrangement_rootedtreestorageassignment.rs`

- ReductionRTAToRTSA struct holding target + source graph edges (needed for extract_solution)
- ReductionResult impl: extract_solution converts RTSA parent array to RTA [parent, identity] config
- ReduceTo impl with overhead = { universe_size = "num_vertices", num_subsets = "num_edges" }
- reduce_to: construct subsets from edges, compute K' = K - |E|

### Step 2: Register in mod.rs

Add `pub(crate) mod rootedtreearrangement_rootedtreestorageassignment;` to `src/rules/mod.rs`.

### Step 3: Write unit tests

File: `src/unit_tests/rules/rootedtreearrangement_rootedtreestorageassignment.rs`

Tests:
1. `test_rootedtreearrangement_to_rootedtreestorageassignment_closed_loop` - path graph P4
2. `test_rootedtreearrangement_to_rootedtreestorageassignment_structure` - verify target dimensions
3. `test_rootedtreearrangement_to_rootedtreestorageassignment_star_graph` - star graph with tight bound
4. `test_rootedtreearrangement_to_rootedtreestorageassignment_no_solution` - infeasible case

### Step 4: Add canonical example to example_db

Add builder function in `src/example_db/rule_builders.rs`.
Use the issue example: 6 vertices, 7 edges, K=9, K'=2.

## Batch 2: Paper entry (Step 5) + exports (Step 6)

### Step 5: Document in paper

Add reduction-rule entry in `docs/paper/reductions.typ` for RootedTreeArrangement -> RootedTreeStorageAssignment.

### Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
```
