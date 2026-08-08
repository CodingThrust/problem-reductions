//! Tests for ReductionGraph: discovery, path finding, and typed API.

use crate::expr::evaluate_approximate;
#[cfg(feature = "ilp-solver")]
use crate::models::algebraic::ILP;
use crate::models::decision::Decision;
use crate::models::formula::KSatisfiability;
use crate::models::misc::Clustering;
use crate::prelude::*;
use crate::rules::{ReductionGraph, ReductionMode, TraversalFlow};
use crate::topology::{KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph};
use crate::types::ProblemSize;
use crate::variant::{K3, KN};
use std::collections::BTreeMap;

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
fn test_reduction_graph_discovers_k3coloring_to_clustering() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction::<KColoring<K3, SimpleGraph>, Clustering>());
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_reduction_graph_discovers_clustering_to_ilp() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction::<Clustering, ILP<bool>>());
}

// ---- Path finding (by name) ----

#[test]
fn test_find_direct_route_by_exact_variants() {
    let graph = ReductionGraph::new();

    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    let path = graph
        .find_all_paths("MaximumIndependentSet", &src, "MinimumVertexCover", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("direct route should exist");
    assert_eq!(path.len(), 1, "Should be a 1-step path");
    assert_eq!(path.source(), Some("MaximumIndependentSet"));
    assert_eq!(path.target(), Some("MinimumVertexCover"));
}

#[test]
fn test_multi_step_path() {
    let graph = ReductionGraph::new();

    // Factoring -> CircuitSAT -> SpinGlass<SimpleGraph, i32> is a 2-step path
    let src = ReductionGraph::variant_to_map(&crate::models::misc::Factoring::variant());
    let dst = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, i32>::variant());
    let path = graph
        .find_all_paths("Factoring", &src, "SpinGlass", &dst)
        .into_iter()
        .find(|path| path.type_names() == ["Factoring", "CircuitSAT", "SpinGlass"])
        .expect("explicit CircuitSAT route should exist");
    assert_eq!(path.len(), 2, "Should be a 2-step path");
    assert_eq!(
        path.type_names(),
        vec!["Factoring", "CircuitSAT", "SpinGlass"]
    );
}

#[test]
fn aggregate_mode_rejects_witness_only_real_edge() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    assert!(!graph
        .find_all_paths_mode(
            "MaximumIndependentSet",
            &src,
            "MinimumVertexCover",
            &dst,
            ReductionMode::Witness
        )
        .is_empty());
    assert!(graph
        .find_all_paths_mode(
            "MaximumIndependentSet",
            &src,
            "MinimumVertexCover",
            &dst,
            ReductionMode::Aggregate
        )
        .is_empty());
}

#[test]
fn natural_edge_supports_both_modes_public_api() {
    let graph = ReductionGraph::new();
    let src =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<KingsSubgraph, i32>::variant());
    let dst =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<UnitDiskGraph, i32>::variant());

    assert!(!graph
        .find_all_paths_mode(
            "MaximumIndependentSet",
            &src,
            "MaximumIndependentSet",
            &dst,
            ReductionMode::Witness
        )
        .is_empty());
    assert!(!graph
        .find_all_paths_mode(
            "MaximumIndependentSet",
            &src,
            "MaximumIndependentSet",
            &dst,
            ReductionMode::Aggregate
        )
        .is_empty());
}

#[test]
fn value_changing_variant_cast_is_not_aggregate_capable() {
    use crate::models::set::MaximumSetPacking;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<f64>::variant());

    assert!(graph
        .find_all_paths_mode(
            "MaximumSetPacking",
            &src,
            "MaximumSetPacking",
            &dst,
            ReductionMode::Aggregate
        )
        .is_empty());
}

