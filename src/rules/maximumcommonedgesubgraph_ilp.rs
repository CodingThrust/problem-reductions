//! Reduction from MaximumCommonEdgeSubgraph to ILP (Integer Linear Programming).
//!
//! Binary mapping variables `x_(u,p)` indicate that source vertex `u in V1`
//! is mapped to target vertex `p in V2`. Row and column inequalities encode a
//! partial injective map. For every label-compatible source/target arc pair
//! `((u, lambda, v), (p, lambda, q))` we introduce a binary `y_(a,b)` that is
//! forced to `1` exactly when both `x_(u,p)` and `x_(v,q)` are selected, via
//! the McCormick linearization. The ILP objective is `max sum y_(a,b)`, which
//! equals the count of preserved labelled arcs.
//!
//! This is a direct ILP rendering of the polyhedral formulation studied by
//! Bahiense, Manic, Piva, and de Souza (DAM 2012) adapted to the directed
//! edge-labelled graph model used in the library.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximumCommonEdgeSubgraph;
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MaximumCommonEdgeSubgraph to ILP.
///
/// Variable layout (all binary):
/// - `x_(u,p)` at index `u * n2 + p` for `u in V1`, `p in V2`
/// - `y_(a,b)` for each label-compatible source/target arc pair, indexed
///   sequentially after the `x` block in the order they are enumerated by
///   the constructor.
#[derive(Debug, Clone)]
pub struct ReductionMCESToILP {
    target: ILP<bool>,
    num_vertices_1: usize,
    num_vertices_2: usize,
}

impl ReductionResult for ReductionMCESToILP {
    type Source = MaximumCommonEdgeSubgraph;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract: for each source vertex `u`, output the unique target vertex
    /// `p` with `x_(u,p) = 1`, or the sentinel `n2` ("bottom") when no
    /// mapping variable is selected.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        Ok({
            let n1 = self.num_vertices_1;
            let n2 = self.num_vertices_2;
            (0..n1)
                .map(|u| {
                    (0..n2)
                        .find(|&p| target_solution[u * n2 + p] == 1)
                        .unwrap_or(n2)
                })
                .collect()
        })
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices_1 * num_vertices_2 + num_arcs_1 * num_arcs_2",
        num_constraints = "num_vertices_1 + num_vertices_2 + 3 * num_arcs_1 * num_arcs_2",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumCommonEdgeSubgraph {
    type Result = ReductionMCESToILP;

    fn reduce_to(&self) -> Self::Result {
        let n1 = self.num_vertices_1();
        let n2 = self.num_vertices_2();
        let arcs_1 = self.graph_1().arcs();
        let arcs_2 = self.graph_2().arcs();

        let num_x = n1 * n2;
        let x_idx = |u: usize, p: usize| -> usize { u * n2 + p };

        // Enumerate label-compatible source/target arc pairs in a fixed order
        // so the y-variable indexing is deterministic.
        let mut y_pairs: Vec<(usize, usize)> = Vec::new();
        for (a_idx, a) in arcs_1.iter().enumerate() {
            for (b_idx, b) in arcs_2.iter().enumerate() {
                if a.label == b.label {
                    y_pairs.push((a_idx, b_idx));
                }
            }
        }

        let num_y = y_pairs.len();
        let num_vars = num_x + num_y;
        let y_idx = |seq: usize| -> usize { num_x + seq };

        let mut constraints: Vec<LinearConstraint> = Vec::new();

        // Row constraints: each source vertex maps to at most one target.
        for u in 0..n1 {
            let terms: Vec<(usize, f64)> = (0..n2).map(|p| (x_idx(u, p), 1.0)).collect();
            constraints.push(LinearConstraint::le(terms, 1.0));
        }

        // Column constraints: each target vertex receives at most one source.
        for p in 0..n2 {
            let terms: Vec<(usize, f64)> = (0..n1).map(|u| (x_idx(u, p), 1.0)).collect();
            constraints.push(LinearConstraint::le(terms, 1.0));
        }

        // Linking constraints: y_(a,b) = x_(u,p) AND x_(v,q) via McCormick.
        for (seq, &(a_idx, b_idx)) in y_pairs.iter().enumerate() {
            let a = arcs_1[a_idx];
            let b = arcs_2[b_idx];
            constraints.extend(mccormick_product(
                y_idx(seq),
                x_idx(a.src, b.src),
                x_idx(a.dst, b.dst),
            ));
        }

        // Objective: maximize the number of preserved labelled arcs.
        let objective: Vec<(usize, f64)> = (0..num_y).map(|seq| (y_idx(seq), 1.0)).collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionMCESToILP {
            target,
            num_vertices_1: n1,
            num_vertices_2: n2,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::models::graph::{LabelledArc, LabelledDigraph};
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumcommonedgesubgraph_to_ilp",
        build: || {
            // Small triangle/path instance: optimal MCES preserves 2 arcs.
            let source = MaximumCommonEdgeSubgraph::new(
                LabelledDigraph::new(
                    3,
                    vec![LabelledArc::new(0, 0, 1), LabelledArc::new(1, 1, 2)],
                ),
                LabelledDigraph::new(
                    3,
                    vec![LabelledArc::new(0, 0, 1), LabelledArc::new(1, 1, 2)],
                ),
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumcommonedgesubgraph_ilp.rs"]
mod tests;
