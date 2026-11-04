use super::*;
use crate::rules::AggregateReductionResult;
use crate::rules::test_helpers::{
    assert_parameter_predictions, assert_satisfaction_round_trip_from_satisfaction_target,
};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::{Min, Or};

fn example(s: usize) -> ComparativeContainment<One> {
    ComparativeContainment::new(
        4,
        vec![vec![0, 1], vec![1, 2], vec![1, 3]],
        vec![vec![0, 1, 2, 3]; s],
    )
    .unwrap()
}

#[test]
fn test_comparativecontainment_to_qubo_closed_loop() {
    for s in [2, 4] {
        let source = example(s);
        let reduction = ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&source).unwrap();
        if s == 2 {
            assert_satisfaction_round_trip_from_satisfaction_target(
                &source,
                &reduction,
                "containment",
            );
        }
        assert_parameter_predictions(&source, &reduction);
        assert_eq!(reduction.extract_value(Or(s == 2)), Or(s == 2));
        assert_eq!(BruteForce::new().solve(&source).unwrap().is_some(), s == 2);
        assert_eq!(
            BruteForce::new()
                .solve(ReductionResult::target_problem(&reduction))
                .unwrap()
                .is_some(),
            s == 2
        );
    }
}

#[test]
fn test_canonical_polynomial_and_rejected_candidates() {
    let reduction = ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&example(2)).unwrap();
    let target = ReductionResult::target_problem(&reduction);
    assert_eq!(target.inner().num_vars(), 7);
    assert_eq!(*target.bound(), -14);
    assert_eq!(target.inner().num_quadratic_terms(), 9);
    assert_eq!(
        target.inner().entries(),
        &[
            (0, 0, -8),
            (0, 2, 4),
            (0, 3, 4),
            (0, 5, 8),
            (0, 6, 8),
            (2, 2, -8),
            (2, 3, 4),
            (2, 4, 8),
            (2, 6, 8),
            (3, 3, -8),
            (3, 4, 8),
            (3, 5, 8),
            (4, 4, -5),
            (5, 5, -5),
            (6, 6, -5),
        ]
    );
    let witness = vec![false, true, false, false, true, true, true];
    assert_eq!(target.inner().evaluate(&witness).unwrap(), Min(Some(-15)));
    assert_eq!(reduction.extract_solution(&witness).unwrap(), witness[..4]);
    let mut faulty = witness.clone();
    faulty[4] = false;
    assert_eq!(target.inner().evaluate(&faulty).unwrap(), Min(Some(-10)));
    assert!(reduction.extract_solution(&faulty).is_err());
    let candidate = vec![true, true, false, false, true, false, false];
    assert_eq!(target.inner().evaluate(&candidate).unwrap(), Min(Some(-13)));
    assert!(reduction.extract_solution(&candidate).is_err());
    for bad in [vec![], vec![false; 6], vec![false; 8]] {
        assert!(reduction.extract_solution(&bad).is_err());
    }
    let no = ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&example(4)).unwrap();
    assert_eq!(*ReductionResult::target_problem(&no).bound(), -16);
    assert!(no.extract_solution(&witness).is_err());
}

fn bits(mask: usize, n: usize) -> Vec<bool> {
    (0..n).map(|i| mask & (1 << i) != 0).collect()
}

