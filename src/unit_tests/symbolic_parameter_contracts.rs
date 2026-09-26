use crate::models::algebraic::AlgebraicEquationsOverGF2;
use crate::models::graph::{MaximumClique, MaximumIndependentSet};
use crate::models::set::ExactCoverBy3Sets;
use crate::parameters::ParameterRelation;
use crate::rules::{ReduceTo, ReductionGraph, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::ProblemParameters;
use crate::Problem;

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
        let graph = ReductionGraph::new();
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
        if transform.relation() != ParameterRelation::UpperBound {
            continue;
        }
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
    assert_eq!(transform.relation(), relation, "{} -> {}", S::NAME, T::NAME);
    let predicted = transform.evaluate(&source.parameters()).unwrap();
    for &field in fields {
        assert_eq!(
            predicted.get(field),
            actual.get(field),
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
fn newly_exact_parameters_match_reduced_instances() {
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
        exact,
    );
    check_reduced_parameters::<_, ILP<i64>>(
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
    check_reduced_parameters::<_, ILP<i64>>(
        FeasibleRegisterAssignment::new(4, vec![(0, 1), (0, 2), (1, 3)], 2, vec![0, 1, 0, 0]),
        &["num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<i64>>(
        IntegerKnapsack::new(vec![3, 4], vec![5, 6], 7).unwrap(),
        &["num_nonzeros"],
        exact,
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
    check_reduced_parameters::<_, ILP<i64>>(
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
        &["num_vars", "num_constraints", "num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, BicliqueCover>(
        BMF::new(vec![vec![true, false], vec![false, true]], 1),
        &["num_vertices", "left_size", "right_size", "rank"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        ILP::<i64>::with_variables(
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
    use crate::models::algebraic::{QuadraticAssignment, BMF, ILP};
    use crate::models::graph::{
        BicliqueCover, HamiltonianCircuit, HamiltonianPath, MinimumVertexCover,
    };
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
    check_reduced_parameters::<_, ILP<i64>>(
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
        &["num_vars", "num_constraints", "num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, ILP<bool>>(
        HamiltonianPath::new(SimpleGraph::new(1, vec![])),
        &["num_vars", "num_constraints", "num_nonzeros"],
        exact,
    );
    check_reduced_parameters::<_, HamiltonianPath<SimpleGraph>>(
        HamiltonianCircuit::new(SimpleGraph::new(0, vec![])),
        &["num_consecutive_positions"],
        ParameterRelation::UpperBound,
    );
    check_reduced_parameters::<_, HamiltonianPath<SimpleGraph>>(
        HamiltonianCircuit::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)])),
        &["num_consecutive_positions"],
        ParameterRelation::UpperBound,
    );
    check_reduced_parameters::<_, LongestCommonSubsequence>(
        MinimumVertexCover::new(SimpleGraph::new(0, vec![]), vec![]),
        &["sum_triangular_lengths"],
        exact,
    );

    let source = BMF::new(vec![vec![true, false], vec![false, true]], 1);
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source).unwrap();
    assert_eq!(
        reduction.target_problem().parameters().get("num_edges"),
        Some(2)
    );
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|entry| entry.source_name == "BMF" && entry.target_name == "BicliqueCover")
        .unwrap();
    let contract = entry.parameter_contract().unwrap();
    assert!(contract.transform().unwrap().get("num_edges").is_none());
    assert!(contract
        .unavailable()
        .iter()
        .any(|field| field.field == "num_edges"));
}
