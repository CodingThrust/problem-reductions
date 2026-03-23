//! Reduction from ShortestWeightConstrainedPath to ILP (Integer Linear Programming).
//!
//! Uses directed-arc variables for each orientation of each undirected edge,
//! together with integer order variables for MTZ-style subtour elimination.
//! Flow-balance constraints force a single directed s-t path, and two bound
//! constraints enforce the length and weight limits.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::ShortestWeightConstrainedPath;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing ShortestWeightConstrainedPath to ILP.
///
/// Variable layout (within `ILP<i32>`):
/// - Arc variables: `a_{e,0}` and `a_{e,1}` for each undirected edge `e`
///   (indices `0..2m`), bounded to {0, 1}
/// - Order variables: `o_v` for each vertex `v` (indices `2m..2m+n`),
///   bounded to `[0, n-1]`
#[derive(Debug, Clone)]
pub struct ReductionSWCPToILP {
    target: ILP<i32>,
    num_edges: usize,
}

impl ReductionSWCPToILP {
    fn arc_var(edge_idx: usize, dir: usize) -> usize {
        2 * edge_idx + dir
    }
}

impl ReductionResult for ReductionSWCPToILP {
    type Source = ShortestWeightConstrainedPath<SimpleGraph, i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_edges)
            .map(|edge_idx| {
                usize::from(
                    target_solution
                        .get(Self::arc_var(edge_idx, 0))
                        .copied()
                        .unwrap_or(0)
                        > 0
                        || target_solution
                            .get(Self::arc_var(edge_idx, 1))
                            .copied()
                            .unwrap_or(0)
                            > 0,
                )
            })
            .collect()
    }
}

#[reduction(overhead = {
    num_vars = "2 * num_edges + num_vertices",
    num_constraints = "5 * num_edges + 4 * num_vertices + 3",
})]
impl ReduceTo<ILP<i32>> for ShortestWeightConstrainedPath<SimpleGraph, i32> {
    type Result = ReductionSWCPToILP;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.graph().edges();
        let num_vertices = self.num_vertices();
        let num_edges = self.num_edges();
        let num_vars = 2 * num_edges + num_vertices;
        let source = self.source_vertex();
        let target = self.target_vertex();
        let big_m = num_vertices as f64;

        let order_var = |vertex: usize| 2 * num_edges + vertex;

