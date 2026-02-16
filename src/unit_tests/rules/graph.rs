use super::*;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::models::optimization::QUBO;
use crate::models::set::MaximumSetPacking;
use crate::rules::cost::MinimizeSteps;
use crate::rules::graph::{classify_problem_category, ReductionStep};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::ProblemSize;
use std::collections::BTreeMap;

#[test]
fn test_find_direct_path() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let paths = graph.find_all_paths("MaximumIndependentSet", &src, "MinimumVertexCover", &dst);
    assert!(!paths.is_empty());
    // At least one path should be a direct reduction (1 edge = 2 steps)
    let shortest = paths.iter().min_by_key(|p| p.len()).unwrap();
    assert_eq!(shortest.type_names().len(), 2);
    assert_eq!(shortest.len(), 1); // One reduction step
}

#[test]
fn test_find_indirect_path() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    let paths = graph.find_all_paths("MaximumIndependentSet", &src, "MaximumSetPacking", &dst);
    assert!(!paths.is_empty());
}

#[test]
fn test_find_shortest_path() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    let path = graph.find_cheapest_path("MaximumIndependentSet", &src, "MaximumSetPacking", &dst, &ProblemSize::new(vec![]), &MinimizeSteps);
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
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let path = graph.find_cheapest_path("MaximumIndependentSet", &src, "QUBO", &dst, &ProblemSize::new(vec![]), &MinimizeSteps);
    assert!(path.is_some());
    assert_eq!(path.unwrap().len(), 1); // Direct path
}

#[test]
fn test_variant_level_paths() {
    let graph = ReductionGraph::new();

    // Variant-level path: MaxCut<SimpleGraph, i32> -> SpinGlass<SimpleGraph, i32>
    let src = ReductionGraph::variant_to_map(&crate::models::graph::MaxCut::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&crate::models::optimization::SpinGlass::<SimpleGraph, i32>::variant());
    let paths = graph.find_all_paths("MaxCut", &src, "SpinGlass", &dst);
    assert!(!paths.is_empty());
    assert_eq!(paths[0].type_names(), vec!["MaxCut", "SpinGlass"]);

    // Unregistered variant pair returns no paths
    let src_f64 = ReductionGraph::variant_to_map(&crate::models::graph::MaxCut::<SimpleGraph, f64>::variant());
    let dst_f64 = ReductionGraph::variant_to_map(&crate::models::optimization::SpinGlass::<SimpleGraph, f64>::variant());
    let paths_f64 = graph.find_all_paths("MaxCut", &src_f64, "SpinGlass", &dst_f64);
    // No direct MaxCut<f64> -> SpinGlass<f64> reduction registered
    assert!(paths_f64.is_empty());
}

#[test]
fn test_find_shortest_path_variants() {
    let graph = ReductionGraph::new();

    let src = ReductionGraph::variant_to_map(&crate::models::graph::MaxCut::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&crate::models::optimization::SpinGlass::<SimpleGraph, i32>::variant());
    let shortest = graph.find_cheapest_path("MaxCut", &src, "SpinGlass", &dst, &ProblemSize::new(vec![]), &MinimizeSteps);
    assert!(shortest.is_some());
    assert_eq!(shortest.unwrap().len(), 1); // Direct path

    let src = ReductionGraph::variant_to_map(&crate::models::specialized::Factoring::variant());
    let dst = ReductionGraph::variant_to_map(&crate::models::optimization::SpinGlass::<SimpleGraph, i32>::variant());
    let shortest = graph.find_cheapest_path("Factoring", &src, "SpinGlass", &dst, &ProblemSize::new(vec![]), &MinimizeSteps);
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
    // Variant-level nodes should be at least as many as base types
    assert!(graph.num_variant_nodes() >= graph.num_types());
}

#[test]
fn test_reduction_path_methods() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let path = graph
        .find_cheapest_path("MaximumIndependentSet", &src, "MinimumVertexCover", &dst, &ProblemSize::new(vec![]), &MinimizeSteps)
        .unwrap();

    assert!(!path.is_empty());
    assert!(path.source().unwrap().contains("MaximumIndependentSet"));
    assert!(path.target().unwrap().contains("MinimumVertexCover"));
}

