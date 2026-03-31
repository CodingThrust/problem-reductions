use super::*;
use crate::models::formula::CNFClause;
use crate::models::formula::NAESatisfiability;
use crate::models::graph::MaxCut;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_naesatisfiability_to_maxcut_closed_loop() {
    // 3 variables, 2 clauses:
    //   C1 = (x1, x2, x3)
    //   C2 = (~x1, ~x2, x3)
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let target = reduction.target_problem();

    // 2*3 = 6 vertices
    assert_eq!(target.num_vertices(), 6);
    // 3 variable edges + 3 + 3 = 9 clause edges
    assert_eq!(target.num_edges(), 9);

    assert_satisfaction_round_trip_from_optimization_target(
        &naesat,
        &reduction,
        "NAESAT -> MaxCut closed loop",
    );
}

#[test]
fn test_naesatisfiability_to_maxcut_single_clause() {
    // Single clause: (x1, x2, x3) — NAE-satisfying iff not all same
    let naesat = NAESatisfiability::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let target = reduction.target_problem();

    // 6 vertices, 3 variable + 3 clause = 6 edges
    assert_eq!(target.num_vertices(), 6);
    assert_eq!(target.num_edges(), 6);

    assert_satisfaction_round_trip_from_optimization_target(
        &naesat,
        &reduction,
        "NAESAT single clause -> MaxCut",
    );
}

#[test]
fn test_naesatisfiability_to_maxcut_two_literal_clause() {
    // Clause with 2 literals: (x1, ~x2) — always NAE-satisfying unless x1=T, x2=F or x1=F, x2=T... actually (x1, ~x2) is NAE-unsatisfied when both literals are same: x1=T,~x2=T (x2=F) or x1=F,~x2=F (x2=T).
    // NAE-satisfied when x1 != ~x2, i.e., x1 == x2.
    let naesat = NAESatisfiability::new(2, vec![CNFClause::new(vec![1, -2])]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let target = reduction.target_problem();

    // 4 vertices, 2 variable + 1 clause = 3 edges
    assert_eq!(target.num_vertices(), 4);
    assert_eq!(target.num_edges(), 3);

    assert_satisfaction_round_trip_from_optimization_target(
        &naesat,
        &reduction,
        "NAESAT 2-literal clause -> MaxCut",
    );
}

#[test]
fn test_naesatisfiability_to_maxcut_four_literal_clause() {
    // Clause with 4 literals: (x1, x2, ~x3, x4)
    let naesat = NAESatisfiability::new(4, vec![CNFClause::new(vec![1, 2, -3, 4])]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let target = reduction.target_problem();

    // 8 vertices, 4 variable + C(4,2)=6 clause = 10 edges
    assert_eq!(target.num_vertices(), 8);
    assert_eq!(target.num_edges(), 10);

    assert_satisfaction_round_trip_from_optimization_target(
        &naesat,
        &reduction,
        "NAESAT 4-literal clause -> MaxCut",
    );
}

#[test]
fn test_naesatisfiability_to_maxcut_extract_solution() {
    // Verify specific extraction: x1=T, x2=F, x3=T
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, 3, 2]),
        ],
    );
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);

    // Vertices: x1(0), ~x1(1), x2(2), ~x2(3), x3(4), ~x3(5)
    // x1=T -> vertex 0 in set 1, vertex 1 in set 0
    // x2=F -> vertex 2 in set 0, vertex 3 in set 1
    // x3=T -> vertex 4 in set 1, vertex 5 in set 0
    let target_config = vec![1, 0, 0, 1, 1, 0];
    let extracted = reduction.extract_solution(&target_config);
    assert_eq!(extracted, vec![1, 0, 1]); // x1=T, x2=F, x3=T

    // Verify this is a valid NAE-SAT solution
    assert!(naesat.evaluate(&extracted).0);
}

#[test]
fn test_naesatisfiability_to_maxcut_mixed_clause_sizes() {
    // Mix of 2-literal and 3-literal clauses
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, -2]),   // 2 literals -> C(2,2)=1 pair
            CNFClause::new(vec![1, 2, 3]), // 3 literals -> C(3,2)=3 pairs
            CNFClause::new(vec![-1, -3]),  // 2 literals -> 1 pair
        ],
    );
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let target = reduction.target_problem();

    // 6 vertices, 3 variable + (1 + 3 + 1) = 8 edges
    assert_eq!(target.num_vertices(), 6);
    assert_eq!(target.num_edges(), 8);

    assert_satisfaction_round_trip_from_optimization_target(
        &naesat,
        &reduction,
        "NAESAT mixed clause sizes -> MaxCut",
    );
}

#[test]
fn test_naesatisfiability_to_maxcut_optimal_cut_value() {
    // Verify the optimal cut value matches theoretical prediction
    // n*M + sum(k_j - 1) for satisfiable instances
    let naesat = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&naesat);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let witness = solver.find_witness(target);
    assert!(witness.is_some());

    let config = witness.unwrap();
    let cut_value = target.cut_size(&config);
    // n=3, m=2, M=3, k1=3, k2=3
    // Expected: 3*3 + (3-1) + (3-1) = 9 + 2 + 2 = 13
    assert_eq!(cut_value, 13);
}
