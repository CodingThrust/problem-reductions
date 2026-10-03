use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::registry::load_dyn;
use crate::solvers::{solve, SolveOutcome, SolverExecution, SolverRequest};
use crate::traits::Problem;
use std::collections::BTreeMap;

#[test]
fn arithmetic_solvers_check_small_witness_ranges_before_large_moduli() {
    let cases = [
        (
            "QuadraticCongruences",
            1u64,
            1_000_000_007u64,
            2u64,
            Some(1u64),
        ),
        ("QuadraticCongruences", 4, 1_000_000_007, 2, None),
        ("QuadraticCongruences", 4, 1_000_000_007, 3, Some(2)),
        ("QuadraticCongruences", 1, 1_000_000_007, 4098, Some(1)),
        // Exhausting factorization's work budget resumes the witness search.
        (
            "QuadraticCongruences",
            4100 * 4100,
            1_000_000_007,
            4101,
            Some(4100),
        ),
        (
            "QuadraticCongruences",
            4101 * 4101,
            1_000_000_007,
            4101,
            None,
        ),
        // Factoring finishes, but scanning all prime roots would cost more.
        (
            "QuadraticCongruences",
            4500 * 4500 % 10_007,
            10_007,
            4501,
            Some(4500),
        ),
        (
            "QuadraticCongruences",
            4501 * 4501 % 10_007,
            10_007,
            4501,
            None,
        ),
        // Zero and nonunit root classes stay compact above the prefix.
        ("QuadraticCongruences", 0, 1 << 28, 20_000, Some(16_384)),
        (
            "QuadraticCongruences",
            12_288 * 12_288,
            1 << 30,
            13_000,
            Some(12_288),
        ),
        ("QuadraticCongruences", 0, 1, 2, Some(1)),
        ("QuadraticCongruences", 0, 7, 7, None),
        ("QuadraticCongruences", 0, 7, 8, Some(7)),
        (
            "QuadraticDiophantineEquations",
            1,
            1_000_000_007,
            1_000_000_008,
            Some(1),
        ),
        (
            "QuadraticDiophantineEquations",
            1,
            1_000_000_007,
            1_000_000_009,
            None,
        ),
    ];
    for (name, a, b, c, expected) in cases {
        let problem = load_dyn(
            name,
            &BTreeMap::new(),
            serde_json::json!({"a": a.to_string(), "b": b.to_string(), "c": c.to_string()}),
        )
        .unwrap();
        let result = solve(&problem, SolverRequest::Default).unwrap();
        match (result.outcome, expected) {
            (SolveOutcome::Optimal { solution, .. }, Some(witness)) => {
                assert_eq!(
                    solution,
                    serde_json::to_value(num_bigint::BigUint::from(witness)).unwrap()
                );
                assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
            }
            (SolveOutcome::Infeasible, None) => {}
            (actual, expected) => {
                panic!("{name}({a}, {b}, {c}): {actual:?}, expected {expected:?}")
            }
        }
    }
}

#[test]
fn arithmetic_solver_search_boundary_matches_integer_enumeration() {
    for b in [5005u64, 6561, 8192] {
        for a in [0u64, 1, 2, 9, 16, 49] {
            for c in [4097u64, 4098, 10_000] {
                let expected = (1..c).any(|x| x * x % b == a);
                let problem = load_dyn(
                    "QuadraticCongruences",
                    &BTreeMap::new(),
                    serde_json::json!({"a": a.to_string(), "b": b.to_string(), "c": c.to_string()}),
                )
                .unwrap();
                let result = solve(&problem, SolverRequest::Customized).unwrap();
                assert_eq!(
                    matches!(result.outcome, SolveOutcome::Optimal { .. }),
                    expected,
                    "x² = {a} mod {b}, x < {c}"
                );
                if let SolveOutcome::Optimal { solution, .. } = result.outcome {
                    assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
                }
            }
        }
    }
}

#[test]
fn arithmetic_solvers_find_large_bounded_roots() {
    use num_bigint::BigUint;

    let x = (BigUint::from(1u8) << 60usize) + BigUint::from(1u8);
    let modulus = BigUint::from(3u8).pow(100);
    let cases = [
        (
            "QuadraticCongruences",
            serde_json::json!({"a": (&x * &x).to_string(), "b": modulus.to_string(), "c": (&x + BigUint::from(1u8)).to_string()}),
        ),
        (
            "QuadraticDiophantineEquations",
            serde_json::json!({"a": "6", "b": (&modulus * 6u8).to_string(), "c": ((&x * &x + &modulus) * 6u8).to_string()}),
        ),
    ];
    for (name, data) in cases {
        let problem = load_dyn(name, &BTreeMap::new(), data).unwrap();
        let result = solve(&problem, SolverRequest::Customized).unwrap();
        let SolveOutcome::Optimal { solution, .. } = result.outcome else {
            panic!("expected the independently constructed root for {name}");
        };
        assert_eq!(solution, serde_json::to_value(&x).unwrap());
        assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
    }
}

