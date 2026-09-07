//! Reduction from DisjointConnectingPaths to ILP.
//!
//! Binary flow variables `f^k_{e,dir}` per commodity per directed arc orientation.
//! Flow conservation and unit vertex capacities enforce vertex-disjoint paths.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::DisjointConnectingPaths;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use std::collections::VecDeque;

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
    num_vertices: usize,
    terminal_pairs: Vec<(usize, usize)>,
    num_edge_vars_per_commodity: usize,
}

impl ReductionResult for ReductionDCPToILP {
    type Source = DisjointConnectingPaths<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let mut result = vec![false; self.edges.len()];
        for (k, &(source, sink)) in self.terminal_pairs.iter().enumerate() {
            let offset = k * self.num_edge_vars_per_commodity;
            let mut adjacency = vec![Vec::new(); self.num_vertices];
            for (edge, &(u, v)) in self.edges.iter().enumerate() {
                if target_solution[offset + 2 * edge] == 1 {
                    adjacency[u].push((v, edge));
                }
                if target_solution[offset + 2 * edge + 1] == 1 {
                    adjacency[v].push((u, edge));
                }
            }
            let mut visited = vec![false; self.num_vertices];
            let mut predecessor = vec![None; self.num_vertices];
            let mut queue = VecDeque::from([source]);
            visited[source] = true;
            while let Some(u) = queue.pop_front() {
                if u == sink {
                    break;
                }
                for &(v, edge) in &adjacency[u] {
                    if !visited[v] {
                        visited[v] = true;
                        predecessor[v] = Some((u, edge));
                        queue.push_back(v);
                    }
                }
            }
            let mut vertex = sink;
            while vertex != source {
                let (previous, edge) = predecessor[vertex].ok_or_else(|| {
                    crate::rules::ExtractionError::invalid(
                        "commodity flow does not connect its terminal pair",
                    )
                })?;
                result[edge] = true;
                vertex = previous;
            }
        }
        Ok(result)
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_pairs * 2 * num_edges",
        num_constraints = "num_pairs * num_vertices + num_vertices",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for DisjointConnectingPaths<SimpleGraph> {
    type Result = ReductionDCPToILP;

    #[allow(clippy::needless_range_loop)]
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let edges = self.ordered_edges();
        let m = edges.len();
        let n = self.num_vertices();
        let k_count = self.num_pairs();

        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<bool>>(operation)
        };
        let num_flow_vars_per_k = m
            .checked_mul(2)
            .ok_or_else(|| overflow("computing the flow-variable stride"))?;
        let num_vars = k_count
            .checked_mul(num_flow_vars_per_k)
            .ok_or_else(|| overflow("computing the flow-variable count"))?;

        let flow_var =
            |k: usize, e: usize, dir: usize| -> usize { k * num_flow_vars_per_k + 2 * e + dir };

        let mut constraints = Vec::new();

        // Build adjacency index: for each vertex, which edges are incident
        let mut vertex_edges: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (e, &(u, v)) in edges.iter().enumerate() {
            vertex_edges[u].push(e);
            vertex_edges[v].push(e);
        }

        let mut is_source = vec![false; n];
        for &(source, _) in self.terminal_pairs() {
            is_source[source] = true;
        }

        for (k, &(s_k, t_k)) in self.terminal_pairs().iter().enumerate() {
            // Flow conservation: outflow - inflow = demand at each vertex
            for vertex in 0..n {
                let mut terms = Vec::new();
                for &e in &vertex_edges[vertex] {
                    let (eu, _ev) = edges[e];
                    if vertex == eu {
                        // vertex is first endpoint: dir=0 is outgoing, dir=1 is incoming
                        terms.push((flow_var(k, e, 0), 1));
                        terms.push((flow_var(k, e, 1), -1));
                    } else {
                        // vertex is second endpoint: dir=1 is outgoing, dir=0 is incoming
                        terms.push((flow_var(k, e, 1), 1));
                        terms.push((flow_var(k, e, 0), -1));
                    }
                }

                let demand = if vertex == s_k {
                    1
                } else if vertex == t_k {
                    -1
                } else {
                    0
                };
                constraints.push(LinearConstraint::eq(terms, demand));
            }
        }

        // Incoming flow records vertex use. A source is occupied by its own
        // commodity even though it has no incoming flow.
        for v in 0..n {
            let mut terms = Vec::new();
            for k in 0..k_count {
                for &e in &vertex_edges[v] {
                    let (eu, _ev) = edges[e];
                    if v == eu {
                        terms.push((flow_var(k, e, 1), 1));
                    } else {
                        terms.push((flow_var(k, e, 0), 1));
                    }
                }
            }
            constraints.push(LinearConstraint::le(terms, 1 - i64::from(is_source[v])));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionDCPToILP {
            target,
            edges,
            num_vertices: n,
            terminal_pairs: self.terminal_pairs().to_vec(),
            num_edge_vars_per_commodity: num_flow_vars_per_k,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "disjointconnectingpaths_to_ilp",
        build: || {
            // 6 vertices, two vertex-disjoint paths
            let source = DisjointConnectingPaths::new(
                SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]),
                vec![(0, 2), (3, 5)],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/disjointconnectingpaths_ilp.rs"]
mod tests;
