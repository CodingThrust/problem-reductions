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
    target: ILP<i32>,
}

impl ReductionResult for ReductionPCNFToILP {
    type Source = PathConstrainedNetworkFlow;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_paths",
        num_constraints = "num_arcs + 1",
    }
)]
impl ReduceTo<ILP<i32>> for PathConstrainedNetworkFlow {
    type Result = ReductionPCNFToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_paths = self.num_paths();
        let num_arcs = self.num_arcs();
        let mut constraints = Vec::new();

        // Arc capacity: sum_{i : a in P_i} f_i <= c_a for all a
        for arc_idx in 0..num_arcs {
            let terms: Vec<(usize, f64)> = self
                .paths()
                .iter()
                .enumerate()
                .filter(|(_, path)| path.contains(&arc_idx))
                .map(|(path_idx, _)| (path_idx, 1.0))
                .collect();
            if !terms.is_empty() {
                constraints.push(LinearConstraint::le(
                    terms,
                    self.capacities()[arc_idx] as f64,
                ));
            }
        }

        // Total flow requirement: sum_i f_i >= R
        let total_terms: Vec<(usize, f64)> = (0..num_paths).map(|i| (i, 1.0)).collect();
        constraints.push(LinearConstraint::ge(total_terms, self.requirement() as f64));

        ReductionPCNFToILP {
            target: ILP::new(num_paths, constraints, vec![], ObjectiveSense::Minimize),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
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
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                source,
                SolutionPair {
                    source_config: vec![1, 1],
                    target_config: vec![1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/pathconstrainednetworkflow_ilp.rs"]
mod tests;
