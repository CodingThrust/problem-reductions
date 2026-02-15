use super::*;
use crate::variant::VariantParam;

#[test]
fn test_graph_type_traits() {
    // Test Default
    let _: SimpleGraph = Default::default();
    let _: UnitDiskGraph = Default::default();
    let _: KingsSubgraph = Default::default();
    let _: TriangularSubgraph = Default::default();
    let _: HyperGraph = Default::default();

    // Test Copy (SimpleGraph implements Copy, so no need to clone)
    let g = SimpleGraph;
    let _g2 = g; // Copy
    let g = SimpleGraph;
    let _g2 = g;
    let _g3 = g; // still usable
}

#[test]
fn test_planargraph_variant_param() {
    use crate::topology::PlanarGraph;
    assert_eq!(PlanarGraph::CATEGORY, "graph");
    assert_eq!(PlanarGraph::VALUE, "PlanarGraph");
    assert_eq!(PlanarGraph::PARENT_VALUE, Some("SimpleGraph"));
}

#[test]
fn test_bipartitegraph_variant_param() {
    use crate::topology::BipartiteGraph;
    assert_eq!(BipartiteGraph::CATEGORY, "graph");
    assert_eq!(BipartiteGraph::VALUE, "BipartiteGraph");
    assert_eq!(BipartiteGraph::PARENT_VALUE, Some("SimpleGraph"));
}

#[test]
fn test_marker_structs_exist() {
    // Verify that all ZST marker structs still exist and can be instantiated
    let _ = SimpleGraph;
    let _ = UnitDiskGraph;
    let _ = KingsSubgraph;
    let _ = TriangularSubgraph;
    let _ = HyperGraph;
}
