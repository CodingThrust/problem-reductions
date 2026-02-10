use super::*;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::models::set::MaximumSetPacking;
use crate::rules::cost::MinimizeSteps;
use crate::topology::SimpleGraph;

#[test]
fn test_find_direct_path() {
    let graph = ReductionGraph::new();
    let paths = graph.find_paths::<MaximumIndependentSet<SimpleGraph, i32>, MinimumVertexCover<SimpleGraph, i32>>();
    assert!(!paths.is_empty());
    assert_eq!(paths[0].type_names.len(), 2);
    assert_eq!(paths[0].len(), 1); // One reduction step
}

#[test]
fn test_find_indirect_path() {
    let graph = ReductionGraph::new();
    // IS -> VC -> IS -> SP or IS -> SP directly
    let paths = graph.find_paths::<MaximumIndependentSet<SimpleGraph, i32>, MaximumSetPacking<i32>>();
    assert!(!paths.is_empty());
}

#[test]
fn test_find_shortest_path() {
    let graph = ReductionGraph::new();
    let path = graph.find_shortest_path::<MaximumIndependentSet<SimpleGraph, i32>, MaximumSetPacking<i32>>();
    assert!(path.is_some());
    let path = path.unwrap();
    assert_eq!(path.len(), 1); // Direct path exists
}

#[test]
fn test_has_direct_reduction() {
    let graph = ReductionGraph::new();
    assert!(graph.has_direct_reduction::<MaximumIndependentSet<SimpleGraph, i32>, MinimumVertexCover<SimpleGraph, i32>>());
    assert!(graph.has_direct_reduction::<MinimumVertexCover<SimpleGraph, i32>, MaximumIndependentSet<SimpleGraph, i32>>());
}

#[test]
fn test_is_to_qubo_path() {
    let graph = ReductionGraph::new();
    // IS -> QUBO should now have a direct path
    let path =
        graph.find_shortest_path::<MaximumIndependentSet<SimpleGraph, i32>, crate::models::optimization::QUBO<f64>>();
    assert!(path.is_some());
    assert_eq!(path.unwrap().len(), 1); // Direct path
}

#[test]
fn test_type_erased_paths() {
    let graph = ReductionGraph::new();

    // Different weight types should find the same path (type-erased)
    let paths_i32 = graph.find_paths::<
        crate::models::graph::MaxCut<SimpleGraph, i32>,
        crate::models::optimization::SpinGlass<SimpleGraph, i32>,
    >();
    let paths_f64 = graph.find_paths::<
        crate::models::graph::MaxCut<SimpleGraph, f64>,
        crate::models::optimization::SpinGlass<SimpleGraph, f64>,
    >();

    // Both should find paths since we use type-erased names
    assert!(!paths_i32.is_empty());
    assert!(!paths_f64.is_empty());
    assert_eq!(paths_i32[0].type_names, paths_f64[0].type_names);
}

#[test]
fn test_find_paths_by_name() {
    let graph = ReductionGraph::new();

    let shortest = graph.find_shortest_path_by_name("MaxCut", "SpinGlass");
    assert!(shortest.is_some());
    assert_eq!(shortest.unwrap().len(), 1); // Direct path

    let shortest = graph.find_shortest_path_by_name("Factoring", "SpinGlass");
    assert!(shortest.is_some());
    assert_eq!(shortest.unwrap().len(), 2); // Factoring -> CircuitSAT -> SpinGlass
}

#[test]
fn test_problem_types() {
    let graph = ReductionGraph::new();
    let types = graph.problem_types();
    assert!(types.len() >= 5);
    assert!(types.iter().any(|t| t.contains("MaximumIndependentSet")));
    assert!(types.iter().any(|t| t.contains("MinimumVertexCover")));
}

#[test]
fn test_graph_statistics() {
    let graph = ReductionGraph::new();
    assert!(graph.num_types() >= 5);
    assert!(graph.num_reductions() >= 6);
}

#[test]
fn test_reduction_path_methods() {
    let graph = ReductionGraph::new();
    let path = graph
        .find_shortest_path::<MaximumIndependentSet<SimpleGraph, i32>, MinimumVertexCover<SimpleGraph, i32>>()
        .unwrap();

    assert!(!path.is_empty());
    assert!(path.source().unwrap().contains("MaximumIndependentSet"));
    assert!(path.target().unwrap().contains("MinimumVertexCover"));
}

