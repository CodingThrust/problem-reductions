use super::*;
use crate::models::algebraic::Bounded;
use crate::models::algebraic::ILP;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Min;

// ─── helpers ───────────────────────────────────────────────────────────────

/// 2 tasks, lengths [1, 1], 2 processors, precedence (0,1).
/// D_max = 2.  Optimal makespan = 2.
fn small_instance() -> PreemptiveScheduling {
    PreemptiveScheduling::new(vec![1, 1], 2, vec![(0, 1)]).unwrap()
}

/// 3 tasks, lengths [2,1,2], 2 processors, precedence (0,2).
/// D_max = 5.  Feasible with makespan ≤ 5.
fn medium_instance() -> PreemptiveScheduling {
    PreemptiveScheduling::new(vec![2, 1, 2], 2, vec![(0, 2)]).unwrap()
}

// ─── structure ─────────────────────────────────────────────────────────────

#[test]
fn test_preemptivescheduling_to_ilp_structure() {
    let p = small_instance();
    let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).unwrap();
    // Precedence leaves only task 0 at slot 0 and task 1 at slot 1.
    assert_eq!(reduction.target_problem().num_vars(), 7);
    assert_eq!(reduction.target_problem().num_constraints(), 11);
    crate::rules::test_helpers::assert_parameter_predictions(&p, &reduction);
}

// ─── closed-loop ───────────────────────────────────────────────────────────

#[test]
fn test_preemptivescheduling_to_ilp_closed_loop() {
    let p = small_instance();
    let reduction: ReductionPSToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let value = p.evaluate(&extracted).unwrap();
    assert!(
        value.0.is_some(),
        "extracted schedule should be valid, got {value:?}"
    );
}

#[test]
fn test_solve_via_registered_integer_ilp_pipeline() {
    let problem = small_instance();
    let solution = ILPSolver::new()
        .solve(&problem)
        .expect("direct ILP<i64, i64, Bounded> reduction should be solvable");

    assert!(problem.evaluate(&solution).unwrap().0.is_some());
}

#[test]
fn test_preemptivescheduling_to_ilp_medium_closed_loop() {
    let p = medium_instance();
    let reduction: ReductionPSToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let value = p.evaluate(&extracted).unwrap();
    assert!(
        value.0.is_some(),
        "extracted schedule should be valid, got {value:?}"
    );
    assert!(
        value.0.map(|v| v <= 5).unwrap_or(false),
        "makespan should be at most 5, got {value:?}"
    );
}

// ─── extract_solution ──────────────────────────────────────────────────────

#[test]
fn test_preemptivescheduling_to_ilp_extract_solution() {
    let p = small_instance();
    let reduction: ReductionPSToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&p).expect("reduction should succeed");
    let ilp_solution = vec![1, 1, 2, 0, 1, 1, 2]; // x, M, S, C
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, vec![vec![true, false], vec![false, true]]);
    assert_eq!(p.evaluate(&extracted).unwrap(), Min(Some(2)));
}

#[test]
fn precedence_encoding_has_constant_size_per_edge() {
    let edges: Vec<_> = (0..12)
        .flat_map(|a| (a + 1..12).map(move |b| (a, b)))
        .collect();
    let independent = PreemptiveScheduling::new(vec![3; 12], 4, vec![]).unwrap();
    let ordered = PreemptiveScheduling::new(vec![3; 12], 4, edges.clone()).unwrap();
    let base = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&independent).unwrap();
    let constrained = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&ordered).unwrap();
    // Finish-before-start needs just two task endpoints per edge, regardless of horizon.
    assert!(
        constrained.target_problem().num_nonzeros()
            <= base.target_problem().num_nonzeros() + 2 * edges.len()
    );
    crate::rules::test_helpers::assert_parameter_predictions(&ordered, &constrained);
}

#[test]
fn scheduling_ilp_preserves_optima_and_rejects_precedence_cycles() {
    use crate::solvers::{BruteForce, ILPSolveError};

    for (lengths, processors, precedences) in [
        (vec![], 1, vec![]),
        (vec![2, 1], 2, vec![]),
        (vec![2, 1], 2, vec![(0, 1)]),
        (vec![1, 1], 2, vec![(0, 1), (0, 1)]),
        (vec![1], 1, vec![(0, 0)]),
        (vec![1, 1], 2, vec![(0, 1), (1, 0)]),
        (vec![1, 1, 1], 2, vec![(0, 1), (1, 2), (2, 0)]),
    ] {
        let source = PreemptiveScheduling::new(lengths, processors, precedences).unwrap();
        let expected = BruteForce::new().solve(&source).unwrap();
        let reduced = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_parameter_predictions(&source, &reduced);
        match (expected, ILPSolver::new().solve(reduced.target_problem())) {
            (Some(expected), Ok(actual)) => {
                let decoded = reduced.extract_solution(&actual).unwrap();
                assert_eq!(source.evaluate(&decoded), source.evaluate(&expected));
            }
            (None, Err(ILPSolveError::Infeasible)) => {}
            other => panic!("source and target disagree: {other:?}"),
        }
    }
}

#[test]
fn endpoint_encoding_allows_interruptions_but_rejects_early_successors() {
    let source = PreemptiveScheduling::new(vec![2, 1, 1, 3], 2, vec![(0, 2)]).unwrap();
    let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    // Task 0 runs at 0 and 2; its successor may start at 3, not at 1.
    let mut witness = vec![
        1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 4, 0, 1, 3, 0, 3, 2, 4, 3,
    ];
    let decoded = reduction.extract_solution(&witness).unwrap();
    assert_eq!(source.evaluate(&decoded).unwrap(), Min(Some(4)));
    witness[7] = 1;
    witness[8] = 0;
    witness[16] = 2;
    witness[20] = 3;
    assert!(!reduction.target_problem().is_feasible(&witness).unwrap());
    assert!(reduction.extract_solution(&witness).is_err());
}

#[test]
fn scheduling_arithmetic_overflow_is_reported_before_allocation() {
    let horizon = i64::try_from(usize::MAX / 2).unwrap();
    // One case exceeds the variable count, the other endpoint-row arithmetic.
    for lengths in [vec![horizon - 1, 1], vec![horizon]] {
        let source = PreemptiveScheduling::new(lengths, 1, vec![]).unwrap();
        assert!(matches!(
            ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}

#[test]
fn parallel_work_uses_a_certified_horizon_instead_of_the_serial_horizon() {
    let source = PreemptiveScheduling::new(vec![1; 4], 2, vec![]).unwrap();
    let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    // Two slots suffice; eight activity bits plus two endpoints per task and M.
    assert!(reduction.target_problem().num_vars() <= 17);
    let values = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    let decoded = reduction.extract_solution(&values).unwrap();
    assert_eq!(source.evaluate(&decoded).unwrap(), Min(Some(2)));
    assert!(decoded.iter().all(|task| task.len() == 4));
}
