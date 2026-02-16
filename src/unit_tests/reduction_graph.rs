//! Tests for ReductionGraph: discovery, path finding, and typed API.

use crate::models::satisfiability::KSatisfiability;
use crate::poly;
use crate::prelude::*;
use crate::rules::{MinimizeSteps, ReductionGraph};
use crate::topology::{SimpleGraph, TriangularSubgraph};
use crate::traits::problem_size;
use crate::types::ProblemSize;
use crate::variant::K3;

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
    let input_size = ProblemSize::new(vec![("num_vertices", 100), ("num_edges", 200)]);

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
    let path = graph.find_cheapest_path(
        "Factoring",
        &src,
        "SpinGlass",
        &dst,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );

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

    let src2 =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst2 = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    let path2 = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src2,
        "MaximumSetPacking",
        &dst2,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );
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

    let shortest = graph.find_cheapest_path(
        "MaximumSetPacking",
        &src,
        "MinimumVertexCover",
        &dst,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );
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
    let is_var =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let vc_var = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let sg_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());
    let qubo_var = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());

    assert!(!graph
        .find_all_paths(
            "MaximumIndependentSet",
            &is_var,
            "MinimumVertexCover",
            &vc_var
        )
        .is_empty());
    assert!(!graph
        .find_all_paths(
            "MinimumVertexCover",
            &vc_var,
            "MaximumIndependentSet",
            &is_var
        )
        .is_empty());

    assert!(!graph
        .find_all_paths("SpinGlass", &sg_var, "QUBO", &qubo_var)
        .is_empty());
    assert!(!graph
        .find_all_paths("QUBO", &qubo_var, "SpinGlass", &sg_var)
        .is_empty());
}

// ---- Overhead evaluation along a path ----

#[test]
fn test_3sat_to_mis_triangular_overhead() {
    use crate::models::satisfiability::CNFClause;

    let graph = ReductionGraph::new();

    let src_var = ReductionGraph::variant_to_map(&KSatisfiability::<K3>::variant());
    let dst_var = ReductionGraph::variant_to_map(
        &MaximumIndependentSet::<TriangularSubgraph, i32>::variant(),
    );

    // 3-SAT instance: 3 variables, 2 clauses, 6 literals
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    let input_size = problem_size(&source);
    assert_eq!(input_size.get("num_vars"), Some(3));
    assert_eq!(input_size.get("num_clauses"), Some(2));
    assert_eq!(input_size.get("num_literals"), Some(6));

    // Find the shortest path
    let path = graph
        .find_cheapest_path(
            "KSatisfiability",
            &src_var,
            "MaximumIndependentSet",
            &dst_var,
            &input_size,
            &MinimizeSteps,
        )
        .expect("Should find path from 3-SAT to MIS on triangular lattice");

    // Path: K3SAT → SAT → MIS{SimpleGraph,i32} → MIS{TriangularSubgraph,i32}
    assert_eq!(
        path.type_names(),
        vec![
            "KSatisfiability",
            "Satisfiability",
            "MaximumIndependentSet"
        ]
    );
    assert_eq!(path.len(), 3);

    // Evaluate overhead at each step
    let sizes = graph.evaluate_path_overhead(&path, &input_size);
    assert_eq!(sizes.len(), 4); // initial + 3 steps

    // Step 0: K3SAT input (V=3, C=2, L=6)
    assert_eq!(sizes[0].get("num_vars"), Some(3));
    assert_eq!(sizes[0].get("num_clauses"), Some(2));
    assert_eq!(sizes[0].get("num_literals"), Some(6));

    // Step 1: K3SAT → SAT (identity: V=3, C=2, L=6)
    assert_eq!(sizes[1].get("num_vars"), Some(3));
    assert_eq!(sizes[1].get("num_clauses"), Some(2));
    assert_eq!(sizes[1].get("num_literals"), Some(6));

    // Step 2: SAT → MIS{SimpleGraph,i32}
    //   num_vertices = num_literals = 6
    //   num_edges = num_literals² = 36
    assert_eq!(sizes[2].get("num_vertices"), Some(6));
    assert_eq!(sizes[2].get("num_edges"), Some(36));

    // Step 3: MIS{SimpleGraph,i32} → MIS{TriangularSubgraph,i32}
    //   num_vertices = num_vertices² = 36
    //   num_edges = num_vertices² = 36
    assert_eq!(sizes[3].get("num_vertices"), Some(36));
    assert_eq!(sizes[3].get("num_edges"), Some(36));

    // Compose overheads symbolically along the path.
    // The composed overhead maps 3-SAT input variables to final MIS{Triangular} output.
    //
    // K3SAT → SAT:      {num_clauses: C, num_vars: V, num_literals: L}  (identity)
    // SAT → MIS:         {num_vertices: L, num_edges: L²}
    // MIS → MIS{Tri}:    {num_vertices: num_vertices², num_edges: num_vertices²}
    //
    // Composed: num_vertices = L², num_edges = L²
    let composed = graph.compose_path_overhead(&path);
    assert_eq!(
        composed.get("num_vertices").unwrap().normalized(),
        poly!(num_literals ^ 2)
    );
    assert_eq!(
        composed.get("num_edges").unwrap().normalized(),
        poly!(num_literals ^ 2)
    );
}

