use super::*;

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
fn test_unit_disk_to_planar_registered() {
    let entries: Vec<_> = inventory::iter::<GraphSubtypeEntry>().collect();
    assert!(entries
        .iter()
        .any(|e| e.subtype == "UnitDiskGraph" && e.supertype == "PlanarGraph"));
}
