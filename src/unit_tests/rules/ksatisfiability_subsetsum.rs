use super::*;
use crate::models::formula::CNFClause;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::variant::K3;
use num_bigint::BigUint;

#[test]
fn test_ksatisfiability_to_subsetsum_closed_loop() {
    // Issue example: (x1 ∨ x2 ∨ x3) ∧ (¬x1 ∨ ¬x2 ∨ x3), n=3, m=2
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),   // x1 ∨ x2 ∨ x3
            CNFClause::new(vec![-1, -2, 3]), // ¬x1 ∨ ¬x2 ∨ x3
        ],
    );
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // Verify structure: 2*3 + 2*2 = 10 elements
    assert_eq!(target.num_elements(), 10);

    // Verify target value: 11144
    assert_eq!(target.target(), &BigUint::from(11144u32));

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(target);
    assert!(!solutions.is_empty());

    // Every SubsetSum solution must map back to a satisfying 3-SAT assignment
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        assert_eq!(extracted.len(), 3);
        assert!(ksat.evaluate(&extracted));
    }
}

#[test]
fn test_ksatisfiability_to_subsetsum_unsatisfiable() {
    // Unsatisfiable: (x1) ∧ (¬x1) ∧ (x1) — but 3-SAT needs 3 literals per clause.
    // Use: (x1 ∨ x1 ∨ x1) ∧ (¬x1 ∨ ¬x1 ∨ ¬x1) ∧ (x1 ∨ x1 ∨ x1)
    // x1=T satisfies C1,C3 but not C2. x1=F satisfies C2 but not C1,C3.
    let ksat = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
            CNFClause::new(vec![1, 1, 1]),
        ],
    );
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&ksat);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let solution = solver.find_satisfying(target);
    assert!(solution.is_none());
}

#[test]
fn test_ksatisfiability_to_subsetsum_single_clause() {
    // Single clause: (x1 ∨ x2 ∨ x3) — 7 out of 8 assignments satisfy it
    let ksat = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // 2*3 + 2*1 = 8 elements
    assert_eq!(target.num_elements(), 8);

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(target);

    // Each SubsetSum solution maps to a satisfying assignment
    let mut sat_assignments = std::collections::HashSet::new();
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ksat.evaluate(&extracted));
        sat_assignments.insert(extracted);
    }
    // Should find exactly 7 distinct satisfying assignments
    assert_eq!(sat_assignments.len(), 7);
}

#[test]
fn test_ksatisfiability_to_subsetsum_structure() {
    // Verify sizes match the issue's example table
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),   // x1 ∨ x2 ∨ x3
            CNFClause::new(vec![-1, -2, 3]), // ¬x1 ∨ ¬x2 ∨ x3
        ],
    );
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&ksat);
    let target = reduction.target_problem();
    let sizes = target.sizes();

    // From the issue:
    // y1=10010, z1=10001, y2=01010, z2=01001, y3=00111, z3=00100
    // g1=00010, h1=00020, g2=00001, h2=00002
    assert_eq!(sizes[0], BigUint::from(10010u32)); // y1
    assert_eq!(sizes[1], BigUint::from(10001u32)); // z1
    assert_eq!(sizes[2], BigUint::from(1010u32)); // y2 (leading zero dropped)
    assert_eq!(sizes[3], BigUint::from(1001u32)); // z2
    assert_eq!(sizes[4], BigUint::from(111u32)); // y3
    assert_eq!(sizes[5], BigUint::from(100u32)); // z3
    assert_eq!(sizes[6], BigUint::from(10u32)); // g1
    assert_eq!(sizes[7], BigUint::from(20u32)); // h1
    assert_eq!(sizes[8], BigUint::from(1u32)); // g2
    assert_eq!(sizes[9], BigUint::from(2u32)); // h2
}

#[test]
fn test_ksatisfiability_to_subsetsum_all_negated() {
    // All negated: (¬x1 ∨ ¬x2 ∨ ¬x3) — 7 satisfying assignments
    let ksat = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![-1, -2, -3])]);
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&ksat);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(target);

    let mut sat_assignments = std::collections::HashSet::new();
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ksat.evaluate(&extracted));
        sat_assignments.insert(extracted);
    }
    assert_eq!(sat_assignments.len(), 7);
}

#[test]
fn test_ksatisfiability_to_subsetsum_extract_solution_example() {
    // Verify the specific example from the issue:
    // x1=T, x2=T, x3=T → select y1, y2, y3 + slack to reach target 11144
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&ksat);
    let target = reduction.target_problem();

    // Construct the known subset for x1=T, x2=T, x3=T:
    // y1=10010, y2=01010, y3=00111 → variable digits sum: 111, clause digits: 31
    // Need clause digits = 44, so slack: C1 needs +1 (g1=10), C2 needs +3 (g2=1, h2=2)
    // Total: 10010 + 01010 + 00111 + 00010 + 00001 + 00002 = 11144
    let specific_config = vec![
        1, 0, // y1 selected, z1 not
        1, 0, // y2 selected, z2 not
        1, 0, // y3 selected, z3 not
        1, 0, // g1 selected, h1 not
        1, 1, // g2 selected, h2 selected
    ];
    assert!(target.evaluate(&specific_config));

    let extracted = reduction.extract_solution(&specific_config);
    assert_eq!(extracted, vec![1, 1, 1]); // x1=T, x2=T, x3=T
    assert!(ksat.evaluate(&extracted));
}
