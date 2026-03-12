# Plan: Add SubgraphIsomorphism Model (#218)

## Overview
Add SubgraphIsomorphism as a new satisfaction problem model. Given two graphs G (host) and H (pattern), determine whether G contains a subgraph isomorphic to H.

## Information Checklist
1. **Problem name:** `SubgraphIsomorphism`
2. **Definition:** Given G=(V1,E1) host graph and H=(V2,E2) pattern graph, does there exist an injective function f: V2 -> V1 such that {u,v} in E2 implies {f(u),f(v)} in E1?
3. **Problem type:** Satisfaction (bool)
4. **Type parameters:** None (both graphs are SimpleGraph)
5. **Struct fields:** `host_graph: SimpleGraph`, `pattern_graph: SimpleGraph`
6. **Configuration space:** `vec![host_graph.num_vertices(); pattern_graph.num_vertices()]` — each pattern vertex maps to a host vertex
7. **Feasibility check:** All variables distinct (injective) AND every pattern edge maps to a host edge
8. **Objective function:** bool — true if valid isomorphism
9. **Complexity:** O(|V1|^|V2| * |E2|) brute force; NP-complete (Cook 1971)
10. **Solving strategy:** BruteForce enumerate all mappings
11. **Category:** `graph/`

## Steps

### Step 1: Create model file
- `src/models/graph/subgraph_isomorphism.rs`
- Struct with `host_graph: SimpleGraph` and `pattern_graph: SimpleGraph`
- `Problem` trait impl with `Metric = bool`, `SatisfactionProblem` marker
- `dims()` returns `vec![n1; n2]` where n1 = host vertices, n2 = pattern vertices
- `evaluate()` checks injectivity + edge preservation
- Getters: `num_host_vertices()`, `num_host_edges()`, `num_pattern_vertices()`, `num_pattern_edges()`
- `declare_variants!` with complexity `"num_host_vertices ^ num_pattern_vertices"`
- `inventory::submit!` for ProblemSchemaEntry

### Step 2: Register model
- Add to `src/models/graph/mod.rs`
- Add to `src/models/mod.rs` re-exports
- Add to `src/lib.rs` prelude

### Step 3: CLI registration
- `problemreductions-cli/src/dispatch.rs`: add `load_problem` + `serialize_any_problem` arms
- `problemreductions-cli/src/problem_name.rs`: add `resolve_alias` entry
- `problemreductions-cli/src/commands/create.rs`: add creation handler with `--graph` (host) + `--pattern` (pattern) flags
- `problemreductions-cli/src/cli.rs`: add `--pattern` flag to CreateArgs, update help table, update `all_data_flags_empty`

### Step 4: Write unit tests
- `src/unit_tests/models/graph/subgraph_isomorphism.rs`
- Test creation, evaluation (valid/invalid), brute-force solver, serialization

### Step 5: Document in paper
- Add `problem-def("SubgraphIsomorphism")` to `docs/paper/reductions.typ`
- Add display-name entry

### Step 6: Verify
- `make test clippy`