#[test]
fn test_bidirectional_paths() {
    let graph = ReductionGraph::new();
    let is_var = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let vc_var = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    // Forward path
    let forward = graph.find_all_paths("MaximumIndependentSet", &is_var, "MinimumVertexCover", &vc_var);
    assert!(!forward.is_empty());

    // Backward path
    let backward = graph.find_all_paths("MinimumVertexCover", &vc_var, "MaximumIndependentSet", &is_var);
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
        json.source_node(e).name == "MaximumIndependentSet"
            && json.target_node(e).name == "MinimumVertexCover"
    });
    let vc_to_is = json.edges.iter().any(|e| {
        json.source_node(e).name == "MinimumVertexCover"
            && json.target_node(e).name == "MaximumIndependentSet"
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
    assert!(
        !json_string.contains("\"bidirectional\""),
        "JSON should not contain the removed 'bidirectional' field"
    );
}

#[test]
fn test_category_from_module_path() {
    assert_eq!(
        ReductionGraph::category_from_module_path(
            "problemreductions::models::graph::maximum_independent_set"
        ),
        "graph"
    );
    assert_eq!(
        ReductionGraph::category_from_module_path(
            "problemreductions::models::set::minimum_set_covering"
        ),
        "set"
    );
    assert_eq!(
        ReductionGraph::category_from_module_path("problemreductions::models::optimization::qubo"),
        "optimization"
    );
    assert_eq!(
        ReductionGraph::category_from_module_path("problemreductions::models::satisfiability::sat"),
        "satisfiability"
    );
    assert_eq!(
        ReductionGraph::category_from_module_path(
            "problemreductions::models::specialized::factoring"
        ),
        "specialized"
    );
    // Fallback for unexpected format
    assert_eq!(
        ReductionGraph::category_from_module_path("foo::bar"),
        "other"
    );
}

#[test]
fn test_doc_path_from_module_path() {
    assert_eq!(
        ReductionGraph::doc_path_from_module_path(
            "problemreductions::models::graph::maximum_independent_set",
            "MaximumIndependentSet"
        ),
        "models/graph/struct.MaximumIndependentSet.html"
    );
    assert_eq!(
        ReductionGraph::doc_path_from_module_path(
            "problemreductions::models::optimization::qubo",
            "QUBO"
        ),
        "models/optimization/struct.QUBO.html"
    );
}

#[test]
fn test_sat_based_reductions() {
    use crate::models::graph::KColoring;
    use crate::models::graph::MinimumDominatingSet;
    use crate::models::satisfiability::Satisfiability;
    use crate::variant::K3;

    let graph = ReductionGraph::new();

    // SAT -> IS
    assert!(graph.has_direct_reduction::<Satisfiability, MaximumIndependentSet<SimpleGraph, i32>>());

    // SAT -> KColoring
    assert!(graph.has_direct_reduction::<Satisfiability, KColoring<K3, SimpleGraph>>());

    // SAT -> MinimumDominatingSet
    assert!(graph.has_direct_reduction::<Satisfiability, MinimumDominatingSet<SimpleGraph, i32>>());
}

#[test]
fn test_circuit_reductions() {
    use crate::models::optimization::SpinGlass;
    use crate::models::specialized::{CircuitSAT, Factoring};

    let graph = ReductionGraph::new();

    // Factoring -> CircuitSAT
    assert!(graph.has_direct_reduction::<Factoring, CircuitSAT>());

    // CircuitSAT -> SpinGlass
    assert!(graph.has_direct_reduction::<CircuitSAT, SpinGlass<SimpleGraph, i32>>());

    // Find path from Factoring to SpinGlass<SimpleGraph, i32>
    let src = ReductionGraph::variant_to_map(&Factoring::variant());
    let dst = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, i32>::variant());
    let paths = graph.find_all_paths("Factoring", &src, "SpinGlass", &dst);
    assert!(!paths.is_empty());
    let shortest = graph.find_cheapest_path("Factoring", &src, "SpinGlass", &dst, &ProblemSize::new(vec![]), &MinimizeSteps).unwrap();
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
    use crate::variant::K3;

    let graph = ReductionGraph::new();

    // SAT <-> 3-SAT (bidirectional)
    assert!(graph.has_direct_reduction::<Satisfiability, KSatisfiability<K3>>());
    assert!(graph.has_direct_reduction::<KSatisfiability<K3>, Satisfiability>());
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
    let path = ReductionPath { steps: vec![] };
    assert!(path.is_empty());
    assert_eq!(path.len(), 0);
    assert!(path.source().is_none());
    assert!(path.target().is_none());
}

