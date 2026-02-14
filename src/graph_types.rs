//! Graph type markers for parametric problem modeling.
//!
//! ZST marker structs for graph types used as type parameters in problem definitions.
//! The subtype hierarchy is managed via `VariantTypeEntry` registrations (see `src/variant.rs`).

/// Simple (arbitrary) graph - the most general graph type.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleGraph;

/// Planar graph - can be drawn on a plane without edge crossings.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlanarGraph;

impl crate::variant::VariantParam for PlanarGraph {
    const CATEGORY: &'static str = "graph";
    const VALUE: &'static str = "PlanarGraph";
    const PARENT_VALUE: Option<&'static str> = Some("SimpleGraph");
}
inventory::submit! {
    crate::variant::VariantTypeEntry {
        category: "graph",
        value: "PlanarGraph",
        parent: Some("SimpleGraph"),
    }
}

/// Unit disk graph - vertices are points, edges connect points within unit distance.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnitDiskGraph;

/// Bipartite graph - vertices can be partitioned into two sets with edges only between sets.
#[derive(Debug, Clone, Copy, Default)]
pub struct BipartiteGraph;

impl crate::variant::VariantParam for BipartiteGraph {
    const CATEGORY: &'static str = "graph";
    const VALUE: &'static str = "BipartiteGraph";
    const PARENT_VALUE: Option<&'static str> = Some("SimpleGraph");
}
inventory::submit! {
    crate::variant::VariantTypeEntry {
        category: "graph",
        value: "BipartiteGraph",
        parent: Some("SimpleGraph"),
    }
}

/// Grid graph - vertices on a grid, edges to neighbors.
#[derive(Debug, Clone, Copy, Default)]
pub struct GridGraph;

/// Triangular lattice graph - a unit disk graph on a triangular grid.
#[derive(Debug, Clone, Copy, Default)]
pub struct Triangular;

/// Hypergraph - most general graph type. Edges can connect any number of vertices.
#[derive(Debug, Clone, Copy, Default)]
pub struct HyperGraph;

#[cfg(test)]
#[path = "unit_tests/graph_types.rs"]
mod tests;
