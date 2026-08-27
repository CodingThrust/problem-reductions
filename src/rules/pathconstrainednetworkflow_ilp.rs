//! Reduction from PathConstrainedNetworkFlow to ILP.
//!
//! One integer variable per prescribed path. Arc capacity aggregation
//! across paths and total flow requirement.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::PathConstrainedNetworkFlow;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing PathConstrainedNetworkFlow to ILP.
#[derive(Debug, Clone)]
pub struct ReductionPCNFToILP {
    target: ILP<i64>,
}

impl ReductionResult for ReductionPCNFToILP {
    type Source = PathConstrainedNetworkFlow;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::decode_usize_values(target_solution)
    }
}

#[reduction(
    size = exact {
        num_vars = "num_paths",
        num_constraints = "num_arcs + 1",
    },
)]
impl ReduceTo<ILP<i64>> for PathConstrainedNetworkFlow {
    type Result = ReductionPCNFToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_paths = self.num_paths();
        let num_arcs = self.num_arcs();
        let mut constraints = Vec::new();

        // Arc capacity: sum_{i : a in P_i} f_i <= c_a for all a
        for arc_idx in 0..num_arcs {
            let terms: Vec<(usize, i64)> = self
                .paths()
                .iter()
                .enumerate()
                .filter(|(_, path)| path.contains(&arc_idx))
                .map(|(path_idx, _)| (path_idx, 1))
                .collect();
            if !terms.is_empty() {
                constraints.push(LinearConstraint::le(terms, self.capacities()[arc_idx]));
            }
        }

        // Total flow requirement: sum_i f_i >= R
        let total_terms: Vec<(usize, i64)> = (0..num_paths).map(|i| (i, 1)).collect();
        constraints.push(LinearConstraint::ge(total_terms, self.requirement()));

        Ok(ReductionPCNFToILP {
            target: ILP::new(num_paths, constraints, vec![], ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "pathconstrainednetworkflow_to_ilp",
        build: || {
            // Simple graph: s=0, t=2, arcs 0->1->2 and 0->2
            // Two paths: [0,1] (0->1->2) and [2] (0->2)
            let source = PathConstrainedNetworkFlow::new(
                DirectedGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
                vec![1, 1, 1],
                0,
                2,
                vec![vec![0, 1], vec![2]],
                2,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/pathconstrainednetworkflow_ilp.rs"]
mod tests;
