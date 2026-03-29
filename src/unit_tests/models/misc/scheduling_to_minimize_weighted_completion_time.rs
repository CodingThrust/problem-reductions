use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_scheduling_min_wct_creation() {
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2, 3, 4, 5],
        vec![6, 4, 3, 2, 1],
        2,
    );
    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.num_processors(), 2);
    assert_eq!(problem.lengths(), &[1, 2, 3, 4, 5]);
    assert_eq!(problem.weights(), &[6, 4, 3, 2, 1]);
    assert_eq!(problem.dims(), vec![2; 5]);
    assert_eq!(
        <SchedulingToMinimizeWeightedCompletionTime as Problem>::NAME,
        "SchedulingToMinimizeWeightedCompletionTime"
    );
    assert_eq!(
        <SchedulingToMinimizeWeightedCompletionTime as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_scheduling_min_wct_evaluate_issue_example() {
    // Issue example: 5 tasks, 2 processors
    // Optimal: P0={t0,t2,t4}, P1={t1,t3} => cost = 47
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2, 3, 4, 5],
        vec![6, 4, 3, 2, 1],
        2,
    );
    // config: [0, 1, 0, 1, 0] means t0->P0, t1->P1, t2->P0, t3->P1, t4->P0
    assert_eq!(problem.evaluate(&[0, 1, 0, 1, 0]), Min(Some(47)));
}

#[test]
fn test_scheduling_min_wct_evaluate_all_one_processor() {
    // All tasks on one processor
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2, 3, 4, 5],
        vec![6, 4, 3, 2, 1],
        2,
    );
    // All on processor 0: Smith's rule order t0,t1,t2,t3,t4
    // C(t0)=1, C(t1)=3, C(t2)=6, C(t3)=10, C(t4)=15
    // WCT = 1*6 + 3*4 + 6*3 + 10*2 + 15*1 = 6+12+18+20+15 = 71
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0]), Min(Some(71)));
}

#[test]
fn test_scheduling_min_wct_evaluate_invalid_config() {
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(vec![1, 2], vec![3, 4], 2);
    // Wrong length
    assert_eq!(problem.evaluate(&[0]), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 0]), Min(None));
    // Out-of-range processor
    assert_eq!(problem.evaluate(&[0, 2]), Min(None));
}

#[test]
fn test_scheduling_min_wct_solver() {
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2, 3, 4, 5],
        vec![6, 4, 3, 2, 1],
        2,
    );
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&witness), Min(Some(47)));
}

#[test]
fn test_scheduling_min_wct_find_all_witnesses() {
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2, 3, 4, 5],
        vec![6, 4, 3, 2, 1],
        2,
    );
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(&problem);
    // Issue says 2 optimal assignments (mirror pair)
    assert_eq!(witnesses.len(), 2);
    for w in &witnesses {
        assert_eq!(problem.evaluate(w), Min(Some(47)));
    }
}

#[test]
fn test_scheduling_min_wct_serialization() {
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2, 3, 4, 5],
        vec![6, 4, 3, 2, 1],
        2,
    );
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SchedulingToMinimizeWeightedCompletionTime =
        serde_json::from_value(json).unwrap();
    assert_eq!(restored.lengths(), problem.lengths());
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(restored.num_processors(), problem.num_processors());
}

#[test]
fn test_scheduling_min_wct_deserialization_rejects_zero_processors() {
    let err =
        serde_json::from_value::<SchedulingToMinimizeWeightedCompletionTime>(serde_json::json!({
            "lengths": [1, 2],
            "weights": [3, 4],
            "num_processors": 0
        }))
        .unwrap_err();
    assert!(
        err.to_string().contains("num_processors must be positive"),
        "unexpected error: {err}"
    );
}

#[test]
#[should_panic(expected = "num_processors must be positive")]
fn test_scheduling_min_wct_zero_processors() {
    SchedulingToMinimizeWeightedCompletionTime::new(vec![1], vec![1], 0);
}

#[test]
#[should_panic(expected = "lengths and weights must have the same length")]
fn test_scheduling_min_wct_mismatched_lengths() {
    SchedulingToMinimizeWeightedCompletionTime::new(vec![1, 2], vec![3], 2);
}

#[test]
#[should_panic(expected = "task lengths must be positive")]
fn test_scheduling_min_wct_zero_length() {
    SchedulingToMinimizeWeightedCompletionTime::new(vec![0, 1], vec![1, 1], 1);
}

#[test]
#[should_panic(expected = "task weights must be positive")]
fn test_scheduling_min_wct_zero_weight() {
    SchedulingToMinimizeWeightedCompletionTime::new(vec![1, 1], vec![0, 1], 1);
}

#[test]
fn test_scheduling_min_wct_single_task() {
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(vec![5], vec![3], 2);
    // Task 0 on processor 0: C(0) = 5, WCT = 5*3 = 15
    assert_eq!(problem.evaluate(&[0]), Min(Some(15)));
    assert_eq!(problem.evaluate(&[1]), Min(Some(15)));
}

#[test]
fn test_scheduling_min_wct_single_processor() {
    // With 1 processor, Smith's rule determines the order
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(vec![2, 1], vec![1, 3], 1);
    // Smith's rule: t1 has l/w=1/3=0.33, t0 has l/w=2/1=2.0
    // Order: t1, t0
    // C(t1) = 1, C(t0) = 3
    // WCT = 1*3 + 3*1 = 6
    assert_eq!(problem.evaluate(&[0, 0]), Min(Some(6)));
}

#[test]
fn test_scheduling_min_wct_three_processors() {
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(vec![3, 3, 3], vec![1, 1, 1], 3);
    assert_eq!(problem.dims(), vec![3; 3]);
    // One task per processor: each completes at 3, WCT = 3*1 + 3*1 + 3*1 = 9
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(Some(9)));
    // All on one processor: C(t0)=3, C(t1)=6, C(t2)=9, WCT = 3+6+9 = 18
    assert_eq!(problem.evaluate(&[0, 0, 0]), Min(Some(18)));
}

#[test]
fn test_scheduling_min_wct_paper_example() {
    // Same as issue example - verifying the worked example
    let problem = SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2, 3, 4, 5],
        vec![6, 4, 3, 2, 1],
        2,
    );
    // P0={t0,t2,t4}: Smith order t0(0.167), t2(1.0), t4(5.0)
    //   C(t0)=1 => 1*6=6, C(t2)=4 => 4*3=12, C(t4)=9 => 9*1=9, subtotal=27
    // P1={t1,t3}: Smith order t1(0.5), t3(2.0)
    //   C(t1)=2 => 2*4=8, C(t3)=6 => 6*2=12, subtotal=20
    // Total = 47
    assert_eq!(problem.evaluate(&[0, 1, 0, 1, 0]), Min(Some(47)));

    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&witness), Min(Some(47)));
}
