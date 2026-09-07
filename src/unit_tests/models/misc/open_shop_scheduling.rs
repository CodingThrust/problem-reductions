use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Min;

/// 2 machines, 2 jobs: smallest non-trivial instance.
/// processing_times[j][i]: J1=[1,2], J2=[2,1]
/// A two-by-two instance with an optimal makespan of 3.
fn two_by_two() -> OpenShopScheduling {
    OpenShopScheduling::new(2, vec![vec![1, 2], vec![2, 1]])
}

#[test]
fn test_open_shop_create_spec_uses_num_processors_input() {
    assert_eq!(
        OpenShopSchedulingCreateSpec::FIELDS[0].name,
        "num_processors"
    );
    let problem = OpenShopScheduling::try_from(OpenShopSchedulingCreateSpec {
        num_processors: 2,
        processing_times: vec![vec![1, 2]],
    })
    .unwrap();
    assert_eq!(problem.num_machines(), 2);
}

/// Two jobs on one machine: a small brute-force instance.
fn small_asymmetric() -> OpenShopScheduling {
    OpenShopScheduling::new(1, vec![vec![2], vec![3]])
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
            vec![3_i64, 1, 2],
            vec![2, 3, 1],
            vec![1, 2, 3],
            vec![2, 2, 1],
        ]
    );
}

#[test]
fn test_open_shop_scheduling_dims() {
    let p = issue_example();
    assert_eq!(p.dimensions(), vec![24usize; 12]);

    let p2 = two_by_two();
    assert_eq!(p2.dimensions(), vec![7usize; 4]);
}

// ─── evaluate ────────────────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_evaluate_issue_example_optimal() {
    let p = issue_example();
    // Job-major start times for a schedule with makespan 8.
    let config = vec![0, 3, 4, 3, 0, 6, 5, 6, 0, 6, 4, 3];
    assert_eq!(p.evaluate(&config).unwrap(), Min(Some(8)));
}

#[test]
fn test_open_shop_scheduling_evaluate_issue_example_suboptimal_schedule() {
    let p = issue_example();
    let config = vec![3, 6, 7, 1, 3, 9, 0, 1, 3, 6, 8, 10];
    let value = p.evaluate(&config).unwrap();
    assert_eq!(value, Min(Some(11)));
}

#[test]
fn test_open_shop_scheduling_evaluate_suboptimal() {
    let p = issue_example();
    let config = vec![0, 3, 4, 3, 5, 8, 5, 8, 10, 6, 10, 13];
    let value = p.evaluate(&config).unwrap();
    // Must be valid and > 8 (non-optimal)
    assert!(value.0.is_some());
    assert!(value.0.unwrap() > 8);
}

#[test]
fn test_open_shop_scheduling_evaluate_overlapping_operations() {
    let p = issue_example();
    let config = vec![0; 12];
    assert_eq!(p.evaluate(&config).unwrap(), Min(None));
}

#[test]
fn test_open_shop_scheduling_evaluate_wrong_length() {
    let p = issue_example();
    // Too short
    assert!(matches!(
        p.evaluate(&vec![0, 1, 2]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    // Too long
    assert!(matches!(
        p.evaluate(&vec![0; 13]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_open_shop_scheduling_evaluate_empty() {
    let p = OpenShopScheduling::new(3, vec![]);
    assert_eq!(p.dimensions(), Vec::<usize>::new());
    assert_eq!(p.evaluate(&vec![]).unwrap(), Min(Some(0)));
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
    let config = vec![0, 1, 1, 0];
    let val = p.evaluate(&config).unwrap();
    assert!(val.0.is_some());
    assert_eq!(val, Min(Some(3)));
}

// ─── decode_orders ───────────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_decode_orders_valid() {
    let p = two_by_two();
    assert_eq!(p.evaluate(&vec![0, 1, 1, 0]).unwrap(), Min(Some(3)));
}

#[test]
fn test_open_shop_scheduling_decode_orders_invalid_duplicate() {
    let p = two_by_two();
    assert_eq!(p.evaluate(&vec![0, 1, 0, 3]).unwrap(), Min(None));
}

#[test]
fn test_open_shop_scheduling_decode_orders_invalid_out_of_range() {
    let p = two_by_two();
    assert_eq!(p.evaluate(&vec![0, 0, 1, 3]).unwrap(), Min(None));
}

// ─── compute_makespan ────────────────────────────────────────────────────────

#[test]
fn test_open_shop_scheduling_compute_makespan_optimal_schedule() {
    let p = issue_example();
    let starts = vec![0, 3, 4, 3, 0, 6, 5, 6, 0, 6, 4, 3];
    assert_eq!(p.evaluate(&starts).unwrap(), Min(Some(8)));
}

#[test]
fn test_open_shop_scheduling_compute_makespan_issue_example_schedule() {
    let p = issue_example();
    let starts = vec![3, 6, 7, 1, 3, 9, 0, 1, 3, 6, 8, 10];
    assert_eq!(p.evaluate(&starts).unwrap(), Min(Some(11)));
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
    let value_solution = solver.solve(&p).unwrap().unwrap();
    let value = p.evaluate(&value_solution).unwrap();
    assert!(value.0.is_some());
    // Optimal value for this instance
    assert_eq!(value, Min(Some(3)));
    let witness = solver.solve(&p).unwrap().unwrap();
    assert_eq!(p.evaluate(&witness).unwrap(), Min(Some(3)));
}

#[test]
fn test_open_shop_scheduling_brute_force_medium() {
    // Small start-time domain keeps exhaustive solving bounded.
    let p = small_asymmetric();
    let solver = BruteForce::new();
    let value_solution = solver.solve(&p).unwrap().unwrap();
    let value = p.evaluate(&value_solution).unwrap();
    assert!(value.0.is_some());
    let witness = solver.solve(&p).unwrap().unwrap();
    assert_eq!(p.evaluate(&witness).unwrap(), value);
}

#[test]
fn test_open_shop_scheduling_canonical_example_config_is_optimal() {
    // Verify that the canonical example config achieves the true optimal makespan = 8
    let p = issue_example();
    let optimal_config = vec![0, 3, 4, 3, 0, 6, 5, 6, 0, 6, 4, 3];
    assert_eq!(p.evaluate(&optimal_config).unwrap(), Min(Some(8)));
}

#[test]
fn test_open_shop_scheduling_construction_paths_reject_invalid_matrices() {
    for (machines, times) in [
        (2, vec![vec![1]]),
        (1, vec![vec![-1]]),
        (2, vec![vec![i64::MAX, 1]]),
    ] {
        assert!(OpenShopScheduling::try_new(machines, times.clone()).is_err());
        assert!(OpenShopScheduling::try_from(OpenShopSchedulingCreateSpec {
            num_processors: machines,
            processing_times: times.clone(),
        })
        .is_err());
        assert!(
            serde_json::from_value::<OpenShopScheduling>(serde_json::json!({
                "num_machines": machines, "processing_times": times,
            }))
            .is_err()
        );
    }
}

#[test]
fn test_open_shop_scheduling_construction_overflow_is_typed() {
    let times = vec![vec![i64::MAX, 1]];
    assert!(matches!(
        OpenShopScheduling::try_new(2, times.clone()),
        Err(crate::registry::ConstructionError::IntegerOverflow(_))
    ));
    assert!(matches!(
        OpenShopScheduling::try_from(OpenShopSchedulingCreateSpec {
            num_processors: 2,
            processing_times: times,
        }),
        Err(crate::registry::ConstructionError::IntegerOverflow(_))
    ));
}
