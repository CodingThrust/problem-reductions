use crate::models::graph::MaximumIndependentSet;
use crate::models::misc::SubsetSum;
use crate::registry::variant::find_variant_entry;
use crate::registry::{load_dyn, serialize_any, DynProblem, LoadedDynProblem};
use crate::topology::SimpleGraph;
use crate::{Problem, Solver};
use std::any::Any;
use std::collections::BTreeMap;

fn solve_subset_sum(any: &dyn Any) -> Option<(Vec<usize>, String)> {
    let p = any.downcast_ref::<SubsetSum>()?;
    let config = crate::BruteForce::new().find_satisfying(p)?;
    let eval = format!("{:?}", p.evaluate(&config));
    Some((config, eval))
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
fn test_loaded_dyn_problem_delegates_to_solve_fn() {
    let problem = SubsetSum::new(vec![3u32, 7u32, 1u32], 4u32);
    let loaded = LoadedDynProblem::new(Box::new(problem), solve_subset_sum);
    let solved = loaded
        .solve_brute_force()
        .expect("expected satisfying solution");
    assert_eq!(solved.1, "true");
    assert_eq!(solved.0.len(), 3);
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
    assert_eq!(loaded.serialize_json(), serde_json::to_value(&problem).unwrap());
    assert!(loaded.solve_brute_force().is_some());
}

#[test]
fn test_load_dyn_solves_subset_sum() {
    let problem = SubsetSum::new(vec![3u32, 7u32, 1u32], 4u32);
    let variant = BTreeMap::new();
    let loaded =
        load_dyn("SubsetSum", &variant, serde_json::to_value(&problem).unwrap()).unwrap();
    let solved = loaded.solve_brute_force().unwrap();
    assert_eq!(solved.1, "true");
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
    let json =
        serialize_any("MaximumIndependentSet", &variant, &problem as &dyn Any).unwrap();
    assert_eq!(json, serde_json::to_value(&problem).unwrap());
}

#[test]
fn test_serialize_any_rejects_partial_variant() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let partial = BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]);
    assert!(
        serialize_any("MaximumIndependentSet", &partial, &problem as &dyn Any).is_none()
    );
}
