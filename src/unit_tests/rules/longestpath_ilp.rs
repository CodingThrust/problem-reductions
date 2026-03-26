use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

fn issue_problem() -> LongestPath<SimpleGraph, i32> {
    LongestPath::new(
        SimpleGraph::new(
            7,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (2, 4),
                (3, 5),
                (4, 5),
                (4, 6),
                (5, 6),
                (1, 6),
            ],
        ),
        vec![3, 2, 4, 1, 5, 2, 3, 2, 4, 1],
        0,
        6,
    )
}

fn simple_path_problem() -> LongestPath<SimpleGraph, i32> {
    LongestPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![2, 3], 0, 2)
}

#[test]
fn test_reduction_creates_expected_ilp_shape() {
    let problem = simple_path_problem();
    let reduction: ReductionLongestPathToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 7);
    assert_eq!(ilp.constraints.len(), 23);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);

    let mut objective = vec![0.0; ilp.num_vars];
    for &(var, coeff) in &ilp.objective {
        objective[var] = coeff;
    }

    assert_eq!(objective[0], 2.0);
    assert_eq!(objective[1], 2.0);
    assert_eq!(objective[2], 3.0);
    assert_eq!(objective[3], 3.0);
    assert_eq!(objective[4], 0.0);
}

#[test]
fn test_longestpath_to_ilp_closed_loop_on_issue_example() {
    let problem = issue_problem();
    let brute_force = BruteForce::new();
    let best = brute_force
        .find_witness(&problem)
        .expect("brute-force optimum");
    let best_value = problem.evaluate(&best);
    assert_eq!(best_value, Max(Some(20)));

    let reduction: ReductionLongestPathToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.is_valid_solution(&extracted));
    assert_eq!(problem.evaluate(&extracted), best_value);
}

#[test]
fn test_solution_extraction_from_handcrafted_ilp_assignment() {
    let problem = simple_path_problem();
    let reduction: ReductionLongestPathToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // x_{0->1}, x_{1->0}, x_{1->2}, x_{2->1}, o_0, o_1, o_2
    let target_solution = vec![1, 0, 1, 0, 0, 1, 2];
    let extracted = reduction.extract_solution(&target_solution);

    assert_eq!(extracted, vec![1, 1]);
    assert_eq!(problem.evaluate(&extracted), Max(Some(5)));
}

#[test]
fn test_source_equals_target_uses_empty_path() {
    let problem = LongestPath::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![4, 5, 6],
        1,
        1,
    );
    let reduction: ReductionLongestPathToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should solve the trivial empty-path case");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![0, 0, 0]);
    assert_eq!(problem.evaluate(&extracted), Max(Some(0)));
}

#[test]
fn test_longestpath_to_ilp_bf_vs_ilp() {
    let problem = simple_path_problem();
    let reduction: ReductionLongestPathToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let bf_value = BruteForce::new().solve(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), bf_value);
}
