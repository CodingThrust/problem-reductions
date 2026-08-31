use super::*;
use crate::expr::Expr;
use crate::models::algebraic::{ILP, QUBO};
use crate::models::formula::{
    CircuitSAT, Maximum2Satisfiability, NAESatisfiability, Satisfiability,
};
use crate::models::graph::MaxCut;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::models::misc::Knapsack;
use crate::models::set::MaximumSetPacking;
use crate::registry::ProblemCategory;
use crate::rules::graph::{ReductionMode, ReductionStep};
use crate::rules::registry::{ReductionEntry, ReductionParameterDeclarations};
use crate::rules::traits::{AggregateReductionResult, ReductionResult};
use crate::solvers::BruteForceProblem as _;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{One, ProblemParameters, Sum};
use petgraph::graph::DiGraph;
use serde_json::json;
use std::any::Any;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};

fn empty_parameter_contract() -> Result<ReductionParameterContract, ParameterContractError> {
    ReductionParameterContract::new(
        "synthetic edge",
        ReductionParameterDeclarations {
            relation: None,
            fields: vec![],
            unavailable: vec![crate::rules::registry::UnavailableParameterField {
                field: "size",
                reason: "synthetic edge does not declare a symbolic parameter",
            }],
        },
    )
}

fn symbolic_size_edge(fields: &[(&'static str, &str)], turing: bool) -> ReductionEdgeData {
    ReductionEdgeData {
        parameter_contract: ReductionParameterContract::new(
            "synthetic edge",
            ReductionParameterDeclarations {
                relation: Some(crate::parameters::ParameterRelation::Exact),
                fields: fields
                    .iter()
                    .map(|(field, expression)| (*field, Expr::try_parse(expression).unwrap()))
                    .collect(),
                unavailable: vec![],
            },
        ),
        reduce_fn: Some(|_| panic!("size search must not execute reductions")),
        reduce_aggregate_fn: None,
        turing,
    }
}

fn named_path(names: &[&str]) -> ReductionPath {
    ReductionPath {
        steps: names
            .iter()
            .map(|name| ReductionStep {
                name: (*name).to_string(),
                variant: BTreeMap::new(),
            })
            .collect(),
    }
}

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
    type Solution = Vec<usize>;
    type Value = Sum<u64>;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Sum(config.iter().sum::<usize>() as u64))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for AggregateChainSource {
    fn dimensions(&self) -> Vec<usize> {
        vec![1]
    }
}

impl Problem for AggregateChainMiddle {
    const NAME: &'static str = "AggregateChainMiddle";
    type Solution = Vec<usize>;
    type Value = Sum<u64>;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Sum(config.iter().sum::<usize>() as u64))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for AggregateChainMiddle {
    fn dimensions(&self) -> Vec<usize> {
        vec![1]
    }
}

impl Problem for AggregateChainTarget {
    const NAME: &'static str = "AggregateChainTarget";
    type Solution = Vec<usize>;
    type Value = Sum<u64>;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Sum(config.iter().sum::<usize>() as u64))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for AggregateChainTarget {
    fn dimensions(&self) -> Vec<usize> {
        vec![1]
    }
}

impl Problem for NaturalVariantProblem {
    const NAME: &'static str = "NaturalVariantProblem";
    type Solution = Vec<usize>;
    type Value = Sum<u64>;

    crate::problem_parameters![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Sum(config.iter().sum::<usize>() as u64))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for NaturalVariantProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![1]
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
) -> Result<Box<dyn crate::rules::traits::DynAggregateReductionResult>, crate::rules::ReductionError>
{
    any.downcast_ref::<AggregateChainSource>().ok_or(
        crate::rules::ReductionError::SourceTypeMismatch {
            source_problem: AggregateChainSource::NAME,
            target_problem: AggregateChainMiddle::NAME,
            expected: std::any::type_name::<AggregateChainSource>(),
        },
    )?;
    Ok(Box::new(SourceToMiddleAggregateResult {
        target: AggregateChainMiddle,
    }))
}

fn reduce_middle_to_target_aggregate(
    any: &dyn Any,
) -> Result<Box<dyn crate::rules::traits::DynAggregateReductionResult>, crate::rules::ReductionError>
{
    any.downcast_ref::<AggregateChainMiddle>().ok_or(
        crate::rules::ReductionError::SourceTypeMismatch {
            source_problem: AggregateChainMiddle::NAME,
            target_problem: AggregateChainTarget::NAME,
            expected: std::any::type_name::<AggregateChainMiddle>(),
        },
    )?;
    Ok(Box::new(MiddleToTargetAggregateResult {
        target: AggregateChainTarget,
    }))
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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        Ok(target_solution.to_vec())
    }
}

