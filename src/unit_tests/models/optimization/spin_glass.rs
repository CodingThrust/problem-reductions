use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_spin_glass_creation() {
    let problem = SpinGlass::<SimpleGraph, f64>::new(
        3,
        vec![((0, 1), 1.0), ((1, 2), -1.0)],
        vec![0.0, 0.0, 0.0],
    );
    assert_eq!(problem.num_spins(), 3);
    assert_eq!(problem.interactions().len(), 2);
    assert_eq!(problem.fields().len(), 3);
}

#[test]
fn test_spin_glass_without_fields() {
    let problem = SpinGlass::<SimpleGraph, f64>::without_fields(3, vec![((0, 1), 1.0)]);
    assert_eq!(problem.fields(), &[0.0, 0.0, 0.0]);
}

#[test]
fn test_config_to_spins() {
    assert_eq!(
        SpinGlass::<SimpleGraph, f64>::config_to_spins(&[0, 0]),
        vec![-1, -1]
    );
    assert_eq!(
        SpinGlass::<SimpleGraph, f64>::config_to_spins(&[1, 1]),
        vec![1, 1]
    );
    assert_eq!(
        SpinGlass::<SimpleGraph, f64>::config_to_spins(&[0, 1]),
        vec![-1, 1]
    );
    assert_eq!(
        SpinGlass::<SimpleGraph, f64>::config_to_spins(&[1, 0]),
        vec![1, -1]
    );
}

#[test]
fn test_compute_energy() {
    // Two spins with J = 1 (ferromagnetic prefers aligned)
    let problem = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);

    // Aligned spins: energy = J * s1 * s2 = 1 * 1 * 1 = 1 or 1 * (-1) * (-1) = 1
    assert_eq!(problem.compute_energy(&[1, 1]), 1.0);
    assert_eq!(problem.compute_energy(&[-1, -1]), 1.0);

    // Anti-aligned spins: energy = J * s1 * s2 = 1 * 1 * (-1) = -1
    assert_eq!(problem.compute_energy(&[1, -1]), -1.0);
    assert_eq!(problem.compute_energy(&[-1, 1]), -1.0);
}

#[test]
fn test_compute_energy_with_fields() {
    let problem = SpinGlass::<SimpleGraph, f64>::new(2, vec![], vec![1.0, -1.0]);

    // Energy = h1*s1 + h2*s2 = 1*s1 + (-1)*s2
    assert_eq!(problem.compute_energy(&[1, 1]), 0.0); // 1 - 1 = 0
    assert_eq!(problem.compute_energy(&[-1, -1]), 0.0); // -1 + 1 = 0
    assert_eq!(problem.compute_energy(&[1, -1]), 2.0); // 1 + 1 = 2
    assert_eq!(problem.compute_energy(&[-1, 1]), -2.0); // -1 - 1 = -2
}

#[test]
fn test_solution_size() {
    let problem = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);

    // config [0,0] -> spins [-1,-1] -> energy = 1
    let sol = problem.solution_size(&[0, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1.0);

    // config [0,1] -> spins [-1,1] -> energy = -1
    let sol = problem.solution_size(&[0, 1]);
    assert_eq!(sol.size, -1.0);
}

#[test]
fn test_brute_force_ferromagnetic() {
    // Ferromagnetic: J > 0 prefers aligned spins to minimize energy
    // But wait, energy = J*s1*s2, so J>0 with aligned gives positive energy
    // For minimization, we want anti-aligned for J>0
    let problem = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Minimum energy is -1 (anti-aligned)
    for sol in &solutions {
        assert_ne!(sol[0], sol[1]);
        assert_eq!(problem.solution_size(sol).size, -1.0);
    }
}

#[test]
fn test_brute_force_antiferromagnetic() {
    // Antiferromagnetic: J < 0, energy = J*s1*s2
    // J<0 with aligned spins gives negative energy (good for minimization)
    let problem = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), -1.0)], vec![0.0, 0.0]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Minimum energy is -1 (aligned)
    for sol in &solutions {
        assert_eq!(sol[0], sol[1]);
        assert_eq!(problem.solution_size(sol).size, -1.0);
    }
}

