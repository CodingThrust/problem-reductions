use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

/// 2 machines, 2 jobs: smallest non-trivial instance.
/// processing_times[j][i]: J1=[1,2], J2=[2,1]
/// All four orderings give the same makespan = 3 (symmetric).
fn two_by_two() -> OpenShopScheduling {
    OpenShopScheduling::new(2, vec![vec![1, 2], vec![2, 1]])
}

/// 3 machines, 3 jobs: a small asymmetric instance.
fn three_by_three() -> OpenShopScheduling {
    OpenShopScheduling::new(3, vec![vec![1, 2, 3], vec![3, 2, 1], vec![2, 1, 2]])
}

/// Issue #506 example: 4 jobs × 3 machines, true optimal makespan = 8.
/// (The issue body incorrectly claimed 11 was optimal; brute-force confirms 8.)
fn issue_example() -> OpenShopScheduling {
    OpenShopScheduling::new(
        3,
        vec![vec![3, 1, 2], vec![2, 3, 1], vec![1, 2, 3], vec![2, 2, 1]],
    )
}

// ─── creation and dims ───────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_creation() {
    let p = issue_example();
    assert_eq!(p.num_machines(), 3);
    assert_eq!(p.num_jobs(), 4);
    assert_eq!(
        p.processing_times(),
        &[
            vec![3usize, 1, 2],
            vec![2, 3, 1],
            vec![1, 2, 3],
            vec![2, 2, 1],
        ]
    );
}

#[test]
fn test_open_shop_scheduling_dims() {
    let p = issue_example();
    // n = 4 jobs, m = 3 machines → n*m = 12 config variables, each in 0..4
    assert_eq!(p.dims(), vec![4usize; 12]);

    let p2 = two_by_two();
    assert_eq!(p2.dims(), vec![2usize; 4]);
}

// ─── evaluate ────────────────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_evaluate_issue_example_optimal() {
    let p = issue_example();
    // Optimal config: M1=[0,1,2,3], M2=[1,0,3,2], M3=[2,3,0,1]
    // True optimal makespan = 8 (the issue body incorrectly claimed 11).
    let config = vec![0, 1, 2, 3, 1, 0, 3, 2, 2, 3, 0, 1];
    assert_eq!(p.evaluate(&config), Min(Some(8)));
}

#[test]
fn test_open_shop_scheduling_evaluate_issue_example_suboptimal_schedule() {
    let p = issue_example();
    // The schedule from the issue body: M1=[2,1,0,3], M2=[2,1,0,3], M3=[2,0,1,3]
    // gives makespan 11, which is valid but not optimal (optimal is 8).
    let config = vec![2, 1, 0, 3, 2, 1, 0, 3, 2, 0, 1, 3];
    let value = p.evaluate(&config);
    assert_eq!(value, Min(Some(11)));
}

#[test]
fn test_open_shop_scheduling_evaluate_suboptimal() {
    let p = issue_example();
    // Identity orderings on all machines: M1=[0,1,2,3], M2=[0,1,2,3], M3=[0,1,2,3]
    let config = vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];
    let value = p.evaluate(&config);
    // Must be valid and > 8 (non-optimal)
    assert!(value.0.is_some());
    assert!(value.0.unwrap() > 8);
}

#[test]
fn test_open_shop_scheduling_evaluate_invalid_not_permutation() {
    let p = issue_example();
    // config[0..4] = [0,0,0,0] is not a permutation → invalid
    let config = vec![0, 0, 0, 0, 0, 1, 2, 3, 0, 1, 2, 3];
    assert_eq!(p.evaluate(&config), Min(None));
}

#[test]
fn test_open_shop_scheduling_evaluate_wrong_length() {
    let p = issue_example();
    // Too short
    assert_eq!(p.evaluate(&[0, 1, 2]), Min(None));
    // Too long
    assert_eq!(p.evaluate(&[0; 13]), Min(None));
}