#[test]
fn test_problem_size_propagation() {
    let graph = ReductionGraph::new();

    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    assert!(!graph
        .find_all_paths("MaximumIndependentSet", &src, "MinimumVertexCover", &dst)
        .is_empty());

    let src2 =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst2 = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    assert!(!graph
        .find_all_paths("MaximumIndependentSet", &src2, "MaximumSetPacking", &dst2)
        .is_empty());
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

#[test]
fn test_subsetsum_to_integerknapsack_is_proof_only() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_by_name("SubsetSum", "IntegerKnapsack"));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "SubsetSum",
        "IntegerKnapsack",
        ReductionMode::Witness,
    ));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "SubsetSum",
        "IntegerKnapsack",
        ReductionMode::Aggregate,
    ));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "SubsetSum",
        "IntegerKnapsack",
        ReductionMode::Turing,
    ));
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_integerknapsack_to_ilp_is_runtime_witness_edge() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_by_name_mode(
        "IntegerKnapsack",
        "ILP",
        ReductionMode::Witness,
    ));
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
fn test_kcoloring_to_partitionintocliques_smoke() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 2);
    let reduction = ReduceTo::<PartitionIntoCliques<SimpleGraph>>::reduce_to(&source);
    assert_eq!(reduction.target_problem().num_cliques(), 2);
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

    assert!(paths.iter().any(|path| path.len() == 2));
}

#[test]
fn test_no_path_exists() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());

    let paths = graph.find_all_paths("QUBO", &src, "MaximumSetPacking", &dst);
    assert!(paths.is_empty());
}

// ---- Display ----

#[test]
fn test_reduction_path_display() {
    let graph = ReductionGraph::new();
    let src_var = ReductionGraph::variant_to_map(&Factoring::variant());
    let dst_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());
    let path = graph
        .find_all_paths("Factoring", &src_var, "SpinGlass", &dst_var)
        .into_iter()
        .find(|path| path.type_names() == ["Factoring", "CircuitSAT", "SpinGlass"])
        .expect("explicit CircuitSAT route");

    let s = format!("{path}");
    // Should contain arrow-separated problem names with variant info
    assert!(s.contains("Factoring"));
    assert!(s.contains("→"));
    assert!(s.contains("SpinGlass"));

    // Step with empty variant
    let step = &path.steps[0];
    assert_eq!(format!("{step}"), "Factoring");

    // Step with non-empty variant
    let last = path.steps.last().unwrap();
    let last_s = format!("{last}");
    assert!(last_s.contains("SpinGlass"));
    assert!(last_s.contains("{"));
}

// ---- Overhead evaluation along a path ----

