use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MinimumCoveringByCliques;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_shape_with_repeated_edges() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(2, vec![(0, 1), (0, 1)]));
    let reduction: ReductionMinimumCoveringByCliquesToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 10);
    // Four activation rows, no missing pairs, twelve product rows, two coverage rows.
    assert_eq!(ilp.constraints().len(), 18);
    crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_minimumcoveringbycliques_to_ilp_closed_loop() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)],
    ));
    let reduction: ReductionMinimumCoveringByCliquesToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");

    let bf_value_solution = BruteForce::new().solve(&source).unwrap().unwrap();

    let bf_value = source.evaluate(&bf_value_solution).unwrap();
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(2)));
    assert_eq!(source.evaluate(&extracted).unwrap(), bf_value);
}

#[test]
fn test_minimumcoveringbycliques_to_ilp_empty_graph() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(3, vec![]));
    let reduction: ReductionMinimumCoveringByCliquesToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 0);
    assert_eq!(ilp.constraints().len(), 0);
    assert_eq!(
        reduction.extract_solution(&vec![]).unwrap(),
        Vec::<usize>::new()
    );
    assert_eq!(source.evaluate(&vec![]).unwrap(), Min(Some(0)));
}

#[test]
fn test_minimumcoveringbycliques_to_ilp_bf_vs_ilp() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)],
    ));
    let reduction: ReductionMinimumCoveringByCliquesToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}
