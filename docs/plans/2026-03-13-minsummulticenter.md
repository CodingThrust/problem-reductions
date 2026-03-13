# Plan: Add MinSumMulticenter Model (#399)

## Overview

Add the MinSumMulticenter (p-median) problem model — a facility location optimization problem that minimizes total weighted distance from vertices to K selected centers.

**Design decision:** Implement as an **optimization problem** (Metric = SolutionSize<W::Sum>, Direction::Minimize), consistent with how MinimumDominatingSet, MinimumVertexCover, etc. are implemented despite GJ defining them as decision problems. The bound B is not stored; brute force finds optimal solutions directly.

## Step 1: Create model file `src/models/graph/min_sum_multicenter.rs`

### Struct
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinSumMulticenter<G, W> {
    graph: G,
    vertex_weights: Vec<W>,
    edge_lengths: Vec<W>,
    k: usize,
}
```

### Constructor & Accessors
- `new(graph, vertex_weights, edge_lengths, k)` — assert vertex_weights.len() == num_vertices, edge_lengths.len() == num_edges, k <= num_vertices, k > 0
- `graph()`, `vertex_weights()`, `edge_lengths()`, `k()` getters
- `num_vertices()`, `num_edges()`, `num_centers()` size getters (in WeightElement-bounded impl block)

### Shortest Path Computation
Implement a private method `shortest_distances_to_centers(&self, config: &[usize]) -> Option<Vec<W::Sum>>`:
- Multi-source BFS-like approach using a priority mechanism
- Build adjacency list with edge lengths from `graph.edges()` and `edge_lengths`
- Use a simple Dijkstra-like algorithm from all selected centers simultaneously
- Returns None if any vertex is unreachable (disconnected graph)
- Returns Some(distances) otherwise

For the evaluate function:
1. Count selected centers; if != k, return Invalid
2. Compute shortest distances from each vertex to nearest center
3. If any vertex unreachable, return Invalid
4. Compute Σ vertex_weights[v].to_sum() * distances[v] for all v
5. Return SolutionSize::Valid(total)

### Problem Trait
```rust
const NAME: &'static str = "MinSumMulticenter";
type Metric = SolutionSize<W::Sum>;
fn dims(&self) -> Vec<usize> { vec![2; num_vertices] }
fn variant() -> ... { variant_params![G, W] }
```

### OptimizationProblem
- Direction::Minimize
- type Value = W::Sum

### declare_variants!
```rust
MinSumMulticenter<SimpleGraph, i32> => "2^num_vertices",
```
Note: No specialized exact algorithm improves on brute-force C(n,K) enumeration for general p-median.

### Schema Registration
Register with `inventory::submit!` including fields: graph, vertex_weights, edge_lengths, k.

## Step 2: Register module

- `src/models/graph/mod.rs` — add `pub(crate) mod min_sum_multicenter;` and `pub use min_sum_multicenter::MinSumMulticenter;`
- `src/models/mod.rs` — add `MinSumMulticenter` to graph re-exports
- `src/lib.rs` — add to prelude

## Step 3: Create unit tests `src/unit_tests/models/graph/min_sum_multicenter.rs`

Link via `#[path]` in model file. Tests:
- `test_min_sum_multicenter_creation` — basic construction, getters
- `test_min_sum_multicenter_evaluate` — manual evaluation of known configs
- `test_min_sum_multicenter_invalid_k` — wrong number of centers returns Invalid
- `test_min_sum_multicenter_solver` — BruteForce finds optimal for small instance
- `test_min_sum_multicenter_disconnected` — unreachable vertex returns Invalid
- `test_min_sum_multicenter_direction` — Direction::Minimize
- `test_min_sum_multicenter_size_getters` — num_vertices, num_edges, num_centers

Use the example from the issue: 7 vertices, 8 edges, unit weights/lengths, K=2. Optimal centers at {2, 5} with total cost 6.

## Step 4: Register in CLI `problemreductions-cli/src/commands/create.rs`

Add custom handler for "MinSumMulticenter" that accepts:
- `--graph` / `--edges` (graph topology)
- `--vertex-weights` (vertex weights)
- `--edge-lengths` (edge lengths)
- `--k` (number of centers)

Add to `example_for()` and the main match dispatch.

## Step 5: Add unit test mod.rs registration

- `src/unit_tests/models/graph/mod.rs` — add `mod min_sum_multicenter;`

## Step 6: Verify

- `make fmt && make clippy && make test`
- Verify the example instance gives expected results
