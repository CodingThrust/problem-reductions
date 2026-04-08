use super::*;
use crate::models::algebraic::{ILP, QUBO};
use crate::models::formula::{
    CircuitSAT, Maximum2Satisfiability, NAESatisfiability, Satisfiability,
};
use crate::models::graph::MaxCut;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::models::misc::Knapsack;
use crate::models::set::MaximumSetPacking;
use crate::rules::cost::{Minimize, MinimizeSteps};
use crate::rules::graph::{classify_problem_category, ReductionMode, ReductionStep};
use crate::rules::registry::{EdgeCapabilities, ReductionEntry};
use crate::rules::traits::{AggregateReductionResult, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{One, ProblemSize, Sum};
use petgraph::graph::DiGraph;
use serde_json::json;
use std::any::Any;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
struct AggregateChainSource;

#[derive(Clone)]
struct AggregateChainMiddle;

#[derive(Clone)]
struct AggregateChainTarget;

#[derive(Clone)]
struct NaturalVariantProblem;

impl Problem for AggregateChainSource {
    const NAME: &'static str = "AggregateChainSource";
    type Value = Sum<u64>;

    fn dims(&self) -> Vec<usize> {
        vec![1]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Sum(config.iter().sum::<usize>() as u64)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl Problem for AggregateChainMiddle {
    const NAME: &'static str = "AggregateChainMiddle";
    type Value = Sum<u64>;

    fn dims(&self) -> Vec<usize> {
        vec![1]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Sum(config.iter().sum::<usize>() as u64)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl Problem for AggregateChainTarget {
    const NAME: &'static str = "AggregateChainTarget";
    type Value = Sum<u64>;

    fn dims(&self) -> Vec<usize> {
        vec![1]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Sum(config.iter().sum::<usize>() as u64)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl Problem for NaturalVariantProblem {
    const NAME: &'static str = "NaturalVariantProblem";
    type Value = Sum<u64>;

    fn dims(&self) -> Vec<usize> {
        vec![1]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Sum(config.iter().sum::<usize>() as u64)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

struct SourceToMiddleAggregateResult {
    target: AggregateChainMiddle,
}

impl AggregateReductionResult for SourceToMiddleAggregateResult {
    type Source = AggregateChainSource;
    type Target = AggregateChainMiddle;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: Sum<u64>) -> Sum<u64> {
        Sum(target_value.0 + 2)
    }
}

struct MiddleToTargetAggregateResult {
    target: AggregateChainTarget,
}

impl AggregateReductionResult for MiddleToTargetAggregateResult {
    type Source = AggregateChainMiddle;
    type Target = AggregateChainTarget;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: Sum<u64>) -> Sum<u64> {
        Sum(target_value.0 + 3)
    }
}

fn reduce_source_to_middle_aggregate(
    any: &dyn Any,
) -> Box<dyn crate::rules::traits::DynAggregateReductionResult> {
    any.downcast_ref::<AggregateChainSource>()
        .expect("expected AggregateChainSource");
    Box::new(SourceToMiddleAggregateResult {
        target: AggregateChainMiddle,
    })
}

fn reduce_middle_to_target_aggregate(
    any: &dyn Any,
) -> Box<dyn crate::rules::traits::DynAggregateReductionResult> {
    any.downcast_ref::<AggregateChainMiddle>()
        .expect("expected AggregateChainMiddle");
    Box::new(MiddleToTargetAggregateResult {
        target: AggregateChainTarget,
    })
}

struct SourceToMiddleWitnessResult {
    target: AggregateChainMiddle,
}

impl ReductionResult for SourceToMiddleWitnessResult {
    type Source = AggregateChainSource;
    type Target = AggregateChainMiddle;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

fn reduce_source_to_middle_witness(
    any: &dyn Any,
) -> Box<dyn crate::rules::traits::DynReductionResult> {
    any.downcast_ref::<AggregateChainSource>()
        .expect("expected AggregateChainSource");
    Box::new(SourceToMiddleWitnessResult {
        target: AggregateChainMiddle,
    })
}

fn reduce_natural_variant_witness(
    any: &dyn Any,
) -> Box<dyn crate::rules::traits::DynReductionResult> {
    let source = any
        .downcast_ref::<NaturalVariantProblem>()
        .expect("expected NaturalVariantProblem");
    Box::new(crate::rules::ReductionAutoCast::<
        NaturalVariantProblem,
        NaturalVariantProblem,
    >::new(source.clone()))
}

fn build_two_node_graph(
    source_name: &'static str,
    source_variant: BTreeMap<String, String>,
    target_name: &'static str,
    target_variant: BTreeMap<String, String>,
    edge: ReductionEdgeData,
) -> ReductionGraph {
    let nodes = vec![
        VariantNode {
            name: source_name,
            variant: source_variant.clone(),
            complexity: "",
        },
        VariantNode {
            name: target_name,
            variant: target_variant.clone(),
            complexity: "",
        },
    ];

    let mut graph = DiGraph::new();
    let source_idx = graph.add_node(0);
    let target_idx = graph.add_node(1);
    graph.add_edge(source_idx, target_idx, edge);

    let mut name_to_nodes = HashMap::new();
    name_to_nodes.insert(source_name, vec![source_idx]);
    name_to_nodes
        .entry(target_name)
        .or_insert_with(Vec::new)
        .push(target_idx);

    ReductionGraph {
        graph,
        nodes,
        name_to_nodes,
        default_variants: HashMap::new(),
    }
}

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
fn test_aggregate_reduction_chain_extracts_value_backwards() {
    let source_variant = BTreeMap::new();
    let middle_variant = BTreeMap::new();
    let target_variant = BTreeMap::new();

    let nodes = vec![
        VariantNode {
            name: AggregateChainSource::NAME,
            variant: source_variant.clone(),
            complexity: "",
        },
        VariantNode {
            name: AggregateChainMiddle::NAME,
            variant: middle_variant.clone(),
            complexity: "",
        },
        VariantNode {
            name: AggregateChainTarget::NAME,
            variant: target_variant.clone(),
            complexity: "",
        },
    ];

    let mut graph = DiGraph::new();
    let source_idx = graph.add_node(0);
    let middle_idx = graph.add_node(1);
    let target_idx = graph.add_node(2);

    graph.add_edge(
        source_idx,
        middle_idx,
        ReductionEdgeData {
            overhead: crate::rules::registry::ReductionOverhead::default(),
            reduce_fn: None,
            reduce_aggregate_fn: Some(reduce_source_to_middle_aggregate),
            capabilities: EdgeCapabilities::aggregate_only(),
        },
    );
    graph.add_edge(
        middle_idx,
        target_idx,
        ReductionEdgeData {
            overhead: crate::rules::registry::ReductionOverhead::default(),
            reduce_fn: None,
            reduce_aggregate_fn: Some(reduce_middle_to_target_aggregate),
            capabilities: EdgeCapabilities::aggregate_only(),
        },
    );

    let reduction_graph = ReductionGraph {
        graph,
        nodes,
        name_to_nodes: HashMap::from([
            (AggregateChainSource::NAME, vec![source_idx]),
            (AggregateChainMiddle::NAME, vec![middle_idx]),
            (AggregateChainTarget::NAME, vec![target_idx]),
        ]),
        default_variants: HashMap::new(),
    };
    let path = ReductionPath {
        steps: vec![
            ReductionStep {
                name: AggregateChainSource::NAME.to_string(),
                variant: source_variant,
            },
            ReductionStep {
                name: AggregateChainMiddle::NAME.to_string(),
                variant: middle_variant,
            },
            ReductionStep {
                name: AggregateChainTarget::NAME.to_string(),
                variant: target_variant,
            },
        ],
    };

    let chain = reduction_graph
        .reduce_aggregate_along_path(&path, &AggregateChainSource as &dyn Any)
        .expect("expected aggregate reduction chain");

    assert_eq!(
        chain.target_problem::<AggregateChainTarget>().dims(),
        vec![1]
    );
    assert_eq!(chain.extract_value_dyn(json!(7)), json!(12));
}

#[test]
fn witness_path_search_rejects_aggregate_only_edge() {
    let source_variant = BTreeMap::new();
    let target_variant = BTreeMap::new();
    let graph = build_two_node_graph(
        AggregateChainSource::NAME,
        source_variant.clone(),
        AggregateChainMiddle::NAME,
        target_variant.clone(),
        ReductionEdgeData {
            overhead: crate::rules::registry::ReductionOverhead::default(),
            reduce_fn: None,
            reduce_aggregate_fn: Some(reduce_source_to_middle_aggregate),
            capabilities: EdgeCapabilities::aggregate_only(),
        },
    );

    assert!(graph
        .find_cheapest_path_mode(
            AggregateChainSource::NAME,
            &source_variant,
            AggregateChainMiddle::NAME,
            &target_variant,
            ReductionMode::Witness,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .is_none());
    assert!(graph
        .find_cheapest_path_mode(
            AggregateChainSource::NAME,
            &source_variant,
            AggregateChainMiddle::NAME,
            &target_variant,
            ReductionMode::Aggregate,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .is_some());
}

#[test]
fn aggregate_path_search_rejects_witness_only_edge() {
    let source_variant = BTreeMap::new();
    let target_variant = BTreeMap::new();
    let graph = build_two_node_graph(
        AggregateChainSource::NAME,
        source_variant.clone(),
        AggregateChainMiddle::NAME,
        target_variant.clone(),
        ReductionEdgeData {
            overhead: crate::rules::registry::ReductionOverhead::default(),
            reduce_fn: Some(reduce_source_to_middle_witness),
            reduce_aggregate_fn: None,
            capabilities: EdgeCapabilities::witness_only(),
        },
    );

    assert!(graph
        .find_cheapest_path_mode(
            AggregateChainSource::NAME,
            &source_variant,
            AggregateChainMiddle::NAME,
            &target_variant,
            ReductionMode::Aggregate,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .is_none());
    assert!(graph
        .find_cheapest_path_mode(
            AggregateChainSource::NAME,
            &source_variant,
            AggregateChainMiddle::NAME,
            &target_variant,
            ReductionMode::Witness,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .is_some());
}

#[test]
fn natural_edge_supports_both_modes() {
    let source_variant = BTreeMap::from([("graph".to_string(), "Source".to_string())]);
    let target_variant = BTreeMap::from([("graph".to_string(), "Target".to_string())]);
    let graph = build_two_node_graph(
        NaturalVariantProblem::NAME,
        source_variant.clone(),
        NaturalVariantProblem::NAME,
        target_variant.clone(),
        ReductionEdgeData {
            overhead: crate::rules::registry::ReductionOverhead::default(),
            reduce_fn: Some(reduce_natural_variant_witness),
            reduce_aggregate_fn: None,
            capabilities: EdgeCapabilities::both(),
        },
    );

    let witness_path = graph.find_cheapest_path_mode(
        NaturalVariantProblem::NAME,
        &source_variant,
        NaturalVariantProblem::NAME,
        &target_variant,
        ReductionMode::Witness,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );
    let aggregate_path = graph.find_cheapest_path_mode(
        NaturalVariantProblem::NAME,
        &source_variant,
        NaturalVariantProblem::NAME,
        &target_variant,
        ReductionMode::Aggregate,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );

    assert!(witness_path.is_some());
    let aggregate_path = aggregate_path.expect("expected aggregate path");
    let chain = graph
        .reduce_aggregate_along_path(&aggregate_path, &NaturalVariantProblem as &dyn Any)
        .expect("expected aggregate chain");
    assert_eq!(chain.extract_value_dyn(json!(7)), json!(7));
}

#[test]
fn reduce_aggregate_along_path_rejects_single_step_path() {
    let source_variant = BTreeMap::new();
    let graph = build_two_node_graph(
        AggregateChainSource::NAME,
        source_variant.clone(),
        AggregateChainMiddle::NAME,
        BTreeMap::new(),
        ReductionEdgeData {
            overhead: crate::rules::registry::ReductionOverhead::default(),
            reduce_fn: None,
            reduce_aggregate_fn: Some(reduce_source_to_middle_aggregate),
            capabilities: EdgeCapabilities::aggregate_only(),
        },
    );
    let single_step_path = ReductionPath {
        steps: vec![ReductionStep {
            name: AggregateChainSource::NAME.to_string(),
            variant: source_variant,
        }],
    };
    assert!(graph
        .reduce_aggregate_along_path(&single_step_path, &AggregateChainSource as &dyn Any)
        .is_none());
}

#[test]
fn reduce_aggregate_returns_none_for_witness_only_edge() {
    let source_variant = BTreeMap::new();
    let target_variant = BTreeMap::new();
    let graph = build_two_node_graph(
        AggregateChainSource::NAME,
        source_variant.clone(),
        AggregateChainMiddle::NAME,
        target_variant.clone(),
        ReductionEdgeData {
            overhead: crate::rules::registry::ReductionOverhead::default(),
            reduce_fn: Some(reduce_source_to_middle_witness),
            reduce_aggregate_fn: None,
            capabilities: EdgeCapabilities::witness_only(),
        },
    );
    let path = ReductionPath {
        steps: vec![
            ReductionStep {
                name: AggregateChainSource::NAME.to_string(),
                variant: source_variant,
            },
            ReductionStep {
                name: AggregateChainMiddle::NAME.to_string(),
                variant: target_variant,
            },
        ],
    };
    assert!(graph
        .reduce_aggregate_along_path(&path, &AggregateChainSource as &dyn Any)
        .is_none());
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
    let path = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src,
        "MaximumSetPacking",
        &dst,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );
    assert!(path.is_some());
    let path = path.unwrap();
    assert_eq!(path.len(), 1); // Direct path exists
}

#[test]
fn test_knapsack_to_ilp_path_exists() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&Knapsack::variant());
    let dst = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let path = graph.find_cheapest_path(
        "Knapsack",
        &src,
        "ILP",
        &dst,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );

    let path = path.expect("Knapsack should reduce to ILP");
    assert_eq!(
        path.type_names(),
        vec!["Knapsack", "ILP"],
        "Knapsack should have a direct ILP reduction"
    );
    assert_eq!(path.len(), 1, "Knapsack -> ILP should be one direct step");
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
    let path = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src,
        "QUBO",
        &dst,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );
    assert!(path.is_some());
    let path = path.unwrap();
    assert!(
        path.len() > 1,
        "MIS -> QUBO should now go through a composite path"
    );
}

#[test]
fn test_variant_level_paths() {
    let graph = ReductionGraph::new();

    // Variant-level path: MaxCut<SimpleGraph, i32> -> SpinGlass<SimpleGraph, i32>
    let src = ReductionGraph::variant_to_map(
        &crate::models::graph::MaxCut::<SimpleGraph, i32>::variant(),
    );
    let dst = ReductionGraph::variant_to_map(
        &crate::models::graph::SpinGlass::<SimpleGraph, i32>::variant(),
    );
    let paths = graph.find_all_paths("MaxCut", &src, "SpinGlass", &dst);
    assert!(!paths.is_empty());
    assert_eq!(paths[0].type_names(), vec!["MaxCut", "SpinGlass"]);

    // Unregistered variant pair returns no paths
    let src_f64 = ReductionGraph::variant_to_map(
        &crate::models::graph::MaxCut::<SimpleGraph, f64>::variant(),
    );
    let dst_f64 = ReductionGraph::variant_to_map(&crate::models::graph::SpinGlass::<
        SimpleGraph,
        f64,
    >::variant());
    let paths_f64 = graph.find_all_paths("MaxCut", &src_f64, "SpinGlass", &dst_f64);
    // No direct MaxCut<f64> -> SpinGlass<f64> reduction registered
    assert!(paths_f64.is_empty());
}

#[test]
fn test_find_shortest_path_variants() {
    let graph = ReductionGraph::new();

    let src = ReductionGraph::variant_to_map(
        &crate::models::graph::MaxCut::<SimpleGraph, i32>::variant(),
    );
    let dst = ReductionGraph::variant_to_map(
        &crate::models::graph::SpinGlass::<SimpleGraph, i32>::variant(),
    );
    let shortest = graph.find_cheapest_path(
        "MaxCut",
        &src,
        "SpinGlass",
        &dst,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );
    assert!(shortest.is_some());
    assert_eq!(shortest.unwrap().len(), 1); // Direct path

    let src = ReductionGraph::variant_to_map(&crate::models::misc::Factoring::variant());
    let dst = ReductionGraph::variant_to_map(
        &crate::models::graph::SpinGlass::<SimpleGraph, i32>::variant(),
    );
    let shortest = graph.find_cheapest_path(
        "Factoring",
        &src,
        "SpinGlass",
        &dst,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );
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
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src,
            "MinimumVertexCover",
            &dst,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();

    assert!(!path.is_empty());
    assert!(path.source().unwrap().contains("MaximumIndependentSet"));
    assert!(path.target().unwrap().contains("MinimumVertexCover"));
}

#[test]
fn test_to_json() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // Check nodes
    assert!(json.nodes.len() >= 10);
    assert!(json.nodes.iter().any(|n| n.name == "MaximumIndependentSet"));
    assert!(json.nodes.iter().any(|n| n.category == "graph"));
    assert!(json.nodes.iter().any(|n| n.category == "algebraic"));

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
        ReductionGraph::category_from_module_path("problemreductions::models::algebraic::qubo"),
        "algebraic"
    );
    assert_eq!(
        ReductionGraph::category_from_module_path("problemreductions::models::formula::sat"),
        "formula"
    );
    assert_eq!(
        ReductionGraph::category_from_module_path("problemreductions::models::misc::factoring"),
        "misc"
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
            "problemreductions::models::algebraic::qubo",
            "QUBO"
        ),
        "models/algebraic/struct.QUBO.html"
    );
}

#[test]
fn test_sat_based_reductions() {
    use crate::models::formula::Satisfiability;
    use crate::models::graph::KColoring;
    use crate::models::graph::MinimumDominatingSet;
    use crate::variant::K3;

    let graph = ReductionGraph::new();

    // SAT -> IS
    assert!(graph.has_direct_reduction::<Satisfiability, MaximumIndependentSet<SimpleGraph, One>>());

    // SAT -> KColoring
    assert!(graph.has_direct_reduction::<Satisfiability, KColoring<K3, SimpleGraph>>());

    // SAT -> MinimumDominatingSet
    assert!(graph.has_direct_reduction::<Satisfiability, MinimumDominatingSet<SimpleGraph, i32>>());
}

#[test]
fn test_circuit_reductions() {
    use crate::models::formula::CircuitSAT;
    use crate::models::graph::SpinGlass;
    use crate::models::misc::Factoring;

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
    let shortest = graph
        .find_cheapest_path(
            "Factoring",
            &src,
            "SpinGlass",
            &dst,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();
    assert_eq!(shortest.len(), 2); // Factoring -> CircuitSAT -> SpinGlass
}

#[test]
fn test_optimization_reductions() {
    use crate::models::algebraic::QUBO;
    use crate::models::graph::MaxCut;
    use crate::models::graph::SpinGlass;

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
    use crate::models::formula::{KSatisfiability, Satisfiability};
    use crate::variant::K3;

    let graph = ReductionGraph::new();

    // SAT <-> 3-SAT (bidirectional)
    assert!(graph.has_direct_reduction::<Satisfiability, KSatisfiability<K3>>());
    assert!(graph.has_direct_reduction::<KSatisfiability<K3>, Satisfiability>());
}

#[test]
fn test_nae_sat_to_maxcut_reduction_registered() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction::<NAESatisfiability, MaxCut<SimpleGraph, i32>>());
}

#[test]
fn test_maximum2satisfiability_to_maxcut_reduction_registered() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction::<Maximum2Satisfiability, MaxCut<SimpleGraph, i32>>());
}

#[test]
fn test_nae_sat_to_partition_into_perfect_matchings_reduction_registered() {
    use crate::models::graph::PartitionIntoPerfectMatchings;

    let graph = ReductionGraph::new();

    assert!(graph
        .has_direct_reduction::<NAESatisfiability, PartitionIntoPerfectMatchings<SimpleGraph>>());
}

#[test]
fn test_all_categories_present() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    let categories: std::collections::HashSet<&str> =
        json.nodes.iter().map(|n| n.category.as_str()).collect();

    assert!(categories.contains("graph"));
    assert!(categories.contains("set"));
    assert!(categories.contains("algebraic"));
    assert!(categories.contains("formula"));
    assert!(categories.contains("misc"));
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
    let is_var =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());

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
        .find_cheapest_path(
            "UnknownProblem",
            &unknown,
            "MaximumIndependentSet",
            &is_var,
            &ProblemSize::new(vec![]),
            &MinimizeSteps
        )
        .is_none());
}

