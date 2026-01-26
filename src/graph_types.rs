//! Graph type markers for parametric problem modeling.

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
}
