//! Reduction from KSatisfiability (3-SAT) to Decision Minimum Vertex Cover.
//!
//! This wraps the classical Garey & Johnson Theorem 3.3 construction in the
//! `Decision<MinimumVertexCover<SimpleGraph, i64>>` wrapper, with threshold
//! `k = n + 2m` for `n` variables and `m` clauses.

use crate::models::decision::Decision;
use crate::models::formula::KSatisfiability;
use crate::models::graph::MinimumVertexCover;
use crate::reduction;
use crate::rules::ksatisfiability_minimumvertexcover::Reduction3SATToMVC;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::variant::K3;

/// Result of reducing KSatisfiability<K3> to Decision<MinimumVertexCover<SimpleGraph, i64>>.
#[derive(Debug, Clone)]
pub struct Reduction3SATToDecisionMVC {
    target: Decision<MinimumVertexCover<SimpleGraph, i64>>,
    base_reduction: Reduction3SATToMVC,
}

impl ReductionResult for Reduction3SATToDecisionMVC {
    type Source = KSatisfiability<K3>;
    type Target = Decision<MinimumVertexCover<SimpleGraph, i64>>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        self.base_reduction.extract_solution(target_solution)
    }
}

#[reduction(
    size = exact {
        num_vertices = "2 * num_vars + 3 * num_clauses",
        num_edges = "num_vars + 6 * num_clauses",
        k = "num_vars + 2 * num_clauses",
    }
)]
impl ReduceTo<Decision<MinimumVertexCover<SimpleGraph, i64>>> for KSatisfiability<K3> {
    type Result = Reduction3SATToDecisionMVC;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let base_reduction = <KSatisfiability<K3> as ReduceTo<
            MinimumVertexCover<SimpleGraph, i64>,
        >>::reduce_to(self)?;
        let bound = self
            .num_clauses()
            .checked_mul(2)
            .and_then(|value| value.checked_add(self.num_vars()))
            .and_then(|value| i64::try_from(value).ok())
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    KSatisfiability<K3>,
                    Decision<MinimumVertexCover<SimpleGraph, i64>>,
                >("computing the target cover bound")
            })?;
        let target = Decision::new(base_reduction.target_problem().clone(), bound);

        Ok(Reduction3SATToDecisionMVC {
            target,
            base_reduction,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_decisionminimumvertexcover",
        build: || {
            let source = KSatisfiability::<K3>::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 3]),
                    CNFClause::new(vec![-1, -2, 3]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<
                _,
                Decision<MinimumVertexCover<SimpleGraph, i64>>,
            >(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 1],
                    target_config: vec![0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_decisionminimumvertexcover.rs"]
mod tests;
