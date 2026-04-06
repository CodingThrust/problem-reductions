use crate::models::formula::CNFClause;
use crate::models::formula::KSatisfiability;
use crate::models::graph::AcyclicPartition;
use crate::rules::ReduceTo;
use crate::rules::traits::ReductionResult;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_acyclicpartition_closed_loop() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<AcyclicPartition<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 7);
    assert_eq!(target.num_arcs(), 10);

    let solutions = BruteForce::new().find_all_witnesses(target);
    assert!(!solutions.is_empty());

    for solution in solutions {
        let extracted = reduction.extract_solution(&solution);
        assert!(source.evaluate(&extracted).0);
    }
}
