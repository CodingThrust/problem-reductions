//! Vertex Covering problem implementation.
//!
//! The Vertex Cover problem asks for a minimum weight subset of vertices
//! such that every edge has at least one endpoint in the subset.

use crate::models::decision::Decision;
use crate::registry::{CreateSpec, FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumVertexCover",
        display_name: "Minimum Vertex Cover",
        aliases: &["MVC"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64", "One"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find minimum weight vertex cover in a graph",
        fields: MinimumVertexCoverCreateSpec::<i64>::FIELDS,
    }
}

/// The Vertex Covering problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - Every edge has at least one endpoint in S (covering constraint)
/// - The total weight Σ_{v ∈ S} w_v is minimized
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumVertexCover;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Create a path graph 0-1-2
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = MinimumVertexCover::new(graph, vec![1; 3]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Minimum vertex cover is just vertex 1
/// assert!(solutions.contains(&vec![false, true, false]));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MinimumVertexCover<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

#[derive(Deserialize)]
struct MinimumVertexCoverData<G, W> {
    graph: G,
    weights: Vec<W>,
}

impl<'de, G, W> Deserialize<'de> for MinimumVertexCover<G, W>
where
    G: Graph + Deserialize<'de>,
    W: Clone + Default + Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = MinimumVertexCoverData::deserialize(deserializer)?;
        Self::try_new(data.graph, data.weights).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MinimumVertexCoverCreateSpec<W> {
    /// The underlying graph G=(V,E).
    graph: SimpleGraph,
    /// Vertex weights w: V -> R.
    weights: Option<Vec<W>>,
}

impl<W: WeightElement> TryFrom<MinimumVertexCoverCreateSpec<W>>
    for MinimumVertexCover<SimpleGraph, W>
{
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MinimumVertexCoverCreateSpec<W>) -> Result<Self, Self::Error> {
        let weights = spec
            .weights
            .unwrap_or_else(|| vec![W::unit(); spec.graph.num_vertices()]);
        if weights.len() != spec.graph.num_vertices() {
            return Err(format!(
                "weights has {} entries, expected {}",
                weights.len(),
                spec.graph.num_vertices()
            )
            .into());
        }
        Self::try_new(spec.graph, weights)
    }
}

impl<G: Graph, W: Clone + Default> MinimumVertexCover<G, W> {
    /// Create a Vertex Covering problem from a graph with given weights.
    pub fn new(graph: G, weights: Vec<W>) -> Self {
        Self::try_new(graph, weights).unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new(graph: G, weights: Vec<W>) -> Result<Self, crate::registry::ConstructionError> {
        if weights.len() != graph.num_vertices() {
            return Err("weights length must match graph num_vertices".into());
        }
        Ok(Self { graph, weights })
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get a reference to the weights.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool
    where
        W: WeightElement,
    {
        !W::IS_UNIT
    }

    /// Check if a configuration is a valid vertex cover.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
        is_vertex_cover_config(&self.graph, config)
    }
}

impl<G: Graph, W: WeightElement> MinimumVertexCover<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G, W> Problem for MinimumVertexCover<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumVertexCover";
    type Solution = Vec<bool>;
    type Value = Min<W::Sum>;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.graph.num_vertices() {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "vertex-selection length does not match the graph".into(),
                ));
            }
            if !is_vertex_cover_config(&self.graph, config) {
                return Ok(Min(None));
            }
            let mut total = W::Sum::zero();
            for (i, &selected) in config.iter().enumerate() {
                if selected {
                    total = W::checked_add_to_sum(
                        total,
                        self.weights[i].to_sum(),
                        "summing selected vertex-cover weights",
                    )?;
                }
            }
            Min(Some(total))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for MinimumVertexCover<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(self.graph.num_vertices())
    }

    fn dimension(&self, _variable: usize) -> Result<usize, crate::solvers::SolveError> {
        Ok(2usize)
    }
}

/// Check if a configuration forms a valid vertex cover.
pub(crate) fn is_vertex_cover_config<G: Graph>(graph: &G, config: &[bool]) -> bool {
    for (u, v) in graph.edges() {
        let u_covered = config.get(u).copied().unwrap_or(false);
        let v_covered = config.get(v).copied().unwrap_or(false);
        if !u_covered && !v_covered {
            return false;
        }
    }
    true
}

crate::impl_random_generate!(MinimumVertexCover<SimpleGraph, i64>, crate::random::SimpleGraphRandomSpec, |spec| {
    Ok(MinimumVertexCover::new(spec.graph()?, vec![1; spec.num_vertices]))
});
crate::impl_random_generate!(MinimumVertexCover<SimpleGraph, One>, crate::random::SimpleGraphRandomSpec, |spec| {
    Ok(MinimumVertexCover::new(spec.graph()?, vec![One; spec.num_vertices]))
});

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MinimumVertexCoverOneCreateSpec {
    /// The underlying graph.
    graph: SimpleGraph,
}

impl TryFrom<MinimumVertexCoverOneCreateSpec> for MinimumVertexCover<SimpleGraph, One> {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MinimumVertexCoverOneCreateSpec) -> Result<Self, Self::Error> {
        let weights = vec![One; spec.graph.num_vertices()];
        Self::try_new(spec.graph, weights)
    }
}

