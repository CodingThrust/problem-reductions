use super::*;
use crate::models::formula::CNFClause;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::variant::{K2, K3};

#[test]
fn test_ksatisfiability_to_qubo_closed_loop() {
    // 3 vars, 4 clauses (matches ground truth):
    // (x1 ∨ x2), (¬x1 ∨ x3), (x2 ∨ ¬x3), (¬x2 ∨ ¬x3)
    let ksat = KSatisfiability::<K2>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),   // x1 ∨ x2
            CNFClause::new(vec![-1, 3]),  // ¬x1 ∨ x3
            CNFClause::new(vec![2, -3]),  // x2 ∨ ¬x3
            CNFClause::new(vec![-2, -3]), // ¬x2 ∨ ¬x3
        ],
    );
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ksat).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    // Verify all solutions satisfy all clauses
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(ksat.evaluate(&extracted).unwrap());
    }
}

#[test]
fn test_ksatisfiability_to_qubo_simple() {
    // 2 vars, 1 clause: (x1 ∨ x2) → 3 satisfying assignments
    let ksat = KSatisfiability::<K2>::new(2, vec![CNFClause::new(vec![1, 2])]);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ksat).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(ksat.evaluate(&extracted).unwrap());
    }
}

#[test]
fn test_ksatisfiability_to_qubo_contradiction() {
    // 1 var, 2 clauses: (x1 ∨ x1) and (¬x1 ∨ ¬x1) — can't satisfy both
    // Actually, this is (x1) and (¬x1), which is a contradiction
    // Max-2-SAT will satisfy 1 of 2 clauses
    let ksat = KSatisfiability::<K2>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1]),   // x1 ∨ x1 = x1
            CNFClause::new(vec![-1, -1]), // ¬x1 ∨ ¬x1 = ¬x1
        ],
    );
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ksat).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    // Both x=0 and x=1 satisfy exactly 1 clause
    assert_eq!(qubo_solutions.len(), 2);
}

#[test]
fn test_ksatisfiability_to_qubo_reversed_vars() {
    // Clause (3, -1) has var_i=2 > var_j=0, triggering the swap branch (line 71).
    // 3 vars, clauses: (x3 ∨ ¬x1), (x1 ∨ x2)
    let ksat = KSatisfiability::<K2>::new(
        3,
        vec![
            CNFClause::new(vec![3, -1]), // var 2 > var 0 → swap
            CNFClause::new(vec![1, 2]),
        ],
    );
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ksat).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(ksat.evaluate(&extracted).unwrap());
    }
}

#[test]
fn test_ksatisfiability_to_qubo_structure() {
    let ksat = KSatisfiability::<K2>::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ksat).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    // QUBO should have at least the original variables
    assert!(qubo.num_variables() >= ksat.num_vars());
}

#[test]
fn test_k3satisfiability_to_qubo_closed_loop() {
    // 3-SAT: 5 vars, 7 clauses
    let ksat = KSatisfiability::<K3>::new(
        5,
        vec![
            CNFClause::new(vec![1, 2, -3]),  // x1 ∨ x2 ∨ ¬x3
            CNFClause::new(vec![-1, 3, 4]),  // ¬x1 ∨ x3 ∨ x4
            CNFClause::new(vec![2, -4, 5]),  // x2 ∨ ¬x4 ∨ x5
            CNFClause::new(vec![-2, 3, -5]), // ¬x2 ∨ x3 ∨ ¬x5
            CNFClause::new(vec![1, -3, 5]),  // x1 ∨ ¬x3 ∨ x5
            CNFClause::new(vec![-1, -2, 4]), // ¬x1 ∨ ¬x2 ∨ x4
            CNFClause::new(vec![3, -4, -5]), // x3 ∨ ¬x4 ∨ ¬x5
        ],
    );
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ksat).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    // QUBO should have 5 + 7 = 12 variables
    assert_eq!(qubo.num_variables(), 12);

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    // Verify all extracted solutions maximize satisfied clauses
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert_eq!(extracted.len(), 5);
        let satisfied = ksat.count_satisfied(&extracted).unwrap();
        assert_eq!(satisfied, 7, "Expected all 7 clauses satisfied");
    }
}

#[test]
fn test_k3satisfiability_to_qubo_single_clause() {
    // Single 3-SAT clause: (x1 ∨ x2 ∨ x3) — 7 satisfying assignments
    let ksat = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ksat).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    // 3 vars + 1 auxiliary = 4 total
    assert_eq!(qubo.num_variables(), 4);

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    // All solutions should satisfy the single clause
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert_eq!(extracted.len(), 3);
        assert!(ksat.evaluate(&extracted).unwrap());
    }
    // 7 out of 8 assignments satisfy (x1 ∨ x2 ∨ x3)
    assert_eq!(qubo_solutions.len(), 7);
}

#[test]
fn test_k3satisfiability_to_qubo_all_negated() {
    // All negated: (¬x1 ∨ ¬x2 ∨ ¬x3) — 7 satisfying assignments
    let ksat = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![-1, -2, -3])]);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&ksat).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_witnesses(qubo).unwrap();

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert!(ksat.evaluate(&extracted).unwrap());
    }
    // 7 out of 8 assignments satisfy (¬x1 ∨ ¬x2 ∨ ¬x3)
    assert_eq!(qubo_solutions.len(), 7);
}