#[test]
fn test_energy_mode() {
    let problem = SpinGlass::<SimpleGraph, f64>::without_fields(2, vec![]);
    assert!(problem.energy_mode().is_minimization());
}

#[test]
fn test_num_variables_flavors() {
    let problem = SpinGlass::<SimpleGraph, f64>::without_fields(5, vec![]);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(problem.num_flavors(), 2);
}

#[test]
fn test_problem_size() {
    let problem = SpinGlass::<SimpleGraph, f64>::new(
        3,
        vec![((0, 1), 1.0), ((1, 2), 1.0)],
        vec![0.0, 0.0, 0.0],
    );
    let size = problem.problem_size();
    assert_eq!(size.get("num_spins"), Some(3));
    assert_eq!(size.get("num_interactions"), Some(2));
}

#[test]
fn test_triangle_frustration() {
    // Triangle with all antiferromagnetic couplings - frustrated system
    let problem = SpinGlass::<SimpleGraph, f64>::new(
        3,
        vec![((0, 1), 1.0), ((1, 2), 1.0), ((0, 2), 1.0)],
        vec![0.0, 0.0, 0.0],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Best we can do is satisfy 2 out of 3 interactions
    // Energy = -1 -1 + 1 = -1 (one frustrated)
    for sol in &solutions {
        assert_eq!(problem.solution_size(sol).size, -1.0);
    }
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem =
        SpinGlass::<SimpleGraph, f64>::from_graph(graph, vec![1.0, 2.0], vec![0.0, 0.0, 0.0]);
    assert_eq!(problem.num_spins(), 3);
    assert_eq!(problem.couplings(), &[1.0, 2.0]);
    assert_eq!(problem.fields(), &[0.0, 0.0, 0.0]);
}

#[test]
fn test_from_graph_without_fields() {
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = SpinGlass::<SimpleGraph, f64>::from_graph_without_fields(graph, vec![1.5]);
    assert_eq!(problem.num_spins(), 2);
    assert_eq!(problem.couplings(), &[1.5]);
    assert_eq!(problem.fields(), &[0.0, 0.0]);
}

#[test]
fn test_graph_accessor() {
    let problem = SpinGlass::<SimpleGraph, f64>::new(3, vec![((0, 1), 1.0)], vec![0.0, 0.0, 0.0]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 1);
}

#[test]
fn test_variant() {
    let variant = SpinGlass::<SimpleGraph, f64>::variant();
    assert_eq!(variant.len(), 2);
    assert_eq!(variant[0], ("graph", "SimpleGraph"));
    assert_eq!(variant[1], ("weight", "f64"));
}

#[test]
fn test_spin_glass_problem_v2() {
    use crate::traits::{OptimizationProblemV2, ProblemV2};
    use crate::types::Direction;

    // Two spins with antiferromagnetic coupling J_01 = 1
    let p = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);
    assert_eq!(p.dims(), vec![2, 2]);

    // config [0, 0] => spins [-1, -1]: H = 1 * (-1)*(-1) = 1
    assert_eq!(ProblemV2::evaluate(&p, &[0, 0]), 1.0);
    // config [1, 1] => spins [+1, +1]: H = 1 * 1*1 = 1
    assert_eq!(ProblemV2::evaluate(&p, &[1, 1]), 1.0);
    // config [0, 1] => spins [-1, +1]: H = 1 * (-1)*(1) = -1
    assert_eq!(ProblemV2::evaluate(&p, &[0, 1]), -1.0);
    // config [1, 0] => spins [+1, -1]: H = 1 * (1)*(-1) = -1
    assert_eq!(ProblemV2::evaluate(&p, &[1, 0]), -1.0);

    assert_eq!(p.direction(), Direction::Minimize);
}
