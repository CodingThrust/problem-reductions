use crate::models::formula::NonTautology;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_non_tautology_creation() {
    let problem = NonTautology::new(3, vec![vec![1, -2], vec![-1, 3]]);

    assert_eq!(problem.num_vars(), 3);
    assert_eq!(problem.num_disjuncts(), 2);
    assert_eq!(problem.disjuncts(), &[vec![1, -2], vec![-1, 3]]);
    assert_eq!(problem.dims(), vec![2, 2, 2]);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_non_tautology_evaluate_marks_falsifying_assignments() {
    let problem = NonTautology::new(2, vec![vec![1, 2], vec![-1, -2]]);

    assert_eq!(problem.evaluate(&[1, 0]), Or(true));
    assert_eq!(problem.evaluate(&[0, 1]), Or(true));
    assert_eq!(problem.evaluate(&[1, 1]), Or(false));
    assert_eq!(problem.evaluate(&[0, 0]), Or(false));
}

#[test]
fn test_non_tautology_solver() {
    let solver = BruteForce::new();

    let non_tautology = NonTautology::new(2, vec![vec![1, 2], vec![-1, -2]]);
    let witnesses = solver.find_all_witnesses(&non_tautology);
    assert_eq!(witnesses.len(), 2);
    assert!(witnesses.contains(&vec![1, 0]));
    assert!(witnesses.contains(&vec![0, 1]));

    let tautology = NonTautology::new(1, vec![vec![1], vec![-1]]);
    assert_eq!(solver.find_witness(&tautology), None);
}

#[test]
fn test_non_tautology_serialization() {
    let problem = NonTautology::new(2, vec![vec![1, -2], vec![-1]]);
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: NonTautology = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip.num_vars(), 2);
    assert_eq!(round_trip.disjuncts(), &[vec![1, -2], vec![-1]]);
}
