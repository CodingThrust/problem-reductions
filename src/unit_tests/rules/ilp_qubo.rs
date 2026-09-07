use super::*;
use crate::models::algebraic::{LinearConstraint, ObjectiveSense};
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;

#[test]
fn test_ilp_to_qubo_closed_loop() {
    // Binary ILP: maximize x0 + 2*x1 + 3*x2
    // s.t. x0 + x1 <= 1, x1 + x2 <= 1
    // Optimal: x = [1, 0, 1] with obj = 4
    let ilp = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1)], 1),
            LinearConstraint::le(vec![(1, 1), (2, 1)], 1),
        ],
        vec![(0, 1), (1, 2), (2, 3)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ilp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(ilp.is_feasible(&extracted).unwrap());
    }

    // Optimal should be [1, 0, 1]
    let best = reduction.extract_solution(&qubo_solutions[0]).unwrap();
    assert_eq!(best, vec![1, 0, 1]);
}

#[test]
fn test_ilp_to_qubo_minimize() {
    // Binary ILP: minimize x0 + 2*x1 + 3*x2
    // s.t. x0 + x1 >= 1 (at least one of x0, x1 selected)
    // Optimal: x = [1, 0, 0] with obj = 1
    let ilp = ILP::<bool>::new(
        3,
        vec![LinearConstraint::ge(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 2), (2, 3)],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ilp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(ilp.is_feasible(&extracted).unwrap());
    }

    let best = reduction.extract_solution(&qubo_solutions[0]).unwrap();
    assert_eq!(best, vec![1, 0, 0]);
}

#[test]
fn test_ilp_to_qubo_equality() {
    // Binary ILP: maximize x0 + x1 + x2
    // s.t. x0 + x1 + x2 = 2
    // Optimal: any 2 of 3 variables = 1
    let ilp = ILP::<bool>::new(
        3,
        vec![LinearConstraint::eq(vec![(0, 1), (1, 1), (2, 1)], 2)],
        vec![(0, 1), (1, 1), (2, 1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ilp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    // Should have exactly 3 optimal solutions (C(3,2))
    assert_eq!(qubo_solutions.len(), 3);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(ilp.is_feasible(&extracted).unwrap());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 2);
    }
}

#[test]
fn test_ilp_to_qubo_ge_with_slack() {
    // Ge constraint with slack_range > 1 to exercise slack variable code path.
    // 3 vars: minimize x0 + x1 + x2
    // s.t. x0 + x1 + x2 >= 1 (max_lhs=3, b=1, slack_range=2, ns=ceil(log2(3))=2)
    let ilp = ILP::<bool>::new(
        3,
        vec![LinearConstraint::ge(vec![(0, 1), (1, 1), (2, 1)], 1)],
        vec![(0, 1), (1, 1), (2, 1)],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ilp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    // 3 original + ceil(log2(3))=2 slack = 5 QUBO variables
    assert_eq!(qubo.num_variables(), 5);

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(ilp.is_feasible(&extracted).unwrap());
    }

    // Optimal: exactly one variable = 1
    let best = reduction.extract_solution(&qubo_solutions[0]).unwrap();
    assert_eq!(best.iter().sum::<i64>(), 1);
}

#[test]
fn test_ilp_to_qubo_le_with_slack() {
    // Le constraint with rhs > 1 to exercise Le slack variable code path.
    // 3 vars: maximize x0 + x1 + x2
    // s.t. x0 + x1 + x2 <= 2 (min_lhs=0, b=2, slack_range=2, ns=ceil(log2(3))=2)
    let ilp = ILP::<bool>::new(
        3,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1), (2, 1)], 2)],
        vec![(0, 1), (1, 1), (2, 1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ilp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    // 3 original + ceil(log2(3))=2 slack = 5 QUBO variables
    assert_eq!(qubo.num_variables(), 5);

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(ilp.is_feasible(&extracted).unwrap());
    }

    // Optimal: exactly 2 of 3 variables = 1 (3 solutions)
    let best = reduction.extract_solution(&qubo_solutions[0]).unwrap();
    assert_eq!(best.iter().sum::<i64>(), 2);
}

#[test]
fn test_ilp_to_qubo_structure() {
    let ilp = ILP::<bool>::new(
        3,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 2), (2, 3)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ilp).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    // Verify QUBO has appropriate structure
    assert!(qubo.num_variables() >= ilp.num_vars());
}

