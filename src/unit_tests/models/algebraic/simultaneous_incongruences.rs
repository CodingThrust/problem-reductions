use crate::models::algebraic::SimultaneousIncongruences;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn example_problem() -> SimultaneousIncongruences {
    // pairs: [(2,2),(1,3),(2,5),(3,7)] — lcm=210, x=5 is a solution
    SimultaneousIncongruences::new(vec![(2, 2), (1, 3), (2, 5), (3, 7)]).unwrap()
}

fn covering_system() -> SimultaneousIncongruences {
    // Erdős covering system: {0 mod 2, 0 mod 3, 1 mod 4, 5 mod 6, 7 mod 12}
    // This covers all integers — note this uses 0-based residues but our
    // constructor requires 1 ≤ aᵢ ≤ bᵢ, so we use aᵢ=bᵢ to represent ≡0.
    // A simpler unsatisfiable instance: two pairs that together cover everything.
    // x≢2 (mod 2) means x odd, and x≢1 (mod 2) means x even — these together
    // leave no valid x.
    SimultaneousIncongruences::new(vec![(2, 2), (1, 2)]).unwrap()
}

#[test]
fn test_simultaneous_incongruences_creation_and_accessors() {
    let p = example_problem();
    assert_eq!(p.num_pairs(), 4);
    assert_eq!(p.pairs(), &[(2, 2), (1, 3), (2, 5), (3, 7)]);
    // lcm(2,3,5,7) = 210
    assert_eq!(p.lcm_moduli(), 210);
    assert_eq!(p.dims(), vec![210]);
    assert_eq!(p.num_variables(), 1);
    assert_eq!(
        <SimultaneousIncongruences as Problem>::NAME,
        "SimultaneousIncongruences"
    );
    assert_eq!(<SimultaneousIncongruences as Problem>::variant(), vec![]);
}

#[test]
fn test_simultaneous_incongruences_evaluate_yes() {
    let p = example_problem();
    // x=5: 5%2=1≠0(=2%2), 5%3=2≠1, 5%5=0≠2, 5%7=5≠3 ✓
    assert_eq!(p.evaluate(&[5]), Or(true));
    // x=1: 1%2=1≠0(=2%2), 1%3=1=1 — fails for pair (1,3)
    assert_eq!(p.evaluate(&[1]), Or(false));
}

#[test]
fn test_simultaneous_incongruences_evaluate_no() {
    let p = covering_system();
    // pairs (2,2) and (1,2): together require x≡0 (mod 2) AND x≡1 (mod 2),
    // which is impossible.
    let lcm = p.lcm_moduli();
    assert_eq!(lcm, 2);
    // All x in {0,1} should fail
    for x in 0..lcm as usize {
        assert_eq!(p.evaluate(&[x]), Or(false), "expected false for x={x}");
    }
}

#[test]
fn test_simultaneous_incongruences_evaluate_invalid_config() {
    let p = example_problem();
    assert_eq!(p.evaluate(&[]), Or(false));
    assert_eq!(p.evaluate(&[0, 1]), Or(false));
}

#[test]
fn test_simultaneous_incongruences_empty_pairs() {
    let p = SimultaneousIncongruences::new(vec![]).unwrap();
    assert_eq!(p.num_pairs(), 0);
    assert_eq!(p.lcm_moduli(), 1);
    assert_eq!(p.dims(), vec![1]);
    // Any x (here x=0) satisfies vacuously
    assert_eq!(p.evaluate(&[0]), Or(true));
}

#[test]
fn test_simultaneous_incongruences_brute_force_finds_witness() {
    let p = example_problem();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&p).unwrap();
    assert_eq!(p.evaluate(&witness), Or(true));
}

#[test]
fn test_simultaneous_incongruences_brute_force_no_witness() {
    let p = covering_system();
    let solver = BruteForce::new();
    assert!(solver.find_witness(&p).is_none());
}

#[test]
fn test_simultaneous_incongruences_serialization() {
    let p = example_problem();
    let json = serde_json::to_value(&p).unwrap();
    assert_eq!(
        json,
        serde_json::json!({"pairs": [[2,2],[1,3],[2,5],[3,7]]})
    );
    let restored: SimultaneousIncongruences = serde_json::from_value(json).unwrap();
    assert_eq!(restored.pairs(), p.pairs());
}

#[test]
fn test_simultaneous_incongruences_deserialization_rejects_invalid() {
    // b=0
    let r: Result<SimultaneousIncongruences, _> =
        serde_json::from_value(serde_json::json!({"pairs": [[1,0]]}));
    assert!(r.is_err());
    // a=0
    let r: Result<SimultaneousIncongruences, _> =
        serde_json::from_value(serde_json::json!({"pairs": [[0,2]]}));
    assert!(r.is_err());
    // a > b
    let r: Result<SimultaneousIncongruences, _> =
        serde_json::from_value(serde_json::json!({"pairs": [[3,2]]}));
    assert!(r.is_err());
}

#[test]
fn test_simultaneous_incongruences_paper_example() {
    // Canonical paper example: pairs [(2,2),(1,3),(2,5),(3,7)], x=5 is a solution
    let p = SimultaneousIncongruences::new(vec![(2, 2), (1, 3), (2, 5), (3, 7)]).unwrap();
    assert_eq!(p.evaluate(&[5]), Or(true));

    let solver = BruteForce::new();
    let witness = solver.find_witness(&p).unwrap();
    assert_eq!(p.evaluate(&witness), Or(true));
}