#[test]
fn test_single_node_path() {
    use std::collections::BTreeMap;
    let path = ReductionPath {
        steps: vec![ReductionStep {
            name: "MaximumIndependentSet".to_string(),
            variant: BTreeMap::new(),
        }],
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
fn test_unknown_name_returns_empty() {
    let graph = ReductionGraph::new();
    let unknown = BTreeMap::new();
    let is_var = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());

    // Unknown source
    assert!(!graph.has_direct_reduction_by_name("UnknownProblem", "MaximumIndependentSet"));
    // Unknown target
    assert!(!graph.has_direct_reduction_by_name("MaximumIndependentSet", "UnknownProblem"));
    // Both unknown
    assert!(!graph.has_direct_reduction_by_name("UnknownA", "UnknownB"));

    // find_all_paths with unknown name
    assert!(graph
        .find_all_paths("UnknownProblem", &unknown, "MaximumIndependentSet", &is_var)
        .is_empty());
    assert!(graph
        .find_all_paths("MaximumIndependentSet", &is_var, "UnknownProblem", &unknown)
        .is_empty());

    // find_shortest_path with unknown name
    assert!(graph
        .find_cheapest_path("UnknownProblem", &unknown, "MaximumIndependentSet", &is_var, &ProblemSize::new(vec![]), &MinimizeSteps)
        .is_none());
}

#[test]
fn test_category_derived_from_schema() {
    // CircuitSAT's category is derived from its ProblemSchemaEntry module_path
    let graph = ReductionGraph::new();
    let json = graph.to_json();
    let circuit = json.nodes.iter().find(|n| n.name == "CircuitSAT").unwrap();
    assert_eq!(circuit.category, "specialized");
}

#[test]
fn test_directed_edge_pairs() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // IS <-> VC: both directions should exist as separate edges
    let is_to_vc = json.edges.iter().any(|e| {
        json.source_node(e).name == "MaximumIndependentSet"
            && json.target_node(e).name == "MinimumVertexCover"
    });
    let vc_to_is = json.edges.iter().any(|e| {
        json.source_node(e).name == "MinimumVertexCover"
            && json.target_node(e).name == "MaximumIndependentSet"
    });
    assert!(is_to_vc, "Should have IS -> VC edge");
    assert!(vc_to_is, "Should have VC -> IS edge");

    // Factoring -> CircuitSAT: only forward direction
    let factoring_to_circuit = json.edges.iter().any(|e| {
        json.source_node(e).name == "Factoring" && json.target_node(e).name == "CircuitSAT"
    });
    let circuit_to_factoring = json.edges.iter().any(|e| {
        json.source_node(e).name == "CircuitSAT" && json.target_node(e).name == "Factoring"
    });
    assert!(factoring_to_circuit, "Should have Factoring -> CircuitSAT");
    assert!(
        !circuit_to_factoring,
        "Should NOT have CircuitSAT -> Factoring"
    );
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
        assert!(!json.source_node(edge).name.is_empty());
        assert!(!json.target_node(edge).name.is_empty());
    }
}

#[test]
fn test_json_variant_content() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // Find a node and verify its variant contains expected keys
    let is_node = json
        .nodes
        .iter()
        .find(|n| n.name == "MaximumIndependentSet");
    assert!(is_node.is_some(), "MaximumIndependentSet node should exist");

    // Find an edge involving MaximumIndependentSet (could be source or target)
    let is_edge = json.edges.iter().find(|e| {
        json.source_node(e).name == "MaximumIndependentSet"
            || json.target_node(e).name == "MaximumIndependentSet"
    });
    assert!(
        is_edge.is_some(),
        "Edge involving MaximumIndependentSet should exist"
    );
}

#[test]
fn test_reduction_variant_nodes_in_json() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // KingsSubgraph variants should appear as nodes (from explicit cast reductions)
    let mis_kingssubgraph = json.nodes.iter().any(|n| {
        n.name == "MaximumIndependentSet"
            && n.variant.get("graph") == Some(&"KingsSubgraph".to_string())
    });
    assert!(mis_kingssubgraph, "MIS/KingsSubgraph node should exist");

    let mis_unitdisk = json.nodes.iter().any(|n| {
        n.name == "MaximumIndependentSet"
            && n.variant.get("graph") == Some(&"UnitDiskGraph".to_string())
    });
    assert!(mis_unitdisk, "MIS/UnitDiskGraph node should exist");
}

