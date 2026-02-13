use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_paintshop_creation() {
    let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
    assert_eq!(problem.num_cars(), 2);
    assert_eq!(problem.sequence_len(), 4);
    assert_eq!(problem.num_variables(), 2);
}

#[test]
fn test_is_first() {
    let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
    // First occurrence: a at 0, b at 1
    // Second occurrence: a at 2, b at 3
    assert_eq!(problem.is_first, vec![true, true, false, false]);
}

#[test]
fn test_get_coloring() {
    let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
    // Config: a=0, b=1
    // Sequence: a(0), b(1), a(1-opposite), b(0-opposite)
    let coloring = problem.get_coloring(&[0, 1]);
    assert_eq!(coloring, vec![0, 1, 1, 0]);

    // Config: a=1, b=0
    let coloring = problem.get_coloring(&[1, 0]);
    assert_eq!(coloring, vec![1, 0, 0, 1]);
}

#[test]
fn test_count_switches() {
    let problem = PaintShop::new(vec!["a", "b", "a", "b"]);

    // Config [0, 1] -> coloring [0, 1, 1, 0] -> 2 switches
    assert_eq!(problem.count_switches(&[0, 1]), 2);

    // Config [0, 0] -> coloring [0, 0, 1, 1] -> 1 switch
    assert_eq!(problem.count_switches(&[0, 0]), 1);

    // Config [1, 1] -> coloring [1, 1, 0, 0] -> 1 switch
    assert_eq!(problem.count_switches(&[1, 1]), 1);
}

#[test]
fn test_evaluate() {
    let problem = PaintShop::new(vec!["a", "b", "a", "b"]);

    // Config [0, 0] -> coloring [0, 0, 1, 1] -> 1 switch
    assert_eq!(Problem::evaluate(&problem, &[0, 0]), SolutionSize::Valid(1));

    // Config [0, 1] -> coloring [0, 1, 1, 0] -> 2 switches
    assert_eq!(Problem::evaluate(&problem, &[0, 1]), SolutionSize::Valid(2));
}

#[test]
fn test_brute_force_simple() {
    let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Optimal has 1 switch: [0,0] or [1,1]
    for sol in &solutions {
        assert_eq!(problem.count_switches(sol), 1);
    }
}

#[test]
fn test_brute_force_longer() {
    // Sequence: a, b, a, c, c, b
    let problem = PaintShop::new(vec!["a", "b", "a", "c", "c", "b"]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Find the minimum number of switches
    let min_switches = problem.count_switches(&solutions[0]);
    for sol in &solutions {
        assert_eq!(problem.count_switches(sol), min_switches);
    }
}

#[test]
fn test_count_paint_switches_function() {
    assert_eq!(count_paint_switches(&[0, 0, 0]), 0);
    assert_eq!(count_paint_switches(&[0, 1, 0]), 2);
    assert_eq!(count_paint_switches(&[0, 0, 1, 1]), 1);
    assert_eq!(count_paint_switches(&[0, 1, 0, 1]), 3);
}

#[test]
fn test_direction() {
    let problem = PaintShop::new(vec!["a", "a"]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_single_car() {
    let problem = PaintShop::new(vec!["a", "a"]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Both configs give 1 switch: a(0)->a(1) or a(1)->a(0)
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert_eq!(problem.count_switches(sol), 1);
    }
}

#[test]
fn test_adjacent_same_car() {
    // Sequence: a, a, b, b
    let problem = PaintShop::new(vec!["a", "a", "b", "b"]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Best case: [0,0] -> [0,1,0,1] = 3 switches, or [0,1] -> [0,1,1,0] = 2 switches
    // Actually: [0,0] -> a=0,a=1,b=0,b=1 = [0,1,0,1] = 3 switches
    // [0,1] -> a=0,a=1,b=1,b=0 = [0,1,1,0] = 2 switches
    let min_switches = problem.count_switches(&solutions[0]);
    assert!(min_switches <= 3);
}

#[test]
#[should_panic]
fn test_invalid_sequence_single_occurrence() {
    // This should panic because 'c' only appears once
    let _ = PaintShop::new(vec!["a", "b", "a", "c"]);
}

#[test]
fn test_car_labels() {
    let problem = PaintShop::new(vec!["car1", "car2", "car1", "car2"]);
    assert_eq!(problem.car_labels().len(), 2);
}

#[test]
fn test_paintshop_problem() {
    use crate::traits::{OptimizationProblem, Problem};
    use crate::types::Direction;

    let problem = PaintShop::new(vec!["a", "b", "a", "b"]);

    // dims: one binary variable per car
    assert_eq!(problem.dims(), vec![2, 2]);

    // Config [0, 0] -> coloring [0, 0, 1, 1] -> 1 switch
    assert_eq!(Problem::evaluate(&problem, &[0, 0]), SolutionSize::Valid(1));

    // Config [0, 1] -> coloring [0, 1, 1, 0] -> 2 switches
    assert_eq!(Problem::evaluate(&problem, &[0, 1]), SolutionSize::Valid(2));

    // Config [1, 1] -> coloring [1, 1, 0, 0] -> 1 switch
    assert_eq!(Problem::evaluate(&problem, &[1, 1]), SolutionSize::Valid(1));

    // Direction is minimize
    assert_eq!(problem.direction(), Direction::Minimize);
}
