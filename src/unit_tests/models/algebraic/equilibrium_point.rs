use crate::models::algebraic::EquilibriumPoint;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

/// Canonical 3-player game from the issue.
/// F1 = x1*x2*x3, F2 = (1-x1)*x2, F3 = x1*(1-x3), M_i = {0, 1}.
fn canonical_problem() -> EquilibriumPoint {
    let polynomials = vec![
        vec![vec![0, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]],
        vec![vec![1, -1, 0, 0], vec![0, 0, 1, 0]],
        vec![vec![0, 1, 0, 0], vec![1, 0, 0, -1]],
    ];
    let range_sets = vec![vec![0, 1], vec![0, 1], vec![0, 1]];
    EquilibriumPoint::new(polynomials, range_sets).unwrap()
}

/// Simple 2-player coordination game: F1 = F2 = x1*x2.
/// Both (0,0) and (1,1) are equilibria (payoff 0 each with M_i = {0,1}).
/// Actually F1(0,0)=0, dev to (1,0): F1(1,0)=0 — no strict improvement.
/// So (0,0) is an equilibrium trivially (no player can strictly improve).
fn coordination_problem() -> EquilibriumPoint {
    // F1 = x1*x2: factors [[0,1,0,0],[0,0,1,0]] (but only constant + 2 player coeffs).
    // Wait: 2 players → factor len = 3.
    let polynomials = vec![
        vec![vec![0, 1, 0], vec![0, 0, 1]], // F1 = x1 * x2
        vec![vec![0, 1, 0], vec![0, 0, 1]], // F2 = x1 * x2
    ];
    let range_sets = vec![vec![0, 1], vec![0, 1]];
    EquilibriumPoint::new(polynomials, range_sets).unwrap()
}

/// A 2-player constant-sum game: F1 = 1 (constant), F2 = 1 (constant).
/// Every config is a Nash equilibrium (no one can improve).
fn trivial_equilibrium_problem() -> EquilibriumPoint {
    let polynomials = vec![
        vec![vec![1, 0, 0]], // F1 = 1
        vec![vec![1, 0, 0]], // F2 = 1
    ];
    let range_sets = vec![vec![0, 1], vec![0, 1]];
    EquilibriumPoint::new(polynomials, range_sets).unwrap()
}

#[test]
fn test_equilibrium_point_creation_and_accessors() {
    let p = canonical_problem();
    assert_eq!(p.num_players(), 3);
    assert_eq!(p.polynomials().len(), 3);
    assert_eq!(p.range_sets().len(), 3);
    assert_eq!(p.range_sets()[0], vec![0, 1]);
    assert_eq!(p.range_sets()[1], vec![0, 1]);
    assert_eq!(p.range_sets()[2], vec![0, 1]);
    assert_eq!(p.dims(), vec![2, 2, 2]);
    assert_eq!(p.num_variables(), 3);
    assert_eq!(<EquilibriumPoint as Problem>::NAME, "EquilibriumPoint");
    assert_eq!(<EquilibriumPoint as Problem>::variant(), vec![]);
}

#[test]
fn test_equilibrium_point_evaluate_canonical_equilibrium() {
    let p = canonical_problem();
    // config [0,1,0] → (0,1,0) is the known equilibrium.
    assert_eq!(p.evaluate(&[0, 1, 0]), Or(true));
}

#[test]
fn test_equilibrium_point_evaluate_non_equilibria() {
    let p = canonical_problem();
    // (1,1,1): F1=1. Dev player1 to 0: F1(0,1,1)=0 < 1 — no. Dev player2 to 0: F1(1,0,1)=0 <1 — no. Dev player3 to 0: F1(1,1,0)=0 <1 — no.
    // F2=(1-1)*1=0. Dev player2 to 0: F2(1,0,1)=0 — no improvement. Player1 dev to 0: F2(0,1,1)=1*1=1 > 0 → NOT equilibrium for player 1 in F2?
    // Wait F2 is player-2's payoff. Let me recheck: player 1 can deviate but that changes F2 too.
    // Actually equilibrium condition: for player i, no y'_i in M_i gives F_i(y) < F_i(y with y_i=y').
    // At (1,1,1): F2(1,1,1)=(1-1)*1=0. Dev player2 to 0: F2(1,0,1)=(1-1)*0=0 — no improvement.
    //             But dev player1 (player 1) only affects F1! F1(1,1,1)=1*1*1=1, dev to 0: F1(0,1,1)=0*1*1=0 — no improvement for player1.
    // F3(1,1,1)=1*(1-1)=0. Dev player3 to 0: F3(1,1,0)=1*(1-0)=1 > 0 → player 3 can improve! NOT equilibrium.
    assert_eq!(p.evaluate(&[1, 1, 1]), Or(false));

    // (0,0,0): F2(0,0,0)=(1-0)*0=0. Dev player2 to 1: F2(0,1,0)=(1-0)*1=1 > 0 → NOT equilibrium.
    assert_eq!(p.evaluate(&[0, 0, 0]), Or(false));
}

