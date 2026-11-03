use super::*;
use crate::rules::test_helpers::{
    assert_optimization_round_trip_from_optimization_target, assert_parameter_predictions,
};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn reduce(source: &MinimumHittingSet) -> ReductionHSToSC {
    ReduceTo::<MinimumSetCovering<i64>>::reduce_to(source).expect("reduction should succeed")
}

fn all_configs(len: usize) -> Vec<Vec<bool>> {
    (0..1usize << len)
        .map(|bits| (0..len).map(|i| bits >> i & 1 == 1).collect())
        .collect()
}

/// Every configuration has the same value on both sides (feasibility and cardinality).
fn assert_values_agree(source: &MinimumHittingSet) {
    let reduction = reduce(source);
    let target = reduction.target_problem();
    assert_eq!(target.universe_size(), source.num_sets());
    assert_eq!(target.num_sets(), source.universe_size());
    assert!(target.weights_ref().iter().all(|&w| w == 1));
    for config in all_configs(source.universe_size()) {
        assert_eq!(
            source.evaluate(&config).unwrap(),
            target.evaluate(&config).unwrap(),
            "{config:?}"
        );
    }
    assert_parameter_predictions(source, &reduction);
}

#[test]
fn test_minimumhittingset_to_minimumsetcovering_closed_loop() {
    let source = MinimumHittingSet::new(5, vec![vec![0, 1], vec![1, 2, 3], vec![0, 3], vec![2, 4]]);
    let reduction = reduce(&source);
    let target = reduction.target_problem();
    assert_eq!(target.universe_size(), 4);
    assert_eq!(
        target.sets(),
        &[vec![0, 2], vec![0, 1], vec![1, 3], vec![1, 2], vec![3]]
    );
    assert_values_agree(&source);
    // {0, 2} is the unique optimum; no singleton is feasible.
    let witness = vec![true, false, true, false, false];
    assert_eq!(target.evaluate(&witness).unwrap(), Min(Some(2)));
    assert_eq!(reduction.extract_solution(&witness).unwrap(), witness);
    for i in 0..5 {
        let mut single = vec![false; 5];
        single[i] = true;
        assert_eq!(target.evaluate(&single).unwrap(), Min(None));
    }
    let optima = BruteForce::new().find_all_witnesses(target).unwrap();
    assert_eq!(optima, vec![witness]);
    assert_optimization_round_trip_from_optimization_target(&source, &reduction, "HS->SC");
}

#[test]
fn test_minimumhittingset_to_minimumsetcovering_exhaustive_small() {
    // Every ordered family of up to three subsets of a universe of size 0..=3.
    for n in 0..=3usize {
        let subsets: Vec<Vec<usize>> = (0..1usize << n)
            .map(|mask| (0..n).filter(|e| mask >> e & 1 == 1).collect())
            .collect();
        for m in 0..=3usize {
            let mut index = vec![0usize; m];
            loop {
                let family: Vec<Vec<usize>> = index.iter().map(|&i| subsets[i].clone()).collect();
                let source = MinimumHittingSet::new(n, family);
                assert_values_agree(&source);
                // Feasible exactly when no member is empty; the round-trip helper needs a witness.
                if source.sets().iter().all(|set| !set.is_empty()) {
                    let reduction = reduce(&source);
                    assert_optimization_round_trip_from_optimization_target(
                        &source,
                        &reduction,
                        "HS->SC exhaustive",
                    );
                }
                // advance the odometer
                let mut pos = 0;
                while pos < m {
                    index[pos] += 1;
                    if index[pos] < subsets.len() {
                        break;
                    }
                    index[pos] = 0;
                    pos += 1;
                }
                if pos == m {
                    break;
                }
            }
        }
    }
}

#[test]
fn test_minimumhittingset_to_minimumsetcovering_empty_family() {
    // No sets to hit: the target universe is empty, every element gives an empty set.
    let source = MinimumHittingSet::new(3, vec![]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().universe_size(), 0);
    assert_eq!(
        reduction.target_problem().sets(),
        &vec![Vec::<usize>::new(); 3]
    );
    assert_values_agree(&source);
    let (value, witnesses) = BruteForce::new()
        .solve_with_witnesses(reduction.target_problem())
        .unwrap();
    assert_eq!(value, Min(Some(0)));
    assert_eq!(witnesses, vec![vec![false; 3]]);
    // Empty universe and empty family.
    let source = MinimumHittingSet::new(0, vec![]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().num_sets(), 0);
    assert_eq!(
        reduction.target_problem().evaluate(&vec![]).unwrap(),
        Min(Some(0))
    );
    assert_eq!(
        reduction.extract_solution(&vec![]).unwrap(),
        Vec::<bool>::new()
    );
}