#[test]
fn test_variant_cast_edges_in_json() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // MIS/KingsSubgraph -> MIS/UnitDiskGraph should exist as an explicit cast reduction
    let has_edge = json.edges.iter().any(|e| {
        json.source_node(e).name == "MaximumIndependentSet"
            && json.target_node(e).name == "MaximumIndependentSet"
            && json.source_node(e).variant.get("graph") == Some(&"KingsSubgraph".to_string())
            && json.target_node(e).variant.get("graph") == Some(&"UnitDiskGraph".to_string())
    });
    assert!(
        has_edge,
        "Variant cast edge MIS/KingsSubgraph -> MIS/UnitDiskGraph should exist"
    );
}

#[test]
fn test_no_self_edge() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // No self-edges (same source and target node index)
    for edge in &json.edges {
        assert!(
            edge.source != edge.target,
            "Should not have self-edge at node index {}",
            edge.source
        );
    }
}

#[test]
fn test_edges_have_doc_paths() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // All explicit reduction edges should have non-empty doc_path
    // (since they all come from #[reduction] registrations with module_path)
    for edge in &json.edges {
        assert!(
            !edge.doc_path.is_empty(),
            "Edge from {} to {} should have a doc_path",
            json.source_node(edge).name,
            json.target_node(edge).name
        );
    }
}

#[test]
fn test_find_cheapest_path_minimize_steps() {
    let graph = ReductionGraph::new();
    let cost_fn = MinimizeSteps;
    let input_size = crate::types::ProblemSize::new(vec![("n", 10), ("m", 20)]);
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    let path = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src,
        "MinimumVertexCover",
        &dst,
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
    let input_size = crate::types::ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 20)]);
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());

    let path = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src,
        "MaximumSetPacking",
        &dst,
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
    let input_size = crate::types::ProblemSize::new(vec![("n", 10)]);
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());

    let path = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src,
        "QUBO",
        &dst,
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
    let input_size = crate::types::ProblemSize::new(vec![("n", 10)]);
    let unknown = BTreeMap::new();
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());

    let path = graph.find_cheapest_path(
        "UnknownProblem",
        &unknown,
        "MinimumVertexCover",
        &dst,
        &input_size,
        &cost_fn,
    );

    assert!(path.is_none());
}

#[test]
fn test_find_cheapest_path_unknown_target() {
    let graph = ReductionGraph::new();
    let cost_fn = MinimizeSteps;
    let input_size = crate::types::ProblemSize::new(vec![("n", 10)]);
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let unknown = BTreeMap::new();

    let path = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src,
        "UnknownProblem",
        &unknown,
        &input_size,
        &cost_fn,
    );

    assert!(path.is_none());
}

#[test]
fn test_classify_problem_category() {
    assert_eq!(
        classify_problem_category("problemreductions::models::graph::maximum_independent_set"),
        "graph"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::satisfiability::satisfiability"),
        "satisfiability"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::set::maximum_set_packing"),
        "set"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::optimization::qubo"),
        "optimization"
    );
    assert_eq!(classify_problem_category("unknown::path"), "other");
}

#[test]
fn test_make_executable_direct() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let rpath = graph
        .find_cheapest_path("MaximumIndependentSet", &src, "MinimumVertexCover", &dst, &ProblemSize::new(vec![]), &MinimizeSteps)
        .unwrap();
    let path = graph.make_executable::<
        MaximumIndependentSet<SimpleGraph, i32>,
        MinimumVertexCover<SimpleGraph, i32>,
    >(&rpath);
    assert!(path.is_some());
}

#[test]
fn test_chained_reduction_direct() {
    use crate::solvers::{BruteForce, Solver};
    use crate::traits::Problem;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let rpath = graph
        .find_cheapest_path("MaximumIndependentSet", &src, "MinimumVertexCover", &dst, &ProblemSize::new(vec![]), &MinimizeSteps)
        .unwrap();
    let path = graph
        .make_executable::<
            MaximumIndependentSet<SimpleGraph, i32>,
            MinimumVertexCover<SimpleGraph, i32>,
        >(&rpath)
        .unwrap();

    let problem = MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), vec![1i32; 4]);
    let reduction = path.reduce(&problem);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.find_best(target).unwrap();
    let source_solution = reduction.extract_solution(&target_solution);
    let metric = problem.evaluate(&source_solution);
    assert!(metric.is_valid());
}

