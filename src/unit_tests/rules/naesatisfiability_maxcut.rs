use super::*;
use crate::models::formula::CNFClause;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_naesatisfiability_to_maxcut_closed_loop() {
    // NAE-3-SAT: C1 = (x1, x2, x3), C2 = (¬x1, ¬x2, ¬x3)
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );

    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);

    assert_satisfaction_round_trip_from_optimization_target(
        &naesat,
        &reduction,
        "NAE-SAT->MaxCut closed loop",
    );
}

#[test]
fn test_reduction_structure() {
    // n=3, m=2: expect 6 vertices, up to 9 edges
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );

    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let maxcut = reduction.target_problem();

    assert_eq!(maxcut.num_vertices(), 6); // 2n
                                          // 3 variable edges + 3 triangle edges for C1 + 3 triangle edges for C2
                                          // No overlap since C1 uses positive literal vertices and C2 uses negative
    assert_eq!(maxcut.num_edges(), 9); // n + 3m = 3 + 6
}

#[test]
fn test_variable_gadget_weights() {
    // Verify variable-gadget edges have weight M = 2m + 1
    let naesat = NAESatisfiability::new(
        2,
        vec![
            CNFClause::new(vec![1, 2, -1]),
            CNFClause::new(vec![-1, -2, 1]),
        ],
    );

    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let maxcut = reduction.target_problem();

    // M = 2*2 + 1 = 5
    // Variable gadget edge (v0, v0') = (0, 1) should have weight >= 5
    let w01 = maxcut.edge_weight(0, 1).copied().unwrap();
    assert!(
        w01 >= 5,
        "Variable gadget edge weight should be at least M=5, got {w01}"
    );
}

#[test]
fn test_optimal_cut_value() {
    // For a satisfiable NAE-3-SAT instance, optimal cut = n*M + 2m
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );

    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let maxcut = reduction.target_problem();

    let solver = BruteForce::new();
    let witness = solver.find_witness(maxcut).unwrap();
    let value = maxcut.evaluate(&witness);

    // n=3, m=2, M=5: threshold = 3*5 + 2*2 = 19
    assert_eq!(value, Max(Some(19)));
}

#[test]
fn test_solution_extraction() {
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );

    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);

    // Target config: v0=1, v0'=0, v1=0, v1'=1, v2=0, v2'=1
    // → x1=1, x2=0, x3=0
    let source_sol = reduction.extract_solution(&[1, 0, 0, 1, 0, 1]);
    assert_eq!(source_sol, vec![1, 0, 0]);
    assert!(naesat.is_valid_solution(&source_sol));
}

#[test]
fn test_larger_instance() {
    // 4 variables, 3 clauses
    let naesat = NAESatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-2, 3, 4]),
            CNFClause::new(vec![1, -3, -4]),
        ],
    );

    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let maxcut = reduction.target_problem();

    assert_eq!(maxcut.num_vertices(), 8); // 2*4

    assert_satisfaction_round_trip_from_optimization_target(
        &naesat,
        &reduction,
        "NAE-SAT->MaxCut larger instance",
    );
}

#[test]
fn test_edge_merging() {
    // Clause contains both x1 and ¬x1: the triangle edge (v1, v1') overlaps
    // with the variable gadget edge and weights should merge.
    let naesat = NAESatisfiability::new(2, vec![CNFClause::new(vec![1, -1, 2])]);

    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let maxcut = reduction.target_problem();

    // Variable edge (0,1) should have weight M + 1 = 3 + 1 = 4
    // (M = 2*1 + 1 = 3, plus 1 from the triangle edge)
    let w01 = maxcut.edge_weight(0, 1).copied().unwrap();
    assert_eq!(w01, 4);

    assert_satisfaction_round_trip_from_optimization_target(
        &naesat,
        &reduction,
        "NAE-SAT->MaxCut edge merging",
    );
}
