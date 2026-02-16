//! Graph topology types.
//!
//! - [`SimpleGraph`]: Standard unweighted graph (default for most problems)
//! - [`HyperGraph`]: Edges can connect any number of vertices
//! - [`PlanarGraph`]: Planar graph
//! - [`BipartiteGraph`]: Bipartite graph
//! - [`UnitDiskGraph`]: Vertices with 2D positions, edges based on distance
//! - [`KingsSubgraph`]: 8-connected grid graph (King's graph)
//! - [`TriangularSubgraph`]: Triangular lattice subgraph

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
