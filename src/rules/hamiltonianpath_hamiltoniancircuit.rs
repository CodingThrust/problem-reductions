//! Reduction from HamiltonianPath to HamiltonianCircuit.
//!
//! Add a vertex adjacent to every source vertex. Rotating a target circuit
//! around that vertex and deleting it recovers a source path. Empty and
//! singleton sources instead produce a triangle to respect circuit semantics.

use crate::models::graph::{HamiltonianCircuit, HamiltonianPath};
use crate::rules::{ReduceTo, ReductionError, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Stores the target circuit problem and the original ordering length.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianPathToHamiltonianCircuit {
    target: HamiltonianCircuit<SimpleGraph>,
    num_original_vertices: usize,
}

impl ReductionResult for ReductionHamiltonianPathToHamiltonianCircuit {
    type Source = HamiltonianPath<SimpleGraph>;
    type Target = HamiltonianCircuit<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &Vec<usize>,
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0,
            "target witness is not satisfying",
        )?;

        let n = self.num_original_vertices;
        if n < 2 {
            return Ok((0..n).collect());
        }
        let pivot = target_solution
            .iter()
            .position(|&vertex| vertex == n)
            .expect("validated circuit contains the added vertex");
        Ok(target_solution[pivot + 1..]
            .iter()
            .chain(&target_solution[..pivot])
            .copied()
            .collect())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionHamiltonianPathToHamiltonianCircuit {}

// The small-order branch has three vertices and edges; otherwise the exact
// counts are n+1 and m+n, since all spokes are distinct from old edges.
#[crate::reduction(transform = upper_bound {
    num_vertices = "num_vertices + 3",
    num_edges = "num_edges + num_vertices + 3",
})]
impl ReduceTo<HamiltonianCircuit<SimpleGraph>> for HamiltonianPath<SimpleGraph> {
    type Result = ReductionHamiltonianPathToHamiltonianCircuit;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let n = self.num_vertices();
        let (num_vertices, num_edges) = checked_target_sizes(n, self.num_edges())?;
        let edges = if n < 2 {
            vec![(0, 1), (0, 2), (1, 2)]
        } else {
            let mut edges = Vec::with_capacity(num_edges);
            edges.extend(self.graph().edges());
            edges.extend((0..n).map(|v| (v, n)));
            edges
        };
        let graph = SimpleGraph::try_new(num_vertices, edges)
            .map_err(<Self as ReduceTo<HamiltonianCircuit<SimpleGraph>>>::target_construction)?;
        Ok(ReductionHamiltonianPathToHamiltonianCircuit {
            target: HamiltonianCircuit::new(graph),
            num_original_vertices: n,
        })
    }
}

fn checked_target_sizes(n: usize, m: usize) -> Result<(usize, usize), ReductionError> {
    if n < 2 {
        return Ok((3, 3));
    }
    let overflow = || {
        ReductionError::integer_overflow::<
            HamiltonianPath<SimpleGraph>,
            HamiltonianCircuit<SimpleGraph>,
        >("computing target graph sizes")
    };
    let vertices = n.checked_add(1).ok_or_else(overflow)?;
    let edges = m.checked_add(n).ok_or_else(overflow)?;
    // SimpleGraph uses petgraph's default index domain for both nodes and edges.
    petgraph::graph::DefaultIx::try_from(vertices).map_err(|_| overflow())?;
    petgraph::graph::DefaultIx::try_from(edges).map_err(|_| overflow())?;
    Ok((vertices, edges))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpath_to_hamiltoniancircuit",
        build: || {
            let source = HamiltonianPath::new(SimpleGraph::new(
                5,
                vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 2), (1, 3)],
            ));
            crate::example_db::specs::rule_example_with_witness::<_, HamiltonianCircuit<SimpleGraph>>(
                source,
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 1, 2, 3, 4]),
                    target_config: serde_json::json!([5, 0, 1, 2, 3, 4]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpath_hamiltoniancircuit.rs"]
mod tests;
