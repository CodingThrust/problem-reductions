//! Minimum Feedback Arc Set problem implementation.
//!
//! The Feedback Arc Set problem asks for a minimum-size subset of arcs
//! whose removal makes a directed graph acyclic (a DAG).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumFeedbackArcSet",
        module_path: module_path!(),
        description: "Find minimum feedback arc set in a directed graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The directed graph G=(V,A)" },
        ],
    }
}

/// The Minimum Feedback Arc Set problem.
///
/// Given a directed graph G = (V, A), find a minimum-size subset A' ⊆ A
/// such that removing A' makes G acyclic (i.e., G - A' is a DAG).
///
/// # Variables
///
/// One binary variable per arc: x_a = 1 means arc a is in the feedback arc set (removed).
/// The configuration space has dimension m = |A|.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumFeedbackArcSet;
/// use problemreductions::topology::DirectedGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Directed cycle: 0->1->2->0
/// let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
/// let problem = MinimumFeedbackArcSet::new(graph);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem).unwrap();
///
/// // Minimum FAS has size 1 (remove any single arc to break the cycle)
/// assert_eq!(solution.iter().sum::<usize>(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumFeedbackArcSet {
    /// The directed graph.
    graph: DirectedGraph,
}

impl MinimumFeedbackArcSet {
    /// Create a Minimum Feedback Arc Set problem from a directed graph.
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

    /// Check if a configuration is a valid feedback arc set.
    ///
    /// A configuration is valid if removing the selected arcs makes the graph acyclic.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_fas(&self.graph, config)
    }
}

impl Problem for MinimumFeedbackArcSet {
    const NAME: &'static str = "MinimumFeedbackArcSet";
    type Metric = SolutionSize<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_arcs()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        if !is_valid_fas(&self.graph, config) {
            return SolutionSize::Invalid;
        }
        let count = config.iter().filter(|&&x| x != 0).count() as i32;
        SolutionSize::Valid(count)
    }
}

impl OptimizationProblem for MinimumFeedbackArcSet {
    type Value = i32;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a configuration forms a valid feedback arc set.
///
/// config[i] = 1 means arc i is selected for removal.
/// The remaining arcs must form a DAG.
fn is_valid_fas(graph: &DirectedGraph, config: &[usize]) -> bool {
    let num_arcs = graph.num_arcs();
    if config.len() != num_arcs {
        return false;
    }
    // kept_arcs[i] = true means arc i is NOT removed (kept in the graph)
    let kept_arcs: Vec<bool> = config.iter().map(|&x| x == 0).collect();
    graph.is_acyclic_subgraph(&kept_arcs)
}

crate::declare_variants! {
    MinimumFeedbackArcSet => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_feedback_arc_set.rs"]
mod tests;