fn check_instance(source: ComparativeContainment<One>) {
    let n = source.universe_size();
    let reduction = ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&source).unwrap();
    let target = ReductionResult::target_problem(&reduction);
    let g: usize = source
        .r_sets()
        .iter()
        .chain(source.s_sets())
        .map(|a| {
            let k = (0..n).filter(|v| !a.contains(v)).count();
            k.saturating_sub(1)
        })
        .sum();
    assert_eq!(target.inner().num_vars(), n + g);
    assert!(target.inner().num_quadratic_terms() <= 3 * g);
    assert!(target.inner().entries().len() <= 6 * g + source.num_r_sets() + source.num_s_sets());
    let mut source_yes = false;
    for mask in 0..(1 << n) {
        let x = bits(mask, n);
        let mut extension = x.clone();
        for a in source.r_sets().iter().chain(source.s_sets()) {
            let outside: Vec<_> = (0..n).filter(|v| !a.contains(v)).collect();
            if outside.len() >= 2 {
                let mut prefix = !x[outside[0]];
                for &v in &outside[1..] {
                    prefix &= !x[v];
                    extension.push(prefix);
                }
            }
        }
        // Source sums are evaluated by the model, independently of QUBO gates.
        let valid = source.evaluate(&x).unwrap().0;
        source_yes |= valid;
        let difference =
            source.s_weight_sum(&x).unwrap().unwrap() - source.r_weight_sum(&x).unwrap().unwrap();
        assert_eq!(
            target.inner().evaluate(&extension).unwrap().unwrap() - target.bound(),
            difference
        );
        assert_eq!(target.evaluate(&extension).unwrap().0, valid);
    }
    let mut target_yes = false;
    for mask in 0..(1 << target.inner().num_vars()) {
        let y = bits(mask, target.inner().num_vars());
        if target.evaluate(&y).unwrap().0 {
            target_yes = true;
            let x = reduction.extract_solution(&y).unwrap();
            assert_eq!(x, y[..n]);
            assert!(source.evaluate(&x).unwrap().0);
        } else {
            assert!(reduction.extract_solution(&y).is_err());
        }
    }
    assert_eq!(source_yes, target_yes);
}

#[test]
fn test_exhaustive_two_element_family_pairs() {
    for n in 0..=2 {
        let sets: Vec<Vec<_>> = (0..1 << n)
            .map(|mask| (0..n).filter(|v| mask & (1 << v) != 0).collect())
            .collect();
        let mut families = vec![vec![]];
        for a in &sets {
            families.push(vec![a.clone()]);
            for b in &sets {
                families.push(vec![a.clone(), b.clone()]);
            }
        }
        for r in &families {
            for s in &families {
                check_instance(ComparativeContainment::new(n, r.clone(), s.clone()).unwrap());
            }
        }
    }
}

fn check_all_two_entries(n: usize) {
    let sets: Vec<Vec<_>> = (0..1 << n)
        .map(|mask| (0..n).filter(|v| mask & (1 << v) != 0).collect())
        .collect();
    for r in 0..=2 {
        for a in &sets {
            for b in &sets {
                let entries = [a.clone(), b.clone()];
                check_instance(
                    ComparativeContainment::new(n, entries[..r].to_vec(), entries[r..].to_vec())
                        .unwrap(),
                );
            }
        }
    }
}

#[test]
fn test_exhaustive_three_element_two_entry_families() {
    check_all_two_entries(3);
}
#[test]
fn test_exhaustive_four_element_two_entry_families() {
    check_all_two_entries(4);
}
#[test]
fn test_exhaustive_five_element_two_entry_families() {
    check_all_two_entries(5);
}

#[test]
fn test_empty_full_duplicate_and_cancelled_sets() {
    for n in 0..=5 {
        let full: Vec<_> = (0..n).collect();
        for (r, s) in [
            (vec![], vec![]),
            (vec![], vec![full.clone()]),
            (vec![vec![]], vec![]),
            (vec![full.clone()], vec![]),
            (vec![full.clone()], vec![full.clone()]),
            (vec![vec![], vec![]], vec![full.clone()]),
            (vec![full.clone(); 2], vec![full.clone(); 3]),
        ] {
            let source = ComparativeContainment::<One>::new(n, r, s).unwrap();
            let reduction = ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&source).unwrap();
            assert_parameter_predictions(&source, &reduction);
            check_instance(source);
        }
    }
    let a = ComparativeContainment::<One>::new(3, vec![vec![0, 2]], vec![vec![1]]).unwrap();
    let b =
        ComparativeContainment::<One>::new(3, vec![vec![2, 0, 2, 0]], vec![vec![1, 1]]).unwrap();
    let ta = ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&a).unwrap();
    let tb = ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&b).unwrap();
    assert_eq!(
        serde_json::to_value(ReductionResult::target_problem(&ta)).unwrap(),
        serde_json::to_value(ReductionResult::target_problem(&tb)).unwrap()
    );
    check_instance(b);
    let source = ComparativeContainment::<One>::new(3, vec![vec![0, 1]], vec![vec![0, 1]]).unwrap();
    let reduced = ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&source).unwrap();
    let target = ReductionResult::target_problem(&reduced);
    assert_eq!(target.inner().num_vars(), 3);
    assert!(target.inner().entries().is_empty());
    assert_eq!(*target.bound(), 0);
}

