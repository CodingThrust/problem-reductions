//! Forward-only reduction from SubsetSum to IntegerKnapsack.
//!
//! The construction maps each source element `a_i` to an item with
//! `size_i = value_i = a_i` and sets the knapsack capacity to the target sum.
//! This is the classical NP-hardness embedding, but it is intentionally
//! registered as a proof-only edge because `IntegerKnapsack` allows
//! multiplicities greater than 1. Consequently, an optimal target witness does
//! not always encode a valid 0-1 subset-sum witness (for example `{3}, B=6`).

use crate::expr::Expr;
use crate::models::misc::SubsetSum;
use crate::models::set::IntegerKnapsack;
use crate::rules::{EdgeCapabilities, ReductionEntry, ReductionOverhead};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::any::Any;

#[cfg(feature = "example-db")]
fn biguint_to_i64(value: &BigUint, what: &str) -> i64 {
    value
        .to_i64()
        .unwrap_or_else(|| panic!("SubsetSum -> IntegerKnapsack requires {what} to fit in i64"))
}

fn biguint_to_usize(value: &BigUint, what: &str) -> usize {
    value
        .to_usize()
        .unwrap_or_else(|| panic!("SubsetSum -> IntegerKnapsack requires {what} to fit in usize"))
}

fn subset_sum_source_size(any: &dyn Any) -> ProblemSize {
    let source = any
        .downcast_ref::<SubsetSum>()
        .expect("SubsetSum -> IntegerKnapsack source type mismatch");
    ProblemSize::new(vec![
        ("num_elements", source.num_elements()),
        ("target", biguint_to_usize(source.target(), "target")),
    ])
}

fn subset_sum_to_integer_knapsack_overhead(any: &dyn Any) -> ProblemSize {
    let source = any
        .downcast_ref::<SubsetSum>()
        .expect("SubsetSum -> IntegerKnapsack source type mismatch");
    ProblemSize::new(vec![
        ("num_items", source.num_elements()),
        ("capacity", biguint_to_usize(source.target(), "target")),
    ])
}

inventory::submit! {
    ReductionEntry {
        source_name: SubsetSum::NAME,
        target_name: IntegerKnapsack::NAME,
        source_variant_fn: <SubsetSum as Problem>::variant,
        target_variant_fn: <IntegerKnapsack as Problem>::variant,
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_items", Expr::Var("num_elements")),
            ("capacity", Expr::Var("target")),
        ]),
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::none(),
        overhead_eval_fn: subset_sum_to_integer_knapsack_overhead,
        source_size_fn: subset_sum_source_size,
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::assemble_rule_example;
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_integerknapsack",
        build: || {
            let source = SubsetSum::new(vec![3u32, 7, 1, 8, 5], 16u32);
            let target = IntegerKnapsack::new(
                source
                    .sizes()
                    .iter()
                    .map(|size| biguint_to_i64(size, "sizes"))
                    .collect(),
                source
                    .sizes()
                    .iter()
                    .map(|value| biguint_to_i64(value, "sizes"))
                    .collect(),
                biguint_to_i64(source.target(), "target"),
            );

            assemble_rule_example(
                &source,
                &target,
                vec![SolutionPair {
                    source_config: vec![1, 0, 0, 1, 1],
                    target_config: vec![1, 0, 0, 1, 1],
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_integerknapsack.rs"]
mod tests;
