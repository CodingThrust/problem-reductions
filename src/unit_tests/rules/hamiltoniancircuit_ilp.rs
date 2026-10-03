use super::*;
use crate::solvers::{BruteForce, ILPSolveError, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_hamiltoniancircuit_to_ilp_closed_loop() {
    let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    for mask in 0..64 {
        let graph = SimpleGraph::new(
            4,
            edges
                .iter()
                .enumerate()
                .filter_map(|(index, &edge)| (mask & (1 << index) != 0).then_some(edge))
                .collect(),
        );
        let source = HamiltonianCircuit::new(graph);
        let expected = BruteForce::new().solve(&source).unwrap();
        let reduction = ReduceTo::<Target>::reduce_to(&source).unwrap();
        match ILPSolver::new().solve(reduction.target_problem()) {
            Ok(solution) => {
                assert!(expected.is_some(), "graph {mask}");
                assert!(
                    source
                        .evaluate(&reduction.extract_solution(&solution).unwrap())
                        .unwrap()
                        .0
                );
            }
            Err(ILPSolveError::Infeasible) => assert!(expected.is_none(), "graph {mask}"),
            Err(error) => panic!("{error}"),
        }
    }
}

#[test]
fn circuit_flow_requires_one_cycle_and_three_distinct_vertices() {
    for (n, edges, expected) in [
        (0, vec![], false),
        (1, vec![(0, 0)], false),
        (2, vec![(0, 1), (0, 1)], false),
        (
            6,
            vec![(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)],
            false,
        ),
        (3, vec![(0, 0), (0, 1), (0, 1), (1, 2), (0, 2)], true),
    ] {
        let source = HamiltonianCircuit::new(SimpleGraph::new(n, edges));
        let reduction = ReduceTo::<Target>::reduce_to(&source).unwrap();
        match ILPSolver::new().solve(reduction.target_problem()) {
            Ok(solution) => {
                assert!(expected);
                assert!(
                    source
                        .evaluate(&reduction.extract_solution(&solution).unwrap())
                        .unwrap()
                        .0
                );
                assert!(reduction
                    .extract_solution(&vec![0; solution.len()])
                    .is_err());
                assert!(reduction
                    .extract_solution(&vec![0; solution.len() + 1])
                    .is_err());
            }
            Err(ILPSolveError::Infeasible) => assert!(!expected),
            Err(error) => panic!("{error}"),
        }
    }
}
