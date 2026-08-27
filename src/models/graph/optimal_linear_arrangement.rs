//! Optimal Linear Arrangement problem implementation.
//!
//! The Optimal Linear Arrangement problem asks for a one-to-one function
//! f: V -> {0, 1, ..., |V|-1} that minimizes the total edge length
//! sum_{{u,v} in E} |f(u) - f(v)|.

use crate::models::decision::Decision;
use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "OptimalLinearArrangement",
        display_name: "Optimal Linear Arrangement",
        aliases: &["OLA"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a vertex ordering on a line minimizing total edge length",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
        ],
    }
}

/// The Optimal Linear Arrangement problem.
///
/// Given an undirected graph G = (V, E), find a bijection f: V -> {0, 1, ..., |V|-1}
/// that minimizes the total edge length sum_{{u,v} in E} |f(u) - f(v)|.
///
/// This is the optimization (minimization) version of the problem.
///
/// # Representation
///
/// Each vertex is assigned a variable representing its position in the arrangement.
/// Variable i takes a value in {0, 1, ..., n-1}, and a valid configuration must be
/// a permutation (all positions are distinct). The objective is to minimize total
/// edge length.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::OptimalLinearArrangement;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Path graph: 0-1-2-3
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = OptimalLinearArrangement::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct OptimalLinearArrangement<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> OptimalLinearArrangement<G> {
    /// Create a new Optimal Linear Arrangement problem.
    ///
    /// # Arguments
    /// * `graph` - The undirected graph G = (V, E)
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration is a valid permutation.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.is_valid_permutation(config)
    }

    /// Check if a configuration forms a valid permutation of {0, ..., n-1}.
    fn is_valid_permutation(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        if config.len() != n {
            return false;
        }
        let mut seen = vec![false; n];
        for &pos in config {
            if pos >= n || seen[pos] {
                return false;
            }
            seen[pos] = true;
        }
        true
    }

    /// Compute the total edge length for a given arrangement.
    ///
    /// Returns `None` if the configuration is not a valid permutation.
    pub fn total_edge_length(
        &self,
        config: &[usize],
    ) -> Result<Option<i64>, crate::traits::EvaluationError> {
        if !self.is_valid_permutation(config) {
            return Ok(None);
        }
        let mut total = 0_i64;
        for (u, v) in self.graph.edges() {
            let fu = config[u];
            let fv = config[v];
            let length = i64::try_from(fu.abs_diff(fv)).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting linear-arrangement edge length to i64".to_string(),
                )
            })?;
            total = total.checked_add(length).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing linear-arrangement edge lengths".to_string(),
                )
            })?;
        }
        Ok(Some(total))
    }
}

impl<G> Problem for OptimalLinearArrangement<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "OptimalLinearArrangement";
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_size![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        let n = self.graph.num_vertices();
        if config.len() != n {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "vertex arrangement length does not match the graph".into(),
            ));
        }
        if config.iter().any(|&position| position >= n) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "vertex arrangement contains an out-of-range position".into(),
            ));
        }
        Ok({
            match self.total_edge_length(config)? {
                Some(cost) => Min(Some(cost)),
                None => Min(None),
            }
        })
    }
}

impl<G> crate::solvers::BruteForceProblem for OptimalLinearArrangement<G>
where
    G: Graph + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }
}

crate::impl_random_generate!(
    OptimalLinearArrangement<SimpleGraph>,
    crate::random::SimpleGraphRandomSpec,
    |spec| { Ok(OptimalLinearArrangement::new(spec.graph()?)) }
);

crate::declare_variants! {
    default OptimalLinearArrangement<SimpleGraph> => "2^num_vertices" random,
}

crate::register_brute_force! {
    OptimalLinearArrangement<SimpleGraph>,
}

impl<G> crate::models::decision::DecisionProblemMeta for OptimalLinearArrangement<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const DECISION_NAME: &'static str = "DecisionOptimalLinearArrangement";
}

impl Decision<OptimalLinearArrangement<SimpleGraph>> {
    /// Number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.inner().num_vertices()
    }

    /// Number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.inner().num_edges()
    }

    /// Decision bound (maximum allowed total edge length) as a nonnegative integer.
    pub fn k(&self) -> usize {
        usize::try_from(*self.bound()).expect("nonnegative decision bound must fit usize")
    }
}

crate::register_decision_variant!(
    OptimalLinearArrangement<SimpleGraph>,
    "DecisionOptimalLinearArrangement",
    "2^num_vertices",
    &["DOLA"],
    "Decision version: does a linear arrangement of total edge length <= bound exist?",
    category: crate::registry::ProblemCategory::Graph,
    dims: [
        VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
    ],
    fields: [
        FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
        FieldInfo { name: "bound", type_name: "i64", description: "Decision bound (maximum allowed total edge length)" },
    ],
    size_getters: [("num_vertices", num_vertices), ("num_edges", num_edges)],
    decode: |_, indices: Vec<usize>| indices
);

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use crate::topology::SimpleGraph;
    // 6 vertices, 7 edges (path + two long chords)
    // Optimal arrangement [0,1,2,3,4,5] gives cost 1+1+1+1+1+3+3 = 11
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "optimal_linear_arrangement",
        instance: Box::new(OptimalLinearArrangement::new(SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 3), (2, 5)],
        ))),
        optimal_config: serde_json::json!(vec![0, 1, 2, 3, 4, 5]),
        optimal_value: serde_json::json!(11),
    }]
}

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_model_example_specs(
) -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use crate::topology::SimpleGraph;
    // Path P_4 (0-1-2-3): optimal arrangement has cost 3; bound 3 is YES.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "decision_optimal_linear_arrangement_simplegraph",
        instance: Box::new(Decision::new(
            OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)])),
            3,
        )),
        optimal_config: serde_json::json!(vec![0, 1, 2, 3]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_rule_example_specs(
) -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decision_optimal_linear_arrangement_to_optimal_linear_arrangement",
        build: || {
            use crate::example_db::specs::assemble_rule_example;
            use crate::export::SolutionPair;
            use crate::rules::{AggregateReductionResult, ReduceToAggregate};
            use crate::topology::SimpleGraph;

            // Path P_4 (0-1-2-3): optimal arrangement has cost 3; bound 3 is YES.
            let source = Decision::new(
                OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)])),
                3,
            );
            let result = source
                .reduce_to_aggregate()
                .expect("reduction should succeed");
            let target = result.target_problem();
            let config = vec![0, 1, 2, 3];
            assemble_rule_example(
                &source,
                target,
                vec![SolutionPair {
                    source_config: serde_json::json!(config.clone()),
                    target_config: serde_json::json!(config),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/optimal_linear_arrangement.rs"]
mod tests;
