#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use crate::models::misc::SubsetSum;
use crate::models::set::IntegerKnapsack;
use crate::solvers::BruteForce;
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
    .unwrap()
}

#[test]
fn test_subsetsum_to_integerknapsack_forward_example() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8, 5], 16u32);
    let target = subset_sum_embedding(&source);
    let source_witness = vec![true, false, false, true, true];

    assert!(source.evaluate(&source_witness).unwrap().is_valid());
    let target_witness = source_witness.iter().copied().map(usize::from).collect();
    assert_eq!(target.evaluate(&target_witness).unwrap(), Max(Some(16)));
}

#[test]
fn test_subsetsum_to_integerknapsack_counterexample_demonstrates_gap() {
    let source = SubsetSum::new(vec![3u32], 6u32);
    let target = subset_sum_embedding(&source);
    let solver = BruteForce::new();

    assert!(solver.solve(&source).unwrap().is_none());
    assert_eq!(
        target
            .evaluate(&solver.solve(&target).unwrap().unwrap())
            .unwrap(),
        Max(Some(6))
    );
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
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!([true, false, false, true, true])
    );
    assert_eq!(
        example.solutions[0].target_config,
        serde_json::json!([1, 0, 0, 1, 1])
    );

    let source: SubsetSum = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: IntegerKnapsack = serde_json::from_value(example.target.instance.clone())
        .expect("target example deserializes");

    let source_config: Vec<bool> =
        serde_json::from_value(example.solutions[0].source_config.clone()).unwrap();
    let target_config: Vec<usize> =
        serde_json::from_value(example.solutions[0].target_config.clone()).unwrap();
    assert!(source.evaluate(&source_config).unwrap().is_valid());
    assert_eq!(target.evaluate(&target_config).unwrap(), Max(Some(16)));
}