#[test]
fn test_sat_qubo_all_short_clauses_and_raw_targets() {
    use crate::rules::AggregateReductionResult;
    use crate::types::{Min, Or};
    macro_rules! verify {
        ($k:ty, $width:expr) => {{
            let mut clauses = vec![vec![]];
            for width in 1..=$width {
                for mask in 0..(1 << width) {
                    clauses.push(
                        (0..width)
                            .map(|p| if mask & (1 << p) == 0 { 1 } else { -1 })
                            .collect(),
                    );
                }
            }
            for a in &clauses {
                for b in &clauses {
                    let source = KSatisfiability::<$k>::new_allow_less(
                        1,
                        vec![CNFClause::new(a.clone()), CNFClause::new(b.clone())],
                    );
                    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
                    let target = ReductionResult::target_problem(&reduction);
                    let mut minimum = i64::MAX;
                    for mask in 0..(1 << target.num_vars()) {
                        let witness: Vec<_> = (0..target.num_vars())
                            .map(|p| mask & (1 << p) != 0)
                            .collect();
                        let energy = target.evaluate(&witness).unwrap().0.unwrap();
                        let mut penalty = 0;
                        for (j, clause) in [a, b].iter().enumerate() {
                            let y: Vec<i64> = clause
                                .iter()
                                .map(|&l| i64::from(witness[0] != (l > 0)))
                                .collect();
                            penalty += match y.as_slice() {
                                [] => 1,
                                &[u] => u,
                                &[u, v] => u * v,
                                &[u, v, w] => {
                                    let z = i64::from(witness[1 + j]);
                                    z * w + 2 * (u * v - 2 * u * z - 2 * v * z + 3 * z)
                                }
                                _ => unreachable!(),
                            };
                        }
                        assert_eq!(energy - reduction.zero_penalty_energy, penalty);
                        assert_eq!(
                            AggregateReductionResult::extract_value(&reduction, Min(Some(energy))),
                            Or(penalty == 0)
                        );
                        if penalty == 0 {
                            let decoded = reduction.extract_solution(&witness).unwrap();
                            assert!(source.evaluate(&decoded).unwrap().0);
                        } else {
                            assert!(reduction.extract_solution(&witness).is_err());
                        }
                        minimum = minimum.min(energy);
                    }
                    let sat = [false, true]
                        .into_iter()
                        .any(|x| source.evaluate(&vec![x]).unwrap().0);
                    assert_eq!(
                        AggregateReductionResult::extract_value(&reduction, Min(Some(minimum))),
                        Or(sat)
                    );
                    assert_eq!(
                        AggregateReductionResult::extract_value(&reduction, Min(None)),
                        Or(false)
                    );
                    assert!(reduction.extract_solution(&vec![]).is_err());
                    assert!(reduction
                        .extract_solution(&vec![false; target.num_vars() + 1])
                        .is_err());
                }
            }
            for n in [0, 3] {
                let source = KSatisfiability::<$k>::new(n, vec![]);
                let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
                assert_eq!(
                    reduction.extract_solution(&vec![false; n]).unwrap(),
                    vec![false; n]
                );
            }
        }};
    }
    verify!(K2, 2);
    verify!(K3, 3);
}

#[test]
fn test_sat_qubo_checked_numeric_boundaries() {
    let mut matrix = vec![vec![i64::MAX]];
    assert!(add_coefficient(&mut matrix, 0, 0, 1).is_err());
    let mut matrix = vec![vec![i64::MIN]];
    assert!(add_coefficient(&mut matrix, 0, 0, -1).is_err());
    assert!(build_qubo_matrix(usize::MAX, &[], 1).is_err());
    // This variable count is legal for the source on both 32- and 64-bit hosts,
    // but its dense target cannot have an addressable number of entries.
    let n = usize::MAX / 2;
    let k2 = KSatisfiability::<K2>::new(n, vec![]);
    let k3 = KSatisfiability::<K3>::new(n, vec![]);
    assert!(matches!(
        ReduceTo::<QUBO<i64>>::reduce_to(&k2),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
    assert!(matches!(
        ReduceTo::<QUBO<i64>>::reduce_to(&k3),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn test_sat_qubo_registered_aggregate_threshold() {
    use crate::types::Or;
    macro_rules! check {
        ($k:ty) => {
            for (clauses, expected) in [(vec![vec![1]], true), (vec![vec![1], vec![-1]], false)] {
                let source = KSatisfiability::<$k>::new_allow_less(
                    1,
                    clauses.into_iter().map(CNFClause::new).collect(),
                );
                let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
                let mut witness = vec![false; reduction.target.num_vars()];
                witness[0] = expected;
                let entries = crate::rules::registry::reduction_entries();
                let edge = entries
                    .iter()
                    .find(|e| {
                        e.source_name == "KSatisfiability"
                            && e.target_name == "QUBO"
                            && (e.source_variant_fn)() == KSatisfiability::<$k>::variant()
                            && (e.target_variant_fn)() == QUBO::<i64>::variant()
                    })
                    .unwrap();
                let aggregate = (edge.reduce_aggregate_fn.unwrap())(&source).unwrap();
                assert_eq!(
                    *aggregate
                        .extract_value_from_solution_dyn(&witness)
                        .unwrap()
                        .downcast::<Or>()
                        .unwrap(),
                    Or(expected)
                );
            }
        };
    }
    check!(K2);
    check!(K3);
}
