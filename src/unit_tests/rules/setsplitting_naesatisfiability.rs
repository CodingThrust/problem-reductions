use crate::models::formula::NAESatisfiability;
use crate::models::set::SetSplitting;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn rule_example_problem() -> SetSplitting {
    SetSplitting::new(4, vec![vec![0, 1], vec![1, 2, 3], vec![0, 2, 3]])
}

#[test]
fn test_setsplitting_to_naesatisfiability_closed_loop() {
    let source = rule_example_problem();
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SetSplitting -> NAE-SAT",
    );
}

#[test]
fn test_setsplitting_to_naesatisfiability_structure() {
    let source = rule_example_problem();
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 4);
    assert_eq!(target.num_clauses(), 3);
    assert_eq!(
        target
            .clauses()
            .iter()
            .map(|clause| clause.literals.clone())
            .collect::<Vec<_>>(),
        vec![vec![1, 2], vec![2, 3, 4], vec![1, 3, 4]],
    );
}

#[test]
fn test_setsplitting_to_naesatisfiability_repeated_members() {
    let source = SetSplitting::new(1, vec![vec![0, 0]]);
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");

    assert_eq!(reduction.target_problem().clauses()[0].literals, [1, 1]);
    assert!(BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .is_none());
}

#[test]
fn test_setsplitting_to_naesatisfiability_extract_solution() {
    let source = rule_example_problem();
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");

    assert_eq!(
        reduction
            .extract_solution(&vec![true, false, true, false])
            .unwrap(),
        vec![true, false, true, false]
    );
}

#[test]
fn test_setsplitting_to_naesatisfiability_target_witness_extracts_to_satisfying_assignment() {
    let source = rule_example_problem();
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let solver = BruteForce::new();

    let target_solution = solver.solve(reduction.target_problem()).unwrap().unwrap();
    let source_solution = reduction.extract_solution(&target_solution).unwrap();

    assert!(source.evaluate(&source_solution).unwrap());
}

#[test]
fn test_setsplitting_to_naesatisfiability_rejects_invalid_witnesses() {
    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&rule_example_problem()).unwrap();
    for witness in [vec![], vec![true; 3], vec![true; 5], vec![false; 4]] {
        assert!(reduction.extract_solution(&witness).is_err());
    }
}

#[test]
fn test_setsplitting_to_naesatisfiability_all_small_assignments() {
    // Every pair of nonempty subsets on up to four elements, with zero
    // standing for an omitted subset. Duplicate occurrences exercise the
    // set semantics, including subsets with only one distinct member.
    for n in 0..=4 {
        for first in 0..(1 << n) {
            for second in 0..(1 << n) {
                let subsets = [first, second]
                    .into_iter()
                    .filter(|&mask| mask != 0)
                    .map(|mask| {
                        let mut subset: Vec<_> = (0..n).filter(|&i| mask & (1 << i) != 0).collect();
                        subset.push(subset[0]);
                        subset
                    })
                    .collect();
                let source = SetSplitting::new(n, subsets);
                let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&source).unwrap();
                for mask in 0..(1 << n) {
                    let witness: Vec<_> = (0..n).map(|i| mask & (1 << i) != 0).collect();
                    let feasible = source.evaluate(&witness).unwrap().0;
                    assert_eq!(
                        reduction.target_problem().evaluate(&witness).unwrap().0,
                        feasible,
                        "n={n}, subsets={:?}, witness={witness:?}",
                        source.subsets()
                    );
                    if feasible {
                        assert_eq!(reduction.extract_solution(&witness).unwrap(), witness);
                    } else {
                        assert!(reduction.extract_solution(&witness).is_err());
                    }
                }
            }
        }
    }
}

#[test]
fn test_setsplitting_to_naesatisfiability_parameter_bounds() {
    use crate::parameters::ParameterRelation;

    let entries = crate::rules::registry::reduction_entries();
    let entry = entries
        .iter()
        .find(|e| e.source_name == "SetSplitting" && e.target_name == "NAESatisfiability")
        .unwrap();
    let contract = entry.parameter_contract().unwrap();
    let transform = contract.transform().unwrap();
    for source in [
        SetSplitting::new(0, vec![]),
        SetSplitting::new(4, vec![]),
        SetSplitting::new(1, vec![vec![0, 0, 0]; 4]),
        SetSplitting::new(2, vec![vec![0, 1]; 4]),
        SetSplitting::new(5, vec![vec![3, 1, 0, 2, 3, 1]]),
    ] {
        let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&source).unwrap();
        let actual = reduction.target_problem().parameters();
        let predicted = transform.evaluate(&source.parameters()).unwrap();
        for (field, _) in transform.expressions() {
            let measured = actual.get(field).unwrap();
            let bound = predicted.get(field).unwrap();
            match transform.relation(field).unwrap() {
                ParameterRelation::Exact => assert_eq!(measured, bound, "{field}"),
                ParameterRelation::UpperBound => assert!(measured <= bound, "{field}"),
            }
        }
    }
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_setsplitting_to_naesatisfiability_literal_domain_boundary() {
    let largest = usize::try_from(i64::MAX).unwrap();
    // Empty families exercise the universe limit without huge allocations.
    let source = SetSplitting::new(largest, vec![]);
    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vars(), largest);
    for n in [largest + 1, usize::MAX] {
        let source = SetSplitting::new(n, vec![]);
        assert!(matches!(
            ReduceTo::<NAESatisfiability>::reduce_to(&source),
            Err(crate::rules::ReductionError::Construction { .. })
        ));
    }
    let source = SetSplitting::new(largest, vec![vec![largest - 1, largest - 1]]);
    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&source).unwrap();
    assert_eq!(
        reduction.target_problem().clauses()[0].literals,
        [i64::MAX; 2]
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_setsplitting_to_naesatisfiability_canonical_example_spec() {
    let specs = crate::rules::setsplitting_naesatisfiability::canonical_rule_example_specs();
    assert_eq!(specs.len(), 1);

    let example = (specs[0].build)();
    assert_eq!(example.source.problem, "SetSplitting");
    assert_eq!(example.target.problem, "NAESatisfiability");
    assert_eq!(example.solutions.len(), 1);

    let pair = &example.solutions[0];
    assert_eq!(
        pair.source_config,
        serde_json::json!([true, false, true, false])
    );
    assert_eq!(
        pair.target_config,
        serde_json::json!([true, false, true, false])
    );
}
