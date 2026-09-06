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
    let reduction =
        ReduceTo::<MaxCut<SimpleGraph, i64>>::reduce_to(&naesat).expect("reduction should succeed");
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
    let reduction =
        ReduceTo::<MaxCut<SimpleGraph, i64>>::reduce_to(&naesat).expect("reduction should succeed");
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
    let reduction =
        ReduceTo::<MaxCut<SimpleGraph, i64>>::reduce_to(&naesat).expect("reduction should succeed");
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
    let reduction =
        ReduceTo::<MaxCut<SimpleGraph, i64>>::reduce_to(&naesat).expect("reduction should succeed");
    let target = reduction.target_problem();

    // One auxiliary variable and two triangles: 10 vertices, 5 + 6 edges.
    assert_eq!(target.num_vertices(), 10);
    assert_eq!(target.num_edges(), 11);

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
    let reduction =
        ReduceTo::<MaxCut<SimpleGraph, i64>>::reduce_to(&naesat).expect("reduction should succeed");

    // Vertices: x1(0), ~x1(1), x2(2), ~x2(3), x3(4), ~x3(5)
    // x1=T -> vertex 0 in set 1, vertex 1 in set 0
    // x2=F -> vertex 2 in set 0, vertex 3 in set 1
    // x3=T -> vertex 4 in set 1, vertex 5 in set 0
    let target_config = vec![true, false, false, true, true, false];
    let extracted = reduction.extract_solution(&target_config).unwrap();
    assert_eq!(extracted, vec![true, false, true]); // x1=T, x2=F, x3=T

    // Verify this is a valid NAE-SAT solution
    assert!(naesat.evaluate(&extracted).unwrap().0);
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
    let reduction =
        ReduceTo::<MaxCut<SimpleGraph, i64>>::reduce_to(&naesat).expect("reduction should succeed");
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
    let reduction =
        ReduceTo::<MaxCut<SimpleGraph, i64>>::reduce_to(&naesat).expect("reduction should succeed");
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let witness = solver.solve(target).unwrap();
    assert!(witness.is_some());

    let config = witness.unwrap();
    let cut_value = target.cut_size(&config).unwrap();
    // n=3, m=2, M=3, k1=3, k2=3
    // Expected: 3*3 + (3-1) + (3-1) = 9 + 2 + 2 = 13
    assert_eq!(cut_value, 13);
}

fn check_every_cut(source: &NAESatisfiability) {
    use crate::rules::AggregateReductionResult;
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i64>>::reduce_to(source).unwrap();
    let target = AggregateReductionResult::target_problem(&reduction);
    let mut decoded = vec![false; 1 << source.num_vars()];
    let mut best = i64::MIN;
    for mask in 0..(1usize << target.num_vertices()) {
        let cut = (0..target.num_vertices())
            .map(|i| mask & (1 << i) != 0)
            .collect();
        let value = target.evaluate(&cut).unwrap();
        best = best.max(value.0.unwrap());
        let certificate = AggregateReductionResult::extract_value(&reduction, value).0;
        match reduction.extract_solution(&cut) {
            Ok(assignment) => {
                assert!(certificate);
                assert!(source.evaluate(&assignment).unwrap().0);
                assert_eq!(
                    assignment,
                    (0..source.num_vars())
                        .map(|i| cut[2 * i])
                        .collect::<Vec<_>>()
                );
                let index = assignment
                    .iter()
                    .enumerate()
                    .fold(0, |index, (i, &bit)| index | (usize::from(bit) << i));
                decoded[index] = true;
            }
            Err(_) => assert!(!certificate),
        }
    }
    for (mask, &has_extension) in decoded.iter().enumerate() {
        let assignment = (0..source.num_vars())
            .map(|i| mask & (1 << i) != 0)
            .collect();
        assert_eq!(has_extension, source.evaluate(&assignment).unwrap().0);
    }
    assert_eq!(
        AggregateReductionResult::extract_value(&reduction, crate::types::Max(Some(best))).0,
        decoded.iter().any(|&valid| valid)
    );
    assert!(!AggregateReductionResult::extract_value(&reduction, crate::types::Max(None)).0);
    assert!(reduction
        .extract_solution(&vec![false; target.num_vertices() + 1])
        .is_err());
}

#[test]
fn test_naesatisfiability_to_maxcut_all_short_clauses_and_cuts() {
    check_every_cut(&NAESatisfiability::new(0, vec![]));
    for n in 1..=3 {
        check_every_cut(&NAESatisfiability::new(n, vec![]));
        let literals: Vec<_> = (1..=n as i64).flat_map(|i| [i, -i]).collect();
        for length in 2..=4 {
            for mut rank in 0..literals.len().pow(length) {
                let clause = (0..length)
                    .map(|_| {
                        let literal = literals[rank % literals.len()];
                        rank /= literals.len();
                        literal
                    })
                    .collect();
                check_every_cut(&NAESatisfiability::new(n, vec![CNFClause::new(clause)]));
            }
        }
    }
}

#[test]
fn test_naesatisfiability_to_maxcut_long_clause_interactions() {
    // Former clique reward tied the feasible assignment 0001 with invalid 0011.
    let source = NAESatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3, 4]),
            CNFClause::new(vec![1, -2]),
            CNFClause::new(vec![2, -3]),
        ],
    );
    check_every_cut(&source);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i64>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.feasible_cut, 26);
    for clauses in [
        vec![vec![1, 1, 1, 1, 1]],
        vec![vec![1, 2, 3, 1, 2], vec![1, -2], vec![2, -3]],
        vec![vec![1, -1, 2, -2, 3, 3]],
        vec![vec![1, 2], vec![1, -2]],
    ] {
        check_every_cut(&NAESatisfiability::new(
            3,
            clauses.into_iter().map(CNFClause::new).collect(),
        ));
    }
}

#[test]
fn test_naesatisfiability_to_maxcut_numeric_limits() {
    assert_eq!(
        nae_maxcut_parameters(4, [4, 2, 2].into_iter()).unwrap(),
        (10, 13, 4, 26)
    );
    assert_eq!(
        nae_maxcut_parameters(0, [].into_iter()).unwrap(),
        (0, 0, 1, 0)
    );
    assert!(matches!(
        nae_maxcut_parameters(1, (0..usize::MAX).map(|_| 2)),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
    for (n, lengths) in [
        (usize::MAX, vec![4]),
        (usize::MAX, vec![]),
        (0, vec![usize::MAX]),
        (0, vec![usize::MAX / 3, usize::MAX / 3]),
        (2_147_483_648, vec![]),
        (1, vec![1_431_655_768]),
    ] {
        assert!(matches!(
            nae_maxcut_parameters(n, lengths.into_iter()),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
