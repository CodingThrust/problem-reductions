use super::*;
use crate::models::algebraic::{ILP, ObjectiveSense};
use crate::models::graph::SteinerTree;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::SolutionSize;

fn canonical_instance() -> SteinerTree<SimpleGraph, i32> {
    let graph = SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (1, 3), (3, 4), (0, 3), (3, 2), (2, 4)],
    );
    SteinerTree::new(graph, vec![2, 2, 1, 1, 5, 5, 6], vec![0, 2, 4])
}

#[test]
fn test_reduction_creates_expected_ilp_shape() {
    let problem = canonical_instance();
    let reduction: ReductionSteinerTreeToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 35);
    assert_eq!(ilp.constraints.len(), 38);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert_eq!(
        ilp.objective,
        vec![
            (0, 2.0),
            (1, 2.0),
            (2, 1.0),
            (3, 1.0),
            (4, 5.0),
            (5, 5.0),
            (6, 6.0),
        ]
    );
}

#[test]
fn test_steinertree_to_ilp_closed_loop() {
    let problem = canonical_instance();
    let reduction: ReductionSteinerTreeToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();
    let best_source = bf.find_all_best(&problem);
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(problem.evaluate(&best_source[0]), SolutionSize::Valid(6));
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(6));
    assert!(problem.is_valid_solution(&extracted));
}

#[test]
fn test_solution_extraction_reads_edge_selector_prefix() {
    let problem = canonical_instance();
    let reduction: ReductionSteinerTreeToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let target_solution = vec![
        1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0,
        0, 0, 0, 0, 0, 0,
    ];

    assert_eq!(
        reduction.extract_solution(&target_solution),
        vec![1, 1, 1, 1, 0, 0, 0]
    );
}

#[test]
fn test_solve_reduced_uses_new_rule() {
    let problem = canonical_instance();
    let solution = ILPSolver::new()
        .solve_reduced(&problem)
        .expect("solve_reduced should find the Steiner tree via ILP");
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(6));
}

#[test]
#[should_panic(expected = "SteinerTree -> ILP requires strictly positive edge weights")]
fn test_reduction_rejects_negative_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = SteinerTree::new(graph, vec![1, -2, 3], vec![0, 1]);
    let _ = ReduceTo::<ILP<bool>>::reduce_to(&problem);
}

#[test]
#[should_panic(expected = "SteinerTree -> ILP requires strictly positive edge weights")]
fn test_reduction_rejects_zero_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = SteinerTree::new(graph, vec![0, 0, 0], vec![0, 1]);
    let _ = ReduceTo::<ILP<bool>>::reduce_to(&problem);
}
