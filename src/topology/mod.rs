//! Graph topology types.
//!
//! - [`SimpleGraph`]: Standard unweighted graph (default for most problems)
//! - [`PlanarGraph`]: Planar graph
//! - [`BipartiteGraph`]: Bipartite graph
//! - [`SimpleDigraph`]: Loop-free directed graph without parallel arcs
//! - [`MultiDigraph`]: Directed multigraph with loops and parallel arcs
//! - [`MultiMixedGraph`]: Mixed multigraph with loops and parallel arcs/edges
//! - [`UnitDiskGraph`]: Vertices with 2D positions, edges based on distance
//! - [`KingsSubgraph`]: 8-connected grid graph (King's graph)
//! - [`TriangularSubgraph`]: Triangular lattice subgraph

mod bipartite_graph;
mod graph;
mod kings_subgraph;
mod multi_digraph;
mod multi_mixed_graph;
mod path;
mod planar_graph;
mod simple_digraph;
pub mod small_graphs;
mod triangular_subgraph;
mod unit_disk_graph;

pub use bipartite_graph::BipartiteGraph;
pub(crate) use graph::validate_simple_connections;
pub use graph::{Graph, SimpleGraph};
pub use kings_subgraph::KingsSubgraph;
pub use multi_digraph::MultiDigraph;
pub use multi_mixed_graph::MultiMixedGraph;
pub(crate) use path::is_simple_st_path;
pub use planar_graph::PlanarGraph;
pub use simple_digraph::SimpleDigraph;
pub use small_graphs::{available_graphs, smallgraph};
pub use triangular_subgraph::TriangularSubgraph;
pub use unit_disk_graph::UnitDiskGraph;
