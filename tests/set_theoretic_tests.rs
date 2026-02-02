//! Integration tests for set-theoretic reduction path finding.

use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::types::ProblemSize;

#[test]
fn test_reduction_graph_discovers_registered_reductions() {
    let graph = ReductionGraph::new();

    // Should have discovered reductions from inventory
    assert!(
        graph.num_types() >= 10,
        "Should have at least 10 problem types"
    );
    assert!(
        graph.num_reductions() >= 15,
        "Should have at least 15 reductions"
    );

    // Specific reductions should exist
    assert!(graph.has_direct_reduction_by_name("IndependentSet", "VertexCovering"));
    assert!(graph.has_direct_reduction_by_name("MaxCut", "SpinGlass"));
    assert!(graph.has_direct_reduction_by_name("Satisfiability", "IndependentSet"));
}

#[test]
fn test_find_path_with_cost_function() {
    let graph = ReductionGraph::new();
    let input_size = ProblemSize::new(vec![("n", 100), ("m", 200)]);

    // Find path from IndependentSet to VertexCovering using SimpleGraph
    // This is a direct path where both source and target use SimpleGraph
    let path = graph.find_cheapest_path(
        ("IndependentSet", "SimpleGraph"),
        ("VertexCovering", "SimpleGraph"),
        &input_size,
        &MinimizeSteps,
    );

    assert!(path.is_some(), "Should find path from IS to VC");
    let path = path.unwrap();
    assert_eq!(path.len(), 1, "Should be a 1-step path");
    assert_eq!(path.source(), Some("IndependentSet"));
    assert_eq!(path.target(), Some("VertexCovering"));
}

#[test]
fn test_multi_step_path() {
    let graph = ReductionGraph::new();

    // Use find_shortest_path_by_name which doesn't validate graph types
    // Factoring -> CircuitSAT -> SpinGlass is a 2-step path
    let path = graph.find_shortest_path_by_name("Factoring", "SpinGlass");

    assert!(
        path.is_some(),
        "Should find path from Factoring to SpinGlass"
    );
    let path = path.unwrap();
    assert_eq!(path.len(), 2, "Should be a 2-step path");
    assert_eq!(
        path.type_names,
        vec!["Factoring", "CircuitSAT", "SpinGlass"]
    );
}

#[test]
fn test_graph_hierarchy_built() {
    let graph = ReductionGraph::new();

    // Test the graph hierarchy was built from GraphSubtypeEntry
    assert!(graph.is_graph_subtype("UnitDiskGraph", "SimpleGraph"));
    assert!(graph.is_graph_subtype("PlanarGraph", "SimpleGraph"));
    assert!(graph.is_graph_subtype("BipartiteGraph", "SimpleGraph"));

    // Reflexive
    assert!(graph.is_graph_subtype("SimpleGraph", "SimpleGraph"));

    // Non-subtype relationships
    assert!(!graph.is_graph_subtype("SimpleGraph", "UnitDiskGraph"));
}

#[test]
fn test_rule_applicability() {
    let graph = ReductionGraph::new();

    // Rule for SimpleGraph applies to UnitDiskGraph source (UnitDisk <= Simple)
    assert!(graph.rule_applicable("UnitDiskGraph", "SimpleGraph", "SimpleGraph", "SimpleGraph"));

    // Rule for UnitDiskGraph doesn't apply to SimpleGraph source (Simple is NOT <= UnitDisk)
    assert!(!graph.rule_applicable("SimpleGraph", "SimpleGraph", "UnitDiskGraph", "SimpleGraph"));
}

#[test]
fn test_bidirectional_reductions() {
    let graph = ReductionGraph::new();

    // IS <-> VC should both be registered
    assert!(graph.has_direct_reduction_by_name("IndependentSet", "VertexCovering"));
    assert!(graph.has_direct_reduction_by_name("VertexCovering", "IndependentSet"));

    // MaxCut <-> SpinGlass should both be registered
    assert!(graph.has_direct_reduction_by_name("MaxCut", "SpinGlass"));
    assert!(graph.has_direct_reduction_by_name("SpinGlass", "MaxCut"));
}

#[test]
fn test_problem_size_propagation() {
    let graph = ReductionGraph::new();
    let input_size = ProblemSize::new(vec![("num_vertices", 50), ("num_edges", 100)]);

    // Path finding should work with size propagation using compatible graph types
    // IndependentSet -> VertexCovering uses SimpleGraph -> SimpleGraph
    let path = graph.find_cheapest_path(
        ("IndependentSet", "SimpleGraph"),
        ("VertexCovering", "SimpleGraph"),
        &input_size,
        &MinimizeSteps,
    );

    assert!(path.is_some());

    // Also test that find_shortest_path_by_name works for multi-step paths
    let path2 = graph.find_shortest_path_by_name("IndependentSet", "SetPacking");
    assert!(path2.is_some());
}

#[test]
fn test_json_export() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // Should have nodes for registered problems
    assert!(!json.nodes.is_empty());
    assert!(!json.edges.is_empty());

    // Categories should be assigned
    let categories: std::collections::HashSet<&str> =
        json.nodes.iter().map(|n| n.category.as_str()).collect();
    assert!(categories.len() >= 3, "Should have multiple categories");
}
