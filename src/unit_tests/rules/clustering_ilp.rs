use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_bf_vs_ilp;
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
    let reduction: ReductionClusteringToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 8);
    assert_eq!(ilp.constraints().len(), 12);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
    assert!(ilp.objective().is_empty());

    let assignment_constraints = ilp
        .constraints()
        .iter()
        .filter(|constraint| constraint.comparison() == Comparison::Eq && constraint.rhs() == 1)
        .count();
    let conflict_constraints = ilp
        .constraints()
        .iter()
        .filter(|constraint| constraint.comparison() == Comparison::Le && constraint.rhs() == 1)
        .count();
    assert_eq!(assignment_constraints, 4);
    assert_eq!(conflict_constraints, 8);
}

#[test]
fn test_clustering_to_ilp_closed_loop() {
    let problem = canonical_yes_instance();
    let reduction: ReductionClusteringToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_clustering_to_ilp_solution_extraction() {
    let problem = canonical_yes_instance();
    let reduction: ReductionClusteringToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let extracted = reduction
        .extract_solution(&vec![1, 0, 1, 0, 0, 1, 0, 1])
        .unwrap();
    assert_eq!(extracted, vec![0, 0, 1, 1]);
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_clustering_to_ilp_infeasible_instance_is_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionClusteringToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    assert!(ILPSolver::new().solve(reduction.target_problem()).is_err());
}