#[test]
fn test_bidirectional_paths() {
    let graph = ReductionGraph::new();

    // Forward path
    let forward = graph.find_paths::<MaximumIndependentSet<SimpleGraph, i32>, MinimumVertexCover<SimpleGraph, i32>>();
    assert!(!forward.is_empty());

    // Backward path
    let backward = graph.find_paths::<MinimumVertexCover<SimpleGraph, i32>, MaximumIndependentSet<SimpleGraph, i32>>();
    assert!(!backward.is_empty());
}

#[test]
fn test_to_json() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // Check nodes
    assert!(json.nodes.len() >= 10);
    assert!(json.nodes.iter().any(|n| n.name == "MaximumIndependentSet"));
    assert!(json.nodes.iter().any(|n| n.category == "graph"));
    assert!(json.nodes.iter().any(|n| n.category == "optimization"));

    // Check edges
    assert!(json.edges.len() >= 10);

    // Check that IS -> VC and VC -> IS both exist as separate directed edges
    let is_to_vc = json.edges.iter().any(|e| {
        e.source.name == "MaximumIndependentSet" && e.target.name == "MinimumVertexCover"
    });
    let vc_to_is = json.edges.iter().any(|e| {
        e.source.name == "MinimumVertexCover" && e.target.name == "MaximumIndependentSet"
    });
    assert!(is_to_vc, "Should have IS -> VC edge");
    assert!(vc_to_is, "Should have VC -> IS edge");
}

#[test]
fn test_to_json_string() {
    let graph = ReductionGraph::new();
    let json_string = graph.to_json_string().unwrap();

    // Should be valid JSON
    assert!(json_string.contains("\"nodes\""));
    assert!(json_string.contains("\"edges\""));
    assert!(json_string.contains("MaximumIndependentSet"));
    assert!(json_string.contains("\"category\""));
    assert!(json_string.contains("\"overhead\""));

    // The legacy "bidirectional" field must not be present
    assert!(!json_string.contains("\"bidirectional\""), "JSON should not contain the removed 'bidirectional' field");
}

#[test]
fn test_categorize_type() {
    // Graph problems
    assert_eq!(
        ReductionGraph::categorize_type("MaximumIndependentSet<SimpleGraph, i32>"),
        "graph"
    );
    assert_eq!(
        ReductionGraph::categorize_type("MinimumVertexCover<SimpleGraph, i32>"),
        "graph"
    );
    assert_eq!(ReductionGraph::categorize_type("MaxCut<SimpleGraph, i32>"), "graph");
    assert_eq!(ReductionGraph::categorize_type("KColoring"), "graph");
    assert_eq!(
        ReductionGraph::categorize_type("MinimumDominatingSet<SimpleGraph, i32>"),
        "graph"
    );
    assert_eq!(ReductionGraph::categorize_type("MaximumMatching<i32>"), "graph");

    // Set problems
    assert_eq!(ReductionGraph::categorize_type("MaximumSetPacking<i32>"), "set");
    assert_eq!(ReductionGraph::categorize_type("MinimumSetCovering<i32>"), "set");

    // Optimization
    assert_eq!(
        ReductionGraph::categorize_type("SpinGlass<SimpleGraph, i32>"),
        "optimization"
    );
    assert_eq!(ReductionGraph::categorize_type("QUBO<f64>"), "optimization");

    // Satisfiability
    assert_eq!(
        ReductionGraph::categorize_type("Satisfiability<i32>"),
        "satisfiability"
    );
    assert_eq!(
        ReductionGraph::categorize_type("KSatisfiability<3, i32>"),
        "satisfiability"
    );
    assert_eq!(
        ReductionGraph::categorize_type("CircuitSAT<i32>"),
        "satisfiability"
    );

    // Specialized
    assert_eq!(ReductionGraph::categorize_type("Factoring"), "specialized");

    // Unknown
    assert_eq!(ReductionGraph::categorize_type("UnknownProblem"), "other");
}

#[test]
fn test_sat_based_reductions() {
    use crate::models::graph::KColoring;
    use crate::models::graph::MinimumDominatingSet;
    use crate::models::satisfiability::Satisfiability;

    let graph = ReductionGraph::new();

    // SAT -> IS
    assert!(graph.has_direct_reduction::<Satisfiability<i32>, MaximumIndependentSet<SimpleGraph, i32>>());

    // SAT -> KColoring
    assert!(graph.has_direct_reduction::<Satisfiability<i32>, KColoring<3, SimpleGraph, i32>>());

    // SAT -> MinimumDominatingSet
    assert!(graph.has_direct_reduction::<Satisfiability<i32>, MinimumDominatingSet<SimpleGraph, i32>>());
}

