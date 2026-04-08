#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use crate::models::misc::SubsetSum;
use crate::models::set::IntegerKnapsack;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Max;
use num_traits::ToPrimitive;

fn subset_sum_embedding(source: &SubsetSum) -> IntegerKnapsack {
    IntegerKnapsack::new(
        source
            .sizes()
            .iter()
            .map(|size| {
                size.to_i64()
                    .expect("test fixture sizes should fit in i64 for IntegerKnapsack")
            })
            .collect(),
        source
            .sizes()
            .iter()
            .map(|value| {
                value
                    .to_i64()
                    .expect("test fixture values should fit in i64 for IntegerKnapsack")
            })
            .collect(),
        source
            .target()
            .to_i64()
            .expect("test fixture target should fit in i64 for IntegerKnapsack"),
    )
}

#[test]
fn test_subsetsum_to_integerknapsack_forward_example() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8, 5], 16u32);
    let target = subset_sum_embedding(&source);
    let source_witness = vec![1, 0, 0, 1, 1];

    assert!(source.evaluate(&source_witness).is_valid());
    assert_eq!(target.evaluate(&source_witness), Max(Some(16)));
}

#[test]
fn test_subsetsum_to_integerknapsack_counterexample_demonstrates_gap() {
    let source = SubsetSum::new(vec![3u32], 6u32);
    let target = subset_sum_embedding(&source);
    let solver = BruteForce::new();

    assert!(solver.find_witness(&source).is_none());
    assert_eq!(solver.solve(&target), Max(Some(6)));
}

#[cfg(feature = "example-db")]
#[test]
fn test_subsetsum_to_integerknapsack_canonical_example_spec() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "subsetsum_to_integerknapsack")
        .expect("missing canonical SubsetSum -> IntegerKnapsack example spec")
        .build)();

    assert_eq!(example.source.problem, "SubsetSum");
    assert_eq!(example.target.problem, "IntegerKnapsack");
    assert_eq!(
        example.target.instance["sizes"],
        serde_json::json!([3, 7, 1, 8, 5])
    );
    assert_eq!(
        example.target.instance["values"],
        serde_json::json!([3, 7, 1, 8, 5])
    );
    assert_eq!(example.target.instance["capacity"], 16);
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(example.solutions[0].source_config, vec![1, 0, 0, 1, 1]);
    assert_eq!(example.solutions[0].target_config, vec![1, 0, 0, 1, 1]);

    let source: SubsetSum = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: IntegerKnapsack = serde_json::from_value(example.target.instance.clone())
        .expect("target example deserializes");

    assert!(source
        .evaluate(&example.solutions[0].source_config)
        .is_valid());
    assert_eq!(
        target.evaluate(&example.solutions[0].target_config),
        Max(Some(16))
    );
}
