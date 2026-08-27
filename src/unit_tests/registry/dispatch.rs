use crate::models::graph::MaximumIndependentSet;
use crate::models::graph::MinimumVertexCover;
use crate::models::misc::SubsetSum;
use crate::registry::variant::find_variant_entry;
use crate::registry::{load_dyn, serialize_any, DynProblem, LoadedDynProblem};
use crate::solvers::{brute_force_dimensions, solve, SolveOutcome, SolverRequest};
use crate::topology::SimpleGraph;
use crate::types::{Max, Sum};
use crate::Problem;
use std::any::Any;
use std::collections::BTreeMap;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct SolutionProblem {
    weights: Vec<u64>,
}

impl SolutionProblem {
    fn num_variables(&self) -> usize {
        self.weights.len()
    }
}

impl Problem for SolutionProblem {
    const NAME: &'static str = "SolutionProblem";
    type Solution = Vec<usize>;
    type Value = Max<u64>;

    crate::problem_size![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            Max(Some(
                config
                    .iter()
                    .zip(&self.weights)
                    .map(|(&c, &w)| if c == 1 { w } else { 0 })
                    .sum(),
            ))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl crate::solvers::BruteForceProblem for SolutionProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
}

crate::declare_variants! {
    default SolutionProblem => "2^num_variables",
}

crate::register_brute_force! {
    SolutionProblem decode |_, indices: Vec<usize>| indices,
}

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "SolutionProblem",
        display_name: "Solution Test Problem",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Test problem for solution-producing reference solving",
        fields: &[],
    }
}

#[test]
fn test_dyn_problem_blanket_impl_exposes_problem_metadata() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
    let dyn_problem: &dyn DynProblem = &problem;

    assert_eq!(dyn_problem.problem_name(), "MaximumIndependentSet");
    assert_eq!(dyn_problem.variant_map()["graph"], "SimpleGraph");
    assert_eq!(
        dyn_problem.size_parameter_names_dyn(),
        MaximumIndependentSet::<SimpleGraph, i64>::size_parameter_names()
    );
    assert_eq!(dyn_problem.size_dyn(), problem.size());
    assert_eq!(problem.size().get("num_vertices"), Some(3));
    assert_eq!(problem.size().get("num_edges"), Some(1));
    assert!(dyn_problem.serialize_json().is_object());
}

#[test]
fn test_dyn_problem_formats_optimization_values_as_max_min() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
    let dyn_problem: &dyn DynProblem = &problem;

    assert_eq!(
        dyn_problem
            .evaluate_dyn(&serde_json::json!([true, false, true]))
            .unwrap(),
        "Max(2)"
    );
    assert_eq!(
        dyn_problem
            .evaluate_dyn(&serde_json::json!([true, true, false]))
            .unwrap(),
        "Max(None)"
    );
}

#[test]
fn test_loaded_dyn_problem_delegates_to_solve_fn() {
    let problem = SubsetSum::new(vec![3u32, 7u32, 1u32], 4u32);
    let loaded = LoadedDynProblem::new(Box::new(problem));

    assert_eq!(
        brute_force_dimensions(&loaded).unwrap(),
        Some(vec![2, 2, 2])
    );
    let solved = solve(&loaded, SolverRequest::BruteForce).unwrap();
    let SolveOutcome::Optimal {
        solution,
        evaluation,
    } = solved.outcome
    else {
        panic!("expected satisfying solution");
    };
    assert_eq!(evaluation, "Or(true)");
    assert_eq!(solution.as_array().unwrap().len(), 3);
}

#[test]
fn loaded_dyn_problem_returns_solution_and_evaluation() {
    let problem = SolutionProblem {
        weights: vec![1, 2, 4],
    };
    let loaded = LoadedDynProblem::new(Box::new(problem));

    let solved = solve(&loaded, SolverRequest::BruteForce).unwrap();
    let SolveOutcome::Optimal {
        solution,
        evaluation,
    } = solved.outcome
    else {
        panic!("expected optimal solution");
    };
    assert_eq!(solution, serde_json::json!([1, 1, 1]));
    assert_eq!(evaluation, "Max(7)");
}

#[test]
fn test_load_dyn_formats_optimization_solve_values_as_max_min() {
    let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i64; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i64".to_string()),
    ]);
    let loaded = load_dyn(
        "MinimumVertexCover",
        &variant,
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap();

    let solved = solve(&loaded, SolverRequest::BruteForce).unwrap();
    let SolveOutcome::Optimal { evaluation, .. } = solved.outcome else {
        panic!("expected optimal solution");
    };
    assert_eq!(evaluation, "Min(1)");
}

#[test]
fn test_find_variant_entry_requires_exact_variant() {
    let partial = BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]);
    assert!(find_variant_entry("MaximumIndependentSet", &partial).is_none());
}

#[test]
fn test_load_dyn_round_trips_maximum_independent_set() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i64".to_string()),
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
    assert!(matches!(
        solve(&loaded, SolverRequest::BruteForce).unwrap().outcome,
        SolveOutcome::Optimal { .. }
    ));
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

    let solved = solve(&loaded, SolverRequest::BruteForce).unwrap();
    let SolveOutcome::Optimal { evaluation, .. } = solved.outcome else {
        panic!("expected satisfying solution");
    };
    assert_eq!(evaluation, "Or(true)");
}

#[test]
fn test_load_dyn_rejects_partial_variant() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
    let partial = BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]);
    let err = load_dyn(
        "MaximumIndependentSet",
        &partial,
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap_err();

    assert!(err.to_string().contains("MaximumIndependentSet"));
}

#[test]
fn test_load_dyn_rejects_alias_name() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i64".to_string()),
    ]);
    assert!(load_dyn("MIS", &variant, serde_json::to_value(&problem).unwrap()).is_err());
}

#[test]
fn test_serialize_any_round_trips_exact_variant() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i64".to_string()),
    ]);
    let json = serialize_any("MaximumIndependentSet", &variant, &problem as &dyn Any).unwrap();
    assert_eq!(json, serde_json::to_value(&problem).unwrap());
}

#[test]
fn test_serialize_any_rejects_partial_variant() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
    let partial = BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]);
    assert!(serialize_any("MaximumIndependentSet", &partial, &problem as &dyn Any).is_none());
}

#[test]
fn test_format_metric_uses_display() {
    use crate::registry::dyn_problem::format_metric;
    use crate::types::{Max, Min, Or};
    assert_eq!(format_metric(&Max(Some(42))), "Max(42)");
    assert_eq!(format_metric(&Max::<i64>(None)), "Max(None)");
    assert_eq!(format_metric(&Min(Some(7))), "Min(7)");
    assert_eq!(format_metric(&Or(true)), "Or(true)");
    assert_eq!(format_metric(&Sum(99u64)), "Sum(99)");
}

#[test]
fn test_loaded_dyn_problem_debug() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i64".to_string()),
    ]);
    let loaded = load_dyn(
        "MaximumIndependentSet",
        &variant,
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap();
    let debug = format!("{:?}", loaded);
    assert!(debug.contains("LoadedDynProblem"));
    assert!(debug.contains("MaximumIndependentSet"));
}