fn reduce_source_to_middle_witness(
    any: &dyn Any,
) -> Result<Box<dyn crate::rules::traits::DynReductionResult>, crate::rules::ReductionError> {
    any.downcast_ref::<AggregateChainSource>().ok_or(
        crate::rules::ReductionError::SourceTypeMismatch {
            source_problem: AggregateChainSource::NAME,
            target_problem: AggregateChainMiddle::NAME,
            expected: std::any::type_name::<AggregateChainSource>(),
        },
    )?;
    Ok(Box::new(SourceToMiddleWitnessResult {
        target: AggregateChainMiddle,
    }))
}

fn fail_source_to_middle_witness(
    _any: &dyn Any,
) -> Result<Box<dyn crate::rules::traits::DynReductionResult>, crate::rules::ReductionError> {
    Err(crate::rules::ReductionError::InvalidTarget {
        source_problem: AggregateChainSource::NAME,
        target_problem: AggregateChainMiddle::NAME,
        message: "synthetic target construction failure".to_string(),
    })
}

static SHARED_PREFIX_EXECUTIONS: AtomicUsize = AtomicUsize::new(0);

fn reduce_counted_source_to_middle_witness(
    any: &dyn Any,
) -> Result<Box<dyn crate::rules::traits::DynReductionResult>, crate::rules::ReductionError> {
    SHARED_PREFIX_EXECUTIONS.fetch_add(1, Ordering::SeqCst);
    reduce_source_to_middle_witness(any)
}

struct MiddleToTargetWitnessResult {
    target: AggregateChainTarget,
}

impl ReductionResult for MiddleToTargetWitnessResult {
    type Source = AggregateChainMiddle;
    type Target = AggregateChainTarget;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        Ok(target_solution.to_vec())
    }
}

fn reduce_middle_to_target_witness(
    any: &dyn Any,
) -> Result<Box<dyn crate::rules::traits::DynReductionResult>, crate::rules::ReductionError> {
    any.downcast_ref::<AggregateChainMiddle>().ok_or(
        crate::rules::ReductionError::SourceTypeMismatch {
            source_problem: AggregateChainMiddle::NAME,
            target_problem: AggregateChainTarget::NAME,
            expected: std::any::type_name::<AggregateChainMiddle>(),
        },
    )?;
    Ok(Box::new(MiddleToTargetWitnessResult {
        target: AggregateChainTarget,
    }))
}

