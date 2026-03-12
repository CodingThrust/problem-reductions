# Plan: Add RuralPostman Model (#248)

## Overview

Add the Rural Postman Problem (RPP) as a satisfaction problem. Given a graph G=(V,E) with edge lengths, a subset E' of required edges, and a bound B, determine if there exists a circuit covering all required edges with total length at most B.

This is a **satisfaction problem** (Metric = bool, implements SatisfactionProblem).

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `RuralPostman` |
| 2 | Mathematical definition | Given G=(V,E), lengths l(e), required edges E'⊆E, bound B, find circuit covering E' with length ≤ B |
| 3 | Problem type | Satisfaction (decision) |
| 4 | Type parameters | `G: Graph`, `W: WeightElement` |
| 5 | Struct fields | `graph: G`, `edge_lengths: Vec<W>`, `required_edges: Vec<usize>`, `bound: W::Sum` |
| 6 | Configuration space | `vec![2; num_edges]` — binary per edge (selected or not) |
| 7 | Feasibility check | Selected edges form a closed walk covering all required edges with total length ≤ bound |
| 8 | Objective function | N/A (satisfaction: returns bool) |
| 9 | Best known exact algorithm | O(num_vertices^2 * 2^num_required_edges) DP over required edge subsets |
| 10 | Solving strategy | BruteForce (enumerate edge subsets, check feasibility) |
| 11 | Category | `graph` |

## Implementation Steps

### Step 1: Create model file
- File: `src/models/graph/rural_postman.rs`
- Struct `RuralPostman<G, W>` with fields: graph, edge_lengths, required_edges, bound
- Constructor, accessors, size getters (num_vertices, num_edges, num_required_edges)
- Problem trait impl with Metric = bool
- SatisfactionProblem impl
- declare_variants! with complexity `num_vertices^2 * 2^num_required_edges`
- The evaluate function checks: config selects edges forming a closed walk that covers all required edges with total length ≤ bound
- Note: Since brute force enumerates binary edge configs (include/exclude each edge), the circuit is the multiset of selected edges. We need to verify the selected edges form an Eulerian subgraph (all vertices have even degree) that is connected (considering only vertices with degree > 0) and covers all required edges.

### Step 2: Register the model
- Add `pub(crate) mod rural_postman;` and `pub use rural_postman::RuralPostman;` to `src/models/graph/mod.rs`
- Add `RuralPostman` to re-export in `src/models/mod.rs`
- Add `RuralPostman` to prelude in `src/lib.rs`

### Step 3: Register in CLI dispatch
- Add `"RuralPostman"` arm in `load_problem()` using `deser_sat::<RuralPostman<SimpleGraph, i32>>`
- Add `"RuralPostman"` arm in `serialize_any_problem()` using `try_ser::<RuralPostman<SimpleGraph, i32>>`

### Step 4: Register CLI alias
- Add `"ruralpostman" | "rpp"` mapping in `resolve_alias()` in `problem_name.rs`
- Add `("RPP", "RuralPostman")` to ALIASES array (RPP is a well-established abbreviation)

### Step 5: Add CLI create support
- Add `"RuralPostman"` match arm in `commands/create.rs`
- Parse `--graph`, `--edge-weights`, `--required-edges` (new flag), `--bound` (new flag)
- Add `required_edges` and `bound` fields to `CreateArgs` in `cli.rs`
- Update `all_data_flags_empty()` to include new flags
- Update help table

### Step 6: Write unit tests
- File: `src/unit_tests/models/graph/rural_postman.rs`
- Test creation and dimensions
- Test evaluate on valid circuit (YES instance from issue)
- Test evaluate on infeasible instance (NO instance)
- Test Chinese Postman special case (E'=E)
- Test brute force solver finds satisfying solution
- Test serialization round-trip

### Step 7: Write paper entry
- Add `"RuralPostman": [Rural Postman],` to display-name dict
- Add problem-def entry with formal definition

### Step 8: Verify
- `make test clippy`