#[test]
fn test_category_derived_from_schema() {
    // CircuitSAT's category is derived from its ProblemSchemaEntry module_path
    let graph = ReductionGraph::new();
    let json = graph.to_json();
    let circuit = json.nodes.iter().find(|n| n.name == "CircuitSAT").unwrap();
    assert_eq!(circuit.category, "formula");
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
fn test_circuitsat_to_satisfiability_direct_edge() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&CircuitSAT::variant());
    let dst = ReductionGraph::variant_to_map(&Satisfiability::variant());

    assert!(graph.has_direct_reduction_by_name("CircuitSAT", "Satisfiability"));

    let path = graph.find_cheapest_path(
        "CircuitSAT",
        &src,
        "Satisfiability",
        &dst,
        &ProblemSize::new(vec![]),
        &MinimizeSteps,
    );
    assert!(
        path.is_some(),
        "CircuitSAT -> Satisfiability path should exist"
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
    let input_size = crate::types::ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 20)]);
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
    let cost_fn = Minimize("num_vars");
    let input_size = crate::types::ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 20)]);
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
    let path = path.unwrap();
    assert!(
        path.len() > 1,
        "MIS -> QUBO should now be discovered through a composite path"
    );
    assert_eq!(
        path.type_names(),
        vec!["MaximumIndependentSet", "MaximumSetPacking", "QUBO"]
    );
}

