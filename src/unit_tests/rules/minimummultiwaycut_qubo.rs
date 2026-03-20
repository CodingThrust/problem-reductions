use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_minimummultiwaycut_to_qubo_closed_loop() {
    // 5 vertices, terminals {0,2,4}, 6 edges with weights [2,3,1,2,4,5]
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let source = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    assert!(!qubo_solutions.is_empty(), "QUBO solver found no solutions");

    // All QUBO optimal solutions should extract to valid source solutions with cost 8
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let metric = source.evaluate(&extracted);
        assert_eq!(metric, SolutionSize::Valid(8));
    }
}

#[test]
fn test_minimummultiwaycut_to_qubo_small() {
    // 3 vertices, 2 terminals {0,2}, edges [(0,1),(1,2)] with weights [1,1]
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let source = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1, 1]);

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    assert!(!qubo_solutions.is_empty(), "QUBO solver found no solutions");

    // All solutions should extract to valid cuts
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let metric = source.evaluate(&extracted);
        // With 2 terminals and path 0-1-2, minimum cut is 1 (cut either edge)
        assert_eq!(metric, SolutionSize::Valid(1));
    }
}

#[test]
fn test_minimummultiwaycut_to_qubo_sizes() {
    // 5 vertices, 3 terminals => QUBO has k*n = 3*5 = 15 variables
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let source = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    assert_eq!(reduction.target_problem().num_variables(), 15);
}

#[test]
fn test_minimummultiwaycut_to_qubo_terminal_pinning() {
    // Verify that in all QUBO optimal solutions, each terminal vertex is
    // assigned to its own terminal position.
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let terminals = vec![0, 2, 4];
    let source = MinimumMultiwayCut::new(graph, terminals.clone(), vec![2, 3, 1, 2, 4, 5]);

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    let k = terminals.len();
    for sol in &qubo_solutions {
        for (t_pos, &t_vertex) in terminals.iter().enumerate() {
            // Terminal vertex should be assigned to its own position
            assert_eq!(
                sol[t_vertex * k + t_pos],
                1,
                "Terminal {} at position {} should be 1",
                t_vertex,
                t_pos
            );
            // And not assigned to any other position
            for s in 0..k {
                if s != t_pos {
                    assert_eq!(
                        sol[t_vertex * k + s],
                        0,
                        "Terminal {} at position {} should be 0",
                        t_vertex,
                        s
                    );
                }
            }
        }
    }
}
