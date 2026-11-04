//! Reduction from OneInThreeSatisfiability to SubsetProduct.
//!
//! Variable `i` receives a private prime `p_i` and clause `j` a prime `q_j`.
//! Each variable has a false item and a true item, both divisible by `p_i`;
//! the exponent of `q_j` in an item counts the literal occurrences of clause
//! `j` that the item makes true. The target is the product of all primes, so by
//! unique factorization a matching subset picks one item per variable and makes
//! exactly one occurrence true in every clause.

use crate::models::formula::OneInThreeSatisfiability;
use crate::models::misc::SubsetProduct;
use crate::reduction;
use crate::rules::ReductionError;
use crate::rules::exactcoverby3sets_subsetproduct::{assigned_primes, product_biguint};
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_bigint::BigUint;

/// Result of reducing OneInThreeSatisfiability to SubsetProduct.
#[derive(Debug, Clone)]
pub struct ReductionOneInThreeSatisfiabilityToSubsetProduct {
    target: SubsetProduct,
}

impl ReductionResult for ReductionOneInThreeSatisfiabilityToSubsetProduct {
    type Source = OneInThreeSatisfiability;
    type Target = SubsetProduct;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0,
            "target witness does not multiply to the target",
        )?;
        // Item `2i + 1` is the true item of variable `i`.
        Ok(target_solution.iter().skip(1).step_by(2).copied().collect())
    }
}

/// Two items per variable, with the overflow reported as a reduction error.
fn item_count(num_vars: usize) -> Result<usize, ReductionError> {
    num_vars.checked_mul(2).ok_or_else(|| {
        ReductionError::integer_overflow::<OneInThreeSatisfiability, SubsetProduct>(
            "doubling the number of variables",
        )
    })
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionOneInThreeSatisfiabilityToSubsetProduct {}

#[reduction(
    transform = exact {
        num_elements = "2 * num_vars",
    })]
impl ReduceTo<SubsetProduct> for OneInThreeSatisfiability {
    type Result = ReductionOneInThreeSatisfiabilityToSubsetProduct;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let num_vars = self.num_vars();
        // `num_vars + num_clauses` cannot overflow once the item count fits: the
        // clauses are stored in memory, so there are at most `isize::MAX` of them.
        let num_items = item_count(num_vars)?;
        let primes = assigned_primes(num_vars + self.num_clauses());
        let (variable_primes, clause_primes) = primes.split_at(num_vars);

        // Items 2i (false) and 2i + 1 (true), each starting at the private prime p_i.
        let mut sizes: Vec<BigUint> = Vec::with_capacity(num_items);
        for &prime in variable_primes {
            sizes.push(BigUint::from(prime));
            sizes.push(BigUint::from(prime));
        }

        // Each literal occurrence multiplies q_j into the item that makes it true.
        for (clause, &clause_prime) in self.clauses().iter().zip(clause_primes) {
            for (variable, &literal) in clause.variables().into_iter().zip(&clause.literals) {
                let item = 2 * variable + usize::from(literal > 0);
                sizes[item] *= clause_prime;
            }
        }

        let target = product_biguint(primes.iter().copied());
        let target = SubsetProduct::try_new(sizes, target)
            .map_err(<Self as ReduceTo<SubsetProduct>>::target_construction)?;
        Ok(ReductionOneInThreeSatisfiabilityToSubsetProduct { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "oneinthreesatisfiability_to_subsetproduct",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, SubsetProduct>(
                OneInThreeSatisfiability::new(
                    3,
                    vec![
                        CNFClause::new(vec![1, 2, 3]),
                        CNFClause::new(vec![-1, -2, 3]),
                        CNFClause::new(vec![1, -2, -3]),
                    ],
                ),
                SolutionPair {
                    source_config: serde_json::json!(vec![false, true, false]),
                    target_config: serde_json::json!(vec![true, false, false, true, true, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/oneinthreesatisfiability_subsetproduct.rs"]
mod tests;