#[test]
fn test_find_cheapest_path_unknown_source() {
    let graph = ReductionGraph::new();
    let cost_fn = MinimizeSteps;
    let input_size = crate::types::ProblemSize::new(vec![]);
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
    let input_size = crate::types::ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 20)]);
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
        classify_problem_category("problemreductions::models::formula::satisfiability"),
        "formula"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::set::maximum_set_packing"),
        "set"
    );
    assert_eq!(
        classify_problem_category("problemreductions::models::algebraic::qubo"),
        "algebraic"
    );
    assert_eq!(classify_problem_category("unknown::path"), "other");
}

#[test]
fn test_reduce_along_path_direct() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let rpath = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src,
            "MinimumVertexCover",
            &dst,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();
    // Just verify the path can produce a chain with a dummy source
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    let chain = graph.reduce_along_path(&rpath, &source as &dyn std::any::Any);
    assert!(chain.is_some());
}

#[test]
fn test_reduction_chain_direct() {
    use crate::solvers::BruteForce;
    use crate::traits::Problem;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let rpath = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src,
            "MinimumVertexCover",
            &dst,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    let chain = graph
        .reduce_along_path(&rpath, &problem as &dyn std::any::Any)
        .unwrap();
    let target: &MinimumVertexCover<SimpleGraph, i32> = chain.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.find_witness(target).unwrap();
    let source_solution = chain.extract_solution(&target_solution);
    let metric = problem.evaluate(&source_solution);
    assert!(metric.is_valid());
}

