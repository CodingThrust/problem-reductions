use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;

#[test]
fn test_setpacking_to_qubo_closed_loop() {
    // 3 sets: {0,2}, {1,2}, {0,3}
    // Overlaps: (0,1) share element 2, (0,2) share element 0
    // Max packing: sets 1 and 2 → {1,2} and {0,3} (no overlap)
    let sp = MaximumSetPacking::<f64>::new(vec![vec![0, 2], vec![1, 2], vec![0, 3]]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(sp.evaluate(&extracted).unwrap().is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x).count(), 2);
    }
}

#[test]
fn test_setpacking_to_qubo_disjoint() {
    // Disjoint sets: all can be packed
    let sp = MaximumSetPacking::<f64>::new(vec![vec![0, 1], vec![2, 3], vec![4]]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(sp.evaluate(&extracted).unwrap().is_valid());
        // All 3 sets should be selected
        assert_eq!(extracted.iter().filter(|&&x| x).count(), 3);
    }
}

#[test]
fn test_setpacking_to_qubo_all_overlap() {
    // All sets overlap: only 1 can be selected
    let sp = MaximumSetPacking::<f64>::new(vec![vec![0, 1], vec![0, 2], vec![0, 3]]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(sp.evaluate(&extracted).unwrap().is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x).count(), 1);
    }
}

#[test]
fn test_setpacking_to_qubo_structure() {
    let sp = MaximumSetPacking::<f64>::new(vec![vec![0, 2], vec![1, 2], vec![0, 3]]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    // QUBO should have same number of variables as sets
    assert_eq!(qubo.num_variables(), 3);
}

#[test]
fn test_setpacking_to_qubo_signed_weights_all_optima() {
    use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;

    // Every intersection graph on three sets and every combination of signed,
    // zero and fractional weights. Edge-specific elements realize each graph.
    let choices = [-3.0, -0.5, 0.0, 0.5, 4.0];
    for graph in 0..8 {
        let mut sets = vec![Vec::new(); 3];
        for (edge, (i, j)) in [(0, 1), (0, 2), (1, 2)].into_iter().enumerate() {
            if graph & (1 << edge) != 0 {
                sets[i].push(edge);
                sets[j].push(edge);
            }
        }
        for a in choices {
            for b in choices {
                for c in choices {
                    let source =
                        MaximumSetPacking::with_weights(sets.clone(), vec![a, b, c]).unwrap();
                    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
                    assert_optimization_round_trip_from_optimization_target(
                        &source,
                        &reduction,
                        "signed intersection graph",
                    );
                }
            }
        }
    }
}

#[test]
fn test_setpacking_to_qubo_penalty_strict_at_large_weights() {
    // Adding one rounds back to the same weight at this magnitude.
    let weight = 2.0_f64.powi(54);
    let source =
        MaximumSetPacking::with_weights(vec![vec![0], vec![0]], vec![weight, weight]).unwrap();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
    assert!(reduction.target_problem().matrix()[0][1] > weight);
    crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "large exactly representable weights",
    );
}

#[test]
fn test_setpacking_to_qubo_empty() {
    let source = MaximumSetPacking::<f64>::new(vec![]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();
    crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "empty packing",
    );
}

#[test]
fn test_setpacking_to_qubo_non_finite_penalty_is_typed_error() {
    let source =
        MaximumSetPacking::with_weights(vec![vec![0], vec![0]], vec![f64::MAX, 1.0]).unwrap();
    assert!(matches!(
        ReduceTo::<QUBO<f64>>::reduce_to(&source),
        Err(crate::rules::ReductionError::NonFiniteResult { .. })
    ));
}
