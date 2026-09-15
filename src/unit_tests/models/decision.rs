use crate::models::decision::Decision;
use crate::models::graph::{MaximumIndependentSet, MinimumDominatingSet, MinimumVertexCover};
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{One, Or};

fn triangle_mvc() -> MinimumVertexCover<SimpleGraph, i64> {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    MinimumVertexCover::new(graph, vec![1; 3])
}

fn star_mds() -> MinimumDominatingSet<SimpleGraph, One> {
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (0, 4)]);
    MinimumDominatingSet::new(graph, vec![One; 5])
}

#[test]
fn test_decision_min_creation() {
    let mvc = triangle_mvc();
    let decision = Decision::new(mvc, 2);
    assert_eq!(decision.bound(), &2);
    assert_eq!(decision.inner().num_vertices(), 3);
}

#[test]
fn decision_parameters_are_exactly_the_inner_problem_parameters() {
    let inner = triangle_mvc();
    let expected_names = MinimumVertexCover::<SimpleGraph, i64>::parameter_names();
    let expected_parameters = inner.parameters();
    let decision = Decision::new(inner, -1);

    assert_eq!(
        Decision::<MinimumVertexCover<SimpleGraph, i64>>::parameter_names(),
        expected_names
    );
    assert_eq!(decision.parameters(), expected_parameters);
    assert_eq!(decision.parameters().get("bound"), None);
}

#[test]
fn test_decision_min_evaluate_feasible() {
    let decision = Decision::new(triangle_mvc(), 2);
    assert_eq!(
        decision.evaluate(&vec![true, true, false]).unwrap(),
        Or(true)
    );
}

#[test]
fn test_decision_min_evaluate_infeasible_cost() {
    let decision = Decision::new(triangle_mvc(), 1);
    assert_eq!(
        decision.evaluate(&vec![true, true, false]).unwrap(),
        Or(false)
    );
}

#[test]
fn test_decision_min_evaluate_infeasible_config() {
    let decision = Decision::new(triangle_mvc(), 3);
    assert_eq!(
        decision.evaluate(&vec![true, false, false]).unwrap(),
        Or(false)
    );
}

#[test]
fn test_decision_max_evaluate() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let mis = MaximumIndependentSet::new(graph, vec![1; 4]);
    let decision = Decision::new(mis, 2);
    assert_eq!(
        decision.evaluate(&vec![true, false, true, false]).unwrap(),
        Or(true)
    );
    assert_eq!(
        decision.evaluate(&vec![true, false, false, false]).unwrap(),
        Or(false)
    );
}

#[test]
fn test_decision_dims() {
    let decision = Decision::new(triangle_mvc(), 2);
    assert_eq!(
        crate::solvers::cartesian_dimensions(&decision).unwrap(),
        vec![2, 2, 2]
    );
}

#[test]
fn test_decision_solver() {
    let decision = Decision::new(triangle_mvc(), 2);
    let solver = BruteForce::new();
    let witness = solver.solve(&decision).unwrap();
    assert!(witness.is_some());
    let config = witness.unwrap();
    assert_eq!(decision.evaluate(&config).unwrap(), Or(true));
}

#[test]
fn test_decision_serialization() {
    let decision = Decision::new(triangle_mvc(), 2);
    let json = serde_json::to_string(&decision).unwrap();
    let deserialized: Decision<MinimumVertexCover<SimpleGraph, i64>> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.bound(), &2);
    assert_eq!(
        deserialized.evaluate(&vec![true, true, false]).unwrap(),
        Or(true)
    );
}

#[test]
fn construction_contract_decision_uses_flat_inner_fields() {
    let inner = triangle_mvc();
    let mut flat = serde_json::to_value(&inner)
        .unwrap()
        .as_object()
        .unwrap()
        .clone();
    flat.insert("bound".to_string(), serde_json::json!(2));
    let variant = crate::export::variant_to_map(
        <Decision<MinimumVertexCover<SimpleGraph, i64>> as Problem>::variant(),
    );

    let constructed = crate::registry::construct_dyn(
        "DecisionMinimumVertexCover",
        &variant,
        serde_json::Value::Object(flat),
    )
    .unwrap();
    let canonical = constructed.serialize_json();

    assert!(canonical.get("inner").is_some());
    assert_eq!(canonical["bound"], serde_json::json!(2));
    assert_eq!(canonical["inner"]["weights"], serde_json::json!([1, 1, 1]));
}

