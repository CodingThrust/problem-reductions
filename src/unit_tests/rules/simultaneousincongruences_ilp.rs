use super::*;
use crate::models::algebraic::Comparison;
use crate::solvers::{BruteForce, ILPSolveError, ILPSolver};
use crate::traits::Problem;

type Row = (Vec<(usize, i64)>, Comparison, i64);

fn reduce(
    pairs: &[(i64, i64)],
) -> (
    SimultaneousIncongruences,
    ReductionSimultaneousIncongruencesToILP,
) {
    let source = SimultaneousIncongruences::new(pairs.to_vec()).unwrap();
    let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    (source, reduced)
}

fn is_feasible(target: &ILP<i64, i64, Bounded>, values: &[i64]) -> bool {
    target.is_feasible(values).unwrap()
}

/// Whether some point of the target is feasible, deciding it by scanning `x` and, for
/// each pair, the local `(q, r, z)` triples (the rows meet only through `x`).
fn target_has_point(target: &ILP<i64, i64, Bounded>, m: usize) -> bool {
    let vars = target.variables();
    let x_hi = vars[0].upper_bound().unwrap();
    (0..=x_hi).any(|x| {
        (0..m).all(|i| {
            let (q, r, z) = (1 + i, 1 + m + i, 1 + 2 * m + i);
            let mut v = vec![0_i64; 1 + 3 * m];
            v[0] = x;
            (0..=vars[q].upper_bound().unwrap()).any(|qv| {
                (0..=vars[r].upper_bound().unwrap()).any(|rv| {
                    (0..=1).any(|zv| {
                        v[q] = qv;
                        v[r] = rv;
                        v[z] = zv;
                        target
                            .constraints()
                            .iter()
                            .filter(|row| row.terms().iter().any(|&(k, _)| [q, r, z].contains(&k)))
                            .all(|row| row.is_satisfied(&v).unwrap())
                    })
                })
            })
        })
    })
}

fn all_pairs(max_modulus: i64) -> Vec<(i64, i64)> {
    (1..=max_modulus)
        .flat_map(|b| (1..=b).map(move |a| (a, b)))
        .collect()
}

fn source_has_witness(source: &SimultaneousIncongruences) -> bool {
    (0..source.lcm_moduli()).any(|x| source.evaluate(&x).unwrap().0)
}

#[test]
fn issue_example_has_the_documented_target_and_witness() {
    let (source, reduced) = reduce(&[(2, 2), (1, 3), (2, 3)]);
    let target = reduced.target_problem();
    assert_eq!(source.lcm_moduli(), 6);
    assert_eq!(target.num_vars(), 10);
    assert_eq!(target.num_constraints(), 9);
    assert_eq!(target.num_nonzeros(), 21);
    let bounds: Vec<(i64, i64)> = target
        .variables()
        .iter()
        .map(|v| (v.lower_bound().unwrap(), v.upper_bound().unwrap()))
        .collect();
    assert_eq!(
        bounds,
        vec![
            (0, 5),
            (0, 2),
            (0, 1),
            (0, 1),
            (0, 1),
            (0, 2),
            (0, 2),
            (0, 1),
            (0, 1),
            (0, 1)
        ]
    );
    let rows: Vec<Row> = target
        .constraints()
        .iter()
        .map(|row| (row.terms().to_vec(), row.comparison(), row.rhs()))
        .collect();
    let expected: Vec<Row> = vec![
        (vec![(0, 1), (1, -2), (4, -1)], Comparison::Eq, 0),
        (vec![(4, 1), (7, -2)], Comparison::Le, -1),
        (vec![(4, 1), (7, -2)], Comparison::Ge, -1),
        (vec![(0, 1), (2, -3), (5, -1)], Comparison::Eq, 0),
        (vec![(5, 1), (8, -3)], Comparison::Le, 0),
        (vec![(5, 1), (8, -3)], Comparison::Ge, -1),
        (vec![(0, 1), (3, -3), (6, -1)], Comparison::Eq, 0),
        (vec![(6, 1), (9, -3)], Comparison::Le, 1),
        (vec![(6, 1), (9, -3)], Comparison::Ge, 0),
    ];
    assert_eq!(rows, expected);
    let witness = vec![3, 1, 1, 1, 1, 0, 0, 1, 0, 0];
    assert!(is_feasible(target, &witness));
    assert_eq!(reduced.extract_solution(&witness).unwrap(), 3);
    assert_eq!(
        (0..6)
            .filter(|x| source.evaluate(x).unwrap().0)
            .collect::<Vec<_>>(),
        vec![3]
    );
    crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
}

