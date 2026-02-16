use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

fn k4_tsp() -> TravelingSalesman<SimpleGraph, i32> {
    TravelingSalesman::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![10, 15, 20, 35, 25, 30],
    )
}

#[test]
fn test_traveling_salesman_creation() {
    // K4 complete graph
    let problem = k4_tsp();
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 6);
    assert_eq!(problem.dims().len(), 6);
}

#[test]
fn test_traveling_salesman_unit_weights() {
    // i32 type is always considered weighted, even with uniform values
    let problem = TravelingSalesman::<_, i32>::unit_weights(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
    );
    assert!(problem.is_weighted());
    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.graph().num_edges(), 5);
}

#[test]
fn test_traveling_salesman_weighted() {
    let problem = k4_tsp();
    assert!(problem.is_weighted());
}

#[test]
fn test_evaluate_valid_cycle() {
    // C5 cycle graph with unit weights: all 5 edges form the only Hamiltonian cycle
    let problem = TravelingSalesman::<_, i32>::unit_weights(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
    );
    // Select all edges -> valid Hamiltonian cycle, cost = 5
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 1]), SolutionSize::Valid(5));
}

#[test]
fn test_evaluate_invalid_degree() {
    // K4: select 3 edges incident to vertex 0 -> degree > 2 at vertex 0
    let problem = k4_tsp();
    // edges: 0-1, 0-2, 0-3, 1-2, 1-3, 2-3
    // Select first 3 edges (all incident to 0): degree(0)=3 -> Invalid
    assert_eq!(problem.evaluate(&[1, 1, 1, 0, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_evaluate_invalid_not_connected() {
    // 6 vertices, two disjoint triangles: 0-1-2-0 and 3-4-5-3
    let problem = TravelingSalesman::<_, i32>::unit_weights(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)]),
    );
    // Select all 6 edges: two disjoint cycles, not a single Hamiltonian cycle
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 1, 1]), SolutionSize::Invalid);
}

#[test]
fn test_evaluate_invalid_wrong_edge_count() {
    // C5 with only 4 edges selected -> not enough edges
    let problem = TravelingSalesman::<_, i32>::unit_weights(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
    );
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 0]), SolutionSize::Invalid);
}

#[test]
fn test_evaluate_no_edges_selected() {
    let problem = TravelingSalesman::<_, i32>::unit_weights(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
    );
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_brute_force_k4() {
    // Instance 1 from issue: K4 with weights
    let problem = k4_tsp();
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(!solutions.is_empty());
    // Optimal cycle: 0->1->3->2->0, cost = 10+25+30+15 = 80
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(80));
    }
}

#[test]
fn test_brute_force_path_graph_no_solution() {
    // Instance 2 from issue: path graph, no Hamiltonian cycle exists
    let problem =
        TravelingSalesman::<_, i32>::unit_weights(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_brute_force_c5_unique_solution() {
    // Instance 3 from issue: C5 cycle graph, unique Hamiltonian cycle
    let problem = TravelingSalesman::<_, i32>::unit_weights(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1, 1, 1]);
    assert_eq!(problem.evaluate(&solutions[0]), SolutionSize::Valid(5));
}

#[test]
fn test_brute_force_bipartite_no_solution() {
    // Instance 4 from issue: K_{2,3} bipartite, no Hamiltonian cycle
    let problem = TravelingSalesman::<_, i32>::unit_weights(
        SimpleGraph::new(5, vec![(0, 2), (0, 3), (0, 4), (1, 2), (1, 3), (1, 4)]),
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_direction() {
    let problem =
        TravelingSalesman::<_, i32>::unit_weights(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_problem_name() {
    assert_eq!(
        <TravelingSalesman<SimpleGraph, i32> as Problem>::NAME,
        "TravelingSalesman"
    );
}

#[test]
fn test_is_hamiltonian_cycle_function() {
    // Triangle: selecting all 3 edges is a valid Hamiltonian cycle
    assert!(is_hamiltonian_cycle(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        &[true, true, true]
    ));
    // Path: not a cycle
    assert!(!is_hamiltonian_cycle(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        &[true, true]
    ));
}

#[test]
fn test_set_weights() {
    let mut problem =
        TravelingSalesman::<_, i32>::unit_weights(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    problem.set_weights(vec![5, 10, 15]);
    assert_eq!(problem.weights(), vec![5, 10, 15]);
}

#[test]
fn test_edges() {
    let problem =
        TravelingSalesman::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), vec![10, 20, 30]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 3);
}

#[test]
fn test_new() {
    let problem = TravelingSalesman::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), vec![10, 20, 30]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.weights(), vec![10, 20, 30]);
}

#[test]
fn test_unit_weights() {
    let problem = TravelingSalesman::<_, i32>::unit_weights(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    assert_eq!(problem.weights(), vec![1, 1, 1]);
}

#[test]
fn test_brute_force_triangle_weighted() {
    // Triangle with weights: unique Hamiltonian cycle using all edges
    let problem =
        TravelingSalesman::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), vec![5, 10, 15]);
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1]);
    assert_eq!(problem.evaluate(&solutions[0]), SolutionSize::Valid(30));
}