fn reduce_natural_variant_witness(
    any: &dyn Any,
) -> Result<Box<dyn crate::rules::traits::DynReductionResult>, crate::rules::ReductionError> {
    let source = any.downcast_ref::<NaturalVariantProblem>().ok_or(
        crate::rules::ReductionError::SourceTypeMismatch {
            source_problem: NaturalVariantProblem::NAME,
            target_problem: NaturalVariantProblem::NAME,
            expected: std::any::type_name::<NaturalVariantProblem>(),
        },
    )?;
    Ok(Box::new(crate::rules::VariantReductionResult::<
        NaturalVariantProblem,
        NaturalVariantProblem,
    >::new(source.clone())))
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
fn execute_paths_executes_a_shared_prefix_once() {
    SHARED_PREFIX_EXECUTIONS.store(0, Ordering::SeqCst);
    let witness_edge = |reduce_fn| ReductionEdgeData {
        parameter_contract: empty_parameter_contract(),
        reduce_fn: Some(reduce_fn),
        reduce_aggregate_fn: None,
        turing: false,
    };
    let graph = ReductionGraph::from_test_edges(
        &[
            AggregateChainSource::NAME,
            AggregateChainMiddle::NAME,
            AggregateChainTarget::NAME,
        ],
        &[
            (
                AggregateChainSource::NAME,
                AggregateChainMiddle::NAME,
                witness_edge(reduce_counted_source_to_middle_witness),
            ),
            (
                AggregateChainMiddle::NAME,
                AggregateChainTarget::NAME,
                witness_edge(reduce_middle_to_target_witness),
            ),
        ],
    );
    let paths = vec![
        named_path(&[AggregateChainSource::NAME, AggregateChainMiddle::NAME]),
        named_path(&[
            AggregateChainSource::NAME,
            AggregateChainMiddle::NAME,
            AggregateChainTarget::NAME,
        ]),
    ];

    let executed = graph
        .execute_paths(&paths, &AggregateChainSource)
        .expect("both paths are executable");

    assert_eq!(executed.len(), 2);
    assert_eq!(SHARED_PREFIX_EXECUTIONS.load(Ordering::SeqCst), 1);
}

#[test]
fn path_parameter_contract_errors_are_typed_and_isolated() {
    let single = named_path(&["A"]);
    let empty = ReductionPath { steps: vec![] };
    let graph = ReductionGraph::from_test_edges(&["A", "B"], &[]);
    assert!(graph.path_parameter_transforms(&single).unwrap().is_empty());
    assert!(graph
        .compose_path_parameter_transform(&single)
        .unwrap()
        .is_none());
    assert!(matches!(
        graph.compose_path_parameter_transform(&empty),
        Err(PathParameterError::EmptyPath)
    ));

    let unknown = named_path(&["A", "Unknown"]);
    assert!(matches!(
        graph.path_parameter_transforms(&unknown),
        Err(PathParameterError::UnknownNode { .. })
    ));

    let disconnected = named_path(&["A", "B"]);
    assert!(matches!(
        graph.path_parameter_transforms(&disconnected),
        Err(PathParameterError::MissingEdge { .. })
    ));

    let unavailable = build_two_node_graph(
        "A",
        BTreeMap::new(),
        "B",
        BTreeMap::new(),
        ReductionEdgeData {
            parameter_contract: empty_parameter_contract(),
            reduce_fn: Some(|_| panic!("metadata inspection must not execute reductions")),
            reduce_aggregate_fn: None,
            turing: false,
        },
    );
    assert!(matches!(
        unavailable.path_parameter_transforms(&disconnected),
        Err(PathParameterError::Unavailable { .. })
    ));

    let invalid_contract = Err(ParameterContractError::EmptyUnavailableReason {
        edge: "A -> B".into(),
        field: "x".into(),
    });
    let invalid = build_two_node_graph(
        "A",
        BTreeMap::new(),
        "B",
        BTreeMap::new(),
        ReductionEdgeData {
            parameter_contract: invalid_contract,
            reduce_fn: Some(|_| panic!("metadata inspection must not execute reductions")),
            reduce_aggregate_fn: None,
            turing: false,
        },
    );
    assert!(matches!(
        invalid.path_parameter_transforms(&disconnected),
        Err(PathParameterError::InvalidContract { .. })
    ));

    let turing = build_two_node_graph(
        "A",
        BTreeMap::new(),
        "B",
        BTreeMap::new(),
        symbolic_size_edge(&[("x", "n")], true),
    );
    assert!(matches!(
        turing.path_parameter_transforms(&disconnected),
        Err(PathParameterError::TuringEdge { .. })
    ));
}

#[test]
fn path_size_composition_and_contract_evaluation_report_errors() {
    let missing_input = build_two_node_graph(
        "A",
        BTreeMap::new(),
        "B",
        BTreeMap::new(),
        symbolic_size_edge(&[("x", "n")], false),
    );
    let direct = named_path(&["A", "B"]);
    let direct_transform = missing_input
        .compose_path_parameter_transform(&direct)
        .unwrap()
        .unwrap();
    assert!(matches!(
        direct_transform.evaluate(&ProblemParameters::default()),
        Err(crate::parameters::ParameterTransformError::MissingInputField { .. })
    ));

    let invalid_composition = ReductionGraph::from_test_edges(
        &["A", "B", "C"],
        &[
            ("A", "B", symbolic_size_edge(&[("x", "n")], false)),
            ("B", "C", symbolic_size_edge(&[("z", "y")], false)),
        ],
    );
    let chained = named_path(&["A", "B", "C"]);
    assert!(matches!(
        invalid_composition.compose_path_parameter_transform(&chained),
        Err(PathParameterError::Step { .. })
    ));

    let valid = ReductionGraph::from_test_edges(
        &["A", "B", "C"],
        &[
            ("A", "B", symbolic_size_edge(&[("x", "n + 1")], false)),
            ("B", "C", symbolic_size_edge(&[("z", "2 * x")], false)),
        ],
    );
    let transform = valid
        .compose_path_parameter_transform(&chained)
        .unwrap()
        .unwrap();
    assert_eq!(
        transform
            .evaluate(&ProblemParameters::new(vec![("n", 3)]))
            .unwrap()
            .get("z"),
        Some(8)
    );
}

#[test]
fn symbolic_path_enumeration_retains_every_path_without_ranking() {
    let graph = ReductionGraph::from_test_edges(
        &["S", "A", "B", "C", "T"],
        &[
            ("S", "A", symbolic_size_edge(&[("x", "2")], false)),
            ("S", "B", symbolic_size_edge(&[("x", "1")], false)),
            ("S", "C", symbolic_size_edge(&[("x", "3")], false)),
            ("A", "T", symbolic_size_edge(&[("y", "x")], false)),
            ("B", "T", symbolic_size_edge(&[("y", "x")], false)),
            ("C", "T", symbolic_size_edge(&[("y", "x")], false)),
        ],
    );
    let variant = BTreeMap::new();
    let paths = graph.find_all_paths_mode("S", &variant, "T", &variant, ReductionMode::Witness);
    assert_eq!(paths.len(), 3);
    let values: BTreeSet<_> = paths
        .iter()
        .map(|path| {
            graph
                .compose_path_parameter_transform(path)
                .unwrap()
                .unwrap()
                .evaluate(&ProblemParameters::default())
                .unwrap()
                .get("y")
                .unwrap()
        })
        .collect();
    assert_eq!(values, BTreeSet::from([1, 2, 3]));
}

#[test]
fn test_find_direct_path() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i64>::variant());
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
            parameter_contract: empty_parameter_contract(),
            reduce_fn: None,
            reduce_aggregate_fn: Some(reduce_source_to_middle_aggregate),
            turing: false,
        },
    );
    graph.add_edge(
        middle_idx,
        target_idx,
        ReductionEdgeData {
            parameter_contract: empty_parameter_contract(),
            reduce_fn: None,
            reduce_aggregate_fn: Some(reduce_middle_to_target_aggregate),
            turing: false,
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
        .expect("aggregate reduction should not fail")
        .expect("expected aggregate reduction chain");

    assert_eq!(
        chain.target_problem::<AggregateChainTarget>().dimensions(),
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
            parameter_contract: empty_parameter_contract(),
            reduce_fn: None,
            reduce_aggregate_fn: Some(reduce_source_to_middle_aggregate),
            turing: false,
        },
    );

    assert!(graph
        .find_all_paths_mode(
            AggregateChainSource::NAME,
            &source_variant,
            AggregateChainMiddle::NAME,
            &target_variant,
            ReductionMode::Witness
        )
        .is_empty());
    assert!(!graph
        .find_all_paths_mode(
            AggregateChainSource::NAME,
            &source_variant,
            AggregateChainMiddle::NAME,
            &target_variant,
            ReductionMode::Aggregate
        )
        .is_empty());
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
            parameter_contract: empty_parameter_contract(),
            reduce_fn: Some(reduce_source_to_middle_witness),
            reduce_aggregate_fn: None,
            turing: false,
        },
    );

    assert!(graph
        .find_all_paths_mode(
            AggregateChainSource::NAME,
            &source_variant,
            AggregateChainMiddle::NAME,
            &target_variant,
            ReductionMode::Aggregate
        )
        .is_empty());
    assert!(!graph
        .find_all_paths_mode(
            AggregateChainSource::NAME,
            &source_variant,
            AggregateChainMiddle::NAME,
            &target_variant,
            ReductionMode::Witness
        )
        .is_empty());
}