#[test]
fn test_numeric_certificate_and_index_boundaries() {
    let limit = i64::MAX;
    for (r, s) in [(0, 0), (1, 0), (2, 3)] {
        let g = usize::try_from((limit - r - s) / (8 * (r + 1))).unwrap();
        let (nvars, penalty) = checked_sizes(
            0,
            usize::try_from(r).unwrap(),
            usize::try_from(s).unwrap(),
            g,
            limit,
        )
        .unwrap();
        assert_eq!((nvars, penalty), (g, r + 1));
        assert!(matches!(
            checked_sizes(
                0,
                usize::try_from(r).unwrap(),
                usize::try_from(s).unwrap(),
                g + 1,
                limit
            ),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
    let max = usize::try_from(limit).unwrap();
    assert_eq!(checked_sizes(0, max - 1, 1, 0, limit).unwrap(), (0, limit));
    assert_eq!(checked_sizes(0, 0, max, 0, limit).unwrap(), (0, 1));
    for counts in [
        (0, max, 0, 0),
        (0, max + 1, 0, 0),
        (0, max - 1, 2, 0),
        (0, 0, max + 1, 0),
        (usize::MAX, 0, 0, 1),
        (0, usize::MAX, 1, 0),
        (0, 0, 0, usize::MAX),
    ] {
        let (n, r, s, g) = counts;
        assert!(matches!(
            checked_sizes(n, r, s, g, limit),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
    assert_eq!(
        checked_sizes(usize::MAX - 1, 0, 0, 1, limit).unwrap(),
        (usize::MAX, 1)
    );
    // A large penalty with zero gates must not compute 8*M first.
    assert!(checked_sizes(0, max - 1, 0, 0, limit).is_ok());
}

#[test]
fn test_constructed_target_at_coefficient_limit() {
    let empty = ComparativeContainment::<One>::new(0, vec![], vec![]).unwrap();
    let zero = construct(&empty, 1).unwrap();
    assert_eq!(
        ReductionResult::target_problem(&zero)
            .evaluate(&vec![])
            .unwrap(),
        Or(true)
    );
    assert!(matches!(
        construct(&empty, 0),
        Err(ReductionError::IntegerOverflow { .. })
    ));

    let mut r = vec![vec![0, 1]; 9];
    r.push(vec![]);
    let source = ComparativeContainment::<One>::new(2, r, vec![]).unwrap();
    // r=10, M=11, g=1: 8*M*g+r+s=98, while an actual coefficient is 22.
    let reduction = construct(&source, 98).unwrap();
    let target = ReductionResult::target_problem(&reduction);
    assert_eq!(*target.bound(), -2);
    assert_eq!(
        target.inner().entries(),
        &[
            (0, 0, -11),
            (0, 1, 11),
            (0, 2, 22),
            (1, 1, -11),
            (1, 2, 22),
            (2, 2, -12),
        ]
    );
    assert!(matches!(
        construct(&source, 97),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    for mask in 0..8 {
        let y = bits(mask, 3);
        assert_eq!(
            target.evaluate(&y).unwrap(),
            ReductionResult::target_problem(
                &ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&source).unwrap()
            )
            .evaluate(&y)
            .unwrap()
        );
    }
}

#[test]
fn test_sparse_dimension_and_gate_count_overflow() {
    let empty = ComparativeContainment::<One>::new(usize::MAX, vec![], vec![]).unwrap();
    let reduced = ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&empty).unwrap();
    assert_eq!(
        ReductionResult::target_problem(&reduced).inner().num_vars(),
        usize::MAX
    );
    assert!(
        ReductionResult::target_problem(&reduced)
            .inner()
            .entries()
            .is_empty()
    );
    for r in [vec![vec![]], vec![vec![], vec![]]] {
        let source = ComparativeContainment::<One>::new(usize::MAX, r, vec![]).unwrap();
        assert!(matches!(
            ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&source),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
}
