use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_independentset_to_qubo_closed_loop() {
    // Path graph: 0-1-2-3 (4 vertices, 3 edges)
    // Maximum IS = {0, 2} or {1, 3} (size 2)
    let is = MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), vec![1i32; 4]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(is.evaluate(&extracted).is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 2);
    }
}

#[test]
fn test_independentset_to_qubo_triangle() {
    // Triangle: 0-1-2 (complete graph K3)
    // Maximum IS = any single vertex (size 1)
    let is = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), vec![1i32; 3]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(is.evaluate(&extracted).is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 1);
    }
}

#[test]
fn test_independentset_to_qubo_empty_graph() {
    // No edges: all vertices form the IS
    let is = MaximumIndependentSet::new(SimpleGraph::new(3, vec![]), vec![1i32; 3]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(is.evaluate(&extracted).is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 3);
    }
}

#[test]
fn test_independentset_to_qubo_structure() {
    let is = MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), vec![1i32; 4]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);
    let qubo = reduction.target_problem();

    // QUBO should have same number of variables as vertices
    assert_eq!(qubo.num_variables(), 4);
}
