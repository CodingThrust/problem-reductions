# Topology Types

## HyperGraph

Generalized graphs with edges connecting multiple vertices.

```rust
use problemreductions::topology::HyperGraph;

let hg = HyperGraph::new(5, vec![
    vec![0, 1, 2],  // Hyperedge connecting 3 vertices
    vec![2, 3, 4],  // Another hyperedge
]);

assert_eq!(hg.num_vertices(), 5);
assert_eq!(hg.num_edges(), 2);

// Get neighbors (vertices sharing a hyperedge)
let neighbors = hg.neighbors(2);  // [0, 1, 3, 4]
```

## UnitDiskGraph

Geometric graphs where edges connect points within a radius.

```rust
use problemreductions::topology::UnitDiskGraph;

let positions = vec![
    (0.0, 0.0),
    (0.5, 0.0),   // Within radius of (0,0)
    (2.0, 0.0),   // Too far from (0,0)
];

let udg = UnitDiskGraph::new(positions, 1.0);

// Get edges as (u, v) pairs
let edges = udg.edges();

// Use with IndependentSet
use problemreductions::prelude::*;
let is = IndependentSet::<i32>::new(3, edges.to_vec());
```

## API Reference

### HyperGraph

```rust
impl HyperGraph {
    pub fn new(num_vertices: usize, edges: Vec<Vec<usize>>) -> Self;
    pub fn num_vertices(&self) -> usize;
    pub fn num_edges(&self) -> usize;
    pub fn neighbors(&self, v: usize) -> Vec<usize>;
    pub fn degree(&self, v: usize) -> usize;
}
```

### UnitDiskGraph

```rust
impl UnitDiskGraph {
    pub fn new(positions: Vec<(f64, f64)>, radius: f64) -> Self;
    pub fn edges(&self) -> &[(usize, usize)];
    pub fn num_vertices(&self) -> usize;
    pub fn neighbors(&self, v: usize) -> Vec<usize>;
}
```
