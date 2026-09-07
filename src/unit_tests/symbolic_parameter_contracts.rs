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
