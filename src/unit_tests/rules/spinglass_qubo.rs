use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_qubo_to_spinglass() {
    // Simple 2-variable QUBO: minimize x0 + x1 - 2*x0*x1
    // Optimal at x = [0, 0] (value 0) or x = [1, 1] (value 0)
    let qubo = QUBO::from_matrix(vec![vec![1.0, -2.0], vec![0.0, 1.0]]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&qubo);
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let sg_solutions = solver.find_best(sg);
    let qubo_solutions: Vec<_> = sg_solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Verify solutions are valid
    assert!(!qubo_solutions.is_empty());

    // Original QUBO at [0,0]: 0, at [1,1]: 1 + 1 - 2 = 0, at [0,1]: 1, at [1,0]: 1
    // So [0,0] and [1,1] are optimal with value 0
    for sol in &qubo_solutions {
        let val = qubo.solution_size(sol).size;
        assert!(
            val <= 0.0 + 1e-6,
            "Expected optimal value near 0, got {}",
            val
        );
    }
}

#[test]
fn test_spinglass_to_qubo() {
    // Simple SpinGlass: J_01 = -1 (ferromagnetic: prefers aligned spins)
    // Energy: J_01 * s0 * s1 = -s0 * s1
    // Aligned spins give -1, anti-aligned give +1
    // Minimum is -1 at [0,0] or [1,1] (both give s=-1,-1 or s=+1,+1)
    let sg = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), -1.0)], vec![0.0, 0.0]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    // Ferromagnetic: aligned spins are optimal
    for sol in &qubo_solutions {
        assert_eq!(sol[0], sol[1], "Ferromagnetic should have aligned spins");
    }
}

#[test]
fn test_roundtrip_qubo_sg_qubo() {
    let original = QUBO::from_matrix(vec![vec![-1.0, 2.0], vec![0.0, -1.0]]);
    let solver = BruteForce::new();
    let original_solutions = solver.find_best(&original);
    let _original_val = original.solution_size(&original_solutions[0]).size;

    // QUBO -> SG -> QUBO
    let reduction1 = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&original);
    let sg = reduction1.target_problem().clone();
    let reduction2 = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let roundtrip = reduction2.target_problem();

    let roundtrip_solutions = solver.find_best(roundtrip);
    let _roundtrip_val = roundtrip.solution_size(&roundtrip_solutions[0]).size;

    // The solutions should have the same configuration
    // (optimal configs should match)
    let orig_configs: std::collections::HashSet<_> = original_solutions.iter().collect();
    let rt_configs: std::collections::HashSet<_> = roundtrip_solutions.iter().collect();
    assert!(
        orig_configs.intersection(&rt_configs).count() > 0,
        "At least one optimal solution should match"
    );
}

#[test]
fn test_antiferromagnetic() {
    // Antiferromagnetic: J > 0, prefers anti-aligned spins
    let sg = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    // Anti-ferromagnetic: opposite spins are optimal
    for sol in &solutions {
        assert_ne!(
            sol[0], sol[1],
            "Antiferromagnetic should have opposite spins"
        );
    }
}

#[test]
fn test_with_onsite_fields() {
    // SpinGlass with only on-site field h_0 = 1
    // Energy = h_0 * s_0 = s_0
    // Minimum at s_0 = -1, i.e., x_0 = 0
    let sg = SpinGlass::<SimpleGraph, f64>::new(1, vec![], vec![1.0]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0], "Should prefer x=0 (s=-1)");
}

#[test]
fn test_reduction_sizes() {
    // Test source_size and target_size methods
    let qubo = QUBO::from_matrix(vec![vec![1.0, -2.0], vec![0.0, 1.0]]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&qubo);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();

    assert!(!source_size.components.is_empty());
    assert!(!target_size.components.is_empty());

    // Test SG to QUBO sizes
    let sg = SpinGlass::<SimpleGraph, f64>::new(3, vec![((0, 1), -1.0)], vec![0.0, 0.0, 0.0]);
    let reduction2 = ReduceTo::<QUBO<f64>>::reduce_to(&sg);

    let source_size2 = reduction2.source_size();
    let target_size2 = reduction2.target_size();

    assert!(!source_size2.components.is_empty());
    assert!(!target_size2.components.is_empty());
}