#[test]
fn test_reduction_chain_multi_step() {
    use crate::solvers::BruteForce;
    use crate::traits::Problem;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    let rpath = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src,
            "MaximumSetPacking",
            &dst,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    let chain = graph
        .reduce_along_path(&rpath, &problem as &dyn std::any::Any)
        .unwrap();
    let target: &MaximumSetPacking<i32> = chain.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.find_witness(target).unwrap();
    let source_solution = chain.extract_solution(&target_solution);
    let metric = problem.evaluate(&source_solution);
    assert!(metric.is_valid());
}

#[test]
fn test_reduction_chain_with_variant_casts() {
    use crate::models::formula::{CNFClause, KSatisfiability};
    use crate::rules::MinimizeSteps;
    use crate::solvers::BruteForce;
    use crate::topology::UnitDiskGraph;
    use crate::traits::Problem;
    use crate::types::ProblemSize;

    let graph = ReductionGraph::new();

    // MIS<UnitDiskGraph, i32> -> MIS<SimpleGraph, i32> (variant cast) -> MVC<SimpleGraph, i32>
    // Use find_cheapest_path for exact variant matching (not name-based)
    let src_var =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<UnitDiskGraph, i32>::variant());
    let dst_var =
        ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let rpath = graph.find_cheapest_path(
        "MaximumIndependentSet",
        &src_var,
        "MinimumVertexCover",
        &dst_var,
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

    // Create a small UnitDiskGraph MIS problem (triangle of close nodes)
    let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (0.5, 0.0), (0.25, 0.4)], 1.0);
    let mis = MaximumIndependentSet::new(udg, vec![1i32, 1, 1]);

    let chain = graph
        .reduce_along_path(&rpath, &mis as &dyn std::any::Any)
        .unwrap();
    let target: &MinimumVertexCover<SimpleGraph, i32> = chain.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.find_witness(target).unwrap();
    let source_solution = chain.extract_solution(&target_solution);
    let metric = mis.evaluate(&source_solution);
    assert!(metric.is_valid());

    // Also test the KSat<K3> -> Sat -> MIS multi-step path
    // Use find_cheapest_path for exact variant matching (not name-based
    // and may pick a path through a different KSat variant)
    let ksat_src =
        ReductionGraph::variant_to_map(&KSatisfiability::<crate::variant::K3>::variant());
    let ksat_dst =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let ksat_rpath = graph.find_cheapest_path(
        "KSatisfiability",
        &ksat_src,
        "MaximumIndependentSet",
        &ksat_dst,
        &crate::types::ProblemSize::new(vec![]),
        &crate::rules::MinimizeSteps,
    );
    assert!(
        ksat_rpath.is_some(),
        "Should find path from KSat<K3> to MIS"
    );
    let ksat_rpath = ksat_rpath.unwrap();

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

    let ksat_chain = graph
        .reduce_along_path(&ksat_rpath, &ksat as &dyn std::any::Any)
        .unwrap();
    let target: &MaximumIndependentSet<SimpleGraph, i32> = ksat_chain.target_problem();

    let target_solution = solver.find_witness(target).unwrap();
    let original_solution = ksat_chain.extract_solution(&target_solution);

    // Verify the extracted solution satisfies the original 3-SAT formula
    assert!(ksat.evaluate(&original_solution));
}

