use problemreductions::models::algebraic::SimultaneousIncongruences;
use problemreductions::models::formula::{CNFClause, KSatisfiability};
use problemreductions::rules::{ReduceTo, ReductionResult};
use problemreductions::solvers::BruteForce;
use problemreductions::variant::K3;
use problemreductions::Problem;

#[test]
fn test_ksatisfiability_to_simultaneous_incongruences_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, 3]),
        ],
    );

    let reduction = ReduceTo::<SimultaneousIncongruences>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.lcm_moduli(), 105);
    assert_eq!(target.num_pairs(), 11);

    let solver = BruteForce::new();
    let target_solution = solver
        .find_witness(target)
        .expect("target should be satisfiable");
    let extracted = reduction.extract_solution(&target_solution);

    assert!(source.evaluate(&extracted));
}
