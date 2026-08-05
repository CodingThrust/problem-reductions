use super::*;
use crate::models::algebraic::MinimumWeightDecoding;
use crate::models::set::ThreeDimensionalMatching;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

fn reduce_tdm(
    universe_size: usize,
    triples: Vec<(usize, usize, usize)>,
) -> (
    ThreeDimensionalMatching,
    ReductionThreeDimensionalMatchingToMinimumWeightDecoding,
) {
    let source = ThreeDimensionalMatching::new(universe_size, triples);
    let reduction = ReduceTo::<MinimumWeightDecoding>::reduce_to(&source);
    (source, reduction)
}

#[test]
fn test_threedimensionalmatching_to_minimumweightdecoding_closed_loop() {
    // YES case: q = 2, two perfect matchings ({t_0, t_1} and {t_2, t_3}).
    let (source, reduction) = reduce_tdm(2, vec![(0, 0, 0), (1, 1, 1), (0, 1, 0), (1, 0, 1)]);
    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "3DM -> MinimumWeightDecoding closed loop (YES, two perfect matchings)",
    );

    // NO case: q = 2, element 1 of Y is never covered.
    // Source is genuinely infeasible; target is also infeasible (Min(None)).
    let (source_no, reduction_no) = reduce_tdm(2, vec![(0, 0, 0), (0, 0, 1), (1, 0, 0)]);
    let target_no = reduction_no.target_problem();
    let solver = BruteForce::new();
    // Confirm the target is infeasible.
    assert!(solver.find_witness(target_no).is_none());
    // Confirm the source is infeasible.
    assert!(solver.find_witness(&source_no).is_none());

    // YES case: q = 3, exactly one perfect matching among five triples.
    let (source_y, reduction_y) = reduce_tdm(
        3,
        vec![(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)],
    );
    assert_satisfaction_round_trip_from_optimization_target(
        &source_y,
        &reduction_y,
        "3DM -> MinimumWeightDecoding closed loop (YES, q = 3)",
    );
}

#[test]
fn test_threedimensionalmatching_to_minimumweightdecoding_structure() {
    // Main branch: q = 2, m = 4 → matrix is 6 × 4, syndrome is all-ones of length 6.
    let (_source, reduction) = reduce_tdm(2, vec![(0, 0, 0), (1, 1, 1), (0, 1, 0), (1, 0, 1)]);
    let target = reduction.target_problem();
    assert_eq!(target.num_rows(), 6);
    assert_eq!(target.num_cols(), 4);
    assert_eq!(target.target(), &[true; 6]);

    // Verify the column pattern matches the worked example:
    //        t0 t1 t2 t3
    //  X=0:   1  0  1  0
    //  X=1:   0  1  0  1
    //  Y=0:   1  0  0  1
    //  Y=1:   0  1  1  0
    //  Z=0:   1  0  1  0
    //  Z=1:   0  1  0  1
    let expected: Vec<Vec<bool>> = vec![
        vec![true, false, true, false],
        vec![false, true, false, true],
        vec![true, false, false, true],
        vec![false, true, true, false],
        vec![true, false, true, false],
        vec![false, true, false, true],
    ];
    assert_eq!(target.matrix(), expected.as_slice());
}

#[test]
fn test_threedimensionalmatching_to_minimumweightdecoding_optimal_value_yes() {
    // YES case: optimum should be Min(Some(q)) = Min(Some(2)).
    let (_source, reduction) = reduce_tdm(2, vec![(0, 0, 0), (1, 1, 1), (0, 1, 0), (1, 0, 1)]);
    let target = reduction.target_problem();
    let solver = BruteForce::new();
    assert_eq!(solver.solve(target), Min(Some(2)));
}

#[test]
fn test_threedimensionalmatching_to_minimumweightdecoding_optimal_value_no() {
    // NO case: target is infeasible → Min(None).
    let (_source, reduction) = reduce_tdm(2, vec![(0, 0, 0), (0, 0, 1), (1, 0, 0)]);
    let target = reduction.target_problem();
    let solver = BruteForce::new();
    assert_eq!(solver.solve(target), Min(None));
}

#[test]
fn test_threedimensionalmatching_to_minimumweightdecoding_sentinel_q_zero() {
    // q = 0, T = []: sentinel target, extracted S = ∅, source.evaluate(∅) = Or(true).
    let (source, reduction) = reduce_tdm(0, vec![]);
    let target = reduction.target_problem();
    assert_eq!(target.num_rows(), 1);
    assert_eq!(target.num_cols(), 1);
    assert_eq!(target.target(), &[false]);

    let solver = BruteForce::new();
    let target_witnesses = solver.find_all_witnesses(target);
    assert!(!target_witnesses.is_empty());
    for witness in &target_witnesses {
        // Sentinel codeword is the all-zero vector of length 1.
        assert_eq!(witness, &vec![0]);
        let extracted = reduction.extract_solution(witness).unwrap();
        // Source has 0 triples → extracted vector has length 0.
        assert_eq!(extracted.len(), source.num_triples());
        assert_eq!(extracted, Vec::<usize>::new());
        // Empty matching of empty universe is valid.
        assert!(source.evaluate(&extracted).0);
    }
}

#[test]
fn test_threedimensionalmatching_to_minimumweightdecoding_sentinel_no_triples() {
    // q >= 1, T = []: sentinel target, extracted S = ∅, source.evaluate(∅) = Or(false).
    for q in [1, 2, 3] {
        let (source, reduction) = reduce_tdm(q, vec![]);
        let target = reduction.target_problem();
        assert_eq!(target.num_rows(), 1);
        assert_eq!(target.num_cols(), 1);

        let solver = BruteForce::new();
        let target_witnesses = solver.find_all_witnesses(target);
        assert!(!target_witnesses.is_empty());
        for witness in &target_witnesses {
            let extracted = reduction.extract_solution(witness).unwrap();
            assert_eq!(extracted.len(), source.num_triples());
            // Empty triple set cannot cover non-empty universe.
            assert!(
                !source.evaluate(&extracted).0,
                "q = {q}, T = []: empty matching must be NO"
            );
        }
        // Direct solve confirms the source is NO.
        assert!(solver.find_witness(&source).is_none());
    }
}

#[test]
fn test_threedimensionalmatching_to_minimumweightdecoding_solution_extraction_identity() {
    // For a YES instance the extracted solution must be a valid source witness.
    let (source, reduction) = reduce_tdm(2, vec![(0, 0, 0), (1, 1, 1), (0, 1, 0), (1, 0, 1)]);
    let target = reduction.target_problem();
    let solver = BruteForce::new();
    let target_witnesses = solver.find_all_witnesses(target);
    let source_witnesses: std::collections::HashSet<Vec<usize>> =
        solver.find_all_witnesses(&source).into_iter().collect();

    assert!(!target_witnesses.is_empty());
    for witness in &target_witnesses {
        let extracted = reduction.extract_solution(witness).unwrap();
        assert_eq!(extracted, *witness);
        assert!(
            source_witnesses.contains(&extracted),
            "extracted witness {extracted:?} must be a valid 3DM solution"
        );
    }
}