#[test]
fn test_3sat_to_mis_triangular_overhead() {
    use crate::models::formula::CNFClause;

    let graph = ReductionGraph::new();

    let src_var = ReductionGraph::variant_to_map(&KSatisfiability::<K3>::variant());
    let dst_var = ReductionGraph::variant_to_map(
        &MaximumIndependentSet::<TriangularSubgraph, i32>::variant(),
    );

    // 3-SAT instance: 3 variables, 2 clauses, 6 literals
    let _source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    let path = graph
        .find_all_paths(
            "KSatisfiability",
            &src_var,
            "MaximumIndependentSet",
            &dst_var,
        )
        .into_iter()
        .find(|path| {
            path.len() == 4
                && path.type_names()
                    == ["KSatisfiability", "Satisfiability", "MaximumIndependentSet"]
        })
        .expect("expected explicit 3-SAT to triangular MIS route");

    // Path: K3SAT → KN_SAT (cast) → SAT → MIS{SimpleGraph,One} → MIS{TriangularSubgraph,i32}
    assert_eq!(
        path.type_names(),
        vec!["KSatisfiability", "Satisfiability", "MaximumIndependentSet"]
    );
    assert_eq!(path.len(), 4);

    // Per-edge symbolic overheads
    let edges = graph.path_overheads(&path);
    assert_eq!(edges.len(), 4);

    // Evaluate overheads at a test point to verify correctness
    let test_size = ProblemSize::new(vec![
        ("num_vars", 3),
        ("num_clauses", 2),
        ("num_literals", 6),
        ("num_vertices", 10),
        ("num_edges", 15),
    ]);
    let approximate = |expression| evaluate_approximate(expression, &test_size).unwrap();

    // Edge 0: K3SAT → KN_SAT (variant cast, identity for num_vars + num_clauses)
    assert_eq!(approximate(edges[0].get("num_vars").unwrap()), 3.0);
    assert_eq!(approximate(edges[0].get("num_clauses").unwrap()), 2.0);

    // Edge 1: KN_SAT → SAT (identity)
    assert_eq!(approximate(edges[1].get("num_vars").unwrap()), 3.0);
    assert_eq!(approximate(edges[1].get("num_clauses").unwrap()), 2.0);
    assert_eq!(approximate(edges[1].get("num_literals").unwrap()), 6.0);

    // Edge 2: SAT → MIS{SimpleGraph,One}
    // num_vertices = num_literals, num_edges = num_literals^2
    assert_eq!(approximate(edges[2].get("num_vertices").unwrap()), 6.0);
    assert_eq!(approximate(edges[2].get("num_edges").unwrap()), 36.0);

    // Edge 3: MIS{SimpleGraph,One} → MIS{TriangularSubgraph,i32}
    // num_vertices = num_vertices², num_edges = num_vertices²
    assert_eq!(approximate(edges[3].get("num_vertices").unwrap()), 100.0);
    assert_eq!(approximate(edges[3].get("num_edges").unwrap()), 100.0);

    // Compose overheads symbolically along the path.
    // The composed overhead maps 3-SAT input variables to final MIS{Triangular} output.
    //
    // K3SAT → KN_SAT:      {num_clauses: C, num_vars: V, num_literals: L}  (identity cast)
    // KN_SAT → SAT:         {num_clauses: C, num_vars: V, num_literals: L}  (identity)
    // SAT → MIS{SG,One}:    {num_vertices: L, num_edges: L²}
    // MIS{SG,One→Tri}:      {num_vertices: V², num_edges: V²}
    //
    // Composed: num_vertices = L², num_edges = L²
    let composed = graph.compose_path_overhead(&path);
    // Evaluate composed at input: L=6, so L²=36
    assert_eq!(approximate(composed.get("num_vertices").unwrap()), 36.0);
    assert_eq!(approximate(composed.get("num_edges").unwrap()), 36.0);
}

// ---- k-neighbor BFS ----

#[test]
fn test_k_neighbors_outgoing() {
    let graph = ReductionGraph::new();
    let variants = graph.variants_for("MaximumIndependentSet");
    assert!(!variants.is_empty());
    let default_variant = &variants[0];

    // 1-hop outgoing: should include direct reduction targets
    let neighbors = graph.k_neighbors(
        "MaximumIndependentSet",
        default_variant,
        1,
        TraversalFlow::Outgoing,
    );
    assert!(!neighbors.is_empty());
    assert!(neighbors.iter().all(|n| n.hops == 1));

    // 2-hop outgoing: should include more problems
    let neighbors_2 = graph.k_neighbors(
        "MaximumIndependentSet",
        default_variant,
        2,
        TraversalFlow::Outgoing,
    );
    assert!(neighbors_2.len() >= neighbors.len());
}

#[test]
fn test_k_neighbors_incoming() {
    let graph = ReductionGraph::new();
    let variants = graph.variants_for("QUBO");
    assert!(!variants.is_empty());

    let neighbors = graph.k_neighbors("QUBO", &variants[0], 1, TraversalFlow::Incoming);
    // QUBO is a common target — should have incoming reductions
    assert!(!neighbors.is_empty());
}

#[test]
fn test_k_neighbors_both() {
    let graph = ReductionGraph::new();
    let variants = graph.variants_for("MaximumIndependentSet");
    let default_variant = &variants[0];

    let out_only = graph.k_neighbors(
        "MaximumIndependentSet",
        default_variant,
        1,
        TraversalFlow::Outgoing,
    );
    let in_only = graph.k_neighbors(
        "MaximumIndependentSet",
        default_variant,
        1,
        TraversalFlow::Incoming,
    );
    let both = graph.k_neighbors(
        "MaximumIndependentSet",
        default_variant,
        1,
        TraversalFlow::Both,
    );
    // Both should be >= max of either direction
    assert!(both.len() >= out_only.len());
    assert!(both.len() >= in_only.len());
}

