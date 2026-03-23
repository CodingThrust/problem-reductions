use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MinimumSumMulticenter;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 3-vertex path: 0 - 1 - 2, unit weights, K=1
    let problem = MinimumSumMulticenter::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1i32; 3],
        vec![1i32; 2],
        1,
    );
    let reduction: ReductionMSMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // num_vars = n + n^2 = 3 + 9 = 12
    assert_eq!(ilp.num_vars, 12, "n + n^2 variables");
    // num_constraints = 1 (cardinality) + n (assignment) + n^2 (capacity)
    //                 = 1 + 3 + 9 = 13
    assert_eq!(
        ilp.constraints.len(),
        13,
        "cardinality + assignment + capacity constraints"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_minimumsummulticenter_to_ilp_bf_vs_ilp() {
    // 3-vertex path: 0 - 1 - 2, unit weights, K=1
    // Optimal: center at vertex 1, total distance = 1+0+1 = 2
    let problem = MinimumSumMulticenter::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1i32; 3],
        vec![1i32; 2],
        1,
    );
    let reduction: ReductionMSMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.find_witness(&problem).expect("should have a solution");
    let bf_cost = problem.evaluate(&bf_witness).unwrap();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        extracted.len(),
        3,
        "extracted solution has one entry per vertex"
    );

    let ilp_cost = problem.evaluate(&extracted).unwrap();
    // Both should find the same optimal cost
    assert_eq!(
        bf_cost, ilp_cost,
        "BruteForce and ILP should agree on optimal cost"
    );
    assert_eq!(bf_cost, 2, "optimal cost is 2 (center at vertex 1)");
}

#[test]
fn test_solution_extraction() {
    // 3-vertex path: center at vertex 1
    let problem = MinimumSumMulticenter::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1i32; 3],
        vec![1i32; 2],
        1,
    );
    let reduction: ReductionMSMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Manually construct a valid ILP solution:
    // x = [0, 1, 0]; each vertex assigned to center 1
    let target_solution = vec![
        0, 1, 0, // x_0, x_1, x_2
        0, 1, 0, // y_{0,0}, y_{0,1}, y_{0,2}
        0, 1, 0, // y_{1,0}, y_{1,1}, y_{1,2}
        0, 1, 0, // y_{2,0}, y_{2,1}, y_{2,2}
    ];
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted, vec![0, 1, 0]);
    assert_eq!(problem.evaluate(&extracted).unwrap(), 2);
}

#[test]
fn test_minimumsummulticenter_to_ilp_trivial() {
    // Single vertex, K=1: the only vertex must be the center, distance = 0
    let problem = MinimumSumMulticenter::new(SimpleGraph::new(1, vec![]), vec![5i32], vec![], 1);
    let reduction: ReductionMSMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // num_vars = 1 + 1 = 2
    assert_eq!(ilp.num_vars, 2);
    // num_constraints = 1 (cardinality) + 1 (assignment) + 1 (capacity) = 3
    assert_eq!(ilp.constraints.len(), 3);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted, vec![1]);
    assert_eq!(problem.evaluate(&extracted).unwrap(), 0);
}
