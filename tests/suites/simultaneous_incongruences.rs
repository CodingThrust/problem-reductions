use problemreductions::models::algebraic::SimultaneousIncongruences;
use problemreductions::solvers::BruteForce;
use problemreductions::traits::Problem;

#[test]
fn test_simultaneous_incongruences_issue_example() {
    let problem = SimultaneousIncongruences::new(vec![2, 3, 5, 7], vec![0, 1, 2, 3], 210);

    assert_eq!(problem.moduli(), &[2, 3, 5, 7]);
    assert_eq!(problem.residues(), &[0, 1, 2, 3]);
    assert_eq!(problem.bound(), 210);
    assert_eq!(problem.num_incongruences(), 4);
    assert_eq!(problem.dims(), vec![210]);
    assert!(problem.evaluate(&[4]));
    assert!(!problem.evaluate(&[2]));
}

#[test]
fn test_simultaneous_incongruences_solver_finds_witness() {
    let problem = SimultaneousIncongruences::new(vec![2, 3, 5, 7], vec![0, 1, 2, 3], 210);
    let solver = BruteForce::new();

    let witness = solver.find_witness(&problem);

    assert_eq!(witness, Some(vec![4]));
}
