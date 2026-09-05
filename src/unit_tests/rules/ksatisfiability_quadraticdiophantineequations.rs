use super::*;
use crate::models::algebraic::QuadraticDiophantineEquations;
use crate::models::formula::CNFClause;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;
use crate::variant::K3;

fn trivial_source() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(2, vec![CNFClause::new(vec![1, -1, 2])])
}

#[test]
fn test_ksatisfiability_to_quadraticdiophantineequations_closed_loop() {
    let source = trivial_source();
    let reduction = ReduceTo::<QuadraticDiophantineEquations>::reduce_to(&source)
        .expect("reduction should succeed");

    let solver = BruteForce::new();
    let target_solution = solver
        .solve(reduction.target_problem())
        .unwrap()
        .expect("target should be satisfiable");

    assert_eq!(
        reduction
            .target_problem()
            .evaluate(&target_solution)
            .unwrap(),
        Or(true)
    );

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_ksatisfiability_to_quadraticdiophantineequations_yes_vector_matches_reference() {
    let source = canonical_source();
    let reduction = ReduceTo::<QuadraticDiophantineEquations>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    let target_config = canonical_witness();

    assert_eq!(target.evaluate(&target_config).unwrap(), Or(true));

    let extracted = reduction.extract_solution(&target_config).unwrap();
    assert_eq!(extracted, vec![true, false, false]);
    assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
}
