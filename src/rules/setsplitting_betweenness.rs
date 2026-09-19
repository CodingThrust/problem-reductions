//! Reduction from Set Splitting to Betweenness.
//!
//! Deduplicate each subset and decompose sizes above 3 using complementarity pairs, then
//! place a single pole element `p` in the Betweenness instance. A size-2
//! subset `{u, v}` becomes `(u, p, v)`, forcing opposite sides of the pole.
//! A size-3 subset `{u, v, w}` becomes `(u, d, v)` and `(d, p, w)` with one
//! fresh auxiliary element `d`, which is satisfiable exactly when the three
//! elements are not monochromatic with respect to the pole.
//! A singleton becomes two incompatible order constraints, preserving infeasibility.

use crate::models::misc::Betweenness;
use crate::models::set::SetSplitting;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SetSplitting to Betweenness.
#[derive(Debug, Clone)]
pub struct ReductionSetSplittingToBetweenness {
    target: Betweenness,
    source_universe_size: usize,
    pole: usize,
}

impl ReductionResult for ReductionSetSplittingToBetweenness {
    type Source = SetSplitting;
    type Target = Betweenness;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !value.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target witness does not satisfy the target problem",
            ));
        }

        let pole_position = target_solution[self.pole];
        Ok(target_solution[..self.source_universe_size]
            .iter()
            .map(|&position| position > pole_position)
            .collect())
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionSetSplittingToBetweenness {
    type Source = SetSplitting;
    type Target = Betweenness;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_value(&self, value: crate::types::Or) -> crate::types::Or {
        value
    }
}

#[reduction(
    transform = unavailable {
        num_elements = "the exact target parameters depend on normalization statistics specific to this reduction",
        num_triples = "the exact target parameters depend on normalization statistics specific to this reduction",
    }
)]
impl ReduceTo<Betweenness> for SetSplitting {
    type Result = ReductionSetSplittingToBetweenness;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let (normalized_universe_size, normalized_subsets) = self.normalized_instance();
        let pole = normalized_universe_size;
        let size3_subsets = normalized_subsets
            .iter()
            .filter(|subset| subset.len() == 3)
            .count();
        let mut triples = Vec::with_capacity(normalized_subsets.len() + size3_subsets);
        let mut num_elements = normalized_universe_size + 1;

        for subset in normalized_subsets {
            match subset.as_slice() {
                [u] => {
                    // A singleton cannot contain both colors. These orders
                    // cannot both hold for three distinct elements.
                    let auxiliary = num_elements;
                    num_elements += 1;
                    triples.push((*u, pole, auxiliary));
                    triples.push((pole, *u, auxiliary));
                }
                [u, v] => triples.push((*u, pole, *v)),
                [u, v, w] => {
                    let auxiliary = num_elements;
                    num_elements += 1;
                    triples.push((*u, auxiliary, *v));
                    triples.push((auxiliary, pole, *w));
                }
                _ => {
                    return Err(crate::rules::ReductionError::invalid_target::<
                        SetSplitting,
                        Betweenness,
                    >(
                        "normalized subset must contain one, two or three elements",
                    ));
                }
            }
        }

        Ok(ReductionSetSplittingToBetweenness {
            target: Betweenness::new(num_elements, triples),
            source_universe_size: self.universe_size(),
            pole,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "setsplitting_to_betweenness",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Betweenness>(
                SetSplitting::new(
                    5,
                    vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 3, 4], vec![1, 2, 3]],
                ),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, true, false, false]),
                    target_config: serde_json::json!(vec![8, 2, 9, 0, 1, 4, 3, 6, 7, 5]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/setsplitting_betweenness.rs"]
mod tests;
