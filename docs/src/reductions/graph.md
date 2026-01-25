# Reduction Graph

The `ReductionGraph` allows discovering reduction paths between problem types.

## Usage

```rust
use problemreductions::prelude::*;
use problemreductions::rules::ReductionGraph;

let graph = ReductionGraph::new();

// Check if direct reduction exists
let has_direct = graph.has_direct_reduction::<IndependentSet<i32>, VertexCovering<i32>>();

// Find all paths between types
let paths = graph.find_paths::<SetPacking<i32>, VertexCovering<i32>>();

// Find shortest path
let shortest = graph.find_shortest_path::<SetPacking<i32>, VertexCovering<i32>>();
```

## Current Reduction Graph

```
IndependentSet<i32> <---> VertexCovering<i32>
       |
       +--------> SetPacking<i32>

SpinGlass<f64> <---> QUBO<f64>

SpinGlass<i32> <---> MaxCut<i32>
```

## API

```rust
impl ReductionGraph {
    pub fn new() -> Self;
    pub fn has_direct_reduction<S: 'static, T: 'static>(&self) -> bool;
    pub fn find_paths<S: 'static, T: 'static>(&self) -> Vec<ReductionPath>;
    pub fn find_shortest_path<S: 'static, T: 'static>(&self) -> Option<ReductionPath>;
    pub fn problem_types(&self) -> Vec<&'static str>;
    pub fn num_types(&self) -> usize;
    pub fn num_reductions(&self) -> usize;
}
```
