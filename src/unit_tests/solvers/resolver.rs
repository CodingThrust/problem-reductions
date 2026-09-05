use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::registry::load_dyn;
use crate::solvers::{solve, SolveOutcome, SolverExecution, SolverRequest};
use crate::traits::Problem;
use std::collections::BTreeMap;

#[test]
fn decision_reductions_check_target_optimum_before_extracting_witness() {
    let variant = BTreeMap::from([("graph".into(), "SimpleGraph".into())]);
    let cases = [
        (
            "HamiltonianCircuit",
            serde_json::json!({"graph": {"num_vertices": 4, "edges": [[0,1],[1,2],[0,2],[2,3]]}}),
            false,
        ),
        (
            "HamiltonianCircuit",
            serde_json::json!({"graph": {"num_vertices": 4, "edges": [[0,1],[1,2],[2,3],[0,3]]}}),
            true,
        ),
        (
            "HamiltonianCircuit",
            serde_json::json!({"graph": {"num_vertices": 3, "edges": [[0,1],[1,2]]}}),
            false,
        ),
        (
            "PartitionIntoCliques",
            serde_json::json!({"graph": {"num_vertices": 3, "edges": []}, "num_cliques": 2}),
            false,
        ),
        (
            "PartitionIntoCliques",
            serde_json::json!({"graph": {"num_vertices": 3, "edges": []}, "num_cliques": 3}),
            true,
        ),
    ];
    for (name, data, expected) in cases {
        let problem = load_dyn(name, &variant, data).unwrap();
        for backend in [
            SolverRequest::BruteForce,
            SolverRequest::Ilp,
            SolverRequest::Default,
        ] {
            match solve(&problem, backend).unwrap().outcome {
                SolveOutcome::Optimal {
                    solution,
                    evaluation,
                } => {
                    assert!(expected, "{name}, {backend:?}");
                    assert_eq!(evaluation, "Or(true)");
                    assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
                }
                SolveOutcome::Infeasible => assert!(!expected, "{name}, {backend:?}"),
            }
        }
    }
}

#[test]
fn hamiltonian_ilp_matches_exhaustive_search_on_small_graphs() {
    let variant = BTreeMap::from([("graph".into(), "SimpleGraph".into())]);
    let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    for mask in 0..64 {
        let selected: Vec<_> = edges
            .iter()
            .enumerate()
            .filter_map(|(i, edge)| (mask & (1 << i) != 0).then_some(edge))
            .collect();
        let problem = load_dyn(
            "HamiltonianCircuit",
            &variant,
            serde_json::json!({"graph": {"num_vertices": 4, "edges": selected}}),
        )
        .unwrap();
        let reference = solve(&problem, SolverRequest::BruteForce).unwrap();
        let actual = solve(&problem, SolverRequest::Ilp).unwrap();
        assert_eq!(
            matches!(actual.outcome, SolveOutcome::Infeasible),
            matches!(reference.outcome, SolveOutcome::Infeasible),
            "graph {mask}"
        );
        if let SolveOutcome::Optimal { solution, .. } = actual.outcome {
            assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
        }
    }
}

#[test]
fn generic_decision_ilp_compares_inner_optimum_with_bound() {
    use crate::models::graph::{
        MinimumDominatingSet, MinimumVertexCover, OptimalLinearArrangement,
    };
    use crate::topology::SimpleGraph;

    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let weighted_variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i64".to_string()),
    ]);
    let cases = [
        (
            "DecisionMinimumVertexCover",
            weighted_variant.clone(),
            serde_json::to_value(MinimumVertexCover::new(graph.clone(), vec![1i64; 3])).unwrap(),
            2,
        ),
        (
            "DecisionMinimumDominatingSet",
            weighted_variant,
            serde_json::to_value(MinimumDominatingSet::new(graph.clone(), vec![1i64; 3])).unwrap(),
            1,
        ),
        (
            "DecisionOptimalLinearArrangement",
            BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
            serde_json::to_value(OptimalLinearArrangement::new(graph)).unwrap(),
            4,
        ),
    ];
    for (name, variant, inner, optimum) in cases {
        for bound in [optimum - 1, optimum, optimum + 1] {
            let loaded = load_dyn(
                name,
                &variant,
                serde_json::json!({"inner": inner, "bound": bound}),
            )
            .unwrap();
            for backend in [
                SolverRequest::BruteForce,
                SolverRequest::Ilp,
                SolverRequest::Default,
            ] {
                let result = solve(&loaded, backend).unwrap();
                if bound < optimum {
                    assert_eq!(
                        result.outcome,
                        SolveOutcome::Infeasible,
                        "{name}, {bound}, {backend:?}"
                    );
                } else {
                    let SolveOutcome::Optimal {
                        solution,
                        evaluation,
                    } = result.outcome
                    else {
                        panic!("expected a witness for {name}, {bound}, {backend:?}");
                    };
                    assert_eq!(evaluation, "Or(true)");
                    assert_eq!(loaded.evaluate_dyn(&solution).unwrap(), "Or(true)");
                }
            }
        }
    }
}

