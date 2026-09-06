use super::*;
use crate::models::algebraic::ILP;
use crate::models::misc::{OpenShopScheduling, Partition};
use crate::solvers::ILPSolver;
use crate::traits::Problem;

fn solve_target(target: &OpenShopScheduling) -> Vec<usize> {
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(target).expect("ILP reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("open-shop target should be feasible");
    reduction.extract_solution(&ilp_solution).unwrap()
}

#[test]
fn test_partition_to_open_shop_scheduling_closed_loop() {
    let source = Partition::new(vec![1, 2, 3]).unwrap();
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap();
    let target_solution = solve_target(reduction.target_problem());
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert!(source.evaluate(&extracted).unwrap());
}

#[test]
fn test_partition_to_open_shop_scheduling_structure() {
    let source = Partition::new(vec![1, 2, 3]).unwrap();
    let reduction =
        ReduceTo::<OpenShopScheduling>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_jobs(), 4);
    assert_eq!(target.num_machines(), 3);
    assert_eq!(
        target.processing_times(),
        &[vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![3, 3, 3]]
    );
}

#[test]
fn test_partition_to_open_shop_scheduling_extract_solution() {
    let source = Partition::new(vec![1, 2, 3]).unwrap();
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap();
    let target_solution = solve_target(reduction.target_problem());
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted.len(), 3);
    assert!(source.evaluate(&extracted).unwrap());
}

#[test]
fn test_partition_to_open_shop_scheduling_odd_total_is_not_satisfying() {
    let source = Partition::new(vec![2, 4, 5]).unwrap();
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap();
    let best = solve_target(reduction.target_problem());
    assert!(reduction.extract_solution(&best).is_err());
}

#[test]
fn test_partition_to_open_shop_scheduling_preserves_construction_overflow() {
    let source = Partition::new(vec![1_i64 << 61, 1_i64 << 61]).unwrap();
    let error = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap_err();
    assert!(matches!(
        error,
        crate::rules::ReductionError::Construction {
            source_problem: "Partition",
            target_problem: "OpenShopScheduling",
            cause: crate::registry::ConstructionError::IntegerOverflow(_),
        }
    ));
}

#[test]
fn test_partition_to_open_shop_all_small_partitions_and_machine_orders() {
    use crate::rules::AggregateReductionResult;
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for n in 1..=4 {
        for mut rank in 0..3usize.pow(n as u32) {
            let sizes: Vec<_> = (0..n)
                .map(|_| {
                    let size = (rank % 3 + 1) as i64;
                    rank /= 3;
                    size
                })
                .collect();
            let source = Partition::new(sizes.clone()).unwrap();
            let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap();
            let target = AggregateReductionResult::target_problem(&reduction);
            assert!(
                !AggregateReductionResult::extract_value(&reduction, crate::types::Min(None)).0
            );
            for mask in 0..(1usize << n) {
                let assignment: Vec<_> = (0..n).map(|i| mask & (1 << i) != 0).collect();
                if !source.evaluate(&assignment).unwrap().0 {
                    continue;
                }
                let half = usize::try_from(source.total_sum() / 2).unwrap();
                for machines in permutations {
                    let mut schedule = vec![0; (n + 1) * 3];
                    for phase in 0..3 {
                        schedule[n * 3 + machines[phase]] = phase * half;
                        for (group, rotation) in [(true, 1), (false, 2)] {
                            let machine = machines[(phase + rotation) % 3];
                            let mut time = phase * half;
                            for job in 0..n {
                                if assignment[job] == group {
                                    schedule[job * 3 + machine] = time;
                                    time += usize::try_from(sizes[job]).unwrap();
                                }
                            }
                            assert_eq!(time, (phase + 1) * half);
                        }
                    }
                    let value = target.evaluate(&schedule).unwrap();
                    assert_eq!(value, crate::types::Min(Some(3 * half as i64)));
                    assert!(AggregateReductionResult::extract_value(&reduction, value).0);
                    assert_eq!(reduction.extract_solution(&schedule).unwrap(), assignment);
                    let delayed: Vec<_> = schedule.iter().map(|&time| time + 1).collect();
                    assert!(target.evaluate(&delayed).unwrap().0.is_some());
                    assert!(reduction.extract_solution(&delayed).is_err());
                }
            }
            assert!(reduction.extract_solution(&vec![0; (n + 1) * 3]).is_err());
            assert!(reduction
                .extract_solution(&vec![0; (n + 1) * 3 + 1])
                .is_err());
        }
    }
}

#[test]
fn test_partition_to_open_shop_odd_singleton_certificate() {
    use crate::rules::AggregateReductionResult;
    let source = Partition::new(vec![1]).unwrap();
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap();
    let schedule = vec![0, 1, 2, 0, 0, 0];
    let value = ReductionResult::target_problem(&reduction)
        .evaluate(&schedule)
        .unwrap();
    assert_eq!(value, crate::types::Min(Some(3)));
    assert!(!AggregateReductionResult::extract_value(&reduction, value).0);
    assert!(reduction.extract_solution(&schedule).is_err());
}

#[test]
#[cfg(target_pointer_width = "64")]
fn test_partition_to_open_shop_certificate_near_horizon_limit() {
    let size = i64::MAX / 9;
    let source = Partition::new(vec![size, size]).unwrap();
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap();
    let a = usize::try_from(size).unwrap();
    let schedule = vec![0, a, 2 * a, 2 * a, 0, a, a, 2 * a, 0];
    assert_eq!(
        reduction.target_problem().evaluate(&schedule).unwrap(),
        crate::types::Min(Some(3 * size))
    );
    assert!(
        source
            .evaluate(&reduction.extract_solution(&schedule).unwrap())
            .unwrap()
            .0
    );
}