#[test]
fn test_equilibrium_point_invalid_config_lengths() {
    let p = canonical_problem();
    assert_eq!(p.evaluate(&[]), Or(false));
    assert_eq!(p.evaluate(&[0, 1]), Or(false));
    assert_eq!(p.evaluate(&[0, 1, 0, 0]), Or(false));
}

#[test]
fn test_equilibrium_point_brute_force_finds_witness() {
    let solver = BruteForce::new();
    let p = canonical_problem();
    let witness = solver.find_witness(&p).unwrap();
    assert_eq!(p.evaluate(&witness), Or(true));
}

#[test]
fn test_equilibrium_point_coordination_game() {
    let p = coordination_problem();
    // (0,0): F1(0,0)=0*0=0. Dev p1 to 1: F1(1,0)=1*0=0 — no improvement. Dev p2 to 1: F2(0,1)=0*1=0 — no improvement.
    // So (0,0) is an equilibrium.
    assert_eq!(p.evaluate(&[0, 0]), Or(true));
    // (1,1): F1(1,1)=1. Dev p1 to 0: F1(0,1)=0 < 1 — no improvement. Dev p2 to 0: F2(1,0)=0 < 1 — no improvement.
    // (1,1) is also an equilibrium.
    assert_eq!(p.evaluate(&[1, 1]), Or(true));
    // (0,1): F1(0,1)=0*1=0. Dev p1 to 1: F1(1,1)=1 > 0 → NOT equilibrium.
    assert_eq!(p.evaluate(&[0, 1]), Or(false));
}

#[test]
fn test_equilibrium_point_trivial_constant_payoffs() {
    let p = trivial_equilibrium_problem();
    // Every config is an equilibrium since payoff is always 1.
    assert_eq!(p.evaluate(&[0, 0]), Or(true));
    assert_eq!(p.evaluate(&[0, 1]), Or(true));
    assert_eq!(p.evaluate(&[1, 0]), Or(true));
    assert_eq!(p.evaluate(&[1, 1]), Or(true));
}

#[test]
fn test_equilibrium_point_serialization() {
    let p = canonical_problem();
    let json = serde_json::to_value(&p).unwrap();
    assert!(json.get("polynomials").is_some());
    assert!(json.get("range_sets").is_some());

    let restored: EquilibriumPoint = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_players(), p.num_players());
    assert_eq!(restored.range_sets(), p.range_sets());
    assert_eq!(restored.polynomials(), p.polynomials());
}

#[test]
fn test_equilibrium_point_deserialization_rejects_invalid() {
    // Mismatched polynomials and range_sets lengths.
    let r: Result<EquilibriumPoint, _> = serde_json::from_value(serde_json::json!({
        "polynomials": [[[0, 1, 0]]],
        "range_sets": [[0, 1], [0, 1]]
    }));
    assert!(r.is_err());

    // Empty range_set.
    let r: Result<EquilibriumPoint, _> = serde_json::from_value(serde_json::json!({
        "polynomials": [[[1, 0]]],
        "range_sets": [[]]
    }));
    assert!(r.is_err());

    // Wrong factor length (should be n+1 = 2, got 3).
    let r: Result<EquilibriumPoint, _> = serde_json::from_value(serde_json::json!({
        "polynomials": [[[0, 1, 2, 3]]],
        "range_sets": [[0, 1]]
    }));
    assert!(r.is_err());
}

#[test]
fn test_equilibrium_point_paper_example() {
    // Canonical example: config [0,1,0] is the equilibrium.
    let p = canonical_problem();
    assert_eq!(p.evaluate(&[0, 1, 0]), Or(true));

    let solver = BruteForce::new();
    let witness = solver.find_witness(&p).unwrap();
    assert_eq!(p.evaluate(&witness), Or(true));
}

#[test]
fn test_equilibrium_point_validation_panics() {
    // polynomials length != range_sets length.
    assert!(
        EquilibriumPoint::new(vec![vec![vec![0, 1, 0]]], vec![vec![0, 1], vec![0, 1]],).is_err()
    );

    // Empty range_set.
    assert!(EquilibriumPoint::new(vec![vec![vec![1, 0]]], vec![vec![]]).is_err());

    // Factor has wrong length.
    assert!(EquilibriumPoint::new(vec![vec![vec![0, 1, 2, 3]]], vec![vec![0, 1]],).is_err());
}

#[test]
fn test_equilibrium_point_single_player() {
    // 1-player game: F1 = x1. M1 = {0, 2}. Equilibrium when player picks max.
    // F1(0) = 0. Deviation to 2: F1(2) = 2 > 0 → config [0] NOT equilibrium.
    // F1(2) = 2. No deviation improves. config [1] IS equilibrium.
    let polynomials = vec![vec![vec![0, 1]]];
    let range_sets = vec![vec![0, 2]];
    let p = EquilibriumPoint::new(polynomials, range_sets).unwrap();
    assert_eq!(p.evaluate(&[0]), Or(false));
    assert_eq!(p.evaluate(&[1]), Or(true));
}
