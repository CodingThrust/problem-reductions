use super::*;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::models::set::MaximumSetPacking;
use crate::rules::cost::MinimizeSteps;
use crate::rules::graph::classify_problem_category;
use crate::topology::SimpleGraph;

#[test]
fn test_resolved_path_basic_structure() {
    use crate::rules::graph::{EdgeKind, ReductionStep, ResolvedPath};
    use std::collections::BTreeMap;

    let steps = vec![
        ReductionStep {
            name: "A".to_string(),
            variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
        },
        ReductionStep {
            name: "B".to_string(),
            variant: BTreeMap::from([("weight".to_string(), "f64".to_string())]),
        },
    ];
    let edges = vec![EdgeKind::Reduction {
        overhead: Default::default(),
    }];
    let path = ResolvedPath {
        steps: steps.clone(),
        edges,
    };

    assert_eq!(path.len(), 1);
    assert_eq!(path.num_reductions(), 1);
    assert_eq!(path.num_casts(), 0);
    assert_eq!(path.steps[0].name, "A");
    assert_eq!(path.steps[1].name, "B");
}

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
    let paths =
        graph.find_paths::<MaximumIndependentSet<SimpleGraph, i32>, MaximumSetPacking<i32>>();
    assert!(!paths.is_empty());
}

