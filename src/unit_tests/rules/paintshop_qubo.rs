use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;

#[test]
fn test_paintshop_to_qubo_closed_loop() {
    // Issue example: Sequence [A, B, C, A, D, B, D, C], 4 cars
    let source = PaintShop::new(vec!["A", "B", "C", "A", "D", "B", "D", "C"]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();

    // 4 cars -> 4 QUBO variables
    assert_eq!(qubo.num_vars(), 4);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "PaintShop->QUBO closed loop",
    );
}

#[test]
fn test_paintshop_to_qubo_simple() {
    // Simple case: a, b, a, b
    let source = PaintShop::new(vec!["a", "b", "a", "b"]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 2);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "PaintShop->QUBO simple",
    );
}

#[test]
fn test_paintshop_to_qubo_optimal_value() {
    // Issue example verifies optimal QUBO = -1, total switches = -1 + 3 = 2
    let source = PaintShop::new(vec!["A", "B", "C", "A", "D", "B", "D", "C"]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_witnesses(qubo);

    // Extract solutions and verify they are optimal for the source
    for sol in &best_target {
        let source_sol = reduction.extract_solution(sol);
        let switches = source.count_switches(&source_sol);
        // Optimal is 2 switches
        assert_eq!(switches, 2, "Expected 2 switches for optimal solution");
    }
}

#[test]
fn test_paintshop_to_qubo_matrix_structure() {
    // Issue example: verify the Q matrix matches expected values
    let source = PaintShop::new(vec!["A", "B", "C", "A", "D", "B", "D", "C"]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();

    let m = qubo.matrix();
    // From the issue:
    // Q = [ -1,  -2,   2,   2 ]
    //     [  0,   2,  -2,   0 ]
    //     [  0,   0,   1,  -2 ]
    //     [  0,   0,   0,   0 ]
    assert_eq!(m[0][0], -1.0);
    assert_eq!(m[0][1], -2.0);
    assert_eq!(m[0][2], 2.0);
    assert_eq!(m[0][3], 2.0);
    assert_eq!(m[1][1], 2.0);
    assert_eq!(m[1][2], -2.0);
    assert_eq!(m[1][3], 0.0);
    assert_eq!(m[2][2], 1.0);
    assert_eq!(m[2][3], -2.0);
    assert_eq!(m[3][3], 0.0);
}

#[test]
fn test_paintshop_to_qubo_two_cars() {
    // Two cars, adjacent: a, b, b, a
    let source = PaintShop::new(vec!["a", "b", "b", "a"]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "PaintShop->QUBO two cars",
    );
}

#[test]
fn test_paintshop_to_qubo_empty_sequence() {
    // Empty PaintShop with 0 cars should not panic
    let source = PaintShop::new(Vec::<&str>::new());
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();
    assert_eq!(qubo.num_vars(), 0);
}

#[cfg(feature = "example-db")]
#[test]
fn test_paintshop_to_qubo_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "paintshop_to_qubo")
        .expect("missing canonical PaintShop -> QUBO example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "PaintShop");
    assert_eq!(example.target.problem, "QUBO");
    assert_eq!(example.source.instance["num_cars"], 4);
    assert_eq!(example.target.instance["num_vars"], 4);
    assert!(!example.solutions.is_empty());
}
