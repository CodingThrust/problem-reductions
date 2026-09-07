//! 3-SAT to the bounded-weight quotient-DAG partition problem.
//!
//! Compose the formal SAT-to-clique reduction with an incidence construction.
//! Unit vertex/edge items encode a clique under a cardinality bound. Two heavy
//! anchors and polynomial arc costs force exactly two blocks and encode the
//! incidence closure and clique-size inequalities, without digit-encoded weights.

use crate::models::formula::KSatisfiability;
use crate::models::graph::{AcyclicPartition, KClique};
use crate::reduction;
use crate::rules::ksatisfiability_kclique::Reduction3SATToKClique;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{DirectedGraph, Graph, SimpleGraph};
use crate::variant::K3;

/// Literal-clique extraction follows the incidence partition projection.
#[derive(Debug, Clone)]
pub struct Reduction3SATToAcyclicPartition {
    sat_to_clique: Reduction3SATToKClique,
    target: AcyclicPartition<i64>,
    source_vertex: usize,
}

impl ReductionResult for Reduction3SATToAcyclicPartition {
    type Source = KSatisfiability<K3>;
    type Target = AcyclicPartition<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        if !crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?
            .0
        {
            return Err(crate::rules::ExtractionError::invalid(
                "target partition does not satisfy the acyclic partition constraints",
            ));
        }
        let source_label = target_solution[self.source_vertex];
        let selected = target_solution[..self.sat_to_clique.target_problem().num_vertices()]
            .iter()
            .map(|&label| label == source_label)
            .collect();
        self.sat_to_clique.extract_solution(&selected)
    }
}

#[reduction(
    transform = upper_bound {
        num_vertices = "(9 * num_clauses^2 + 3 * num_clauses + 6) / 2",
        num_arcs = "18 * num_clauses^2 + 2",
    }
)]
impl ReduceTo<AcyclicPartition<i64>> for KSatisfiability<K3> {
    type Result = Reduction3SATToAcyclicPartition;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let sat_to_clique = ReduceTo::<KClique<SimpleGraph>>::reduce_to(self)?;
        let clique = sat_to_clique.target_problem();
        let n = clique.num_vertices();
        let edges = clique.graph().edges();
        let e = edges.len();
        let (target_n, arc_count, capacity, magnitude, weight_bound, cost_bound) =
            incidence_parameters(n, e, clique.k())?;
        let items = target_n - 2;
        let source_vertex = items;
        let sink_vertex = items + 1;
        let mut degree = vec![0i64; n];
        for &(u, v) in &edges {
            // Degree <= e, whose doubled value fits the checked magnitude.
            degree[u] += 1;
            degree[v] += 1;
        }
        let mut arcs = Vec::with_capacity(arc_count);
        let mut arc_costs = Vec::with_capacity(arc_count);
        let profits = degree
            .iter()
            .map(|&d| d + 1)
            .chain(std::iter::repeat_n(0, e));
        for (item, profit) in profits.enumerate() {
            arcs.push((source_vertex, item));
            arc_costs.push(magnitude);
            arcs.push((item, sink_vertex));
            arc_costs.push(magnitude - profit);
        }
        for (j, &(u, v)) in edges.iter().enumerate() {
            arcs.push((u, n + j));
            arc_costs.push(1);
            arcs.push((v, n + j));
            arc_costs.push(1);
        }
        let mut weights = vec![1; items];
        weights.push(weight_bound - capacity);
        weights.push(
            weight_bound
                - <Self as ReduceTo<AcyclicPartition<i64>>>::exact_i64(
                    items,
                    "representing incidence item count",
                )?,
        );
        let target = AcyclicPartition::new(
            DirectedGraph::new(target_n, arcs),
            weights,
            arc_costs,
            weight_bound,
            cost_bound,
        );
        Ok(Reduction3SATToAcyclicPartition {
            sat_to_clique,
            target,
            source_vertex,
        })
    }
}

/// All values use polynomial-size counts; check them before graph allocation.
fn incidence_parameters(
    n: usize,
    e: usize,
    k: usize,
) -> Result<(usize, usize, i64, i64, i64, i64), crate::rules::ReductionError> {
    type Source = KSatisfiability<K3>;
    type Target = AcyclicPartition<i64>;
    let overflow = || {
        crate::rules::ReductionError::integer_overflow::<Source, Target>(
            "encoding clique incidence partition parameters",
        )
    };
    let items = n.checked_add(e).ok_or_else(overflow)?;
    let target_n = items.checked_add(2).ok_or_else(overflow)?;
    let arcs = items
        .checked_add(e)
        .and_then(|x| x.checked_mul(2))
        .ok_or_else(overflow)?;
    let l = i64::try_from(items).map_err(|_| overflow())?;
    let k = i64::try_from(k).map_err(|_| overflow())?;
    let next = k.checked_add(1).ok_or_else(overflow)?;
    let capacity = if k % 2 == 0 {
        (k / 2).checked_mul(next)
    } else {
        k.checked_mul(next / 2)
    }
    .ok_or_else(overflow)?;
    let magnitude = l
        .checked_mul(2)
        .and_then(|x| x.checked_add(1))
        .ok_or_else(overflow)?;
    let bound = l
        .checked_add(capacity)
        .and_then(|x| x.checked_mul(2))
        .and_then(|x| x.checked_add(1))
        .ok_or_else(overflow)?;
    let square = k.checked_mul(k).ok_or_else(overflow)?;
    let cost_bound = magnitude
        .checked_mul(l)
        .and_then(|x| x.checked_sub(square))
        .ok_or_else(overflow)?;
    Ok((target_n, arcs, capacity, magnitude, bound, cost_bound))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_acyclicpartition",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, AcyclicPartition<i64>>(
                KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]),
                SolutionPair {
                    source_config: serde_json::json!(vec![true]),
                    target_config: serde_json::json!(vec![0, 1, 1, 0, 0, 1, 1, 0, 1]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_acyclicpartition.rs"]
mod tests;