#[test]
fn test_find_shortest_path() {
    let graph = ReductionGraph::new();
    let path = graph
        .find_shortest_path::<MaximumIndependentSet<SimpleGraph, i32>, MaximumSetPacking<i32>>();
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
fn test_unknown_name_returns_empty() {
    let graph = ReductionGraph::new();

    // Unknown source
    assert!(!graph.has_direct_reduction_by_name("UnknownProblem", "MaximumIndependentSet"));
    // Unknown target
    assert!(!graph.has_direct_reduction_by_name("MaximumIndependentSet", "UnknownProblem"));
    // Both unknown
    assert!(!graph.has_direct_reduction_by_name("UnknownA", "UnknownB"));

    // find_paths with unknown name
    assert!(graph
        .find_paths_by_name("UnknownProblem", "MaximumIndependentSet")
        .is_empty());
    assert!(graph
        .find_paths_by_name("MaximumIndependentSet", "UnknownProblem")
        .is_empty());

    // find_shortest_path with unknown name
    assert!(graph
        .find_shortest_path_by_name("UnknownProblem", "MaximumIndependentSet")
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

// New tests for set-theoretic path finding

#[test]
fn test_graph_hierarchy_built() {
    let graph = ReductionGraph::new();
    let hierarchy = graph.graph_hierarchy();

    // Should have relationships from VariantTypeEntry registrations
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
    assert!(graph.is_graph_subtype("KingsSubgraph", "UnitDiskGraph"));
    assert!(graph.is_graph_subtype("UnitDiskGraph", "SimpleGraph"));
    assert!(graph.is_graph_subtype("SimpleGraph", "HyperGraph"));
}

#[test]
fn test_is_graph_subtype_transitive() {
    let graph = ReductionGraph::new();

    // Transitive closure: KingsSubgraph -> UnitDiskGraph -> SimpleGraph -> HyperGraph
    assert!(graph.is_graph_subtype("KingsSubgraph", "SimpleGraph"));
    assert!(graph.is_graph_subtype("KingsSubgraph", "HyperGraph"));
    assert!(graph.is_graph_subtype("UnitDiskGraph", "HyperGraph"));
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
        source_variant: vec![("graph", "PlanarGraph"), ("weight", "One")],
        target_variant: vec![("graph", "SimpleGraph"), ("weight", "One")],
        overhead: ReductionOverhead::default(),
    };

    assert_eq!(edge.source_graph(), "PlanarGraph");
    assert_eq!(edge.target_graph(), "SimpleGraph");
}

#[test]
fn test_reduction_edge_default_graph() {
    // When no "graph" key is present, default to SimpleGraph
    let edge = ReductionEdge {
        source_variant: vec![("weight", "One")],
        target_variant: vec![],
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
fn test_weight_hierarchy_built() {
    let graph = ReductionGraph::new();
    let hierarchy = graph.weight_hierarchy();
    assert!(
        hierarchy
            .get("One")
            .map(|s| s.contains("i32"))
            .unwrap_or(false),
        "One should have i32 as supertype"
    );
    assert!(
        hierarchy
            .get("i32")
            .map(|s| s.contains("f64"))
            .unwrap_or(false),
        "i32 should have f64 as supertype"
    );
    assert!(
        hierarchy
            .get("One")
            .map(|s| s.contains("f64"))
            .unwrap_or(false),
        "One should transitively have f64 as supertype"
    );
}

#[test]
fn test_is_weight_subtype() {
    let graph = ReductionGraph::new();

    // Reflexive
    assert!(graph.is_weight_subtype("i32", "i32"));
    assert!(graph.is_weight_subtype("One", "One"));

    // Direct
    assert!(graph.is_weight_subtype("One", "i32"));
    assert!(graph.is_weight_subtype("i32", "f64"));

    // Transitive
    assert!(graph.is_weight_subtype("One", "f64"));

    // Not supertypes
    assert!(!graph.is_weight_subtype("i32", "One"));
    assert!(!graph.is_weight_subtype("f64", "i32"));
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

    // KingsSubgraph variants should appear as nodes
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

    // MaxCut/Grid was removed (orphan with no reduction path)
}

#[test]
fn test_natural_edge_graph_relaxation() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // MIS/KingsSubgraph -> MIS/SimpleGraph should exist (graph type relaxation)
    let has_edge = json.edges.iter().any(|e| {
        json.source_node(e).name == "MaximumIndependentSet"
            && json.target_node(e).name == "MaximumIndependentSet"
            && json.source_node(e).variant.get("graph") == Some(&"KingsSubgraph".to_string())
            && json.target_node(e).variant.get("graph") == Some(&"SimpleGraph".to_string())
    });
    assert!(
        has_edge,
        "Natural edge MIS/KingsSubgraph -> MIS/SimpleGraph should exist"
    );
}

#[test]
fn test_natural_edge_triangular_to_simplegraph() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // MIS/TriangularSubgraph -> MIS/SimpleGraph should exist (TriangularSubgraph is a subtype of SimpleGraph)
    let has_edge = json.edges.iter().any(|e| {
        json.source_node(e).name == "MaximumIndependentSet"
            && json.target_node(e).name == "MaximumIndependentSet"
            && json.source_node(e).variant.get("graph") == Some(&"TriangularSubgraph".to_string())
            && json.target_node(e).variant.get("graph") == Some(&"SimpleGraph".to_string())
    });
    assert!(
        has_edge,
        "Natural edge MIS/TriangularSubgraph -> MIS/SimpleGraph should exist"
    );
}

#[test]
fn test_natural_edge_gridgraph_to_unitdisk() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // MIS/KingsSubgraph -> MIS/UnitDiskGraph should exist
    let has_edge = json.edges.iter().any(|e| {
        json.source_node(e).name == "MaximumIndependentSet"
            && json.target_node(e).name == "MaximumIndependentSet"
            && json.source_node(e).variant.get("graph") == Some(&"KingsSubgraph".to_string())
            && json.target_node(e).variant.get("graph") == Some(&"UnitDiskGraph".to_string())
    });
    assert!(
        has_edge,
        "Natural edge MIS/KingsSubgraph -> MIS/UnitDiskGraph should exist"
    );
}

#[test]
fn test_no_natural_edge_wrong_direction() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // No NATURAL edge from SimpleGraph -> KingsSubgraph (wrong direction for graph relaxation).
    // A real reduction edge from SimpleGraph -> KingsSubgraph may exist (unit disk mapping).
    let has_natural_edge = json.edges.iter().any(|e| {
        json.source_node(e).name == "MaximumIndependentSet"
            && json.target_node(e).name == "MaximumIndependentSet"
            && json.source_node(e).variant.get("graph") == Some(&"SimpleGraph".to_string())
            && json.target_node(e).variant.get("graph") == Some(&"KingsSubgraph".to_string())
            && e.doc_path.is_empty() // natural edges have empty doc_path
    });
    assert!(
        !has_natural_edge,
        "Should NOT have natural edge MIS/SimpleGraph -> MIS/KingsSubgraph"
    );
}

#[test]
fn test_no_natural_self_edge() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // No self-edges (same variant to same variant)
    for edge in &json.edges {
        if json.source_node(edge).name == json.target_node(edge).name {
            assert!(
                json.source_node(edge).variant != json.target_node(edge).variant,
                "Should not have self-edge: {} {:?}",
                json.source_node(edge).name,
                json.source_node(edge).variant
            );
        }
    }
}