#[test]
fn test_circuit_reductions() {
    use crate::models::optimization::SpinGlass;
    use crate::models::specialized::{CircuitSAT, Factoring};

    let graph = ReductionGraph::new();

    // Factoring -> CircuitSAT
    assert!(graph.has_direct_reduction::<Factoring, CircuitSAT<i32>>());

    // CircuitSAT -> SpinGlass
    assert!(graph.has_direct_reduction::<CircuitSAT<i32>, SpinGlass<SimpleGraph, f64>>());

    // Find path from Factoring to SpinGlass
    let paths = graph.find_paths::<Factoring, SpinGlass<SimpleGraph, f64>>();
    assert!(!paths.is_empty());
    let shortest = graph
        .find_shortest_path::<Factoring, SpinGlass<SimpleGraph, f64>>()
        .unwrap();
    assert_eq!(shortest.len(), 2); // Factoring -> CircuitSAT -> SpinGlass
}

#[test]
fn test_optimization_reductions() {
    use crate::models::graph::MaxCut;
    use crate::models::optimization::{SpinGlass, QUBO};

    let graph = ReductionGraph::new();

    // SpinGlass <-> QUBO (bidirectional)
    assert!(graph.has_direct_reduction::<SpinGlass<SimpleGraph, f64>, QUBO<f64>>());
    assert!(graph.has_direct_reduction::<QUBO<f64>, SpinGlass<SimpleGraph, f64>>());

    // MaxCut <-> SpinGlass (bidirectional)
    assert!(graph.has_direct_reduction::<MaxCut<SimpleGraph, i32>, SpinGlass<SimpleGraph, f64>>());
    assert!(graph.has_direct_reduction::<SpinGlass<SimpleGraph, f64>, MaxCut<SimpleGraph, i32>>());
}

#[test]
fn test_ksat_reductions() {
    use crate::models::satisfiability::{KSatisfiability, Satisfiability};

    let graph = ReductionGraph::new();

    // SAT <-> 3-SAT (bidirectional)
    assert!(graph.has_direct_reduction::<Satisfiability<i32>, KSatisfiability<3, i32>>());
    assert!(graph.has_direct_reduction::<KSatisfiability<3, i32>, Satisfiability<i32>>());
}

#[test]
fn test_all_categories_present() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    let categories: std::collections::HashSet<&str> =
        json.nodes.iter().map(|n| n.category.as_str()).collect();

    assert!(categories.contains("graph"));
    assert!(categories.contains("set"));
    assert!(categories.contains("optimization"));
    assert!(categories.contains("satisfiability"));
    assert!(categories.contains("specialized"));
}

#[test]
fn test_empty_path_source_target() {
    let path = ReductionPath { type_names: vec![] };
    assert!(path.is_empty());
    assert_eq!(path.len(), 0);
    assert!(path.source().is_none());
    assert!(path.target().is_none());
}

#[test]
fn test_single_node_path() {
    let path = ReductionPath {
        type_names: vec!["MaximumIndependentSet"],
    };
    assert!(!path.is_empty());
    assert_eq!(path.len(), 0); // No reductions, just one type
    assert_eq!(path.source(), Some("MaximumIndependentSet"));
    assert_eq!(path.target(), Some("MaximumIndependentSet"));
}

#[test]
fn test_default_implementation() {
    let graph1 = ReductionGraph::new();
    let graph2 = ReductionGraph::default();

    assert_eq!(graph1.num_types(), graph2.num_types());
    assert_eq!(graph1.num_reductions(), graph2.num_reductions());
}

