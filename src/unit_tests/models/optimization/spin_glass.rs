use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;
include!("../../jl_helpers.rs");

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
fn test_direction() {
    let problem = SpinGlass::<SimpleGraph, f64>::without_fields(2, vec![]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_num_variables() {
    let problem = SpinGlass::<SimpleGraph, f64>::without_fields(5, vec![]);
    assert_eq!(problem.num_variables(), 5);
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
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/spinglass.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let nv = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let edges = jl_parse_edges(&instance["instance"]);
        let j_values = jl_parse_i32_vec(&instance["instance"]["J"]);
        let h_values = jl_parse_i32_vec(&instance["instance"]["h"]);
        let interactions: Vec<((usize, usize), i32)> = edges.into_iter().zip(j_values).collect();
        let problem = SpinGlass::<SimpleGraph, i32>::new(nv, interactions, h_values);
        for eval in instance["evaluations"].as_array().unwrap() {
            let jl_config = jl_parse_config(&eval["config"]);
            let config = jl_flip_config(&jl_config);
            let result = problem.evaluate(&config);
            let jl_size = eval["size"].as_i64().unwrap() as i32;
            assert!(result.is_valid(), "SpinGlass should always be valid");
            assert_eq!(
                result.unwrap(),
                jl_size,
                "SpinGlass energy mismatch for config {:?}",
                config
            );
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_flip_configs_set(&jl_parse_configs_set(&instance["best_solutions"]));
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "SpinGlass best solutions mismatch");
    }
}