#[test]
fn exhaustive_small_lists_agree_and_every_witness_extracts() {
    let base = all_pairs(3);
    let mut lists = vec![vec![]];
    let mut count = 0;
    let mut frontier = vec![vec![]];
    for _ in 0..5 {
        let mut next = Vec::new();
        for list in &frontier {
            for &p in &base {
                let mut l: Vec<(i64, i64)> = list.clone();
                l.push(p);
                next.push(l);
            }
        }
        lists.extend(next.iter().cloned());
        frontier = next;
    }
    for pairs in &lists {
        let (source, reduced) = reduce(pairs);
        let target = reduced.target_problem();
        let m = pairs.len();
        assert_eq!(target.num_vars(), 1 + 3 * m);
        assert_eq!(target.num_constraints(), 3 * m);
        assert_eq!(target.num_nonzeros(), 7 * m);
        let expected = source_has_witness(&source);
        assert_eq!(target_has_point(target, m), expected, "{pairs:?}");
        for x in (0..source.lcm_moduli()).filter(|x| source.evaluate(x).unwrap().0) {
            // Lift every witness in one period and extract it back.
            let mut v = vec![x];
            v.extend(pairs.iter().map(|&(_, b)| x / b));
            v.extend(pairs.iter().map(|&(_, b)| x % b));
            v.extend(pairs.iter().map(|&(a, b)| i64::from(x % b > a % b)));
            assert!(is_feasible(target, &v), "{pairs:?} {v:?}");
            assert_eq!(reduced.extract_solution(&v).unwrap(), x);
        }
        count += 1;
    }
    assert_eq!(count, 1 + 6 + 36 + 216 + 1296 + 7776);
}

#[test]
fn test_simultaneousincongruences_to_ilp_closed_loop() {
    for pairs in [
        vec![],
        vec![(1, 1)],
        vec![(2, 2)],
        vec![(2, 2), (1, 2)],
        vec![(2, 2), (1, 2), (1, 3)],
        vec![(2, 2), (1, 3), (2, 3)],
        vec![(2, 2), (1, 3), (2, 5), (3, 7)],
        vec![(4, 4), (1, 4), (2, 4), (3, 4)],
        vec![(1, 4), (1, 6), (5, 6)],
    ] {
        let (source, reduced) = reduce(&pairs);
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
        let expected = BruteForce::new().solve(&source).unwrap();
        match (expected, ILPSolver::new().solve(&source)) {
            (Some(_), Ok(witness)) => assert!(source.evaluate(&witness).unwrap().0, "{pairs:?}"),
            (None, Err(ILPSolveError::Infeasible)) => {}
            other => panic!("{pairs:?} disagrees: {other:?}"),
        }
    }
}

#[test]
fn raw_target_domains_agree_with_the_source() {
    for pairs in [
        vec![(1, 1)],
        vec![(2, 2)],
        vec![(1, 2)],
        vec![(3, 3)],
        vec![(1, 2), (1, 2)],
        vec![(2, 2), (1, 2)],
        vec![(2, 2), (1, 3)],
        vec![(1, 1), (1, 2)],
        vec![(2, 3), (1, 2)],
    ] {
        let (source, reduced) = reduce(&pairs);
        let target = reduced.target_problem();
        let sizes: Vec<i64> = target
            .variables()
            .iter()
            .map(|v| v.upper_bound().unwrap() + 1)
            .collect();
        let total: i64 = sizes.iter().product();
        let mut feasible = false;
        for mut index in 0..total {
            let mut v = Vec::new();
            for &s in &sizes {
                v.push(index % s);
                index /= s;
            }
            if is_feasible(target, &v) {
                feasible = true;
                assert!(
                    source
                        .evaluate(&reduced.extract_solution(&v).unwrap())
                        .unwrap()
                        .0
                );
            }
        }
        assert_eq!(feasible, source_has_witness(&source), "{pairs:?}");
    }
}

#[test]
fn selector_lemma_for_every_modulus_up_to_two_hundred() {
    for b in 1..=200_i64 {
        for rho in 0..b {
            let a = if rho == 0 { b } else { rho };
            let (_, reduced) = reduce(&[(a, b)]);
            let target = reduced.target_problem();
            for r in 0..b {
                let some_z = (0..=1).any(|z| {
                    let v = [0, 0, r, z];
                    target.constraints()[1..]
                        .iter()
                        .all(|row| row.is_satisfied(&v).unwrap())
                });
                assert_eq!(some_z, r != rho, "b={b} rho={rho} r={r}");
            }
        }
    }
}

#[test]
fn special_cases_empty_modulus_one_and_residue_equal_to_modulus() {
    let (source, reduced) = reduce(&[]);
    let target = reduced.target_problem();
    assert_eq!(
        (
            target.num_vars(),
            target.num_constraints(),
            target.num_nonzeros()
        ),
        (1, 0, 0)
    );
    assert_eq!(target.variables()[0].upper_bound(), Some(0));
    assert_eq!(reduced.extract_solution(&vec![0]).unwrap(), 0);
    assert!(source.evaluate(&0).unwrap().0);

    let (source, reduced) = reduce(&[(1, 1)]);
    let target = reduced.target_problem();
    assert_eq!(
        (
            target.num_vars(),
            target.num_constraints(),
            target.num_nonzeros()
        ),
        (4, 3, 7)
    );
    assert!(!target_has_point(target, 1));
    assert!(!source_has_witness(&source));
    assert_eq!(
        ILPSolver::new().solve(&source),
        Err(ILPSolveError::Infeasible)
    );

    // a = b gives rho = 0: the upper row forces z = 1 and the lower row r >= 1.
    let (_, reduced) = reduce(&[(5, 5)]);
    let target = reduced.target_problem();
    assert!(is_feasible(target, &[1, 0, 1, 1]));
    assert!(!is_feasible(target, &[0, 0, 0, 0]));
    assert!(!is_feasible(target, &[0, 0, 0, 1]));
}

