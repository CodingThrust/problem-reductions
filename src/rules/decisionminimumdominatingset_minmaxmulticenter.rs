//! Reduction from Decision Minimum Dominating Set to Min-Max Multicenter.
//!
//! On unit-weight, unit-length graphs, choosing `K` centers with maximum
//! distance at most `1` is exactly choosing a dominating set of size `K`.

use crate::models::decision::Decision;
use crate::models::graph::{MinMaxMulticenter, MinimumDominatingSet};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of reducing DecisionMinimumDominatingSet to MinMaxMulticenter.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMinimumDominatingSetToMinMaxMulticenter {
    target: MinMaxMulticenter<SimpleGraph, One>,
}

impl ReductionResult for ReductionDecisionMinimumDominatingSetToMinMaxMulticenter {
    type Source = Decision<MinimumDominatingSet<SimpleGraph, One>>;
    type Target = MinMaxMulticenter<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<MinMaxMulticenter<SimpleGraph, One>>
    for Decision<MinimumDominatingSet<SimpleGraph, One>>
{
    type Result = ReductionDecisionMinimumDominatingSetToMinMaxMulticenter;

    fn reduce_to(&self) -> Self::Result {
        let source_graph = self.inner().graph();
        let target = MinMaxMulticenter::new(
            SimpleGraph::new(source_graph.num_vertices(), source_graph.edges()),
            vec![One; source_graph.num_vertices()],
            vec![One; source_graph.num_edges()],
            self.k(),
        );
        ReductionDecisionMinimumDominatingSetToMinMaxMulticenter { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decisionminimumdominatingset_to_minmaxmulticenter",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MinMaxMulticenter<SimpleGraph, One>,
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
#[path = "../unit_tests/rules/decisionminimumdominatingset_minmaxmulticenter.rs"]
mod tests;