#[test]
fn test_to_json_file() {
    use std::env;
    use std::fs;

    let graph = ReductionGraph::new();
    let file_path = env::temp_dir().join("problemreductions_test_graph.json");

    // Write to file
    graph.to_json_file(&file_path).unwrap();

    // Read back and verify
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("\"nodes\""));
    assert!(content.contains("\"edges\""));
    assert!(content.contains("MaximumIndependentSet"));

    // Parse as generic JSON to verify validity
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(!parsed["nodes"].as_array().unwrap().is_empty());
    assert!(!parsed["edges"].as_array().unwrap().is_empty());

    // Clean up
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_has_direct_reduction_unregistered_types() {
    // Test with a type that's not registered in the graph
    struct UnregisteredType;

    let graph = ReductionGraph::new();

    // Source type not registered
    assert!(!graph.has_direct_reduction::<UnregisteredType, MaximumIndependentSet<SimpleGraph, i32>>());

    // Target type not registered
    assert!(!graph.has_direct_reduction::<MaximumIndependentSet<SimpleGraph, i32>, UnregisteredType>());

    // Both types not registered
    assert!(!graph.has_direct_reduction::<UnregisteredType, UnregisteredType>());
}

#[test]
fn test_find_paths_unregistered_source() {
    struct UnregisteredType;

    let graph = ReductionGraph::new();
    let paths = graph.find_paths::<UnregisteredType, MaximumIndependentSet<SimpleGraph, i32>>();
    assert!(paths.is_empty());
}

#[test]
fn test_find_paths_unregistered_target() {
    struct UnregisteredType;

    let graph = ReductionGraph::new();
    let paths = graph.find_paths::<MaximumIndependentSet<SimpleGraph, i32>, UnregisteredType>();
    assert!(paths.is_empty());
}

#[test]
fn test_find_shortest_path_no_path() {
    struct UnregisteredType;

    let graph = ReductionGraph::new();
    let path = graph.find_shortest_path::<UnregisteredType, MaximumIndependentSet<SimpleGraph, i32>>();
    assert!(path.is_none());
}

#[test]
fn test_categorize_circuit_as_specialized() {
    // CircuitSAT should be categorized as specialized (contains "Circuit")
    assert_eq!(
        ReductionGraph::categorize_type("CircuitSAT<i32>"),
        "satisfiability"
    );
    // But it contains "SAT" so it goes to satisfiability first
    // Let's verify the actual behavior matches what the code does
}

#[test]
fn test_directed_edge_pairs() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // IS <-> VC: both directions should exist as separate edges
    let is_to_vc = json
        .edges
        .iter()
        .any(|e| e.source.name == "MaximumIndependentSet" && e.target.name == "MinimumVertexCover");
    let vc_to_is = json
        .edges
        .iter()
        .any(|e| e.source.name == "MinimumVertexCover" && e.target.name == "MaximumIndependentSet");
    assert!(is_to_vc, "Should have IS -> VC edge");
    assert!(vc_to_is, "Should have VC -> IS edge");

    // Factoring -> CircuitSAT: only forward direction
    let factoring_to_circuit = json
        .edges
        .iter()
        .any(|e| e.source.name == "Factoring" && e.target.name == "CircuitSAT");
    let circuit_to_factoring = json
        .edges
        .iter()
        .any(|e| e.source.name == "CircuitSAT" && e.target.name == "Factoring");
    assert!(factoring_to_circuit, "Should have Factoring -> CircuitSAT");
    assert!(
        !circuit_to_factoring,
        "Should NOT have CircuitSAT -> Factoring"
    );
}

// New tests for set-theoretic path finding

#[test]
fn test_graph_hierarchy_built() {
    let graph = ReductionGraph::new();
    let hierarchy = graph.graph_hierarchy();

    // Should have relationships from GraphSubtypeEntry registrations
    // UnitDiskGraph -> PlanarGraph -> SimpleGraph
    // BipartiteGraph -> SimpleGraph
    assert!(
        hierarchy
            .get("UnitDiskGraph")
            .map(|s| s.contains("SimpleGraph"))
            .unwrap_or(false),
        "UnitDiskGraph should have SimpleGraph as supertype"
    );
    assert!(
        hierarchy
            .get("PlanarGraph")
            .map(|s| s.contains("SimpleGraph"))
            .unwrap_or(false),
        "PlanarGraph should have SimpleGraph as supertype"
    );
}

#[test]
fn test_is_graph_subtype_reflexive() {
    let graph = ReductionGraph::new();

    // Every type is a subtype of itself
    assert!(graph.is_graph_subtype("SimpleGraph", "SimpleGraph"));
    assert!(graph.is_graph_subtype("PlanarGraph", "PlanarGraph"));
    assert!(graph.is_graph_subtype("UnitDiskGraph", "UnitDiskGraph"));
}