// ---- Overhead validation ----

#[test]
fn test_validate_overhead_variables_valid() {
    use crate::rules::validate_overhead_variables;
    use crate::rules::registry::ReductionOverhead;

    let overhead = ReductionOverhead::new(vec![
        ("num_vertices", poly!(num_vars)),
        ("num_edges", poly!(num_vars ^ 2)),
    ]);
    // Should not panic: inputs {num_vars} ⊆ source, outputs {num_vertices, num_edges} ⊆ target
    validate_overhead_variables(
        "Source",
        "Target",
        &overhead,
        &["num_vars", "num_clauses"],
        &["num_vertices", "num_edges"],
    );
}

#[test]
#[should_panic(expected = "overhead references input variables")]
fn test_validate_overhead_variables_missing_input() {
    use crate::rules::validate_overhead_variables;
    use crate::rules::registry::ReductionOverhead;

    let overhead = ReductionOverhead::new(vec![
        ("num_vertices", poly!(num_colors)),
    ]);
    validate_overhead_variables(
        "Source",
        "Target",
        &overhead,
        &["num_vars", "num_clauses"],  // no "num_colors"
        &["num_vertices"],
    );
}

#[test]
#[should_panic(expected = "overhead output fields")]
fn test_validate_overhead_variables_missing_output() {
    use crate::rules::validate_overhead_variables;
    use crate::rules::registry::ReductionOverhead;

    let overhead = ReductionOverhead::new(vec![
        ("num_gates", poly!(num_vars)),
    ]);
    validate_overhead_variables(
        "Source",
        "Target",
        &overhead,
        &["num_vars"],
        &["num_vertices", "num_edges"],  // no "num_gates"
    );
}

#[test]
fn test_validate_overhead_variables_skips_output_when_empty() {
    use crate::rules::validate_overhead_variables;
    use crate::rules::registry::ReductionOverhead;

    let overhead = ReductionOverhead::new(vec![
        ("anything", poly!(num_vars)),
    ]);
    // Should not panic: target_size_names is empty so output check is skipped
    validate_overhead_variables(
        "Source",
        "Target",
        &overhead,
        &["num_vars"],
        &[],
    );
}

#[test]
fn test_validate_overhead_variables_identity() {
    use crate::rules::validate_overhead_variables;
    use crate::rules::registry::ReductionOverhead;

    let names = &["num_vertices", "num_edges"];
    let overhead = ReductionOverhead::identity(names);
    validate_overhead_variables("A", "B", &overhead, names, names);
}
