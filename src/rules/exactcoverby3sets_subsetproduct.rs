//! Reduction from ExactCoverBy3Sets to SubsetProduct.
//!
//! Assign a distinct prime to each universe element. Each triple becomes the
//! product of its three primes, and the target is the product of all universe
//! primes. Unique factorization then makes exact covers correspond exactly to
//! subsets whose product matches the target.

use crate::models::misc::SubsetProduct;
use crate::models::set::ExactCoverBy3Sets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

const FIRST_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

#[derive(Debug, Clone)]
pub struct ReductionX3CToSubsetProduct {
    target: SubsetProduct,
}

impl ReductionResult for ReductionX3CToSubsetProduct {
    type Source = ExactCoverBy3Sets;
    type Target = SubsetProduct;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

fn checked_product<I>(values: I, what: &str) -> u64
where
    I: IntoIterator<Item = u64>,
{
    values.into_iter().fold(1u64, |product, value| {
        product.checked_mul(value).unwrap_or_else(|| {
            panic!("ExactCoverBy3Sets -> SubsetProduct requires {what} to fit in u64")
        })
    })
}

fn assigned_primes(universe_size: usize) -> &'static [u64] {
    assert!(
        universe_size <= FIRST_PRIMES.len(),
        "ExactCoverBy3Sets -> SubsetProduct requires the target product to fit in u64; universe_size={universe_size} exceeds the supported limit {}",
        FIRST_PRIMES.len()
    );
    &FIRST_PRIMES[..universe_size]
}

#[reduction(overhead = {
    num_elements = "num_sets",
})]
impl ReduceTo<SubsetProduct> for ExactCoverBy3Sets {
    type Result = ReductionX3CToSubsetProduct;

    fn reduce_to(&self) -> Self::Result {
        let primes = assigned_primes(self.universe_size());
        let values = self
            .sets()
            .iter()
            .map(|set| checked_product(set.iter().map(|&element| primes[element]), "set value"))
            .collect();
        let target = checked_product(primes.iter().copied(), "target product");

        ReductionX3CToSubsetProduct {
            target: SubsetProduct::new(values, target),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_to_subsetproduct",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, SubsetProduct>(
                ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]),
                SolutionPair {
                    source_config: vec![1, 1, 0],
                    target_config: vec![1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/exactcoverby3sets_subsetproduct.rs"]
mod tests;
