use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

/// 3-vertex path: 0 -- 1 -- 2, s=0, t=2.
fn simple_path_problem() -> ShortestWeightConstrainedPath<SimpleGraph, i32> {
    ShortestWeightConstrainedPath::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![2, 3],
        vec![1, 2],
        0,
        2,
        4, // weight_bound
    )
}

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = simple_path_problem();
    let reduction: ReductionSWCPToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 2 edges => 4 arc vars + 3 order vars = 7
    assert_eq!(ilp.num_vars, 7);
    // 5*2 + 4*3 + 2 = 10 + 12 + 2 = 24
    assert_eq!(ilp.constraints.len(), 24);
    // Optimization: minimize total length
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(!ilp.objective.is_empty());
}

#[test]
fn test_shortestweightconstrainedpath_to_ilp_bf_vs_ilp() {
    // Larger instance with multiple paths
    let problem = ShortestWeightConstrainedPath::new(
        SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4)]),
        vec![2, 5, 3, 1, 2], // lengths
        vec![3, 1, 2, 4, 1], // weights
        0,
        4,
        10, // weight_bound
    );

    let bf = BruteForce::new();
    let bf_value = bf.solve(&problem);

    let reduction: ReductionSWCPToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_result = ilp_solver.solve(reduction.target_problem());

    match ilp_result {
        Some(ilp_solution) => {
            let extracted = reduction.extract_solution(&ilp_solution);
            let ilp_value = problem.evaluate(&extracted);
            // Both should agree on the optimal length
            assert_eq!(ilp_value, bf_value);
        }
        None => {
            // ILP found no feasible solution; brute force should agree
            assert_eq!(bf_value, Min(None));
        }
    }
}

#[test]
fn test_solution_extraction() {
    let problem = simple_path_problem();
    let reduction: ReductionSWCPToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Handcrafted ILP solution: path 0->1->2
    // a_{0,fwd}=1, a_{0,rev}=0, a_{1,fwd}=1, a_{1,rev}=0, o_0=0, o_1=1, o_2=2
    let target_solution = vec![1, 0, 1, 0, 0, 1, 2];
    let extracted = reduction.extract_solution(&target_solution);

    assert_eq!(extracted, vec![1, 1]);
    // length = 2 + 3 = 5
    assert_eq!(problem.evaluate(&extracted), Min(Some(5)));
}

#[test]
fn test_shortestweightconstrainedpath_to_ilp_trivial() {
    // s == t: trivially feasible (empty path, zero length)
    let problem = ShortestWeightConstrainedPath::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![2, 3],
        vec![1, 2],
        1,
        1,
        4, // weight_bound
    );
    let reduction: ReductionSWCPToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should solve the trivial s==t case");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![0, 0]);
    assert_eq!(problem.evaluate(&extracted), Min(Some(0)));
}