#[test]
fn test_k_neighbors_unknown_problem() {
    let graph = ReductionGraph::new();
    let empty = BTreeMap::new();
    let neighbors = graph.k_neighbors("NonExistent", &empty, 2, TraversalFlow::Outgoing);
    assert!(neighbors.is_empty());
}

#[test]
fn test_k_neighbors_zero_hops() {
    let graph = ReductionGraph::new();
    let variants = graph.variants_for("MaximumIndependentSet");
    let default_variant = &variants[0];
    let neighbors = graph.k_neighbors(
        "MaximumIndependentSet",
        default_variant,
        0,
        TraversalFlow::Outgoing,
    );
    assert!(neighbors.is_empty());
}

// ---- Default variant resolution ----

#[test]
fn default_variant_for_mis_uses_declared_default() {
    let graph = ReductionGraph::new();
    let default = graph.default_variant_for("MaximumIndependentSet");
    assert!(
        default.is_some(),
        "MaximumIndependentSet should have a declared default variant"
    );
    let variant = default.unwrap();
    assert_eq!(
        variant.get("graph").map(|s| s.as_str()),
        Some("SimpleGraph"),
        "default MIS variant should use SimpleGraph"
    );
    assert_eq!(
        variant.get("weight").map(|s| s.as_str()),
        Some("One"),
        "default MIS variant should use One (unit weight)"
    );
}

#[test]
fn default_variant_for_unknown_problem_returns_none() {
    let graph = ReductionGraph::new();
    let default = graph.default_variant_for("NonExistentProblem");
    assert!(
        default.is_none(),
        "unknown problem should have no default variant"
    );
}

#[test]
fn default_variant_for_mvc_uses_declared_default() {
    let graph = ReductionGraph::new();
    let default = graph.default_variant_for("MinimumVertexCover");
    assert!(
        default.is_some(),
        "MinimumVertexCover should have a declared default variant"
    );
    let variant = default.unwrap();
    assert_eq!(
        variant.get("graph").map(|s| s.as_str()),
        Some("SimpleGraph"),
        "default MVC variant should use SimpleGraph"
    );
    assert_eq!(
        variant.get("weight").map(|s| s.as_str()),
        Some("i32"),
        "default MVC variant should use i32"
    );
}

#[test]
fn default_variant_for_qubo_uses_declared_default() {
    let graph = ReductionGraph::new();
    let default = graph.default_variant_for("QUBO");
    assert!(
        default.is_some(),
        "QUBO should have a declared default variant"
    );
    let variant = default.unwrap();
    assert_eq!(
        variant.get("weight").map(|s| s.as_str()),
        Some("f64"),
        "default QUBO variant should use f64"
    );
}

#[test]
fn default_variant_for_ksat_uses_declared_default() {
    let graph = ReductionGraph::new();
    let default = graph.default_variant_for("KSatisfiability");
    assert!(
        default.is_some(),
        "KSatisfiability should have a declared default variant"
    );
    let variant = default.unwrap();
    assert_eq!(
        variant.get("k").map(|s| s.as_str()),
        Some("KN"),
        "default KSatisfiability variant should use KN"
    );
}

#[test]
fn default_variant_for_sat_returns_empty() {
    // Satisfiability has no variant dimensions, so its default is an empty map
    let graph = ReductionGraph::new();
    let default = graph.default_variant_for("Satisfiability");
    assert!(
        default.is_some(),
        "Satisfiability should have a declared default variant"
    );
    assert!(
        default.unwrap().is_empty(),
        "Satisfiability default variant should be empty (no dimensions)"
    );
}

// ---- Capped path enumeration ----

