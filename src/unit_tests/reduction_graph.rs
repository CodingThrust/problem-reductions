//! Tests for ReductionGraph: discovery, path finding, and typed API.

use crate::prelude::*;
use crate::rules::{MinimizeSteps, ReductionGraph};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
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

    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    let path = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src,
        "MinimumVertexCover",
        &dst,
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

    // Factoring -> CircuitSAT -> SpinGlass<SimpleGraph, i32> is a 2-step path
    let src = ReductionGraph::variant_to_map(&crate::models::specialized::Factoring::variant());
    let dst = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, i32>::variant());
    let path = graph.find_cheapest_path("Factoring", &src, "SpinGlass", &dst, &ProblemSize::new(vec![]), &MinimizeSteps);

    assert!(
        path.is_some(),
        "Should find path from Factoring to SpinGlass"
    );
    let path = path.unwrap();
    assert_eq!(path.len(), 2, "Should be a 2-step path");
    assert_eq!(
        path.type_names(),
        vec!["Factoring", "CircuitSAT", "SpinGlass"]
    );
}

#[test]
fn test_problem_size_propagation() {
    let graph = ReductionGraph::new();
    let input_size = ProblemSize::new(vec![("num_vertices", 50), ("num_edges", 100)]);

    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    let path = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src,
        "MinimumVertexCover",
        &dst,
        &input_size,
        &MinimizeSteps,
    );

    assert!(path.is_some());

    let src2 = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst2 = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    let path2 = graph.find_cheapest_path("MaximumIndependentSet", &src2, "MaximumSetPacking", &dst2, &ProblemSize::new(vec![]), &MinimizeSteps);
    assert!(path2.is_some());
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

// ---- Path finding (variant-level API) ----

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
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    let paths = graph.find_all_paths("MaximumIndependentSet", &src, "MinimumVertexCover", &dst);
    assert!(!paths.is_empty());
    assert!(
        paths.iter().any(|p| p.len() == 1),
        "Should contain a direct (1-step) path, got lengths: {:?}",
        paths.iter().map(|p| p.len()).collect::<Vec<_>>()
    );
}

#[test]
fn test_find_indirect_path() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    // MaximumSetPacking -> MaximumIndependentSet -> MinimumVertexCover
    let paths = graph.find_all_paths("MaximumSetPacking", &src, "MinimumVertexCover", &dst);
    assert!(!paths.is_empty());

    let shortest = graph.find_cheapest_path("MaximumSetPacking", &src, "MinimumVertexCover", &dst, &ProblemSize::new(vec![]), &MinimizeSteps);
    assert!(shortest.is_some());
    assert_eq!(shortest.unwrap().len(), 2);
}

#[test]
fn test_no_path_exists() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());

    let paths = graph.find_all_paths("QUBO", &src, "MaximumSetPacking", &dst);
    assert!(paths.is_empty());
}

#[test]
fn test_bidirectional_paths() {
    let graph = ReductionGraph::new();
    let is_var = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let vc_var = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let sg_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());
    let qubo_var = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());

    assert!(!graph.find_all_paths("MaximumIndependentSet", &is_var, "MinimumVertexCover", &vc_var).is_empty());
    assert!(!graph.find_all_paths("MinimumVertexCover", &vc_var, "MaximumIndependentSet", &is_var).is_empty());

    assert!(!graph.find_all_paths("SpinGlass", &sg_var, "QUBO", &qubo_var).is_empty());
    assert!(!graph.find_all_paths("QUBO", &qubo_var, "SpinGlass", &sg_var).is_empty());
}