#[test]
fn test_open_shop_scheduling_evaluate_empty() {
    let p = OpenShopScheduling::new(3, vec![]);
    assert_eq!(p.dims(), Vec::<usize>::new());
    assert_eq!(p.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_open_shop_scheduling_evaluate_two_by_two() {
    let p = two_by_two();
    // M1=[0,1], M2=[0,1]: valid permutations
    // Simulation:
    //   Step 1: best start is min over M1(J1: max(0,0)=0) and M2(J1: max(0,0)=0)
    //           → machine 0 (tie-break), schedule J1 on M1: [0,1), machine_avail[0]=1, job_avail[0]=1
    //   Step 2: M1 next is J2 (start=max(1,0)=1), M2 next is J1 (start=max(0,1)=1)
    //           → machine 0 (tie-break), schedule J2 on M1: [1,3), machine_avail[0]=3, job_avail[1]=3
    //   Step 3: M1 done, M2 next is J1 (start=max(0,1)=1), schedule J1 on M2: [1,3), machine_avail[1]=3, job_avail[0]=3
    //   Step 4: M2 next is J2 (start=max(3,3)=3), schedule J2 on M2: [3,4), machine_avail[1]=4, job_avail[1]=4
    // Makespan = 4
    let config = vec![0, 1, 0, 1];
    let val = p.evaluate(&config);
    assert!(val.0.is_some());
    assert_eq!(val, Min(Some(4)));
}

// ─── decode_orders ───────────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_decode_orders_valid() {
    let p = two_by_two();
    let config = vec![0, 1, 1, 0];
    let orders = p.decode_orders(&config).unwrap();
    assert_eq!(orders, vec![vec![0, 1], vec![1, 0]]);
}

#[test]
fn test_open_shop_scheduling_decode_orders_invalid_duplicate() {
    let p = two_by_two();
    let config = vec![0, 0, 1, 0]; // first machine has duplicate 0
    assert!(p.decode_orders(&config).is_none());
}

#[test]
fn test_open_shop_scheduling_decode_orders_invalid_out_of_range() {
    let p = two_by_two();
    let config = vec![0, 2, 1, 0]; // job 2 out of range for n=2
    assert!(p.decode_orders(&config).is_none());
}

// ─── compute_makespan ────────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_compute_makespan_optimal_schedule() {
    let p = issue_example();
    // True optimal: M1=[0,1,2,3], M2=[1,0,3,2], M3=[2,3,0,1], makespan=8
    let orders = vec![
        vec![0, 1, 2, 3], // M1
        vec![1, 0, 3, 2], // M2
        vec![2, 3, 0, 1], // M3
    ];
    assert_eq!(p.compute_makespan(&orders), 8);
}

#[test]
fn test_open_shop_scheduling_compute_makespan_issue_example_schedule() {
    let p = issue_example();
    // The schedule from the issue body: makespan=11 (valid but suboptimal)
    let orders = vec![vec![2, 1, 0, 3], vec![2, 1, 0, 3], vec![2, 0, 1, 3]];
    // Manually verified start/finish times:
    // J1: M1=[3,6), M2=[6,7), M3=[7,9)
    // J2: M1=[1,3), M2=[3,6), M3=[9,10)
    // J3: M1=[0,1), M2=[1,3), M3=[3,6)
    // J4: M1=[6,8), M2=[8,10), M3=[10,11)
    assert_eq!(p.compute_makespan(&orders), 11);
}

// ─── problem trait ───────────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_problem_name_and_variant() {
    assert_eq!(<OpenShopScheduling as Problem>::NAME, "OpenShopScheduling");
    assert!(<OpenShopScheduling as Problem>::variant().is_empty());
}

// ─── serialization ───────────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_serialization() {
    let p = issue_example();
    let json = serde_json::to_value(&p).unwrap();
    let restored: OpenShopScheduling = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_machines(), p.num_machines());
    assert_eq!(restored.num_jobs(), p.num_jobs());
    assert_eq!(restored.processing_times(), p.processing_times());
}

// ─── brute-force solver ──────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_brute_force_small() {
    // 2x2 instance: brute force over 2^4 = 16 configs (4 valid schedules)
    let p = two_by_two();
    let solver = BruteForce::new();
    let value = Solver::solve(&solver, &p);
    assert!(value.0.is_some());
    // Optimal value for this instance
    assert_eq!(value, Min(Some(3)));
    let witness = solver.find_witness(&p).unwrap();
    assert_eq!(p.evaluate(&witness), Min(Some(3)));
}

#[test]
fn test_open_shop_scheduling_brute_force_medium() {
    // 3x3 instance: brute force over 3^9 = 19683 configs (216 valid schedules)
    let p = three_by_three();
    let solver = BruteForce::new();
    let value = Solver::solve(&solver, &p);
    assert!(value.0.is_some());
    let witness = solver.find_witness(&p).unwrap();
    assert_eq!(p.evaluate(&witness), value);
}

#[test]
fn test_open_shop_scheduling_canonical_example_config_is_optimal() {
    // Verify that the canonical example config achieves the true optimal makespan = 8
    let p = issue_example();
    let optimal_config = vec![0, 1, 2, 3, 1, 0, 3, 2, 2, 3, 0, 1];
    assert_eq!(p.evaluate(&optimal_config), Min(Some(8)));
}
