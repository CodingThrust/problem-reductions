use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
include!("../../jl_helpers.rs");

#[test]
fn test_set_packing_creation() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
    assert_eq!(problem.num_sets(), 3);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_set_packing_with_weights() {
    let problem = MaximumSetPacking::with_weights(vec![vec![0, 1], vec![2, 3]], vec![5, 10]);
    assert_eq!(problem.weights_ref(), &vec![5, 10]);
}

#[test]
fn test_sets_overlap() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);

    assert!(problem.sets_overlap(0, 1)); // Share element 1
    assert!(!problem.sets_overlap(0, 2)); // No overlap
    assert!(!problem.sets_overlap(1, 2)); // No overlap
}

#[test]
fn test_overlapping_pairs() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    let pairs = problem.overlapping_pairs();
    assert_eq!(pairs.len(), 2);
    assert!(pairs.contains(&(0, 1)));
    assert!(pairs.contains(&(1, 2)));
}

#[test]
fn test_is_set_packing_function() {
    let sets = vec![vec![0, 1], vec![1, 2], vec![3, 4]];

    assert!(is_set_packing(&sets, &[true, false, true])); // Disjoint
    assert!(is_set_packing(&sets, &[false, true, true])); // Disjoint
    assert!(!is_set_packing(&sets, &[true, true, false])); // Overlap on 1
    assert!(is_set_packing(&sets, &[false, false, false])); // Empty is valid
}

#[test]
fn test_direction() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1]]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_empty_sets() {
    let problem = MaximumSetPacking::<i32>::new(vec![]);
    // Empty packing is valid with size 0
    assert_eq!(Problem::evaluate(&problem, &[]), SolutionSize::Valid(0));
}

#[test]
fn test_get_set() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(problem.get_set(0), Some(&vec![0, 1]));
    assert_eq!(problem.get_set(1), Some(&vec![2, 3]));
    assert_eq!(problem.get_set(2), None);
}

#[test]
fn test_relationship_to_independent_set() {
    // MaximumSetPacking on sets is equivalent to MaximumIndependentSet on the intersection graph
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;

    let sets = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
    let sp_problem = MaximumSetPacking::<i32>::new(sets.clone());

    // Build intersection graph
    let edges = sp_problem.overlapping_pairs();
    let n = sets.len();
    let is_problem = MaximumIndependentSet::new(SimpleGraph::new(n, edges), vec![1i32; n]);

    let solver = BruteForce::new();

    let sp_solutions = solver.find_all_best(&sp_problem);
    let is_solutions = solver.find_all_best(&is_problem);

    // Should have same optimal value
    let sp_size: usize = sp_solutions[0].iter().sum();
    let is_size: usize = is_solutions[0].iter().sum();
    assert_eq!(sp_size, is_size);
}

#[test]
fn test_is_set_packing_wrong_len() {
    let sets = vec![vec![0, 1], vec![1, 2]];
    assert!(!is_set_packing(&sets, &[true])); // Wrong length
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/setpacking.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let sets = jl_parse_sets(&instance["instance"]["sets"]);
        let weights = jl_parse_i32_vec(&instance["instance"]["weights"]);
        let problem = if weights.iter().all(|&w| w == 1) {
            MaximumSetPacking::<i32>::new(sets)
        } else {
            MaximumSetPacking::with_weights(sets, weights)
        };
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(
                result.is_valid(),
                jl_valid,
                "SetPacking validity mismatch for config {:?}",
                config
            );
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(
                    result.unwrap(),
                    jl_size,
                    "SetPacking size mismatch for config {:?}",
                    config
                );
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "SetPacking best solutions mismatch");
    }
}
