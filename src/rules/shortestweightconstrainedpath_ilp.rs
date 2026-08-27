//! Reduction from ShortestWeightConstrainedPath to ILP (Integer Linear Programming).
//!
//! Uses directed-arc variables for each orientation of each undirected edge,
//! together with integer order variables for MTZ-style subtour elimination.
//! Flow-balance constraints force a single directed s-t path, the weight
//! bound constraint enforces the weight limit, and the objective minimizes
//! total path length.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::ShortestWeightConstrainedPath;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::{i64_to_exact_f64, WeightElement};

/// Result of reducing ShortestWeightConstrainedPath to ILP.
///
/// Variable layout (within `ILP<i64>`):
/// - Arc variables: `a_{e,0}` and `a_{e,1}` for each undirected edge `e`
///   (indices `0..2m`), bounded to {0, 1}
/// - Order variables: `o_v` for each vertex `v` (indices `2m..2m+n`),
///   bounded to `[0, n-1]`
#[derive(Debug, Clone)]
pub struct ReductionSWCPToILP {
    target: ILP<i64>,
    num_edges: usize,
}

impl ReductionSWCPToILP {
    fn arc_var(edge_idx: usize, dir: usize) -> usize {
        2 * edge_idx + dir
    }
}

impl ReductionResult for ReductionSWCPToILP {
    type Source = ShortestWeightConstrainedPath<SimpleGraph, i64>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            (0..self.num_edges)
                .map(|edge_idx| {
                    target_solution[Self::arc_var(edge_idx, 0)] > 0
                        || target_solution[Self::arc_var(edge_idx, 1)] > 0
                })
                .collect()
        })
    }
}

#[reduction(
    size = exact {
        num_vars = "2 * num_edges + num_vertices",
        num_constraints = "5 * num_edges + 4 * num_vertices + 2",
    },)]
impl ReduceTo<ILP<i64>> for ShortestWeightConstrainedPath<SimpleGraph, i64> {
    type Result = ReductionSWCPToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let edges = self.graph().edges();
        let num_vertices = self.num_vertices();
        let num_edges = self.num_edges();
        let num_vars = 2 * num_edges + num_vertices;
        let source = self.source_vertex();
        let target = self.target_vertex();
        let big_m = Self::exact_i64(num_vertices, "encoding the vertex order")?;

        let order_var = |vertex: usize| 2 * num_edges + vertex;

        // Build adjacency: outgoing[v] and incoming[v] collect arc variable
        // references for arcs leaving / entering vertex v.
        let mut outgoing: Vec<Vec<(usize, i64)>> = vec![Vec::new(); num_vertices];
        let mut incoming: Vec<Vec<(usize, i64)>> = vec![Vec::new(); num_vertices];

        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            let forward = ReductionSWCPToILP::arc_var(edge_idx, 0); // u -> v
            let reverse = ReductionSWCPToILP::arc_var(edge_idx, 1); // v -> u
            outgoing[u].push((forward, 1));
            incoming[v].push((forward, 1));
            outgoing[v].push((reverse, 1));
            incoming[u].push((reverse, 1));
        }

        let mut constraints = Vec::new();

        // --- Arc variables are binary within `ILP<i64>`: 0 <= a_{e,d} <= 1 ---
        for edge_idx in 0..num_edges {
            constraints.push(LinearConstraint::le(
                vec![(ReductionSWCPToILP::arc_var(edge_idx, 0), 1)],
                1,
            ));
            constraints.push(LinearConstraint::le(
                vec![(ReductionSWCPToILP::arc_var(edge_idx, 1), 1)],
                1,
            ));
        }

        // --- Order variables stay within [0, |V|-1] ---
        let max_order = if num_vertices == 0 { 0 } else { big_m - 1 };
        for vertex in 0..num_vertices {
            constraints.push(LinearConstraint::le(
                vec![(order_var(vertex), 1)],
                max_order,
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
                    1
                } else if vertex == target {
                    -1
                } else {
                    0
                }
            } else {
                0
            };
            constraints.push(LinearConstraint::eq(balance_terms, rhs));
            constraints.push(LinearConstraint::le(outgoing[vertex].clone(), 1));
            constraints.push(LinearConstraint::le(incoming[vertex].clone(), 1));
        }

        // --- At most one direction per undirected edge ---
        for edge_idx in 0..num_edges {
            constraints.push(LinearConstraint::le(
                vec![
                    (ReductionSWCPToILP::arc_var(edge_idx, 0), 1),
                    (ReductionSWCPToILP::arc_var(edge_idx, 1), 1),
                ],
                1,
            ));
        }

        // --- MTZ ordering: if arc u->v is selected then order(v) >= order(u) + 1 ---
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            // o_v - o_u - M * a_{e,0} >= 1 - M
            constraints.push(LinearConstraint::ge(
                vec![
                    (order_var(v), 1),
                    (order_var(u), -1),
                    (ReductionSWCPToILP::arc_var(edge_idx, 0), -big_m),
                ],
                1 - big_m,
            ));
            // o_u - o_v - M * a_{e,1} >= 1 - M
            constraints.push(LinearConstraint::ge(
                vec![
                    (order_var(u), 1),
                    (order_var(v), -1),
                    (ReductionSWCPToILP::arc_var(edge_idx, 1), -big_m),
                ],
                1 - big_m,
            ));
        }

        // --- Fix source order to 0 ---
        constraints.push(LinearConstraint::eq(vec![(order_var(source), 1)], 0));

        // --- Weight bound: Σ wt_e * (a_{e,0} + a_{e,1}) <= weight_bound ---
        let edge_weights: Vec<i64> = self
            .edge_weights()
            .iter()
            .map(WeightElement::to_sum)
            .collect();
        let weight_terms: Vec<(usize, i64)> = edges
            .iter()
            .enumerate()
            .flat_map(|(edge_idx, _)| {
                let coeff = edge_weights[edge_idx];
                [
                    (ReductionSWCPToILP::arc_var(edge_idx, 0), coeff),
                    (ReductionSWCPToILP::arc_var(edge_idx, 1), coeff),
                ]
            })
            .collect();
        constraints.push(LinearConstraint::le(weight_terms, *self.weight_bound()));

        // --- Objective: minimize total path length ---
        let edge_lengths: Vec<f64> = self
            .edge_lengths()
            .iter()
            .map(|length| {
                i64_to_exact_f64(length.to_sum()).map_err(|error| {
                    crate::rules::ReductionError::inexact_float_conversion::<
                        ShortestWeightConstrainedPath<SimpleGraph, i64>,
                        ILP<i64>,
                    >(error)
                })
            })
            .collect::<Result<_, _>>()?;
        let objective: Vec<(usize, f64)> = edges
            .iter()
            .enumerate()
            .flat_map(|(edge_idx, _)| {
                let coeff = edge_lengths[edge_idx];
                [
                    (ReductionSWCPToILP::arc_var(edge_idx, 0), coeff),
                    (ReductionSWCPToILP::arc_var(edge_idx, 1), coeff),
                ]
            })
            .collect();
        let target_ilp = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionSWCPToILP {
            target: target_ilp,
            num_edges,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "shortestweightconstrainedpath_to_ilp",
        build: || {
            // 3-vertex path: 0 -- 1 -- 2, s=0, t=2
            // edge_lengths = [2, 3], edge_weights = [1, 2]
            // weight_bound = 4
            // The only s-t path uses both edges: length=5, weight=3 <= 4 => feasible
            let source = ShortestWeightConstrainedPath::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![2, 3],
                vec![1, 2],
                0,
                2,
                4,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/shortestweightconstrainedpath_ilp.rs"]
mod tests;
