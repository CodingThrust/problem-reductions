//! Reduction from BalancedCompleteBipartiteSubgraph to ILP.
//!
//! Binary variables x_l for left vertices, y_r for right vertices.
//! Cardinality: Σ x_l = k, Σ y_r = k.
//! Non-edge forbidding: x_l + y_r ≤ 1 for every non-edge (l, r).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::BalancedCompleteBipartiteSubgraph;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ReductionBCBSToILP {
    target: ILP<bool>,
    num_vertices: usize,
}

impl ReductionResult for ReductionBCBSToILP {
    type Source = BalancedCompleteBipartiteSubgraph;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_vertices].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices",
        num_constraints = "num_vertices * num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for BalancedCompleteBipartiteSubgraph {
    type Result = ReductionBCBSToILP;

    fn reduce_to(&self) -> Self::Result {
        let left = self.left_size();
        let right = self.right_size();
        let n = left + right;
        let k = self.k();
        let mut constraints = Vec::new();

        // Build edge lookup (bipartite-local coords)
        let edge_set: HashSet<(usize, usize)> = self.graph().left_edges().iter().copied().collect();

        // Σ x_l = k (for l in 0..left)
        let left_terms: Vec<(usize, f64)> = (0..left).map(|l| (l, 1.0)).collect();
        constraints.push(LinearConstraint::eq(left_terms, k as f64));

        // Σ y_r = k (for r in 0..right, variable index = left + r)
        let right_terms: Vec<(usize, f64)> = (0..right).map(|r| (left + r, 1.0)).collect();
        constraints.push(LinearConstraint::eq(right_terms, k as f64));

        // Non-edge constraints: x_l + y_r ≤ 1 for (l, r) not in E
        for l in 0..left {
            for r in 0..right {
                if !edge_set.contains(&(l, r)) {
                    constraints.push(LinearConstraint::le(vec![(l, 1.0), (left + r, 1.0)], 1.0));
                }
            }
        }

        let target = ILP::new(n, constraints, vec![], ObjectiveSense::Minimize);
        ReductionBCBSToILP {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::BipartiteGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "balancedcompletebipartitesubgraph_to_ilp",
        build: || {
            let source = BalancedCompleteBipartiteSubgraph::new(
                BipartiteGraph::new(3, 3, vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 1), (2, 2)]),
                2,
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 0, 1, 1, 0],
                    target_config: vec![1, 1, 0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/balancedcompletebipartitesubgraph_ilp.rs"]
mod tests;
