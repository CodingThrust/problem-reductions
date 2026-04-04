//! Reduction from OptimumCommunicationSpanningTree to ILP (Integer Linear Programming).
//!
//! Uses a multi-commodity flow formulation:
//! - Binary edge variables x_e for each edge of K_n
//! - For each pair (u,v) with r(u,v) > 0, directed flow variables route 1 unit
//!   from u to v through the tree
//! - Tree constraints: sum x_e = n-1, and connectivity via flow conservation
//! - Objective: minimize sum_{(u,v): r>0} r(u,v) * w(e) * (flow_uv(e->dir) + flow_uv(e<-dir))

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::OptimumCommunicationSpanningTree;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing OptimumCommunicationSpanningTree to ILP.
///
/// Variable layout (all binary):
/// - `x_e` for each undirected edge `e` (indices `0..m`)
/// - For each commodity `k` (pair (u,v) with u < v and r(u,v) > 0):
///   `f^k_(i,j)` and `f^k_(j,i)` for each edge (i,j), two directed flow variables
///   (indices `m + k * 2m .. m + (k+1) * 2m`)
#[derive(Debug, Clone)]
pub struct ReductionOptimumCommunicationSpanningTreeToILP {
    target: ILP<bool>,
    num_edges: usize,
}

impl ReductionResult for ReductionOptimumCommunicationSpanningTreeToILP {
    type Source = OptimumCommunicationSpanningTree;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_edges].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_edges + 2 * num_edges * num_vertices * (num_vertices - 1) / 2",
        num_constraints = "1 + num_vertices * num_vertices * (num_vertices - 1) / 2 + 2 * num_edges * num_vertices * (num_vertices - 1) / 2",
    }
)]
impl ReduceTo<ILP<bool>> for OptimumCommunicationSpanningTree {
    type Result = ReductionOptimumCommunicationSpanningTreeToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let m = self.num_edges();
        let edges = self.edges();
        let w = self.edge_weights();
        let r = self.requirements();

        // Enumerate commodities: all pairs (s, t) with s < t and r(s,t) > 0
        let mut commodities: Vec<(usize, usize)> = Vec::new();
        for (s, row) in r.iter().enumerate() {
            for (t, &req) in row.iter().enumerate().skip(s + 1) {
                if req > 0 {
                    commodities.push((s, t));
                }
            }
        }
        let num_commodities = commodities.len();

        let num_vars = m + 2 * m * num_commodities;
        let mut constraints = Vec::new();

        // Edge variable index
        let edge_var = |edge_idx: usize| edge_idx;

        // Flow variable index: for commodity k, edge e, direction dir (0 = i->j, 1 = j->i)
        let flow_var =
            |k: usize, edge_idx: usize, dir: usize| -> usize { m + k * 2 * m + 2 * edge_idx + dir };

        // Constraint 1: Tree has exactly n-1 edges
        // sum x_e = n-1
        let tree_terms: Vec<(usize, f64)> = (0..m).map(|e| (edge_var(e), 1.0)).collect();
        constraints.push(LinearConstraint::eq(tree_terms, (n - 1) as f64));

        // Constraint 2: Flow conservation for each commodity
        for (k, &(src, dst)) in commodities.iter().enumerate() {
            for vertex in 0..n {
                let mut terms = Vec::new();
                for (edge_idx, &(i, j)) in edges.iter().enumerate() {
                    // Flow into vertex minus flow out of vertex
                    if j == vertex {
                        // Edge (i, j): direction 0 = i->j (inflow), direction 1 = j->i (outflow)
                        terms.push((flow_var(k, edge_idx, 0), 1.0));
                        terms.push((flow_var(k, edge_idx, 1), -1.0));
                    }
                    if i == vertex {
                        // Edge (i, j): direction 1 = j->i (inflow), direction 0 = i->j (outflow)
                        terms.push((flow_var(k, edge_idx, 1), 1.0));
                        terms.push((flow_var(k, edge_idx, 0), -1.0));
                    }
                }

                let rhs = if vertex == src {
                    -1.0 // source: net outflow of 1
                } else if vertex == dst {
                    1.0 // sink: net inflow of 1
                } else {
                    0.0 // transit: balanced
                };
                constraints.push(LinearConstraint::eq(terms, rhs));
            }
        }

        // Constraint 3: Capacity linking: flow <= edge selector
        for k in 0..num_commodities {
            for edge_idx in 0..m {
                let sel = edge_var(edge_idx);
                // f^k_(i->j) <= x_e
                constraints.push(LinearConstraint::le(
                    vec![(flow_var(k, edge_idx, 0), 1.0), (sel, -1.0)],
                    0.0,
                ));
                // f^k_(j->i) <= x_e
                constraints.push(LinearConstraint::le(
                    vec![(flow_var(k, edge_idx, 1), 1.0), (sel, -1.0)],
                    0.0,
                ));
            }
        }

        // Objective: minimize sum over commodities k of r(s,t) * sum_e w(e) * (f^k_e_fwd + f^k_e_bwd)
        // This equals sum_{s<t} r(s,t) * W_T(s,t) because flow routes exactly along the tree path.
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for (k, &(s, t)) in commodities.iter().enumerate() {
            let req = r[s][t] as f64;
            for (edge_idx, &(i, j)) in edges.iter().enumerate() {
                let weight = w[i][j] as f64;
                let coeff = req * weight;
                if coeff != 0.0 {
                    objective.push((flow_var(k, edge_idx, 0), coeff));
                    objective.push((flow_var(k, edge_idx, 1), coeff));
                }
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionOptimumCommunicationSpanningTreeToILP {
            target,
            num_edges: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "optimum_communication_spanning_tree_to_ilp",
        build: || {
            // K3 example from issue #967
            let edge_weights = vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]];
            let requirements = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
            let source = OptimumCommunicationSpanningTree::new(edge_weights, requirements);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/optimumcommunicationspanningtree_ilp.rs"]
mod tests;