#[test]
fn test_chained_reduction_multi_step() {
    use crate::solvers::{BruteForce, Solver};
    use crate::traits::Problem;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    let rpath = graph
        .find_cheapest_path("MaximumIndependentSet", &src, "MaximumSetPacking", &dst, &ProblemSize::new(vec![]), &MinimizeSteps)
        .unwrap();
    let path = graph
        .make_executable::<
            MaximumIndependentSet<SimpleGraph, i32>,
            MaximumSetPacking<i32>,
        >(&rpath)
        .unwrap();

    let problem = MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), vec![1i32; 4]);
    let reduction = path.reduce(&problem);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.find_best(target).unwrap();
    let source_solution = reduction.extract_solution(&target_solution);
    let metric = problem.evaluate(&source_solution);
    assert!(metric.is_valid());
}

#[test]
fn test_chained_reduction_with_variant_casts() {
    use crate::models::satisfiability::{CNFClause, KSatisfiability};
    use crate::rules::MinimizeSteps;
    use crate::solvers::{BruteForce, Solver};
    use crate::topology::UnitDiskGraph;
    use crate::traits::Problem;
    use crate::types::ProblemSize;

    let graph = ReductionGraph::new();

    // MIS<UnitDiskGraph, i32> -> MIS<SimpleGraph, i32> (variant cast) -> MVC<SimpleGraph, i32>
    // Use find_cheapest_path for exact variant matching (not name-based)
    let src_var = ReductionGraph::variant_to_map(&MaximumIndependentSet::<UnitDiskGraph, i32>::variant());
    let dst_var = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let rpath = graph.find_cheapest_path(
        "MaximumIndependentSet", &src_var,
        "MinimumVertexCover", &dst_var,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );
    assert!(
        rpath.is_some(),
        "Should find path from MIS<UnitDiskGraph> to MVC<SimpleGraph> via variant cast"
    );
    let rpath = rpath.unwrap();
    assert!(
        rpath.len() >= 2,
        "Path should cross variant cast boundary (at least 2 steps)"
    );

    let path = graph
        .make_executable::<
            MaximumIndependentSet<UnitDiskGraph, i32>,
            MinimumVertexCover<SimpleGraph, i32>,
        >(&rpath)
        .unwrap();

    // Create a small UnitDiskGraph MIS problem (triangle of close nodes)
    let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (0.5, 0.0), (0.25, 0.4)], 1.0);
    let mis = MaximumIndependentSet::new(udg, vec![1i32, 1, 1]);

    let reduction = path.reduce(&mis);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.find_best(target).unwrap();
    let source_solution = reduction.extract_solution(&target_solution);
    let metric = mis.evaluate(&source_solution);
    assert!(metric.is_valid());

    // Also test the KSat<K3> -> Sat -> MIS multi-step path
    // Use find_cheapest_path for exact variant matching (not name-based
    // and may pick a path through a different KSat variant)
    let ksat_src = ReductionGraph::variant_to_map(&KSatisfiability::<crate::variant::K3>::variant());
    let ksat_dst = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let ksat_rpath = graph.find_cheapest_path(
        "KSatisfiability", &ksat_src,
        "MaximumIndependentSet", &ksat_dst,
        &crate::types::ProblemSize::new(vec![]),
        &crate::rules::MinimizeSteps,
    );
    assert!(
        ksat_rpath.is_some(),
        "Should find path from KSat<K3> to MIS"
    );
    let ksat_rpath = ksat_rpath.unwrap();
    let ksat_path = graph
        .make_executable::<
            KSatisfiability<crate::variant::K3>,
            MaximumIndependentSet<SimpleGraph, i32>,
        >(&ksat_rpath)
        .unwrap();

    // Create a 3-SAT formula
    let ksat = KSatisfiability::<crate::variant::K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, -2, -3]),
            CNFClause::new(vec![-1, 2, 3]),
            CNFClause::new(vec![1, -2, 3]),
        ],
    );

    let reduction = ksat_path.reduce(&ksat);
    let target = reduction.target_problem();

    let target_solution = solver.find_best(target).unwrap();
    let original_solution = reduction.extract_solution(&target_solution);

    // Verify the extracted solution satisfies the original 3-SAT formula
    assert!(ksat.evaluate(&original_solution));
}
