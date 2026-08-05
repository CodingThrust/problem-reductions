use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;
use std::f64::consts::{FRAC_PI_2, PI};

const EPS: f64 = 1e-9;

fn worked_example() -> MinimumDiscretePlanarInverseKinematics {
    MinimumDiscretePlanarInverseKinematics::new(
        vec![2.0, 1.0],
        (2.0, 1.0),
        vec![vec![0.0, FRAC_PI_2], vec![0.0, FRAC_PI_2]],
        vec![vec![(0, 0), (0, 1), (1, 1)]],
    )
}

#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_closed_loop() {
    let source = worked_example();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);

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
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(reduction.target_problem());

    assert_eq!(reduction.target_problem().num_vars(), 3);
    assert_eq!(qubo_solutions.len(), 1);
    assert_eq!(
        reduction.extract_solution(&qubo_solutions[0]).unwrap(),
        vec![1]
    );
    assert!(matches!(source.evaluate(&[1]), Min(Some(v)) if v.abs() < EPS));
}

#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_single_sample_per_link() {
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![1.0, 2.0, 1.5],
        (0.0, 4.5),
        vec![vec![FRAC_PI_2], vec![FRAC_PI_2], vec![FRAC_PI_2]],
        vec![vec![(0, 0)], vec![(0, 0)]],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(reduction.target_problem());

    assert_eq!(reduction.target_problem().num_vars(), 3);
    assert_eq!(qubo_solutions, vec![vec![1, 1, 1]]);
    assert_eq!(
        reduction.extract_solution(&qubo_solutions[0]).unwrap(),
        vec![0, 0, 0]
    );
    assert!(matches!(source.evaluate(&[0, 0, 0]), Min(Some(v)) if v.abs() < EPS));
}

#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_empty_allowed_pairs() {
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![1.0, 1.0],
        (2.0, 0.0),
        vec![vec![0.0, FRAC_PI_2], vec![0.0, FRAC_PI_2]],
        vec![vec![]],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(reduction.target_problem());

    assert_eq!(solver.solve(&source), Min(None));
    assert!(!qubo_solutions.is_empty(), "QUBO solver found no solutions");
    for target_solution in qubo_solutions {
        let extracted = reduction.extract_solution(&target_solution).unwrap();
        assert_eq!(source.evaluate(&extracted), Min(None));
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
    assert_eq!(example.solutions[0].source_config, vec![0_usize, 1]);
    assert_eq!(example.solutions[0].target_config, vec![1_usize, 0, 0, 1]);
}
