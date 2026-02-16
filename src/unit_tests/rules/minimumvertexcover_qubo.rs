use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_vertexcovering_to_qubo_closed_loop() {
    // Cycle C4: 0-1-2-3-0 (4 vertices, 4 edges)
    // Minimum VC = 2 vertices (e.g., {0, 2} or {1, 3})
    let vc = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
        vec![1i32; 4],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&vc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(vc.evaluate(&extracted).is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 2);
    }
}

#[test]
fn test_vertexcovering_to_qubo_triangle() {
    // Triangle K3: minimum VC = 2 (any two vertices)
    let vc = MinimumVertexCover::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&vc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(vc.evaluate(&extracted).is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 2);
    }
}

#[test]
fn test_vertexcovering_to_qubo_star() {
    // Star graph: center vertex 0 connected to 1, 2, 3
    // Minimum VC = {0} (just the center)
    let vc = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![1i32; 4],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&vc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(vc.evaluate(&extracted).is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 1);
    }
}

#[test]
fn test_vertexcovering_to_qubo_structure() {
    let vc = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
        vec![1i32; 4],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&vc);
    let qubo = reduction.target_problem();

    // QUBO should have same number of variables as vertices
    assert_eq!(qubo.num_variables(), 4);
}
