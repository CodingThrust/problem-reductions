use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::registry::load_dyn;
use crate::solvers::{solve_deterministically, SolverExecution, SolverRequest};
use crate::traits::Problem;
use std::collections::BTreeMap;

#[test]
fn deterministic_solver_dispatch_customized_registration_wins_default_dispatch() {
    use crate::models::set::MinimumCardinalityKey;

    let problem = MinimumCardinalityKey::new(3, vec![(vec![0], vec![1, 2])]);
    let loaded = crate::registry::load_dyn(
        MinimumCardinalityKey::NAME,
        &BTreeMap::new(),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let result = solve_deterministically(&loaded, SolverRequest::Default).unwrap();
    assert_eq!(
        result.solver,
        SolverExecution::Customized {
            implementation: "fd-minimum-cardinality-key"
        }
    );

    let explicit = solve_deterministically(&loaded, SolverRequest::Customized).unwrap();
    assert_eq!(explicit, result);
}

#[test]
fn deterministic_solver_dispatch_unregistered_customized_override_is_a_capability_error() {
    use crate::models::graph::MaxCut;
    use crate::topology::SimpleGraph;

    let problem = MaxCut::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32]);
    let loaded = crate::registry::load_dyn(
        MaxCut::<SimpleGraph, i32>::NAME,
        &BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let error = solve_deterministically(&loaded, SolverRequest::Customized).unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::DeterministicSolveError::MissingCustomizedCapability(_)
    ));
}

#[test]
fn deterministic_solver_dispatch_unregistered_ilp_override_is_a_capability_error_without_fallback()
{
    use crate::models::graph::MaxCut;
    use crate::topology::SimpleGraph;

    // MaxCut<i32> has a discoverable graph route toward ILP, but that route is
    // partial for valid negative-weight instances and is intentionally not a
    // registered solver pipeline.
    let problem = MaxCut::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32]);
    let loaded = crate::registry::load_dyn(
        MaxCut::<SimpleGraph, i32>::NAME,
        &BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let default = solve_deterministically(&loaded, SolverRequest::Default).unwrap();
    assert_eq!(default.solver, SolverExecution::BruteForce);
    let error = solve_deterministically(&loaded, SolverRequest::Ilp).unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::DeterministicSolveError::MissingIlpCapability(_)
    ));
}

#[test]
fn deterministic_solver_dispatch_customized_failure_does_not_fall_back() {
    use crate::models::misc::AdditionalKey;

    // {0} is the only candidate key and it is already known, so the registered
    // customized solver has no witness. Brute force can still report the aggregate
    // infeasibility result, which lets this test distinguish fallback from error.
    let problem = AdditionalKey::new(3, vec![(vec![0], vec![1, 2])], vec![0, 1, 2], vec![vec![0]]);
    let loaded = load_dyn(
        AdditionalKey::NAME,
        &BTreeMap::new(),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let error = solve_deterministically(&loaded, SolverRequest::Default).unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::DeterministicSolveError::CustomizedNoSolution { .. }
    ));
    let brute_force = solve_deterministically(&loaded, SolverRequest::BruteForce).unwrap();
    assert_eq!(brute_force.solver, SolverExecution::BruteForce);
    assert!(brute_force.config.is_none());
}

