//! Reduction from GraphPartitioning to ILP (Integer Linear Programming).
//!
//! Uses the standard balanced-cut ILP formulation:
//! - Variables: `x_v` for vertex-side assignment and `y_e` for edge-crossing indicators
//! - Constraints: one balance equality plus two linking inequalities per edge
//! - Objective: minimize the number of crossing edges

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::GraphPartitioning;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing GraphPartitioning to ILP.
///
/// Variable layout (all binary):
/// - `x_v` for `v = 0..n-1`: vertex `v` belongs to side `B`
/// - `y_e` for `e = 0..m-1`: edge `e` crosses the partition
#[derive(Debug, Clone)]
pub struct ReductionGraphPartitioningToILP {
    target: ILP<bool>,
    num_vertices: usize,
}

impl ReductionResult for ReductionGraphPartitioningToILP {
    type Source = GraphPartitioning<SimpleGraph>;
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
        num_vars = "num_vertices + num_edges",
        num_constraints = "2 * num_edges + 1",
    }
)]
impl ReduceTo<ILP<bool>> for GraphPartitioning<SimpleGraph> {
    type Result = ReductionGraphPartitioningToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.graph().edges();
        let m = edges.len();
        let num_vars = n + m;

        let mut constraints = Vec::with_capacity(2 * m + 1);

        let balance_terms: Vec<(usize, f64)> = (0..n).map(|v| (v, 1.0)).collect();
        constraints.push(LinearConstraint::eq(balance_terms, n as f64 / 2.0));

        for (edge_idx, (u, v)) in edges.iter().enumerate() {
            let y_var = n + edge_idx;
            constraints.push(LinearConstraint::ge(
                vec![(y_var, 1.0), (*u, -1.0), (*v, 1.0)],
                0.0,
            ));
            constraints.push(LinearConstraint::ge(
                vec![(y_var, 1.0), (*u, 1.0), (*v, -1.0)],
                0.0,
            ));
        }

        let objective: Vec<(usize, f64)> = (0..m).map(|edge_idx| (n + edge_idx, 1.0)).collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionGraphPartitioningToILP {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "graphpartitioning_to_ilp",
        build: || {
            let source = GraphPartitioning::new(SimpleGraph::new(
                6,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 2),
                    (1, 3),
                    (2, 3),
                    (2, 4),
                    (3, 4),
                    (3, 5),
                    (4, 5),
                ],
            ));
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 0, 1, 1, 1],
                    target_config: vec![0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/graphpartitioning_ilp.rs"]
mod tests;