#[test]
fn arithmetic_solvers_match_integer_enumeration() {
    for b in 1u64..=20 {
        for a in 0..b {
            for c in [1u64, 2, 4, 9] {
                let expected = (1..c).any(|x| x * x % b == a);
                let problem = load_dyn(
                    "QuadraticCongruences",
                    &BTreeMap::new(),
                    serde_json::json!({"a": a.to_string(), "b": b.to_string(), "c": c.to_string()}),
                )
                .unwrap();
                let actual = solve(&problem, SolverRequest::Customized).unwrap();
                assert_eq!(
                    matches!(actual.outcome, SolveOutcome::Optimal { .. }),
                    expected,
                    "x² = {a} mod {b}, x < {c}"
                );
                if let SolveOutcome::Optimal { solution, .. } = actual.outcome {
                    assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
                }
            }
        }
    }
    for a in 1u64..=4 {
        for b in 1u64..=12 {
            for c in 1u64..=30 {
                let expected = (1..c).any(|x| a * x * x < c && (c - a * x * x) % b == 0);
                let problem = load_dyn(
                    "QuadraticDiophantineEquations",
                    &BTreeMap::new(),
                    serde_json::json!({"a": a.to_string(), "b": b.to_string(), "c": c.to_string()}),
                )
                .unwrap();
                let actual = solve(&problem, SolverRequest::Customized).unwrap();
                assert_eq!(
                    matches!(actual.outcome, SolveOutcome::Optimal { .. }),
                    expected,
                    "{a}x² + {b}y = {c}"
                );
                if let SolveOutcome::Optimal { solution, .. } = actual.outcome {
                    assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
                }
            }
        }
    }
}

#[test]
fn register_solver_matches_exhaustive_ordering_search() {
    use crate::models::misc::RegisterSufficiency;
    for mask in 0..64 {
        let arcs: Vec<_> = [(1, 0), (2, 0), (2, 1), (3, 0), (3, 1), (3, 2)]
            .into_iter()
            .enumerate()
            .filter_map(|(i, edge)| (mask & (1 << i) != 0).then_some(edge))
            .collect();
        for bound in 0..=4 {
            let model = RegisterSufficiency::new(4, arcs.clone(), bound);
            let problem = load_dyn(
                RegisterSufficiency::NAME,
                &BTreeMap::new(),
                serde_json::to_value(model).unwrap(),
            )
            .unwrap();
            let expected = solve(&problem, SolverRequest::BruteForce).unwrap();
            let actual = solve(&problem, SolverRequest::Customized).unwrap();
            assert_eq!(
                matches!(actual.outcome, SolveOutcome::Optimal { .. }),
                matches!(expected.outcome, SolveOutcome::Optimal { .. }),
                "DAG {mask}, bound {bound}"
            );
            if let SolveOutcome::Optimal { solution, .. } = actual.outcome {
                assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
            }
        }
    }
}

#[test]
fn register_solver_checks_identical_dependency_groups() {
    use crate::models::misc::RegisterSufficiency;
    // Each consumer needs twelve inputs simultaneously. The final consumer
    // requires both results, so thirteen registers suffice and eleven cannot.
    let arcs = (0..12)
        .map(|v| (24, v))
        .chain((12..24).map(|v| (25, v)))
        .chain([(26, 24), (26, 25)])
        .collect::<Vec<_>>();
    for (bound, expected) in [(11, false), (13, true)] {
        let model = RegisterSufficiency::new(27, arcs.clone(), bound);
        let actual = model.solve_exact();
        assert_eq!(actual.is_some(), expected);
        if let Some(solution) = actual {
            assert!(model.evaluate(&solution).unwrap().0);
        }
    }
}

#[test]
fn ensemble_solver_returns_a_minimum_shared_union_program() {
    use crate::models::misc::EnsembleComputation;
    let model = EnsembleComputation::new(5, vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 1, 4]], 8);
    let problem = load_dyn(
        EnsembleComputation::NAME,
        &BTreeMap::new(),
        serde_json::to_value(model).unwrap(),
    )
    .unwrap();
    let actual = solve(&problem, SolverRequest::Customized).unwrap();
    let SolveOutcome::Optimal { solution, .. } = actual.outcome else {
        panic!("shared union program exists");
    };
    // Three distinct triples each need a gate and all triples can share {0,1}.
    // No triple is a disjoint union of two available singletons: optimum = 4.
    assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Min(4)");
}