        // Build adjacency: outgoing[v] and incoming[v] collect arc variable
        // references for arcs leaving / entering vertex v.
        let mut outgoing: Vec<Vec<(usize, f64)>> = vec![Vec::new(); num_vertices];
        let mut incoming: Vec<Vec<(usize, f64)>> = vec![Vec::new(); num_vertices];

        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            let forward = ReductionSWCPToILP::arc_var(edge_idx, 0); // u -> v
            let reverse = ReductionSWCPToILP::arc_var(edge_idx, 1); // v -> u
            outgoing[u].push((forward, 1.0));
            incoming[v].push((forward, 1.0));
            outgoing[v].push((reverse, 1.0));
            incoming[u].push((reverse, 1.0));
        }

        let mut constraints = Vec::new();

        // --- Arc variables are binary within ILP<i32>: 0 <= a_{e,d} <= 1 ---
        for edge_idx in 0..num_edges {
            constraints.push(LinearConstraint::le(
                vec![(ReductionSWCPToILP::arc_var(edge_idx, 0), 1.0)],
                1.0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(ReductionSWCPToILP::arc_var(edge_idx, 1), 1.0)],
                1.0,
            ));
        }

        // --- Order variables stay within [0, |V|-1] ---
        for vertex in 0..num_vertices {
            constraints.push(LinearConstraint::le(
                vec![(order_var(vertex), 1.0)],
                num_vertices.saturating_sub(1) as f64,
            ));
        }

        // --- Flow balance and degree bounds ---
        for vertex in 0..num_vertices {
            // net flow: out - in
            let mut balance_terms = outgoing[vertex].clone();
            for &(var, coef) in &incoming[vertex] {
                balance_terms.push((var, -coef));
            }

            let rhs = if source != target {
                if vertex == source {
                    1.0
                } else if vertex == target {
                    -1.0
                } else {
                    0.0
                }
            } else {
                0.0
            };
            constraints.push(LinearConstraint::eq(balance_terms, rhs));
            constraints.push(LinearConstraint::le(outgoing[vertex].clone(), 1.0));
            constraints.push(LinearConstraint::le(incoming[vertex].clone(), 1.0));
        }

        // --- At most one direction per undirected edge ---
        for edge_idx in 0..num_edges {
            constraints.push(LinearConstraint::le(
                vec![
                    (ReductionSWCPToILP::arc_var(edge_idx, 0), 1.0),
                    (ReductionSWCPToILP::arc_var(edge_idx, 1), 1.0),
                ],
                1.0,
            ));
        }

        // --- MTZ ordering: if arc u->v is selected then order(v) >= order(u) + 1 ---
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            // o_v - o_u - M * a_{e,0} >= 1 - M
            constraints.push(LinearConstraint::ge(
                vec![
                    (order_var(v), 1.0),
                    (order_var(u), -1.0),
                    (ReductionSWCPToILP::arc_var(edge_idx, 0), -big_m),
                ],
                1.0 - big_m,
            ));
            // o_u - o_v - M * a_{e,1} >= 1 - M
            constraints.push(LinearConstraint::ge(
                vec![
                    (order_var(u), 1.0),
                    (order_var(v), -1.0),
                    (ReductionSWCPToILP::arc_var(edge_idx, 1), -big_m),
                ],
                1.0 - big_m,
            ));
        }

        // --- Fix source order to 0 ---
        constraints.push(LinearConstraint::eq(vec![(order_var(source), 1.0)], 0.0));

        // --- Length bound: Σ len_e * (a_{e,0} + a_{e,1}) <= length_bound ---
        let length_terms: Vec<(usize, f64)> = edges
            .iter()
            .enumerate()
            .flat_map(|(edge_idx, _)| {
                let coeff = self.edge_lengths()[edge_idx].to_sum() as f64;
                [
                    (ReductionSWCPToILP::arc_var(edge_idx, 0), coeff),
                    (ReductionSWCPToILP::arc_var(edge_idx, 1), coeff),
                ]
            })
            .collect();
        constraints.push(LinearConstraint::le(
            length_terms,
            *self.length_bound() as f64,
        ));

        // --- Weight bound: Σ wt_e * (a_{e,0} + a_{e,1}) <= weight_bound ---
        let weight_terms: Vec<(usize, f64)> = edges
            .iter()
            .enumerate()
            .flat_map(|(edge_idx, _)| {
                let coeff = self.edge_weights()[edge_idx].to_sum() as f64;
                [
                    (ReductionSWCPToILP::arc_var(edge_idx, 0), coeff),
                    (ReductionSWCPToILP::arc_var(edge_idx, 1), coeff),
                ]
            })
            .collect();
        constraints.push(LinearConstraint::le(
            weight_terms,
            *self.weight_bound() as f64,
        ));

        // Feasibility problem: use a dummy zero objective with Minimize.
        let objective: Vec<(usize, f64)> = vec![];
        let target_ilp = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionSWCPToILP {
            target: target_ilp,
            num_edges,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "shortestweightconstrainedpath_to_ilp",
        build: || {
            // 3-vertex path: 0 -- 1 -- 2, s=0, t=2
            // edge_lengths = [2, 3], edge_weights = [1, 2]
            // length_bound = 6, weight_bound = 4
            // The only s-t path uses both edges: length=5 <= 6, weight=3 <= 4 => feasible
            let source = ShortestWeightConstrainedPath::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![2, 3],
                vec![1, 2],
                0,
                2,
                6,
                4,
            );
            // ILP vars: a_{0,fwd}, a_{0,rev}, a_{1,fwd}, a_{1,rev}, o_0, o_1, o_2
            // Path 0->1->2: a_{0,fwd}=1, a_{1,fwd}=1, orders: 0, 1, 2
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                source,
                SolutionPair {
                    source_config: vec![1, 1],
                    target_config: vec![1, 0, 1, 0, 0, 1, 2],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/shortestweightconstrainedpath_ilp.rs"]
mod tests;
