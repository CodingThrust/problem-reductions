use super::*;
use crate::solvers::SolveOutcome;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_cvp_constructs_integer_and_real_targets() {
    let integer =
        ClosestVectorProblem::<i64>::new(vec![vec![2, 0, 0], vec![1, 2, 0]], vec![3_i64, 3, 1])
            .unwrap();
    assert_eq!(integer.num_basis_vectors(), 2);
    assert_eq!(integer.ambient_dimension(), 3);
    assert_eq!(integer.target(), &[3, 3, 1]);
    assert_eq!(
        ClosestVectorProblem::<i64>::variant(),
        vec![("coefficient", "i64")]
    );

    let real = ClosestVectorProblem::<f64>::new(
        vec![vec![2.0, 0.0, 0.0], vec![1.0, 2.0, 0.0]],
        vec![2.5, 1.25, -0.5],
    )
    .unwrap();
    assert_eq!(real.target(), &[2.5, 1.25, -0.5]);
    assert_eq!(
        ClosestVectorProblem::<f64>::variant(),
        vec![("coefficient", "f64")]
    );
}

#[test]
fn test_cvp_evaluates_without_coefficient_bounds() {
    let problem =
        ClosestVectorProblem::<i64>::new(vec![vec![2, 0, 0], vec![1, 2, 0]], vec![3_i64, 3, 1])
            .unwrap();
    assert_eq!(problem.evaluate(&vec![1, 1]).unwrap(), Min(Some(2)));
    assert!(problem.evaluate(&vec![11, -12]).unwrap().0.is_some());
    assert!(matches!(
        problem.evaluate(&vec![1]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_cvp_rejects_invalid_basis() {
    assert!(ClosestVectorProblem::<i64>::new(vec![vec![1_i64]], vec![0_i64, 0]).is_err());
    assert!(
        ClosestVectorProblem::<i64>::new(vec![vec![1_i64, 0], vec![2_i64, 0]], vec![0_i64, 0],)
            .is_err()
    );
    assert!(
        ClosestVectorProblem::<i64>::new(vec![vec![1_i64], vec![2_i64]], vec![0_i64],).is_err()
    );
}

#[test]
fn test_cvp_rank_uses_exact_integer_elimination() {
    let problem = ClosestVectorProblem::<i64>::new(
        vec![vec![i64::MAX, 1], vec![1, i64::MAX]],
        vec![0_i64, 0],
    )
    .unwrap();
    assert_eq!(problem.independent_rows(), vec![0, 1]);
    // Swapped pivots and a redundant ambient row preserve column rank.
    let rectangular =
        ClosestVectorProblem::<i64>::new(vec![vec![0, 0, 1], vec![0, 1, 0]], vec![0_i64; 3])
            .unwrap();
    assert_eq!(rectangular.independent_rows(), vec![2, 1]);
}

#[test]
fn test_cvp_rejects_non_finite_real_target() {
    assert!(matches!(
        ClosestVectorProblem::<f64>::new(vec![vec![1.0]], vec![f64::NAN]),
        Err(ConstructionError::NonFiniteFloat(_))
    ));
    assert!(matches!(
        ClosestVectorProblem::<f64>::new(vec![vec![1.0]], vec![f64::INFINITY]),
        Err(ConstructionError::NonFiniteFloat(_))
    ));
}

#[test]
fn test_cvp_integer_coordinates_preserve_zero_and_unit_distance() {
    let target = (1_i64 << 53) + 1;
    let problem = ClosestVectorProblem::<i64>::new(vec![vec![1]], vec![target]).unwrap();
    assert_eq!(
        crate::solvers::customized::closest_vector_problem::solve(&problem).unwrap(),
        vec![target]
    );
    assert_eq!(problem.squared_distance(&[target]).unwrap(), 0);
    assert_eq!(problem.squared_distance(&[target - 1]).unwrap(), 1);
    let cancellation = ClosestVectorProblem::<i64>::new(
        vec![vec![i64::MAX, 1], vec![i64::MAX - 1, 1]],
        vec![1_i64, 0],
    )
    .unwrap();
    assert_eq!(cancellation.squared_distance(&[1, -1]).unwrap(), 0);
}

#[test]
fn test_cvp_float_evaluation_and_solve_preserve_numeric_status() {
    let problem = ClosestVectorProblem::<f64>::new(vec![vec![1.0]], vec![0.25]).unwrap();
    assert_eq!(problem.squared_distance(&[1]).unwrap(), 0.5625);
    let value = problem.evaluate(&vec![1]).unwrap();
    let serialized =
        crate::registry::DynProblem::evaluate_json(&problem, &serde_json::json!([1])).unwrap();
    assert_eq!(
        serde_json::from_value::<Min<f64>>(serialized).unwrap(),
        value
    );
    assert_eq!(
        crate::registry::DynProblem::evaluate_dyn(&problem, &serde_json::json!([1])).unwrap(),
        ("Min(0.5625)".into(), true)
    );
    let loaded = crate::registry::LoadedDynProblem::new(Box::new(problem));
    let outcome = crate::solvers::solve(&loaded, crate::solvers::SolverRequest::Default)
        .unwrap()
        .outcome;
    assert_eq!(
        outcome,
        SolveOutcome::Feasible {
            solution: serde_json::json!([0]),
            evaluation: "Min(0.0625)".into(),
        }
    );
}

#[test]
fn test_cvp_serialization_round_trips_both_targets() {
    let integer = ClosestVectorProblem::<i64>::new(vec![vec![1_i64]], vec![2_i64]).unwrap();
    let json = serde_json::to_string(&integer).unwrap();
    assert!(!json.contains("bounds"));
    let decoded: ClosestVectorProblem<i64> = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.basis(), integer.basis());
    assert_eq!(decoded.target(), integer.target());

    let real = ClosestVectorProblem::<f64>::new(vec![vec![1.0]], vec![2.5]).unwrap();
    let json = serde_json::to_string(&real).unwrap();
    let decoded: ClosestVectorProblem<f64> = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.target(), real.target());
}

#[test]
fn test_cvp_create_specs_have_no_bounds() {
    let integer = ClosestVectorProblem::<i64>::try_from(ClosestVectorProblemI64CreateSpec {
        basis: vec![vec![1]],
        target: vec![2],
    })
    .unwrap();
    assert_eq!(integer.target(), &[2]);

    let real = ClosestVectorProblem::<f64>::try_from(ClosestVectorProblemF64CreateSpec {
        basis: vec![vec![1.0]],
        target: vec![2.5],
    })
    .unwrap();
    assert_eq!(real.target(), &[2.5]);
}

#[test]
fn test_cvp_registers_both_target_variants() {
    let mut variants = crate::registry::variant_entries()
        .into_iter()
        .filter(|entry| entry.name == ClosestVectorProblem::<i64>::NAME)
        .map(|entry| entry.variant_map())
        .collect::<Vec<_>>();
    variants.sort();
    assert_eq!(
        variants,
        vec![
            std::collections::BTreeMap::from([("coefficient".into(), "f64".into())]),
            std::collections::BTreeMap::from([("coefficient".into(), "i64".into())]),
        ]
    );
}

#[test]
fn test_cvp_empty_basis_is_valid() {
    let problem = ClosestVectorProblem::<i64>::new(Vec::new(), vec![3_i64, 4]).unwrap();
    assert_eq!(problem.evaluate(&Vec::new()).unwrap(), Min(Some(25)));
}

#[test]
fn test_cvp_oblique_grid_quantization() {
    use crate::models::decision::Decision;
    use crate::rules::{ReduceTo, ReductionResult};
    let problem =
        ClosestVectorProblem::<f64>::new(vec![vec![1.0, 0.0], vec![0.5, 0.8]], vec![1.6, 0.9])
            .unwrap();
    let solution =
        crate::solvers::customized::closest_vector_problem::solve_float(&problem).unwrap();
    assert_eq!(solution, vec![1, 1]);
    assert!((problem.squared_distance(&solution).unwrap() - 0.02).abs() < 1e-12);
    let decision = Decision::new(problem.clone(), 0.03);
    assert!(decision.evaluate(&solution).unwrap().0);
    let too_close = Decision::new(problem.clone(), 0.01);
    assert!(!too_close.evaluate(&solution).unwrap().0);
    let reduction = ReduceTo::<ClosestVectorProblem<f64>>::reduce_to(&too_close).unwrap();
    assert!(matches!(
        reduction.recover_result(
            &too_close,
            SolveOutcome::feasible(&problem, solution).unwrap()
        ),
        Err(crate::rules::ExtractionError::InsufficientSolutionQuality)
    ));
    let loaded = crate::registry::LoadedDynProblem::new(Box::new(problem));
    assert!(matches!(
        crate::solvers::solve(&loaded, crate::solvers::SolverRequest::Default)
            .unwrap()
            .outcome,
        SolveOutcome::Feasible { .. }
    ));
    for (bound, accepted) in [(0.03, true), (0.01, false)] {
        let decision = Decision::new(
            ClosestVectorProblem::<f64>::new(vec![vec![1.0, 0.0], vec![0.5, 0.8]], vec![1.6, 0.9])
                .unwrap(),
            bound,
        );
        let loaded = crate::registry::LoadedDynProblem::new(Box::new(decision));
        let result = crate::solvers::solve(&loaded, crate::solvers::SolverRequest::Default);
        if accepted {
            assert!(matches!(
                result.unwrap().outcome,
                SolveOutcome::Feasible { .. }
            ));
        } else {
            assert!(matches!(
                result,
                Err(crate::solvers::SolveError::Extraction(
                    crate::rules::ExtractionError::InsufficientSolutionQuality
                ))
            ));
        }
    }
    let graph = crate::rules::ReductionGraph::new();
    assert!(!graph.has_direct_reduction::<ClosestVectorProblem<i64>, ClosestVectorProblem<f64>>());
}

#[test]
fn test_cvp_reports_declared_arithmetic_errors() {
    let integer = ClosestVectorProblem::<i64>::new(vec![vec![2]], vec![0]).unwrap();
    assert!(matches!(
        integer.evaluate(&vec![i64::MAX]),
        Err(EvaluationError::IntegerOverflow(_))
    ));
    let float = ClosestVectorProblem::<f64>::new(vec![vec![f64::MAX]], vec![0.0]).unwrap();
    assert!(matches!(
        float.evaluate(&vec![2]),
        Err(EvaluationError::NonFiniteResult(_))
    ));
    assert!(float.evaluate(&vec![]).is_err());
    assert!(ClosestVectorProblem::<f64>::new(vec![vec![f64::NAN]], vec![0.0]).is_err());
    assert!(ClosestVectorProblem::<f64>::new(vec![vec![1.0], vec![2.0]], vec![0.0]).is_err());
    assert!(
        ClosestVectorProblem::<f64>::new(vec![vec![1.0, 2.0], vec![2.0, 4.0]], vec![0.0, 0.0])
            .is_err()
    );
}