#[test]
fn test_ilp_qubo_all_small_rows_and_target_assignments() {
    use crate::rules::AggregateReductionResult;
    use crate::Problem;
    for n in 0usize..=3 {
        for mut code in 0..3usize.pow(n as u32) {
            let coefficients: Vec<_> = (0..n)
                .map(|i| {
                    let c = (code % 3) as i64 - 1;
                    code /= 3;
                    (i, c)
                })
                .collect();
            for rhs in -2..=2 {
                for comparison in [Comparison::Eq, Comparison::Le, Comparison::Ge] {
                    let row = match comparison {
                        Comparison::Eq => LinearConstraint::eq(coefficients.clone(), rhs),
                        Comparison::Le => LinearConstraint::le(coefficients.clone(), rhs),
                        Comparison::Ge => LinearConstraint::ge(coefficients.clone(), rhs),
                    };
                    for sense in [ObjectiveSense::Minimize, ObjectiveSense::Maximize] {
                        let objective: Vec<_> = (0..n)
                            .map(|i| (i, if i % 2 == 0 { 2 } else { -1 }))
                            .collect();
                        let source =
                            ILP::<bool>::new(n, vec![row.clone()], objective.clone(), sense)
                                .unwrap();
                        let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
                        let target = AggregateReductionResult::target_problem(&reduction);
                        let penalty =
                            objective.iter().map(|(_, c)| c.abs()).sum::<i64>() + rhs.abs() + 1;
                        let constant = penalty * rhs * rhs;
                        for mask in 0usize..(1 << target.num_vars()) {
                            let config: Vec<_> = (0..target.num_vars())
                                .map(|i| (mask >> i) & 1 == 1)
                                .collect();
                            let prefix: Vec<_> =
                                config[..n].iter().map(|&x| i64::from(x)).collect();
                            let source_value = source.evaluate(&prefix).unwrap();
                            let energy = target.evaluate(&config).unwrap();
                            let original_objective = source.evaluate_objective(&prefix).unwrap();
                            let normalized_objective = match sense {
                                ObjectiveSense::Minimize => original_objective,
                                ObjectiveSense::Maximize => -original_objective,
                            };
                            // Independently detect zero squared-residual penalty.
                            let certifies = source_value.is_valid()
                                && energy.0.unwrap() + constant == normalized_objective;
                            let decoded = reduction.extract_solution(&config);
                            assert_eq!(decoded.is_ok(), certifies);
                            let extracted_value =
                                AggregateReductionResult::extract_value(&reduction, energy);
                            assert_eq!(extracted_value.is_valid(), certifies);
                            if let Ok(solution) = decoded {
                                assert_eq!(source.evaluate(&solution).unwrap(), source_value);
                                assert_eq!(extracted_value, source_value);
                            }
                        }
                        let best = BruteForce::new().solve(target).unwrap().unwrap();
                        let actual = AggregateReductionResult::extract_value(
                            &reduction,
                            target.evaluate(&best).unwrap(),
                        );
                        let mut expected = match sense {
                            ObjectiveSense::Minimize => crate::types::Extremum::minimize(None),
                            ObjectiveSense::Maximize => crate::types::Extremum::maximize(None),
                        };
                        for mask in 0usize..(1 << n) {
                            let assignment: Vec<_> =
                                (0..n).map(|i| ((mask >> i) & 1) as i64).collect();
                            expected = crate::types::Aggregate::combine(
                                expected,
                                source.evaluate(&assignment).unwrap(),
                            )
                            .unwrap();
                        }
                        assert_eq!(actual, expected);
                        assert!(reduction
                            .extract_solution(&vec![false; target.num_vars() + 1])
                            .is_err());
                    }
                }
            }
        }
    }
}

#[test]
fn test_ilp_qubo_inconsistent_rows_and_absent_aggregate() {
    use crate::rules::AggregateReductionResult;
    use crate::Problem;
    for sense in [ObjectiveSense::Minimize, ObjectiveSense::Maximize] {
        let source = ILP::<bool>::new(
            3,
            vec![
                LinearConstraint::eq(vec![(0, 1)], 0),
                LinearConstraint::eq(vec![(0, 1)], 1),
            ],
            vec![],
            sense,
        )
        .unwrap();
        let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
        for config in [vec![false; 3], vec![true; 3]] {
            let value = ReductionResult::target_problem(&reduction)
                .evaluate(&config)
                .unwrap();
            assert!(!AggregateReductionResult::extract_value(&reduction, value).is_valid());
            assert!(reduction.extract_solution(&config).is_err());
        }
        assert!(
            !AggregateReductionResult::extract_value(&reduction, crate::types::Min(None))
                .is_valid()
        );
        assert!(!AggregateReductionResult::extract_value(
            &reduction,
            crate::types::Min(Some(i64::MIN))
        )
        .is_valid());
        assert!(!AggregateReductionResult::extract_value(
            &reduction,
            crate::types::Min(Some(i64::MAX))
        )
        .is_valid());
    }
}

#[test]
fn test_ilp_qubo_checked_dimensions_and_energy_interval() {
    assert_eq!(qubo_num_variables(3, 2).unwrap(), 5);
    for (n, s) in [(usize::MAX, 1), (0, usize::MAX)] {
        assert!(matches!(
            qubo_num_variables(n, s),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
    assert_eq!(
        feasible_energy_range(&[1, -2], &[1], 3).unwrap(),
        (3, -4, -1)
    );
    for (cost, rhs, penalty) in [
        (vec![], vec![i64::MAX], 1),
        (vec![], vec![2], i64::MAX),
        (vec![], vec![1, 1], i64::MAX),
        (vec![i64::MAX, i64::MAX], vec![], 1),
        (vec![i64::MIN], vec![], 1),
        (vec![2], vec![1], i64::MAX),
    ] {
        assert!(matches!(
            feasible_energy_range(&cost, &rhs, penalty),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
