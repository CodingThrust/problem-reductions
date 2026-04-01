use super::*;
use crate::models::misc::ThreePartition;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::traits::Problem;
use crate::types::Min;

/// m=1: 3 elements, bound=15, sizes=[4, 5, 6]. Only 1 group, no separators.
/// 3 jobs, 6 tasks. dims = [3,2,1,3,2,1] => 36 configs. Fast for brute force.
#[test]
fn test_threepartition_to_jobshopscheduling_closed_loop() {
    let source = ThreePartition::new(vec![4, 5, 6], 15);
    let reduction = ReduceTo::<JobShopScheduling>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "ThreePartition -> JobShopScheduling closed loop (m=1)",
    );
}

/// Verify the target problem structure for m=1.
#[test]
fn test_threepartition_to_jss_structure_m1() {
    let source = ThreePartition::new(vec![4, 5, 6], 15);
    let reduction = ReduceTo::<JobShopScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    // m=1: 3 element jobs, 0 separator jobs
    assert_eq!(target.num_processors(), 2);
    assert_eq!(target.num_jobs(), 3);
    assert_eq!(target.num_tasks(), 6);

    // Each element job has 2 tasks
    for (i, job) in target.jobs().iter().enumerate() {
        assert_eq!(job.len(), 2, "element job {i} should have 2 tasks");
        assert_eq!(
            job[0].0, 0,
            "element job {i} task 0 should be on processor 0"
        );
        assert_eq!(
            job[1].0, 1,
            "element job {i} task 1 should be on processor 1"
        );
        // Tasks have equal length = source size
        assert_eq!(job[0].1, job[1].1);
    }

    let sizes = source.sizes();
    assert_eq!(target.jobs()[0][0].1, sizes[0]);
    assert_eq!(target.jobs()[1][0].1, sizes[1]);
    assert_eq!(target.jobs()[2][0].1, sizes[2]);
}

/// Verify the target problem structure for m=2.
#[test]
fn test_threepartition_to_jss_structure_m2() {
    // m=2: 6 elements, bound=20, sizes satisfy B/4 < s < B/2
    let source = ThreePartition::new(vec![6, 7, 7, 6, 8, 6], 20);
    let reduction = ReduceTo::<JobShopScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    // m=2: 6 element jobs + 1 separator job = 7 jobs
    assert_eq!(target.num_processors(), 2);
    assert_eq!(target.num_jobs(), 7); // 6 + 2 - 1
    assert_eq!(target.num_tasks(), 13); // 2*6 + 2 - 1

    // Element jobs (0..5): 2 tasks each
    for i in 0..6 {
        let job = &target.jobs()[i];
        assert_eq!(job.len(), 2);
        assert_eq!(job[0].0, 0);
        assert_eq!(job[1].0, 1);
    }

    // Separator job (index 6): 1 task on processor 0
    let separator = &target.jobs()[6];
    assert_eq!(separator.len(), 1);
    assert_eq!(separator[0].0, 0);

    // Separator length L = m*B + 1 = 2*20 + 1 = 41
    assert_eq!(separator[0].1, 41);
}

/// Verify that the threshold is correct for m=2.
#[test]
fn test_threepartition_to_jss_threshold_m2() {
    let source = ThreePartition::new(vec![6, 7, 7, 6, 8, 6], 20);
    let reduction = ReduceTo::<JobShopScheduling>::reduce_to(&source);

    // D = m*B + (m-1)*L = 2*20 + 1*41 = 81
    assert_eq!(reduction.threshold(), 81);
}