#[test]
fn tree_and_weighted_sequencing_default_to_ilp() {
    let tree_variant = BTreeMap::from([
        ("graph".into(), "SimpleGraph".into()),
        ("weight".into(), "i64".into()),
    ]);
    let cases = [
        (
            "SteinerTree",
            tree_variant.clone(),
            serde_json::json!({"graph": {"num_vertices": 3, "edges": [[0,1],[1,2]]},
                "edge_weights": [1,-3], "terminals": [0,1]}),
        ),
        (
            "SteinerTree",
            tree_variant,
            serde_json::json!({"graph": {"num_vertices": 3, "edges": [[0,1]]},
                "edge_weights": [1], "terminals": [0,2]}),
        ),
        (
            "SequencingToMinimizeWeightedCompletionTime",
            BTreeMap::new(),
            serde_json::json!({"lengths": [2,1,0], "weights": [3,5,2],
                "precedences": [[0,2],[1,2]]}),
        ),
    ];
    for (name, variant, data) in cases {
        let problem = load_dyn(name, &variant, data).unwrap();
        let expected = solve(&problem, SolverRequest::BruteForce).unwrap();
        let actual = solve(&problem, SolverRequest::Default).unwrap();
        assert!(
            matches!(actual.solver, SolverExecution::Ilp { .. }),
            "{name}"
        );
        match (expected.outcome, actual.outcome) {
            (SolveOutcome::Infeasible, SolveOutcome::Infeasible) => {}
            (
                SolveOutcome::Optimal {
                    evaluation: expected,
                    ..
                },
                SolveOutcome::Optimal {
                    solution,
                    evaluation,
                },
            ) => {
                assert_eq!(evaluation, expected, "{name}");
                assert_eq!(problem.evaluate_dyn(&solution).unwrap(), expected, "{name}");
            }
            outcomes => panic!("{name}: mismatched outcomes: {outcomes:?}"),
        }
    }
}

#[test]
fn decision_ilp_paths_respect_bounds_and_return_valid_witnesses() {
    let graph = serde_json::json!({"num_vertices": 3, "edges": [[0,1],[1,2]]});
    let cases = [
        (
            "DecisionMinimumVertexCover",
            BTreeMap::from([
                ("graph".into(), "SimpleGraph".into()),
                ("weight".into(), "One".into()),
            ]),
            serde_json::json!({"graph": graph, "weights": [1,1,1]}),
            1,
        ),
        (
            "DecisionOpenShopScheduling",
            BTreeMap::new(),
            serde_json::json!({"num_machines": 2, "processing_times": [[2,1],[1,2]]}),
            3,
        ),
        (
            "DecisionRuralPostman",
            BTreeMap::from([
                ("graph".into(), "SimpleGraph".into()),
                ("weight".into(), "i64".into()),
            ]),
            serde_json::json!({"graph": graph, "edge_lengths": [1,1], "required_edges": [0,1]}),
            4,
        ),
    ];
    for (name, variant, inner, optimum) in cases {
        for bound in [optimum - 1, optimum, optimum + 1] {
            let problem = load_dyn(
                name,
                &variant,
                serde_json::json!({"inner": inner, "bound": bound}),
            )
            .unwrap();
            let result = solve(&problem, SolverRequest::Default).unwrap();
            assert!(
                matches!(result.solver, SolverExecution::Ilp { .. }),
                "{name}"
            );
            match result.outcome {
                SolveOutcome::Optimal { solution, .. } => {
                    assert!(bound >= optimum, "{name}, bound {bound}");
                    assert_eq!(problem.evaluate_dyn(&solution).unwrap(), "Or(true)");
                }
                SolveOutcome::Infeasible => assert!(bound < optimum, "{name}, bound {bound}"),
            }
        }
    }
}

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
fn deterministic_solver_dispatch_integer_ilp_uses_native_terminal() {
    let problem = ILP::<bool>::new(0, vec![], vec![], ObjectiveSense::Minimize).unwrap();
    let loaded = load_dyn(
        ILP::<bool>::NAME,
        &BTreeMap::from([
            ("variable".to_string(), "bool".to_string()),
            ("coefficient".to_string(), "i64".to_string()),
            ("bounds".to_string(), "general".to_string()),
        ]),
        serde_json::to_value(problem).unwrap(),
    )
    .unwrap();

    let result = solve(&loaded, SolverRequest::Default).unwrap();
    assert_eq!(
        result.solver,
        SolverExecution::Ilp {
            reduction_path: vec!["ILP<general, i64, bool>".to_string()]
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
            ("bounds".to_string(), "general".to_string()),
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
            reduction_path: vec!["Source".to_string(), "ILP<general, i64, bool>".to_string()]
        })
        .unwrap(),
        serde_json::json!({
            "kind": "ilp",
            "reduction_path": ["Source", "ILP<general, i64, bool>"]
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
            "ILP<general, i64, bool>",
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
