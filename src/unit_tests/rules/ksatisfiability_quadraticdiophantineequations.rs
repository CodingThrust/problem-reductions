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
    let reduction = ReduceTo::<QuadraticDiophantineEquations>::reduce_to(&source);

    let solver = BruteForce::new();
    let target_solution = solver
        .find_witness(reduction.target_problem())
        .expect("target should be satisfiable");

    assert_eq!(
        reduction.target_problem().evaluate(&target_solution),
        Or(true)
    );

    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(source.evaluate(&extracted), Or(true));
}

#[test]
fn test_ksatisfiability_to_quadraticdiophantineequations_yes_vector_matches_reference() {
    let source = canonical_source();
    let reduction = ReduceTo::<QuadraticDiophantineEquations>::reduce_to(&source);
    let target = reduction.target_problem();

    let target_config = target
        .encode_witness(&canonical_witness())
        .expect("reference witness must fit target encoding");

    assert_eq!(target.evaluate(&target_config), Or(true));

    let extracted = reduction.extract_solution(&target_config);
    assert_eq!(extracted, vec![1, 0, 0]);
    assert_eq!(source.evaluate(&extracted), Or(true));
}
