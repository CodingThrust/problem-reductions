use super::*;
use crate::models::misc::RegisterSufficiency;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Or;

fn feasible_example() -> RegisterSufficiency {
    RegisterSufficiency::new(4, vec![(2, 0), (3, 1)], 2)
}

fn infeasible_example() -> RegisterSufficiency {
    RegisterSufficiency::new(4, vec![(1, 0), (2, 1), (3, 2), (3, 0)], 1)
}

#[allow(dead_code)]
fn canonical_example() -> RegisterSufficiency {
    RegisterSufficiency::new(
        7,
        vec![
            (2, 0),
            (2, 1),
            (3, 1),
            (4, 2),
            (4, 3),
            (5, 0),
            (6, 4),
            (6, 5),
        ],
        3,
    )
}

#[test]
fn test_register_sufficiency_to_ilp_structure() {
    let source = feasible_example();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 62);
    assert_eq!(ilp.constraints().len(), 180);
    assert_eq!(ilp.objective(), vec![]);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_register_sufficiency_to_ilp_closed_loop() {
    let source = feasible_example();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("feasible register-sufficiency instance should yield a feasible ILP");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
    let mut sorted = extracted.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1, 2, 3]);
}

#[test]
fn test_register_sufficiency_to_ilp_infeasible() {
    let source = infeasible_example();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");

    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_err(),
        "register-sufficiency instance with bound one should be infeasible"
    );
}

#[test]
fn test_register_sufficiency_to_ilp_bf_vs_ilp() {
    let source = feasible_example();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}

#[cfg(feature = "example-db")]
#[test]
fn test_register_sufficiency_to_ilp_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "registersufficiency_to_ilp")
        .expect("missing canonical RegisterSufficiency -> ILP example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "RegisterSufficiency");
    assert_eq!(example.target.problem, "ILP");
    assert_eq!(example.source.instance["num_vertices"], 7);
    assert_eq!(example.source.instance["bound"], 3);
    assert_eq!(example.source.instance["arcs"].as_array().unwrap().len(), 8);
    assert_eq!(
        example.target.instance["variables"]
            .as_array()
            .unwrap()
            .len(),
        182
    );
    assert_eq!(
        example.target.instance["constraints"]
            .as_array()
            .unwrap()
            .len(),
        542
    );
    assert_eq!(example.solutions.len(), 1);

    let source = canonical_example();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let solution = &example.solutions[0];
    let source_config: Vec<usize> = serde_json::from_value(solution.source_config.clone()).unwrap();
    let target_config: Vec<i64> = serde_json::from_value(solution.target_config.clone()).unwrap();
    assert_eq!(source.evaluate(&source_config).unwrap(), Or(true));
    assert_eq!(
        reduction.extract_solution(&target_config).unwrap(),
        source_config
    );
}
