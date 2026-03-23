use crate::models::graph::MaximumIndependentSet;
use crate::models::graph::MinimumVertexCover;
use crate::models::misc::SubsetSum;
use crate::registry::variant::find_variant_entry;
use crate::registry::{load_dyn, serialize_any, DynProblem, LoadedDynProblem};
use crate::topology::SimpleGraph;
use crate::types::Sum;
use crate::{Problem, Solver};
use std::any::Any;
use std::collections::BTreeMap;

fn solve_subset_sum_value(any: &dyn Any) -> String {
    let p = any.downcast_ref::<SubsetSum>().unwrap();
    if let Some(config) = crate::BruteForce::new().find_witness(p) {
        format!("{:?}", p.evaluate(&config))
    } else {
        "false".to_string()
    }
}

fn solve_subset_sum_witness(any: &dyn Any) -> Option<(Vec<usize>, String)> {
    let p = any.downcast_ref::<SubsetSum>()?;
    let config = crate::BruteForce::new().find_witness(p)?;
    let eval = format!("{:?}", p.evaluate(&config));
    Some((config, eval))
}

#[derive(Clone, serde::Serialize)]
struct AggregateOnlyProblem {
    weights: Vec<u64>,
}

impl Problem for AggregateOnlyProblem {
    const NAME: &'static str = "AggregateOnlyProblem";
    type Value = Sum<u64>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Sum(config
            .iter()
            .zip(&self.weights)
            .map(|(&c, &w)| if c == 1 { w } else { 0 })
            .sum())
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

fn solve_aggregate_value(any: &dyn Any) -> String {
    let p = any.downcast_ref::<AggregateOnlyProblem>().unwrap();
    format!("{:?}", crate::BruteForce::new().solve(p))
}

fn solve_aggregate_witness(_: &dyn Any) -> Option<(Vec<usize>, String)> {
    None
}

#[test]
fn test_dyn_problem_blanket_impl_exposes_problem_metadata() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let dyn_problem: &dyn DynProblem = &problem;

    assert_eq!(dyn_problem.problem_name(), "MaximumIndependentSet");
    assert_eq!(dyn_problem.num_variables_dyn(), 3);
    assert_eq!(dyn_problem.dims_dyn(), vec![2, 2, 2]);
    assert_eq!(dyn_problem.variant_map()["graph"], "SimpleGraph");
    assert!(dyn_problem.serialize_json().is_object());
}

#[test]
fn test_dyn_problem_formats_optimization_values_as_legacy_valid_invalid() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let dyn_problem: &dyn DynProblem = &problem;

    assert_eq!(dyn_problem.evaluate_dyn(&[1, 0, 1]), "Valid(2)");
    assert_eq!(dyn_problem.evaluate_dyn(&[1, 1, 0]), "Invalid");
}

#[test]
fn test_loaded_dyn_problem_delegates_to_value_and_witness_fns() {
    let problem = SubsetSum::new(vec![3u32, 7u32, 1u32], 4u32);
    let loaded = LoadedDynProblem::new(
        Box::new(problem),
        solve_subset_sum_value,
        solve_subset_sum_witness,
    );

    assert_eq!(loaded.solve_brute_force_value(), "Or(true)");
    let solved = loaded
        .solve_brute_force_witness()
        .expect("expected satisfying solution");
    assert_eq!(solved.1, "Or(true)");
    assert_eq!(solved.0.len(), 3);
}

#[test]
fn loaded_dyn_problem_returns_none_for_aggregate_only_witness() {
    let loaded = LoadedDynProblem::new(
        Box::new(AggregateOnlyProblem {
            weights: vec![1, 2, 4],
        }),
        solve_aggregate_value,
        solve_aggregate_witness,
    );

    assert_eq!(loaded.solve_brute_force_value(), "Sum(28)");
    assert!(loaded.solve_brute_force_witness().is_none());
}

#[test]
fn test_load_dyn_formats_optimization_solve_values_as_legacy_valid_invalid() {
    let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let loaded = load_dyn(
        "MinimumVertexCover",
        &variant,
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap();

    assert_eq!(loaded.solve_brute_force_value(), "Valid(1)");
    let solved = loaded.solve_brute_force_witness().unwrap();
    assert_eq!(solved.1, "Valid(1)");
}

#[test]
fn test_find_variant_entry_requires_exact_variant() {
    let partial = BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]);
    assert!(find_variant_entry("MaximumIndependentSet", &partial).is_none());
}

#[test]
fn test_load_dyn_round_trips_maximum_independent_set() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let loaded = load_dyn(
        "MaximumIndependentSet",
        &variant,
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap();

    assert_eq!(loaded.problem_name(), "MaximumIndependentSet");
    assert_eq!(
        loaded.serialize_json(),
        serde_json::to_value(&problem).unwrap()
    );
    assert!(!loaded.solve_brute_force_value().is_empty());
    assert!(loaded.solve_brute_force_witness().is_some());
}

#[test]
fn test_load_dyn_solves_subset_sum() {
    let problem = SubsetSum::new(vec![3u32, 7u32, 1u32], 4u32);
    let variant = BTreeMap::new();
    let loaded = load_dyn(
        "SubsetSum",
        &variant,
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap();

    assert_eq!(loaded.solve_brute_force_value(), "Or(true)");
    let solved = loaded.solve_brute_force_witness().unwrap();
    assert_eq!(solved.1, "Or(true)");
}

#[test]
fn test_load_dyn_rejects_partial_variant() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let partial = BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]);
    let err = load_dyn(
        "MaximumIndependentSet",
        &partial,
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap_err();

    assert!(err.contains("MaximumIndependentSet"));
}

#[test]
fn test_load_dyn_rejects_alias_name() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    assert!(load_dyn("MIS", &variant, serde_json::to_value(&problem).unwrap()).is_err());
}

#[test]
fn test_serialize_any_round_trips_exact_variant() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let json = serialize_any("MaximumIndependentSet", &variant, &problem as &dyn Any).unwrap();
    assert_eq!(json, serde_json::to_value(&problem).unwrap());
}

#[test]
fn test_serialize_any_rejects_partial_variant() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let partial = BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]);
    assert!(serialize_any("MaximumIndependentSet", &partial, &problem as &dyn Any).is_none());
}
