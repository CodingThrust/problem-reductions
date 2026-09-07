use super::*;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Triangle with unit lengths
    let problem = LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
    );
    let reduction: ReductionLongestCircuitToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // Three edge, three vertex, three root, and eighteen flow variables.
    assert_eq!(ilp.num_vars(), 27);
    assert_eq!(ilp.num_constraints(), 41);
    assert_eq!(ilp.sense(), ObjectiveSense::Maximize);
}

#[test]
fn test_longestcircuit_to_ilp_closed_loop() {
    // Hexagon with varying edge lengths
    let problem = LongestCircuit::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 5),
                (5, 0),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 5),
            ],
        ),
        vec![3, 2, 4, 1, 5, 2, 3, 2, 1, 2],
    );
    // BruteForce on source to verify feasibility
    let bf = BruteForce::new();
    let bf_solution = bf
        .solve(&problem)
        .unwrap()
        .expect("brute-force should find a solution");
    assert!(problem.evaluate(&bf_solution).unwrap().0.is_some());

    // Solve via ILP
    let reduction: ReductionLongestCircuitToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert!(
        problem.evaluate(&extracted).unwrap().0.is_some(),
        "ILP solution should be a valid circuit"
    );
    assert_eq!(
        problem.evaluate(&extracted).unwrap(),
        problem.evaluate(&bf_solution).unwrap()
    );
}

#[test]
fn test_longestcircuit_to_ilp_triangle() {
    // Triangle: all edges length 1
    let problem = LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
    );
    let reduction: ReductionLongestCircuitToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_solution_extraction() {
    let problem = LongestCircuit::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2), (1, 3)]),
        vec![1, 1, 1, 1, 2, 2],
    );
    let reduction: ReductionLongestCircuitToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert!(problem.evaluate(&extracted).unwrap().0.is_some());
}

#[test]
fn test_longestcircuit_to_ilp_bf_vs_ilp() {
    let problem = LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
    );
    let reduction: ReductionLongestCircuitToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_longestcircuit_to_ilp_cycle_excludes_any_vertex() {
    // A leaf attached to a triangle; every vertex label can be the leaf.
    for leaf in 0..4 {
        let [a, b, c] = [1, 2, 3].map(|offset| (leaf + offset) % 4);
        let problem = LongestCircuit::new(
            SimpleGraph::new(4, vec![(leaf, a), (a, b), (b, c), (c, a)]),
            vec![10, 1, 2, 3],
        );
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).unwrap();
        let target_solution = ILPSolver::new().solve(reduction.target_problem()).unwrap();
        let extracted = reduction.extract_solution(&target_solution).unwrap();
        assert_eq!(extracted, vec![false, true, true, true]);
    }
}

#[test]
fn test_longestcircuit_to_ilp_selects_one_best_cycle() {
    // The longer triangle excludes vertex 0, with or without a bridge.
    for connected in [false, true] {
        let mut edges = vec![(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)];
        let mut lengths = vec![1, 1, 1, 4, 5, 6];
        if connected {
            edges.push((2, 3));
            lengths.push(20);
        }
        let problem = LongestCircuit::new(SimpleGraph::new(6, edges), lengths);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).unwrap();
        let target_solution = ILPSolver::new().solve(reduction.target_problem()).unwrap();
        let extracted = reduction.extract_solution(&target_solution).unwrap();
        assert_eq!(
            problem.evaluate(&extracted).unwrap(),
            crate::types::Max(Some(15))
        );
    }
}

#[test]
fn test_longestcircuit_to_ilp_acyclic_graphs() {
    for n in 0..4 {
        let edges: Vec<_> = (1..n).map(|v| (v - 1, v)).collect();
        let m = edges.len();
        let problem = LongestCircuit::new(SimpleGraph::new(n, edges), vec![1; m]);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).unwrap();
        let target = reduction.target_problem();
        assert_eq!(target.num_vars(), m + 2 * n + 2 * m * n);
        assert_eq!(target.num_constraints(), 2 + n + 2 * n * n + 2 * m * n);
        assert!(BruteForce::new().solve(&problem).unwrap().is_none());
        assert!(matches!(
            ILPSolver::new().solve(target),
            Err(crate::solvers::ILPSolveError::Infeasible)
        ));
    }
}