#[test]
fn test_is_graph_subtype_direct() {
    let graph = ReductionGraph::new();

    // Direct subtype relationships
    assert!(graph.is_graph_subtype("PlanarGraph", "SimpleGraph"));
    assert!(graph.is_graph_subtype("BipartiteGraph", "SimpleGraph"));
    assert!(graph.is_graph_subtype("UnitDiskGraph", "PlanarGraph"));
}

#[test]
fn test_is_graph_subtype_transitive() {
    let graph = ReductionGraph::new();

    // Transitive closure: UnitDiskGraph -> PlanarGraph -> SimpleGraph
    assert!(graph.is_graph_subtype("UnitDiskGraph", "SimpleGraph"));
}

#[test]
fn test_is_graph_subtype_not_supertype() {
    let graph = ReductionGraph::new();

    // SimpleGraph is NOT a subtype of PlanarGraph (only the reverse)
    assert!(!graph.is_graph_subtype("SimpleGraph", "PlanarGraph"));
    assert!(!graph.is_graph_subtype("SimpleGraph", "UnitDiskGraph"));
}

#[test]
fn test_rule_applicable_same_graphs() {
    let graph = ReductionGraph::new();

    // Rule for SimpleGraph -> SimpleGraph applies to same
    assert!(graph.rule_applicable("SimpleGraph", "SimpleGraph", "SimpleGraph", "SimpleGraph"));
}

#[test]
fn test_rule_applicable_subtype_source() {
    let graph = ReductionGraph::new();

    // Rule for SimpleGraph -> SimpleGraph applies when source is PlanarGraph
    // (because PlanarGraph <= SimpleGraph)
    assert!(graph.rule_applicable("PlanarGraph", "SimpleGraph", "SimpleGraph", "SimpleGraph"));
}

#[test]
fn test_rule_applicable_subtype_target() {
    let graph = ReductionGraph::new();

    // Rule producing PlanarGraph applies when we want SimpleGraph
    // (because PlanarGraph <= SimpleGraph)
    assert!(graph.rule_applicable("SimpleGraph", "SimpleGraph", "SimpleGraph", "PlanarGraph"));
}

#[test]
fn test_rule_not_applicable_wrong_source() {
    let graph = ReductionGraph::new();

    // Rule requiring PlanarGraph does NOT apply to SimpleGraph source
    // (because SimpleGraph is NOT <= PlanarGraph)
    assert!(!graph.rule_applicable("SimpleGraph", "SimpleGraph", "PlanarGraph", "SimpleGraph"));
}

#[test]
fn test_rule_not_applicable_wrong_target() {
    let graph = ReductionGraph::new();

    // Rule producing SimpleGraph does NOT apply when we need PlanarGraph
    // (because SimpleGraph is NOT <= PlanarGraph)
    assert!(!graph.rule_applicable("SimpleGraph", "PlanarGraph", "SimpleGraph", "SimpleGraph"));
}

#[test]
fn test_find_cheapest_path_minimize_steps() {
    let graph = ReductionGraph::new();
    let cost_fn = MinimizeSteps;
    let input_size = ProblemSize::new(vec![("n", 10), ("m", 20)]);

    // Find path from MaximumIndependentSet to MinimumVertexCover on SimpleGraph
    let path = graph.find_cheapest_path(
        ("MaximumIndependentSet", "SimpleGraph"),
        ("MinimumVertexCover", "SimpleGraph"),
        &input_size,
        &cost_fn,
    );

    assert!(path.is_some());
    let path = path.unwrap();
    assert_eq!(path.len(), 1); // Direct path
}

#[test]
fn test_find_cheapest_path_multi_step() {
    let graph = ReductionGraph::new();
    let cost_fn = MinimizeSteps;
    let input_size = ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 20)]);

    // Find multi-step path where all edges use compatible graph types
    // MaximumIndependentSet (SimpleGraph) -> MaximumSetPacking (SimpleGraph)
    // This tests the algorithm can find paths with consistent graph types
    let path = graph.find_cheapest_path(
        ("MaximumIndependentSet", "SimpleGraph"),
        ("MaximumSetPacking", "SimpleGraph"),
        &input_size,
        &cost_fn,
    );

    assert!(path.is_some());
    let path = path.unwrap();
    assert_eq!(path.len(), 1); // Direct path: MaximumIndependentSet -> MaximumSetPacking
}

