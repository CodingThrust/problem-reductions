use super::*;
#[test]
fn create_spec_requires_bundle_coverage() {
    assert!(
        IntegralFlowBundles::try_from(IntegralFlowBundlesCreateSpec {
            arcs: vec![(0, 1), (1, 2)],
            num_vertices: None,
            bundles: vec![vec![0]],
            bundle_capacities: vec![1],
            source: 0,
            sink: 2,
            requirement: 1
        })
        .is_err()
    );
}
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn yes_instance() -> IntegralFlowBundles {
    IntegralFlowBundles::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2), (2, 1)]),
        0,
        3,
        vec![vec![0, 1], vec![2, 5], vec![3, 4]],
        vec![1, 1, 1],
        1,
    )
}

fn no_instance() -> IntegralFlowBundles {
    IntegralFlowBundles::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2), (2, 1)]),
        0,
        3,
        vec![vec![0, 1], vec![2, 5], vec![3, 4]],
        vec![1, 1, 1],
        2,
    )
}

fn satisfying_config() -> Vec<usize> {
    vec![1, 0, 1, 0, 0, 0]
}

#[test]
fn test_integral_flow_bundles_creation_and_getters() {
    let problem = yes_instance();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_arcs(), 6);
    assert_eq!(problem.num_bundles(), 3);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 3);
    assert_eq!(problem.requirement(), 1);
    assert_eq!(problem.bundle_capacities(), &[1, 1, 1]);
    assert_eq!(problem.graph().arcs().len(), 6);
}

#[test]
fn test_integral_flow_bundles_dims_use_tight_arc_bounds() {
    let problem = yes_instance();
    assert_eq!(
        crate::solvers::cartesian_dimensions(&problem).unwrap(),
        vec![2, 2, 2, 2, 2, 2]
    );
}

#[test]
fn test_integral_flow_bundles_evaluate_yes_and_no_examples() {
    let yes = yes_instance();
    let no = no_instance();
    let config = satisfying_config();
    assert!(yes.evaluate(&config).unwrap());
    assert!(!no.evaluate(&config).unwrap());
    assert!(yes.is_valid_solution(&config).unwrap());
}

#[test]
fn test_integral_flow_bundles_rejects_bad_bundle_sum_or_conservation() {
    let problem = yes_instance();

    let mut bundle_violation = satisfying_config();
    bundle_violation[1] = 1;
    assert!(!problem.evaluate(&bundle_violation).unwrap());

    let conservation_violation = vec![1, 0, 0, 0, 0, 0];
    assert!(!problem.evaluate(&conservation_violation).unwrap());
}

#[test]
fn test_integral_flow_bundles_solver_and_paper_example() {
    let problem = yes_instance();
    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&problem).unwrap();
    assert!(!all.is_empty());
    assert!(all.contains(&satisfying_config()));
    assert!(problem.evaluate(&satisfying_config()).unwrap());
}

#[test]
fn test_integral_flow_bundles_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let roundtrip: IntegralFlowBundles = serde_json::from_str(&json).unwrap();
    assert_eq!(roundtrip.num_vertices(), 4);
    assert_eq!(roundtrip.num_arcs(), 6);
    assert_eq!(roundtrip.num_bundles(), 3);
    assert_eq!(roundtrip.requirement(), 1);
}

#[test]
fn test_integral_flow_bundles_problem_name() {
    assert_eq!(
        <IntegralFlowBundles as Problem>::NAME,
        "IntegralFlowBundles"
    );
}

#[test]
fn creation_and_deserialization_enforce_the_same_flow_constraints() {
    let input = serde_json::json!({
        "arcs": [[0, 1], [1, 2]], "num_vertices": 3,
        "source": 0, "sink": 2, "requirement": 1,
        "bundles": [[0], [1]], "bundle_capacities": [1, 1],
    });
    let problem = IntegralFlowBundles::try_from(
        serde_json::from_value::<IntegralFlowBundlesCreateSpec>(input.clone()).unwrap(),
    )
    .unwrap();
    let persisted = serde_json::to_value(&problem).unwrap();
    let restored: IntegralFlowBundles = serde_json::from_value(persisted.clone()).unwrap();
    assert_eq!(
        restored.evaluate(&vec![1, 1]).unwrap(),
        crate::types::Or(true)
    );

    for (field, value, message) in [
        ("source", serde_json::json!(3), "source"),
        ("sink", serde_json::json!(3), "sink"),
        ("sink", serde_json::json!(0), "distinct"),
        ("bundle_capacities", serde_json::json!([1]), "length"),
        ("requirement", serde_json::json!(0), "positive"),
        ("requirement", serde_json::json!(-1), "positive"),
        ("bundle_capacities", serde_json::json!([0, 1]), "positive"),
        ("bundle_capacities", serde_json::json!([-1, 1]), "positive"),
        ("bundles", serde_json::json!([[2], [1]]), "out of range"),
        ("bundles", serde_json::json!([[0, 0], [1]]), "duplicate"),
        (
            "bundles",
            serde_json::json!([[0], []]),
            "at least one bundle",
        ),
    ] {
        let mut invalid_input = input.clone();
        invalid_input[field] = value.clone();
        let spec = serde_json::from_value::<IntegralFlowBundlesCreateSpec>(invalid_input).unwrap();
        let error = IntegralFlowBundles::try_from(spec).unwrap_err();
        assert!(error.to_string().contains(message), "{field}: {error}");

        let mut invalid_persisted = persisted.clone();
        invalid_persisted[field] = value;
        let error = serde_json::from_value::<IntegralFlowBundles>(invalid_persisted).unwrap_err();
        assert!(error.to_string().contains(message), "{field}: {error}");
    }
}