#[test]
fn construction_contract_decision_rejects_nested_persisted_shape() {
    let variant = crate::export::variant_to_map(
        <Decision<MinimumVertexCover<SimpleGraph, i64>> as Problem>::variant(),
    );
    let error = crate::registry::construct_dyn(
        "DecisionMinimumVertexCover",
        &variant,
        serde_json::json!({"inner": triangle_mvc(), "bound": 2}),
    )
    .err()
    .expect("nested persisted shape must not be accepted for construction");

    assert!(error
        .to_string()
        .contains("unknown construction input(s): inner"));
}

#[test]
fn test_decision_reduce_to_aggregate() {
    use crate::rules::{ReduceTo, ReductionResult};

    let decision = Decision::new(triangle_mvc(), 2);
    let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i64>>::reduce_to(&decision)
        .expect("reduction should succeed");
    let target = result.target_problem();
    assert_eq!(target.num_vertices(), 3);

    let target_val = target.evaluate(&vec![true, true, false]).unwrap();
    let source_val = result.map_value(target_val);
    assert_eq!(source_val, Or(true));

    let target_val = target.evaluate(&vec![true, true, true]).unwrap();
    let source_val = result.map_value(target_val);
    assert_eq!(source_val, Or(false));
}

#[test]
fn test_decision_reduce_to_aggregate_infeasible_bound() {
    use crate::rules::{ReduceTo, ReductionResult};

    let decision = Decision::new(triangle_mvc(), 1);
    let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i64>>::reduce_to(&decision)
        .expect("reduction should succeed");
    let target = result.target_problem();

    for mask in 0..8 {
        let config = vec![mask & 0b001 != 0, mask & 0b010 != 0, mask & 0b100 != 0];
        let target_val = target.evaluate(&config).unwrap();
        let source_val = result.map_value(target_val);
        assert_eq!(
            source_val,
            Or(false),
            "config {config:?} should be infeasible"
        );
    }
}

#[test]
fn test_decision_mds_creation() {
    let mds = star_mds();
    let decision = Decision::new(mds, 1);
    assert_eq!(decision.bound(), &1);
    assert_eq!(decision.inner().num_vertices(), 5);
}

#[test]
fn test_decision_mds_evaluate_feasible() {
    let decision = Decision::new(star_mds(), 1);
    assert_eq!(
        decision
            .evaluate(&vec![true, false, false, false, false])
            .unwrap(),
        Or(true)
    );
}

#[test]
fn test_decision_mds_evaluate_infeasible_cost() {
    let decision = Decision::new(star_mds(), 0);
    assert_eq!(
        decision
            .evaluate(&vec![true, false, false, false, false])
            .unwrap(),
        Or(false)
    );
}

#[test]
fn test_decision_mds_reduce_to_aggregate() {
    use crate::rules::{ReduceTo, ReductionResult};

    let decision = Decision::new(star_mds(), 1);
    let result = ReduceTo::<MinimumDominatingSet<SimpleGraph, One>>::reduce_to(&decision)
        .expect("reduction should succeed");
    let target = result.target_problem();
    assert_eq!(target.num_vertices(), 5);

    let target_val = target
        .evaluate(&vec![true, false, false, false, false])
        .unwrap();
    let source_val = result.map_value(target_val);
    assert_eq!(source_val, Or(true));

    let target_val = target
        .evaluate(&vec![true, true, false, false, false])
        .unwrap();
    let source_val = result.map_value(target_val);
    assert_eq!(source_val, Or(false));
}

