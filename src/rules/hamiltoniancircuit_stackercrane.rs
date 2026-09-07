//! Reduction from HamiltonianCircuit to StackerCrane.
//!
//! Vertex splitting into the mixed routing formulation of StackerCrane.
//! Each vertex v_i is split into v_i^in (= 2i) and v_i^out (= 2i+1). A mandatory
//! directed arc (v_i^in → v_i^out) of length 1 is added for each vertex. For each
//! undirected edge {v_i, v_j} in the source graph, two undirected connector edges
//! {v_i^out, v_j^in} and {v_j^out, v_i^in} of length 1 are added.
//!
//! The source graph has a Hamiltonian circuit iff the optimal Stacker Crane tour
//! cost equals 2n and n >= 3 (n service arcs and n unit-cost connectors).
//! Using connector length 1 (rather than 0) ensures that multi-hop connector
//! paths cost strictly more than single-hop ones. Only permutations attaining
//! this lower bound certify a Hamiltonian circuit.

use crate::models::graph::HamiltonianCircuit;
use crate::models::misc::StackerCrane;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to StackerCrane.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToStackerCrane {
    target: StackerCrane,
}

impl ReductionResult for ReductionHamiltonianCircuitToStackerCrane {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = StackerCrane;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !crate::rules::AggregateReductionResult::extract_value(self, value).0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target tour does not certify a Hamiltonian circuit",
            ));
        }
        // Service arc i corresponds to source vertex i.
        Ok(target_solution.to_vec())
    }
}

impl crate::rules::AggregateReductionResult for ReductionHamiltonianCircuitToStackerCrane {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = StackerCrane;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, value: crate::types::Min<i64>) -> crate::types::Or {
        crate::types::Or(
            self.target.num_arcs() >= 3
                && value
                    .0
                    .is_some_and(|cost| usize::try_from(cost) == Ok(self.target.num_vertices())),
        )
    }
}

#[reduction(
    aggregate = custom,
    transform = exact {
        num_vertices = "2 * num_vertices",
        num_arcs = "num_vertices",
        num_edges = "2 * num_edges",
    }
)]
impl ReduceTo<StackerCrane> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToStackerCrane;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();

        // Each vertex i becomes two vertices: 2i (in) and 2i+1 (out).
        let (target_num_vertices, target_num_edges) = split_graph_dimensions(n, self.num_edges())?;

        // One mandatory arc per original vertex: (2i, 2i+1) with length 1.
        let arcs: Vec<(usize, usize)> = (0..n).map(|i| (2 * i, 2 * i + 1)).collect();
        let arc_lengths: Vec<i64> = vec![1; n];

        // For each original edge {u, v}, add two undirected connector edges:
        //   {u^out, v^in} = {2u+1, 2v}  with length 1
        //   {v^out, u^in} = {2v+1, 2u}  with length 1
        // Using length 1 (not 0) prevents multi-hop zero-cost shortcuts that
        // would create optimal SC permutations not corresponding to valid HCs.
        let mut edges = Vec::with_capacity(target_num_edges);
        let mut edge_lengths = Vec::with_capacity(target_num_edges);
        for (u, v) in self.graph().edges() {
            edges.push((2 * u + 1, 2 * v));
            edge_lengths.push(1);
            edges.push((2 * v + 1, 2 * u));
            edge_lengths.push(1);
        }

        let target =
            StackerCrane::try_new(target_num_vertices, arcs, edges, arc_lengths, edge_lengths)
                .map_err(<Self as ReduceTo<StackerCrane>>::target_construction)?;

        Ok(ReductionHamiltonianCircuitToStackerCrane { target })
    }
}

/// Check split indices and every finite service-order cost before allocation.
fn split_graph_dimensions(
    n: usize,
    m: usize,
) -> Result<(usize, usize), crate::rules::ReductionError> {
    type Source = HamiltonianCircuit<SimpleGraph>;
    let overflow = || {
        crate::rules::ReductionError::integer_overflow::<Source, StackerCrane>(
            "encoding split graph dimensions and route costs",
        )
    };
    let vertices = n.checked_mul(2).ok_or_else(overflow)?;
    let edges = m.checked_mul(2).ok_or_else(overflow)?;
    // A shortest connector is simple and has at most 2n-1 unit steps.
    // The n services therefore cost at most n * (1 + (2n-1)).
    let cost_bound = n.checked_mul(vertices).ok_or_else(overflow)?;
    <Source as ReduceTo<StackerCrane>>::exact_i64(cost_bound, "bounding split graph route costs")?;
    Ok((vertices, edges))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_stackercrane",
        build: || {
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            crate::example_db::specs::rule_example_with_witness::<_, StackerCrane>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![0, 1, 2, 3]),
                    target_config: serde_json::json!(vec![0, 1, 2, 3]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_stackercrane.rs"]
mod tests;
