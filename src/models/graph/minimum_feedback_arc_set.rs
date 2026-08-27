//! Minimum Feedback Arc Set problem implementation.
//!
//! The Feedback Arc Set problem asks for a minimum-weight subset of arcs
//! whose removal makes a directed graph acyclic (a DAG).

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumFeedbackArcSet",
        display_name: "Minimum Feedback Arc Set",
        aliases: &["FAS"],
        dimensions: &[
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find minimum weight feedback arc set in a directed graph",
        fields: MinimumFeedbackArcSetCreateSpec::FIELDS,
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
/// use problemreductions::{Problem, BruteForce};
///
/// // Directed cycle: 0->1->2->0
/// let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
/// let problem = MinimumFeedbackArcSet::new(graph, vec![1i64; 3]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap().unwrap();
///
/// // Minimum FAS has size 1 (remove any single arc to break the cycle)
/// assert_eq!(solution.iter().filter(|&&selected| selected).count(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumFeedbackArcSet<W> {
    /// The directed graph.
    graph: DirectedGraph,
    /// Weights for each arc.
    weights: Vec<W>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MinimumFeedbackArcSetCreateSpec {
    /// The directed graph.
    graph: DirectedGraph,
    /// Arc weights; defaults to one per arc.
    weights: Option<Vec<i64>>,
}
impl TryFrom<MinimumFeedbackArcSetCreateSpec> for MinimumFeedbackArcSet<i64> {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MinimumFeedbackArcSetCreateSpec) -> Result<Self, Self::Error> {
        let count = spec.graph.num_arcs();
        let weights = spec.weights.unwrap_or_else(|| vec![1; count]);
        if weights.len() != count {
            return Err(format!("weights has {} entries, expected {count}", weights.len()).into());
        }
        Ok(Self::new(spec.graph, weights))
    }
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
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
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
    type Solution = Vec<bool>;
    type Value = Min<W::Sum>;

    crate::problem_size![("num_arcs", num_arcs), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_arcs() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "arc-selection length does not match the graph".into(),
            ));
        }
        Ok({
            if !is_valid_fas(&self.graph, config) {
                return Ok(Min(None));
            }
            let mut total = W::Sum::zero();
            for (i, &selected) in config.iter().enumerate() {
                if selected {
                    total = W::checked_add_to_sum(
                        total,
                        self.weights[i].to_sum(),
                        "summing selected feedback-arc weights",
                    )?;
                }
            }
            Min(Some(total))
        })
    }
}

impl<W> crate::solvers::BruteForceProblem for MinimumFeedbackArcSet<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_arcs()]
    }
}

/// Check if a configuration forms a valid feedback arc set.
///
/// config[i] = 1 means arc i is selected for removal.
/// The remaining arcs must form a DAG.
fn is_valid_fas(graph: &DirectedGraph, config: &[bool]) -> bool {
    let num_arcs = graph.num_arcs();
    if config.len() != num_arcs {
        return false;
    }
    // kept_arcs[i] = true means arc i is NOT removed (kept in the graph)
    let kept_arcs: Vec<bool> = config.iter().map(|&removed| !removed).collect();
    graph.is_acyclic_subgraph(&kept_arcs)
}

crate::declare_variants! {
    default MinimumFeedbackArcSet<i64> => "2^num_vertices" create MinimumFeedbackArcSetCreateSpec,
}

crate::register_brute_force! {
    MinimumFeedbackArcSet<i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use crate::topology::DirectedGraph;
    // 3-node cycle, unit weights; remove one arc to break cycle, cost = 1
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_feedback_arc_set",
        instance: Box::new(MinimumFeedbackArcSet::new(
            DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
            vec![1i64, 1, 1],
        )),
        optimal_config: serde_json::json!(vec![false, false, true]),
        optimal_value: serde_json::json!(1),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_feedback_arc_set.rs"]
mod tests;
