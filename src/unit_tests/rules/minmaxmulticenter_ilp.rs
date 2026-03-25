use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MinMaxMulticenter;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 3-vertex path: 0 - 1 - 2, unit weights/lengths, K=1
    let problem = MinMaxMulticenter::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1i32; 3],
        vec![1i32; 2],
        1,
    );
    let reduction: ReductionMMCToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // num_vars = n + n^2 + 1 = 3 + 9 + 1 = 13
    assert_eq!(ilp.num_vars, 13, "n + n^2 + 1 variables");
    // num_constraints = 1 (cardinality) + n (assignment) + n^2 (link) + n (x bounds) + n^2 (y bounds) + n (minimax)
    //                 = 1 + 3 + 9 + 3 + 9 + 3 = 28
    assert_eq!(
        ilp.constraints.len(),
        28,
        "cardinality + assignment + link + binary bounds + minimax constraints"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    // Objective should minimize z (last variable)
    assert_eq!(ilp.objective, vec![(12, 1.0)]);
}

#[test]
fn test_minmaxmulticenter_to_ilp_bf_vs_ilp() {
    // 3-vertex path: 0 - 1 - 2, unit weights/lengths, K=1
    // Optimal: place center at vertex 1, max distance = 1
    let problem = MinMaxMulticenter::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1i32; 3],
        vec![1i32; 2],
        1,
    );
    let reduction: ReductionMMCToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.find_witness(&problem).expect("should have optimal");
    assert_eq!(problem.evaluate(&bf_witness), Min(Some(1)));

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        extracted.len(),
        3,
        "extracted solution has one entry per vertex"
    );
    assert_eq!(problem.evaluate(&extracted), Min(Some(1)));
}

#[test]
fn test_solution_extraction() {
    // 3-vertex path: center at vertex 1
    let problem = MinMaxMulticenter::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1i32; 3],
        vec![1i32; 2],
        1,
    );
    let reduction: ReductionMMCToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Manually construct a valid ILP solution:
    // x = [0, 1, 0]; each vertex assigned to center 1; z = 1
    let target_solution = vec![
        0, 1, 0, // x_0, x_1, x_2
        0, 1, 0, // y_{0,0}, y_{0,1}, y_{0,2}
        0, 1, 0, // y_{1,0}, y_{1,1}, y_{1,2}
        0, 1, 0, // y_{2,0}, y_{2,1}, y_{2,2}
        1, // z
    ];
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted, vec![0, 1, 0]);
    assert_eq!(problem.evaluate(&extracted), Min(Some(1)));
}

#[test]
fn test_minmaxmulticenter_to_ilp_weighted() {
    // Single weighted edge with length 100. With k=1, optimal = 100.
    let problem = MinMaxMulticenter::new(
        SimpleGraph::new(2, vec![(0, 1)]),
        vec![1i32; 2],
        vec![100i32],
        1,
    );

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem).expect("should have optimal");
    let bf_value = problem.evaluate(&bf_witness);
    assert_eq!(bf_value, Min(Some(100)));

    let reduction: ReductionMMCToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Min(Some(100)));
}

#[test]
fn test_minmaxmulticenter_to_ilp_trivial() {
    // Single vertex, K=1: the only vertex is the center, distance = 0
    let problem = MinMaxMulticenter::new(SimpleGraph::new(1, vec![]), vec![5i32], vec![], 1);
    let reduction: ReductionMMCToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // num_vars = 1 + 1 + 1 = 3
    assert_eq!(ilp.num_vars, 3);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted.len(), 1);
    assert_eq!(problem.evaluate(&extracted), Min(Some(0)));
}
