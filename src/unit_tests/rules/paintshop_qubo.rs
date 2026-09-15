use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;

#[test]
fn test_paintshop_to_qubo_closed_loop() {
    // Issue example: Sequence [A, B, C, A, D, B, D, C], 4 cars
    let source = PaintShop::new(vec!["A", "B", "C", "A", "D", "B", "D", "C"]);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).expect("reduction should succeed");
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
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).expect("reduction should succeed");
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
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_witnesses(qubo).unwrap();

    // Extract solutions and verify they are optimal for the source
    for sol in &best_target {
        let source_sol = reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), (sol).clone()).unwrap(),
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution");
        let switches = source.count_switches(&source_sol).unwrap();
        // Optimal is 2 switches
        assert_eq!(switches, 2, "Expected 2 switches for optimal solution");
    }
}

#[test]
fn test_paintshop_to_qubo_matrix_structure() {
    // Verify the Q matrix matches expected values
    let source = PaintShop::new(vec!["A", "B", "C", "A", "D", "B", "D", "C"]);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    // Expected coefficients:
    // Q = [ -1,  -2,   2,   2 ]
    //     [  0,   2,  -2,   0 ]
    //     [  0,   0,   1,  -2 ]
    //     [  0,   0,   0,   0 ]
    assert_eq!(qubo.get(0, 0).unwrap(), -1);
    assert_eq!(qubo.get(0, 1).unwrap(), -2);
    assert_eq!(qubo.get(0, 2).unwrap(), 2);
    assert_eq!(qubo.get(0, 3).unwrap(), 2);
    assert_eq!(qubo.get(1, 1).unwrap(), 2);
    assert_eq!(qubo.get(1, 2).unwrap(), -2);
    assert_eq!(qubo.get(1, 3).unwrap(), 0);
    assert_eq!(qubo.get(2, 2).unwrap(), 1);
    assert_eq!(qubo.get(2, 3).unwrap(), -2);
    assert_eq!(qubo.get(3, 3).unwrap(), 0);
}

#[test]
fn test_paintshop_to_qubo_two_cars() {
    // Two cars, adjacent: a, b, b, a
    let source = PaintShop::new(vec!["a", "b", "b", "a"]);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).expect("reduction should succeed");

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
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).expect("reduction should succeed");
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
    assert_eq!(example.target.instance["matrix"]["nrows"], 4);
    assert!(!example.solutions.is_empty());
}
