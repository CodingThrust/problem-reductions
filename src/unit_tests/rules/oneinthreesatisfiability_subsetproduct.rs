use super::*;
use crate::models::formula::CNFClause;
use crate::rules::ReductionError;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::traits::Problem;
use crate::types::Or;
use num_bigint::BigUint;

fn source(num_vars: usize, clauses: &[[i64; 3]]) -> OneInThreeSatisfiability {
    OneInThreeSatisfiability::new(
        num_vars,
        clauses
            .iter()
            .map(|clause| CNFClause::new(clause.to_vec()))
            .collect(),
    )
}

fn reduce(source: &OneInThreeSatisfiability) -> ReductionOneInThreeSatisfiabilityToSubsetProduct {
    ReduceTo::<SubsetProduct>::reduce_to(source).expect("reduction should succeed")
}

fn sizes_of(target: &SubsetProduct) -> Vec<u64> {
    target
        .sizes()
        .iter()
        .map(|size| u64::try_from(size).unwrap())
        .collect()
}

fn assignments(num_vars: usize) -> impl Iterator<Item = Vec<bool>> {
    (0..1usize << num_vars).map(move |mask| (0..num_vars).map(|i| mask >> i & 1 == 1).collect())
}

/// Checks the whole target-witness set against the source assignments.
fn assert_equivalent(source: &OneInThreeSatisfiability) {
    let reduction = reduce(source);
    let target = reduction.target_problem();
    assert_eq!(target.num_elements(), 2 * source.num_vars());
    let mut witnesses = 0usize;
    for selection in assignments(target.num_elements()) {
        let target_value = target.evaluate(&selection).unwrap().0;
        if target_value {
            witnesses += 1;
            let extracted = reduction.extract_solution(&selection).unwrap();
            assert!(source.evaluate(&extracted).unwrap().0);
            for (i, &value) in extracted.iter().enumerate() {
                assert_ne!(selection[2 * i], selection[2 * i + 1]);
                assert_eq!(selection[2 * i + 1], value);
            }
        } else {
            assert!(reduction.extract_solution(&selection).is_err());
        }
    }
    let satisfying = assignments(source.num_vars())
        .filter(|assignment| source.evaluate(assignment).unwrap().0)
        .count();
    assert_eq!(witnesses, satisfying);
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_yes_example() {
    let source = source(3, &[[1, 2, 3], [-1, -2, 3], [1, -2, -3]]);
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    assert_eq!(sizes_of(target), vec![22, 182, 429, 21, 65, 385]);
    assert_eq!(target.target(), &BigUint::from(30030u64));
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "OneInThreeSatisfiability -> SubsetProduct closed loop",
    );

    let witness = vec![true, false, false, true, true, false];
    assert!(target.evaluate(&witness).unwrap().0);
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![false, true, false]
    );
    // It is the only witness.
    let count = assignments(6)
        .filter(|selection| target.evaluate(selection).unwrap().0)
        .count();
    assert_eq!(count, 1);
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_no_example() {
    let source = source(3, &[[1, 2, 3], [-1, -2, -3]]);
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    assert_eq!(sizes_of(target), vec![22, 14, 33, 21, 55, 35]);
    assert_eq!(target.target(), &BigUint::from(2310u64));
    assert!(assignments(3).all(|a| !source.evaluate(&a).unwrap().0));
    assert!(assignments(6).all(|s| !target.evaluate(&s).unwrap().0));
    assert_equivalent(&source);
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_all_single_clauses() {
    // Every ordered clause over three variables: repeats and both signs included.
    let literals = [-3i64, -2, -1, 1, 2, 3];
    for a in literals {
        for b in literals {
            for c in literals {
                assert_equivalent(&source(3, &[[a, b, c]]));
            }
        }
    }
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_all_clause_pairs() {
    let literals = [-2i64, -1, 1, 2];
    let mut clauses: Vec<[i64; 3]> = Vec::new();
    for a in literals {
        for b in literals {
            for c in literals {
                clauses.push([a, b, c]);
            }
        }
    }
    for first in &clauses {
        for second in &clauses {
            assert_equivalent(&source(2, &[*first, *second]));
        }
    }
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_five_variables() {
    assert_equivalent(&source(
        5,
        &[[1, 2, 3], [-3, 4, 5], [1, -4, -5], [2, -2, 5]],
    ));
    assert_equivalent(&source(5, &[[1, 1, 2], [3, 4, 5], [-1, -2, 5]]));
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_special_cases() {
    // Repeated literal: exponent 3 on the true item, 0 on the false item.
    let triple = reduce(&source(1, &[[1, 1, 1]]));
    assert_eq!(sizes_of(triple.target_problem()), vec![2, 2 * 27]);
    assert_equivalent(&source(1, &[[1, 1, 1]]));
    assert_equivalent(&source(1, &[[-1, -1, -1]]));
    let double = reduce(&source(2, &[[1, 1, 2]]));
    assert_eq!(sizes_of(double.target_problem()), vec![2, 2 * 25, 3, 3 * 5]);

    // Both signs of one variable in one clause: equal-valued items stay two indices.
    let both = reduce(&source(2, &[[1, -1, 2]]));
    assert_eq!(sizes_of(both.target_problem()), vec![10, 10, 3, 15]);
    assert_equivalent(&source(2, &[[1, -1, 2]]));

    // Unused variables and no clauses: two equal items per variable, 2^n witnesses.
    let unused = reduce(&source(3, &[]));
    assert_eq!(sizes_of(unused.target_problem()), vec![2, 2, 3, 3, 5, 5]);
    assert_eq!(unused.target_problem().target(), &BigUint::from(30u64));
    assert_equivalent(&source(3, &[]));
    assert_equivalent(&source(4, &[[1, 2, 2]]));

    // Empty instance: no items, target 1, the empty selection is the witness.
    let empty = reduce(&source(0, &[]));
    assert_eq!(empty.target_problem().num_elements(), 0);
    assert_eq!(empty.target_problem().target(), &BigUint::from(1u64));
    assert_eq!(empty.extract_solution(&vec![]).unwrap(), Vec::<bool>::new());
    assert_equivalent(&source(0, &[]));
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_uses_first_primes_exactly() {
    // 6 variables and 6 clauses: the first twelve primes, each with exponent one in the target.
    let clauses: Vec<[i64; 3]> = (1..=6).map(|j| [j, -((j % 6) + 1), 1]).collect();
    let reduction = reduce(&source(6, &clauses));
    let primes: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23 * 29 * 31 * 37;
    assert_eq!(reduction.target_problem().target(), &BigUint::from(primes));
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_large_numbers_exceed_u64() {
    // 20 variables, 30 clauses (target B has 50 prime factors, far beyond 64 bits), planted assignment: in every clause the first literal is true
    // under it and the other two are false, so it is satisfying by construction.
    let planted: Vec<bool> = (0..20).map(|i| i % 3 != 0).collect();
    let signed = |variable: usize, want_true: bool| {
        let literal = variable as i64 + 1;
        if planted[variable] == want_true {
            literal
        } else {
            -literal
        }
    };
    let clauses: Vec<[i64; 3]> = (0..30)
        .map(|j| {
            let (a, b, c) = (j % 20, (j + 7) % 20, (j + 13) % 20);
            [signed(a, true), signed(b, false), signed(c, false)]
        })
        .collect();
    let source = source(20, &clauses);
    // The three variables of a clause are distinct, so the planted assignment satisfies it.
    assert!(source.evaluate(&planted).unwrap().0);

    let reduction = reduce(&source);
    let target = reduction.target_problem();
    assert_eq!(target.num_elements(), 40);
    assert!(target.target().bits() > 64);

    let selection: Vec<bool> = planted.iter().flat_map(|&v| [!v, v]).collect();
    assert!(target.evaluate(&selection).unwrap().0);
    assert_eq!(reduction.extract_solution(&selection).unwrap(), planted);
    // Flipping one variable breaks the product.
    let mut flipped = selection.clone();
    flipped.swap(0, 1);
    assert!(!target.evaluate(&flipped).unwrap().0);
    assert!(reduction.extract_solution(&flipped).is_err());
    // Products are exact: selecting every item overshoots the target.
    assert!(!target.evaluate(&vec![true; 40]).unwrap().0);
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_extract_rejects_invalid_witnesses() {
    let source = source(3, &[[1, 2, 3], [-1, -2, 3], [1, -2, -3]]);
    let reduction = reduce(&source);
    // A non-solution of the right length.
    assert!(reduction.extract_solution(&vec![false; 6]).is_err());
    // Both items of a pair selected.
    assert!(
        reduction
            .extract_solution(&vec![true, true, false, true, true, false])
            .is_err()
    );
    // Wrong lengths.
    assert!(reduction.extract_solution(&vec![]).is_err());
    assert!(reduction.extract_solution(&vec![true; 5]).is_err());
    assert!(reduction.extract_solution(&vec![true; 7]).is_err());
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_identity_value_mapping() {
    let yes = source(3, &[[1, 2, 3], [-1, -2, 3], [1, -2, -3]]);
    let no = source(3, &[[1, 2, 3], [-1, -2, -3]]);
    for (source, feasible) in [(yes, true), (no, false)] {
        let reduction = reduce(&source);
        let target_feasible = crate::solvers::BruteForce::new()
            .solve(reduction.target_problem())
            .unwrap()
            .is_some();
        let source_value =
            crate::rules::AggregateReductionResult::extract_value(&reduction, Or(target_feasible));
        assert_eq!(source_value, Or(feasible));
    }
}

#[test]
fn test_oneinthreesatisfiability_to_subsetproduct_item_count_boundary() {
    // The source model caps variables at i64::MAX, so the doubling can only fail on narrower
    // targets; the helper is checked at the boundary on both sides.
    assert_eq!(item_count(0).unwrap(), 0);
    assert_eq!(item_count(7).unwrap(), 14);
    assert_eq!(item_count(usize::MAX / 2).unwrap(), usize::MAX - 1);
    let error = item_count(usize::MAX / 2 + 1).unwrap_err();
    assert!(matches!(error, ReductionError::IntegerOverflow { .. }));
    assert!(error.to_string().contains("doubling"));
    assert!(matches!(
        item_count(usize::MAX),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}