#[test]
fn test_natural_edge_has_identity_overhead() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // Find a natural edge and verify its overhead is identity (field == formula)
    let natural_edge = json.edges.iter().find(|e| {
        json.source_node(e).name == "MaximumIndependentSet"
            && json.target_node(e).name == "MaximumIndependentSet"
            && json.source_node(e).variant.get("graph") == Some(&"KingsSubgraph".to_string())
            && json.target_node(e).variant.get("graph") == Some(&"SimpleGraph".to_string())
            && json.source_node(e).variant.get("weight") == Some(&"i32".to_string())
            && json.target_node(e).variant.get("weight") == Some(&"i32".to_string())
    });
    assert!(natural_edge.is_some(), "Natural edge should exist");
    let edge = natural_edge.unwrap();
    // Overhead should be identity: each field maps to itself
    assert!(
        !edge.overhead.is_empty(),
        "Natural edge should have identity overhead"
    );
    for o in &edge.overhead {
        assert_eq!(
            o.field, o.formula,
            "Natural edge overhead should be identity: {} != {}",
            o.field, o.formula
        );
    }
}

#[test]
fn test_find_matching_entry_ksat_k3() {
    let graph = ReductionGraph::new();
    let variant_k3: std::collections::BTreeMap<String, String> =
        [("k".to_string(), "K3".to_string())].into();

    let entry = graph.find_best_entry("KSatisfiability", "QUBO", &variant_k3);
    assert!(entry.is_some());
    let entry = entry.unwrap();
    let source_var = &entry.source_variant;
    let overhead = &entry.overhead;
    // K=3 overhead has num_clauses term; K=2 does not
    assert!(overhead
        .output_size
        .iter()
        .any(|(field, _)| *field == "num_vars"));
    // K=3 overhead: poly!(num_vars) + poly!(num_clauses) → two terms total
    let num_vars_poly = &overhead
        .output_size
        .iter()
        .find(|(f, _)| *f == "num_vars")
        .unwrap()
        .1;
    assert!(
        num_vars_poly.terms.len() >= 2,
        "K=3 overhead should have num_vars + num_clauses"
    );
    // Verify the source variant matches k=K3
    assert_eq!(source_var.get("k"), Some(&"K3".to_string()));
}

#[test]
fn test_find_matching_entry_ksat_k2() {
    let graph = ReductionGraph::new();
    let variant_k2: std::collections::BTreeMap<String, String> =
        [("k".to_string(), "K2".to_string())].into();

    let entry = graph.find_best_entry("KSatisfiability", "QUBO", &variant_k2);
    assert!(entry.is_some());
    let entry = entry.unwrap();
    let overhead = &entry.overhead;
    // K=2 overhead: just poly!(num_vars) → one term
    let num_vars_poly = &overhead
        .output_size
        .iter()
        .find(|(f, _)| *f == "num_vars")
        .unwrap()
        .1;
    assert_eq!(
        num_vars_poly.terms.len(),
        1,
        "K=2 overhead should have only num_vars"
    );
}

#[test]
fn test_find_matching_entry_no_match() {
    let graph = ReductionGraph::new();
    let variant: std::collections::BTreeMap<String, String> =
        [("k".to_string(), "K99".to_string())].into();

    // k=K99 is not a subtype of K2 or K3
    let entry = graph.find_best_entry("KSatisfiability", "QUBO", &variant);
    assert!(entry.is_none());
}

#[test]
fn test_resolve_path_direct_same_variant() {
    use std::collections::BTreeMap;
    let graph = ReductionGraph::new();

    // MIS(SimpleGraph, i32) → VC(SimpleGraph, i32) — no cast needed
    let name_path = graph
        .find_shortest_path::<
            MaximumIndependentSet<SimpleGraph, i32>,
            MinimumVertexCover<SimpleGraph, i32>,
        >()
        .unwrap();

    let source_variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let target_variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);

    let resolved = graph
        .resolve_path(&name_path, &source_variant, &target_variant)
        .unwrap();

    assert_eq!(resolved.num_reductions(), 1);
    assert_eq!(resolved.num_casts(), 0);
    assert_eq!(resolved.steps.len(), 2);
    assert_eq!(resolved.steps[0].name, "MaximumIndependentSet");
    assert_eq!(resolved.steps[1].name, "MinimumVertexCover");
}

