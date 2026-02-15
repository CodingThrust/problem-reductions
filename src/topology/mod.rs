//! Graph topology types.
//!
//! This module provides the [`Graph`] trait and various graph implementations:
//!
//! - [`SimpleGraph`]: Standard unweighted graph (default for most problems)
//! - [`UnitDiskGraph`]: Vertices with 2D positions, edges based on distance
//! - [`HyperGraph`]: Edges can connect any number of vertices
//!
//! # Design Philosophy
//!
//! Following Julia's Graphs.jl pattern, problems are generic over graph type:
//!
//! ```
//! use problemreductions::topology::{Graph, SimpleGraph, UnitDiskGraph};
//! use problemreductions::models::graph::MaximumIndependentSet;
//!
//! // Problems work with any graph type - SimpleGraph by default
//! let simple_graph_problem: MaximumIndependentSet<SimpleGraph, i32> = MaximumIndependentSet::new(3, vec![(0, 1)]);
//! assert_eq!(simple_graph_problem.graph().num_vertices(), 3);
//!
//! // Different graph topologies enable different reduction algorithms
//! // (UnitDiskGraph example would require specific constructors)
//! ```

mod bipartite_graph;
mod graph;
mod hypergraph;
mod kings_subgraph;
mod planar_graph;
pub mod small_graphs;
mod triangular_subgraph;
mod unit_disk_graph;

pub use bipartite_graph::BipartiteGraph;
pub use graph::{Graph, GraphCast, SimpleGraph};
pub use hypergraph::HyperGraph;
pub use kings_subgraph::KingsSubgraph;
pub use planar_graph::PlanarGraph;
pub use small_graphs::{available_graphs, smallgraph};
pub use triangular_subgraph::TriangularSubgraph;
pub use unit_disk_graph::UnitDiskGraph;
