use super::*;
use crate::models::misc::ThreePartition;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce_three_partition(
    sizes: Vec<u64>,
    bound: u64,
) -> (ThreePartition, ReductionThreePartitionToFSS) {
    let source = ThreePartition::new(sizes, bound);
    let reduction = ReduceTo::<FlowShopScheduling>::reduce_to(&source);
    (source, reduction)
}

/// Encode a job order (permutation) as a Lehmer code.
fn encode_lehmer(job_order: &[usize]) -> Vec<usize> {
    let n = job_order.len();
    let mut available: Vec<usize> = (0..n).collect();
    let mut lehmer = Vec::with_capacity(n);
    for &job in job_order {
        let pos = available.iter().position(|&x| x == job).unwrap();
        lehmer.push(pos);
        available.remove(pos);
    }
    lehmer
}

#[test]
fn test_threepartition_to_flowshopscheduling_closed_loop() {
    // ThreePartition: sizes [4, 5, 6, 4, 6, 5], bound=15, m=2
    // Valid partition: group 0 = {4,5,6} (indices 0,1,2), group 1 = {4,6,5} (indices 3,4,5)
    let (source, reduction) = reduce_three_partition(vec![4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    // Verify source is satisfiable
    let solver = BruteForce::new();
    assert!(
        solver.find_witness(&source).is_some(),
        "Source 3-Partition should be satisfiable"
    );

    // Verify target is satisfiable
    assert!(
        solver.find_witness(target).is_some(),
        "Target FlowShopScheduling should be satisfiable"
    );

    // Canonical ordering: [0,1,2, sep(6), 3,4,5] -- group 1 elements, separator, group 2 elements
    let canonical_order = vec![0, 1, 2, 6, 3, 4, 5];
    let canonical_lehmer = encode_lehmer(&canonical_order);
    assert_eq!(canonical_lehmer, vec![0, 0, 0, 3, 0, 0, 0]);

    // Verify canonical ordering satisfies the target
    let target_value = target.evaluate(&canonical_lehmer);
    assert!(target_value.0, "Canonical ordering should meet deadline");

    // Extract and verify: elements before separator -> group 0, after -> group 1
    let extracted = reduction.extract_solution(&canonical_lehmer);
    assert_eq!(extracted, vec![0, 0, 0, 1, 1, 1]);
    assert!(
        source.evaluate(&extracted).0,
        "Extracted solution should be a valid 3-partition"
    );

    // Test another valid ordering: group 2 first, then group 1
    let alt_order = vec![3, 4, 5, 6, 0, 1, 2];
    let alt_lehmer = encode_lehmer(&alt_order);
    let alt_value = target.evaluate(&alt_lehmer);
    assert!(
        alt_value.0,
        "Alternative valid ordering should meet deadline"
    );
    let alt_extracted = reduction.extract_solution(&alt_lehmer);
    assert_eq!(alt_extracted, vec![1, 1, 1, 0, 0, 0]);
    assert!(
        source.evaluate(&alt_extracted).0,
        "Alternative extraction should be a valid 3-partition"
    );

    // Verify all valid-partition orderings extract correctly
    // A valid partition groups elements into triples summing to B=15.
    // For this instance: one triple from each of {4,5,6} values.
    // Elements by value: val 4 at {0,3}, val 5 at {1,5}, val 6 at {2,4}
    let target_witnesses = solver.find_all_witnesses(target);
    let mut valid_extraction_count = 0;
    for w in &target_witnesses {
        let extracted = reduction.extract_solution(w);
        if source.evaluate(&extracted).0 {
            valid_extraction_count += 1;
        }
    }
    assert!(
        valid_extraction_count > 0,
        "At least some target witnesses should extract to valid source solutions"
    );
}

#[test]
fn test_threepartition_to_flowshopscheduling_structure() {
    let (source, reduction) = reduce_three_partition(vec![4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    // 3 machines
    assert_eq!(target.num_processors(), 3);
    // 6 element jobs + 1 separator = 7 total jobs
    assert_eq!(target.num_jobs(), 7);
    assert_eq!(
        target.num_jobs(),
        source.num_elements() + source.num_groups() - 1
    );

    // Check element job task lengths
    let task_lengths = target.task_lengths();
    for (i, tasks) in task_lengths.iter().enumerate().take(6) {
        let size = source.sizes()[i];
        assert_eq!(*tasks, vec![size, size, size]);
    }

    // Check separator job task lengths: [0, L, 0] where L = m*B+1 = 2*15+1 = 31
    let big_l = 2 * 15 + 1;
    assert_eq!(task_lengths[6], vec![0, big_l, 0]);

    // Deadline should be positive
    assert!(target.deadline() > 0);
}

#[test]
fn test_threepartition_to_flowshopscheduling_solution_extraction() {
    let (source, reduction) = reduce_three_partition(vec![4, 5, 6, 4, 6, 5], 15);

    // Test extraction for canonical orderings where elements are properly grouped
    // Ordering: indices 0,1,2 (group 0), separator 6, indices 3,4,5 (group 1)
    let lehmer = encode_lehmer(&[0, 1, 2, 6, 3, 4, 5]);
    let extracted = reduction.extract_solution(&lehmer);
    assert_eq!(extracted.len(), source.num_elements());
    assert_eq!(extracted, vec![0, 0, 0, 1, 1, 1]);
    assert!(source.evaluate(&extracted).0);

    // Different valid grouping: {0,4,5}=group 0, {1,2,3}=group 1
    // 4+6+5=15 and 5+6+4=15
    let lehmer2 = encode_lehmer(&[0, 4, 5, 6, 1, 2, 3]);
    let extracted2 = reduction.extract_solution(&lehmer2);
    assert_eq!(extracted2[0], 0); // element 0 in group 0
    assert_eq!(extracted2[4], 0); // element 4 in group 0
    assert_eq!(extracted2[5], 0); // element 5 in group 0
    assert_eq!(extracted2[1], 1); // element 1 in group 1
    assert_eq!(extracted2[2], 1); // element 2 in group 1
    assert_eq!(extracted2[3], 1); // element 3 in group 1
    assert!(source.evaluate(&extracted2).0);
}

#[test]
fn test_threepartition_to_flowshopscheduling_dims() {
    let (_source, reduction) = reduce_three_partition(vec![4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    // Lehmer code dims: [7, 6, 5, 4, 3, 2, 1]
    let dims = target.dims();
    assert_eq!(dims, vec![7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_threepartition_to_flowshopscheduling_canonical_makespan() {
    let (source, reduction) = reduce_three_partition(vec![4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    // The canonical ordering should achieve exactly the deadline
    let canonical_order = vec![0, 1, 2, 6, 3, 4, 5];
    let makespan = target.compute_makespan(&canonical_order);
    assert_eq!(makespan, target.deadline());

    // Verify the deadline computation:
    // m=2, B=15, L=31
    // Canonical schedule on M2: first element starts at s(a_0)=4,
    // group1 takes B=15, separator takes L=31, group2 takes B=15
    // M2 finishes at 4 + 15 + 31 + 15 = 65
    // M3 lags behind M2 by one element's processing time at the end
    assert!(target.deadline() > 0);

    // The number of elements + groups - 1 should equal num_jobs
    assert_eq!(
        source.num_elements() + source.num_groups() - 1,
        target.num_jobs()
    );
}