#[test]
fn deterministic_solver_dispatch_direct_ilp_uses_registered_one_node_pipeline() {
    let problem = ILP::<bool>::new(0, vec![], vec![], ObjectiveSense::Minimize);
    let loaded = load_dyn(
        ILP::<bool>::NAME,
        &BTreeMap::from([("variable".to_string(), "bool".to_string())]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let result = solve_deterministically(&loaded, SolverRequest::Default).unwrap();
    assert_eq!(
        result.solver,
        SolverExecution::Ilp {
            reduction_path: vec!["ILP<bool>".to_string()]
        }
    );
    assert_eq!(result.config, Some(vec![]));
}

#[test]
fn deterministic_solver_dispatch_ilp_failure_does_not_fall_back() {
    let problem = ILP::<bool>::new(
        0,
        vec![LinearConstraint::le(vec![], -1.0)],
        vec![],
        ObjectiveSense::Minimize,
    );
    let loaded = load_dyn(
        ILP::<bool>::NAME,
        &BTreeMap::from([("variable".to_string(), "bool".to_string())]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let error = solve_deterministically(&loaded, SolverRequest::Default).unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::DeterministicSolveError::IlpSolve {
            source: crate::solvers::ILPSolveError::Infeasible,
            ..
        }
    ));
    let brute_force = solve_deterministically(&loaded, SolverRequest::BruteForce).unwrap();
    assert_eq!(brute_force.solver, SolverExecution::BruteForce);
    assert!(brute_force.config.is_none());
}

#[test]
fn deterministic_solver_execution_has_stable_tagged_json_contract() {
    assert_eq!(
        serde_json::to_value(SolverExecution::Customized {
            implementation: "customized-id"
        })
        .unwrap(),
        serde_json::json!({"kind": "customized", "implementation": "customized-id"})
    );
    assert_eq!(
        serde_json::to_value(SolverExecution::Ilp {
            reduction_path: vec!["Source".to_string(), "ILP<bool>".to_string()]
        })
        .unwrap(),
        serde_json::json!({
            "kind": "ilp",
            "reduction_path": ["Source", "ILP<bool>"]
        })
    );
    assert_eq!(
        serde_json::to_value(SolverExecution::BruteForce).unwrap(),
        serde_json::json!({"kind": "brute-force"})
    );
}

#[test]
fn deterministic_solver_dispatch_fixed_multihop_pipeline_is_repeatable() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![crate::types::One; 3],
    );
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "One".to_string()),
    ]);
    let loaded = load_dyn(
        MaximumIndependentSet::<SimpleGraph, crate::types::One>::NAME,
        &variant,
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let first = solve_deterministically(&loaded, SolverRequest::Ilp).unwrap();
    let second = solve_deterministically(&loaded, SolverRequest::Ilp).unwrap();
    assert_eq!(first, second);
    let SolverExecution::Ilp { reduction_path } = first.solver else {
        panic!("expected ILP execution metadata");
    };
    assert_eq!(
        reduction_path,
        vec![
            "MaximumIndependentSet<SimpleGraph, One>",
            "MaximumIndependentSet<SimpleGraph, i32>",
            "MaximumSetPacking<i32>",
            "ILP<bool>",
        ]
    );
}

#[test]
fn deterministic_solver_dispatch_customized_default_allows_explicit_ilp_override() {
    use crate::models::graph::RootedTreeArrangement;
    use crate::topology::SimpleGraph;

    let problem = RootedTreeArrangement::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 3);
    let loaded = load_dyn(
        RootedTreeArrangement::<SimpleGraph>::NAME,
        &BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let default = solve_deterministically(&loaded, SolverRequest::Default).unwrap();
    assert!(matches!(default.solver, SolverExecution::Customized { .. }));

    let explicit_ilp = solve_deterministically(&loaded, SolverRequest::Ilp).unwrap();
    assert!(matches!(explicit_ilp.solver, SolverExecution::Ilp { .. }));
    assert_eq!(default.evaluation, explicit_ilp.evaluation);
}

#[test]
fn deterministic_solver_dispatch_repeats_each_available_solver_class() {
    use crate::models::graph::RootedTreeArrangement;
    use crate::topology::SimpleGraph;

    let problem = RootedTreeArrangement::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 3);
    let loaded = load_dyn(
        RootedTreeArrangement::<SimpleGraph>::NAME,
        &BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let mut evaluations = Vec::new();
    for request in [
        SolverRequest::Default,
        SolverRequest::Customized,
        SolverRequest::Ilp,
        SolverRequest::BruteForce,
    ] {
        let first = solve_deterministically(&loaded, request).unwrap();
        let second = solve_deterministically(&loaded, request).unwrap();
        assert_eq!(first, second, "{request:?} changed its witness");
        evaluations.push(first.evaluation);
    }
    assert!(evaluations.windows(2).all(|pair| pair[0] == pair[1]));
}
