//! Reduction from MinimumCoveringByCliques to MinimumIntersectionGraphBasis.
//!
//! The instance mapping is the identity on the underlying graph. Witness
//! extraction converts an intersection representation back into an edge-clique
//! cover by labeling each edge with any shared universe element.

use crate::models::graph::{MinimumCoveringByCliques, MinimumIntersectionGraphBasis};
use crate::reduction;
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::topology::{Graph, SimpleGraph};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ReductionMinimumCoveringByCliquesToMinimumIntersectionGraphBasis {
    target: MinimumIntersectionGraphBasis<SimpleGraph>,
}

fn extract_edge_clique_cover(graph: &SimpleGraph, target_solution: &[Vec<bool>]) -> Vec<usize> {
    let m = graph.num_edges();
    let mut label_map = BTreeMap::new();
    let mut source_solution = Vec::with_capacity(m);
    for (u, v) in graph.edges() {
        for shared_label in (0..m)
            .filter(|&slot| target_solution[u][slot] && target_solution[v][slot])
            .take(1)
        {
            let next_label = label_map.len();
            source_solution.push(*label_map.entry(shared_label).or_insert(next_label));
        }
    }
    source_solution
}

#[cfg(any(test, feature = "example-db"))]
fn intersection_basis_config(graph: &SimpleGraph, subsets: &[&[usize]]) -> Vec<Vec<bool>> {
    let n = graph.num_vertices();
    let m = graph.num_edges();

    assert_eq!(subsets.len(), n, "one subset per vertex");

    if m == 0 {
        assert!(
            subsets.iter().all(|subset| subset.is_empty()),
            "empty graphs have empty subsets in canonical configs"
        );
        return vec![Vec::new(); n];
    }

    let mut config = vec![vec![false; m]; n];
    for (vertex, subset) in subsets.iter().enumerate() {
        for &slot in *subset {
            assert!(slot < m, "intersection-basis slot out of range");
            config[vertex][slot] = true;
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

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| self.map_solution(solution))
    }
}

impl ReductionMinimumCoveringByCliquesToMinimumIntersectionGraphBasis {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        Ok(extract_edge_clique_cover(
            self.target.graph(),
            target_solution,
        ))
    }
}

#[reduction(
    transform = exact {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<MinimumIntersectionGraphBasis<SimpleGraph>>
    for MinimumCoveringByCliques<SimpleGraph>
{
    type Result = ReductionMinimumCoveringByCliquesToMinimumIntersectionGraphBasis;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(Self::Result {
            target: MinimumIntersectionGraphBasis::new(self.graph().clone()),
        })
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
                    source_config: serde_json::json!(vec![0, 0, 0, 1]),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumcoveringbycliques_minimumintersectiongraphbasis.rs"]
mod tests;
