//! Graph type markers for parametric problem modeling.

use inventory;

/// Marker trait for graph types.
pub trait GraphMarker: 'static + Clone + Send + Sync {}

/// Compile-time subtype relationship between graph types.
pub trait GraphSubtype<G: GraphMarker>: GraphMarker {}

// Reflexive: every type is a subtype of itself
impl<G: GraphMarker> GraphSubtype<G> for G {}

/// Simple (arbitrary) graph - the most general graph type.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleGraph;

impl GraphMarker for SimpleGraph {}

/// Planar graph - can be drawn on a plane without edge crossings.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlanarGraph;

impl GraphMarker for PlanarGraph {}

/// Unit disk graph - vertices are points, edges connect points within unit distance.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnitDiskGraph;

impl GraphMarker for UnitDiskGraph {}

/// Bipartite graph - vertices can be partitioned into two sets with edges only between sets.
#[derive(Debug, Clone, Copy, Default)]
pub struct BipartiteGraph;

impl GraphMarker for BipartiteGraph {}

/// Runtime registration of graph subtype relationships.
pub struct GraphSubtypeEntry {
    pub subtype: &'static str,
    pub supertype: &'static str,
}

inventory::collect!(GraphSubtypeEntry);

/// Macro to declare both compile-time trait and runtime registration.
#[macro_export]
macro_rules! declare_graph_subtype {
    ($sub:ty => $sup:ty) => {
        impl $crate::graph_types::GraphSubtype<$sup> for $sub {}

        ::inventory::submit! {
            $crate::graph_types::GraphSubtypeEntry {
                subtype: stringify!($sub),
                supertype: stringify!($sup),
            }
        }
    };
}

// Declare the graph type hierarchy.
// Note: All direct relationships must be declared explicitly for compile-time trait bounds.
// Transitive closure is only computed at runtime in build_graph_hierarchy().
declare_graph_subtype!(UnitDiskGraph => PlanarGraph);
declare_graph_subtype!(UnitDiskGraph => SimpleGraph); // Needed for compile-time GraphSubtype<SimpleGraph>
declare_graph_subtype!(PlanarGraph => SimpleGraph);
declare_graph_subtype!(BipartiteGraph => SimpleGraph);

#[cfg(test)]
#[path = "unit_tests/graph_types.rs"]
mod tests;
