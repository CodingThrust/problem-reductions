//! Eulerian Path problem implementation.
//!
//! Given a finite directed multigraph `D = (V, A)` with loops and parallel arcs
//! allowed, determine whether there exists a directed trail that uses every
//! arc in `A` exactly once.
//!
//! Conventions:
//! - repeated arc occurrences are distinguished,
//! - loops are allowed,
//! - isolated vertices are allowed and ignored,
//! - a closed trail is accepted,
//! - the empty-arc instance is accepted, witnessed by the empty trail.
//!
//! The problem is a satisfaction (witness) problem and is solvable in linear
//! time `O(num_vertices + num_arcs)` by the classical degree-balance plus
//! Hierholzer construction (Bang-Jensen & Gutin 2009; Ebert 1988).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "EulerianPath",
        display_name: "Eulerian Path",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Does the directed multigraph admit a directed trail using every arc exactly once?",
        fields: &[
            FieldInfo {
                name: "graph",
                type_name: "DirectedGraph",
                description: "The directed multigraph D=(V,A); parallel arcs and loops allowed",
            },
        ],
    }
}

/// The Eulerian Path problem on directed multigraphs.
///
/// A configuration is an arc-ordering `pi`: position `t` carries the index of
/// the arc occurrence used as the `t`-th arc of the trail.
///
/// `dims() = vec![m; m]` where `m = num_arcs()`. A configuration is feasible
/// when:
/// 1. it is a permutation of `0..m` (all values distinct, each in range), and
/// 2. for every consecutive pair `(pi[t], pi[t+1])`, the target vertex of arc
///    `pi[t]` equals the source vertex of arc `pi[t+1]`.
///
/// When `m = 0`, `dims = vec![]` and the empty configuration is the unique
/// (trivially satisfying) witness.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::EulerianPath;
/// use problemreductions::topology::DirectedGraph;
/// use problemreductions::{BruteForce, Problem};
///
/// // V = {0,1,2}; A = [(0,1), (0,1), (1,2), (2,0)] (parallel arc (0,1)).
/// let graph = DirectedGraph::new(3, vec![(0, 1), (0, 1), (1, 2), (2, 0)]);
/// let problem = EulerianPath::new(graph);
///
/// // Witness: ordering [a_0, a_2, a_3, a_1] = (0->1)->(1->2)->(2->0)->(0->1)
/// // traces trail 0->1->2->0->1.
/// let witness = BruteForce::new().solve(&problem).unwrap();
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EulerianPath {
    graph: DirectedGraph,
}

impl EulerianPath {
    /// Create a new Eulerian Path instance from a directed multigraph.
    pub fn new(graph: DirectedGraph) -> Self {
        Self { graph }
    }

    /// Borrow the underlying directed multigraph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Number of arc occurrences in the underlying multigraph (`m = |A|`).
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Check whether an arc ordering forms a valid directed Eulerian trail.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_eulerian_trail(&self.graph, config)
    }
}

impl Problem for EulerianPath {
    const NAME: &'static str = "EulerianPath";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![("num_arcs", num_arcs), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        let m = self.graph.num_arcs();
        if config.len() != m {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "arc ordering length does not match the graph".into(),
            ));
        }
        if config.iter().any(|&arc| arc >= m) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "arc ordering contains an out-of-range arc".into(),
            ));
        }
        Ok(crate::types::Or(is_valid_eulerian_trail(
            &self.graph,
            config,
        )))
    }
}

impl crate::solvers::BruteForceProblem for EulerianPath {
    fn dimensions(&self) -> Vec<usize> {
        let m = self.graph.num_arcs();
        vec![m; m]
    }
}

/// Decide whether `config` represents a valid directed Eulerian trail on
/// `graph`.
///
/// A configuration is valid when it is a permutation of `0..m` and each
/// consecutive pair of chosen arcs shares an endpoint (head of the previous
/// arc equals tail of the next arc). The empty configuration on the empty
/// multigraph (`m == 0`) is accepted.
fn is_valid_eulerian_trail(graph: &DirectedGraph, config: &[usize]) -> bool {
    let m = graph.num_arcs();
    if config.len() != m {
        return false;
    }
    if m == 0 {
        return true;
    }

    // Permutation check: all values in 0..m and distinct.
    let mut seen = vec![false; m];
    for &idx in config {
        if idx >= m || seen[idx] {
            return false;
        }
        seen[idx] = true;
    }

    // Consecutive-arc connectivity: head(arcs[pi[t]]) == tail(arcs[pi[t+1]]).
    let arcs = graph.arcs();
    for window in config.windows(2) {
        let (_prev_src, prev_tgt) = arcs[window[0]];
        let (next_src, _next_tgt) = arcs[window[1]];
        if prev_tgt != next_src {
            return false;
        }
    }
    true
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Canonical YES instance from the issue: V = {0,1,2},
    // A = [(0,1), (0,1), (1,2), (2,0)] (parallel arcs a_0, a_1 between 0 and 1).
    // Witness ordering (a_0, a_2, a_3, a_1) traces 0->1->2->0->1.
    let graph = DirectedGraph::new(3, vec![(0, 1), (0, 1), (1, 2), (2, 0)]);
    let optimal_config = vec![0usize, 2, 3, 1];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "eulerian_path",
        instance: Box::new(EulerianPath::new(graph)),
        optimal_config: serde_json::to_value(optimal_config)
            .expect("solution serialization must succeed"),
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default EulerianPath => "num_vertices + num_arcs",
}

crate::register_brute_force! {
    EulerianPath,
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/eulerian_path.rs"]
mod tests;