#[test]
fn witness_executor_does_not_imply_aggregate_capability() {
    let source_variant = BTreeMap::from([("graph".to_string(), "Source".to_string())]);
    let target_variant = BTreeMap::from([("graph".to_string(), "Target".to_string())]);
    let graph = build_two_node_graph(
        NaturalVariantProblem::NAME,
        source_variant.clone(),
        NaturalVariantProblem::NAME,
        target_variant.clone(),
        ReductionEdgeData {
            parameter_contract: empty_parameter_contract(),
            reduce_fn: Some(reduce_natural_variant_witness),
            reduce_aggregate_fn: None,
            turing: false,
        },
    );

    assert!(!graph
        .find_all_paths_mode(
            NaturalVariantProblem::NAME,
            &source_variant,
            NaturalVariantProblem::NAME,
            &target_variant,
            ReductionMode::Witness
        )
        .is_empty());
    assert!(graph
        .find_all_paths_mode(
            NaturalVariantProblem::NAME,
            &source_variant,
            NaturalVariantProblem::NAME,
            &target_variant,
            ReductionMode::Aggregate
        )
        .is_empty());
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
            parameter_contract: empty_parameter_contract(),
            reduce_fn: None,
            reduce_aggregate_fn: Some(reduce_source_to_middle_aggregate),
            turing: false,
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
        .expect("single-step path lookup should not fail")
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
            parameter_contract: empty_parameter_contract(),
            reduce_fn: Some(reduce_source_to_middle_witness),
            reduce_aggregate_fn: None,
            turing: false,
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
        .expect("witness-only edge lookup should not fail")
        .is_none());
}

