use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;
use std::f64::consts::{FRAC_PI_2, PI};

const EPS: f64 = 1e-9;

#[test]
fn test_large_link_keeps_one_hot_penalty_strict() {
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![134_217_728.0],
        (0.0, 0.0),
        vec![vec![0.0]],
        vec![],
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
    let solutions = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    assert_eq!(solutions, vec![vec![true]]);
}

#[test]
fn test_extraction_rejects_forbidden_pair() {
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&worked_example()).unwrap();
    assert!(reduction
        .extract_solution(&vec![false, true, true, false])
        .is_err());
}

fn worked_example() -> MinimumDiscretePlanarInverseKinematics {
    MinimumDiscretePlanarInverseKinematics::new(
        vec![2.0, 1.0],
        (2.0, 1.0),
        vec![vec![0.0, FRAC_PI_2], vec![0.0, FRAC_PI_2]],
        vec![vec![(0, 0), (0, 1), (1, 1)]],
    )
    .unwrap()
}

#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_closed_loop() {
    let source = worked_example();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).expect("reduction should succeed");

    assert_eq!(reduction.target_problem().num_vars(), 4);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinimumDiscretePlanarInverseKinematics->QUBO closed loop",
    );
}

#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_single_link() {
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![2.0],
        (0.0, 2.0),
        vec![vec![0.0, FRAC_PI_2, PI]],
        vec![],
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).expect("reduction should succeed");
    let solver = BruteForce::new();
    let qubo_solutions = solver
        .find_all_witnesses(reduction.target_problem())
        .unwrap();

    assert_eq!(reduction.target_problem().num_vars(), 3);
    assert_eq!(qubo_solutions.len(), 1);
    assert_eq!(
        reduction.extract_solution(&qubo_solutions[0]).unwrap(),
        vec![1]
    );
    assert!(matches!(source.evaluate(&vec![1]).unwrap(), Min(Some(v)) if v.abs() < EPS));
}

#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_single_sample_per_link() {
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![1.0, 2.0, 1.5],
        (0.0, 4.5),
        vec![vec![FRAC_PI_2], vec![FRAC_PI_2], vec![FRAC_PI_2]],
        vec![vec![(0, 0)], vec![(0, 0)]],
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).expect("reduction should succeed");
    let solver = BruteForce::new();
    let qubo_solutions = solver
        .find_all_witnesses(reduction.target_problem())
        .unwrap();

    assert_eq!(reduction.target_problem().num_vars(), 3);
    assert_eq!(qubo_solutions, vec![vec![true, true, true]]);
    assert_eq!(
        reduction.extract_solution(&qubo_solutions[0]).unwrap(),
        vec![0, 0, 0]
    );
    assert!(matches!(source.evaluate(&vec![0, 0, 0]).unwrap(), Min(Some(v)) if v.abs() < EPS));
}

#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_empty_allowed_pairs() {
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![1.0, 1.0],
        (2.0, 0.0),
        vec![vec![0.0, FRAC_PI_2], vec![0.0, FRAC_PI_2]],
        vec![vec![]],
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).expect("reduction should succeed");
    let solver = BruteForce::new();
    let qubo_solutions = solver
        .find_all_witnesses(reduction.target_problem())
        .unwrap();

    assert!(solver.solve(&source).unwrap().is_none());
    assert!(!qubo_solutions.is_empty(), "QUBO solver found no solutions");
    for target_solution in qubo_solutions {
        assert!(reduction.extract_solution(&target_solution).is_err());
    }
}

#[test]
fn test_extraction_rejects_malformed_selectors() {
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&worked_example()).unwrap();
    for config in [
        vec![],
        vec![false, false, false, true],
        vec![true, true, false, true],
    ] {
        assert!(reduction.extract_solution(&config).is_err());
    }
}

#[test]
fn test_non_finite_penalty_is_a_reduction_error() {
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![f64::MAX],
        (0.0, 0.0),
        vec![vec![0.0]],
        vec![],
    )
    .unwrap();
    assert!(matches!(
        ReduceTo::<QUBO<f64>>::reduce_to(&source),
        Err(crate::rules::ReductionError::NonFiniteResult { .. })
    ));
}

#[test]
fn test_all_two_link_pair_relations_preserve_optima() {
    let solver = BruteForce::new();
    for mask in 0..16 {
        let pairs: Vec<_> = (0..4)
            .filter(|bit| mask & (1 << bit) != 0)
            .map(|bit| (bit / 2, bit % 2))
            .collect();
        for target in [(0.0, 0.0), (2.0, 1.0), (-1.0, 2.0)] {
            let source = MinimumDiscretePlanarInverseKinematics::new(
                vec![2.0, 1.0],
                target,
                vec![vec![0.0, FRAC_PI_2], vec![0.0, FRAC_PI_2]],
                vec![pairs.clone()],
            )
            .unwrap();
            let optimum = solver
                .solve(&source)
                .unwrap()
                .map(|config| source.evaluate(&config).unwrap().0.unwrap());
            let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
            for bits in solver
                .find_all_witnesses(reduction.target_problem())
                .unwrap()
            {
                let extracted = reduction.extract_solution(&bits);
                if let Some(expected) = optimum {
                    let actual = source.evaluate(&extracted.unwrap()).unwrap().0.unwrap();
                    assert!(
                        (actual - expected).abs() < EPS,
                        "mask {mask}, target {target:?}"
                    );
                } else {
                    assert!(extracted.is_err());
                }
            }
        }
    }
}

#[cfg(feature = "example-db")]
#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "minimumdiscreteplanarinversekinematics_to_qubo")
        .expect("missing canonical MinimumDiscretePlanarInverseKinematics -> QUBO example spec");
    let example = (spec.build)();

    assert_eq!(
        example.source.problem,
        "MinimumDiscretePlanarInverseKinematics"
    );
    assert_eq!(example.target.problem, "QUBO");
    assert_eq!(example.target.instance["num_vars"], 4);
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!([0, 1])
    );
    assert_eq!(
        example.solutions[0].target_config,
        serde_json::json!([true, false, false, true])
    );
}
