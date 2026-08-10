//! Reduction from MinimumCoveringByCliques to MinimumIntersectionGraphBasis.
//!
//! The instance mapping is the identity on the underlying graph. Witness
//! extraction converts an intersection representation back into an edge-clique
//! cover by labeling each edge with any shared universe element.

use crate::models::graph::{MinimumCoveringByCliques, MinimumIntersectionGraphBasis};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ReductionMinimumCoveringByCliquesToMinimumIntersectionGraphBasis {
    target: MinimumIntersectionGraphBasis<SimpleGraph>,
}

fn extract_edge_clique_cover(graph: &SimpleGraph, target_solution: &[usize]) -> Option<Vec<usize>> {
    let n = graph.num_vertices();
    let m = graph.num_edges();

    if m == 0 {
        return target_solution.is_empty().then(Vec::new);
    }

    if target_solution.len() != n * m {
        return None;
    }

    let mut label_map = BTreeMap::new();
    let mut next_label = 0usize;
    let mut source_solution = Vec::with_capacity(m);

    for (u, v) in graph.edges() {
        let shared_label = (0..m).find(|&slot| {
            target_solution[u * m + slot] == 1 && target_solution[v * m + slot] == 1
        })?;
        let compressed = *label_map.entry(shared_label).or_insert_with(|| {
            let label = next_label;
            next_label += 1;
            label
        });
        source_solution.push(compressed);
    }

    Some(source_solution)
}

#[cfg(any(test, feature = "example-db"))]
fn intersection_basis_config(graph: &SimpleGraph, subsets: &[&[usize]]) -> Vec<usize> {
    let n = graph.num_vertices();
    let m = graph.num_edges();

    assert_eq!(subsets.len(), n, "one subset per vertex");

    if m == 0 {
        assert!(
            subsets.iter().all(|subset| subset.is_empty()),
            "empty graphs have empty subsets in canonical configs"
        );
        return Vec::new();
    }

    let mut config = vec![0; n * m];
    for (vertex, subset) in subsets.iter().enumerate() {
        for &slot in *subset {
            assert!(slot < m, "intersection-basis slot out of range");
            config[vertex * m + slot] = 1;
        }
    }
    config
}

impl ReductionResult for ReductionMinimumCoveringByCliquesToMinimumIntersectionGraphBasis {
    type Source = MinimumCoveringByCliques<SimpleGraph>;
    type Target = MinimumIntersectionGraphBasis<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            if !self.target.evaluate(target_solution).is_valid() {
                return Err(crate::rules::ExtractionError::invalid(
                    "target configuration is not a valid intersection graph basis",
                ));
            }

            extract_edge_clique_cover(self.target.graph(), target_solution).ok_or_else(|| {
                crate::rules::ExtractionError::invalid(
                    "target basis does not assign a shared label to every source edge",
                )
            })?
        })
    }
}

#[reduction(
    exact = {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<MinimumIntersectionGraphBasis<SimpleGraph>>
    for MinimumCoveringByCliques<SimpleGraph>
{
    type Result = ReductionMinimumCoveringByCliquesToMinimumIntersectionGraphBasis;

    fn reduce_to(&self) -> Self::Result {
        Self::Result {
            target: MinimumIntersectionGraphBasis::new(self.graph().clone()),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumcoveringbycliques_to_minimumintersectiongraphbasis",
        build: || {
            let source = MinimumCoveringByCliques::new(SimpleGraph::new(
                4,
                vec![(0, 1), (0, 2), (1, 2), (2, 3)],
            ));
            let target_config =
                intersection_basis_config(source.graph(), &[&[0], &[0], &[0, 1], &[1]]);

            crate::example_db::specs::rule_example_with_witness::<
                _,
                MinimumIntersectionGraphBasis<SimpleGraph>,
            >(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 0, 1],
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumcoveringbycliques_minimumintersectiongraphbasis.rs"]
mod tests;
