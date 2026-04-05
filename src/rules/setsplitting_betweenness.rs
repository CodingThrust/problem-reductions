//! Reduction from Set Splitting to Betweenness.
//!
//! Decompose each subset to size 2 or 3 using complementarity pairs, then
//! place a single pole element `p` in the Betweenness instance. A size-2
//! subset `{u, v}` becomes `(u, p, v)`, forcing opposite sides of the pole.
//! A size-3 subset `{u, v, w}` becomes `(u, d, v)` and `(d, p, w)` with one
//! fresh auxiliary element `d`, which is satisfiable exactly when the three
//! elements are not monochromatic with respect to the pole.

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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        assert!(
            target_solution.len() > self.pole,
            "Betweenness solution has {} positions but pole index is {}",
            target_solution.len(),
            self.pole
        );
        assert!(
            target_solution.len() >= self.source_universe_size,
            "Betweenness solution has {} positions but source requires {} elements",
            target_solution.len(),
            self.source_universe_size
        );

        let pole_position = target_solution[self.pole];
        target_solution[..self.source_universe_size]
            .iter()
            .map(|&position| usize::from(position > pole_position))
            .collect()
    }
}

#[reduction(
    overhead = {
        num_elements = "normalized_universe_size + 1 + normalized_num_size3_subsets",
        num_triples = "normalized_num_size2_subsets + 2 * normalized_num_size3_subsets",
    }
)]
impl ReduceTo<Betweenness> for SetSplitting {
    type Result = ReductionSetSplittingToBetweenness;

    fn reduce_to(&self) -> Self::Result {
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
                [u, v] => triples.push((*u, pole, *v)),
                [u, v, w] => {
                    let auxiliary = num_elements;
                    num_elements += 1;
                    triples.push((*u, auxiliary, *v));
                    triples.push((auxiliary, pole, *w));
                }
                _ => unreachable!("normalization only produces size-2 or size-3 subsets"),
            }
        }

        ReductionSetSplittingToBetweenness {
            target: Betweenness::new(num_elements, triples),
            source_universe_size: self.universe_size(),
            pole,
        }
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
                    source_config: vec![1, 0, 1, 0, 0],
                    target_config: vec![8, 2, 9, 0, 1, 4, 3, 6, 7, 5],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/setsplitting_betweenness.rs"]
mod tests;
