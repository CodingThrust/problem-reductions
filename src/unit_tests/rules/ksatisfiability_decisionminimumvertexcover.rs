use super::*;
use crate::models::decision::Decision;
use crate::models::formula::CNFClause;
use crate::models::graph::MinimumVertexCover;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_decisionminimumvertexcover_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<Decision<MinimumVertexCover<SimpleGraph, i32>>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.inner().num_vertices(), 12);
    assert_eq!(target.inner().num_edges(), 15);
    assert_eq!(target.bound(), &7);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "3SAT -> Decision MVC closed loop",
    );
}

#[test]
fn test_ksatisfiability_to_decisionminimumvertexcover_unsatisfiable() {
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
            CNFClause::new(vec![1, 1, 1]),
        ],
    );
    let reduction = ReduceTo::<Decision<MinimumVertexCover<SimpleGraph, i32>>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.bound(), &7);
    assert!(BruteForce::new().find_witness(target).is_none());
}

#[test]
fn test_ksatisfiability_to_decisionminimumvertexcover_structure_and_bound() {
    let source = KSatisfiability::<K3>::new(2, vec![CNFClause::new(vec![1, -1, 2])]);
    let reduction = ReduceTo::<Decision<MinimumVertexCover<SimpleGraph, i32>>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.inner().num_vertices(), 7);
    assert_eq!(target.inner().num_edges(), 8);
    assert_eq!(target.bound(), &4);
}

#[test]
fn test_ksatisfiability_to_decisionminimumvertexcover_extract_solution() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<Decision<MinimumVertexCover<SimpleGraph, i32>>>::reduce_to(&source);
    let cover = vec![0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0];

    assert_eq!(
        reduction.target_problem().evaluate(&cover),
        crate::types::Or(true)
    );
    assert_eq!(reduction.extract_solution(&cover), vec![0, 0, 1]);
}
