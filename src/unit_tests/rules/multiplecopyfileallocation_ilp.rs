use super::*;
use crate::models::graph::MultipleCopyFileAllocation;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 3-vertex path: 0 - 1 - 2
    let problem = MultipleCopyFileAllocation::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1, 1],
        vec![5, 5, 5],
    );
    let reduction: ReductionMCFAToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // num_vars = n + n^2 = 3 + 9 = 12
    assert_eq!(ilp.num_vars(), 12, "n + n^2 variables");
    // num_constraints = n (assignment) + n^2 (capacity) = 3 + 9 = 12
    assert_eq!(
        ilp.constraints().len(),
        12,
        "assignment + capacity constraints"
    );
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_multiplecopyfileallocation_to_ilp_bf_vs_ilp() {
    // Small instance: 3-vertex path, place copy at center
    // storage=[5,5,5], usage=[1,1,1]
    // Optimal: copy at vertex 1, cost = 5 + 1 + 0 + 1 = 7
    let problem = MultipleCopyFileAllocation::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1, 1],
        vec![5, 5, 5],
    );
    let reduction: ReductionMCFAToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.solve(&problem).unwrap().expect("should have a witness");
    assert!(problem.evaluate(&bf_witness).unwrap().0.is_some());

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(
        extracted.len(),
        3,
        "extracted solution has one entry per vertex"
    );
    assert!(problem.evaluate(&extracted).unwrap().0.is_some());
}

#[test]
fn test_solution_extraction() {
    // 3-vertex path: copy at vertex 1 (index 1 = 1)
    let problem = MultipleCopyFileAllocation::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1, 1],
        vec![5, 5, 5],
    );
    let reduction: ReductionMCFAToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    // Manually construct a valid ILP solution:
    // x = [0, 1, 0]; y_{0,1}=1 y_{1,1}=1 y_{2,1}=1, rest 0
    let target_solution = vec![
        0, 1, 0, // x_0, x_1, x_2
        0, 1, 0, // y_{0,0}, y_{0,1}, y_{0,2}
        0, 1, 0, // y_{1,0}, y_{1,1}, y_{1,2}
        0, 1, 0, // y_{2,0}, y_{2,1}, y_{2,2}
    ];
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![false, true, false]);
    assert_eq!(problem.evaluate(&extracted).unwrap(), Min(Some(7)));
}

#[test]
fn test_multiplecopyfileallocation_to_ilp_trivial() {
    // Single vertex, copy must be placed at itself, zero access cost.
    let problem = MultipleCopyFileAllocation::new(SimpleGraph::new(1, vec![]), vec![2], vec![3]);
    let reduction: ReductionMCFAToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // num_vars = 1 + 1 = 2
    assert_eq!(ilp.num_vars(), 2);
    // num_constraints = 1 (assignment) + 1 (capacity) = 2
    assert_eq!(ilp.constraints().len(), 2);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted.len(), 1);
    assert_eq!(problem.evaluate(&extracted).unwrap(), Min(Some(3)));
}

#[test]
fn test_multiplecopyfileallocation_unreachable_assignments_are_forbidden() {
    let problem = MultipleCopyFileAllocation::new(
        SimpleGraph::new(4, vec![(0, 1)]),
        vec![1, 0, 0, 0],
        vec![8, 4, 6, 2],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).unwrap();
    let direct = BruteForce::new().solve(&problem).unwrap().unwrap();
    let target = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    let extracted = reduction.extract_solution(&target).unwrap();

    assert_eq!(
        problem.evaluate(&extracted).unwrap(),
        problem.evaluate(&direct).unwrap()
    );
    assert!(extracted[2] && extracted[3]);
}
