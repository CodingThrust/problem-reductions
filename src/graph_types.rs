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

/// Grid graph - vertices on a grid, edges to neighbors.
#[derive(Debug, Clone, Copy, Default)]
pub struct GridGraph;
impl GraphMarker for GridGraph {}

/// Hypergraph - most general graph type. Edges can connect any number of vertices.
#[derive(Debug, Clone, Copy, Default)]
pub struct HyperGraph;
impl GraphMarker for HyperGraph {}

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

// Corrected graph type hierarchy (all transitive relationships declared for compile-time bounds).
//   HyperGraph (most general)
//   └── SimpleGraph
//       ├── PlanarGraph
//       ├── BipartiteGraph
//       └── UnitDiskGraph
//           └── GridGraph
declare_graph_subtype!(GridGraph => UnitDiskGraph);
declare_graph_subtype!(GridGraph => SimpleGraph);
declare_graph_subtype!(GridGraph => HyperGraph);
declare_graph_subtype!(UnitDiskGraph => SimpleGraph);
declare_graph_subtype!(UnitDiskGraph => HyperGraph);
declare_graph_subtype!(PlanarGraph => SimpleGraph);
declare_graph_subtype!(PlanarGraph => HyperGraph);
declare_graph_subtype!(BipartiteGraph => SimpleGraph);
declare_graph_subtype!(BipartiteGraph => HyperGraph);
declare_graph_subtype!(SimpleGraph => HyperGraph);

/// Runtime registration of weight subtype relationships.
pub struct WeightSubtypeEntry {
    pub subtype: &'static str,
    pub supertype: &'static str,
}

inventory::collect!(WeightSubtypeEntry);

/// Macro to declare weight subtype relationships (runtime only).
#[macro_export]
macro_rules! declare_weight_subtype {
    ($sub:expr => $sup:expr) => {
        ::inventory::submit! {
            $crate::graph_types::WeightSubtypeEntry {
                subtype: $sub,
                supertype: $sup,
            }
        }
    };
}

// Weight type hierarchy (with transitive relationships):
//   Unweighted (most restrictive) => i32 => f64 (most general)
declare_weight_subtype!("Unweighted" => "i32");
declare_weight_subtype!("Unweighted" => "f64"); // transitive
declare_weight_subtype!("i32" => "f64");

#[cfg(test)]
#[path = "unit_tests/graph_types.rs"]
mod tests;