#[test]
fn find_paths_up_to_stops_after_limit() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());

    // Get all paths to know the total count
    let all = graph.find_all_paths("MaximumIndependentSet", &src, "QUBO", &dst);
    assert!(all.len() > 3, "need multiple paths for this test");

    // With a limit of 3, should get exactly 3
    let limited = graph.find_paths_up_to("MaximumIndependentSet", &src, "QUBO", &dst, 3);
    assert!(
        limited.len() <= 3 && limited.len() < all.len(),
        "should stop before enumerating all {} paths, got {}",
        all.len(),
        limited.len()
    );
}

#[test]
fn find_paths_up_to_returns_all_when_limit_exceeds_total() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    let all = graph.find_all_paths("MaximumIndependentSet", &src, "MinimumVertexCover", &dst);
    let limited = graph.find_paths_up_to(
        "MaximumIndependentSet",
        &src,
        "MinimumVertexCover",
        &dst,
        1000,
    );
    assert_eq!(
        limited.len(),
        all.len(),
        "should return all paths when limit exceeds total"
    );
}

#[test]
fn find_paths_up_to_no_path() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());

    let limited = graph.find_paths_up_to("QUBO", &src, "MaximumSetPacking", &dst, 10);
    assert!(limited.is_empty());
}

// ---- Exact source+target variant matching ----

#[test]
fn find_entry_rejects_wrong_target_variant() {
    let graph = ReductionGraph::new();
    let source =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    // MIS<SG,i32> -> MVC<SG,i32> exists, but MVC<SG,f64> does not
    let wrong_target = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "f64".to_string()),
    ]);
    let result = graph.find_entry(
        "MaximumIndependentSet",
        &source,
        "MinimumVertexCover",
        &wrong_target,
    );
    assert!(result.is_none(), "Should reject wrong target variant");
}

#[test]
fn find_entry_accepts_exact_source_and_target_variant() {
    let graph = ReductionGraph::new();
    let source =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let target = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let result = graph.find_entry(
        "MaximumIndependentSet",
        &source,
        "MinimumVertexCover",
        &target,
    );
    assert!(
        result.is_some(),
        "Should find exact match on both source and target variant"
    );
}

#[test]
fn test_has_direct_reduction_mode_witness() {
    let graph = ReductionGraph::new();

    // MIS -> MVC is witness-only, so Witness mode should find it
    assert!(graph
        .has_direct_reduction_mode::<MaximumIndependentSet<SimpleGraph, i32>, MinimumVertexCover<SimpleGraph, i32>>(
            ReductionMode::Witness,
        ));
}

#[test]
fn test_has_direct_reduction_by_name_mode() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_by_name_mode(
        "MaximumIndependentSet",
        "MinimumVertexCover",
        ReductionMode::Witness,
    ));

    // Aggregate mode should not find witness-only edges
    assert!(!graph.has_direct_reduction_by_name_mode(
        "MaximumIndependentSet",
        "MinimumVertexCover",
        ReductionMode::Aggregate,
    ));
}

#[test]
fn test_minimumvertexcover_to_minimummaximalmatching_is_proof_only_direct_edge() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_by_name("MinimumVertexCover", "MinimumMaximalMatching",));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "MinimumVertexCover",
        "MinimumMaximalMatching",
        ReductionMode::Witness,
    ));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "MinimumVertexCover",
        "MinimumMaximalMatching",
        ReductionMode::Aggregate,
    ));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "MinimumVertexCover",
        "MinimumMaximalMatching",
        ReductionMode::Turing,
    ));
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_minimumcoveringbycliques_to_ilp_is_runtime_witness_edge() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_by_name_mode(
        "MinimumCoveringByCliques",
        "ILP",
        ReductionMode::Witness,
    ));
}

#[test]
fn test_find_all_paths_mode_witness() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    let paths = graph.find_all_paths_mode(
        "MaximumIndependentSet",
        &src,
        "MinimumVertexCover",
        &dst,
        ReductionMode::Witness,
    );
    assert!(!paths.is_empty());
}

