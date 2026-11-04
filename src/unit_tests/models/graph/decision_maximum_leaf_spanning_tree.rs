use crate::models::{Decision, MaximumLeafSpanningTree};
use crate::registry::construct_dyn;
use crate::solvers::{BruteForce, BruteForceProblem};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Or;
use serde_json::json;
use std::collections::BTreeMap;

type LeafDecision = Decision<MaximumLeafSpanningTree<SimpleGraph>>;

fn star(bound: i64) -> LeafDecision {
    Decision::new(MaximumLeafSpanningTree::new(SimpleGraph::star(4)), bound)
}

#[test]
fn test_decision_maximum_leaf_spanning_tree_creation() {
    let variant = BTreeMap::from([("graph".to_owned(), "SimpleGraph".to_owned())]);
    let problem = construct_dyn(
        "DecisionMaximumLeafSpanningTree",
        &variant,
        json!({"graph": {"num_vertices": 4, "edges": [[0,1], [0,2], [0,3]]}, "bound": 3}),
    )
    .unwrap();
    assert_eq!(problem.serialize_json()["bound"], 3);
    assert_eq!(
        problem.serialize_json()["inner"]["graph"]["num_vertices"],
        4
    );
}

#[test]
fn test_decision_maximum_leaf_spanning_tree_evaluation() {
    let witness = vec![true; 3];
    for (bound, expected) in [
        (i64::MIN, true),
        (2, true),
        (3, true),
        (4, false),
        (i64::MAX, false),
    ] {
        let problem = star(bound);
        assert_eq!(problem.evaluate(&witness).unwrap(), Or(expected));
        assert_eq!(*problem.bound(), bound);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.dimensions(), vec![2; 3]);
        assert_eq!(problem.parameters(), problem.inner().parameters());
    }
    assert_eq!(
        star(0).evaluate(&vec![true, false, true]).unwrap(),
        Or(false)
    );
    assert!(star(0).evaluate(&vec![true; 2]).is_err());
    let disconnected = Decision::new(
        MaximumLeafSpanningTree::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 0)])),
        -1,
    );
    assert_eq!(disconnected.evaluate(&vec![true; 3]).unwrap(), Or(false));
    let pair = Decision::new(MaximumLeafSpanningTree::new(SimpleGraph::path(2)), 2);
    assert_eq!(pair.evaluate(&vec![true]).unwrap(), Or(true));
}

#[test]
fn test_decision_maximum_leaf_spanning_tree_serialization() {
    let problem = star(3);
    let encoded = serde_json::to_value(&problem).unwrap();
    let restored: LeafDecision = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(
        restored.inner().graph().edges(),
        problem.inner().graph().edges()
    );
    assert_eq!(restored.evaluate(&vec![true; 3]).unwrap(), Or(true));
    assert_eq!(*restored.bound(), 3);
    assert_eq!(encoded["inner"]["graph"]["num_vertices"], 4);
    let variant = BTreeMap::from([("graph".to_owned(), "SimpleGraph".to_owned())]);
    for n in [0, 1] {
        let flat = json!({"graph": {"num_vertices": n, "edges": []}, "bound": 0});
        assert!(construct_dyn("DecisionMaximumLeafSpanningTree", &variant, flat).is_err());
        assert!(
            serde_json::from_value::<LeafDecision>(
                json!({"inner": {"graph": {"num_vertices": n, "edges": []}}, "bound": 0})
            )
            .is_err()
        );
        assert!(MaximumLeafSpanningTree::try_new(SimpleGraph::new(n, vec![])).is_err());
    }
    assert!(serde_json::from_value::<LeafDecision>(json!({"inner": {"graph": {"num_vertices": 2, "edges": [[0,1]]}}, "bound": 9223372036854775808_u64})).is_err());
}

#[test]
fn test_decision_maximum_leaf_spanning_tree_solver() {
    for bound in [2, 3, 4] {
        let problem = star(bound);
        let solution = BruteForce::new().solve(&problem).unwrap();
        assert_eq!(solution.is_some(), bound <= 3);
        if let Some(witness) = solution {
            assert_eq!(problem.evaluate(&witness).unwrap(), Or(true));
        }
    }
}

#[test]
fn test_decision_maximum_leaf_spanning_tree_ilp_solver() {
    use crate::solvers::ILPSolver;
    for bound in [2, 3, 4] {
        let problem = star(bound);
        let result = ILPSolver::new().solve(&problem);
        if bound <= 3 {
            let witness = result.unwrap();
            assert_eq!(problem.evaluate(&witness).unwrap(), Or(true));
        } else {
            assert!(matches!(
                result,
                Err(crate::solvers::ILPSolveError::Infeasible)
            ));
        }
    }
}

#[test]
#[cfg(feature = "example-db")]
fn test_decision_maximum_leaf_spanning_tree_paper_example() {
    let spec = super::decision_canonical_model_example_specs()
        .pop()
        .unwrap();
    let instance = spec.instance.serialize_json();
    let problem: LeafDecision = serde_json::from_value(instance).unwrap();
    let witness: Vec<bool> = serde_json::from_value(spec.optimal_config).unwrap();
    assert_eq!(
        problem.inner().graph().edges(),
        vec![(0, 1), (0, 2), (0, 3)]
    );
    assert_eq!(witness, vec![true; 3]);
    assert_eq!(*problem.bound(), 3);
    assert_eq!(problem.evaluate(&witness).unwrap(), Or(true));
    assert_eq!(spec.optimal_value, json!(true));
}