#[test]
fn reduce_along_path_preserves_edge_failure() {
    let source_variant = BTreeMap::new();
    let target_variant = BTreeMap::new();
    let graph = build_two_node_graph(
        AggregateChainSource::NAME,
        source_variant.clone(),
        AggregateChainMiddle::NAME,
        target_variant.clone(),
        ReductionEdgeData {
            parameter_contract: empty_parameter_contract(),
            reduce_fn: Some(fail_source_to_middle_witness),
            reduce_aggregate_fn: None,
            turing: false,
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

    let error = match graph.reduce_along_path(&path, &AggregateChainSource as &dyn Any) {
        Err(error) => error,
        Ok(_) => panic!("registered edge failure must be returned"),
    };
    assert_eq!(
        error,
        crate::rules::ReductionError::InvalidTarget {
            source_problem: AggregateChainSource::NAME,
            target_problem: AggregateChainMiddle::NAME,
            message: "synthetic target construction failure".to_string(),
        }
    );
}

#[test]
fn test_find_indirect_path() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i64>::variant());
    let paths = graph.find_all_paths("MaximumIndependentSet", &src, "MaximumSetPacking", &dst);
    assert!(!paths.is_empty());
}

#[test]
fn test_find_direct_path_in_all_routes() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i64>::variant());
    let path = graph
        .find_all_paths("MaximumIndependentSet", &src, "MaximumSetPacking", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("direct route");
    assert_eq!(path.len(), 1); // Direct path exists
}

#[test]
fn test_knapsack_to_ilp_path_exists() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&Knapsack::variant());
    let dst = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let path = graph
        .find_all_paths("Knapsack", &src, "ILP", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("Knapsack should reduce directly to ILP");
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
    assert!(graph.has_direct_reduction::<MaximumIndependentSet<SimpleGraph, i64>, MinimumVertexCover<SimpleGraph, i64>>());
    assert!(graph.has_direct_reduction::<MinimumVertexCover<SimpleGraph, i64>, MaximumIndependentSet<SimpleGraph, i64>>());
}

#[test]
fn test_is_to_qubo_path() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let path = graph
        .find_all_paths("MaximumIndependentSet", &src, "QUBO", &dst)
        .into_iter()
        .find(|path| path.type_names() == ["MaximumIndependentSet", "MaximumSetPacking", "QUBO"])
        .expect("explicit QUBO route");
    assert!(
        path.len() > 1,
        "MIS -> QUBO should now go through a composite path"
    );
}

