use super::*;
use crate::models::algebraic::VarBounds;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;

fn canonical_cvp() -> ClosestVectorProblem<i32> {
    ClosestVectorProblem::new(
        vec![vec![2, 0], vec![1, 2]],
        vec![2.8, 1.5],
        vec![VarBounds::bounded(-2, 4), VarBounds::bounded(-2, 4)],
    )
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn test_closestvectorproblem_to_qubo_closed_loop() {
    let source = canonical_cvp();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);

    assert_eq!(reduction.target_problem().num_vars(), 6);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "ClosestVectorProblem->QUBO closed loop",
    );
}

#[test]
fn test_closestvectorproblem_to_qubo_example_matrix_coefficients() {
    let source = canonical_cvp();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 6);
    assert_close(*qubo.get(0, 0).expect("Q[0,0]"), -31.2);
    assert_close(*qubo.get(0, 1).expect("Q[0,1]"), 16.0);
    assert_close(*qubo.get(0, 2).expect("Q[0,2]"), 24.0);
    assert_close(*qubo.get(1, 2).expect("Q[1,2]"), 48.0);
    assert_close(*qubo.get(2, 5).expect("Q[2,5]"), 36.0);
    assert_close(*qubo.get(5, 5).expect("Q[5,5]"), -73.8);
}

#[test]
fn test_extract_solution_ignores_duplicate_exact_range_encodings() {
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&canonical_cvp());

    assert_eq!(reduction.extract_solution(&[1, 1, 0, 1, 1, 0]), vec![3, 3]);
    assert_eq!(reduction.extract_solution(&[0, 0, 1, 0, 0, 1]), vec![3, 3]);
}

#[cfg(feature = "example-db")]
#[test]
fn test_closestvectorproblem_to_qubo_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "closestvectorproblem_to_qubo")
        .expect("missing canonical ClosestVectorProblem -> QUBO example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "ClosestVectorProblem");
    assert_eq!(example.target.problem, "QUBO");
    assert_eq!(example.target.instance["num_vars"], 6);
    assert_eq!(example.solutions[0].source_config, vec![3, 3]);
    assert_eq!(example.solutions[0].target_config, vec![0, 0, 1, 0, 0, 1]);
}

#[test]
fn test_duplicate_target_encodings_have_equal_qubo_value() {
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&canonical_cvp());
    let qubo = reduction.target_problem();
    let solver = BruteForce::new();
    let best = solver.find_all_best(qubo);

    assert!(best.contains(&vec![0, 0, 1, 0, 0, 1]) || best.contains(&vec![1, 1, 0, 1, 1, 0]));
    assert_close(
        qubo.evaluate(&[0, 0, 1, 0, 0, 1]),
        qubo.evaluate(&[1, 1, 0, 1, 1, 0]),
    );
}
