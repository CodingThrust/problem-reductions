use super::*;
use crate::models::formula::CNFClause;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;
use num_traits::ToPrimitive;

fn source(n: usize, clauses: Vec<Vec<i64>>) -> KSatisfiability<K3> {
    KSatisfiability::try_new_allow_less(n, clauses.into_iter().map(CNFClause::new).collect())
        .unwrap()
}

#[test]
fn test_ksatisfiability_to_quadraticcongruences_closed_loop() {
    // This target is small enough to exhaust with the registered reference solver.
    let source = source(4, vec![vec![1, -1, 4]]);
    let reduction = ReduceTo::<QuadraticCongruences>::reduce_to(&source).unwrap();
    let solution = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    assert_eq!(
        reduction.target_problem().evaluate(&solution).unwrap(),
        Or(true)
    );
    assert_eq!(
        source
            .evaluate(&reduction.extract_solution(&solution).unwrap())
            .unwrap(),
        Or(true)
    );
}

#[test]
fn test_native_clauses_and_arbitrary_crt_signs() {
    for source in [
        source(0, vec![]),
        source(0, vec![vec![]]),
        source(3, vec![vec![1, 1, 1]]),
        source(3, vec![vec![-1, -1, -1]]),
        source(3, vec![vec![1, 2]]),
        source(3, vec![vec![1, 2, 3]]),
        source(3, vec![vec![-3, -1, 2]]),
        source(2, vec![vec![1], vec![-1]]),
        source(2, vec![vec![1, 2], vec![-1, -2]]),
        source(2, vec![vec![], vec![1, 2]]),
        source(17, vec![vec![17, 17, 17]]),
    ] {
        let construction = build_construction(&source).unwrap();
        let reduction = ReduceTo::<QuadraticCongruences>::reduce_to(&source).unwrap();
        let count = construction.thetas.len();
        let k: BigUint = construction.prime_powers.iter().product();
        assert!(&construction.h * 2u32 < k);
        let mut recovered = BTreeSet::new();
        for mask in 0..(1usize << count) {
            let signs: Vec<i8> = (0..count)
                .map(|i| if mask & (1 << i) == 0 { 1 } else { -1 })
                .collect();
            let witness = witness_value_from_alphas(&signs, &construction.thetas);
            let valid = reduction.target_problem().evaluate(&witness).unwrap().0;
            let extracted = reduction.extract_solution(&witness);
            if valid {
                let extracted = extracted.unwrap();
                assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
                for (i, &original) in construction.active_to_source.iter().enumerate() {
                    assert_eq!(
                        extracted[original],
                        signs[0] != signs[2 * construction.clauses.len() + i + 1]
                    );
                }
                recovered.insert(extracted);
            } else {
                assert!(extracted.is_err());
            }
        }
        // Enumerate only appearing variables; unused coordinates are free.
        let mut expected = BTreeSet::new();
        for mask in 0..(1usize << construction.active_to_source.len()) {
            let mut assignment = vec![false; source.num_vars()];
            for (i, &original) in construction.active_to_source.iter().enumerate() {
                assignment[original] = mask & (1 << i) != 0;
            }
            if source.evaluate(&assignment).unwrap().0 {
                let signs = build_alphas(&construction, &assignment).unwrap();
                let witness = witness_value_from_alphas(&signs, &construction.thetas);
                assert_eq!(
                    reduction.target_problem().evaluate(&witness).unwrap(),
                    Or(true)
                );
                assert_eq!(reduction.extract_solution(&witness).unwrap(), assignment);
                expected.insert(assignment);
            } else {
                assert!(build_alphas(&construction, &assignment).is_none());
            }
        }
        assert_eq!(recovered, expected);
    }
}