/// For m=2, manually construct a valid schedule config and verify extraction.
#[test]
fn test_threepartition_to_jss_extraction_m2() {
    // sizes = [6, 7, 7, 6, 8, 6], bound = 20, m = 2
    // Valid partition: group 0 = {7, 7, 6} (indices 1,2,3), group 1 = {6, 8, 6} (indices 0,4,5)
    let source = ThreePartition::new(vec![6, 7, 7, 6, 8, 6], 20);
    let reduction = ReduceTo::<JobShopScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    // Machine 0 tasks (local indices 0..6):
    //   local 0 -> element 0 (task id 0)
    //   local 1 -> element 1 (task id 2)
    //   local 2 -> element 2 (task id 4)
    //   local 3 -> element 3 (task id 6)
    //   local 4 -> element 4 (task id 8)
    //   local 5 -> element 5 (task id 10)
    //   local 6 -> separator 0 (task id 12)
    //
    // We want machine 0 order: elem1, elem2, elem3, separator0, elem0, elem4, elem5
    // That's local indices: [1, 2, 3, 6, 0, 4, 5]
    //
    // Lehmer encoding of permutation [1, 2, 3, 6, 0, 4, 5]:
    // available = [0,1,2,3,4,5,6]
    // pick 1 from [0,1,2,3,4,5,6] -> index 1, remaining [0,2,3,4,5,6]
    // pick 2 from [0,2,3,4,5,6] -> index 1, remaining [0,3,4,5,6]
    // pick 3 from [0,3,4,5,6] -> index 1, remaining [0,4,5,6]
    // pick 6 from [0,4,5,6] -> index 3, remaining [0,4,5]
    // pick 0 from [0,4,5] -> index 0, remaining [4,5]
    // pick 4 from [4,5] -> index 0, remaining [5]
    // pick 5 from [5] -> index 0
    let machine0_lehmer = vec![1, 1, 1, 3, 0, 0, 0];

    // Machine 1 tasks (local indices 0..5):
    //   local 0 -> element 0 (task id 1)
    //   local 1 -> element 1 (task id 3)
    //   local 2 -> element 2 (task id 5)
    //   local 3 -> element 3 (task id 7)
    //   local 4 -> element 4 (task id 9)
    //   local 5 -> element 5 (task id 11)
    //
    // Any valid ordering; use identity: [0,1,2,3,4,5] => Lehmer [0,0,0,0,0,0]
    let machine1_lehmer = vec![0, 0, 0, 0, 0, 0];

    let mut config = machine0_lehmer;
    config.extend(machine1_lehmer);

    // Verify the schedule produces a valid makespan
    let value = target.evaluate(&config);
    assert!(value.0.is_some(), "config should produce a valid schedule");

    // Extract and verify source solution
    let source_config = reduction.extract_solution(&config);
    assert_eq!(source_config.len(), 6);

    // Elements 1,2,3 should be in group 0 (before separator)
    // Elements 0,4,5 should be in group 1 (after separator)
    assert_eq!(source_config[1], 0); // element 1 in group 0
    assert_eq!(source_config[2], 0); // element 2 in group 0
    assert_eq!(source_config[3], 0); // element 3 in group 0
    assert_eq!(source_config[0], 1); // element 0 in group 1
    assert_eq!(source_config[4], 1); // element 4 in group 1
    assert_eq!(source_config[5], 1); // element 5 in group 1

    // Verify the extracted solution is a valid 3-partition
    let source_value = source.evaluate(&source_config);
    assert!(
        source_value.0,
        "extracted solution should be a valid 3-partition"
    );
}

/// For m=1, verify that optimal makespan equals the sum of all sizes.
#[test]
fn test_threepartition_to_jss_makespan_m1() {
    let source = ThreePartition::new(vec![4, 5, 6], 15);
    let reduction = ReduceTo::<JobShopScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    // With m=1, no separators, threshold = 1*15 + 0 = 15
    assert_eq!(reduction.threshold(), 15);

    // Identity ordering: Lehmer [0,0,0] for machine 0, [0,0,0] for machine 1
    let config = vec![0, 0, 0, 0, 0, 0];
    let value = target.evaluate(&config);

    // Tasks on machine 0: 4, 5, 6 (total 15)
    // Tasks on machine 1: must wait for respective machine 0 tasks
    // Machine 0: [0,4], [4,9], [9,15]
    // Machine 1: [4,8], [9,14], [15,21]
    // Makespan = 21
    assert_eq!(value, Min(Some(21)));
}

/// Verify overhead expressions are correct.
#[test]
fn test_threepartition_to_jss_overhead() {
    let source = ThreePartition::new(vec![4, 5, 6], 15);
    let reduction = ReduceTo::<JobShopScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    // num_jobs = num_elements + num_groups - 1 = 3 + 1 - 1 = 3
    assert_eq!(
        target.num_jobs(),
        source.num_elements() + source.num_groups() - 1
    );
    // num_tasks = 2 * num_elements + num_groups - 1 = 6 + 0 = 6
    assert_eq!(
        target.num_tasks(),
        2 * source.num_elements() + source.num_groups() - 1
    );

    // Also check for m=2
    let source2 = ThreePartition::new(vec![6, 7, 7, 6, 8, 6], 20);
    let reduction2 = ReduceTo::<JobShopScheduling>::reduce_to(&source2);
    let target2 = reduction2.target_problem();

    assert_eq!(
        target2.num_jobs(),
        source2.num_elements() + source2.num_groups() - 1
    );
    assert_eq!(
        target2.num_tasks(),
        2 * source2.num_elements() + source2.num_groups() - 1
    );
}
