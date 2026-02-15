//! Graph type markers for parametric problem modeling.
//!
//! ZST marker structs for graph types used as type parameters in problem definitions.
//! The subtype hierarchy is managed via `VariantParam` trait implementations (see `src/variant.rs`).

/// Simple (arbitrary) graph - the most general graph type.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleGraph;

/// Unit disk graph - vertices are points, edges connect points within unit distance.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnitDiskGraph;

/// King's subgraph - a unit disk graph on a square grid with king's move connectivity.
#[derive(Debug, Clone, Copy, Default)]
pub struct KingsSubgraph;

/// Triangular subgraph - a unit disk graph on a triangular lattice.
#[derive(Debug, Clone, Copy, Default)]
pub struct TriangularSubgraph;

/// Hypergraph - most general graph type. Edges can connect any number of vertices.
#[derive(Debug, Clone, Copy, Default)]
pub struct HyperGraph;

#[cfg(test)]
#[path = "unit_tests/graph_types.rs"]
mod tests;