#[test]
fn test_resolve_path_with_natural_cast() {
    use crate::topology::KingsSubgraph;
    use std::collections::BTreeMap;
    let graph = ReductionGraph::new();

    // MIS(KingsSubgraph) → VC(SimpleGraph) — needs a natural cast MIS(KingsSubgraph)→MIS(SimpleGraph)
    let name_path = graph
        .find_shortest_path::<
            MaximumIndependentSet<KingsSubgraph, i32>,
            MinimumVertexCover<SimpleGraph, i32>,
        >()
        .unwrap();

    let source_variant = BTreeMap::from([
        ("graph".to_string(), "KingsSubgraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let target_variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);

    let resolved = graph
        .resolve_path(&name_path, &source_variant, &target_variant)
        .unwrap();

    // Should be: MIS(KingsSubgraph) --NaturalCast--> MIS(SimpleGraph) --Reduction--> VC(SimpleGraph)
    assert_eq!(resolved.num_reductions(), 1);
    assert_eq!(resolved.num_casts(), 1);
    assert_eq!(resolved.steps.len(), 3);
    assert_eq!(resolved.steps[0].name, "MaximumIndependentSet");
    assert_eq!(
        resolved.steps[0].variant.get("graph").unwrap(),
        "KingsSubgraph"
    );
    assert_eq!(resolved.steps[1].name, "MaximumIndependentSet");
    assert_eq!(
        resolved.steps[1].variant.get("graph").unwrap(),
        "SimpleGraph"
    );
    assert_eq!(resolved.steps[2].name, "MinimumVertexCover");
    assert!(matches!(resolved.edges[0], EdgeKind::NaturalCast));
    assert!(matches!(resolved.edges[1], EdgeKind::Reduction { .. }));
}

#[test]
fn test_resolve_path_ksat_disambiguates() {
    use crate::rules::graph::EdgeKind;
    use std::collections::BTreeMap;
    let graph = ReductionGraph::new();

    let name_path = graph
        .find_shortest_path_by_name("KSatisfiability", "QUBO")
        .unwrap();

    // Resolve with k=K3
    let source_k3 = BTreeMap::from([("k".to_string(), "K3".to_string())]);
    let target = BTreeMap::from([("weight".to_string(), "f64".to_string())]);

    let resolved_k3 = graph.resolve_path(&name_path, &source_k3, &target).unwrap();
    assert_eq!(resolved_k3.num_reductions(), 1);

    // Extract overhead from the reduction edge
    let overhead_k3 = match &resolved_k3.edges.last().unwrap() {
        EdgeKind::Reduction { overhead } => overhead,
        _ => panic!("last edge should be Reduction"),
    };
    // K=3 overhead has 2 terms in num_vars polynomial
    let num_vars_poly_k3 = &overhead_k3
        .output_size
        .iter()
        .find(|(f, _)| *f == "num_vars")
        .unwrap()
        .1;
    assert!(num_vars_poly_k3.terms.len() >= 2);

    // Resolve with k=K2
    let source_k2 = BTreeMap::from([("k".to_string(), "K2".to_string())]);
    let resolved_k2 = graph.resolve_path(&name_path, &source_k2, &target).unwrap();
    let overhead_k2 = match &resolved_k2.edges.last().unwrap() {
        EdgeKind::Reduction { overhead } => overhead,
        _ => panic!("last edge should be Reduction"),
    };
    let num_vars_poly_k2 = &overhead_k2
        .output_size
        .iter()
        .find(|(f, _)| *f == "num_vars")
        .unwrap()
        .1;
    assert_eq!(num_vars_poly_k2.terms.len(), 1);
}

#[test]
fn test_resolve_path_incompatible_returns_none() {
    use std::collections::BTreeMap;
    let graph = ReductionGraph::new();

    let name_path = graph
        .find_shortest_path_by_name("KSatisfiability", "QUBO")
        .unwrap();

    // k=K99 matches neither K2 nor K3
    let source = BTreeMap::from([("k".to_string(), "K99".to_string())]);
    let target = BTreeMap::from([("weight".to_string(), "f64".to_string())]);

    let resolved = graph.resolve_path(&name_path, &source, &target);
    assert!(resolved.is_none());
}

#[test]
fn test_filter_redundant_base_nodes() {
    use std::collections::{BTreeMap, HashSet};

    let mut node_set: HashSet<(String, BTreeMap<String, String>)> = HashSet::new();

    // Base node (empty variant) — should be removed because variant-specific sibling exists
    node_set.insert(("MIS".to_string(), BTreeMap::new()));

    // Variant-specific node
    let mut variant = BTreeMap::new();
    variant.insert("graph".to_string(), "GridGraph".to_string());
    node_set.insert(("MIS".to_string(), variant));

    // Base node with no siblings — should be kept
    node_set.insert(("QUBO".to_string(), BTreeMap::new()));

    filter_redundant_base_nodes(&mut node_set);

    assert_eq!(node_set.len(), 2);
    assert!(!node_set.iter().any(|(name, v)| name == "MIS" && v.is_empty()));
    assert!(node_set.iter().any(|(name, _)| name == "QUBO"));
}

#[test]
fn test_classify_problem_category() {
    assert_eq!(
        classify_problem_category("problemreductions::models::graph::maximum_independent_set"),
        "graph"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::sat::satisfiability"),
        "sat"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::set::maximum_set_packing"),
        "set"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::optimization::qubo"),
        "optimization"
    );
    assert_eq!(
        classify_problem_category("unknown::path"),
        "other"
    );
}