#[test]
fn test_variant_level_paths() {
    let graph = ReductionGraph::new();

    // Variant-level path: MaxCut<SimpleGraph, i64> -> SpinGlass<SimpleGraph, i64>
    let src = ReductionGraph::variant_to_map(
        &crate::models::graph::MaxCut::<SimpleGraph, i64>::variant(),
    );
    let dst = ReductionGraph::variant_to_map(
        &crate::models::graph::SpinGlass::<SimpleGraph, i64>::variant(),
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
fn test_find_direct_path_variants() {
    let graph = ReductionGraph::new();

    let src = ReductionGraph::variant_to_map(
        &crate::models::graph::MaxCut::<SimpleGraph, i64>::variant(),
    );
    let dst = ReductionGraph::variant_to_map(
        &crate::models::graph::SpinGlass::<SimpleGraph, i64>::variant(),
    );
    assert!(graph
        .find_all_paths("MaxCut", &src, "SpinGlass", &dst)
        .iter()
        .any(|path| path.len() == 1));

    let src = ReductionGraph::variant_to_map(&crate::models::misc::Factoring::variant());
    let dst = ReductionGraph::variant_to_map(
        &crate::models::graph::SpinGlass::<SimpleGraph, i64>::variant(),
    );
    assert!(graph
        .find_all_paths("Factoring", &src, "SpinGlass", &dst)
        .iter()
        .any(|path| path.type_names() == ["Factoring", "CircuitSAT", "SpinGlass"]));
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
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i64>::variant());
    let path = graph
        .find_all_paths("MaximumIndependentSet", &src, "MinimumVertexCover", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("direct route");

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
    assert!(json
        .nodes
        .iter()
        .any(|n| n.category == ProblemCategory::Graph));
    assert!(json
        .nodes
        .iter()
        .any(|n| n.category == ProblemCategory::Algebraic));

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
    let json_value = graph.to_json_value().unwrap();
    let json_string = graph.to_json_string().unwrap();

    // Should be valid JSON
    assert!(json_value["nodes"].is_array());
    assert!(json_value["edges"].is_array());
    assert!(json_string.contains("\"nodes\""));
    assert!(json_string.contains("\"edges\""));
    assert!(json_string.contains("MaximumIndependentSet"));
    assert!(json_string.contains("\"category\""));
    assert!(json_string.contains("\"parameters\""));
    assert!(!json_string.contains("\"overhead\""));

    // The legacy "bidirectional" field must not be present
    assert!(
        !json_string.contains("\"bidirectional\""),
        "JSON should not contain the removed 'bidirectional' field"
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
    assert!(graph.has_direct_reduction::<Satisfiability, MinimumDominatingSet<SimpleGraph, i64>>());
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
    assert!(graph.has_direct_reduction::<CircuitSAT, SpinGlass<SimpleGraph, i64>>());

    // Find path from Factoring to SpinGlass<SimpleGraph, i64>
    let src = ReductionGraph::variant_to_map(&Factoring::variant());
    let dst = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, i64>::variant());
    let paths = graph.find_all_paths("Factoring", &src, "SpinGlass", &dst);
    assert!(!paths.is_empty());
    assert!(paths
        .iter()
        .any(|path| path.type_names() == ["Factoring", "CircuitSAT", "SpinGlass"]));
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
    assert!(graph.has_direct_reduction::<MaxCut<SimpleGraph, i64>, SpinGlass<SimpleGraph, f64>>());
    assert!(graph.has_direct_reduction::<SpinGlass<SimpleGraph, f64>, MaxCut<SimpleGraph, i64>>());
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

    assert!(graph.has_direct_reduction::<NAESatisfiability, MaxCut<SimpleGraph, i64>>());
}

#[test]
fn test_maximum2satisfiability_to_maxcut_reduction_registered() {
    let graph = ReductionGraph::new();

    assert!(graph.has_direct_reduction::<Maximum2Satisfiability, MaxCut<SimpleGraph, i64>>());
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
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());

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
}

#[test]
fn test_category_comes_from_schema() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();
    let circuit = json.nodes.iter().find(|n| n.name == "CircuitSAT").unwrap();
    assert_eq!(circuit.category, ProblemCategory::Formula);
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

    assert!(graph
        .find_all_paths("CircuitSAT", &src, "Satisfiability", &dst)
        .iter()
        .any(|path| path.len() == 1));
}

#[test]
fn test_variant_to_map() {
    let variant: &[(&str, &str)] = &[("graph", "SimpleGraph"), ("weight", "i64")];
    let map = ReductionGraph::variant_to_map(variant);
    assert_eq!(map.get("graph"), Some(&"SimpleGraph".to_string()));
    assert_eq!(map.get("weight"), Some(&"i64".to_string()));
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

    // KingsSubgraph variants should appear as registered nodes.
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
fn test_variant_reduction_edges_in_json() {
    let graph = ReductionGraph::new();
    let json = graph.to_json();

    // MIS/KingsSubgraph -> MIS/UnitDiskGraph is an explicit variant reduction.
    let has_edge = json.edges.iter().any(|e| {
        json.source_node(e).name == "MaximumIndependentSet"
            && json.target_node(e).name == "MaximumIndependentSet"
            && json.source_node(e).variant.get("graph") == Some(&"KingsSubgraph".to_string())
            && json.target_node(e).variant.get("graph") == Some(&"UnitDiskGraph".to_string())
    });
    assert!(
        has_edge,
        "Variant reduction edge MIS/KingsSubgraph -> MIS/UnitDiskGraph should exist"
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
fn test_reduce_along_path_direct() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i64>::variant());
    let rpath = graph
        .find_all_paths("MaximumIndependentSet", &src, "MinimumVertexCover", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("direct route");
    // Just verify the path can produce a chain with a dummy source
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i64; 4],
    );
    let chain = graph
        .reduce_along_path(&rpath, &source as &dyn std::any::Any)
        .expect("direct reduction should not fail");
    assert!(chain.is_some());
}

