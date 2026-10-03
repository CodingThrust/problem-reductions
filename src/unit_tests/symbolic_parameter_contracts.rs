use crate::models::algebraic::AlgebraicEquationsOverGF2;
use crate::models::algebraic::Bounded;
use crate::models::graph::{MaximumClique, MaximumIndependentSet};
use crate::models::set::ExactCoverBy3Sets;
use crate::parameters::ParameterRelation;
use crate::rules::{ReduceTo, ReductionGraph, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::ProblemParameters;
use crate::Problem;

#[test]
fn parameter_schemas_keep_distinct_counts_without_synonymous_aliases() {
    let graph = ReductionGraph::new();
    for (model, retained) in [
        ("ExactCoverBy3Sets", &["num_subsets"][..]),
        ("ClosestString", &["string_length", "total_length"]),
        ("ClosestSubstring", &["total_length", "total_num_windows"]),
        ("ThreePartition", &["num_elements", "num_groups"]),
        ("PaintShop", &["num_cars", "num_sequence"]),
        (
            "MinimumCodeGenerationOneRegister",
            &["num_vertices", "num_leaves", "num_internal"],
        ),
        (
            "LongestCommonSubsequence",
            &["max_length", "num_transitions"],
        ),
    ] {
        let fields = graph.parameter_names(model);
        for field in retained {
            assert!(fields.iter().any(|name| name == field), "{model}: {field}");
        }
    }
    assert!(!graph
        .parameter_names("ExactCoverBy3Sets")
        .iter()
        .any(|field| field == "num_sets"));
    for (model, expected) in [
        ("HamiltonianPath", &["num_edges", "num_vertices"][..]),
        (
            "PreemptiveScheduling",
            &[
                "d_max",
                "max_schedule_magnitude_bits",
                "num_precedences",
                "num_processors",
                "num_tasks",
            ],
        ),
    ] {
        let mut fields = graph.parameter_names(model);
        fields.sort();
        assert_eq!(fields, expected, "{model} public parameter schema");
    }
}

#[test]
fn exact_rule_formula_matches_the_constructed_target() {
    let source = MaximumIndependentSet::<SimpleGraph, i64>::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1; 5],
    );
    let reduction = <MaximumIndependentSet<SimpleGraph, i64> as ReduceTo<
        MaximumClique<SimpleGraph, i64>,
    >>::reduce_to(&source)
    .expect("reduction should succeed");
    let target = reduction.target_problem();
    let graph = ReductionGraph::new();
    let source_variant =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i64>::variant());
    let target_variant =
        ReductionGraph::variant_to_map(&MaximumClique::<SimpleGraph, i64>::variant());
    let path = graph
        .find_all_paths(
            MaximumIndependentSet::<SimpleGraph, i64>::NAME,
            &source_variant,
            MaximumClique::<SimpleGraph, i64>::NAME,
            &target_variant,
        )
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("direct reduction is registered");

    let transform = graph
        .compose_path_parameter_transform(&path)
        .unwrap()
        .unwrap();
    let predicted = transform
        .evaluate(&ProblemParameters::new(vec![
            ("num_vertices", 5),
            ("num_edges", 4),
        ]))
        .unwrap();
    assert_eq!(
        predicted.get("num_vertices"),
        Some(u64::try_from(target.num_vertices()).unwrap())
    );
    assert_eq!(
        predicted.get("num_edges"),
        Some(u64::try_from(target.num_edges()).unwrap())
    );
}

#[test]
fn incoming_rule_measures_every_declared_field_on_a_sink_variant() {
    let source = ExactCoverBy3Sets::new(3, vec![[0, 1, 2]]);
    let reduction = <ExactCoverBy3Sets as ReduceTo<AlgebraicEquationsOverGF2>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let target_variant = ReductionGraph::variant_to_map(&AlgebraicEquationsOverGF2::variant());

    let measured = ReductionGraph::compute_problem_parameters(
        AlgebraicEquationsOverGF2::NAME,
        &target_variant,
        target,
    );

    assert_eq!(
        measured.get("num_variables"),
        Some(u64::try_from(target.num_variables()).unwrap())
    );
    assert_eq!(
        measured.get("num_equations"),
        Some(u64::try_from(target.num_equations()).unwrap())
    );
}

