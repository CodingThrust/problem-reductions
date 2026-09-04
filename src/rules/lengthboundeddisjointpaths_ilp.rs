//! Reduction from LengthBoundedDisjointPaths to ILP.
//!
//! Binary flow variables per commodity per directed edge orientation.
//! Conservation, edge/vertex disjointness, and length bound.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::LengthBoundedDisjointPaths;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use std::collections::VecDeque;

/// Result of reducing LengthBoundedDisjointPaths to ILP.
///
/// Variable layout (all binary):
/// - Flow: `f^k_{e,dir}` at index `k * 2m + 2e + dir`
/// - Activation: `a_k` at index `J * 2m + k`
///
/// Total: `J * (2m + 1)` variables.
#[derive(Debug, Clone)]
pub struct ReductionLBDPToILP {
    target: ILP<bool>,
    /// Edges in the source graph's order, with their original orientations.
    edges: Vec<(usize, usize)>,
    num_vertices: usize,
    num_paths: usize,
    source: usize,
    sink: usize,
}

impl ReductionResult for ReductionLBDPToILP {
    type Source = LengthBoundedDisjointPaths<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let m = self.edges.len();
        let flow_vars_per_k = 2 * m;
        let activation_offset = self.num_paths * flow_vars_per_k;
        let mut result = vec![vec![false; m]; self.num_paths];
        for (k, path) in result.iter_mut().enumerate() {
            if target_solution[activation_offset + k] == 0 {
                if target_solution[k * flow_vars_per_k..(k + 1) * flow_vars_per_k]
                    .iter()
                    .any(|&flow| flow != 0)
                {
                    return Err(crate::rules::ExtractionError::invalid(
                        "inactive path slot contains flow",
                    ));
                }
                continue;
            }
            let mut adjacency = vec![Vec::new(); self.num_vertices];
            for (e, &(u, v)) in self.edges.iter().enumerate() {
                if target_solution[k * flow_vars_per_k + 2 * e] == 1 {
                    adjacency[u].push((v, e));
                }
                if target_solution[k * flow_vars_per_k + 2 * e + 1] == 1 {
                    adjacency[v].push((u, e));
                }
            }

            // A unit flow contains an s-t path; BFS excludes any extra circulation.
            let mut visited = vec![false; self.num_vertices];
            let mut predecessor = vec![None; self.num_vertices];
            let mut queue = VecDeque::from([self.source]);
            visited[self.source] = true;
            while let Some(u) = queue.pop_front() {
                if u == self.sink {
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
            let mut vertex = self.sink;
            while vertex != self.source {
                let (previous, edge) = predecessor[vertex].ok_or_else(|| {
                    crate::rules::ExtractionError::invalid(
                        "active path flow does not connect source to sink",
                    )
                })?;
                path[edge] = true;
                vertex = previous;
            }
        }
        Ok(result)
    }
}

#[reduction(
    transform = upper_bound {
        num_vars = "max_paths * 2 * num_edges + max_paths",
        num_constraints = "max_paths * num_vertices + max_paths * num_edges + max_paths + num_edges + num_vertices + max_paths",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for LengthBoundedDisjointPaths<SimpleGraph> {
    type Result = ReductionLBDPToILP;

    #[allow(clippy::needless_range_loop)]
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let edges = self.graph().edges();

        let m = edges.len();
        let n = self.num_vertices();
        let j = self.max_paths();
        let max_len = Self::exact_i64(self.max_length(), "encoding the path-length bound")?;
        let s = self.source();
        let t = self.sink();

        // Variable layout: flow variables + activation variables a_k
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<bool>>(operation)
        };
        let flow_vars_per_k = m
            .checked_mul(2)
            .ok_or_else(|| overflow("computing the flow-variable stride"))?;
        let num_flow = j
            .checked_mul(flow_vars_per_k)
            .ok_or_else(|| overflow("computing the flow-variable count"))?;
        let a_var = |k: usize| num_flow + k;
        let num_vars = num_flow
            .checked_add(j)
            .ok_or_else(|| overflow("computing the ILP variable count"))?;

        let flow_var = |k: usize, e: usize, dir: usize| k * flow_vars_per_k + 2 * e + dir;

        // Build vertex-to-edge adjacency
        let mut vertex_edges: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (e, &(u, v)) in edges.iter().enumerate() {
            vertex_edges[u].push(e);
            vertex_edges[v].push(e);
        }

        let mut constraints = Vec::new();

        for k in 0..j {
            // Flow conservation: outflow - inflow = a_k at source, -a_k at sink, 0 elsewhere
            for vertex in 0..n {
                let mut terms = Vec::new();
                for &e in &vertex_edges[vertex] {
                    let (eu, _) = edges[e];
                    if vertex == eu {
                        terms.push((flow_var(k, e, 0), 1)); // outgoing
                        terms.push((flow_var(k, e, 1), -1)); // incoming
                    } else {
                        terms.push((flow_var(k, e, 1), 1)); // outgoing
                        terms.push((flow_var(k, e, 0), -1)); // incoming
                    }
                }
                if vertex == s {
                    // outflow - inflow = a_k  =>  outflow - inflow - a_k = 0
                    terms.push((a_var(k), -1));
                    constraints.push(LinearConstraint::eq(terms, 0));
                } else if vertex == t {
                    // outflow - inflow = -a_k  =>  outflow - inflow + a_k = 0
                    terms.push((a_var(k), 1));
                    constraints.push(LinearConstraint::eq(terms, 0));
                } else {
                    constraints.push(LinearConstraint::eq(terms, 0));
                }
            }

            // Anti-parallel
            for e in 0..m {
                constraints.push(LinearConstraint::le(
                    vec![(flow_var(k, e, 0), 1), (flow_var(k, e, 1), 1)],
                    1,
                ));
            }

            // Length bound: total flow for commodity k <= max_length * a_k
            let mut len_terms = Vec::new();
            for e in 0..m {
                len_terms.push((flow_var(k, e, 0), 1));
                len_terms.push((flow_var(k, e, 1), 1));
            }
            len_terms.push((a_var(k), -max_len));
            constraints.push(LinearConstraint::le(len_terms, 0));
        }

        // Edge disjointness: each edge used by at most one commodity
        for e in 0..m {
            let mut terms = Vec::new();
            for k in 0..j {
                terms.push((flow_var(k, e, 0), 1));
                terms.push((flow_var(k, e, 1), 1));
            }
            constraints.push(LinearConstraint::le(terms, 1));
        }

        // Vertex disjointness for non-terminal vertices
        for v in 0..n {
            if v == s || v == t {
                continue;
            }
            let mut terms = Vec::new();
            for k in 0..j {
                for &e in &vertex_edges[v] {
                    let (eu, _) = edges[e];
                    if v == eu {
                        terms.push((flow_var(k, e, 0), 1));
                    } else {
                        terms.push((flow_var(k, e, 1), 1));
                    }
                }
            }
            constraints.push(LinearConstraint::le(terms, 1));
        }

        // Objective: maximize number of active path slots
        let objective: Vec<(usize, i64)> = (0..j).map(|k| (a_var(k), 1)).collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize)
            .map_err(Self::target_construction)?;

        Ok(ReductionLBDPToILP {
            target,
            edges,
            num_vertices: n,
            num_paths: j,
            source: s,
            sink: t,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "lengthboundeddisjointpaths_to_ilp",
        build: || {
            // 4-vertex diamond: s=0, t=3, K=2
            let source = LengthBoundedDisjointPaths::new(
                SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
                0,
                3,
                2,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/lengthboundeddisjointpaths_ilp.rs"]
mod tests;