#[test]
fn test_find_all_paths_mode_aggregate_rejects_witness_only() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    // MIS -> MVC is witness-only, so aggregate mode should find no paths
    let paths = graph.find_all_paths_mode(
        "MaximumIndependentSet",
        &src,
        "MinimumVertexCover",
        &dst,
        ReductionMode::Aggregate,
    );
    assert!(paths.is_empty());
}

#[test]
fn test_decision_minimum_vertex_cover_has_both_edges() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_by_name_mode(
        "DecisionMinimumVertexCover",
        "MinimumVertexCover",
        ReductionMode::Aggregate,
    ));
    assert!(graph.has_direct_reduction_by_name_mode(
        "DecisionMinimumVertexCover",
        "MinimumVertexCover",
        ReductionMode::Witness,
    ));
}

#[test]
fn test_decision_minimum_dominating_set_has_both_edges() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_by_name_mode(
        "DecisionMinimumDominatingSet",
        "MinimumDominatingSet",
        ReductionMode::Aggregate,
    ));
    assert!(graph.has_direct_reduction_by_name_mode(
        "DecisionMinimumDominatingSet",
        "MinimumDominatingSet",
        ReductionMode::Witness,
    ));
}

#[test]
fn test_decision_minimum_dominating_set_to_minmax_multicenter_has_direct_witness_edge() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_mode::<
        Decision<MinimumDominatingSet<SimpleGraph, One>>,
        MinMaxMulticenter<SimpleGraph, One>,
    >(ReductionMode::Witness));
    assert!(!graph.has_direct_reduction_mode::<
        Decision<MinimumDominatingSet<SimpleGraph, One>>,
        MinMaxMulticenter<SimpleGraph, One>,
    >(ReductionMode::Aggregate));
    assert!(!graph.has_direct_reduction_mode::<
        Decision<MinimumDominatingSet<SimpleGraph, One>>,
        MinMaxMulticenter<SimpleGraph, One>,
    >(ReductionMode::Turing));
}

#[test]
fn test_decision_minimum_dominating_set_to_minimum_sum_multicenter_has_direct_witness_edge() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_mode::<
        Decision<MinimumDominatingSet<SimpleGraph, One>>,
        MinimumSumMulticenter<SimpleGraph, i32>,
    >(ReductionMode::Witness));
    assert!(!graph.has_direct_reduction_mode::<
        Decision<MinimumDominatingSet<SimpleGraph, One>>,
        MinimumSumMulticenter<SimpleGraph, i32>,
    >(ReductionMode::Aggregate));
    assert!(!graph.has_direct_reduction_mode::<
        Decision<MinimumDominatingSet<SimpleGraph, One>>,
        MinimumSumMulticenter<SimpleGraph, i32>,
    >(ReductionMode::Turing));
}

#[test]
fn test_optimization_to_decision_turing_edges() {
    let graph = ReductionGraph::new();

    // MinimumVertexCover → DecisionMinimumVertexCover (Turing)
    assert!(graph.has_direct_reduction_by_name_mode(
        "MinimumVertexCover",
        "DecisionMinimumVertexCover",
        ReductionMode::Turing,
    ));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "MinimumVertexCover",
        "DecisionMinimumVertexCover",
        ReductionMode::Witness,
    ));
    assert!(!graph.has_direct_reduction_by_name_mode(
        "MinimumVertexCover",
        "DecisionMinimumVertexCover",
        ReductionMode::Aggregate,
    ));

    // MinimumDominatingSet → DecisionMinimumDominatingSet (Turing)
    assert!(graph.has_direct_reduction_by_name_mode(
        "MinimumDominatingSet",
        "DecisionMinimumDominatingSet",
        ReductionMode::Turing,
    ));
}

#[test]
fn test_ksatisfiability_k3_to_decision_minimum_vertex_cover_direct_witness_edge() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction_mode::<
        KSatisfiability<K3>,
        Decision<MinimumVertexCover<SimpleGraph, i32>>,
    >(ReductionMode::Witness));
    assert!(!graph.has_direct_reduction_mode::<
        KSatisfiability<K3>,
        Decision<MinimumVertexCover<SimpleGraph, i32>>,
    >(ReductionMode::Aggregate));
    assert!(!graph.has_direct_reduction_mode::<
        KSatisfiability<K3>,
        Decision<MinimumVertexCover<SimpleGraph, i32>>,
    >(ReductionMode::Turing));
}

