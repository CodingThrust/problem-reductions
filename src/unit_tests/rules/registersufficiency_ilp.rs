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
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 24);
    assert_eq!(ilp.constraints().len(), 36);
    assert_eq!(ilp.objective(), vec![]);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_register_sufficiency_to_ilp_closed_loop() {
    let source = feasible_example();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");

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
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");

    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_err(),
        "register-sufficiency instance with bound one should be infeasible"
    );
}

#[test]
fn test_register_sufficiency_to_ilp_bf_vs_ilp() {
    let source = feasible_example();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
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
        91
    );
    assert_eq!(
        example.target.instance["constraints"]
            .as_array()
            .unwrap()
            .len(),
        168
    );
    assert_eq!(example.solutions.len(), 1);

    let source = canonical_example();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
    let solution = &example.solutions[0];
    let source_config: Vec<usize> = serde_json::from_value(solution.source_config.clone()).unwrap();
    let target_config: Vec<i64> = serde_json::from_value(solution.target_config.clone()).unwrap();
    assert_eq!(source.evaluate(&source_config).unwrap(), Or(true));
    assert_eq!(
        reduction.extract_solution(&target_config).unwrap(),
        source_config
    );
}

#[test]
fn cumulative_schedule_preserves_live_values_and_rejects_invalid_targets() {
    for (source, feasible) in [
        (RegisterSufficiency::new(0, vec![], 0), true),
        (RegisterSufficiency::new(3, vec![(1, 0), (2, 1)], 1), true),
        (RegisterSufficiency::new(3, vec![(2, 0), (2, 1)], 1), false),
        (RegisterSufficiency::new(3, vec![(2, 0), (2, 1)], 2), true),
        (RegisterSufficiency::new(3, vec![], 2), false),
        (RegisterSufficiency::new(3, vec![], 3), true),
        (RegisterSufficiency::new(2, vec![(1, 0), (0, 1)], 2), false),
        (RegisterSufficiency::new(2, vec![(1, 0), (1, 0)], 1), true),
        (RegisterSufficiency::new(1, vec![], 0), false),
    ] {
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        match ILPSolver::new().solve(reduction.target_problem()) {
            Ok(solution) => {
                assert!(feasible);
                assert_eq!(
                    source
                        .evaluate(&reduction.extract_solution(&solution).unwrap())
                        .unwrap(),
                    Or(true)
                );
            }
            Err(crate::solvers::ILPSolveError::Infeasible) => assert!(!feasible),
            Err(error) => panic!("unexpected solver failure: {error}"),
        }
        if source.num_vertices() > 0 {
            assert!(reduction
                .extract_solution(&vec![0; reduction.target_problem().num_vars()])
                .is_err());
        }
    }
}

#[test]
fn cumulative_schedule_decodes_every_feasible_target_with_surplus_live_bits() {
    let source = RegisterSufficiency::new(2, vec![(1, 0)], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    let mut feasible = 0;
    for bits in 0..1 << target.num_vars() {
        let solution: Vec<_> = (0..target.num_vars())
            .map(|i| i64::from(bits & (1 << i) != 0))
            .collect();
        if target.evaluate(&solution).unwrap().value.is_some() {
            assert_eq!(reduction.extract_solution(&solution).unwrap(), vec![0, 1]);
            feasible += 1;
        }
    }
    // The first live bit is forced; the final one may be either value.
    assert_eq!(feasible, 2);
}
