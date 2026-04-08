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
    target: MinimumSumMulticenter<SimpleGraph, i32>,
}

impl ReductionResult for ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter {
    type Source = Decision<MinimumDominatingSet<SimpleGraph, One>>;
    type Target = MinimumSumMulticenter<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = { num_vertices = "num_vertices" })]
impl ReduceTo<MinimumSumMulticenter<SimpleGraph, i32>>
    for Decision<MinimumDominatingSet<SimpleGraph, One>>
{
    type Result = ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter;

    fn reduce_to(&self) -> Self::Result {
        let source_graph = self.inner().graph();
        let target = MinimumSumMulticenter::new(
            SimpleGraph::new(source_graph.num_vertices(), source_graph.edges()),
            vec![1i32; source_graph.num_vertices()],
            vec![1i32; source_graph.num_edges()],
            self.k(),
        );
        ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter { target }
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
                MinimumSumMulticenter<SimpleGraph, i32>,
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