crate::declare_variants! {
    default MinimumVertexCover<SimpleGraph, i64> => "1.1996^num_vertices" create MinimumVertexCoverCreateSpec<i64> random,
    MinimumVertexCover<SimpleGraph, One> => "1.1996^num_vertices" create MinimumVertexCoverOneCreateSpec random,
}

crate::register_brute_force! {
    MinimumVertexCover<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MinimumVertexCover<SimpleGraph, One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

impl<G, W> crate::models::decision::DecisionProblemMeta for MinimumVertexCover<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
    W::Sum: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned,
{
    const DECISION_NAME: &'static str = "DecisionMinimumVertexCover";
}

impl<W> Decision<MinimumVertexCover<SimpleGraph, W>>
where
    W: WeightElement + crate::variant::VariantParam,
    W::Sum: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned,
{
    /// Number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.inner().num_vertices()
    }

    /// Number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.inner().num_edges()
    }
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct DecisionMinimumVertexCoverRandomSpec {
    /// Number of graph vertices.
    num_vertices: usize,
    /// Independent edge probability (default: 0.5).
    edge_prob: Option<f64>,
    /// Seed for reproducible generation.
    seed: Option<i64>,
    /// Maximum allowed cover cost.
    bound: i64,
}

crate::impl_random_generate!(
    Decision<MinimumVertexCover<SimpleGraph, i64>>,
    DecisionMinimumVertexCoverRandomSpec,
    |spec| {
        if spec.bound < 0 {
            return Err("bound must be nonnegative".to_string().into());
        }
        let graph = crate::random::SimpleGraphRandomSpec {
            num_vertices: spec.num_vertices,
            edge_prob: spec.edge_prob,
            seed: spec.seed,
        }
        .graph()?;
        Ok(Decision::new(
            MinimumVertexCover::new(graph, vec![1; spec.num_vertices]),
            spec.bound,
        ))
    }
);

crate::register_decision_variant!(
    MinimumVertexCover<SimpleGraph, i64>,
    "DecisionMinimumVertexCover",
    "1.1996^num_vertices",
    &["DMVC", "VC", "VertexCover"],
    "Decision version: does a vertex cover of cost <= bound exist?",
    category: crate::registry::ProblemCategory::Graph,
    dims: [
        VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        VariantDimension::new("weight", "i64", &["i64", "One"]),
    ],
    fields: [
        FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        FieldInfo { name: "bound", type_name: "W::Sum", description: "Decision bound (maximum allowed cover cost)" },
    ],
    additional: [MinimumVertexCover<SimpleGraph, One> => "1.1996^num_vertices"],
    decode: |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    random
);

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_vertex_cover_simplegraph",
        instance: Box::new(MinimumVertexCover::new(
            SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]),
            vec![1i64; 5],
        )),
        optimal_config: serde_json::json!(vec![true, false, false, true, true]),
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_model_example_specs(
) -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![
        crate::example_db::specs::ModelExampleSpec {
            id: "decision_minimum_vertex_cover_simplegraph",
            instance: Box::new(crate::models::decision::Decision::new(
                MinimumVertexCover::new(
                    SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]),
                    vec![1i64; 4],
                ),
                2,
            )),
            optimal_config: serde_json::json!(vec![true, false, true, false]),
            optimal_value: serde_json::json!(true),
        },
        crate::example_db::specs::ModelExampleSpec {
            id: "decision_minimum_vertex_cover_unit",
            instance: Box::new(Decision::new(
                MinimumVertexCover::new(SimpleGraph::path(3), vec![One; 3]),
                1,
            )),
            optimal_config: serde_json::json!([false, true, false]),
            optimal_value: serde_json::json!(true),
        },
    ]
}

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_rule_example_specs(
) -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::{rule_example_with_witness, RuleExampleSpec};
    use crate::export::SolutionPair;
    vec![
        RuleExampleSpec {
            id: "decision_minimum_vertex_cover_to_minimum_vertex_cover",
            build: || {
                let source = Decision::new(
                    MinimumVertexCover::new(
                        SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]),
                        vec![1i64; 4],
                    ),
                    2,
                );
                rule_example_with_witness::<_, MinimumVertexCover<SimpleGraph, i64>>(
                    source,
                    SolutionPair {
                        source_config: serde_json::json!([true, false, true, false]),
                        target_config: serde_json::json!([true, false, true, false]),
                    },
                )
            },
        },
        RuleExampleSpec {
            id: "decision_minimum_vertex_cover_unit_to_minimum_vertex_cover",
            build: || {
                let source = Decision::new(
                    MinimumVertexCover::new(SimpleGraph::path(3), vec![One; 3]),
                    1,
                );
                rule_example_with_witness::<_, MinimumVertexCover<SimpleGraph, One>>(
                    source,
                    SolutionPair {
                        source_config: serde_json::json!([false, true, false]),
                        target_config: serde_json::json!([false, true, false]),
                    },
                )
            },
        },
    ]
}

/// Check if a set of vertices forms a vertex cover.
///
/// # Arguments
/// * `graph` - The graph
/// * `selected` - Boolean slice indicating which vertices are selected
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
#[cfg(test)]
pub(crate) fn is_vertex_cover<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );
    for (u, v) in graph.edges() {
        if !selected[u] && !selected[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_vertex_cover.rs"]
mod tests;
