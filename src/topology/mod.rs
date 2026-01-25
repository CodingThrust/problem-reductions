//! Graph topology types.
//!
//! This module provides specialized graph representations beyond standard graphs:
//! - `HyperGraph`: Edges can connect any number of vertices
//! - `UnitDiskGraph`: Vertices with positions, edges based on distance

mod hypergraph;
mod unit_disk_graph;

pub use hypergraph::HyperGraph;
pub use unit_disk_graph::UnitDiskGraph;
