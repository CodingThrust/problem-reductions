use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::AcyclicPartition;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn small_instance() -> AcyclicPartition<i32> {
    // Chain 0->1->2->3, unit weights, unit arc costs, B=3, K=2
    AcyclicPartition::new(
        DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1, 1],
        vec![1, 1, 1],
        3,
        2,
    )
}

#[test]
fn test_acyclicpartition_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionAcyclicPartitionToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    // Solve source with brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&source);
    assert!(!bf_solutions.is_empty(), "source should be satisfiable");

    // Solve ILP
    let ilp_solver = ILPSolver::new();
    let ilp_sol = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);

    assert!(
        source.evaluate(&extracted).0,
        "extracted solution must be valid"
    );
}

#[test]
fn test_reduction_num_vars() {
    let source = small_instance();
    let reduction: ReductionAcyclicPartitionToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    // n=4, m=3: n^2 + m*n + m + 2*n = 16 + 12 + 3 + 8 = 39
    assert_eq!(ilp.num_vars, 39);
}

#[test]
fn test_extract_solution() {
    let source = small_instance();
    let reduction: ReductionAcyclicPartitionToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);
    assert_eq!(extracted.len(), 4);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_infeasible_instance() {
    // Cycle 0->1->2->0, B=1, K=0.
    // Each partition can hold weight <= 1 (one vertex each),
    // so 3 separate partitions with crossing cost = 3 > K=0.
    // Can't merge either since weight > B=1.
    let source = AcyclicPartition::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1, 1, 1],
        vec![1, 1, 1],
        1,
        0,
    );
    let reduction: ReductionAcyclicPartitionToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    assert!(solver.solve(ilp).is_none());
}
