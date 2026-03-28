//! Reduction from HamiltonianCircuit to StrongConnectivityAugmentation.
//!
//! Based on the Eswaran & Tarjan (1976) construction: start with an arc-less
//! digraph on n vertices. For each ordered pair (u, v), create a candidate arc
//! with weight 1 if {u, v} is an edge in the source graph, and weight 2
//! otherwise. Set the budget B = n. A Hamiltonian circuit exists in the source
//! graph if and only if a cost-n strong connectivity augmentation exists using
//! only weight-1 arcs.

use crate::models::graph::{HamiltonianCircuit, StrongConnectivityAugmentation};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{DirectedGraph, Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to StrongConnectivityAugmentation.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToStrongConnectivityAugmentation {
    target: StrongConnectivityAugmentation<i32>,
    n: usize,
}

impl ReductionResult for ReductionHamiltonianCircuitToStrongConnectivityAugmentation {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = StrongConnectivityAugmentation<i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.n;
        if n == 0 {
            return vec![];
        }

        // Build directed adjacency from selected arcs.
        let candidate_arcs = self.target.candidate_arcs();
        let mut successors = vec![Vec::new(); n];
        for (idx, &selected) in target_solution.iter().enumerate() {
            if selected == 1 {
                let (u, v, _) = candidate_arcs[idx];
                successors[u].push(v);
            }
        }

        // Walk the directed cycle starting from vertex 0.
        let mut order = Vec::with_capacity(n);
        let mut current = 0;
        let mut visited = vec![false; n];
        for _ in 0..n {
            if visited[current] {
                // Not a valid Hamiltonian cycle; return fallback.
                return vec![0; n];
            }
            visited[current] = true;
            order.push(current);
            if successors[current].len() != 1 {
                return vec![0; n];
            }
            current = successors[current][0];
        }

        order
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_arcs = "0",
        num_potential_arcs = "num_vertices * (num_vertices - 1)",
    }
)]
impl ReduceTo<StrongConnectivityAugmentation<i32>> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToStrongConnectivityAugmentation;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let graph = DirectedGraph::empty(n);

        // Generate all ordered pairs (u, v) with u != v as candidate arcs.
        let mut candidate_arcs = Vec::with_capacity(n * (n - 1));
        for u in 0..n {
            for v in 0..n {
                if u != v {
                    let weight = if self.graph().has_edge(u, v) { 1 } else { 2 };
                    candidate_arcs.push((u, v, weight));
                }
            }
        }

        let bound = n as i32;
        let target = StrongConnectivityAugmentation::new(graph, candidate_arcs, bound);

        ReductionHamiltonianCircuitToStrongConnectivityAugmentation { target, n }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_strongconnectivityaugmentation",
        build: || {
            // 4-cycle: 0-1-2-3-0
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            let reduction = ReduceTo::<StrongConnectivityAugmentation<i32>>::reduce_to(&source);
            let target = reduction.target_problem();

            // The HC permutation [0, 1, 2, 3] corresponds to the directed cycle
            // 0->1->2->3->0. We need to find the indices of these arcs in the
            // candidate list. Candidate arcs are ordered: for each u in 0..n,
            // for each v in 0..n where u!=v, so arc (u,v) is at index
            // u*(n-1) + (if v > u then v-1 else v).
            let n = 4;
            let mut target_config = vec![0usize; n * (n - 1)];
            let cycle_arcs = [(0, 1), (1, 2), (2, 3), (3, 0)];
            for (u, v) in cycle_arcs {
                let idx = u * (n - 1) + if v > u { v - 1 } else { v };
                target_config[idx] = 1;
            }

            // Verify the target config is valid
            assert!(
                target.is_valid_solution(&target_config),
                "canonical target config must be a valid SCA solution"
            );

            crate::example_db::specs::assemble_rule_example(
                &source,
                target,
                vec![SolutionPair {
                    source_config: vec![0, 1, 2, 3],
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_strongconnectivityaugmentation.rs"]
mod tests;
