//! Reduction from BicliqueCover to ILP.
//!
//! Variables: binary x_{l,b} for left-vertex/biclique membership,
//!            binary y_{r,b} for right-vertex/biclique membership,
//!            binary z_{(l,r),b} = x_{l,b} * y_{r,b} (McCormick product).
//! Sub-biclique constraint: for every non-edge (l,r) ∉ E(G) and every
//!     biclique b, x_{l,b} + y_{r,b} ≤ 1, so no biclique covers any
//!     non-edge of G.
//! Coverage: Σ_b z_{(l,r),b} ≥ 1 for every edge (l,r) ∈ E(G).
//! Objective: minimize Σ x_{l,b} + Σ y_{r,b}.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::BicliqueCover;
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};
#[derive(Debug, Clone)]
pub struct ReductionBicliqueCoverToILP {
    target: ILP<bool>,
    /// Number of source-problem variables (num_vertices * k).
    source_vars: usize,
}

impl ReductionResult for ReductionBicliqueCoverToILP {
    type Source = BicliqueCover;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract the vertex-by-biclique membership bits, discarding z auxiliaries.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.source_vars].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices * rank + num_vertices * num_vertices * rank",
        num_constraints = "num_vertices * num_vertices * rank + num_vertices * num_vertices * rank + num_edges",
    }
)]
impl ReduceTo<ILP<bool>> for BicliqueCover {
    type Result = ReductionBicliqueCoverToILP;

    fn reduce_to(&self) -> Self::Result {
        use crate::topology::Graph;
        let left = self.left_size();
        let right = self.right_size();
        let n = left + right;
        let k = self.k();
        let mut constraints = Vec::new();

        // Variable layout:
        // x_{l,b}: index l*k + b          (left membership)         [0, left*k)
        // y_{r,b}: index left*k + r*k + b  (right membership)       [left*k, n*k)
        // z_{(l,r),b}: index n*k + (l*right + r)*k + b  (products)  [n*k, n*k + left*right*k)
        let x_idx = |l: usize, b: usize| -> usize { l * k + b };
        let y_idx = |r: usize, b: usize| -> usize { left * k + r * k + b };
        let z_idx = |l: usize, r: usize, b: usize| -> usize { n * k + (l * right + r) * k + b };

        let num_vars = n * k + left * right * k;
        let source_vars = n * k;

        // Edge lookup using graph adjacency in unified coords.
        let graph = self.graph();

        // McCormick for z_{(l,r),b} = x_{l,b} * y_{r,b},
        // plus sub-biclique constraint: x_{l,b} + y_{r,b} ≤ 1 for non-edges.
        for l in 0..left {
            for r in 0..right {
                for b in 0..k {
                    constraints.extend(mccormick_product(z_idx(l, r, b), x_idx(l, b), y_idx(r, b)));
                    if !graph.has_edge(l, left + r) {
                        constraints.push(LinearConstraint::le(
                            vec![(x_idx(l, b), 1.0), (y_idx(r, b), 1.0)],
                            1.0,
                        ));
                    }
                }
            }
        }

        // Coverage: Σ_b z_{(l,r),b} ≥ 1 for every edge
        for &(l, r) in self.graph().left_edges() {
            let terms: Vec<(usize, f64)> = (0..k).map(|b| (z_idx(l, r, b), 1.0)).collect();
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        // Objective: minimize Σ x_{l,b} + Σ y_{r,b}
        let mut objective: Vec<(usize, f64)> = Vec::with_capacity(n * k);
        for v in 0..n {
            for b in 0..k {
                objective.push((v * k + b, 1.0));
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionBicliqueCoverToILP {
            target,
            source_vars,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::BipartiteGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "bicliquecover_to_ilp",
        build: || {
            // L={0,1}, R={0,1,2}, edges: (0,0),(0,1),(1,1),(1,2), k=2
            let source = BicliqueCover::new(
                BipartiteGraph::new(2, 3, vec![(0, 0), (0, 1), (1, 1), (1, 2)]),
                2,
            );
            let reduction: ReductionBicliqueCoverToILP =
                crate::rules::ReduceTo::<ILP<bool>>::reduce_to(&source);
            let ilp_sol = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("ILP should be solvable");
            let extracted = reduction.extract_solution(&ilp_sol);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: extracted,
                    target_config: ilp_sol,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/bicliquecover_ilp.rs"]
mod tests;
