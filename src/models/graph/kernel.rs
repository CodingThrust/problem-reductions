//! Kernel problem implementation.
//!
//! The Kernel problem asks whether a directed graph contains a kernel, i.e.,
//! a subset of vertices that is both independent (no arc between any two
//! selected vertices) and absorbing (every unselected vertex has an arc to
//! some selected vertex).

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Kernel",
        display_name: "Kernel",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "DirectedGraph", &["DirectedGraph"]),
        ],
        module_path: module_path!(),
        description: "Does the directed graph contain a kernel (independent and absorbing vertex subset)?",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The directed graph G=(V,A)" },
        ],
    }
}

/// The Kernel problem.
///
/// Given a directed graph G = (V, A), find a kernel V' ⊆ V such that:
/// 1. **Independence:** no two vertices in V' are joined by an arc (neither
///    (u,v) nor (v,u) is in A for any u,v ∈ V').
/// 2. **Absorption:** every vertex u ∉ V' has an arc to some vertex v ∈ V'
///    (i.e., (u,v) ∈ A).
///
/// # Representation
///
/// A configuration is a binary vector of length |V|, where `config[v] = 1`
/// means vertex v is selected into V'.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::Kernel;
/// use problemreductions::topology::DirectedGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let graph = DirectedGraph::new(5, vec![
///     (0,1),(0,2),(1,3),(2,3),(3,4),(4,0),(4,1),
/// ]);
/// let problem = Kernel::new(graph);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kernel {
    graph: DirectedGraph,
}

impl Kernel {
    /// Create a new Kernel problem from a directed graph.
    pub fn new(graph: DirectedGraph) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the number of vertices in the directed graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs in the directed graph.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }
}

impl Problem for Kernel {
    const NAME: &'static str = "Kernel";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        let n = self.graph.num_vertices();

        // Collect selected vertices
        let selected: Vec<bool> = config.iter().map(|&c| c == 1).collect();

        // Independence: no arc between any two selected vertices
        for u in 0..n {
            if !selected[u] {
                continue;
            }
            // Check that no successor of u is also selected
            for &v in &self.graph.successors(u) {
                if selected[v] {
                    return crate::types::Or(false);
                }
            }
        }

        // Absorption: every unselected vertex must have an arc to some selected vertex
        for u in 0..n {
            if selected[u] {
                continue;
            }
            let has_arc_to_selected = self.graph.successors(u).iter().any(|&v| selected[v]);
            if !has_arc_to_selected {
                return crate::types::Or(false);
            }
        }

        crate::types::Or(true)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 5 vertices, arcs: (0,1),(0,2),(1,3),(2,3),(3,4),(4,0),(4,1)
    // Kernel: V' = {0, 3} → config [1,0,0,1,0]
    let graph = DirectedGraph::new(
        5,
        vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (4, 0), (4, 1)],
    );
    let optimal_config = vec![1, 0, 0, 1, 0];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "kernel",
        instance: Box::new(Kernel::new(graph)),
        optimal_config,
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default Kernel => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kernel.rs"]
mod tests;
