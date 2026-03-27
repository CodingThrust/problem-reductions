use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Triangle graph, k=3
    let graph = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = KClique::new(graph, 3);
    let reduction: ReductionKCliqueToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 3);
    // 1 cardinality + 0 non-edges (complete graph)
    assert_eq!(ilp.constraints.len(), 1);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_kclique_to_ilp_bf_vs_ilp() {
    // K4 graph, k=3 → has 3-clique
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = KClique::new(graph, 3);
    let reduction: ReductionKCliqueToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.find_witness(&problem).expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_solution_extraction() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = KClique::new(graph, 3);
    let reduction: ReductionKCliqueToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
    // Should select exactly 3 vertices
    assert_eq!(extracted.iter().sum::<usize>(), 3);
}

#[test]
fn test_kclique_to_ilp_trivial() {
    // Empty graph (no edges), k=1 → trivially feasible (any single vertex is a 1-clique)
    let graph = SimpleGraph::new(3, vec![]);
    let problem = KClique::new(graph, 1);
    let reduction: ReductionKCliqueToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 3);
}
