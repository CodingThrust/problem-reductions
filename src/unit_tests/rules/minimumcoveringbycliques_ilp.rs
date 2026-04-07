use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MinimumCoveringByCliques;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_shape_on_path_p3() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction: ReductionMinimumCoveringByCliquesToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 12);
    assert_eq!(ilp.constraints.len(), 22);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_minimumcoveringbycliques_to_ilp_closed_loop() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)],
    ));
    let reduction: ReductionMinimumCoveringByCliquesToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source);

    let bf_value = BruteForce::new().solve(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(source.evaluate(&extracted), Min(Some(2)));
    assert_eq!(source.evaluate(&extracted), bf_value);
}

#[test]
fn test_minimumcoveringbycliques_to_ilp_empty_graph() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(3, vec![]));
    let reduction: ReductionMinimumCoveringByCliquesToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 0);
    assert_eq!(reduction.extract_solution(&[]), Vec::<usize>::new());
    assert_eq!(source.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_minimumcoveringbycliques_to_ilp_bf_vs_ilp() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)],
    ));
    let reduction: ReductionMinimumCoveringByCliquesToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source);
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}
