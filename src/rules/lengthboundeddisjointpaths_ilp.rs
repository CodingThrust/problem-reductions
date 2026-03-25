//! Reduction from LengthBoundedDisjointPaths to ILP.
//!
//! Binary flow variables per commodity per directed edge orientation.
//! Conservation, edge/vertex disjointness, and length bound.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::LengthBoundedDisjointPaths;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing LengthBoundedDisjointPaths to ILP.
///
/// Variable layout (all binary):
/// - Flow: `f^k_{e,dir}` at index `k * 2m + 2e + dir`
///
/// Total: `J * 2m` variables.
#[derive(Debug, Clone)]
pub struct ReductionLBDPToILP {
    target: ILP<bool>,
    /// Canonical sorted edges.
    edges: Vec<(usize, usize)>,
    num_vertices: usize,
    num_paths: usize,
}

impl ReductionResult for ReductionLBDPToILP {
    type Source = LengthBoundedDisjointPaths<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // For each path slot k, set the source vertex-indicator block to 1
        // exactly on the vertices incident to the commodity-k path, including s and t.
        let m = self.edges.len();
        let n = self.num_vertices;
        let j = self.num_paths;
        let flow_vars_per_k = 2 * m;

        let mut result = vec![0usize; j * n];
        for k in 0..j {
            // Find which vertices are on the path for commodity k
            let mut on_path = vec![false; n];
            for e in 0..m {
                let (u, v) = self.edges[e];
                let fwd = target_solution[k * flow_vars_per_k + 2 * e];
                let rev = target_solution[k * flow_vars_per_k + 2 * e + 1];
                if fwd == 1 {
                    on_path[u] = true;
                    on_path[v] = true;
                }
                if rev == 1 {
                    on_path[u] = true;
                    on_path[v] = true;
                }
            }
            for v in 0..n {
                if on_path[v] {
                    result[k * n + v] = 1;
                }
            }
        }
        result
    }
}

#[reduction(
    overhead = {
        num_vars = "max_paths * 2 * num_edges + max_paths",
        num_constraints = "max_paths * num_vertices + max_paths * num_edges + max_paths + num_edges + num_vertices + max_paths",
    }
)]
impl ReduceTo<ILP<bool>> for LengthBoundedDisjointPaths<SimpleGraph> {
    type Result = ReductionLBDPToILP;

    #[allow(clippy::needless_range_loop)]
    fn reduce_to(&self) -> Self::Result {
        let mut edges: Vec<(usize, usize)> = self
            .graph()
            .edges()
            .into_iter()
            .map(|(u, v)| if u <= v { (u, v) } else { (v, u) })
            .collect();
        edges.sort_unstable();

        let m = edges.len();
        let n = self.num_vertices();
        let j = self.max_paths();
        let max_len = self.max_length();
        let s = self.source();
        let t = self.sink();

        // Variable layout: flow variables + activation variables a_k
        let flow_vars_per_k = 2 * m;
        let num_flow = j * flow_vars_per_k;
        let a_var = |k: usize| num_flow + k;
        let num_vars = num_flow + j;

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
                        terms.push((flow_var(k, e, 0), 1.0)); // outgoing
                        terms.push((flow_var(k, e, 1), -1.0)); // incoming
                    } else {
                        terms.push((flow_var(k, e, 1), 1.0)); // outgoing
                        terms.push((flow_var(k, e, 0), -1.0)); // incoming
                    }
                }
                if vertex == s {
                    // outflow - inflow = a_k  =>  outflow - inflow - a_k = 0
                    terms.push((a_var(k), -1.0));
                    constraints.push(LinearConstraint::eq(terms, 0.0));
                } else if vertex == t {
                    // outflow - inflow = -a_k  =>  outflow - inflow + a_k = 0
                    terms.push((a_var(k), 1.0));
                    constraints.push(LinearConstraint::eq(terms, 0.0));
                } else {
                    constraints.push(LinearConstraint::eq(terms, 0.0));
                }
            }

            // Anti-parallel
            for e in 0..m {
                constraints.push(LinearConstraint::le(
                    vec![(flow_var(k, e, 0), 1.0), (flow_var(k, e, 1), 1.0)],
                    1.0,
                ));
            }

            // Length bound: total flow for commodity k <= max_length * a_k
            let mut len_terms = Vec::new();
            for e in 0..m {
                len_terms.push((flow_var(k, e, 0), 1.0));
                len_terms.push((flow_var(k, e, 1), 1.0));
            }
            len_terms.push((a_var(k), -(max_len as f64)));
            constraints.push(LinearConstraint::le(len_terms, 0.0));
        }

        // Edge disjointness: each edge used by at most one commodity
        for e in 0..m {
            let mut terms = Vec::new();
            for k in 0..j {
                terms.push((flow_var(k, e, 0), 1.0));
                terms.push((flow_var(k, e, 1), 1.0));
            }
            constraints.push(LinearConstraint::le(terms, 1.0));
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
                        terms.push((flow_var(k, e, 0), 1.0));
                    } else {
                        terms.push((flow_var(k, e, 1), 1.0));
                    }
                }
            }
            constraints.push(LinearConstraint::le(terms, 1.0));
        }

        // Objective: maximize number of active path slots
        let objective: Vec<(usize, f64)> = (0..j).map(|k| (a_var(k), 1.0)).collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionLBDPToILP {
            target,
            edges,
            num_vertices: n,
            num_paths: j,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::rules::ReduceTo as _;

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
#[path = "../unit_tests/rules/lengthboundeddisjointpaths_ilp.rs"]
mod tests;
