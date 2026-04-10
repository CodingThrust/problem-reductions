use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Or;

fn canonical_yes_instance() -> Clustering {
    Clustering::new(
        vec![
            vec![0, 1, 3, 3],
            vec![1, 0, 3, 3],
            vec![3, 3, 0, 1],
            vec![3, 3, 1, 0],
        ],
        2,
        1,
    )
}

fn infeasible_instance() -> Clustering {
    Clustering::new(vec![vec![0, 3, 1], vec![3, 0, 1], vec![1, 1, 0]], 1, 1)
}

#[test]
fn test_clustering_to_ilp_structure() {
    let problem = canonical_yes_instance();
    let reduction: ReductionClusteringToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 8);
    assert_eq!(ilp.constraints.len(), 12);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());

    let assignment_constraints = ilp
        .constraints
        .iter()
        .filter(|constraint| constraint.cmp == Comparison::Eq && constraint.rhs == 1.0)
        .count();
    let conflict_constraints = ilp
        .constraints
        .iter()
        .filter(|constraint| constraint.cmp == Comparison::Le && constraint.rhs == 1.0)
        .count();
    assert_eq!(assignment_constraints, 4);
    assert_eq!(conflict_constraints, 8);
}

#[test]
fn test_clustering_to_ilp_closed_loop() {
    let problem = canonical_yes_instance();
    let reduction: ReductionClusteringToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "Clustering->ILP closed loop",
    );
}

#[test]
fn test_clustering_to_ilp_solution_extraction() {
    let problem = canonical_yes_instance();
    let reduction: ReductionClusteringToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let extracted = reduction.extract_solution(&[1, 0, 1, 0, 0, 1, 0, 1]);
    assert_eq!(extracted, vec![0, 0, 1, 1]);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_clustering_to_ilp_infeasible_instance_is_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionClusteringToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert!(ILPSolver::new().solve(reduction.target_problem()).is_none());
}