#[test]
fn test_decision_mds_solver() {
    let decision = Decision::new(star_mds(), 1);
    let solver = BruteForce::new();
    let witness = solver.solve(&decision).unwrap();
    assert!(witness.is_some());
    let config = witness.unwrap();
    assert_eq!(decision.evaluate(&config).unwrap(), Or(true));
}

#[test]
fn test_decision_mis_unit_registration_and_construction() {
    use crate::models::decision::DecisionCreateSpec;
    use crate::registry::CreateSpec;
    type Unit = MaximumIndependentSet<SimpleGraph, One>;
    type Weighted = MaximumIndependentSet<SimpleGraph, i64>;
    let entries: Vec<_> = crate::registry::variant_entries()
        .into_iter()
        .filter(|e| e.name == "DecisionMaximumIndependentSet")
        .collect();
    assert_eq!(entries.len(), 2);
    let defaults: Vec<_> = entries.iter().filter(|e| e.is_default).collect();
    assert_eq!(defaults.len(), 1);
    assert_eq!(
        defaults[0].variant(),
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    );
    assert!(!DecisionCreateSpec::<Unit>::inputs()
        .iter()
        .any(|input| input.name == "weights"));
    assert!(DecisionCreateSpec::<Weighted>::inputs()
        .iter()
        .any(|input| input.name == "weights"));
    let data = serde_json::json!({"graph":[[0,1],[1,2]],"bound":2});
    let spec: DecisionCreateSpec<Unit> = serde_json::from_value(data.clone()).unwrap();
    let decision: Decision<Unit> = spec.into();
    assert_eq!(decision.evaluate(&vec![true, false, true]), Ok(Or(true)));
    let witness = BruteForce::new().solve(&decision).unwrap().unwrap();
    assert_eq!(decision.evaluate(&witness), Ok(Or(true)));
    let entry = entries.into_iter().find(|e| !e.is_default).unwrap();
    assert!((entry.complexity_eval_fn)(&decision) > 1.0);
    assert_eq!(
        (entry.parameter_measure_fn)(&decision),
        decision.parameters()
    );
    let mut invalid = data;
    invalid["weights"] = serde_json::json!([1, 1, 1]);
    assert!(serde_json::from_value::<DecisionCreateSpec<Unit>>(invalid).is_err());
}

#[test]
fn test_decision_mis_unit_dynamic_identity_edges() {
    let decision = Decision::new(
        MaximumIndependentSet::new(SimpleGraph::path(3), vec![One; 3]),
        2,
    );
    let variant = Decision::<MaximumIndependentSet<SimpleGraph, One>>::variant();
    let entries = crate::rules::registry::reduction_entries();
    let edge = entries
        .iter()
        .find(|e| {
            e.source_name == "DecisionMaximumIndependentSet"
                && e.target_name == "MaximumIndependentSet"
                && (e.source_variant_fn)() == variant
        })
        .unwrap();
    assert_eq!((edge.parameter_declarations_fn)().fields.len(), 2);
    let witness = vec![true, false, true];
    let reduced = (edge.reduce_fn.unwrap())(&decision).unwrap();
    let target = reduced
        .witness
        .target_problem_any()
        .downcast_ref::<MaximumIndependentSet<SimpleGraph, One>>()
        .unwrap();
    assert_eq!(
        target.evaluate(&witness).unwrap(),
        crate::types::Max(Some(2))
    );
    let target_result = SolveOutcome::optimal(target, witness.clone()).unwrap();
    let recovered = reduced
        .witness
        .recover_result_dyn(&decision, crate::solvers::erase_outcome(target_result))
        .unwrap();
    assert_eq!(
        crate::solvers::downcast_outcome::<Vec<bool>, Or>(recovered).unwrap(),
        SolveOutcome::Optimal {
            solution: witness,
            evaluation: Or(true)
        }
    );
    assert!(matches!(
        (edge.reduce_fn.unwrap())(decision.inner()),
        Err(crate::rules::ReductionError::SourceTypeMismatch { .. })
    ));
    assert!(matches!(
        reduced
            .witness
            .recover_result_dyn(decision.inner(), SolveOutcome::Infeasible),
        Err(crate::rules::ExtractionError::InvalidTargetSolution(_))
    ));
    let reverse = entries
        .iter()
        .find(|e| {
            e.source_name == "MaximumIndependentSet"
                && e.target_name == "DecisionMaximumIndependentSet"
                && (e.source_variant_fn)() == variant
        })
        .unwrap();
    assert!(reverse.turing);
    assert!(reverse.reduce_fn.is_none());
    assert_eq!((reverse.parameter_declarations_fn)().fields.len(), 2);
}

