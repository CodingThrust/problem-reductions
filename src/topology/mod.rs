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
//! ```rust,ignore
//! // Problems work with any graph type
//! pub struct IndependentSet<G: Graph = SimpleGraph, W = i32> {
//!     graph: G,
//!     weights: Vec<W>,
//! }
//!
//! // Reductions can target specific topologies
//! impl ReduceTo<IndependentSet<SimpleGraph>> for SAT { ... }
//! impl ReduceTo<IndependentSet<UnitDiskGraph>> for SAT { ... }  // Different gadgets!
//! ```

mod graph;
mod hypergraph;
mod unit_disk_graph;

pub use graph::{Graph, SimpleGraph};
pub use hypergraph::HyperGraph;
pub use unit_disk_graph::UnitDiskGraph;
