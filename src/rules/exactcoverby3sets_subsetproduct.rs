//! Reduction from ExactCoverBy3Sets to SubsetProduct.
//!
//! Assign a distinct prime to each universe element. Each triple becomes the
//! product of its three primes, and the target is the product of all universe
//! primes. Unique factorization then makes exact covers correspond exactly to
//! subsets whose product matches the target.

use crate::models::formula::ksat::first_n_odd_primes;
use crate::models::misc::SubsetProduct;
use crate::models::set::ExactCoverBy3Sets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_bigint::BigUint;
use num_traits::One;

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

fn product_biguint<I>(values: I) -> BigUint
where
    I: IntoIterator<Item = u64>,
{
    values.into_iter().fold(BigUint::one(), |product, value| {
        product * BigUint::from(value)
    })
}

fn assigned_primes(universe_size: usize) -> Vec<u64> {
    match universe_size {
        0 => Vec::new(),
        1 => vec![2],
        _ => {
            let mut primes = Vec::with_capacity(universe_size);
            primes.push(2);
            primes.extend(first_n_odd_primes(universe_size - 1));
            primes
        }
    }
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
            .map(|set| product_biguint(set.iter().map(|&element| primes[element])))
            .collect();
        let target = product_biguint(primes.iter().copied());

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