#[test]
fn test_size_field_names_returns_own_fields() {
    let graph = ReductionGraph::new();

    // MIS should report its own fields (num_vertices, num_edges),
    // not the target's fields from any reduction.
    let mis_fields = graph.size_field_names("MaximumIndependentSet");
    assert!(
        mis_fields.contains(&"num_vertices"),
        "MIS should have num_vertices, got: {:?}",
        mis_fields
    );
    assert!(
        mis_fields.contains(&"num_edges"),
        "MIS should have num_edges, got: {:?}",
        mis_fields
    );
    // Should NOT contain target fields like num_vars or num_constraints
    assert!(
        !mis_fields.contains(&"num_constraints"),
        "MIS should not report ILP's num_constraints, got: {:?}",
        mis_fields
    );

    // QUBO should report num_vars
    let qubo_fields = graph.size_field_names("QUBO");
    assert!(
        qubo_fields.contains(&"num_vars"),
        "QUBO should have num_vars, got: {:?}",
        qubo_fields
    );

    // Unknown problem returns empty
    let unknown_fields = graph.size_field_names("NonExistentProblem");
    assert!(unknown_fields.is_empty());
}

#[test]
fn test_overhead_variables_are_consistent() {
    // For each reduction, the input variables of the overhead should be
    // a subset of the source problem's size fields (as derived from all
    // reductions where it appears).
    let graph = ReductionGraph::new();

    for entry in inventory::iter::<ReductionEntry> {
        let overhead = entry.overhead();
        let input_vars = overhead.input_variable_names();
        if input_vars.is_empty() {
            continue;
        }

        let source_fields: std::collections::HashSet<&str> = graph
            .size_field_names(entry.source_name)
            .into_iter()
            .collect();

        for var in &input_vars {
            assert!(
                source_fields.contains(var),
                "Reduction {} -> {}: overhead references variable '{}' \
                 which is not a known size field of {}. Known fields: {:?}",
                entry.source_name,
                entry.target_name,
                var,
                entry.source_name,
                source_fields
            );
        }
    }
}

