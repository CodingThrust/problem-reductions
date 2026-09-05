use crate::models::misc::NonLivenessFreePetriNet;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Or;

/// Chain net: t0 moves token s0->s1, t1 moves s1->s2, t2 moves s2->s3, then deadlock.
fn chain_net() -> NonLivenessFreePetriNet {
    NonLivenessFreePetriNet::new(
        4,
        3,
        vec![(0, 0), (1, 1), (2, 2)],
        vec![(0, 1), (1, 2), (2, 3)],
        vec![1, 0, 0, 0],
    )
    .unwrap()
}

/// Cycle net: token oscillates between two places, both transitions always fireable.
fn cycle_net() -> NonLivenessFreePetriNet {
    NonLivenessFreePetriNet::new(2, 2, vec![(0, 0), (1, 1)], vec![(0, 1), (1, 0)], vec![1, 0])
        .unwrap()
}

#[test]
fn test_non_liveness_free_petri_net_basic() {
    let problem = chain_net();
    assert_eq!(problem.num_places(), 4);
    assert_eq!(problem.num_transitions(), 3);
    assert_eq!(problem.num_arcs(), 6);
    assert_eq!(problem.initial_token_sum(), 1);
    assert_eq!(problem.dimensions(), vec![2; 3]);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(
        <NonLivenessFreePetriNet as Problem>::NAME,
        "NonLivenessFreePetriNet"
    );
    assert_eq!(<NonLivenessFreePetriNet as Problem>::variant(), vec![]);
}

#[test]
fn test_non_liveness_chain_net_is_not_live() {
    let problem = chain_net();
    // All transitions are dead: after the chain fires, nothing can fire again.
    // Selecting all transitions should yield true.
    assert_eq!(problem.evaluate(&vec![true, true, true]).unwrap(), Or(true));
    // Selecting just one transition should also yield true.
    assert_eq!(
        problem.evaluate(&vec![true, false, false]).unwrap(),
        Or(true)
    );
    assert_eq!(
        problem.evaluate(&vec![false, true, false]).unwrap(),
        Or(true)
    );
    assert_eq!(
        problem.evaluate(&vec![false, false, true]).unwrap(),
        Or(true)
    );
    // Selecting no transition yields false (no claimed dead transition).
    assert_eq!(
        problem.evaluate(&vec![false, false, false]).unwrap(),
        Or(false)
    );
}

#[test]
fn test_non_liveness_cycle_net_is_live() {
    let problem = cycle_net();
    // In the cycle net, both transitions can always fire. No transition is dead.
    assert_eq!(problem.evaluate(&vec![true, true]).unwrap(), Or(false));
    assert_eq!(problem.evaluate(&vec![true, false]).unwrap(), Or(false));
    assert_eq!(problem.evaluate(&vec![false, true]).unwrap(), Or(false));
    assert_eq!(problem.evaluate(&vec![false, false]).unwrap(), Or(false));
}

#[test]
fn test_non_liveness_solver_finds_witness_chain() {
    let problem = chain_net();
    let solver = BruteForce::new();
    let witness = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&witness).unwrap(), Or(true));
}

#[test]
fn test_non_liveness_solver_no_witness_cycle() {
    let problem = cycle_net();
    let solver = BruteForce::new();
    assert!(solver.solve(&problem).unwrap().is_none());
}

#[test]
fn test_non_liveness_wrong_config_length() {
    let problem = chain_net();
    assert!(matches!(
        problem.evaluate(&vec![true, false]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![true, false, false, false]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_non_liveness_serialization_round_trip() {
    let problem = chain_net();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "num_places": 4,
            "num_transitions": 3,
            "place_to_transition": [[0, 0], [1, 1], [2, 2]],
            "transition_to_place": [[0, 1], [1, 2], [2, 3]],
            "initial_marking": [1, 0, 0, 0],
        })
    );

    let restored: NonLivenessFreePetriNet = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_places(), problem.num_places());
    assert_eq!(restored.num_transitions(), problem.num_transitions());
    assert_eq!(
        restored.place_to_transition(),
        problem.place_to_transition()
    );
    assert_eq!(
        restored.transition_to_place(),
        problem.transition_to_place()
    );
    assert_eq!(restored.initial_marking(), problem.initial_marking());
}

#[test]
fn test_non_liveness_deserialization_rejects_invalid() {
    let invalid_cases = [
        // Zero places
        serde_json::json!({
            "num_places": 0,
            "num_transitions": 1,
            "place_to_transition": [],
            "transition_to_place": [],
            "initial_marking": [],
        }),
        // Zero transitions
        serde_json::json!({
            "num_places": 1,
            "num_transitions": 0,
            "place_to_transition": [],
            "transition_to_place": [],
            "initial_marking": [0],
        }),
        // Marking length mismatch
        serde_json::json!({
            "num_places": 2,
            "num_transitions": 1,
            "place_to_transition": [],
            "transition_to_place": [],
            "initial_marking": [0],
        }),
        // Place index out of range
        serde_json::json!({
            "num_places": 2,
            "num_transitions": 1,
            "place_to_transition": [[5, 0]],
            "transition_to_place": [],
            "initial_marking": [0, 0],
        }),
    ];

    for invalid in invalid_cases {
        assert!(serde_json::from_value::<NonLivenessFreePetriNet>(invalid).is_err());
    }
}

#[test]
fn test_non_liveness_rejects_zero_places() {
    assert!(NonLivenessFreePetriNet::new(0, 1, vec![], vec![], vec![]).is_err());
}

#[test]
fn test_non_liveness_rejects_zero_transitions() {
    assert!(NonLivenessFreePetriNet::new(1, 0, vec![], vec![], vec![0]).is_err());
}

#[test]
fn test_non_liveness_rejects_marking_length_mismatch() {
    assert!(NonLivenessFreePetriNet::new(2, 1, vec![], vec![], vec![0]).is_err());
}

#[test]
fn test_non_liveness_rejects_free_choice_violation() {
    // t0 has preset {s0}, t1 has preset {s0, s1} -- they share s0 but have different presets
    assert!(NonLivenessFreePetriNet::new(
        2,
        2,
        vec![(0, 0), (0, 1), (1, 1)],
        vec![(0, 0), (1, 1)],
        vec![1, 1],
    )
    .is_err());
}
