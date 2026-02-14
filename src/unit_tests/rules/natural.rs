use crate::models::graph::MaximumIndependentSet;
use crate::rules::graph::{EdgeKind, ReductionGraph};
use crate::topology::{SimpleGraph, Triangular};
use std::collections::BTreeMap;

#[test]
fn test_natural_cast_triangular_to_simple_via_resolve() {
    let graph = ReductionGraph::new();

    // Find any path from MIS to itself (via VC round-trip) to test natural cast insertion
    // Instead, directly test that resolve_path inserts a natural cast for MIS(Triangular)->VC(SimpleGraph)
    let name_path = graph
        .find_shortest_path::<
            MaximumIndependentSet<Triangular, i32>,
            crate::models::graph::MinimumVertexCover<SimpleGraph, i32>,
        >()
        .unwrap();

    let source_variant = BTreeMap::from([
        ("graph".to_string(), "Triangular".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let target_variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);

    let resolved = graph
        .resolve_path(&name_path, &source_variant, &target_variant)
        .unwrap();

    // Path should be: MIS(Triangular) --NaturalCast--> MIS(SimpleGraph) --Reduction--> VC(SimpleGraph)
    assert_eq!(resolved.num_casts(), 1);
    assert_eq!(resolved.num_reductions(), 1);
    assert!(matches!(resolved.edges[0], EdgeKind::NaturalCast));
    assert!(matches!(resolved.edges[1], EdgeKind::Reduction { .. }));
    assert_eq!(
        resolved.steps[0].variant.get("graph").unwrap(),
        "Triangular"
    );
    assert_eq!(
        resolved.steps[1].variant.get("graph").unwrap(),
        "SimpleGraph"
    );
}
