//! Reduction from unweighted MaxCut to Maximum 2-Satisfiability.

use crate::models::formula::{CNFClause, Maximum2Satisfiability};
use crate::models::graph::MaxCut;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of reducing unweighted MaxCut to Maximum2Satisfiability.
#[derive(Debug, Clone)]
pub struct ReductionMaxCutToMaximum2Satisfiability {
    target: Maximum2Satisfiability,
}

impl ReductionResult for ReductionMaxCutToMaximum2Satisfiability {
    type Source = MaxCut<SimpleGraph, One>;
    type Target = Maximum2Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices",
        num_clauses = "2 * num_edges",
    }
)]
impl ReduceTo<Maximum2Satisfiability> for MaxCut<SimpleGraph, One> {
    type Result = ReductionMaxCutToMaximum2Satisfiability;

    fn reduce_to(&self) -> Self::Result {
        let clauses = self
            .graph()
            .edges()
            .into_iter()
            .flat_map(|(u, v)| {
                let u = (u + 1) as i32;
                let v = (v + 1) as i32;
                [CNFClause::new(vec![u, v]), CNFClause::new(vec![-u, -v])]
            })
            .collect();

        ReductionMaxCutToMaximum2Satisfiability {
            target: Maximum2Satisfiability::new(self.num_vertices(), clauses),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maxcut_to_maximum2satisfiability",
        build: || {
            let source = MaxCut::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2)]),
                vec![One; 5],
            );
            crate::example_db::specs::rule_example_with_witness::<_, Maximum2Satisfiability>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 0, 1],
                    target_config: vec![0, 1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maxcut_maximum2satisfiability.rs"]
mod tests;
