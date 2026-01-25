# Graph Topology

This module provides the `Graph` trait and various graph implementations for defining computational problems on different graph structures.

## Design Philosophy

Following Julia's Graphs.jl pattern, problems are generic over graph type. This enables **topology-aware reductions**:

```rust
use problemreductions::topology::{Graph, SimpleGraph, UnitDiskGraph};
use problemreductions::models::graph::IndependentSetT;

// Standard problem on SimpleGraph (default)
let is_simple: IndependentSetT = IndependentSetT::new(4, vec![(0, 1), (1, 2)]);

// Same problem on UnitDiskGraph (for quantum hardware)
let udg = UnitDiskGraph::new(positions, radius);
let is_udg: IndependentSetT<UnitDiskGraph> = IndependentSetT::from_graph(udg);
```

Different reductions can target different topologies:

```rust
// Standard reduction -> arbitrary graph
impl ReduceTo<IndependentSetT<SimpleGraph>> for Satisfiability { ... }

// Topology-aware reduction -> unit disk graph (using geometric gadgets)
impl ReduceTo<IndependentSetT<UnitDiskGraph>> for Satisfiability { ... }
```

## The Graph Trait

All graph types implement the `Graph` trait:

```rust
pub trait Graph: Clone + Send + Sync {
    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn edges(&self) -> Vec<(usize, usize)>;
    fn has_edge(&self, u: usize, v: usize) -> bool;
    fn neighbors(&self, v: usize) -> Vec<usize>;
    fn degree(&self, v: usize) -> usize;  // default impl
    fn is_empty(&self) -> bool;            // default impl
}
```

## SimpleGraph

The default graph type for most problems. Wraps petgraph's `UnGraph` with convenient constructors:

```rust
use problemreductions::topology::SimpleGraph;

// Create from vertices and edges
let g = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// Factory methods
let complete = SimpleGraph::complete(4);    // K4
let path = SimpleGraph::path(5);            // 0-1-2-3-4
let cycle = SimpleGraph::cycle(4);          // 0-1-2-3-0
let star = SimpleGraph::star(5);            // 0 connected to 1,2,3,4
let grid = SimpleGraph::grid(3, 4);         // 3x4 grid graph
```

## UnitDiskGraph

Geometric graphs where vertices have 2D positions and edges connect vertices within a distance threshold. Useful for:

- Quantum computing (neutral atom arrays)
- Wireless network problems
- Geometric constraint satisfaction

```rust
use problemreductions::topology::UnitDiskGraph;

let positions = vec![
    (0.0, 0.0),
    (1.0, 0.0),   // Distance 1.0 from (0,0)
    (2.5, 0.0),   // Distance 2.5 from (0,0)
];

let udg = UnitDiskGraph::new(positions, 1.5);
// Edges: 0-1 (distance 1.0 <= 1.5)
// No edge 0-2 (distance 2.5 > 1.5)
// No edge 1-2 (distance 1.5 <= 1.5, so actually connected!)

// Grid factory
let grid_udg = UnitDiskGraph::grid(3, 4, 1.0, 1.5);
```

### Using with Problems

```rust
use problemreductions::topology::UnitDiskGraph;
use problemreductions::models::graph::IndependentSetT;

// Create UDG representing quantum hardware connectivity
let udg = UnitDiskGraph::new(atom_positions, interaction_radius);

// Define Independent Set problem on this topology
let problem: IndependentSetT<UnitDiskGraph> = IndependentSetT::from_graph(udg);

// Solve for maximum independent set
let solutions = solver.find_best(&problem);
```

## HyperGraph

Generalized graphs where edges (hyperedges) can connect any number of vertices. Note: HyperGraph does NOT implement the `Graph` trait since hyperedges are fundamentally different from 2-vertex edges.

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

// Check if it's actually a regular graph
if hg.is_regular_graph() {
    let edges = hg.to_graph_edges().unwrap();
}
```

## API Reference

### Graph Trait

```rust
pub trait Graph: Clone + Send + Sync {
    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn edges(&self) -> Vec<(usize, usize)>;
    fn has_edge(&self, u: usize, v: usize) -> bool;
    fn neighbors(&self, v: usize) -> Vec<usize>;
    fn degree(&self, v: usize) -> usize;
    fn is_empty(&self) -> bool;
    fn for_each_edge<F>(&self, f: F) where F: FnMut(usize, usize);
}
```

### SimpleGraph

```rust
impl SimpleGraph {
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self;
    pub fn empty(num_vertices: usize) -> Self;
    pub fn complete(num_vertices: usize) -> Self;
    pub fn path(num_vertices: usize) -> Self;
    pub fn cycle(num_vertices: usize) -> Self;
    pub fn star(num_vertices: usize) -> Self;
    pub fn grid(rows: usize, cols: usize) -> Self;
}
```

### UnitDiskGraph

```rust
impl UnitDiskGraph {
    pub fn new(positions: Vec<(f64, f64)>, radius: f64) -> Self;
    pub fn unit(positions: Vec<(f64, f64)>) -> Self;  // radius = 1.0
    pub fn grid(rows: usize, cols: usize, spacing: f64, radius: f64) -> Self;
    pub fn radius(&self) -> f64;
    pub fn position(&self, v: usize) -> Option<(f64, f64)>;
    pub fn positions(&self) -> &[(f64, f64)];
    pub fn vertex_distance(&self, u: usize, v: usize) -> Option<f64>;
    pub fn bounding_box(&self) -> Option<((f64, f64), (f64, f64))>;
}
```

### HyperGraph

```rust
impl HyperGraph {
    pub fn new(num_vertices: usize, edges: Vec<Vec<usize>>) -> Self;
    pub fn empty(num_vertices: usize) -> Self;
    pub fn num_vertices(&self) -> usize;
    pub fn num_edges(&self) -> usize;
    pub fn edges(&self) -> &[Vec<usize>];
    pub fn edge(&self, index: usize) -> Option<&Vec<usize>>;
    pub fn has_edge(&self, edge: &[usize]) -> bool;
    pub fn neighbors(&self, v: usize) -> Vec<usize>;
    pub fn degree(&self, v: usize) -> usize;
    pub fn edges_containing(&self, v: usize) -> Vec<&Vec<usize>>;
    pub fn max_edge_size(&self) -> usize;
    pub fn is_regular_graph(&self) -> bool;
    pub fn to_graph_edges(&self) -> Option<Vec<(usize, usize)>>;
}
```
