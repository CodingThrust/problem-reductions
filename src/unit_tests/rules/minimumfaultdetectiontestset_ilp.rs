use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense};
use crate::models::misc::MinimumFaultDetectionTestSet;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Min;

fn issue_problem() -> MinimumFaultDetectionTestSet {
    MinimumFaultDetectionTestSet::new(
        7,
        vec![
            (0, 2),
            (0, 3),
            (1, 3),
            (1, 4),
            (2, 5),
            (3, 5),
            (3, 6),
            (4, 6),
        ],
        vec![0, 1],
        vec![5, 6],
    )
}

#[test]
fn test_reduction_creates_covering_ilp() {
    let problem = issue_problem();
    let reduction: ReductionMFDTSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 4);
    assert_eq!(ilp.constraints.len(), 3);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert_eq!(ilp.objective, vec![(0, 1.0), (1, 1.0), (2, 1.0), (3, 1.0)]);

    assert_eq!(ilp.constraints[0].cmp, Comparison::Ge);
    assert_eq!(ilp.constraints[0].rhs, 1.0);
    assert_eq!(ilp.constraints[0].terms, vec![(0, 1.0)]);

    assert_eq!(ilp.constraints[1].cmp, Comparison::Ge);
    assert_eq!(ilp.constraints[1].rhs, 1.0);
    assert_eq!(
        ilp.constraints[1].terms,
        vec![(0, 1.0), (1, 1.0), (2, 1.0), (3, 1.0)]
    );

    assert_eq!(ilp.constraints[2].cmp, Comparison::Ge);
    assert_eq!(ilp.constraints[2].rhs, 1.0);
    assert_eq!(ilp.constraints[2].terms, vec![(3, 1.0)]);
}

#[test]
fn test_minimumfaultdetectiontestset_to_ilp_closed_loop() {
    let problem = issue_problem();
    let reduction: ReductionMFDTSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![1, 0, 0, 1]);
    assert_eq!(problem.evaluate(&extracted), Min(Some(2)));
    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_reduction_is_infeasible_when_an_internal_vertex_has_no_covering_pair() {
    let problem = MinimumFaultDetectionTestSet::new(3, vec![], vec![0], vec![2]);
    let reduction: ReductionMFDTSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 1);
    assert_eq!(ilp.constraints.len(), 1);
    assert!(ilp.constraints[0].terms.is_empty());
    assert_eq!(ilp.constraints[0].cmp, Comparison::Ge);
    assert_eq!(ilp.constraints[0].rhs, 1.0);

    assert_eq!(problem.evaluate(&[0]), Min(None));
    assert_eq!(problem.evaluate(&[1]), Min(None));
    assert!(ILPSolver::new().solve(ilp).is_none());
}

#[test]
fn test_reduction_handles_instances_without_internal_vertices() {
    let problem = MinimumFaultDetectionTestSet::new(2, vec![(0, 1)], vec![0], vec![1]);
    let reduction: ReductionMFDTSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 1);
    assert!(ilp.constraints.is_empty());

    let ilp_solution = ILPSolver::new()
        .solve(ilp)
        .expect("ILP should be feasible when there are no internal vertices");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![0]);
    assert_eq!(problem.evaluate(&extracted), Min(Some(0)));
}