#[test]
fn test_reduction_chain_direct() {
    use crate::solvers::BruteForce;
    use crate::traits::Problem;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i64>::variant());
    let rpath = graph
        .find_all_paths("MaximumIndependentSet", &src, "MinimumVertexCover", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("direct route");

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i64; 4],
    );
    let chain = graph
        .reduce_along_path(&rpath, &problem as &dyn std::any::Any)
        .unwrap()
        .unwrap();
    let target: &MinimumVertexCover<SimpleGraph, i64> = chain.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.solve(target).unwrap().unwrap();
    let source_solution = chain.extract_solution(&target_solution).unwrap();
    let metric = problem.evaluate(&source_solution).unwrap();
    assert!(metric.is_valid());
}

#[test]
fn test_reduction_chain_multi_step() {
    use crate::solvers::BruteForce;
    use crate::traits::Problem;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i64>::variant());
    let rpath = graph
        .find_all_paths("MaximumIndependentSet", &src, "MaximumSetPacking", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("direct route");

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i64; 4],
    );
    let chain = graph
        .reduce_along_path(&rpath, &problem as &dyn std::any::Any)
        .unwrap()
        .unwrap();
    let target: &MaximumSetPacking<i64> = chain.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.solve(target).unwrap().unwrap();
    let source_solution = chain.extract_solution(&target_solution).unwrap();
    let metric = problem.evaluate(&source_solution).unwrap();
    assert!(metric.is_valid());
}

#[test]
fn test_reduction_chain_with_variant_reductions() {
    use crate::models::formula::{CNFClause, KSatisfiability};
    use crate::solvers::BruteForce;
    use crate::topology::UnitDiskGraph;
    use crate::traits::Problem;

    let graph = ReductionGraph::new();

    // MIS<UnitDiskGraph, i64> -> MIS<SimpleGraph, i64> -> MVC<SimpleGraph, i64>
    // Resolve a route with exact source and target variants.
    let src_var =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<UnitDiskGraph, i64>::variant());
    let dst_var =
        ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i64>::variant());
    let rpath = graph
        .find_all_paths(
            "MaximumIndependentSet",
            &src_var,
            "MinimumVertexCover",
            &dst_var,
        )
        .into_iter()
        .find(|path| path.len() >= 2)
        .expect("variant-reduction route");
    assert!(
        rpath.len() >= 2,
        "Path should include the variant reduction (at least 2 steps)"
    );

    // Create a small UnitDiskGraph MIS problem (triangle of close nodes)
    let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (0.5, 0.0), (0.25, 0.4)], 1.0).unwrap();
    let mis = MaximumIndependentSet::new(udg, vec![1i64, 1, 1]);

    let chain = graph
        .reduce_along_path(&rpath, &mis as &dyn std::any::Any)
        .unwrap()
        .unwrap();
    let target: &MinimumVertexCover<SimpleGraph, i64> = chain.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.solve(target).unwrap().unwrap();
    let source_solution = chain.extract_solution(&target_solution).unwrap();
    let metric = mis.evaluate(&source_solution).unwrap();
    assert!(metric.is_valid());

    // Also test the KSat<K3> -> Sat -> MIS multi-step path
    // Resolve the explicit KSat -> SAT -> MIS route with exact variants.
    let ksat_src =
        ReductionGraph::variant_to_map(&KSatisfiability::<crate::variant::K3>::variant());
    let ksat_dst =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let ksat_rpath = graph
        .find_all_paths(
            "KSatisfiability",
            &ksat_src,
            "MaximumIndependentSet",
            &ksat_dst,
        )
        .into_iter()
        .find(|path| {
            path.len() == 4
                && path.type_names()
                    == ["KSatisfiability", "Satisfiability", "MaximumIndependentSet"]
        })
        .expect("explicit SAT route");

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
        .unwrap()
        .unwrap();
    let target: &MaximumIndependentSet<SimpleGraph, i64> = ksat_chain.target_problem();

    let target_solution = solver.solve(target).unwrap().unwrap();
    let original_solution = ksat_chain.extract_solution(&target_solution).unwrap();

    // Verify the extracted solution satisfies the original 3-SAT formula
    assert!(ksat.evaluate(&original_solution).unwrap());
}

#[test]
fn test_parameter_names_returns_own_fields() {
    let graph = ReductionGraph::new();

    // MIS should report its own fields (num_vertices, num_edges),
    // not the target's fields from any reduction.
    let mis_fields = graph.parameter_names("MaximumIndependentSet");
    assert!(
        mis_fields.iter().any(|field| field == "num_vertices"),
        "MIS should have num_vertices, got: {:?}",
        mis_fields
    );
    assert!(
        mis_fields.iter().any(|field| field == "num_edges"),
        "MIS should have num_edges, got: {:?}",
        mis_fields
    );
    // Should NOT contain target fields like num_vars or num_constraints
    assert!(
        !mis_fields.iter().any(|field| field == "num_constraints"),
        "MIS should not report ILP's num_constraints, got: {:?}",
        mis_fields
    );

    // QUBO should report num_vars
    let qubo_fields = graph.parameter_names("QUBO");
    assert!(
        qubo_fields.iter().any(|field| field == "num_vars"),
        "QUBO should have num_vars, got: {:?}",
        qubo_fields
    );

    // Unknown problem returns empty
    let unknown_fields = graph.parameter_names("NonExistentProblem");
    assert!(unknown_fields.is_empty());
}