#[test]
fn test_variant_entry_complexity_available() {
    let entries: Vec<_> = inventory::iter::<crate::registry::VariantEntry>
        .into_iter()
        .collect();
    assert!(
        !entries.is_empty(),
        "VariantEntry inventory should not be empty"
    );

    let mis_entry = entries.iter().find(|e| e.name == "MaximumIndependentSet");
    assert!(mis_entry.is_some(), "MIS should have a VariantEntry");
    let mis_entry = mis_entry.unwrap();
    assert!(
        !mis_entry.complexity.is_empty(),
        "complexity should not be empty"
    );

    // Exercise Debug impl for VariantEntry
    let debug_str = format!("{:?}", mis_entry);
    assert!(debug_str.contains("VariantEntry"));
    assert!(debug_str.contains("MaximumIndependentSet"));
    assert!(debug_str.contains("complexity"));
}

#[test]
fn test_variant_complexity() {
    let graph = ReductionGraph::new();
    let variant = ReductionGraph::variant_to_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
    let complexity = graph.variant_complexity("MaximumIndependentSet", &variant);
    assert_eq!(complexity, Some("1.1996^num_vertices"));

    // Unknown problem returns None
    let unknown = BTreeMap::new();
    assert_eq!(
        graph.variant_complexity("NonExistentProblem", &unknown),
        None
    );
}

