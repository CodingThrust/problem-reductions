use super::*;
use crate::traits::Problem;
use crate::types::Min;

// ─── helpers ───────────────────────────────────────────────────────────────

/// Small instance: 2 tasks with lengths [2, 1], 2 processors, no precedences.
/// D_max = 3.  Config length = 2 * 3 = 6.
fn small_instance() -> PreemptiveScheduling {
    PreemptiveScheduling::new(vec![2, 1], 2, vec![])
}

/// 2 tasks with a precedence: task 0 → task 1.
/// lengths [1, 1], 2 processors, precedence (0,1).
/// D_max = 2.  Config length = 2 * 2 = 4.
fn precedence_instance() -> PreemptiveScheduling {
    PreemptiveScheduling::new(vec![1, 1], 2, vec![(0, 1)])
}

// ─── creation / accessor tests ─────────────────────────────────────────────

#[test]
fn test_preemptive_scheduling_creation() {
    let p = PreemptiveScheduling::new(vec![2, 1, 3], 2, vec![(0, 2)]);
    assert_eq!(p.num_tasks(), 3);
    assert_eq!(p.num_processors(), 2);
    assert_eq!(p.num_precedences(), 1);
    assert_eq!(p.lengths(), &[2, 1, 3]);
    assert_eq!(p.precedences(), &[(0, 2)]);
    assert_eq!(p.d_max(), 6);
    assert_eq!(p.dims(), vec![2; 3 * 6]);
    assert_eq!(
        <PreemptiveScheduling as Problem>::NAME,
        "PreemptiveScheduling"
    );
    assert!(<PreemptiveScheduling as Problem>::variant().is_empty());
}

#[test]
fn test_preemptive_scheduling_empty_tasks() {
    let p = PreemptiveScheduling::new(vec![], 1, vec![]);
    assert_eq!(p.num_tasks(), 0);
    assert_eq!(p.d_max(), 0);
    assert_eq!(p.dims(), Vec::<usize>::new());
    assert_eq!(p.evaluate(&[]), Min(Some(0)));
}

// ─── evaluate: valid configs ────────────────────────────────────────────────

#[test]
fn test_preemptive_scheduling_evaluate_valid_no_precedence() {
    let p = small_instance();
    // D_max=3; t0 active at 0,1  t1 active at 0
    // config layout: [t0s0, t0s1, t0s2,  t1s0, t1s1, t1s2]
    let config = vec![1, 1, 0, 1, 0, 0];
    assert_eq!(p.evaluate(&config), Min(Some(2)));
}

#[test]
fn test_preemptive_scheduling_evaluate_valid_split() {
    // Single processor, 1 task of length 2; split into slots 0 and 2
    let p = PreemptiveScheduling::new(vec![2], 1, vec![]);
    // D_max=2, config length=2
    let config = vec![1, 1];
    assert_eq!(p.evaluate(&config), Min(Some(2)));
}

#[test]
fn test_preemptive_scheduling_evaluate_valid_precedence() {
    // Task 0 finishes at slot 0 (last=0), task 1 starts at slot 1 (first=1). OK.
    let p = precedence_instance();
    // D_max=2; t0=[1,0], t1=[0,1]
    let config = vec![1, 0, 0, 1];
    assert_eq!(p.evaluate(&config), Min(Some(2)));
}

#[test]
fn test_preemptive_scheduling_makespan_correct() {
    // 3 tasks on 3 processors, no precedences, all finish at slot 2
    let p = PreemptiveScheduling::new(vec![1, 1, 1], 3, vec![]);
    // D_max=3; each task active in exactly 1 slot, all at slot 2
    let config = vec![
        0, 0, 1, // t0 at slot 2
        0, 0, 1, // t1 at slot 2
        0, 0, 1, // t2 at slot 2
    ];
    // 3 tasks at slot 2 <= 3 processors OK, makespan = 3
    assert_eq!(p.evaluate(&config), Min(Some(3)));
}

// ─── evaluate: invalid configs ─────────────────────────────────────────────

#[test]
fn test_preemptive_scheduling_evaluate_wrong_length() {
    let p = small_instance();
    assert_eq!(p.evaluate(&[]), Min(None));
    assert_eq!(p.evaluate(&[1, 1, 0]), Min(None)); // too short
    assert_eq!(p.evaluate(&[1, 1, 0, 1, 0, 0, 0]), Min(None)); // too long
}

#[test]
fn test_preemptive_scheduling_evaluate_wrong_active_count() {
    let p = small_instance();
    // t0 needs 2 active slots but gets 1; t1 needs 1 but gets 1
    let config = vec![1, 0, 0, 1, 0, 0];
    assert_eq!(p.evaluate(&config), Min(None));
}

#[test]
fn test_preemptive_scheduling_evaluate_processor_overflow() {
    // 3 tasks, 2 processors; all three tasks at slot 0
    let p = PreemptiveScheduling::new(vec![1, 1, 1], 2, vec![]);
    // D_max=3; all at slot 0 → 3 tasks > 2 processors
    let config = vec![1, 0, 0, 1, 0, 0, 1, 0, 0];
    assert_eq!(p.evaluate(&config), Min(None));
}