#[test]
fn test_minimumhittingset_to_minimumsetcovering_empty_member_is_infeasible() {
    let source = MinimumHittingSet::new(2, vec![vec![], vec![0]]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().sets(), &[vec![1], vec![]]);
    assert_values_agree(&source);
    for config in all_configs(2) {
        assert_eq!(source.evaluate(&config).unwrap(), Min(None));
        assert_eq!(
            reduction.target_problem().evaluate(&config).unwrap(),
            Min(None)
        );
        // An infeasible target selection is rejected, not extracted.
        assert!(reduction.extract_solution(&config).is_err());
    }
    // Zero-element universe with one empty member.
    let source = MinimumHittingSet::new(0, vec![vec![]]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().universe_size(), 1);
    assert_eq!(reduction.target_problem().num_sets(), 0);
    assert_values_agree(&source);
    assert!(reduction.extract_solution(&vec![]).is_err());
    let (value, witnesses) = BruteForce::new()
        .solve_with_witnesses(reduction.target_problem())
        .unwrap();
    assert_eq!(value, Min(None));
    assert!(witnesses.is_empty());
}

#[test]
fn test_minimumhittingset_to_minimumsetcovering_repeated_sets_and_unused_elements() {
    // Members 0 and 1 repeat; element 3 occurs nowhere.
    let source = MinimumHittingSet::new(4, vec![vec![0, 1], vec![1, 0], vec![2]]);
    let reduction = reduce(&source);
    assert_eq!(
        reduction.target_problem().sets(),
        &[vec![0, 1], vec![0, 1], vec![2], vec![]]
    );
    assert_values_agree(&source);
    assert_optimization_round_trip_from_optimization_target(&source, &reduction, "repeated");
    // Duplicated elements inside a member are normalised by the source model.
    let source = MinimumHittingSet::new(3, vec![vec![0, 0, 1, 1], vec![2, 2]]);
    assert_eq!(
        reduce(&source).target_problem().sets(),
        &[vec![0], vec![0], vec![1]]
    );
}

#[test]
fn test_minimumhittingset_to_minimumsetcovering_extraction_rejects_bad_witnesses() {
    let source = MinimumHittingSet::new(3, vec![vec![0, 1], vec![2]]);
    let reduction = reduce(&source);
    // Wrong lengths.
    assert!(reduction.extract_solution(&vec![true, true]).is_err());
    assert!(
        reduction
            .extract_solution(&vec![true, true, true, true])
            .is_err()
    );
    assert!(reduction.extract_solution(&vec![]).is_err());
    // Non-cover.
    assert!(
        reduction
            .extract_solution(&vec![true, true, false])
            .is_err()
    );
    assert!(reduction.extract_solution(&vec![false; 3]).is_err());
    // Non-minimal covers are accepted and mapped identically.
    assert_eq!(
        reduction.extract_solution(&vec![true, true, true]).unwrap(),
        vec![true, true, true]
    );
}

#[test]
fn test_minimumhittingset_to_minimumsetcovering_numeric_boundary() {
    // The objective of a selection is a unit-weight sum bounded by the universe size,
    // which must fit i64. On a 64-bit target a larger universe is rejected with a typed
    // error before anything is allocated.
    if usize::BITS == 64 {
        let too_large = MinimumHittingSet::new(i64::MAX as usize + 1, vec![]);
        let error = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&too_large)
            .expect_err("universe beyond i64 must be rejected");
        assert!(matches!(
            error,
            crate::rules::ReductionError::IntegerOverflow { .. }
        ));
        let largest = MinimumHittingSet::new(usize::MAX, vec![]);
        assert!(ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&largest).is_err());
    }
    // Values just below the limit are accepted by the check itself.
    assert_eq!(
        <MinimumHittingSet as ReduceTo<MinimumSetCovering<i64>>>::exact_i64(
            i64::MAX as usize,
            "boundary"
        )
        .unwrap(),
        i64::MAX
    );
    // Target evaluation of unit weights is the cardinality, with a typed error on overflow.
    let target =
        MinimumSetCovering::<i64>::with_weights(0, vec![vec![], vec![]], vec![i64::MAX, 1]);
    assert!(target.evaluate(&vec![true, true]).is_err());
    assert_eq!(
        target.evaluate(&vec![true, false]).unwrap(),
        Min(Some(i64::MAX))
    );
}
