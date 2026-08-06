//! Reduction from MinimumCostMaximumFlow to MinimumCostCirculation.
//!
//! Standard textbook equivalence: a minimum-cost maximum-flow instance
//! `(G, s, t, c, cost)` can be solved as a minimum-cost circulation on
//! the augmented graph `G' = G + (t -> s)` where the new return arc has
//! capacity `U = sum_{e in delta^+(s)} u_e` and cost `-B` with
//! `B = 1 + sum_{e in E} c_e`.
//!
//! Because `B` strictly exceeds the cost of any feasible `s-t` flow,
//! the negative return arc forces the circulation to push the flow
//! value `|f|` as large as possible (lex priority), and ties on the
//! flow value are broken by minimizing the original arc cost — exactly
//! the lexicographic objective of MinimumCostMaximumFlow.
//!
//! Reference: MIT 6.854 Course Staff, "Min-cost flow algorithms",
//! <https://courses.csail.mit.edu/6.854/21/Scribe/s10-minCostFlowAlg/s10-minCostFlowAlg.html>.

use crate::models::graph::{MinimumCostCirculation, MinimumCostMaximumFlow};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::DirectedGraph;

/// Result of reducing MinimumCostMaximumFlow to MinimumCostCirculation.
///
/// The target circulation graph keeps every original arc in order and
/// appends a single return arc `(t, s)` at the end. The original arc
/// count `num_original_arcs` is the number of flow variables to recover
/// when extracting a witness configuration.
#[derive(Debug, Clone)]
pub struct ReductionMCMFToMCC {
    target: MinimumCostCirculation,
    num_original_arcs: usize,
}

impl ReductionResult for ReductionMCMFToMCC {
    type Source = MinimumCostMaximumFlow;
    type Target = MinimumCostCirculation;

    fn target_problem(&self) -> &MinimumCostCirculation {
        &self.target
    }

    /// Extract the source flow by discarding the return arc: the first
    /// `num_original_arcs` entries of the circulation are exactly the
    /// flow values on the original arcs.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_original_arcs].to_vec())
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_arcs = "num_arcs + 1",
    }
)]
impl ReduceTo<MinimumCostCirculation> for MinimumCostMaximumFlow {
    type Result = ReductionMCMFToMCC;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let m = self.num_arcs();
        let source = self.source();
        let sink = self.sink();

        // U = sum of capacities of arcs leaving the source.
        let u_bound: i64 = self
            .graph()
            .arcs()
            .iter()
            .zip(self.capacities().iter())
            .filter_map(|(&(u, _), &cap)| if u == source { Some(cap) } else { None })
            .sum();

        // B = 1 + sum of all original arc costs. Strictly exceeds any
        // simple s-t path cost, so the return arc's negative cost
        // dominates all positive original costs lexicographically.
        let b_const: i64 = 1 + self.costs().iter().sum::<i64>();

        // Keep every original arc and append the return arc (t, s).
        let mut arcs = self.graph().arcs();
        arcs.push((sink, source));

        let mut capacities = self.capacities().to_vec();
        capacities.push(u_bound);

        let mut costs = self.costs().to_vec();
        costs.push(-b_const);

        let target = MinimumCostCirculation::new(DirectedGraph::new(n, arcs), capacities, costs);

        ReductionMCMFToMCC {
            target,
            num_original_arcs: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumcostmaximumflow_to_minimumcostcirculation",
        build: || {
            // Canonical 4-vertex diamond from issue #1029/#1031.
            // Optimal source flow: [2, 1, 1, 1, 2] with value 3, cost 7.
            // Target circulation appends return arc (3 -> 0) with
            // flow value 3, giving config [2, 1, 1, 1, 2, 3].
            let source = MinimumCostMaximumFlow::new(
                DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)]),
                0,
                3,
                vec![2, 1, 1, 1, 2],
                vec![1, 0, 0, 1, 2],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MinimumCostCirculation>(
                source,
                SolutionPair {
                    source_config: vec![2, 1, 1, 1, 2],
                    target_config: vec![2, 1, 1, 1, 2, 3],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumcostmaximumflow_minimumcostcirculation.rs"]
mod tests;