#[test]
fn unit_vertex_cover_uses_registered_construction_and_solver() {
    use crate::models::decision::DecisionCreateSpec;
    type Unit = MinimumVertexCover<SimpleGraph, One>;
    let spec: DecisionCreateSpec<Unit> = serde_json::from_value(serde_json::json!({
        "graph": {"num_vertices": 3, "edges": [[0, 1], [1, 2]]}, "bound": 1
    }))
    .unwrap();
    let problem: Decision<Unit> = spec.into();
    let solution = BruteForce::new().solve(&problem).unwrap().unwrap();
    assert_eq!(solution, vec![false, true, false]);
    assert_eq!(problem.evaluate(&solution), Ok(Or(true)));
    let restored: Decision<Unit> =
        serde_json::from_value(serde_json::to_value(&problem).unwrap()).unwrap();
    assert_eq!(restored.evaluate(&solution), Ok(Or(true)));
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
}

#[test]
fn decision_executed_result_maps_witness_and_bound_together() {
    use crate::rules::{ReduceTo, ReductionResult};
    use crate::types::Min;

    let witness = vec![true, true, false];
    for bound in [1, 2] {
        let decision = Decision::new(triangle_mvc(), bound);
        let result =
            <Decision<_> as ReduceTo<MinimumVertexCover<SimpleGraph, i64>>>::reduce_to(&decision)
                .unwrap();
        let target = ReductionResult::target_problem(&result);
        assert!(std::ptr::eq(
            target,
            crate::rules::ReductionResult::target_problem(&result),
        ));
        let value = target.evaluate(&witness).unwrap();
        assert_eq!(result.map_value(value), Or(bound == 2));
        assert_eq!(result.map_value(Min(None)), Or(false));
        if bound == 2 {
            assert_eq!(
                result
                    .recover_result(
                        &decision,
                        SolveOutcome::optimal(result.target_problem(), witness.clone()).unwrap()
                    )
                    .unwrap()
                    .into_solution()
                    .expect("qualifying target result must recover a source solution"),
                witness
            );
        }
    }
}

#[test]
fn recovery_distinguishes_a_proved_no_from_an_insufficient_incumbent() {
    use crate::rules::{ExtractionError, ReduceTo, ReductionResult};
    use SolveOutcome;
    let source = Decision::new(triangle_mvc(), 2);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i64>>::reduce_to(&source).unwrap();
    let candidate = vec![true, true, true];
    assert!(matches!(
        reduction.recover_result(
            &source,
            SolveOutcome::feasible(reduction.target_problem(), candidate).unwrap()
        ),
        Err(ExtractionError::InsufficientSolutionQuality)
    ));
    let optimum = vec![true, true, false];
    for outcome in [
        SolveOutcome::optimal(reduction.target_problem(), optimum.clone()).unwrap(),
        SolveOutcome::feasible(reduction.target_problem(), optimum).unwrap(),
    ] {
        let recovered = reduction.recover_result(&source, outcome).unwrap();
        assert_eq!(
            source.evaluate(recovered.solution().unwrap()).unwrap(),
            Or(true)
        );
    }
    let no_source = Decision::new(triangle_mvc(), 1);
    let no_reduction =
        ReduceTo::<MinimumVertexCover<SimpleGraph, i64>>::reduce_to(&no_source).unwrap();
    assert_eq!(
        no_reduction
            .recover_result(
                &no_source,
                SolveOutcome::optimal(no_reduction.target_problem(), vec![true, true, false])
                    .unwrap()
            )
            .unwrap(),
        SolveOutcome::Infeasible
    );
}
