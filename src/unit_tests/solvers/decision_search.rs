use super::*;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::solvers::SolveError;
use crate::topology::SimpleGraph;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct FixedObjective<V>(V);

impl<V: DecisionSearchValue> Problem for FixedObjective<V> {
    const NAME: &'static str = "FixedObjective";
    type Solution = Vec<usize>;
    type Value = V;
    fn parameter_names() -> &'static [&'static str] {
        &["num_variables"]
    }
    fn parameters(&self) -> crate::types::ProblemParameters {
        crate::types::ProblemParameters::new(vec![("num_variables", 0)])
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![(
            "objective",
            std::any::type_name::<V>().rsplit("::").next().unwrap(),
        )]
    }
    fn evaluate(&self, _: &Self::Solution) -> Result<V, crate::traits::EvaluationError> {
        Ok(self.0.clone())
    }
}

impl<V: DecisionSearchValue> DecisionProblemMeta for FixedObjective<V> {
    const DECISION_NAME: &'static str = "DecisionFixedObjective";
}

impl<V: DecisionSearchValue> crate::solvers::BruteForceProblem for FixedObjective<V> {
    fn dimensions(&self) -> Vec<usize> {
        vec![]
    }
}

crate::register_brute_force! {
    Decision<FixedObjective<Min<i64>>>,
    Decision<FixedObjective<Max<i64>>>,
}

crate::declare_variants! {
    default Decision<FixedObjective<Min<i64>>> => "1",
    Decision<FixedObjective<Max<i64>>> => "1",
}

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "DecisionFixedObjective",
        display_name: "Fixed Objective Decision Test Problem",
        aliases: &[],
        dimensions: &[crate::registry::VariantDimension::new(
            "objective",
            "Min<i64>",
            &["Min<i64>", "Max<i64>"],
        )],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Fixed objective for decision-search boundary tests",
        fields: &[],
    }
}

#[test]
fn test_decision_search_matches_optimum_at_integer_boundaries() {
    for weight in [i64::MIN, i64::MIN + 1, -3, -1, 0, 1, i64::MAX - 1, i64::MAX] {
        let min = MinimumVertexCover::new(SimpleGraph::new(1, vec![(0, 0)]), vec![weight]);
        let max = MaximumIndependentSet::new(SimpleGraph::empty(1), vec![weight]);
        let maximum = weight.max(0);
        for (lower, upper) in [(i64::MIN, i64::MAX), (weight, weight)] {
            assert_eq!(
                solve_via_decision(&min, lower, upper).unwrap(),
                Some(weight)
            );
        }
        for (lower, upper) in [(i64::MIN, i64::MAX), (maximum, maximum)] {
            assert_eq!(
                solve_via_decision(&max, lower, upper).unwrap(),
                Some(maximum)
            );
        }
    }
}

#[test]
fn test_decision_search_rejects_invalid_or_excluding_intervals() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let min = MinimumVertexCover::new(graph.clone(), vec![1_i64; 3]);
    let max = MaximumIndependentSet::new(graph, vec![1_i64; 3]);
    assert_eq!(solve_via_decision(&min, 0, 3).unwrap(), Some(1));
    assert_eq!(solve_via_decision(&max, 0, 3).unwrap(), Some(2));
    for result in [
        solve_via_decision(&min, 0, 0),
        solve_via_decision(&min, 2, 3),
        solve_via_decision(&max, 0, 1),
        solve_via_decision(&max, 3, 4),
    ] {
        assert!(matches!(
            result,
            Err(SolveError::OptimumOutsideSearchInterval { .. })
        ));
    }
    for result in [
        solve_via_decision(&min, 2, 1),
        solve_via_decision(&max, 2, 1),
    ] {
        assert!(matches!(
            result,
            Err(SolveError::InvalidSearchInterval { lower: 2, upper: 1 })
        ));
    }
}

#[test]
fn test_decision_search_infeasibility_and_evaluation_failure() {
    for (lower, upper) in [(0, 3), (i64::MIN, i64::MAX)] {
        assert_eq!(
            solve_via_decision(&FixedObjective(Min(None)), lower, upper).unwrap(),
            None
        );
        assert_eq!(
            solve_via_decision(&FixedObjective(Max(None)), lower, upper).unwrap(),
            None
        );
    }
    let min = MinimumVertexCover::new(SimpleGraph::empty(2), vec![i64::MAX; 2]);
    let max = MaximumIndependentSet::new(SimpleGraph::empty(2), vec![i64::MIN; 2]);
    for result in [
        solve_via_decision(&min, -3, -1),
        solve_via_decision(&max, 1, 3),
    ] {
        assert!(matches!(result, Err(SolveError::Evaluation(_))));
    }
}

#[test]
fn test_decision_search_matches_brute_force_on_five_cycle() {
    let graph = SimpleGraph::cycle(5);
    let min = MinimumVertexCover::new(graph.clone(), vec![1_i64; 5]);
    let max = MaximumIndependentSet::new(graph, vec![1_i64; 5]);
    let solver = crate::solvers::BruteForce::new();
    let min_witness = solver.solve(&min).unwrap().unwrap();
    let max_witness = solver.solve(&max).unwrap().unwrap();
    assert_eq!(min.evaluate(&min_witness).unwrap().0, Some(3));
    assert_eq!(max.evaluate(&max_witness).unwrap().0, Some(2));
    assert_eq!(
        solve_via_decision(&min, 0, 5).unwrap(),
        min.evaluate(&min_witness).unwrap().0
    );
    assert_eq!(
        solve_via_decision(&max, 0, 5).unwrap(),
        max.evaluate(&max_witness).unwrap().0
    );
}
