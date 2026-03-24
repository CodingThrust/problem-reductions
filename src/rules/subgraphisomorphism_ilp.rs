//! Reduction from SubgraphIsomorphism to ILP (Integer Linear Programming).
//!
//! Injective assignment with non-edge constraints:
//! - Binary x_{v,u}: pattern vertex v maps to host vertex u
//! - Assignment: each pattern vertex to exactly one host vertex
//! - Injectivity: each host vertex receives at most one pattern vertex
//! - Non-edge forbiddance: for each pattern edge {v,w} and each host non-edge {u,u'},
//!   x_{v,u} + x_{w,u'} <= 1 AND x_{v,u'} + x_{w,u} <= 1

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::SubgraphIsomorphism;
use crate::reduction;
use crate::rules::ilp_helpers::one_hot_assignment_constraints;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::Graph;

/// Result of reducing SubgraphIsomorphism to ILP.
///
/// Variable layout (all binary):
/// - `x_{v,u}` at index `v * n_host + u` for pattern vertex v, host vertex u
#[derive(Debug, Clone)]
pub struct ReductionSubIsoToILP {
    target: ILP<bool>,
    num_pattern_vertices: usize,
    num_host_vertices: usize,
}

impl ReductionResult for ReductionSubIsoToILP {
    type Source = SubgraphIsomorphism;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract: for each pattern vertex v, output the unique host vertex u with x_{v,u} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n_host = self.num_host_vertices;
        (0..self.num_pattern_vertices)
            .map(|v| {
                (0..n_host)
                    .find(|&u| target_solution[v * n_host + u] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_pattern_vertices * num_host_vertices",
        num_constraints = "num_pattern_vertices + num_host_vertices + num_pattern_edges * num_host_vertices^2",
    }
)]
impl ReduceTo<ILP<bool>> for SubgraphIsomorphism {
    type Result = ReductionSubIsoToILP;

    fn reduce_to(&self) -> Self::Result {
        let n_pat = self.num_pattern_vertices();
        let n_host = self.num_host_vertices();
        let host = self.host_graph();
        let pattern = self.pattern_graph();
        let pat_edges = pattern.edges();

        let num_vars = n_pat * n_host;

        let mut constraints = Vec::new();

        // Assignment constraints
        constraints.extend(one_hot_assignment_constraints(n_pat, n_host, 0));

        // Non-edge forbiddance: for each pattern edge {v,w} and each host non-edge {u,u'}
        for &(v, w) in &pat_edges {
            for u in 0..n_host {
                for u_prime in 0..n_host {
                    if u == u_prime {
                        continue;
                    }
                    if host.has_edge(u, u_prime) {
                        continue;
                    }
                    // x_{v,u} + x_{w,u'} <= 1
                    constraints.push(LinearConstraint::le(
                        vec![(v * n_host + u, 1.0), (w * n_host + u_prime, 1.0)],
                        1.0,
                    ));
                }
            }
        }

        // Feasibility: no objective
        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        ReductionSubIsoToILP {
            target,
            num_pattern_vertices: n_pat,
            num_host_vertices: n_host,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::SimpleGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subgraphisomorphism_to_ilp",
        build: || {
            // Host: C4, Pattern: P3 (path on 3 vertices embeddable in cycle)
            let host = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
            let pattern = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
            let source = SubgraphIsomorphism::new(host, pattern);
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let ilp_solution = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&ilp_solution);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config,
                    target_config: ilp_solution,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subgraphisomorphism_ilp.rs"]
mod tests;
