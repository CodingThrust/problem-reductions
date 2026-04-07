#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use super::*;
#[cfg(feature = "ilp-solver")]
use crate::models::algebraic::ILP;
use crate::models::formula::CNFClause;
#[cfg(feature = "example-db")]
use crate::models::graph::DirectedTwoCommodityIntegralFlow;
use crate::rules::{ReduceTo, ReductionGraph, ReductionResult};
#[cfg(feature = "ilp-solver")]
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::variant::K3;

fn issue_example() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
        ],
    )
}

fn unsatisfiable_instance() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    )
}

fn all_assignments(num_vars: usize) -> Vec<Vec<usize>> {
    (0..(1usize << num_vars))
        .map(|mask| {
            (0..num_vars)
                .map(|bit| usize::from(((mask >> bit) & 1) == 1))
                .collect()
        })
        .collect()
}

#[cfg(feature = "ilp-solver")]
fn solve_target_via_ilp(problem: &crate::models::graph::DirectedTwoCommodityIntegralFlow) -> Option<Vec<usize>> {
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(problem);
    let ilp_solution = ILPSolver::new().solve(reduction.target_problem())?;
    let extracted = reduction.extract_solution(&ilp_solution);
    problem.evaluate(&extracted).0.then_some(extracted)
}

#[test]
fn test_ksatisfiability_to_directedtwocommodityintegralflow_structure() {
    let source = issue_example();
    let reduction = ReduceTo::<crate::models::graph::DirectedTwoCommodityIntegralFlow>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 36);
    assert_eq!(target.num_arcs(), 48);
    assert_eq!(target.requirement_1(), 1);
    assert_eq!(target.requirement_2(), 2);
    assert_eq!(target.max_capacity(), 1);
}

#[test]
fn test_ksatisfiability_to_directedtwocommodityintegralflow_assignment_encoding_matches_truth_table() {
    let source = issue_example();
    let reduction = ReduceTo::<crate::models::graph::DirectedTwoCommodityIntegralFlow>::reduce_to(&source);
    let target = reduction.target_problem();

    for assignment in all_assignments(source.num_vars()) {
        let flow = reduction.encode_assignment(&assignment);
        assert_eq!(
            source.evaluate(&assignment).0,
            target.evaluate(&flow).0,
            "assignment {:?} should preserve satisfiability through the encoded flow",
            assignment
        );
    }
}

#[test]
fn test_ksatisfiability_to_directedtwocommodityintegralflow_extract_solution_from_encoded_witness() {
    let source = issue_example();
    let reduction = ReduceTo::<crate::models::graph::DirectedTwoCommodityIntegralFlow>::reduce_to(&source);

    let assignment = vec![1, 1, 0];
    let flow = reduction.encode_assignment(&assignment);
    assert!(reduction.target_problem().evaluate(&flow).0);
    assert_eq!(reduction.extract_solution(&flow), assignment);
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ksatisfiability_to_directedtwocommodityintegralflow_closed_loop() {
    let source = issue_example();
    let reduction = ReduceTo::<crate::models::graph::DirectedTwoCommodityIntegralFlow>::reduce_to(&source);

    let target_solution = solve_target_via_ilp(reduction.target_problem())
        .expect("satisfiable source instance should produce a feasible two-commodity flow");

    assert!(reduction.target_problem().evaluate(&target_solution).0);

    let extracted = reduction.extract_solution(&target_solution);
    assert!(source.evaluate(&extracted).0);
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ksatisfiability_to_directedtwocommodityintegralflow_unsatisfiable() {
    let source = unsatisfiable_instance();
    let reduction = ReduceTo::<crate::models::graph::DirectedTwoCommodityIntegralFlow>::reduce_to(&source);
    let maybe_solution = solve_target_via_ilp(reduction.target_problem());
    assert!(
        maybe_solution.is_none(),
        "unsatisfiable 3SAT instance should produce an infeasible two-commodity flow"
    );
}

#[test]
fn test_reduction_graph_registers_ksatisfiability_to_directedtwocommodityintegralflow() {
    let graph = ReductionGraph::new();
    assert!(graph.has_direct_reduction_by_name(
        "KSatisfiability",
        "DirectedTwoCommodityIntegralFlow",
    ));
}

#[cfg(feature = "example-db")]
#[test]
fn test_ksatisfiability_to_directedtwocommodityintegralflow_canonical_example_spec() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "ksatisfiability_to_directedtwocommodityintegralflow")
        .expect("missing canonical 3SAT -> DirectedTwoCommodityIntegralFlow example spec")
        .build)();

    assert_eq!(example.source.problem, "KSatisfiability");
    assert_eq!(example.target.problem, "DirectedTwoCommodityIntegralFlow");
    assert_eq!(example.target.instance["requirement_1"], serde_json::json!(1));
    assert_eq!(example.target.instance["requirement_2"], serde_json::json!(2));
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(example.solutions[0].source_config, vec![1, 1, 0]);

    let source: KSatisfiability<K3> = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: DirectedTwoCommodityIntegralFlow =
        serde_json::from_value(example.target.instance.clone())
            .expect("target example deserializes");

    assert!(source
        .evaluate(&example.solutions[0].source_config)
        .is_valid());
    assert!(target
        .evaluate(&example.solutions[0].target_config)
        .is_valid());
}
