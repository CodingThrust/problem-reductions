#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use super::*;
use crate::models::formula::CNFClause;
#[cfg(feature = "example-db")]
use crate::models::graph::IntegralFlowHomologousArcs;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionGraph, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn issue_example() -> Satisfiability {
    Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![-1, 3]),
            CNFClause::new(vec![-2, -3]),
            CNFClause::new(vec![1, 3]),
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

#[test]
fn test_satisfiability_to_integralflowhomologousarcs_closed_loop() {
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<IntegralFlowHomologousArcs>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SAT->IntegralFlowHomologousArcs closed loop",
    );
}

#[test]
fn test_satisfiability_to_integralflowhomologousarcs_issue_example_structure() {
    let source = issue_example();
    let reduction = ReduceTo::<IntegralFlowHomologousArcs>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 43);
    assert_eq!(target.num_arcs(), 51);
    assert_eq!(target.requirement(), 3);
    assert_eq!(target.homologous_pairs().len(), 8);
    assert_eq!(target.max_capacity(), 1);
}

#[test]
fn test_satisfiability_to_integralflowhomologousarcs_issue_example_assignment_encoding() {
    let source = issue_example();
    let reduction = ReduceTo::<IntegralFlowHomologousArcs>::reduce_to(&source);
    let target = reduction.target_problem();

    let satisfying_assignment = vec![1, 0, 1];
    let satisfying_flow = reduction.encode_assignment(&satisfying_assignment);
    assert!(target.evaluate(&satisfying_flow).0);
    assert_eq!(
        reduction.extract_solution(&satisfying_flow),
        satisfying_assignment
    );

    let unsatisfying_assignment = vec![1, 1, 1];
    let unsatisfying_flow = reduction.encode_assignment(&unsatisfying_assignment);
    assert!(!target.evaluate(&unsatisfying_flow).0);
}

#[test]
fn test_satisfiability_to_integralflowhomologousarcs_issue_example_truth_table_matches_flow() {
    let source = issue_example();
    let reduction = ReduceTo::<IntegralFlowHomologousArcs>::reduce_to(&source);
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
fn test_satisfiability_to_integralflowhomologousarcs_unsat_source_has_no_target_witness() {
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);
    let reduction = ReduceTo::<IntegralFlowHomologousArcs>::reduce_to(&source);

    assert_eq!(
        BruteForce::new().find_witness(reduction.target_problem()),
        None
    );
}

#[test]
fn test_reduction_graph_registers_satisfiability_to_integralflowhomologousarcs() {
    let graph = ReductionGraph::new();
    assert!(graph.has_direct_reduction_by_name("Satisfiability", "IntegralFlowHomologousArcs",));
}

#[cfg(feature = "example-db")]
#[test]
fn test_satisfiability_to_integralflowhomologousarcs_canonical_example_spec() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "satisfiability_to_integralflowhomologousarcs")
        .expect("missing canonical SAT -> IFHA example spec")
        .build)();

    assert_eq!(example.source.problem, "Satisfiability");
    assert_eq!(example.target.problem, "IntegralFlowHomologousArcs");
    assert_eq!(example.target.instance["requirement"], serde_json::json!(3));
    assert_eq!(
        example.target.instance["homologous_pairs"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(example.solutions[0].source_config, vec![1, 0, 1]);

    let source: Satisfiability = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: IntegralFlowHomologousArcs =
        serde_json::from_value(example.target.instance.clone())
            .expect("target example deserializes");

    assert!(source
        .evaluate(&example.solutions[0].source_config)
        .is_valid());
    assert!(target
        .evaluate(&example.solutions[0].target_config)
        .is_valid());
}