#[test]
fn generic_decision_ilp_matches_exhaustive_search_on_small_graphs() {
    use crate::models::graph::{
        MinimumDominatingSet, MinimumVertexCover, OptimalLinearArrangement,
    };
    use crate::topology::SimpleGraph;

    let weighted = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i64".to_string()),
    ]);
    let unweighted = BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]);
    for mask in 0..8 {
        let graph = SimpleGraph::new(
            3,
            [(0, 1), (0, 2), (1, 2)]
                .into_iter()
                .enumerate()
                .filter_map(|(i, edge)| (mask & (1 << i) != 0).then_some(edge))
                .collect(),
        );
        let models = [
            (
                "DecisionMinimumVertexCover",
                &weighted,
                serde_json::to_value(MinimumVertexCover::new(graph.clone(), vec![1i64, 2, 3]))
                    .unwrap(),
            ),
            (
                "DecisionMinimumDominatingSet",
                &weighted,
                serde_json::to_value(MinimumDominatingSet::new(graph.clone(), vec![1i64, 2, 3]))
                    .unwrap(),
            ),
            (
                "DecisionOptimalLinearArrangement",
                &unweighted,
                serde_json::to_value(OptimalLinearArrangement::new(graph)).unwrap(),
            ),
        ];
        for (name, variant, inner) in models {
            for bound in 0..=6 {
                let loaded = load_dyn(
                    name,
                    variant,
                    serde_json::json!({"inner": inner, "bound": bound}),
                )
                .unwrap();
                let reference = solve(&loaded, SolverRequest::BruteForce).unwrap();
                let actual = solve(&loaded, SolverRequest::Ilp).unwrap();
                assert_eq!(
                    matches!(actual.outcome, SolveOutcome::Infeasible),
                    matches!(reference.outcome, SolveOutcome::Infeasible),
                    "{name}, graph {mask}, bound {bound}"
                );
                if let SolveOutcome::Optimal { solution, .. } = actual.outcome {
                    assert_eq!(loaded.evaluate_dyn(&solution).unwrap(), "Or(true)");
                }
            }
        }
    }
}

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

    let result = solve(&loaded, SolverRequest::Default).unwrap();
    assert_eq!(
        result.solver,
        SolverExecution::Customized {
            implementation: "fd-minimum-cardinality-key"
        }
    );

    let explicit = solve(&loaded, SolverRequest::Customized).unwrap();
    assert_eq!(explicit, result);
}

#[test]
fn deterministic_solver_dispatch_unregistered_customized_override_is_a_capability_error() {
    use crate::models::graph::MaxCut;
    use crate::topology::SimpleGraph;

    let problem = MaxCut::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i64]);
    let loaded = crate::registry::load_dyn(
        MaxCut::<SimpleGraph, i64>::NAME,
        &BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i64".to_string()),
        ]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let error = solve(&loaded, SolverRequest::Customized).unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::SolveError::MissingCustomizedCapability(_)
    ));
}

