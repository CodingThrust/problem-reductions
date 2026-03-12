use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_travelingsalesman_to_qubo_closed_loop() {
    // K3 complete graph with weights [1, 2, 3]
    let graph = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let tsp = TravelingSalesman::new(graph, vec![1i32, 2, 3]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&tsp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    // All QUBO solutions should extract to valid TSP solutions
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let metric = tsp.evaluate(&extracted);
        assert!(metric.is_valid(), "Extracted solution should be valid");
        // K3 has only one Hamiltonian cycle (all 3 edges), cost = 1+2+3 = 6
        assert_eq!(metric, SolutionSize::Valid(6));
    }

    // There are multiple QUBO optima (different position assignments for the same tour),
    // but they should all extract to valid tours with cost 6.
    assert!(
        !qubo_solutions.is_empty(),
        "Should find at least one QUBO solution"
    );
}

#[test]
fn test_travelingsalesman_to_qubo_k4() {
    // K4 with unit weights
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let tsp = TravelingSalesman::new(graph, vec![1i32; 6]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&tsp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    // Every Hamiltonian cycle in K4 uses exactly 4 edges, so cost = 4
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let metric = tsp.evaluate(&extracted);
        assert!(metric.is_valid(), "Extracted solution should be valid");
        assert_eq!(metric, SolutionSize::Valid(4));
    }

    // K4 has 3 distinct Hamiltonian cycles, but each has multiple position encodings
    // (4 rotations x 2 directions = 8 QUBO solutions per cycle, total 24).
    // Just verify we get a non-trivial number of solutions.
    assert!(
        qubo_solutions.len() >= 3,
        "Should find at least 3 QUBO solutions for K4"
    );
}

#[test]
fn test_travelingsalesman_to_qubo_sizes() {
    // K3: n=3, QUBO should have n^2 = 9 variables
    let graph3 = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let tsp3 = TravelingSalesman::new(graph3, vec![1i32; 3]);
    let reduction3 = ReduceTo::<QUBO<f64>>::reduce_to(&tsp3);
    assert_eq!(reduction3.target_problem().num_variables(), 9);

    // K4: n=4, QUBO should have n^2 = 16 variables
    let graph4 = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let tsp4 = TravelingSalesman::new(graph4, vec![1i32; 6]);
    let reduction4 = ReduceTo::<QUBO<f64>>::reduce_to(&tsp4);
    assert_eq!(reduction4.target_problem().num_variables(), 16);
}
