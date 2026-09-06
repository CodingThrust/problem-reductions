//! Unit decision dominating set to min-max multicenter.
//!
//! Add two isolated vertices and use clamp(K,-1,n)+2 centers. Every finite
//! placement selects both isolates; radius <= 1 is then exactly the source
//! dominating-set threshold. This includes empty graphs and every signed K.

use crate::models::decision::Decision;
use crate::models::graph::{MinMaxMulticenter, MinimumDominatingSet};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::{Min, One, Or};

/// The source vertices precede the two mandatory auxiliary centers.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMinimumDominatingSetToMinMaxMulticenter {
    target: MinMaxMulticenter<SimpleGraph, One>,
    source_num_vertices: usize,
}

impl ReductionResult for ReductionDecisionMinimumDominatingSetToMinMaxMulticenter {
    type Source = Decision<MinimumDominatingSet<SimpleGraph, One>>;
    type Target = MinMaxMulticenter<SimpleGraph, One>;

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
                "target placement does not certify a dominating set: radius must be at most one",
            ));
        }
        Ok(target_solution[..self.source_num_vertices].to_vec())
    }
}

impl crate::rules::AggregateReductionResult
    for ReductionDecisionMinimumDominatingSetToMinMaxMulticenter
{
    type Source = Decision<MinimumDominatingSet<SimpleGraph, One>>;
    type Target = MinMaxMulticenter<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: Min<i64>) -> Or {
        Or(target_value.0.is_some_and(|radius| radius <= 1))
    }
}

#[reduction(
    aggregate = custom,
    transform = exact {
        num_vertices = "num_vertices + 2",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<MinMaxMulticenter<SimpleGraph, One>>
    for Decision<MinimumDominatingSet<SimpleGraph, One>>
{
    type Result = ReductionDecisionMinimumDominatingSetToMinMaxMulticenter;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let source_graph = self.inner().graph();
        let n = source_graph.num_vertices();
        let (target_n, centers) = multicenter_parameters(n, *self.bound())?;
        let target = MinMaxMulticenter::new(
            SimpleGraph::new(target_n, source_graph.edges()),
            vec![One; target_n],
            vec![One; source_graph.num_edges()],
            centers,
        );
        Ok(ReductionDecisionMinimumDominatingSetToMinMaxMulticenter {
            target,
            source_num_vertices: n,
        })
    }
}

/// Check parameter arithmetic before allocating either graph or weight vectors.
fn multicenter_parameters(
    n: usize,
    bound: i64,
) -> Result<(usize, usize), crate::rules::ReductionError> {
    type Source = Decision<MinimumDominatingSet<SimpleGraph, One>>;
    type Target = MinMaxMulticenter<SimpleGraph, One>;
    let overflow = || {
        crate::rules::ReductionError::integer_overflow::<Source, Target>(
            "encoding min-max multicenter parameters",
        )
    };
    let target_n = n.checked_add(2).ok_or_else(overflow)?;
    let n_i64 = i64::try_from(n).map_err(|_| overflow())?;
    // Subset sizes lie in [0,n], so all lower bounds share the NO case -1.
    let normalized = bound.clamp(-1, n_i64);
    let centers = normalized.checked_add(2).ok_or_else(overflow)?;
    let centers = usize::try_from(centers).map_err(|_| overflow())?;
    Ok((target_n, centers))
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
                    source_config: serde_json::json!(vec![true, false, false, true, false, false]),
                    target_config: serde_json::json!(vec![
                        true, false, false, true, false, false, true, true
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/decisionminimumdominatingset_minmaxmulticenter.rs"]
mod tests;
