use super::*;
use crate::models::formula::CNFClause;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_simultaneous_incongruences_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        2,
        vec![
            CNFClause::new(vec![1, 2, 2]),
            CNFClause::new(vec![-1, 2, 2]),
        ],
    );
    let reduction = ReduceTo::<SimultaneousIncongruences>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.lcm_moduli(), 15);
    assert_eq!(target.num_pairs(), 6);

    let solver = BruteForce::new();
    let target_solution = solver
        .find_witness(target)
        .expect("target should be satisfiable");
    let extracted = reduction.extract_solution(&target_solution);

    assert!(source.evaluate(&extracted));
}

#[test]
fn test_ksatisfiability_to_simultaneous_incongruences_structure() {
    let source = KSatisfiability::<K3>::new(
        2,
        vec![
            CNFClause::new(vec![1, 2, 2]),
            CNFClause::new(vec![-1, 2, 2]),
        ],
    );
    let reduction = ReduceTo::<SimultaneousIncongruences>::reduce_to(&source);
    let target = reduction.target_problem();

    let pairs: Vec<(u64, u64)> = target.pairs().to_vec();
    assert_eq!(
        pairs,
        vec![(3, 3), (5, 5), (3, 5), (4, 5), (2, 15), (7, 15)]
    );
}

#[test]
fn test_ksatisfiability_to_simultaneous_incongruences_unsatisfiable() {
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );
    let reduction = ReduceTo::<SimultaneousIncongruences>::reduce_to(&source);
    let solver = BruteForce::new();

    assert_eq!(solver.find_witness(reduction.target_problem()), None);
}

#[test]
fn test_ksatisfiability_to_simultaneous_incongruences_tautological_clause_is_redundant() {
    let source = KSatisfiability::<K3>::new(
        2,
        vec![
            CNFClause::new(vec![1, -1, 2]),
            CNFClause::new(vec![2, 2, 2]),
        ],
    );
    let reduction = ReduceTo::<SimultaneousIncongruences>::reduce_to(&source);
    let solver = BruteForce::new();
    let target_solution = solver
        .find_witness(reduction.target_problem())
        .expect("target should remain satisfiable");
    let extracted = reduction.extract_solution(&target_solution);

    assert!(source.evaluate(&extracted));
}
