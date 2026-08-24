//! Reduction from Decision Minimum Dominating Set to Minimum Sum Multicenter.
//!
//! On unit-weight, unit-length graphs, choosing `K` centers with total distance
//! `n - K` is exactly choosing a dominating set of size at most `K`.

use crate::models::decision::Decision;
use crate::models::graph::{MinimumDominatingSet, MinimumSumMulticenter};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of reducing DecisionMinimumDominatingSet to MinimumSumMulticenter.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter {
    target: MinimumSumMulticenter<SimpleGraph, i64>,
}

impl ReductionResult for ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter {
    type Source = Decision<MinimumDominatingSet<SimpleGraph, One>>;
    type Target = MinimumSumMulticenter<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.to_vec())
    }
}

#[reduction(size = upper_bound { num_vertices = "num_vertices", num_edges = "num_edges" })]
impl ReduceTo<MinimumSumMulticenter<SimpleGraph, i64>>
    for Decision<MinimumDominatingSet<SimpleGraph, One>>
{
    type Result = ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let source_graph = self.inner().graph();
        let target = MinimumSumMulticenter::new(
            SimpleGraph::new(source_graph.num_vertices(), source_graph.edges()),
            vec![1i64; source_graph.num_vertices()],
            vec![1i64; source_graph.num_edges()],
            self.k(),
        );
        Ok(ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decisionminimumdominatingset_to_minimumsummulticenter",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MinimumSumMulticenter<SimpleGraph, i64>,
            >(
                Decision::new(
                    MinimumDominatingSet::new(
                        SimpleGraph::new(
                            6,
                            vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (3, 5), (4, 5)],
                        ),
                        vec![One; 6],
                    ),
                    2,
                ),
                SolutionPair {
                    source_config: vec![1, 0, 0, 1, 0, 0],
                    target_config: vec![1, 0, 0, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/decisionminimumdominatingset_minimumsummulticenter.rs"]
mod tests;
