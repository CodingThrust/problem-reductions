use super::*;
use crate::models::misc::{SequencingToMinimizeWeightedTardiness, ThreePartition};
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce(sizes: Vec<u64>, bound: u64) -> (ThreePartition, ReductionThreePartitionToSMWT) {
    let source = ThreePartition::new(sizes, bound);
    let reduction = ReduceTo::<SequencingToMinimizeWeightedTardiness>::reduce_to(&source);
    (source, reduction)
}

fn assert_satisfiability_matches(
    source: &ThreePartition,
    target: &SequencingToMinimizeWeightedTardiness,
    expected: bool,
) {
    let solver = BruteForce::new();
    assert_eq!(
        solver.find_witness(source).is_some(),
        expected,
        "source satisfiability mismatch"
    );
    assert_eq!(
        solver.find_witness(target).is_some(),
        expected,
        "target satisfiability mismatch"
    );
}

/// Verify the decision-level round trip: source is satisfiable iff target is,
/// and at least one target witness extracts to a valid source witness.
fn assert_decision_round_trip(
    source: &ThreePartition,
    reduction: &ReductionThreePartitionToSMWT,
    context: &str,
) {
    let solver = BruteForce::new();
    let target = reduction.target_problem();
    let source_sat = solver.find_witness(source).is_some();
    let target_witnesses = solver.find_all_witnesses(target);
    let target_sat = !target_witnesses.is_empty();
    assert_eq!(
        source_sat, target_sat,
        "{context}: satisfiability mismatch (source={source_sat}, target={target_sat})"
    );

    if source_sat {
        // At least one target witness must extract to a valid source witness
        let found_valid = target_witnesses.iter().any(|tw| {
            let extracted = reduction.extract_solution(tw);
            source.evaluate(&extracted).0
        });
        assert!(
            found_valid,
            "{context}: no target witness extracted to a valid source solution"
        );
    }
}

#[test]
fn test_threepartition_to_sequencingtominimizeweightedtardiness_closed_loop() {
    // m=2, B=20, sizes with B/4 < s < B/2 (i.e., 5 < s < 10)
    // sizes: [7, 7, 6, 7, 7, 6], sum = 40 = 2*20
    // Valid partition: {7,7,6} and {7,7,6}
    let (source, reduction) = reduce(vec![7, 7, 6, 7, 7, 6], 20);

    assert_decision_round_trip(&source, &reduction, "ThreePartition -> SMWT closed loop");
}

#[test]
fn test_threepartition_to_sequencingtominimizeweightedtardiness_structure() {
    // m=2, B=20, 6 elements + 1 filler = 7 tasks
    let (source, reduction) = reduce(vec![7, 7, 6, 7, 7, 6], 20);
    let target = reduction.target_problem();

    let m = source.num_groups();
    let b = source.bound();

    // Total tasks: 3m + (m-1) = 6 + 1 = 7
    assert_eq!(target.num_tasks(), 7);
    assert_eq!(target.num_tasks(), source.num_elements() + m - 1);

    // Element task lengths match source sizes
    let lengths = target.lengths();
    for (len, &size) in lengths.iter().zip(source.sizes()) {
        assert_eq!(*len, size);
    }

    // Filler task length = 1
    for &len in &lengths[source.num_elements()..] {
        assert_eq!(len, 1);
    }

    // Element task weights = 1
    let weights = target.weights();
    for &w in &weights[..source.num_elements()] {
        assert_eq!(w, 1);
    }

    // Filler task weight = mB + 1
    let filler_weight = (m as u64) * b + 1;
    for &w in &weights[source.num_elements()..] {
        assert_eq!(w, filler_weight);
    }

    // Element task deadlines = mB + (m-1) = horizon
    let horizon = (m as u64) * b + (m as u64 - 1);
    let deadlines = target.deadlines();
    for &d in &deadlines[..source.num_elements()] {
        assert_eq!(d, horizon);
    }

    // Filler deadlines: (j+1)*B + (j+1)
    for (j, &d) in deadlines[source.num_elements()..].iter().enumerate() {
        let expected = ((j + 1) as u64) * b + (j + 1) as u64;
        assert_eq!(d, expected);
    }

    // Bound = 0
    assert_eq!(target.bound(), 0);
}

#[test]
fn test_threepartition_to_sequencingtominimizeweightedtardiness_m1() {
    // m=1, B=20, 3 elements, no fillers
    // sizes: [7, 7, 6], sum = 20 = 1*20
    let (source, reduction) = reduce(vec![7, 7, 6], 20);
    let target = reduction.target_problem();

    // 3 element tasks, 0 filler tasks
    assert_eq!(target.num_tasks(), 3);
    assert_eq!(target.bound(), 0);

    // All deadlines = 1*20 + 0 = 20
    for &d in target.deadlines() {
        assert_eq!(d, 20);
    }

    // m=1: any permutation of 3 tasks should satisfy (sum = B, all fit by deadline)
    assert_decision_round_trip(&source, &reduction, "ThreePartition -> SMWT m=1");
}

#[test]
fn test_threepartition_to_sequencingtominimizeweightedtardiness_solution_extraction() {
    let (source, reduction) = reduce(vec![7, 7, 6, 7, 7, 6], 20);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(target);
    assert!(!target_solutions.is_empty(), "target should be satisfiable");

    // Verify that at least one target solution extracts to a valid source solution
    let mut found_valid = false;
    for sol in &target_solutions {
        let extracted = reduction.extract_solution(sol);
        assert_eq!(extracted.len(), source.num_elements());
        if source.evaluate(&extracted).0 {
            found_valid = true;
        }
    }
    assert!(
        found_valid,
        "at least one extraction must yield a valid 3-partition"
    );
}

#[test]
fn test_threepartition_to_sequencingtominimizeweightedtardiness_satisfiability_match() {
    // Feasible instance: m=2, B=20
    let (source, reduction) = reduce(vec![7, 7, 6, 7, 7, 6], 20);
    assert_satisfiability_matches(&source, reduction.target_problem(), true);

    // m=1: always feasible (3 elements sum to B)
    let (source1, reduction1) = reduce(vec![7, 7, 6], 20);
    assert_satisfiability_matches(&source1, reduction1.target_problem(), true);
}
