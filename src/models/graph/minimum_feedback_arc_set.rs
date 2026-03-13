//! Minimum Feedback Arc Set problem implementation.
//!
//! The Feedback Arc Set problem asks for a minimum-weight subset of arcs
//! whose removal makes a directed graph acyclic (a DAG).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumFeedbackArcSet",
        module_path: module_path!(),
        description: "Find minimum weight feedback arc set in a directed graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The directed graph G=(V,A)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Arc weights w: A -> R" },
        ],
    }
}

/// The Minimum Feedback Arc Set problem.
///
/// Given a directed graph G = (V, A) and weights w_a for each arc,
/// find a subset A' ⊆ A such that:
/// - Removing A' from G yields a directed acyclic graph (DAG)
/// - The total weight Σ_{a ∈ A'} w_a is minimized
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
/// let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem).unwrap();
///
/// // Minimum FAS has size 1 (remove any single arc to break the cycle)
/// assert_eq!(solution.iter().sum::<usize>(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumFeedbackArcSet<W> {
    /// The directed graph.
    graph: DirectedGraph,
    /// Weights for each arc.
    weights: Vec<W>,
}

impl<W: Clone + Default> MinimumFeedbackArcSet<W> {
    /// Create a Minimum Feedback Arc Set problem from a directed graph with given weights.
    pub fn new(graph: DirectedGraph, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_arcs(),
            "weights length must match graph num_arcs"
        );
        Self { graph, weights }
    }

    /// Get a reference to the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get a reference to the weights slice.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Set arc weights.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(
            weights.len(),
            self.graph.num_arcs(),
            "weights length must match graph num_arcs"
        );
        self.weights = weights;
    }

    /// Check if a configuration is a valid feedback arc set.
    ///
    /// A configuration is valid if removing the selected arcs makes the graph acyclic.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_fas(&self.graph, config)
    }
}

impl<W: WeightElement> MinimumFeedbackArcSet<W> {
    /// Check if the problem has non-unit weights.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
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

impl<W> Problem for MinimumFeedbackArcSet<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumFeedbackArcSet";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_arcs()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !is_valid_fas(&self.graph, config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::Sum::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected != 0 {
                total += self.weights[i].to_sum();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<W> OptimizationProblem for MinimumFeedbackArcSet<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

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
    MinimumFeedbackArcSet<i32> => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_feedback_arc_set.rs"]
mod tests;