#[test]
fn test_compute_source_size() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1, 1],
    );
    let size = ReductionGraph::compute_source_size("MaximumIndependentSet", &problem);
    assert_eq!(size.get("num_vertices"), Some(4));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_compute_source_size_unknown_problem() {
    let problem = 42u32;
    let size = ReductionGraph::compute_source_size("NonExistentProblem", &problem);
    assert!(size.components.is_empty());
}

#[test]
fn test_evaluate_path_overhead() {
    use crate::rules::cost::MinimizeStepsThenOverhead;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let input_size = ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 20)]);

    let path = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src,
            "MinimumVertexCover",
            &dst,
            &input_size,
            &MinimizeStepsThenOverhead,
        )
        .expect("should find path");

    let final_size = graph
        .evaluate_path_overhead(&path, &input_size)
        .expect("should evaluate overhead");

    // MIS → MVC preserves num_vertices and num_edges
    assert_eq!(final_size.get("num_vertices"), Some(10));
    assert_eq!(final_size.get("num_edges"), Some(20));
}

#[test]
fn test_evaluate_path_overhead_multistep() {
    use crate::rules::cost::MinimizeStepsThenOverhead;

    // MIS → SetPacking<One> → SetPacking<i32> → ILP<bool> (3 steps with size transformations)
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, One>::variant());
    let dst_variants = graph.variants_for("ILP");
    let dst = dst_variants
        .iter()
        .find(|v| v.get("variable") == Some(&"bool".to_string()))
        .expect("ILP<bool> variant should exist");
    let input_size = ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 20)]);

    let path = graph
        .find_cheapest_path_mode(
            "MaximumIndependentSet",
            &src,
            "ILP",
            dst,
            ReductionMode::Witness,
            &input_size,
            &MinimizeStepsThenOverhead,
        )
        .expect("should find path");

    assert!(
        path.len() >= 2,
        "path should have at least 2 steps, got {}",
        path.len()
    );

    let final_size = graph
        .evaluate_path_overhead(&path, &input_size)
        .expect("should evaluate overhead");

    // MIS(V=10,E=20) → SetPacking(sets=V=10, universe=E=20) → ... → ILP(vars=10, constraints=20)
    // The final ILP dimensions should reflect the composed overhead, not the input.
    assert_eq!(final_size.get("num_vars"), Some(10));
    assert_eq!(final_size.get("num_constraints"), Some(20));
    // Original MIS fields should NOT appear in the final output
    assert_eq!(final_size.get("num_vertices"), None);
    assert_eq!(final_size.get("num_edges"), None);
}
