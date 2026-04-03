use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_simultaneous_incongruences_creation() {
    let problem = SimultaneousIncongruences::new(vec![2, 3, 5, 7], vec![0, 1, 2, 3], 210);

    assert_eq!(problem.moduli(), &[2, 3, 5, 7]);
    assert_eq!(problem.residues(), &[0, 1, 2, 3]);
    assert_eq!(problem.bound(), 210);
    assert_eq!(problem.num_incongruences(), 4);
    assert_eq!(problem.dims(), vec![210]);
    assert_eq!(problem.num_variables(), 1);
}

#[test]
fn test_simultaneous_incongruences_evaluate() {
    let problem = SimultaneousIncongruences::new(vec![2, 3, 5, 7], vec![0, 1, 2, 3], 210);

    assert!(problem.evaluate(&[4]));
    assert!(!problem.evaluate(&[2]));
    assert!(!problem.evaluate(&[209]));
    assert!(!problem.evaluate(&[]));
    assert!(!problem.evaluate(&[210]));
}

#[test]
fn test_simultaneous_incongruences_solver_and_serialization() {
    let problem = SimultaneousIncongruences::new(vec![2, 3, 5, 7], vec![0, 1, 2, 3], 210);
    let solver = BruteForce::new();

    assert_eq!(solver.find_witness(&problem), Some(vec![4]));

    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: SimultaneousIncongruences = serde_json::from_str(&json).unwrap();
    assert_eq!(round_trip.moduli(), problem.moduli());
    assert_eq!(round_trip.residues(), problem.residues());
    assert_eq!(round_trip.bound(), problem.bound());
}

#[test]
fn test_simultaneous_incongruences_paper_example() {
    let problem = SimultaneousIncongruences::new(vec![2, 3, 5, 7], vec![0, 1, 2, 3], 210);
    let solver = BruteForce::new();

    assert!(problem.evaluate(&[4]));
    assert_eq!(solver.find_witness(&problem), Some(vec![4]));
}