#[test]
fn test_preemptive_scheduling_evaluate_precedence_violation() {
    // Task 0 last active slot 1, task 1 first active slot 1 — not strictly less
    let p = precedence_instance();
    // D_max=2; t0=[0,1], t1=[0,1] — both active at slot 1; last of pred = 1, first of succ = 0
    // Actually last_pred = 1, first_succ = 0 → 1 >= 0 → violation
    let config = vec![0, 1, 1, 0];
    assert_eq!(p.evaluate(&config), Min(None));
}

#[test]
fn test_preemptive_scheduling_evaluate_precedence_same_slot() {
    // Tasks assigned to the same slot; last_pred = 0, first_succ = 0 → violation
    let p = precedence_instance();
    // t0=[1,0], t1=[1,0]
    let config = vec![1, 0, 1, 0];
    assert_eq!(p.evaluate(&config), Min(None));
}

// ─── paper canonical example ────────────────────────────────────────────────

#[test]
fn test_preemptive_scheduling_paper_example() {
    // 5 tasks, lengths [2,1,3,2,1], 2 processors, precedences [(0,2),(1,3)]
    // Optimal makespan = 5
    let p = PreemptiveScheduling::new(vec![2, 1, 3, 2, 1], 2, vec![(0, 2), (1, 3)]);
    let d = p.d_max(); // = 9
    assert_eq!(d, 9);

    let mut config = vec![0usize; 5 * d];
    // t0 (task index 0) occupies config[0..d]
    config[0] = 1; // t0 at slot 0
    config[1] = 1; // t0 at slot 1
                   // t1 (task index 1) occupies config[d..2*d]
    config[d] = 1; // t1 at slot 0
                   // t2 (task index 2) occupies config[2*d..3*d]
    config[2 * d + 2] = 1; // t2 at slot 2
    config[2 * d + 3] = 1; // t2 at slot 3
    config[2 * d + 4] = 1; // t2 at slot 4
                           // t3 (task index 3) occupies config[3*d..4*d]
    config[3 * d + 2] = 1; // t3 at slot 2
    config[3 * d + 3] = 1; // t3 at slot 3
                           // t4 (task index 4) occupies config[4*d..5*d]
    config[4 * d + 1] = 1; // t4 at slot 1

    assert_eq!(p.evaluate(&config), Min(Some(5)));
}

// ─── serialization ──────────────────────────────────────────────────────────

#[test]
fn test_preemptive_scheduling_serialization() {
    let p = PreemptiveScheduling::new(vec![2, 1, 3], 2, vec![(0, 2)]);
    let json = serde_json::to_value(&p).unwrap();
    let restored: PreemptiveScheduling = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_tasks(), p.num_tasks());
    assert_eq!(restored.num_processors(), p.num_processors());
    assert_eq!(restored.lengths(), p.lengths());
    assert_eq!(restored.precedences(), p.precedences());
}

#[test]
fn test_preemptive_scheduling_serialization_roundtrip_evaluate() {
    let p = PreemptiveScheduling::new(vec![1, 1], 2, vec![(0, 1)]);
    let json = serde_json::to_value(&p).unwrap();
    let p2: PreemptiveScheduling = serde_json::from_value(json).unwrap();
    // valid: t0 at 0, t1 at 1
    let config = vec![1, 0, 0, 1];
    assert_eq!(p.evaluate(&config), p2.evaluate(&config));
}

// ─── validation panics ──────────────────────────────────────────────────────

#[test]
#[should_panic(expected = "task lengths must be positive")]
fn test_preemptive_scheduling_zero_length() {
    PreemptiveScheduling::new(vec![0, 1], 2, vec![]);
}

#[test]
#[should_panic(expected = "num_processors must be positive")]
fn test_preemptive_scheduling_zero_processors() {
    PreemptiveScheduling::new(vec![1, 1], 0, vec![]);
}

#[test]
#[should_panic(expected = "precedence index out of range")]
fn test_preemptive_scheduling_precedence_out_of_range() {
    PreemptiveScheduling::new(vec![1, 1], 2, vec![(0, 5)]);
}

// ─── serde validation ───────────────────────────────────────────────────────

#[test]
fn test_preemptive_scheduling_deserialize_invalid_zero_length() {
    let json = serde_json::json!({
        "lengths": [0, 1],
        "num_processors": 2,
        "precedences": []
    });
    let result: Result<PreemptiveScheduling, _> = serde_json::from_value(json);
    assert!(result.is_err());
}

#[test]
fn test_preemptive_scheduling_deserialize_invalid_zero_processors() {
    let json = serde_json::json!({
        "lengths": [1, 2],
        "num_processors": 0,
        "precedences": []
    });
    let result: Result<PreemptiveScheduling, _> = serde_json::from_value(json);
    assert!(result.is_err());
}