#[test]
fn deterministic_solver_dispatch_unregistered_ilp_override_is_a_capability_error_without_fallback()
{
    use crate::models::graph::MaxCut;
    use crate::topology::SimpleGraph;

    // MaxCut<i64> has a discoverable graph route toward ILP, but that route is
    // partial for valid negative-weight instances and is intentionally not a
    // registered solver pipeline.
    let problem = MaxCut::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i64]);
    let loaded = crate::registry::load_dyn(
        MaxCut::<SimpleGraph, i64>::NAME,
        &BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i64".to_string()),
        ]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let default = solve(&loaded, SolverRequest::Default).unwrap();
    assert_eq!(default.solver, SolverExecution::BruteForce);
    let error = solve(&loaded, SolverRequest::Ilp).unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::SolveError::MissingIlpCapability(_)
    ));
}

#[test]
fn deterministic_solver_dispatch_customized_infeasibility_does_not_fall_back() {
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

    let result = solve(&loaded, SolverRequest::Default).unwrap();
    assert_eq!(result.outcome, SolveOutcome::Infeasible);
    let brute_force = solve(&loaded, SolverRequest::BruteForce).unwrap();
    assert_eq!(brute_force.solver, SolverExecution::BruteForce);
    assert_eq!(brute_force.outcome, SolveOutcome::Infeasible);
}

