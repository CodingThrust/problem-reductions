//! Reduction from Decision Minimum Dominating Set to Minimum Sum Multicenter.
//!
//! For K >= 0, add an isolated vertex and choose min(K, n) + 1 centers.
//! Finite cost forces the isolated vertex to be a center. The optimum equals
//! n - min(K, n) iff the original graph has a dominating set of size <= K.
//! For K < 0, two added isolated vertices and one center force infeasibility.

use crate::models::decision::Decision;
use crate::models::graph::{MinimumDominatingSet, MinimumSumMulticenter};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::{Min, One, Or};

/// Result of reducing DecisionMinimumDominatingSet to MinimumSumMulticenter.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter {
    target: MinimumSumMulticenter<SimpleGraph, i64>,
    source_num_vertices: usize,
    threshold: i64,
}

impl ReductionResult for ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter {
    type Source = Decision<MinimumDominatingSet<SimpleGraph, One>>;
    type Target = MinimumSumMulticenter<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !crate::rules::AggregateReductionResult::extract_value(self, value).0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target placement does not certify a dominating set within the source bound",
            ));
        }
        // Original vertices precede the auxiliary isolated vertices.
        Ok(target_solution[..self.source_num_vertices].to_vec())
    }
}

impl crate::rules::AggregateReductionResult
    for ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter
{
    type Source = Decision<MinimumDominatingSet<SimpleGraph, One>>;
    type Target = MinimumSumMulticenter<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: Min<i64>) -> Or {
        Or(target_value.0 == Some(self.threshold))
    }
}

#[reduction(
    aggregate = custom,
    transform = upper_bound { num_vertices = "num_vertices + 2", num_edges = "num_edges" }
)]
impl ReduceTo<MinimumSumMulticenter<SimpleGraph, i64>>
    for Decision<MinimumDominatingSet<SimpleGraph, One>>
{
    type Result = ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let source_graph = self.inner().graph();
        let n = source_graph.num_vertices();
        let (target_n, centers, threshold) = multicenter_parameters(n, *self.bound())?;
        let target = MinimumSumMulticenter::new(
            SimpleGraph::new(target_n, source_graph.edges()),
            vec![1i64; target_n],
            vec![1i64; source_graph.num_edges()],
            centers,
        );
        Ok(
            ReductionDecisionMinimumDominatingSetToMinimumSumMulticenter {
                target,
                source_num_vertices: n,
                threshold,
            },
        )
    }
}

/// Compute the construction's counts without allocating the graph, so numeric
/// domain boundaries can be checked independently of available memory.
fn multicenter_parameters(
    n: usize,
    bound: i64,
) -> Result<(usize, usize, i64), crate::rules::ReductionError> {
    type Source = Decision<MinimumDominatingSet<SimpleGraph, One>>;
    type Target = MinimumSumMulticenter<SimpleGraph, i64>;
    let overflow = || {
        crate::rules::ReductionError::integer_overflow::<Source, Target>(
            "encoding multicenter construction parameters",
        )
    };
    let extra_vertices = if bound < 0 { 2 } else { 1 };
    let target_n = n.checked_add(extra_vertices).ok_or_else(overflow)?;
    let n_i64 = i64::try_from(n).map_err(|_| overflow())?;
    if bound < 0 {
        // Two auxiliary isolates cannot both be served by one center.
        return Ok((target_n, 1, -1));
    }
    // No subset contains more than n vertices, so these bounds are equivalent.
    let q = bound.min(n_i64);
    let q_usize = usize::try_from(q).map_err(|_| overflow())?;
    // q <= n and n + 1 was checked above, so q + 1 cannot overflow.
    Ok((target_n, q_usize + 1, n_i64 - q))
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
                    source_config: serde_json::json!(vec![true, false, false, true, false, false]),
                    target_config: serde_json::json!(vec![
                        true, false, false, true, false, false, true
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/decisionminimumdominatingset_minimumsummulticenter.rs"]
mod tests;