#[test]
fn test_unsatisfiable_formula_has_no_quadratic_sign_witness() {
    let rows: Vec<_> = (0..8)
        .map(|mask| {
            (0..3)
                .map(|i| if mask & (1 << i) == 0 { i + 1 } else { -i - 1 })
                .collect()
        })
        .collect();
    let source = source(3, rows);
    let construction = build_construction(&source).unwrap();
    let coefficients: Vec<i64> = construction
        .coefficients
        .iter()
        .map(|c| c.to_i64().unwrap())
        .collect();
    let tau = construction.tau.to_i64().unwrap();
    let modulus = 2 * 8i64.pow(9);
    let mut linear: i64 = coefficients.iter().sum();
    let mut signs = vec![1i64; coefficients.len()];
    // Global negation has the same square, so fix alpha_0=+1. This exhausts
    // all 524288 candidate CRT sign classes, including non-knapsack roots.
    for mask in 0..(1usize << (coefficients.len() - 1)) {
        if mask != 0 {
            let bit = mask.trailing_zeros() as usize + 1;
            linear -= 2 * signs[bit] * coefficients[bit];
            signs[bit] = -signs[bit];
        }
        assert_ne!((linear * linear - tau * tau).rem_euclid(modulus), 0);
    }
    // Explicit regression: leave only the highest clause false and substitute
    // y=1 for its impossible y=-1. The old doubled construction accepted this.
    let mut signs = vec![1i8; construction.thetas.len()];
    for (j, clause) in construction.clauses.iter().enumerate() {
        let y = if j == 7 {
            1
        } else {
            clause.iter().filter(|&&lit| lit < 0).count() - 1
        };
        signs[2 * j + 1] = if y & 1 == 0 { 1 } else { -1 };
        signs[2 * j + 2] = if y < 2 { 1 } else { -1 };
    }
    assert_eq!(construction.clauses.last().unwrap(), &vec![1, 2, 3]);
    let witness = witness_value_from_alphas(&signs, &construction.thetas);
    assert_eq!(construction.target.evaluate(&witness).unwrap(), Or(false));
}

#[test]
fn test_normalization_preserves_free_variables_and_formula() {
    let canonical = source(5, vec![vec![-5, 2]]);
    let redundant = source(5, vec![vec![2, -5, 2], vec![-5, 2, -5], vec![1, -1, 4]]);
    let first = ReduceTo::<QuadraticCongruences>::reduce_to(&canonical).unwrap();
    let second = ReduceTo::<QuadraticCongruences>::reduce_to(&redundant).unwrap();
    assert_eq!(
        serde_json::to_value(first.target_problem()).unwrap(),
        serde_json::to_value(second.target_problem()).unwrap()
    );
    let assignment = [true, true, true, true, false];
    let witness = witness_config_for_assignment(&redundant, &assignment).unwrap();
    assert_eq!(
        second.extract_solution(&witness).unwrap(),
        vec![false, true, false, false, false]
    );
    assert!(witness_config_for_assignment(&redundant, &[]).is_none());
    assert!(
        witness_config_for_assignment(&redundant, &[false, false, false, false, true]).is_none()
    );
}

#[test]
fn test_rejects_infeasible_and_out_of_bound_integers() {
    let source = source(3, vec![vec![1, 2, 3]]);
    let reduction = ReduceTo::<QuadraticCongruences>::reduce_to(&source).unwrap();
    for witness in [
        BigUint::zero(),
        reduction.h.clone(),
        reduction.target.c().clone(),
        reduction.target.c() + 1u32,
    ] {
        assert_eq!(reduction.target.evaluate(&witness).unwrap(), Or(false));
        assert!(reduction.extract_solution(&witness).is_err());
    }
}

#[test]
fn test_prime_generation_and_parameter_bounds() {
    for (candidate, prime) in [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (9, false),
        (13, true),
        (25, false),
        (u64::MAX, false),
    ] {
        assert_eq!(is_prime(candidate), prime);
    }
    assert!(admissible_primes(0).unwrap().is_empty());
    assert_eq!(admissible_primes(4).unwrap(), vec![13, 17, 19, 23]);
    for source in [
        source(0, vec![]),
        source(0, vec![vec![]]),
        source(3, vec![vec![1, 2, 3]]),
        source(1, vec![vec![1], vec![-1]]),
    ] {
        let reduction = ReduceTo::<QuadraticCongruences>::reduce_to(&source).unwrap();
        let n = 2 * source.num_clauses() + source.num_vars() + 1;
        let bound = 64 * n * n + 3 * source.num_clauses() + 4;
        let target = reduction.target_problem();
        assert!(
            target.bit_length_a() <= bound
                && target.bit_length_b() <= bound
                && target.bit_length_c() <= bound
        );
    }
}
