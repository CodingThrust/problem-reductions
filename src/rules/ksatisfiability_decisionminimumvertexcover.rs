//! Reduction from KSatisfiability (3-SAT) to Decision Minimum Vertex Cover.
//!
//! This wraps the classical Garey & Johnson Theorem 3.3 construction in the
//! `Decision<MinimumVertexCover<SimpleGraph, i32>>` wrapper, with threshold
//! `k = n + 2m` for `n` variables and `m` clauses.

use crate::models::decision::Decision;
use crate::models::formula::KSatisfiability;
use crate::models::graph::MinimumVertexCover;
use crate::reduction;
use crate::rules::ksatisfiability_minimumvertexcover::Reduction3SATToMVC;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::variant::K3;

/// Result of reducing KSatisfiability<K3> to Decision<MinimumVertexCover<SimpleGraph, i32>>.
#[derive(Debug, Clone)]
pub struct Reduction3SATToDecisionMVC {
    target: Decision<MinimumVertexCover<SimpleGraph, i32>>,
    base_reduction: Reduction3SATToMVC,
}

impl ReductionResult for Reduction3SATToDecisionMVC {
    type Source = KSatisfiability<K3>;
    type Target = Decision<MinimumVertexCover<SimpleGraph, i32>>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.base_reduction.extract_solution(target_solution)
    }
}

#[reduction(
    overhead = {
        num_vertices = "2 * num_vars + 3 * num_clauses",
        num_edges = "num_vars + 6 * num_clauses",
        k = "num_vars + 2 * num_clauses",
    }
)]
impl ReduceTo<Decision<MinimumVertexCover<SimpleGraph, i32>>> for KSatisfiability<K3> {
    type Result = Reduction3SATToDecisionMVC;

    fn reduce_to(&self) -> Self::Result {
        let base_reduction = <KSatisfiability<K3> as ReduceTo<
            MinimumVertexCover<SimpleGraph, i32>,
        >>::reduce_to(self);
        let bound = i32::try_from(self.num_vars() + 2 * self.num_clauses())
            .expect("decision minimum vertex cover bound must fit in i32");
        let target = Decision::new(base_reduction.target_problem().clone(), bound);

        Reduction3SATToDecisionMVC {
            target,
            base_reduction,
        }
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
                Decision<MinimumVertexCover<SimpleGraph, i32>>,
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
