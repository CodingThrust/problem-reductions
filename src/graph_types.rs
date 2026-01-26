//! Graph type markers for parametric problem modeling.

use inventory;

/// Marker trait for graph types.
pub trait GraphMarker: 'static + Clone + Send + Sync {
    /// The name of this graph type for runtime queries.
    const NAME: &'static str;
}

/// Compile-time subtype relationship between graph types.
pub trait GraphSubtype<G: GraphMarker>: GraphMarker {}

// Reflexive: every type is a subtype of itself
impl<G: GraphMarker> GraphSubtype<G> for G {}

/// Simple (arbitrary) graph - the most general graph type.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleGraph;

impl GraphMarker for SimpleGraph {
    const NAME: &'static str = "SimpleGraph";
}

/// Planar graph - can be drawn on a plane without edge crossings.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlanarGraph;

impl GraphMarker for PlanarGraph {
    const NAME: &'static str = "PlanarGraph";
}

/// Unit disk graph - vertices are points, edges connect points within unit distance.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnitDiskGraph;

impl GraphMarker for UnitDiskGraph {
    const NAME: &'static str = "UnitDiskGraph";
}

/// Bipartite graph - vertices can be partitioned into two sets with edges only between sets.
#[derive(Debug, Clone, Copy, Default)]
pub struct BipartiteGraph;

impl GraphMarker for BipartiteGraph {
    const NAME: &'static str = "BipartiteGraph";
}

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
                subtype: <$sub as $crate::graph_types::GraphMarker>::NAME,
                supertype: <$sup as $crate::graph_types::GraphMarker>::NAME,
            }
        }
    };
}

// Declare the graph type hierarchy.
// Note: All direct relationships must be declared explicitly for compile-time trait bounds.
// Transitive closure is only computed at runtime in build_graph_hierarchy().
declare_graph_subtype!(UnitDiskGraph => PlanarGraph);
declare_graph_subtype!(UnitDiskGraph => SimpleGraph);  // Needed for compile-time GraphSubtype<SimpleGraph>
declare_graph_subtype!(PlanarGraph => SimpleGraph);
declare_graph_subtype!(BipartiteGraph => SimpleGraph);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_marker_names() {
        assert_eq!(SimpleGraph::NAME, "SimpleGraph");
        assert_eq!(PlanarGraph::NAME, "PlanarGraph");
        assert_eq!(UnitDiskGraph::NAME, "UnitDiskGraph");
        assert_eq!(BipartiteGraph::NAME, "BipartiteGraph");
    }

    #[test]
    fn test_reflexive_subtype() {
        fn assert_subtype<A: GraphSubtype<B>, B: GraphMarker>() {}

        // Every type is a subtype of itself
        assert_subtype::<SimpleGraph, SimpleGraph>();
        assert_subtype::<PlanarGraph, PlanarGraph>();
        assert_subtype::<UnitDiskGraph, UnitDiskGraph>();
    }

    #[test]
    fn test_subtype_entries_registered() {
        let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();

        // Should have at least 4 entries
        assert!(entries.len() >= 4);

        // Check specific relationships
        assert!(entries.iter().any(|e|
            e.subtype == "UnitDiskGraph" && e.supertype == "SimpleGraph"
        ));
        assert!(entries.iter().any(|e|
            e.subtype == "PlanarGraph" && e.supertype == "SimpleGraph"
        ));
    }

    #[test]
    fn test_declared_subtypes() {
        fn assert_subtype<A: GraphSubtype<B>, B: GraphMarker>() {}

        // Declared relationships
        assert_subtype::<UnitDiskGraph, PlanarGraph>();
        assert_subtype::<UnitDiskGraph, SimpleGraph>();
        assert_subtype::<PlanarGraph, SimpleGraph>();
        assert_subtype::<BipartiteGraph, SimpleGraph>();
    }

    #[test]
    fn test_graph_type_traits() {
        // Test Default
        let _: SimpleGraph = Default::default();
        let _: PlanarGraph = Default::default();
        let _: UnitDiskGraph = Default::default();
        let _: BipartiteGraph = Default::default();

        // Test Copy (SimpleGraph implements Copy, so no need to clone)
        let g = SimpleGraph;
        let _g2 = g;  // Copy
        let g = SimpleGraph;
        let _g2 = g;
        let _g3 = g; // still usable
    }

    #[test]
    fn test_bipartite_entry_registered() {
        let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();
        assert!(entries
            .iter()
            .any(|e| e.subtype == "BipartiteGraph" && e.supertype == "SimpleGraph"));
    }

    #[test]
    fn test_unit_disk_to_planar_registered() {
        let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();
        assert!(entries
            .iter()
            .any(|e| e.subtype == "UnitDiskGraph" && e.supertype == "PlanarGraph"));
    }
}
