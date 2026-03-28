use super::*;
use crate::models::misc::StaffScheduling;
use crate::models::set::ExactCoverBy3Sets;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_exactcoverby3sets_to_staffscheduling_closed_loop() {
    // Universe {0,1,2,3,4,5}, subsets [{0,1,2}, {3,4,5}, {0,3,4}, {1,2,5}]
    // Exact cover: S0={0,1,2} + S1={3,4,5}
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4], [1, 2, 5]]);
    let result = ReduceTo::<StaffScheduling>::reduce_to(&source);
    let target = result.target_problem();

    // Check target dimensions
    assert_eq!(target.num_periods(), 6);
    assert_eq!(target.shifts_per_schedule(), 3);
    assert_eq!(target.num_schedules(), 4);
    assert_eq!(target.num_workers(), 2); // q = 6/3 = 2

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "ExactCoverBy3Sets->StaffScheduling closed loop",
    );
}

#[test]
fn test_exactcoverby3sets_to_staffscheduling_no_solution() {
    // Universe {0,1,2,3,4,5} with overlapping subsets that cannot form exact cover
    // All subsets share element 0
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [0, 3, 4], [0, 4, 5]]);
    let result = ReduceTo::<StaffScheduling>::reduce_to(&source);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(result.target_problem());
    assert!(
        solutions.is_empty(),
        "No exact cover exists, so StaffScheduling should have no solution"
    );
}

#[test]
fn test_exactcoverby3sets_to_staffscheduling_unique_cover() {
    // Universe {0,1,2,3,4,5,6,7,8} (q=3)
    // Only one exact cover: S0 + S1 + S2
    let source = ExactCoverBy3Sets::new(9, vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]]);
    let result = ReduceTo::<StaffScheduling>::reduce_to(&source);
    let target = result.target_problem();

    assert_eq!(target.num_periods(), 9);
    assert_eq!(target.num_workers(), 3); // q = 9/3 = 3

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(target);
    // Each satisfying target config should extract to selecting all 3 subsets
    for sol in &solutions {
        let extracted = result.extract_solution(sol);
        assert!(
            source.evaluate(&extracted).0,
            "Extracted solution must be valid"
        );
    }
    // There should be exactly one satisfying assignment (up to extraction)
    let extracted_solutions: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| result.extract_solution(s))
        .collect();
    assert!(
        extracted_solutions.iter().all(|s| *s == vec![1, 1, 1]),
        "Only exact cover is all three subsets"
    );
}

#[test]
fn test_exactcoverby3sets_to_staffscheduling_extract_solution() {
    // Verify extract_solution maps correctly
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4], [1, 2, 5]]);
    let result = ReduceTo::<StaffScheduling>::reduce_to(&source);

    // StaffScheduling config: [1, 1, 0, 0] means 1 worker on schedule 0 and 1 on schedule 1
    let target_config = vec![1, 1, 0, 0];
    let extracted = result.extract_solution(&target_config);
    assert_eq!(extracted, vec![1, 1, 0, 0]);

    // Verify the extracted solution is valid in the source
    assert!(source.evaluate(&extracted).0);

    // Config with 0 workers everywhere should extract to all-zero (no subsets selected)
    let empty_config = vec![0, 0, 0, 0];
    let extracted_empty = result.extract_solution(&empty_config);
    assert_eq!(extracted_empty, vec![0, 0, 0, 0]);
}

#[test]
fn test_exactcoverby3sets_to_staffscheduling_schedule_structure() {
    // Verify the schedule patterns are correctly constructed
    let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5]]);
    let result = ReduceTo::<StaffScheduling>::reduce_to(&source);
    let target = result.target_problem();

    let schedules = target.schedules();
    // Schedule 0 should have shifts at positions 0, 1, 2
    assert_eq!(schedules[0], vec![true, true, true, false, false, false]);
    // Schedule 1 should have shifts at positions 3, 4, 5
    assert_eq!(schedules[1], vec![false, false, false, true, true, true]);

    // Requirements should all be 1
    assert!(target.requirements().iter().all(|&r| r == 1));
}