#[test]
fn deterministic_solver_dispatch_integer_ilp_uses_registered_cast_pipeline() {
    let problem = ILP::<bool>::new(0, vec![], vec![], ObjectiveSense::Minimize).unwrap();
    let loaded = load_dyn(
        ILP::<bool>::NAME,
        &BTreeMap::from([
            ("variable".to_string(), "bool".to_string()),
            ("coefficient".to_string(), "i64".to_string()),
        ]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let result = solve(&loaded, SolverRequest::Default).unwrap();
    assert_eq!(
        result.solver,
        SolverExecution::Ilp {
            reduction_path: vec!["ILP<i64, bool>".to_string(), "ILP<f64, bool>".to_string()]
        }
    );
    assert!(matches!(
        result.outcome,
        SolveOutcome::Optimal {
            ref solution,
            ..
        } if solution.as_array().is_some_and(Vec::is_empty)
    ));
}

#[test]
fn deterministic_solver_dispatch_ilp_infeasibility_does_not_fall_back() {
    let problem = ILP::<bool>::new(
        0,
        vec![LinearConstraint::le(vec![], -1)],
        vec![],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    let loaded = load_dyn(
        ILP::<bool>::NAME,
        &BTreeMap::from([
            ("variable".to_string(), "bool".to_string()),
            ("coefficient".to_string(), "i64".to_string()),
        ]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let result = solve(&loaded, SolverRequest::Default).unwrap();
    assert_eq!(result.outcome, SolveOutcome::Infeasible);
    assert!(matches!(
        solve(&loaded, SolverRequest::BruteForce),
        Err(crate::solvers::SolveError::MissingRegistration(_))
    ));
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
            reduction_path: vec!["Source".to_string(), "ILP<i64, bool>".to_string()]
        })
        .unwrap(),
        serde_json::json!({
            "kind": "ilp",
            "reduction_path": ["Source", "ILP<i64, bool>"]
        })
    );
    assert_eq!(
        serde_json::to_value(SolverExecution::BruteForce).unwrap(),
        serde_json::json!({"kind": "brute-force"})
    );
}

#[test]
fn solve_outcome_has_disjoint_json_states() {
    assert_eq!(
        serde_json::to_value(SolveOutcome::Optimal {
            solution: serde_json::json!([1, 0]),
            evaluation: "Max(1)".to_string(),
        })
        .unwrap(),
        serde_json::json!({
            "status": "optimal",
            "solution": [1, 0],
            "evaluation": "Max(1)"
        })
    );
    assert_eq!(
        serde_json::to_value(SolveOutcome::Infeasible).unwrap(),
        serde_json::json!({"status": "infeasible"})
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

    let first = solve(&loaded, SolverRequest::Ilp).unwrap();
    let second = solve(&loaded, SolverRequest::Ilp).unwrap();
    assert_eq!(first, second);
    let SolverExecution::Ilp { reduction_path } = first.solver else {
        panic!("expected ILP execution metadata");
    };
    assert_eq!(
        reduction_path,
        vec![
            "MaximumIndependentSet<SimpleGraph, One>",
            "MaximumIndependentSet<SimpleGraph, i64>",
            "MaximumSetPacking<i64>",
            "ILP<i64, bool>",
            "ILP<f64, bool>",
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

    let default = solve(&loaded, SolverRequest::Default).unwrap();
    assert!(matches!(default.solver, SolverExecution::Customized { .. }));

    let explicit_ilp = solve(&loaded, SolverRequest::Ilp).unwrap();
    assert!(matches!(explicit_ilp.solver, SolverExecution::Ilp { .. }));
    let SolveOutcome::Optimal {
        evaluation: default_evaluation,
        ..
    } = default.outcome
    else {
        panic!("customized solver should find an optimum");
    };
    let SolveOutcome::Optimal {
        evaluation: ilp_evaluation,
        ..
    } = explicit_ilp.outcome
    else {
        panic!("ILP solver should find an optimum");
    };
    assert_eq!(default_evaluation, ilp_evaluation);
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
        let first = solve(&loaded, request).unwrap();
        let second = solve(&loaded, request).unwrap();
        assert_eq!(first, second, "{request:?} changed its witness");
        let SolveOutcome::Optimal { evaluation, .. } = first.outcome else {
            panic!("{request:?} should find an optimum");
        };
        evaluations.push(evaluation);
    }
    assert!(evaluations.windows(2).all(|pair| pair[0] == pair[1]));
}

fn check_unit_dominating_decision(num_vertices: usize, edges: &[(usize, usize)], bound: i64) {
    let variant = BTreeMap::from([
        ("graph".into(), "SimpleGraph".into()),
        ("weight".into(), "One".into()),
    ]);
    let problem = load_dyn(
        "DecisionMinimumDominatingSet",
        &variant,
        serde_json::json!({
            "inner": {
                "graph": {"num_vertices": num_vertices, "edges": edges},
                "weights": vec![1; num_vertices],
            },
            "bound": bound,
        }),
    )
    .unwrap();
    let reference = solve(&problem, SolverRequest::BruteForce).unwrap();
    for backend in [SolverRequest::Ilp, SolverRequest::Default] {
        let actual = solve(&problem, backend).unwrap();
        let SolverExecution::Ilp { reduction_path } = &actual.solver else {
            panic!("expected the registered ILP pipeline");
        };
        assert!(
            reduction_path
                .iter()
                .any(|node| node.starts_with("MinimumSumMulticenter")),
            "{reduction_path:?}"
        );
        assert_eq!(
            matches!(actual.outcome, SolveOutcome::Infeasible),
            matches!(reference.outcome, SolveOutcome::Infeasible),
            "n={num_vertices}, edges={edges:?}, bound={bound}, backend={backend:?}"
        );
        if let SolveOutcome::Optimal {
            solution,
            evaluation,
        } = actual.outcome
        {
            assert_eq!(evaluation, "Or(true)");
            assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
        }
    }
}

#[test]
fn unit_dominating_decision_ilp_handles_zero_negative_and_large_bounds() {
    for (n, edges) in [(0, vec![]), (1, vec![]), (3, vec![(0, 1), (1, 2)])] {
        for bound in [i64::MIN, -1, 0, 1, 3, i64::MAX] {
            check_unit_dominating_decision(n, &edges, bound);
        }
    }
}

#[test]
fn unit_dominating_decision_ilp_rejects_no_instance_with_feasible_multicenter_target() {
    check_unit_dominating_decision(4, &[(0, 1), (1, 2), (2, 3)], 1);
}

#[test]
fn unit_dominating_decision_ilp_matches_all_three_vertex_graphs() {
    for mask in 0..8 {
        let edges: Vec<_> = [(0, 1), (0, 2), (1, 2)]
            .into_iter()
            .enumerate()
            .filter_map(|(index, edge)| (mask & (1 << index) != 0).then_some(edge))
            .collect();
        for bound in 0..=4 {
            check_unit_dominating_decision(3, &edges, bound);
        }
    }
}