#[test]
fn test_find_paths_bounded_limits_depth() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());

    // With tight bound, should find fewer (or no) paths than unbounded
    let bounded = graph.find_paths_up_to_mode_bounded(
        "MaximumIndependentSet",
        &src,
        "QUBO",
        &dst,
        ReductionMode::Witness,
        100,
        Some(2),
    );
    let unbounded = graph.find_paths_up_to_mode_bounded(
        "MaximumIndependentSet",
        &src,
        "QUBO",
        &dst,
        ReductionMode::Witness,
        100,
        None,
    );
    assert!(
        bounded.len() <= unbounded.len(),
        "bounded ({}) should find <= unbounded ({}) paths",
        bounded.len(),
        unbounded.len()
    );

    // With bound 0, only direct edges (no intermediates) — MIS→QUBO has no direct edge
    let direct_only = graph.find_paths_up_to_mode_bounded(
        "MaximumIndependentSet",
        &src,
        "QUBO",
        &dst,
        ReductionMode::Witness,
        100,
        Some(0),
    );
    assert!(
        direct_only.is_empty(),
        "MIS→QUBO has no direct edge, so bound=0 should return empty"
    );
}

#[test]
fn test_find_paths_bounded_returns_shortest_when_truncated() {
    use crate::expr::Expr;
    use crate::rules::registry::ReductionOverhead;
    use crate::rules::ReductionEdgeData;

    fn edge() -> ReductionEdgeData {
        fn reduce(_source: &dyn std::any::Any) -> Box<dyn crate::rules::DynReductionResult> {
            Box::new(crate::rules::ReductionAutoCast::<
                crate::models::formula::Satisfiability,
                crate::models::formula::Satisfiability,
            >::new(
                crate::models::formula::Satisfiability::new(0, vec![])
            ))
        }

        ReductionEdgeData {
            overhead: ReductionOverhead::new(vec![("n", Expr::variable("n"))]),
            reduce_fn: Some(reduce),
            reduce_aggregate_fn: None,
            turing: false,
        }
    }

    // Topology where DFS discovery order surfaces a LONG route before the SHORT one.
    // From S the first outgoing edge (S->A) leads into a long chain A->B->C->T, while a
    // later edge S->T is a direct hop. petgraph's DFS explores S->A first, so the
    // 4-edge route is discovered before the 1-edge direct route. With a tight limit,
    // the old `.take(limit)` in discovery order would keep the long route and drop the
    // short one; length-first enumeration must return the short route.
    let graph = ReductionGraph::from_test_edges(
        &["S", "A", "B", "C", "T"],
        &[
            ("S", "A", edge()),
            ("A", "B", edge()),
            ("B", "C", edge()),
            ("C", "T", edge()),
            ("S", "T", edge()),
        ],
    );

    let empty = BTreeMap::new();

    // Sanity: both routes exist when unbounded.
    let all = graph.find_paths_up_to("S", &empty, "T", &empty, 100);
    assert_eq!(all.len(), 2, "expected the direct route and the long chain");

    // With limit 1, the SHORT (direct) route must be the one returned.
    let limited = graph.find_paths_up_to("S", &empty, "T", &empty, 1);
    assert_eq!(limited.len(), 1);
    assert_eq!(
        limited[0].len(),
        1,
        "truncated result must keep the shortest (direct) route, not the long chain"
    );

    // Results are length-sorted (non-decreasing edge counts).
    let lens: Vec<usize> = all.iter().map(|p| p.len()).collect();
    assert!(
        lens.windows(2).all(|w| w[0] <= w[1]),
        "paths must be returned shortest-first, got lengths {lens:?}"
    );
    assert_eq!(lens, vec![1, 4]);
}
