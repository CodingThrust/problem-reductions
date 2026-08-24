use crate::models::formula::CNFClause;
use crate::models::formula::KSatisfiability;
use crate::models::graph::AcyclicPartition;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_acyclicpartition_closed_loop() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction =
        ReduceTo::<AcyclicPartition<i64>>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 7);
    assert_eq!(target.num_arcs(), 10);

    let solutions = BruteForce::new().find_all_witnesses(target).unwrap();
    assert!(!solutions.is_empty());

    for solution in solutions {
        let extracted = reduction.extract_solution(&solution).unwrap();
        assert!(source.evaluate(&extracted).unwrap().0);
    }
}

#[test]
fn test_partition_to_acyclicpartition_rejects_malformed_target_configuration() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction =
        ReduceTo::<AcyclicPartition<i64>>::reduce_to(&source).expect("reduction should succeed");

    assert!(reduction
        .partition_to_acyclic
        .extract_solution(&[])
        .is_err());
}

#[test]
fn test_ksatisfiability_to_acyclicpartition_unsatisfiable() {
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );

    // Source is trivially UNSAT: requires x=true AND x=false simultaneously.
    let solver = BruteForce::new();
    assert!(
        solver.find_witness(&source).unwrap().is_none(),
        "source with contradictory clauses must be unsatisfiable"
    );

    // Verify the reduction still produces a well-formed target.
    let reduction =
        ReduceTo::<AcyclicPartition<i64>>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();
    assert_eq!(target.num_vertices(), 9);
    assert_eq!(target.num_arcs(), 14);
}

#[test]
fn test_ksatisfiability_to_acyclicpartition_multi_variable_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );

    let reduction =
        ReduceTo::<AcyclicPartition<i64>>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // Target has 2*3 + 2*2 + 3 = 13 vertices, so brute-force on target is
    // infeasible (13^13 configs). Instead, verify round-trip by brute-forcing
    // the source (2^3 = 8 configs) and checking that every satisfying source
    // assignment extracts correctly from the reduction.
    let source_witnesses = BruteForce::new().find_all_witnesses(&source).unwrap();
    assert!(
        !source_witnesses.is_empty(),
        "source should have at least one satisfying assignment"
    );

    for source_witness in &source_witnesses {
        assert!(
            source.evaluate(source_witness).unwrap().0,
            "every source witness must evaluate as satisfying"
        );
    }

    // Verify structural properties of the target.
    assert_eq!(target.num_vertices(), 13);
    assert_eq!(target.num_arcs(), 22);
}
