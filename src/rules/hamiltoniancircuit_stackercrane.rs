//! Reduction from HamiltonianCircuit to StackerCrane.
//!
//! Based on the vertex-splitting construction of Frederickson, Hecht & Kim (1978).
//! Each vertex v_i is split into v_i^in (= 2i) and v_i^out (= 2i+1). A mandatory
//! directed arc (v_i^in → v_i^out) of length 1 is added for each vertex. For each
//! undirected edge {v_i, v_j} in the source graph, two undirected connector edges
//! {v_i^out, v_j^in} and {v_j^out, v_i^in} of length 1 are added.
//!
//! The source graph has a Hamiltonian circuit iff the optimal Stacker Crane tour
//! cost equals 2n (n arcs of cost 1 plus n single-hop connectors of cost 1).
//! Using connector length 1 (rather than 0) ensures that multi-hop connector
//! paths cost strictly more than single-hop ones, so every optimal permutation
//! corresponds to a valid Hamiltonian circuit.

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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // The target config is a permutation of arc indices.
        // Arc i corresponds to original vertex i (arc from 2i to 2i+1).
        // The permutation order directly gives the Hamiltonian circuit vertex order.
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vertices = "2 * num_vertices",
        num_arcs = "num_vertices",
        num_edges = "2 * num_edges",
    }
)]
impl ReduceTo<StackerCrane> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToStackerCrane;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();

        // Each vertex i becomes two vertices: 2i (in) and 2i+1 (out).
        let target_num_vertices = 2 * n;

        // One mandatory arc per original vertex: (2i, 2i+1) with length 1.
        let arcs: Vec<(usize, usize)> = (0..n).map(|i| (2 * i, 2 * i + 1)).collect();
        let arc_lengths: Vec<i32> = vec![1; n];

        // For each original edge {u, v}, add two undirected connector edges:
        //   {u^out, v^in} = {2u+1, 2v}  with length 1
        //   {v^out, u^in} = {2v+1, 2u}  with length 1
        // Using length 1 (not 0) prevents multi-hop zero-cost shortcuts that
        // would create optimal SC permutations not corresponding to valid HCs.
        let mut edges = Vec::new();
        let mut edge_lengths = Vec::new();
        for (u, v) in self.graph().edges() {
            edges.push((2 * u + 1, 2 * v));
            edge_lengths.push(1);
            edges.push((2 * v + 1, 2 * u));
            edge_lengths.push(1);
        }

        let target = StackerCrane::new(target_num_vertices, arcs, edges, arc_lengths, edge_lengths);

        ReductionHamiltonianCircuitToStackerCrane { target }
    }
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
                    source_config: vec![0, 1, 2, 3],
                    target_config: vec![0, 1, 2, 3],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_stackercrane.rs"]
mod tests;
