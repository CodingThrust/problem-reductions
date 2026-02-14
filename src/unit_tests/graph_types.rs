use super::*;

#[test]
fn test_reflexive_subtype() {
    fn assert_subtype<A: GraphSubtype<B>, B: GraphMarker>() {}

    // Every type is a subtype of itself
    assert_subtype::<SimpleGraph, SimpleGraph>();
    assert_subtype::<PlanarGraph, PlanarGraph>();
    assert_subtype::<UnitDiskGraph, UnitDiskGraph>();
    assert_subtype::<BipartiteGraph, BipartiteGraph>();
    assert_subtype::<GridGraph, GridGraph>();
    assert_subtype::<HyperGraph, HyperGraph>();
}

#[test]
fn test_subtype_entries_registered() {
    let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();

    // Should have at least 10 entries
    assert!(entries.len() >= 10);

    // Check specific relationships
    assert!(entries
        .iter()
        .any(|e| e.subtype == "UnitDiskGraph" && e.supertype == "SimpleGraph"));
    assert!(entries
        .iter()
        .any(|e| e.subtype == "PlanarGraph" && e.supertype == "SimpleGraph"));
}

#[test]
fn test_declared_subtypes() {
    fn assert_subtype<A: GraphSubtype<B>, B: GraphMarker>() {}

    // Declared relationships
    assert_subtype::<GridGraph, UnitDiskGraph>();
    assert_subtype::<GridGraph, SimpleGraph>();
    assert_subtype::<GridGraph, HyperGraph>();
    assert_subtype::<UnitDiskGraph, SimpleGraph>();
    assert_subtype::<UnitDiskGraph, HyperGraph>();
    assert_subtype::<PlanarGraph, SimpleGraph>();
    assert_subtype::<PlanarGraph, HyperGraph>();
    assert_subtype::<BipartiteGraph, SimpleGraph>();
    assert_subtype::<BipartiteGraph, HyperGraph>();
    assert_subtype::<SimpleGraph, HyperGraph>();
}

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
fn test_bipartite_entry_registered() {
    let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();
    assert!(entries
        .iter()
        .any(|e| e.subtype == "BipartiteGraph" && e.supertype == "SimpleGraph"));
}

#[test]
fn test_unit_disk_to_planar_not_registered() {
    let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();
    // UnitDiskGraph => PlanarGraph was removed (incorrect relationship)
    assert!(!entries
        .iter()
        .any(|e| e.subtype == "UnitDiskGraph" && e.supertype == "PlanarGraph"));
}

#[test]
fn test_gridgraph_subtypes() {
    fn assert_subtype<A: GraphSubtype<B>, B: GraphMarker>() {}
    assert_subtype::<GridGraph, UnitDiskGraph>();
    assert_subtype::<GridGraph, SimpleGraph>();
    assert_subtype::<GridGraph, HyperGraph>();
}

#[test]
fn test_hypergraph_subtypes() {
    fn assert_subtype<A: GraphSubtype<B>, B: GraphMarker>() {}
    assert_subtype::<SimpleGraph, HyperGraph>();
    assert_subtype::<PlanarGraph, HyperGraph>();
    assert_subtype::<UnitDiskGraph, HyperGraph>();
    assert_subtype::<BipartiteGraph, HyperGraph>();
    assert_subtype::<GridGraph, HyperGraph>();
}

#[test]
fn test_gridgraph_entries_registered() {
    let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();
    assert!(entries
        .iter()
        .any(|e| e.subtype == "GridGraph" && e.supertype == "UnitDiskGraph"));
}

#[test]
fn test_hypergraph_entries_registered() {
    let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();
    assert!(entries
        .iter()
        .any(|e| e.subtype == "SimpleGraph" && e.supertype == "HyperGraph"));
}

#[test]
fn test_weight_subtype_entries_registered() {
    let entries: Vec<_> = inventory::iter::<WeightSubtypeEntry>().collect();
    assert!(entries
        .iter()
        .any(|e| e.subtype == "One" && e.supertype == "i32"));
    assert!(entries
        .iter()
        .any(|e| e.subtype == "i32" && e.supertype == "f64"));
    assert!(entries
        .iter()
        .any(|e| e.subtype == "One" && e.supertype == "f64"));
}