#[test]
fn infeasible_instance_has_no_target_point_and_rejects_every_assignment() {
    let (source, reduced) = reduce(&[(2, 2), (1, 2), (1, 3)]);
    let target = reduced.target_problem();
    assert!(!source_has_witness(&source));
    assert!(!target_has_point(target, 3));
    assert_eq!(
        ILPSolver::new().solve(&source),
        Err(ILPSolveError::Infeasible)
    );
    for x in 0..6_i64 {
        let v = vec![x, x / 2, x / 2, x / 3, x % 2, x % 2, x % 3, 0, 0, 0];
        assert!(!is_feasible(target, &v));
        assert!(reduced.extract_solution(&v).is_err());
    }
}

#[test]
fn extraction_rejects_non_solutions_and_wrong_lengths() {
    let (_, reduced) = reduce(&[(2, 2), (1, 3), (2, 3)]);
    let good = vec![3, 1, 1, 1, 1, 0, 0, 1, 0, 0];
    assert_eq!(reduced.extract_solution(&good).unwrap(), 3);
    let mut wrong_selector = good.clone();
    wrong_selector[7] = 0;
    let mut wrong_division = good.clone();
    wrong_division[1] = 0;
    let mut out_of_bounds = good.clone();
    out_of_bounds[0] = 9;
    let mut negative = good.clone();
    negative[4] = -1;
    for invalid in [
        wrong_selector,
        wrong_division,
        out_of_bounds,
        negative,
        vec![],
        good[..9].to_vec(),
        [good.clone(), vec![0]].concat(),
        vec![0; 10],
    ] {
        assert!(reduced.extract_solution(&invalid).is_err(), "{invalid:?}");
    }
}

#[test]
fn largest_accepted_period_builds_exact_bounds_without_overflow() {
    // 2^63 - 1 = 7^2 * 73 * 127 * 337 * 92737 * 649657.
    let b1: i64 = 49 * 73 * 127 * 337;
    let b2: i64 = 92737 * 649657;
    assert_eq!(b1.checked_mul(b2), Some(i64::MAX));
    let (source, reduced) = reduce(&[(3, b1), (b2, b2)]);
    let target = reduced.target_problem();
    assert_eq!(source.lcm_moduli(), i64::MAX);
    assert_eq!(target.variables()[0].upper_bound(), Some(i64::MAX - 1));
    assert_eq!(target.variables()[1].upper_bound(), Some(b2 - 1));
    assert_eq!(target.variables()[2].upper_bound(), Some(b1 - 1));
    assert_eq!(target.variables()[3].upper_bound(), Some(b1 - 1));
    assert_eq!(target.variables()[4].upper_bound(), Some(b2 - 1));
    assert_eq!(target.max_constraint_magnitude_bits(), 63);
    let rows = target.constraints();
    assert_eq!(rows[0].terms(), &[(0, 1), (1, -b1), (3, -1)]);
    assert_eq!(rows[1].rhs(), 2);
    assert_eq!(rows[2].rhs(), 3 - b1 + 1);
    assert_eq!(rows[4].rhs(), -1);
    assert_eq!(rows[5].rhs(), -b2 + 1);
    // The extreme in-domain vector keeps every division-row value inside i64.
    let top = vec![i64::MAX - 1, 0, 0, b1 - 1, b2 - 1, 0, 0];
    assert!(rows[0].is_satisfied(&top).is_ok());
    let low = vec![0, b2 - 1, 0, b1 - 1, 0, 0, 0];
    assert!(rows[0].is_satisfied(&low).is_ok());
    crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);

    // One step past the boundary is not a source instance at all.
    assert!(SimultaneousIncongruences::new(vec![(1, b1), (1, b2 + 1)]).is_err());

    // Modulus 2^63 - 1 alone: the widest row coefficients.
    let (source, reduced) = reduce(&[(i64::MAX, i64::MAX)]);
    let target = reduced.target_problem();
    assert_eq!(target.variables()[1].upper_bound(), Some(0));
    assert_eq!(target.variables()[2].upper_bound(), Some(i64::MAX - 1));
    assert_eq!(target.constraints()[2].rhs(), -(i64::MAX - 1));
    assert_eq!(target.max_constraint_magnitude_bits(), 63);
    assert_eq!(source.lcm_moduli(), i64::MAX);
}

#[test]
fn magnitude_bits_stay_within_the_period_bit_length() {
    for pairs in [
        vec![(1, 1)],
        vec![(2, 2)],
        vec![(1, 4), (1, 6)],
        vec![(1, 1 << 40), (3, 1 << 20)],
        vec![(5, (1 << 62) - 1)],
        vec![(1, 1 << 62)],
    ] {
        let (source, reduced) = reduce(&pairs);
        assert!(
            reduced.target_problem().max_constraint_magnitude_bits() <= source.lcm_bits(),
            "{pairs:?}"
        );
    }
}
