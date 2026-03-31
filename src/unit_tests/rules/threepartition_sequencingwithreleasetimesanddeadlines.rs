use super::*;
use crate::models::misc::{SequencingWithReleaseTimesAndDeadlines, ThreePartition};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce(sizes: Vec<u64>, bound: u64) -> (ThreePartition, ReductionThreePartitionToSRTD) {
    let source = ThreePartition::new(sizes, bound);
    let reduction = ReduceTo::<SequencingWithReleaseTimesAndDeadlines>::reduce_to(&source);
    (source, reduction)
}

#[test]
fn test_threepartition_to_sequencingwithreleasetimesanddeadlines_closed_loop() {
    // sizes=[4,5,6,4,6,5], bound=15, m=2
    // Valid partition: {4,5,6} and {4,6,5}
    let (source, reduction) = reduce(vec![4, 5, 6, 4, 6, 5], 15);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ThreePartition -> SequencingWithReleaseTimesAndDeadlines closed loop",
    );
}

#[test]
fn test_threepartition_to_sequencingwithreleasetimesanddeadlines_structure() {
    let (source, reduction) = reduce(vec![4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    // 3m = 6 element tasks + m-1 = 1 filler task = 7 total
    assert_eq!(target.num_tasks(), 7);
    assert_eq!(source.num_elements() + source.num_groups() - 1, 7);

    // Element tasks: lengths match source sizes
    let lengths = target.lengths();
    assert_eq!(&lengths[..6], &[4, 5, 6, 4, 6, 5]);
    // Filler task has length 1
    assert_eq!(lengths[6], 1);

    // Element tasks: release = 0, deadline = m*(B+1)-1 = 2*16-1 = 31
    for i in 0..6 {
        assert_eq!(target.release_times()[i], 0);
        assert_eq!(target.deadlines()[i], 31);
    }

    // Filler task: release = 1*15+0 = 15, deadline = 16
    assert_eq!(target.release_times()[6], 15);
    assert_eq!(target.deadlines()[6], 16);

    // Time horizon
    assert_eq!(target.time_horizon(), 31);
}

#[test]
fn test_threepartition_to_sequencingwithreleasetimesanddeadlines_satisfiability() {
    let (source, reduction) = reduce(vec![4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    // Source is satisfiable
    assert!(solver.find_witness(&source).is_some());
    // Target should also be satisfiable
    assert!(solver.find_witness(target).is_some());
}

#[test]
fn test_threepartition_to_sequencingwithreleasetimesanddeadlines_solution_extraction() {
    let (source, reduction) = reduce(vec![4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(target);

    for sol in &target_solutions {
        let extracted = reduction.extract_solution(sol);
        assert_eq!(extracted.len(), source.num_elements());
        let source_valid = source.evaluate(&extracted);
        assert!(
            source_valid.0,
            "Valid schedule should yield valid 3-partition"
        );
    }
}

#[test]
fn test_threepartition_to_sequencingwithreleasetimesanddeadlines_dims() {
    let (_, reduction) = reduce(vec![4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    // 7 tasks -> Lehmer dims [7,6,5,4,3,2,1]
    let dims = target.dims();
    assert_eq!(dims, vec![7, 6, 5, 4, 3, 2, 1]);
}
