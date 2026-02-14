use super::*;
use crate::variant::{VariantParam, VariantTypeEntry};

#[test]
fn test_graph_type_traits() {
    // Test Default
    let _: SimpleGraph = Default::default();
    let _: PlanarGraph = Default::default();
    let _: UnitDiskGraph = Default::default();
    let _: BipartiteGraph = Default::default();
    let _: GridGraph = Default::default();
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
    assert_eq!(PlanarGraph::CATEGORY, "graph");
    assert_eq!(PlanarGraph::VALUE, "PlanarGraph");
    assert_eq!(PlanarGraph::PARENT_VALUE, Some("SimpleGraph"));
}

#[test]
fn test_bipartitegraph_variant_param() {
    assert_eq!(BipartiteGraph::CATEGORY, "graph");
    assert_eq!(BipartiteGraph::VALUE, "BipartiteGraph");
    assert_eq!(BipartiteGraph::PARENT_VALUE, Some("SimpleGraph"));
}

#[test]
fn test_graph_variant_type_entries_registered() {
    let entries: Vec<_> = inventory::iter::<VariantTypeEntry>()
        .filter(|e| e.category == "graph")
        .collect();

    // Should include PlanarGraph, BipartiteGraph, and the topology types
    assert!(
        entries.iter().any(|e| e.value == "PlanarGraph" && e.parent == Some("SimpleGraph")),
        "PlanarGraph should be registered with parent SimpleGraph"
    );
    assert!(
        entries.iter().any(|e| e.value == "BipartiteGraph" && e.parent == Some("SimpleGraph")),
        "BipartiteGraph should be registered with parent SimpleGraph"
    );
    assert!(
        entries.iter().any(|e| e.value == "SimpleGraph"),
        "SimpleGraph should be registered"
    );
    assert!(
        entries.iter().any(|e| e.value == "UnitDiskGraph"),
        "UnitDiskGraph should be registered"
    );
    assert!(
        entries.iter().any(|e| e.value == "GridGraph"),
        "GridGraph should be registered"
    );
    assert!(
        entries.iter().any(|e| e.value == "Triangular"),
        "Triangular should be registered"
    );
    assert!(
        entries.iter().any(|e| e.value == "HyperGraph"),
        "HyperGraph should be registered"
    );
}

#[test]
fn test_weight_variant_type_entries_registered() {
    let entries: Vec<_> = inventory::iter::<VariantTypeEntry>()
        .filter(|e| e.category == "weight")
        .collect();

    assert!(
        entries.iter().any(|e| e.value == "One" && e.parent == Some("i32")),
        "One should be registered with parent i32"
    );
    assert!(
        entries.iter().any(|e| e.value == "i32" && e.parent == Some("f64")),
        "i32 should be registered with parent f64"
    );
    assert!(
        entries.iter().any(|e| e.value == "f64" && e.parent.is_none()),
        "f64 should be registered as root"
    );
}

#[test]
fn test_unitdiskgraph_to_planargraph_not_parent() {
    // UnitDiskGraph's parent is SimpleGraph, not PlanarGraph
    let entries: Vec<_> = inventory::iter::<VariantTypeEntry>()
        .filter(|e| e.category == "graph" && e.value == "UnitDiskGraph")
        .collect();

    for entry in &entries {
        assert_ne!(
            entry.parent,
            Some("PlanarGraph"),
            "UnitDiskGraph should not have PlanarGraph as parent"
        );
    }
}

#[test]
fn test_marker_structs_exist() {
    // Verify that all ZST marker structs still exist and can be instantiated
    let _ = SimpleGraph;
    let _ = PlanarGraph;
    let _ = UnitDiskGraph;
    let _ = BipartiteGraph;
    let _ = GridGraph;
    let _ = Triangular;
    let _ = HyperGraph;
}