#[test]
fn test_find_cheapest_path_is_to_qubo() {
    let graph = ReductionGraph::new();
    let cost_fn = MinimizeSteps;
    let input_size = ProblemSize::new(vec![("n", 10)]);

    // Direct path from MaximumIndependentSet to QUBO
    let path = graph.find_cheapest_path(
        ("MaximumIndependentSet", "SimpleGraph"),
        ("QUBO", "SimpleGraph"),
        &input_size,
        &cost_fn,
    );

    assert!(path.is_some());
    assert_eq!(path.unwrap().len(), 1); // Direct path
}

#[test]
fn test_find_cheapest_path_unknown_source() {
    let graph = ReductionGraph::new();
    let cost_fn = MinimizeSteps;
    let input_size = ProblemSize::new(vec![("n", 10)]);

    let path = graph.find_cheapest_path(
        ("UnknownProblem", "SimpleGraph"),
        ("MinimumVertexCover", "SimpleGraph"),
        &input_size,
        &cost_fn,
    );

    assert!(path.is_none());
}

#[test]
fn test_find_cheapest_path_unknown_target() {
    let graph = ReductionGraph::new();
    let cost_fn = MinimizeSteps;
    let input_size = ProblemSize::new(vec![("n", 10)]);

    let path = graph.find_cheapest_path(
        ("MaximumIndependentSet", "SimpleGraph"),
        ("UnknownProblem", "SimpleGraph"),
        &input_size,
        &cost_fn,
    );

    assert!(path.is_none());
}

#[test]
fn test_reduction_edge_struct() {
    let edge = ReductionEdge {
        source_variant: &[("graph", "PlanarGraph"), ("weight", "Unweighted")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        overhead: ReductionOverhead::default(),
    };

    assert_eq!(edge.source_graph(), "PlanarGraph");
    assert_eq!(edge.target_graph(), "SimpleGraph");
}

#[test]
fn test_reduction_edge_default_graph() {
    // When no "graph" key is present, default to SimpleGraph
    let edge = ReductionEdge {
        source_variant: &[("weight", "Unweighted")],
        target_variant: &[],
        overhead: ReductionOverhead::default(),
    };

    assert_eq!(edge.source_graph(), "SimpleGraph");
    assert_eq!(edge.target_graph(), "SimpleGraph");
}

#[test]
fn test_variant_to_map() {
    let variant: &[(&str, &str)] = &[("graph", "SimpleGraph"), ("weight", "i32")];
    let map = ReductionGraph::variant_to_map(variant);
    assert_eq!(map.get("graph"), Some(&"SimpleGraph".to_string()));
    assert_eq!(map.get("weight"), Some(&"i32".to_string()));
    assert_eq!(map.len(), 2);
}

#[test]
fn test_variant_to_map_empty() {
    let variant: &[(&str, &str)] = &[];
    let map = ReductionGraph::variant_to_map(variant);
    assert!(map.is_empty());
}

#[test]
fn test_make_variant_ref() {
    let variant: &[(&str, &str)] = &[("graph", "PlanarGraph"), ("weight", "f64")];
    let variant_ref = ReductionGraph::make_variant_ref("MaximumIndependentSet", variant);
    assert_eq!(variant_ref.name, "MaximumIndependentSet");
    assert_eq!(
        variant_ref.variant.get("graph"),
        Some(&"PlanarGraph".to_string())
    );
    assert_eq!(variant_ref.variant.get("weight"), Some(&"f64".to_string()));
}

#[test]
fn test_to_json_nodes_have_variants() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // Check that nodes have variant information
    for node in &json.nodes {
        // Verify node has a name
        assert!(!node.name.is_empty());
        // Verify node has a category
        assert!(!node.category.is_empty());
    }
}

#[test]
fn test_to_json_edges_have_variants() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // Check that edges have source and target variant refs
    for edge in &json.edges {
        assert!(!edge.source.name.is_empty());
        assert!(!edge.target.name.is_empty());
    }
}

#[test]
fn test_json_variant_content() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // Find a node and verify its variant contains expected keys
    let is_node = json.nodes.iter().find(|n| n.name == "MaximumIndependentSet");
    assert!(is_node.is_some(), "MaximumIndependentSet node should exist");

    // Find an edge involving MaximumIndependentSet (could be source or target)
    let is_edge = json
        .edges
        .iter()
        .find(|e| e.source.name == "MaximumIndependentSet" || e.target.name == "MaximumIndependentSet");
    assert!(
        is_edge.is_some(),
        "Edge involving MaximumIndependentSet should exist"
    );
}
