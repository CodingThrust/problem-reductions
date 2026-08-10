//! Reduction from MaximumEdgeWeightedKClique to ILP (Integer Linear Programming).
//!
//! Binary variables `x_v` per vertex (1 iff vertex `v` is selected) and
//! `y_uv` per graph edge (1 iff both endpoints are selected). An exact
//! cardinality constraint forces `|S| = k`. For every non-edge `{u, v}`, the
//! pair `x_u + x_v <= 1` rules out non-adjacent selected pairs, so the
//! selected vertex set is forced to be a clique. For every edge `{u, v}`,
//! the McCormick triple `y_uv <= x_u`, `y_uv <= x_v`,
//! `y_uv >= x_u + x_v - 1` linearizes the AND of the endpoint variables, so
//! `y_uv = 1` iff both endpoints are selected. The ILP objective
//! `max sum_{{u,v} in E} w_uv * y_uv` then matches the induced edge-weight
//! total of the source instance.
//!
//! The lower-bound constraint `y_uv >= x_u + x_v - 1` is required because
//! edge weights may be negative: without it the ILP could leave a
//! negative-weight `y_uv` at zero even when both endpoints are selected,
//! over-reporting the objective.
//!
//! Reference: Park, Lee, and Park, "An extended formulation approach to the
//! edge-weighted maximal clique problem," EJOR 95(3):671--682 (1996);
//! Gouveia and Martins, "Solving the maximum edge-weight clique problem in
//! sparse graphs with compact formulations," EURO J. Comput. Optim. 3(1)
//! (2015).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximumEdgeWeightedKClique;
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::Graph;
use crate::types::WeightElement;
use crate::variant::VariantParam;
use std::marker::PhantomData;

/// Result of reducing MaximumEdgeWeightedKClique to ILP.
///
/// Variable layout (all binary):
/// - `x_v` at index `v` for `v in [0, num_vertices)`,
/// - `y_uv` at index `num_vertices + e` for the `e`-th graph edge in
///   `graph.edges()` order.
#[derive(Debug, Clone)]
pub struct ReductionMaximumEdgeWeightedKCliqueToILP<W> {
    target: ILP<bool>,
    num_vertices: usize,
    _marker: PhantomData<W>,
}

impl<W> ReductionResult for ReductionMaximumEdgeWeightedKCliqueToILP<W>
where
    W: WeightElement + VariantParam,
{
    type Source = MaximumEdgeWeightedKClique<W>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract: take the first `num_vertices` entries of the ILP solution.
    /// They are exactly the binary `x_v` selection variables.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_vertices].to_vec())
    }
}

fn build_reduction<W>(
    src: &MaximumEdgeWeightedKClique<W>,
    objective_coefficients: Vec<f64>,
) -> ReductionMaximumEdgeWeightedKCliqueToILP<W>
where
    W: WeightElement + VariantParam,
{
    let n = src.num_vertices();
    let edges = src.graph().edges();
    let m = edges.len();
    debug_assert_eq!(objective_coefficients.len(), m);

    let num_vars = n + m;
    let x_idx = |v: usize| -> usize { v };
    let y_idx = |e: usize| -> usize { n + e };

    let mut constraints: Vec<LinearConstraint> = Vec::new();

    // Exact-cardinality constraint: sum_v x_v = k.
    let cardinality_terms: Vec<(usize, f64)> = (0..n).map(|v| (x_idx(v), 1.0)).collect();
    constraints.push(LinearConstraint::eq(cardinality_terms, src.k() as f64));

    // Non-edge clique constraints: x_u + x_v <= 1 for every non-edge.
    for u in 0..n {
        for v in (u + 1)..n {
            if !src.graph().has_edge(u, v) {
                constraints.push(LinearConstraint::le(
                    vec![(x_idx(u), 1.0), (x_idx(v), 1.0)],
                    1.0,
                ));
            }
        }
    }

    // Linking constraints: y_uv = x_u AND x_v via McCormick.
    for (e, &(u, v)) in edges.iter().enumerate() {
        constraints.extend(mccormick_product(y_idx(e), x_idx(u), x_idx(v)));
    }

    // Objective: maximize sum_e w_e * y_e.
    let objective: Vec<(usize, f64)> = objective_coefficients
        .into_iter()
        .enumerate()
        .map(|(e, w)| (y_idx(e), w))
        .collect();

    let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

    ReductionMaximumEdgeWeightedKCliqueToILP {
        target,
        num_vertices: n,
        _marker: PhantomData,
    }
}

#[reduction(
    exact = {
        num_vars = "num_vertices + num_edges",
        num_constraints = "1 + num_vertices * (num_vertices - 1) / 2 + 2 * num_edges",
    },
    unavailable = {
        coefficient_encoding_bits = "the source size vector omits coefficient magnitudes and sparsity needed to bound the encoded coefficients",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumEdgeWeightedKClique<i32> {
    type Result = ReductionMaximumEdgeWeightedKCliqueToILP<i32>;

    fn reduce_to(&self) -> Self::Result {
        let coefficients: Vec<f64> = self.edge_weights().iter().map(|w| *w as f64).collect();
        build_reduction(self, coefficients)
    }
}

#[reduction(
    exact = {
        num_vars = "num_vertices + num_edges",
        num_constraints = "1 + num_vertices * (num_vertices - 1) / 2 + 2 * num_edges",
    },
    unavailable = {
        coefficient_encoding_bits = "the source size vector omits coefficient magnitudes and sparsity needed to bound the encoded coefficients",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumEdgeWeightedKClique<f64> {
    type Result = ReductionMaximumEdgeWeightedKCliqueToILP<f64>;

    fn reduce_to(&self) -> Self::Result {
        let coefficients: Vec<f64> = self.edge_weights().to_vec();
        build_reduction(self, coefficients)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::SimpleGraph;
    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumedgeweightedkclique_i32_to_ilp",
            build: || {
                // Canonical issue #1020 instance: 4 vertices, 5 edges, k = 3.
                // Optimum induced weight is 5 + 4 + (-1) = 8 on clique {0, 1, 2}.
                let source = MaximumEdgeWeightedKClique::<i32>::new(
                    SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
                    vec![5, 4, -1, 1, 0],
                    3,
                );
                crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumedgeweightedkclique_f64_to_ilp",
            build: || {
                let source = MaximumEdgeWeightedKClique::<f64>::new(
                    SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
                    vec![5.0, 4.0, -1.0, 1.0, 0.0],
                    3,
                );
                crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumedgeweightedkclique_ilp.rs"]
mod tests;
