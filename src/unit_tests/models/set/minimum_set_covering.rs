use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
include!("../../jl_helpers.rs");

#[test]
fn test_set_covering_creation() {
    let problem = MinimumSetCovering::<i32>::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    assert_eq!(problem.universe_size(), 4);
    assert_eq!(problem.num_sets(), 3);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_set_covering_with_weights() {
    let problem = MinimumSetCovering::with_weights(3, vec![vec![0, 1], vec![1, 2]], vec![5, 10]);
    assert_eq!(problem.weights_ref(), &vec![5, 10]);
}

#[test]
fn test_covered_elements() {
    let problem = MinimumSetCovering::<i32>::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    let covered = problem.covered_elements(&[1, 0, 0]);
    assert!(covered.contains(&0));
    assert!(covered.contains(&1));
    assert!(!covered.contains(&2));

    let covered = problem.covered_elements(&[1, 0, 1]);
    assert!(covered.contains(&0));
    assert!(covered.contains(&1));
    assert!(covered.contains(&2));
    assert!(covered.contains(&3));
}

#[test]
fn test_evaluate_valid() {
    let problem = MinimumSetCovering::<i32>::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    // Select first and third sets: covers {0,1} + {2,3} = {0,1,2,3}
    assert_eq!(
        Problem::evaluate(&problem, &[1, 0, 1]),
        SolutionSize::Valid(2)
    );

    // Select all sets
    assert_eq!(
        Problem::evaluate(&problem, &[1, 1, 1]),
        SolutionSize::Valid(3)
    );
}

#[test]
fn test_evaluate_invalid() {
    let problem = MinimumSetCovering::<i32>::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    // Select only first set: missing 2, 3 - returns Invalid
    assert_eq!(
        Problem::evaluate(&problem, &[1, 0, 0]),
        SolutionSize::Invalid
    );

    // Select none
    assert_eq!(
        Problem::evaluate(&problem, &[0, 0, 0]),
        SolutionSize::Invalid
    );
}

#[test]
fn test_brute_force_simple() {
    // Universe {0,1,2}, sets: {0,1}, {1,2}, {0,2}
    // Minimum cover: any 2 sets work
    let problem = MinimumSetCovering::<i32>::new(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 2);
        // Verify it's a valid cover
        assert!(Problem::evaluate(&problem, sol).is_valid());
    }
}

#[test]
fn test_brute_force_weighted() {
    // Prefer lighter sets
    let problem = MinimumSetCovering::with_weights(
        3,
        vec![vec![0, 1, 2], vec![0, 1], vec![2]],
        vec![10, 3, 3],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Should select sets 1 and 2 (total 6) instead of set 0 (total 10)
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1, 1]);
}

#[test]
fn test_is_set_cover_function() {
    let sets = vec![vec![0, 1], vec![1, 2], vec![2, 3]];

    assert!(is_set_cover(4, &sets, &[true, false, true]));
    assert!(is_set_cover(4, &sets, &[true, true, true]));
    assert!(!is_set_cover(4, &sets, &[true, false, false]));
    assert!(!is_set_cover(4, &sets, &[false, false, false]));
}

#[test]
fn test_get_set() {
    let problem = MinimumSetCovering::<i32>::new(4, vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(problem.get_set(0), Some(&vec![0, 1]));
    assert_eq!(problem.get_set(1), Some(&vec![2, 3]));
    assert_eq!(problem.get_set(2), None);
}

#[test]
fn test_direction() {
    let problem = MinimumSetCovering::<i32>::new(2, vec![vec![0, 1]]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_single_set_covers_all() {
    let problem = MinimumSetCovering::<i32>::new(3, vec![vec![0, 1, 2], vec![0], vec![1], vec![2]]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // First set alone covers everything
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 0, 0, 0]);
}

#[test]
fn test_overlapping_sets() {
    // All sets overlap on element 1
    let problem = MinimumSetCovering::<i32>::new(3, vec![vec![0, 1], vec![1, 2], vec![1]]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Minimum is selecting first two sets
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 2);
    }
}

#[test]
fn test_empty_universe() {
    let problem = MinimumSetCovering::<i32>::new(0, vec![]);
    // Empty universe is trivially covered with size 0
    assert_eq!(Problem::evaluate(&problem, &[]), SolutionSize::Valid(0));
}

#[test]
fn test_is_set_cover_wrong_len() {
    let sets = vec![vec![0, 1], vec![1, 2]];
    assert!(!is_set_cover(3, &sets, &[true])); // Wrong length
}

#[test]
fn test_set_covering_problem() {
    // Universe {0,1,2,3}, S0={0,1}, S1={2,3}
    let p = MinimumSetCovering::<i32>::new(4, vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(p.dims(), vec![2, 2]);

    // Select both -> covers all, weight=2
    assert_eq!(Problem::evaluate(&p, &[1, 1]), SolutionSize::Valid(2));
    // Select only S0 -> doesn't cover {2,3}, invalid
    assert_eq!(Problem::evaluate(&p, &[1, 0]), SolutionSize::Invalid);
    // Select none -> doesn't cover anything -> invalid
    assert_eq!(Problem::evaluate(&p, &[0, 0]), SolutionSize::Invalid);

    assert_eq!(p.direction(), Direction::Minimize);
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/setcovering.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let universe_size = instance["instance"]["universe_size"].as_u64().unwrap() as usize;
        let sets = jl_parse_sets(&instance["instance"]["sets"]);
        let weights = jl_parse_i32_vec(&instance["instance"]["weights"]);
        let problem = MinimumSetCovering::<i32>::with_weights(universe_size, sets, weights);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(result.is_valid(), jl_valid, "SetCovering validity mismatch for config {:?}", config);
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(result.unwrap(), jl_size, "SetCovering size mismatch for config {:?}", config);
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "SetCovering best solutions mismatch");
    }
}