#[test]
fn every_registered_rule_has_one_valid_parameter_contract() {
    for entry in crate::rules::registry::reduction_entries() {
        let contract = entry.parameter_contract().unwrap_or_else(|error| {
            panic!(
                "{} -> {} has an invalid parameter contract: {error}",
                entry.source_name, entry.target_name
            )
        });
        assert!(contract.transform().is_some() || !contract.unavailable().is_empty());
    }
}

#[cfg(feature = "example-db")]
#[test]
fn canonical_examples_satisfy_upper_bound_parameter_contracts() {
    let graph = ReductionGraph::new();
    for spec in crate::rules::canonical_rule_example_specs() {
        let example = (spec.build)();
        let source = crate::registry::load_dyn(
            &example.source.problem,
            &example.source.variant,
            example.source.instance.clone(),
        )
        .unwrap();
        let target = crate::registry::load_dyn(
            &example.target.problem,
            &example.target.variant,
            example.target.instance.clone(),
        )
        .unwrap();
        let entry = graph
            .find_entry(
                &example.source.problem,
                &example.source.variant,
                &example.target.problem,
                &example.target.variant,
            )
            .unwrap_or_else(|| panic!("{} has no registered direct edge", spec.id));
        let Ok(contract) = entry.parameter_contract else {
            continue;
        };
        let Some(transform) = contract.transform() else {
            continue;
        };
        let source_size = ReductionGraph::compute_problem_parameters(
            &example.source.problem,
            &example.source.variant,
            source.as_any(),
        );
        let target_size = ReductionGraph::compute_problem_parameters(
            &example.target.problem,
            &example.target.variant,
            target.as_any(),
        );
        let predicted = transform
            .evaluate(&source_size)
            .unwrap_or_else(|error| panic!("{}: {error}", spec.id));

        for (field, actual) in target_size.components {
            let Some(predicted_value) = predicted.get(&field) else {
                continue;
            };
            assert!(
                predicted_value >= actual,
                "{}: target field {field}: predicted {predicted_value}, actual {actual}",
                spec.id
            );
        }
    }
}

fn check_reduced_parameters<S, T>(source: S, fields: &[&str], relation: ParameterRelation)
where
    S: Problem + ReduceTo<T>,
    T: Problem,
{
    let reduction = source.reduce_to().expect("reduction should succeed");
    let actual = reduction.target_problem().parameters();
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == S::NAME
                && entry.target_name == T::NAME
                && entry.source_variant() == S::variant()
                && entry.target_variant() == T::variant()
        })
        .expect("direct reduction is registered");
    let contract = entry.parameter_contract().unwrap();
    let transform = contract.transform().expect("symbolic transform exists");
    let predicted = transform.evaluate(&source.parameters()).unwrap();
    // Auxiliary fields have their own relations (for example, ILP magnitude
    // bounds alongside exact variable counts). Check each against the target.
    for (field, _) in transform.expressions() {
        let predicted = predicted.get(field).unwrap();
        let actual = actual.get(field).unwrap();
        assert!(
            match transform.relation(field).unwrap() {
                ParameterRelation::Exact => predicted == actual,
                ParameterRelation::UpperBound => predicted >= actual,
            },
            "{} -> {}: {field}: predicted {predicted}, measured {actual}",
            S::NAME,
            T::NAME
        );
    }
    for &field in fields {
        assert_eq!(
            transform.relation(field),
            Some(relation),
            "{} -> {}: {field}",
            S::NAME,
            T::NAME
        );
        assert!(
            !contract
                .unavailable()
                .iter()
                .any(|item| item.field == field),
            "{} -> {}: {field} is still unavailable",
            S::NAME,
            T::NAME
        );
    }
}

