use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn canonical_cvp() -> ClosestVectorProblem<i64> {
    ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 2]], vec![3_i64, 2]).unwrap()
}

fn canonical_bits() -> Vec<bool> {
    vec![
        false, false, false, true, true, false, false, true, false, false, true,
    ]
}

#[test]
fn test_closestvectorproblem_to_qubo_closed_loop() {
    let source = canonical_cvp();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
    let target_solution = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    let source_solution = reduction.extract_solution(&target_solution).unwrap();

    assert_eq!(source_solution, vec![1, 1]);
    assert_eq!(source.evaluate(&source_solution).unwrap().0, Some(0.0));
    assert_eq!(reduction.target_problem().num_vars(), 11);
}

#[test]
fn test_closestvectorproblem_to_qubo_coefficients() {
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&canonical_cvp()).unwrap();
    let qubo = reduction.target_problem();

    assert_eq!(qubo.get(0, 0), Some(&-248.0));
    assert_eq!(qubo.get(0, 1), Some(&16.0));
    assert_eq!(qubo.get(0, 6), Some(&4.0));
    assert_eq!(qubo.get(6, 6), Some(&-241.0));
}

#[test]
fn test_closestvectorproblem_to_qubo_exact_range_decoding() {
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&canonical_cvp()).unwrap();
    assert_eq!(
        reduction.extract_solution(&canonical_bits()).unwrap(),
        vec![1, 1]
    );

    let duplicate = vec![
        true, false, false, true, false, true, true, true, true, true, false,
    ];
    assert_eq!(reduction.extract_solution(&duplicate).unwrap(), vec![1, 1]);
    assert_eq!(
        reduction
            .target_problem()
            .evaluate(&canonical_bits())
            .unwrap(),
        reduction.target_problem().evaluate(&duplicate).unwrap()
    );
}

#[test]
fn test_closestvectorproblem_to_qubo_preserves_optimum_outside_old_box() {
    let source = ClosestVectorProblem::new(vec![vec![1]], vec![20_i64]).unwrap();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
    let target_solution = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    assert_eq!(
        reduction.extract_solution(&target_solution).unwrap(),
        vec![20]
    );
}

#[test]
fn test_closestvectorproblem_to_qubo_reports_numeric_boundaries() {
    let absolute_value = ClosestVectorProblem::new(vec![vec![1]], vec![i64::MIN]).unwrap();
    assert!(matches!(
        ReduceTo::<QUBO<f64>>::reduce_to(&absolute_value),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));

    let inexact_float = ClosestVectorProblem::new(vec![vec![100_000_000]], vec![1_i64]).unwrap();
    assert!(matches!(
        ReduceTo::<QUBO<f64>>::reduce_to(&inexact_float),
        Err(crate::rules::ReductionError::InexactFloatConversion { .. })
    ));
}

#[cfg(feature = "example-db")]
#[test]
fn test_closestvectorproblem_to_qubo_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "closestvectorproblem_to_qubo")
        .unwrap();
    let example = (spec.build)();

    assert_eq!(example.source.problem, "ClosestVectorProblem");
    assert_eq!(example.target.problem, "QUBO");
    assert_eq!(example.target.instance["num_vars"], 11);
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!([1, 1])
    );
    assert_eq!(
        example.solutions[0].target_config,
        serde_json::to_value(canonical_bits()).unwrap()
    );
}
