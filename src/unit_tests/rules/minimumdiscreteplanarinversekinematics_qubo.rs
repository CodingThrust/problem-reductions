use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
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
        reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), qubo_solutions[0].clone())
                    .unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
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
        reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), qubo_solutions[0].clone())
                    .unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
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
        let value = reduction
            .target_problem()
            .evaluate(&target_solution)
            .unwrap();
        assert_eq!(reduction.map_value(value), Min(None));
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
    assert_eq!(example.target.instance["matrix"]["nrows"], 4);
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!([0, 1])
    );
    assert_eq!(
        example.solutions[0].target_config,
        serde_json::json!([true, false, false, true])
    );
}

#[test]
fn optimum_energy_recovers_distance_and_infeasibility() {
    for source in [
        worked_example(),
        MinimumDiscretePlanarInverseKinematics::new(
            vec![1.0, 1.0, 1.0],
            (0.0, 0.0),
            vec![vec![0.0, PI]; 3],
            vec![vec![(0, 0)], vec![(1, 0)]],
        )
        .unwrap(),
    ] {
        let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
        let entry = inventory::iter::<crate::rules::ReductionEntry>
            .into_iter()
            .find(|entry| {
                entry.source_name == "MinimumDiscretePlanarInverseKinematics"
                    && entry.target_name == "QUBO"
            })
            .unwrap();
        let chain =
            crate::rules::ReductionChain::execute(&source, &[entry.reduce_fn.unwrap()]).unwrap();

        let solver = BruteForce::new();
        let expected = solver
            .solve(&source)
            .unwrap()
            .map(|solution| source.evaluate(&solution).unwrap().0.unwrap());
        for solution in solver
            .find_all_witnesses(reduction.target_problem())
            .unwrap()
        {
            let completed = chain
                .recover_result_json(
                    &source,
                    serde_json::json!({"status": "optimal", "solution": solution}),
                )
                .unwrap()
                .0;
            assert_eq!(
                matches!(completed, SolveOutcome::Optimal { .. }),
                expected.is_some()
            );
            let recovered = reduction
                .map_value(reduction.target_problem().evaluate(&solution).unwrap())
                .0;
            match (expected, recovered) {
                (Some(expected), Some(actual)) => {
                    assert!((actual - expected).abs() < EPS);
                    assert_eq!(
                        source
                            .evaluate(
                                &reduction
                                    .recover_result(
                                        &source,
                                        SolveOutcome::optimal(
                                            reduction.target_problem(),
                                            solution.clone()
                                        )
                                        .unwrap()
                                    )
                                    .map(|result| result.into_solution().expect(
                                        "qualifying target result must recover a source solution"
                                    ))
                                    .unwrap()
                            )
                            .unwrap(),
                        Min(Some(expected))
                    );
                }
                (None, None) => {}
                other => panic!("source and recovered outcomes disagree: {other:?}"),
            }
        }
        assert_eq!(reduction.map_value(Min(None)), Min(None));
    }
}

#[test]
fn nonfinite_energy_relation_is_a_construction_error() {
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![1e200],
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

/// Decode three two-sample one-hot blocks, or `None` when a block is not one-hot.
fn decode_one_hot_pairs(assignment: &[bool]) -> Option<Vec<usize>> {
    assignment
        .chunks(2)
        .map(|block| match block {
            [true, false] => Some(0),
            [false, true] => Some(1),
            _ => None,
        })
        .collect()
}

#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_feasible_target_incumbents() {
    // Three unit links pointing right or left; the second joint forbids (left, right).
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![1.0, 1.0, 1.0],
        (1.0, 0.0),
        vec![vec![0.0, PI]; 3],
        vec![
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            vec![(0, 0), (0, 1), (1, 1)],
        ],
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
    let qubo = reduction.target_problem();
    assert_eq!(qubo.num_vars(), 6);

    let mut suboptimal = 0;
    let mut rejected = 0;
    for bits in 0..1usize << 6 {
        let candidate: Vec<bool> = (0..6).map(|i| bits & (1 << i) != 0).collect();
        let expected = decode_one_hot_pairs(&candidate)
            .map(|config| (source.evaluate(&config).unwrap(), config))
            .filter(|(evaluation, _)| evaluation.0.is_some());
        let recovered =
            reduction.recover_result(&source, SolveOutcome::feasible(qubo, candidate).unwrap());
        match expected {
            Some((evaluation, solution)) => {
                // (right, right, right) ends at (3, 0), squared distance 4 from the goal.
                suboptimal += usize::from(matches!(evaluation, Min(Some(v)) if v > 1.0));
                assert_eq!(
                    recovered,
                    Ok(SolveOutcome::Feasible {
                        solution,
                        evaluation,
                    })
                );
            }
            None => {
                rejected += 1;
                assert_eq!(
                    recovered,
                    Err(crate::rules::ExtractionError::InsufficientSolutionQuality)
                );
            }
        }
    }
    // Six of the eight one-hot assignments satisfy the transition constraints.
    assert_eq!(rejected, 64 - 6);
    assert!(suboptimal > 0);
}

#[test]
fn test_minimumdiscreteplanarinversekinematics_to_qubo_infeasible_source_recovers_infeasible() {
    // The middle link would need sample 0 and sample 1 at once.
    let source = MinimumDiscretePlanarInverseKinematics::new(
        vec![1.0, 1.0, 1.0],
        (0.0, 0.0),
        vec![vec![0.0, PI]; 3],
        vec![vec![(0, 0)], vec![(1, 0)]],
    )
    .unwrap();
    assert_eq!(BruteForce::new().solve(&source).unwrap(), None);

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
    let qubo = reduction.target_problem();
    let optimum = BruteForce::new().solve(qubo).unwrap().unwrap();
    assert_eq!(
        reduction.recover_result(
            &source,
            SolveOutcome::optimal(qubo, optimum.clone()).unwrap()
        ),
        Ok(SolveOutcome::Infeasible)
    );
    // The same assignment without an optimality proof establishes nothing.
    assert_eq!(
        reduction.recover_result(&source, SolveOutcome::feasible(qubo, optimum).unwrap()),
        Err(crate::rules::ExtractionError::InsufficientSolutionQuality)
    );
}