#[test]
fn parameter_relations_match_reduced_instances() {
    use crate::models::algebraic::MinimumMatrixCover;
    use crate::models::algebraic::{
        IntegerVariable, LinearConstraint, ObjectiveSense, QuadraticAssignment, BMF, ILP,
    };
    use crate::models::graph::BicliqueCover;
    use crate::models::graph::{
        HamiltonianPath, MaximumContactMapOverlap, MinimumVertexCover, OptimalLinearArrangement,
    };
    use crate::models::misc::{
        ClosestString, ConsistencyOfDatabaseFrequencyTables, ExpectedRetrievalCost,
        FeasibleRegisterAssignment, LongestCommonSubsequence, MaximumLikelihoodRanking,
        MultiprocessorScheduling, Partition, RegisterSufficiency, ResourceConstrainedScheduling,
        SequencingToMinimizeWeightedCompletionTime, SumOfSquaresPartition, ThreePartition,
    };
    use crate::models::set::{IntegerKnapsack, ThreeDimensionalMatching};
    use crate::types::One;

    let exact = ParameterRelation::Exact;
    check_reduced_parameters::<_, ILP<bool>>(
        BMF::new(vec![vec![true, false], vec![false, true]], 2),
        &["num_nonzeros"],
        ParameterRelation::UpperBound,
    );
    check_reduced_parameters::<_, ILP<i64, i64, Bounded>>(
        ClosestString::new(2, vec![vec![0, 1], vec![1, 0]]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        ConsistencyOfDatabaseFrequencyTables::new(1, vec![2, 2], vec![], vec![]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        ExactCoverBy3Sets::new(3, vec![[0, 1, 2]]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool, f64>>(
        ExpectedRetrievalCost::new(vec![0.5, 0.5], 2).unwrap(),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<i64, i64, Bounded>>(
        FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0]),
        &["num_nonzeros"],
        ParameterRelation::UpperBound,
    );
    check_reduced_parameters::<_, ILP<i64, i64, Bounded>>(
        IntegerKnapsack::new(vec![3, 4], vec![5, 6], 7).unwrap(),
        &["num_nonzeros"],
        ParameterRelation::UpperBound,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        LongestCommonSubsequence::new(2, vec![vec![0, 1], vec![1, 0, 1]]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        MaximumContactMapOverlap::new(3, vec![(0, 2)], 3, vec![(0, 1)]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        MaximumLikelihoodRanking::new(vec![vec![0, 1, 2], vec![2, 0, 1], vec![1, 2, 0]]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        MinimumMatrixCover::new(vec![vec![0, 2], vec![3, 0]]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        RegisterSufficiency::new(4, vec![(2, 0), (3, 1)], 2),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        SumOfSquaresPartition::new(vec![1, 2, 3], 2),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        ThreeDimensionalMatching::new(2, vec![(0, 1, 1), (1, 0, 0)]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        QuadraticAssignment::new(vec![vec![0, 1], vec![2, 0]], vec![vec![0, 3], vec![4, 0]]),
        &["num_vars", "num_constraints", "num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)])),
        &["num_vars", "num_constraints"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (0, 1), (1, 1)])),
        &["num_nonzeros"],
        ParameterRelation::UpperBound,
    );
    check_reduced_parameters::<_, BicliqueCover>(
        BMF::new(vec![vec![true, false], vec![false, true]], 1),
        &["num_vertices", "left_size", "right_size", "rank"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        ILP::<i64, i64, Bounded>::with_variables(
            vec![IntegerVariable::new(Some(0), Some(3)).unwrap()],
            vec![LinearConstraint::le(vec![(0, 1)], 2)],
            vec![],
            ObjectiveSense::Minimize,
        )
        .unwrap(),
        &["num_constraints"],
        exact,
    );
    check_reduced_parameters::<_, LongestCommonSubsequence>(
        MinimumVertexCover::new(SimpleGraph::path(4), vec![One; 4]),
        &["sum_triangular_lengths"],
        exact,
    );
    check_reduced_parameters::<_, SequencingToMinimizeWeightedCompletionTime>(
        OptimalLinearArrangement::new(SimpleGraph::path(4)),
        &["num_precedences"],
        exact,
    );
    check_reduced_parameters::<_, MultiprocessorScheduling>(
        Partition::new(vec![1, 2, 3]).unwrap(),
        &["num_processors"],
        exact,
    );
    check_reduced_parameters::<_, ResourceConstrainedScheduling>(
        ThreePartition::new(vec![4, 5, 6, 4, 6, 5], 15),
        &["deadline", "num_resources"],
        exact,
    );
}

#[test]
fn exact_parameter_formulas_cover_sparse_and_boundary_instances() {
    use crate::models::algebraic::{QuadraticAssignment, ILP};
    use crate::models::graph::{HamiltonianCircuit, HamiltonianPath, MinimumVertexCover};
    use crate::models::misc::{
        ConsistencyOfDatabaseFrequencyTables, FrequencyTable, KnownValue, LongestCommonSubsequence,
        MaximumLikelihoodRanking, RegisterSufficiency,
    };

    let exact = ParameterRelation::Exact;
    check_reduced_parameters::<_, ILP<bool>>(
        ConsistencyOfDatabaseFrequencyTables::new(
            2,
            vec![2, 2],
            vec![FrequencyTable::new(0, 1, vec![vec![1, 0], vec![0, 1]])],
            vec![KnownValue::new(0, 0, 0)],
        ),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        LongestCommonSubsequence::new(2, vec![vec![], vec![0, 1]]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        MaximumLikelihoodRanking::new(vec![]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        RegisterSufficiency::new(0, vec![], 0),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        QuadraticAssignment::new(vec![], vec![vec![0]]),
        &["num_vars", "num_constraints", "num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        QuadraticAssignment::new(vec![vec![0]], vec![vec![0, 1], vec![1, 0]]),
        &["num_vars", "num_constraints", "num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        HamiltonianPath::new(SimpleGraph::new(0, vec![])),
        &["num_vars", "num_constraints"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        HamiltonianPath::new(SimpleGraph::new(1, vec![])),
        &["num_vars", "num_constraints"],
        exact,
    );
    check_reduced_parameters::<_, HamiltonianPath<SimpleGraph>>(
        HamiltonianCircuit::new(SimpleGraph::new(0, vec![])),
        &["num_vertices"],
        exact,
    );
    check_reduced_parameters::<_, HamiltonianPath<SimpleGraph>>(
        HamiltonianCircuit::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)])),
        &["num_vertices"],
        exact,
    );
    check_reduced_parameters::<_, LongestCommonSubsequence>(
        MinimumVertexCover::new(SimpleGraph::new(0, vec![]), vec![]),
        &["sum_triangular_lengths"],
        exact,
    );
}

#[test]
fn multiprocessor_magnitude_predictions_cover_lengths_and_deadlines() {
    use crate::models::algebraic::ILP;
    use crate::models::misc::{MultiprocessorScheduling, Partition};

    for (lengths, deadline, bits) in [
        (vec![], 0, 1),
        (vec![], 8, 4),
        (vec![0], 0, 1),
        (vec![7], 1, 3),
        (vec![8], 1, 4),
        (vec![1], 8, 4),
        (vec![i64::MAX], 0, 63),
        (vec![0], i64::MAX, 63),
    ] {
        let source = MultiprocessorScheduling::new(lengths, 2, deadline);
        assert_eq!(
            source.parameters().get("max_numeric_magnitude_bits"),
            Some(bits)
        );
        check_reduced_parameters::<_, ILP<bool>>(
            source,
            &["max_constraint_magnitude_bits"],
            ParameterRelation::Exact,
        );
    }
    // The deadline can require more bits than any individual input size.
    for sizes in [vec![1], vec![3; 4], vec![1, 2], vec![i64::MAX]] {
        check_reduced_parameters::<_, MultiprocessorScheduling>(
            Partition::new(sizes).unwrap(),
            &["num_tasks", "num_processors"],
            ParameterRelation::Exact,
        );
    }
}

#[test]
fn augmentation_magnitude_predictions_cover_weights_and_budgets() {
    use crate::models::algebraic::ILP;
    use crate::models::graph::{BiconnectivityAugmentation, StrongConnectivityAugmentation};
    use crate::topology::DirectedGraph;

    fn check<S: Problem + ReduceTo<ILP<bool>>>(source: S, bits: u64) {
        assert_eq!(
            source.parameters().get("max_numeric_magnitude_bits"),
            Some(bits)
        );
        check_reduced_parameters::<_, ILP<bool>>(
            source,
            &["max_constraint_magnitude_bits"],
            ParameterRelation::Exact,
        );
    }
    for (weight, budget, bits) in [
        (0, 0, 1),
        (1, 8, 4),
        (8, 1, 4),
        (7, 1, 3),
        (-8, 1, 4),
        (1, -8, 4),
        (i64::MIN, 0, 64),
        (0, i64::MIN, 64),
        (i64::MAX, 0, 63),
        (1, i64::MAX, 63),
    ] {
        check(
            BiconnectivityAugmentation::new(SimpleGraph::empty(2), vec![(0, 1, weight)], budget),
            bits,
        );
        if weight > 0 && budget >= 0 {
            check(
                StrongConnectivityAugmentation::new(
                    DirectedGraph::empty(2),
                    vec![(0, 1, weight)],
                    budget,
                ),
                bits,
            );
        }
    }
    for (budget, bits) in [(0, 1), (8, 4), (i64::MAX, 63)] {
        check(
            BiconnectivityAugmentation::<_, i64>::new(SimpleGraph::empty(0), vec![], budget),
            bits,
        );
        check(
            StrongConnectivityAugmentation::<i64>::new(DirectedGraph::empty(0), vec![], budget),
            bits,
        );
    }
}
#[test]
fn missing_structural_bounds_cover_sparse_and_normalized_instances() {
    use crate::models::{
        algebraic::BMF,
        graph::{BalancedCompleteBipartiteSubgraph, BicliqueCover, KClique},
        set::MaximumSetPacking,
    };
    use crate::types::One;
    let upper = ParameterRelation::UpperBound;
    for matrix in [
        vec![],
        vec![vec![]],
        vec![vec![false; 3]; 2],
        vec![vec![true, false], vec![false, true]],
        vec![vec![true; 3]; 2],
    ] {
        check_reduced_parameters::<_, BicliqueCover>(BMF::new(matrix, 1), &["num_edges"], upper);
    }
    for subsets in [vec![], vec![[0, 1, 2]], vec![[0, 1, 2], [0, 1, 2]]] {
        check_reduced_parameters::<_, MaximumSetPacking<One>>(
            ExactCoverBy3Sets::new(6, subsets),
            &["universe_size"],
            upper,
        );
    }
    for graph in [
        SimpleGraph::empty(3),
        SimpleGraph::new(3, vec![(0, 0), (0, 1), (0, 1)]),
        SimpleGraph::complete(3),
    ] {
        check_reduced_parameters::<_, BalancedCompleteBipartiteSubgraph>(
            KClique::new(graph, 2),
            &["num_vertices"],
            upper,
        );
    }
}

#[test]
fn sat_bounds_cover_empty_short_and_repeated_clauses() {
    use crate::models::{
        formula::{CNFClause, CircuitSAT, KSatisfiability, NAESatisfiability, Satisfiability},
        graph::IntegralFlowHomologousArcs,
    };
    use crate::variant::K3;
    for clauses in [
        vec![],
        vec![CNFClause::new(vec![])],
        vec![CNFClause::new(vec![1])],
        vec![CNFClause::new(vec![1, -1, 2, 2, 3])],
    ] {
        let source = Satisfiability::new(4, clauses);
        check_reduced_parameters::<_, KSatisfiability<K3>>(
            source.clone(),
            &["num_literals"],
            ParameterRelation::UpperBound,
        );
        check_reduced_parameters::<_, NAESatisfiability>(
            source.clone(),
            &["num_literal_pairs"],
            ParameterRelation::UpperBound,
        );
        check_reduced_parameters::<_, CircuitSAT>(
            source.clone(),
            &["num_expression_nodes", "num_assignment_outputs"],
            ParameterRelation::UpperBound,
        );
        check_reduced_parameters::<_, IntegralFlowHomologousArcs>(
            source,
            &["max_capacity"],
            ParameterRelation::UpperBound,
        );
    }
}

#[test]
fn circuit_bounds_cover_fanin_constants_and_multiple_outputs() {
    use crate::models::formula::{Assignment, BooleanExpr, Circuit, CircuitSAT, Satisfiability};
    for expr in [
        BooleanExpr::constant(true),
        BooleanExpr::not(BooleanExpr::var("x")),
        BooleanExpr::xor(vec![BooleanExpr::var("x"); 8]),
        BooleanExpr::and(vec![
            BooleanExpr::or(vec![
                BooleanExpr::var("x"),
                BooleanExpr::var("y")
            ]);
            4
        ]),
    ] {
        for outputs in [vec![], vec!["a".into(), "b".into()]] {
            let source =
                CircuitSAT::new(Circuit::new(vec![Assignment::new(outputs, expr.clone())]));
            check_reduced_parameters::<_, Satisfiability>(
                source,
                &["num_vars", "num_clauses", "num_literals"],
                ParameterRelation::UpperBound,
            );
        }
    }
}

#[test]
fn factoring_circuit_bounds_cover_zero_width_and_overflow_sentinels() {
    use crate::models::{formula::CircuitSAT, misc::Factoring};
    for m in 0..=3 {
        for n in m..=3 {
            for target in [0u64, 1, 255] {
                check_reduced_parameters::<_, CircuitSAT>(
                    Factoring::with_factor_bits(target, m, n),
                    &["num_assignment_outputs", "num_expression_nodes"],
                    ParameterRelation::UpperBound,
                );
            }
        }
    }
}

#[test]
fn sat_flow_and_scheduling_bounds_cover_fixed_outputs() {
    use crate::models::{
        formula::{CNFClause, KSatisfiability},
        graph::DirectedTwoCommodityIntegralFlow,
        misc::PreemptiveScheduling,
    };
    use crate::variant::K3;
    for clauses in [
        vec![],
        vec![CNFClause::new(vec![])],
        vec![CNFClause::new(vec![1, 1, -2])],
    ] {
        let source = KSatisfiability::<K3>::new_allow_less(2, clauses);
        check_reduced_parameters::<_, DirectedTwoCommodityIntegralFlow>(
            source.clone(),
            &["max_capacity"],
            ParameterRelation::Exact,
        );
        check_reduced_parameters::<_, PreemptiveScheduling>(
            source,
            &["num_precedences"],
            ParameterRelation::UpperBound,
        );
    }
}

#[test]
fn set_splitting_bounds_cover_deduplication_and_large_subsets() {
    use crate::models::{misc::Betweenness, set::SetSplitting};
    for subsets in [
        vec![],
        vec![vec![0, 0]],
        vec![vec![0, 1]],
        vec![(0..8).collect()],
        vec![vec![0; 20], (0..8).collect()],
    ] {
        check_reduced_parameters::<_, Betweenness>(
            SetSplitting::new(8, subsets),
            &["num_elements", "num_triples"],
            ParameterRelation::UpperBound,
        );
    }
}

#[test]
fn decision_cover_bounds_do_not_depend_on_threshold_magnitude() {
    use crate::models::{
        graph::{HamiltonianCircuit, MinimumVertexCover},
        Decision,
    };
    use crate::types::One;
    for graph in [
        SimpleGraph::empty(0),
        SimpleGraph::path(3),
        SimpleGraph::new(3, vec![(0, 0), (0, 1), (1, 2)]),
    ] {
        for threshold in [i64::MIN, 0, 1, 2, i64::MAX] {
            check_reduced_parameters::<_, HamiltonianCircuit<SimpleGraph>>(
                Decision::new(
                    MinimumVertexCover::<_, One>::new(
                        graph.clone(),
                        vec![One; crate::topology::Graph::num_vertices(&graph)],
                    ),
                    threshold,
                ),
                &["num_vertices", "num_edges"],
                ParameterRelation::UpperBound,
            );
        }
    }
}

#[test]
fn subset_lattice_dimensions_use_existing_numeric_magnitude() {
    use crate::models::{algebraic::ClosestVectorProblem, misc::SubsetSum, Decision};
    for (sizes, target) in [
        (vec![], 0u64),
        (vec![1], 0),
        (vec![7], 8),
        (vec![8], 7),
        (vec![1, 3], 4),
    ] {
        check_reduced_parameters::<_, Decision<ClosestVectorProblem>>(
            SubsetSum::new(sizes, target),
            &["ambient_dimension", "num_basis_vectors"],
            ParameterRelation::Exact,
        );
    }
}

#[test]
fn knapsack_qubo_bounds_cover_capacity_boundaries() {
    use crate::models::{algebraic::QUBO, misc::Knapsack};
    for (capacity, bits) in [(0, 1), (1, 1), (2, 2), (3, 2), (4, 3), (7, 3), (8, 4)] {
        let source = Knapsack::new(vec![0, 1, 2], vec![0, 2, 1], capacity);
        assert_eq!(source.parameters().get("capacity_bits"), Some(bits));
        check_reduced_parameters::<_, QUBO<i64>>(source, &["num_vars"], ParameterRelation::Exact);
    }
}

#[test]
fn knapsack_ilp_magnitude_uses_capacity_bits() {
    use crate::models::{algebraic::ILP, misc::Knapsack};
    for (capacity, bits) in [(0, 1), (7, 3), (8, 4), (i64::MAX, 63)] {
        let source = Knapsack::new(vec![0, 1, i64::MAX], vec![1, 2, 3], capacity);
        assert_eq!(source.parameters().get("capacity_bits"), Some(bits));
        check_reduced_parameters::<_, ILP<bool>>(
            source,
            &["max_constraint_magnitude_bits"],
            ParameterRelation::UpperBound,
        );
    }
}

#[test]
fn partition_propagates_capacity_bits_without_raw_capacity() {
    use crate::models::misc::{Knapsack, Partition};
    for sizes in [vec![1], vec![1, 2], vec![7, 8], vec![i64::MAX]] {
        check_reduced_parameters::<_, Knapsack>(
            Partition::new(sizes).unwrap(),
            &["capacity_bits"],
            ParameterRelation::UpperBound,
        );
    }
}

#[test]
fn integer_knapsack_capacity_bits_bound_ilp_magnitudes() {
    use crate::models::{algebraic::ILP, set::IntegerKnapsack};
    for (capacity, bits) in [(0, 1), (7, 3), (8, 4), (i64::MAX, 63)] {
        let source = IntegerKnapsack::new(vec![1, i64::MAX], vec![1, 2], capacity).unwrap();
        assert_eq!(source.parameters().get("capacity_bits"), Some(bits));
        check_reduced_parameters::<_, ILP<i64, i64, Bounded>>(
            source,
            &["max_constraint_magnitude_bits"],
            ParameterRelation::UpperBound,
        );
    }
}

#[test]
fn open_shop_horizon_bits_cover_totals_and_decision_bounds() {
    use crate::models::{algebraic::ILP, misc::OpenShopScheduling, Decision};
    for (machines, times, bits) in [
        (0, vec![vec![]], 1),
        (2, vec![], 1),
        (2, vec![vec![0, 0]], 1),
        (2, vec![vec![3, 4]], 3),
        (2, vec![vec![4, 4]], 4),
        (1, vec![vec![i64::MAX]], 63),
    ] {
        let source = OpenShopScheduling::new(machines, times);
        assert_eq!(source.parameters().get("schedule_horizon_bits"), Some(bits));
        check_reduced_parameters::<_, ILP<i64, i64, Bounded>>(
            source.clone(),
            &["max_constraint_magnitude_bits"],
            ParameterRelation::UpperBound,
        );
        for bound in [i64::MIN, 0, i64::MAX] {
            check_reduced_parameters::<_, ILP<i64, i64, Bounded>>(
                Decision::new(source.clone(), bound),
                &["max_constraint_magnitude_bits"],
                ParameterRelation::UpperBound,
            );
        }
    }
}

#[test]
fn partition_propagates_open_shop_horizon_bits() {
    use crate::models::{
        misc::{OpenShopScheduling, Partition},
        Decision,
    };
    for sizes in [vec![1], vec![1, 1], vec![1, 2], vec![1 << 20; 2]] {
        check_reduced_parameters::<_, Decision<OpenShopScheduling>>(
            Partition::new(sizes).unwrap(),
            &["schedule_horizon_bits"],
            ParameterRelation::UpperBound,
        );
    }
}

#[test]
fn closest_vector_size_bounds_cover_numeric_and_rank_variation() {
    use crate::models::algebraic::{ClosestVectorProblem, QUBO};
    for (basis, target, bits) in [
        (vec![], vec![], 1),
        (vec![], vec![-8], 4),
        (vec![vec![1]], vec![0], 1),
        (vec![vec![-8]], vec![1], 4),
        (vec![vec![1]], vec![8], 4),
        (vec![vec![2, 0], vec![1, 2]], vec![3, 2], 2),
        (vec![vec![1, 8], vec![0, 1]], vec![-1, 1], 4),
    ] {
        let source = ClosestVectorProblem::new(basis, target).unwrap();
        assert_eq!(
            source.parameters().get("max_numeric_magnitude_bits"),
            Some(bits)
        );
        check_reduced_parameters::<_, QUBO<i64>>(
            source,
            &["num_vars", "num_quadratic_terms"],
            ParameterRelation::UpperBound,
        );
    }
    for value in [i64::MIN, i64::MAX] {
        let source = ClosestVectorProblem::new(vec![vec![value]], vec![value]).unwrap();
        assert_eq!(
            source.parameters().get("max_numeric_magnitude_bits"),
            Some(if value == i64::MIN { 64 } else { 63 })
        );
    }
}

#[test]
fn incongruence_pair_bounds_cover_prime_growth_and_repeated_literals() {
    use crate::models::{
        algebraic::SimultaneousIncongruences,
        formula::{CNFClause, KSatisfiability},
    };
    use crate::variant::K3;
    for variables in 0..=12 {
        let clauses = if variables == 0 {
            vec![]
        } else {
            vec![CNFClause::new(vec![1, 1, -1])]
        };
        check_reduced_parameters::<_, SimultaneousIncongruences>(
            KSatisfiability::<K3>::new(variables, clauses),
            &["num_pairs"],
            ParameterRelation::UpperBound,
        );
    }
}

#[test]
fn vertex_cover_lcs_transition_bounds_cover_empty_strings() {
    use crate::models::{graph::MinimumVertexCover, misc::LongestCommonSubsequence};
    use crate::{
        topology::{Graph, SimpleGraph},
        types::One,
    };
    for graph in [
        SimpleGraph::empty(0),
        SimpleGraph::empty(1),
        SimpleGraph::path(3),
    ] {
        let weights = vec![One; graph.num_vertices()];
        check_reduced_parameters::<_, LongestCommonSubsequence>(
            MinimumVertexCover::new(graph, weights),
            &["num_transitions"],
            ParameterRelation::UpperBound,
        );
    }
}
