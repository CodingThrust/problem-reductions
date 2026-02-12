//! Tests for ReductionGraph: discovery, path finding, graph hierarchy, and typed API.

use crate::prelude::*;
use crate::rules::{MinimizeSteps, ReductionGraph};
use crate::topology::SimpleGraph;
use crate::types::ProblemSize;

// ---- Discovery and registration ----

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
    assert!(graph.has_direct_reduction_by_name("MaximumIndependentSet", "MinimumVertexCover"));
    assert!(graph.has_direct_reduction_by_name("MaxCut", "SpinGlass"));
    assert!(graph.has_direct_reduction_by_name("Satisfiability", "MaximumIndependentSet"));
}

#[test]
fn test_bidirectional_reductions() {
    let graph = ReductionGraph::new();

    // IS <-> VC should both be registered
    assert!(graph.has_direct_reduction_by_name("MaximumIndependentSet", "MinimumVertexCover"));
    assert!(graph.has_direct_reduction_by_name("MinimumVertexCover", "MaximumIndependentSet"));

    // MaxCut <-> SpinGlass should both be registered
    assert!(graph.has_direct_reduction_by_name("MaxCut", "SpinGlass"));
    assert!(graph.has_direct_reduction_by_name("SpinGlass", "MaxCut"));
}

// ---- Path finding (by name) ----

#[test]
fn test_find_path_with_cost_function() {
    let graph = ReductionGraph::new();
    let input_size = ProblemSize::new(vec![("n", 100), ("m", 200)]);

    let path = graph.find_cheapest_path(
        ("MaximumIndependentSet", "SimpleGraph"),
        ("MinimumVertexCover", "SimpleGraph"),
        &input_size,
        &MinimizeSteps,
    );

    assert!(path.is_some(), "Should find path from IS to VC");
    let path = path.unwrap();
    assert_eq!(path.len(), 1, "Should be a 1-step path");
    assert_eq!(path.source(), Some("MaximumIndependentSet"));
    assert_eq!(path.target(), Some("MinimumVertexCover"));
}

#[test]
fn test_multi_step_path() {
    let graph = ReductionGraph::new();

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
fn test_problem_size_propagation() {
    let graph = ReductionGraph::new();
    let input_size = ProblemSize::new(vec![("num_vertices", 50), ("num_edges", 100)]);

    let path = graph.find_cheapest_path(
        ("MaximumIndependentSet", "SimpleGraph"),
        ("MinimumVertexCover", "SimpleGraph"),
        &input_size,
        &MinimizeSteps,
    );

    assert!(path.is_some());

    let path2 = graph.find_shortest_path_by_name("MaximumIndependentSet", "MaximumSetPacking");
    assert!(path2.is_some());
}

// ---- Graph hierarchy ----

#[test]
fn test_graph_hierarchy_built() {
    let graph = ReductionGraph::new();

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

// ---- JSON export ----

#[test]
fn test_json_export() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    assert!(!json.nodes.is_empty());
    assert!(!json.edges.is_empty());

    let categories: std::collections::HashSet<&str> =
        json.nodes.iter().map(|n| n.category.as_str()).collect();
    assert!(categories.len() >= 3, "Should have multiple categories");
}

// ---- Path finding (typed API) ----

#[test]
fn test_direct_reduction_exists() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction::<MaximumIndependentSet<SimpleGraph, i32>, MinimumVertexCover<SimpleGraph, i32>>());
    assert!(graph.has_direct_reduction::<MinimumVertexCover<SimpleGraph, i32>, MaximumIndependentSet<SimpleGraph, i32>>());
    assert!(graph
        .has_direct_reduction::<MaximumIndependentSet<SimpleGraph, i32>, MaximumSetPacking<i32>>());
    assert!(graph.has_direct_reduction::<SpinGlass<SimpleGraph, f64>, QUBO<f64>>());
    assert!(graph.has_direct_reduction::<SpinGlass<SimpleGraph, f64>, MaxCut<SimpleGraph, i32>>());
}

#[test]
fn test_find_direct_path() {
    let graph = ReductionGraph::new();

    let paths = graph.find_paths::<MaximumIndependentSet<SimpleGraph, i32>, MinimumVertexCover<SimpleGraph, i32>>();
    assert!(!paths.is_empty());
    assert_eq!(paths[0].len(), 1);
}

#[test]
fn test_find_indirect_path() {
    let graph = ReductionGraph::new();

    // MaximumSetPacking -> MaximumIndependentSet -> MinimumVertexCover
    let paths = graph.find_paths::<MaximumSetPacking<i32>, MinimumVertexCover<SimpleGraph, i32>>();
    assert!(!paths.is_empty());

    let shortest =
        graph.find_shortest_path::<MaximumSetPacking<i32>, MinimumVertexCover<SimpleGraph, i32>>();
    assert!(shortest.is_some());
    assert_eq!(shortest.unwrap().len(), 2);
}

#[test]
fn test_no_path_exists() {
    let graph = ReductionGraph::new();

    let paths = graph.find_paths::<QUBO<f64>, MaximumSetPacking<i32>>();
    assert!(paths.is_empty());
}

#[test]
fn test_bidirectional_paths() {
    let graph = ReductionGraph::new();

    assert!(!graph
        .find_paths::<MaximumIndependentSet<SimpleGraph, i32>, MinimumVertexCover<SimpleGraph, i32>>()
        .is_empty());
    assert!(!graph
        .find_paths::<MinimumVertexCover<SimpleGraph, i32>, MaximumIndependentSet<SimpleGraph, i32>>()
        .is_empty());

    assert!(!graph
        .find_paths::<SpinGlass<SimpleGraph, f64>, QUBO<f64>>()
        .is_empty());
    assert!(!graph
        .find_paths::<QUBO<f64>, SpinGlass<SimpleGraph, f64>>()
        .is_empty());
}
