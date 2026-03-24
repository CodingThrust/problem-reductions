//! Reduction from DisjointConnectingPaths to ILP.
//!
//! Binary flow variables `f^k_{e,dir}` per commodity per directed arc orientation.
//! Flow conservation, anti-parallel constraints, and vertex disjointness.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::DisjointConnectingPaths;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing DisjointConnectingPaths to ILP.
///
/// Variable layout (all binary):
/// - `f^k_{e,dir}` for each commodity k and each directed orientation of each edge.
///   For edge index `e` with endpoints `(u,v)`, direction 0 is u->v and direction 1 is v->u.
///   Index: `k * 2m + 2e + dir` for k in 0..K, e in 0..m, dir in {0,1}.
///
/// Total: `K * 2m` variables.
#[derive(Debug, Clone)]
pub struct ReductionDCPToILP {
    target: ILP<bool>,
    /// Canonical edge list used during construction.
    edges: Vec<(usize, usize)>,
    num_commodities: usize,
    num_edge_vars_per_commodity: usize,
}

impl ReductionResult for ReductionDCPToILP {
    type Source = DisjointConnectingPaths<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Mark an edge selected iff some orientation carries flow for some commodity.
        let m = self.edges.len();
        let mut result = vec![0usize; m];
        for k in 0..self.num_commodities {
            for e in 0..m {
                let fwd = target_solution[k * self.num_edge_vars_per_commodity + 2 * e];
                let rev = target_solution[k * self.num_edge_vars_per_commodity + 2 * e + 1];
                if fwd == 1 || rev == 1 {
                    result[e] = 1;
                }
            }
        }
        result
    }
}

#[reduction(
    overhead = {
        num_vars = "num_pairs * 2 * num_edges",
        num_constraints = "num_pairs * num_vertices + num_pairs * num_edges + num_edges + num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for DisjointConnectingPaths<SimpleGraph> {
    type Result = ReductionDCPToILP;

    #[allow(clippy::needless_range_loop)]
    fn reduce_to(&self) -> Self::Result {
        let edges = self.ordered_edges();
        let m = edges.len();
        let n = self.num_vertices();
        let k_count = self.num_pairs();

        // Variable layout: only flow variables, no MTZ ordering needed for binary flow
        let num_flow_vars_per_k = 2 * m; // f^k_{e,dir}
        let num_vars = k_count * num_flow_vars_per_k;

        let flow_var =
            |k: usize, e: usize, dir: usize| -> usize { k * num_flow_vars_per_k + 2 * e + dir };

        let mut constraints = Vec::new();

        // Build adjacency index: for each vertex, which edges are incident
        let mut vertex_edges: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (e, &(u, v)) in edges.iter().enumerate() {
            vertex_edges[u].push(e);
            vertex_edges[v].push(e);
        }

        // Identify terminal vertices
        let mut is_terminal = vec![false; n];
        for &(s, t) in self.terminal_pairs() {
            is_terminal[s] = true;
            is_terminal[t] = true;
        }

        for (k, &(s_k, t_k)) in self.terminal_pairs().iter().enumerate() {
            // Flow conservation: outflow - inflow = demand at each vertex
            for vertex in 0..n {
                let mut terms = Vec::new();
                for &e in &vertex_edges[vertex] {
                    let (eu, _ev) = edges[e];
                    if vertex == eu {
                        // vertex is first endpoint: dir=0 is outgoing, dir=1 is incoming
                        terms.push((flow_var(k, e, 0), 1.0));
                        terms.push((flow_var(k, e, 1), -1.0));
                    } else {
                        // vertex is second endpoint: dir=1 is outgoing, dir=0 is incoming
                        terms.push((flow_var(k, e, 1), 1.0));
                        terms.push((flow_var(k, e, 0), -1.0));
                    }
                }

                let demand = if vertex == s_k {
                    1.0
                } else if vertex == t_k {
                    -1.0
                } else {
                    0.0
                };
                constraints.push(LinearConstraint::eq(terms, demand));
            }

            // Anti-parallel: f^k_{e,0} + f^k_{e,1} <= 1 for each edge
            for e in 0..m {
                constraints.push(LinearConstraint::le(
                    vec![(flow_var(k, e, 0), 1.0), (flow_var(k, e, 1), 1.0)],
                    1.0,
                ));
            }
        }

        // Edge disjointness: each edge is used by at most one commodity
        // sum_k (f^k_{e,0} + f^k_{e,1}) <= 1
        for e in 0..m {
            let mut terms = Vec::new();
            for k in 0..k_count {
                terms.push((flow_var(k, e, 0), 1.0));
                terms.push((flow_var(k, e, 1), 1.0));
            }
            constraints.push(LinearConstraint::le(terms, 1.0));
        }

        // Vertex disjointness: for each non-terminal vertex v,
        // sum over all commodities k of (outgoing flow from v) <= 1
        for v in 0..n {
            if is_terminal[v] {
                continue;
            }
            let mut terms = Vec::new();
            for k in 0..k_count {
                for &e in &vertex_edges[v] {
                    let (eu, _ev) = edges[e];
                    if v == eu {
                        terms.push((flow_var(k, e, 0), 1.0));
                    } else {
                        terms.push((flow_var(k, e, 1), 1.0));
                    }
                }
            }
            constraints.push(LinearConstraint::le(terms, 1.0));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        ReductionDCPToILP {
            target,
            edges,
            num_commodities: k_count,
            num_edge_vars_per_commodity: num_flow_vars_per_k,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::rules::ReduceTo as _;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "disjointconnectingpaths_to_ilp",
        build: || {
            // 6 vertices, two vertex-disjoint paths
            let source = DisjointConnectingPaths::new(
                SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]),
                vec![(0, 2), (3, 5)],
            );
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
#[path = "../unit_tests/rules/disjointconnectingpaths_ilp.rs"]
mod tests;