#[test]
fn parameter_contract_variables_are_registered_source_fields() {
    let graph = ReductionGraph::new();

    for entry in inventory::iter::<ReductionEntry> {
        let declarations = (entry.parameter_declarations_fn)();
        let input_vars: std::collections::HashSet<_> = declarations
            .fields
            .iter()
            .flat_map(|(_, expression)| expression.variables())
            .collect();
        if input_vars.is_empty() {
            continue;
        }

        let source_fields: std::collections::HashSet<String> = graph
            .parameter_names(entry.source_name)
            .into_iter()
            .collect();

        for var in &input_vars {
            assert!(
                source_fields.contains(*var),
                "Reduction {} -> {}: parameter contract references variable '{}' \
                 which is not a known parameter field of {}. Known fields: {:?}",
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
    let variant = ReductionGraph::variant_to_map(&[("graph", "SimpleGraph"), ("weight", "i64")]);
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
fn test_compute_problem_parameters_uses_exact_variant_executor() {
    let problem = MaximumIndependentSet::<SimpleGraph, i64>::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1, 1],
    );
    let variant =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let size =
        ReductionGraph::compute_problem_parameters("MaximumIndependentSet", &variant, &problem);
    assert_eq!(size.get("num_vertices"), Some(4));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_outgoing_reductions_from_uses_exact_variant_and_mode() {
    let graph = ReductionGraph::new();
    let unit =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, One>::variant());
    let weighted =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());

    let unit_targets =
        graph.outgoing_reductions_from("MaximumIndependentSet", &unit, ReductionMode::Witness);
    assert!(unit_targets
        .iter()
        .all(|edge| edge.source_variant == unit && edge.capabilities.witness));
    assert!(unit_targets.iter().any(|edge| {
        edge.target_name == "MaximumSetPacking"
            && edge.target_variant.get("weight").map(String::as_str) == Some("One")
    }));
    assert!(!unit_targets
        .iter()
        .any(|edge| edge.target_name == "IntegralFlowBundles"));

    let weighted_targets =
        graph.outgoing_reductions_from("MaximumIndependentSet", &weighted, ReductionMode::Witness);
    assert!(weighted_targets
        .iter()
        .all(|edge| edge.source_variant == weighted && edge.capabilities.witness));
    assert!(weighted_targets
        .iter()
        .any(|edge| edge.target_name == "IntegralFlowBundles"));
    assert!(!weighted_targets.iter().any(|edge| {
        edge.target_name == "MaximumIndependentSet"
            && edge.target_variant.get("graph").map(String::as_str) == Some("KingsSubgraph")
    }));
}

#[test]
#[should_panic(expected = "registered problem variant not found")]
fn test_outgoing_reductions_from_rejects_unknown_exact_variant() {
    let graph = ReductionGraph::new();
    graph.outgoing_reductions_from(
        "MaximumIndependentSet",
        &BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i128".to_string()),
        ]),
        ReductionMode::Witness,
    );
}

#[test]
#[should_panic(expected = "unregistered exact problem variant")]
fn test_compute_problem_parameters_unknown_problem() {
    let problem = 42u32;
    ReductionGraph::compute_problem_parameters("NonExistentProblem", &BTreeMap::new(), &problem);
}

#[test]
fn test_composed_path_parameters_transform_evaluation() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i64>::variant());
    let input_size = ProblemParameters::new(vec![("num_vertices", 10), ("num_edges", 20)]);

    let path = graph
        .find_all_paths("MaximumIndependentSet", &src, "MinimumVertexCover", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("direct route");

    let transform = graph
        .compose_path_parameter_transform(&path)
        .unwrap()
        .unwrap();
    let final_size = transform
        .evaluate(&input_size)
        .expect("should evaluate composed parameter transform");

    // MIS → MVC preserves num_vertices and num_edges
    assert_eq!(final_size.get("num_vertices"), Some(10));
    assert_eq!(final_size.get("num_edges"), Some(20));
}
