//! Independent Set problem implementation.
//!
//! The Independent Set problem asks for a maximum weight subset of vertices
//! such that no two vertices in the subset are adjacent.

use crate::registry::{
    ConstructionError, CreateSpec, FieldInfo, ProblemSchemaEntry, VariantDimension,
};
use crate::topology::{Graph, KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph};
use crate::traits::Problem;
use crate::types::{Max, One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumIndependentSet",
        display_name: "Maximum Independent Set",
        aliases: &["MIS"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph", "KingsSubgraph", "TriangularSubgraph", "UnitDiskGraph"]),
            VariantDimension::new("weight", "One", &["One", "i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find maximum weight independent set in a graph",
        fields: MaximumIndependentSetSimpleOneCreateSpec::FIELDS,
    }
}

/// The Independent Set problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - No two vertices in S are adjacent (independent set constraint)
/// - The total weight Σ_{v ∈ S} w_v is maximized
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `KingsSubgraph`, `UnitDiskGraph`)
/// * `W` - The weight type (e.g., `i64`, `f64`, `One`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumIndependentSet;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Create a triangle graph (3 vertices, 3 edges)
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = MaximumIndependentSet::new(graph, vec![1; 3]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Maximum independent set in a triangle has size 1
/// assert!(solutions.iter().all(|s| s.iter().filter(|&&selected| selected).count() == 1));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumIndependentSet<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

macro_rules! simple_mis_spec {
    ($name:ident,$weight:ty,$one:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            #[create(codec = "edge-list")]
            graph: Vec<(usize, usize)>,
            num_vertices: Option<usize>,
            #[create(codec = "comma-separated")]
            weights: Option<Vec<$weight>>,
        }
        impl TryFrom<$name> for MaximumIndependentSet<SimpleGraph, $weight> {
            type Error = crate::registry::ConstructionError;
            fn try_from(spec: $name) -> Result<Self, crate::registry::ConstructionError> {
                if spec.graph.is_empty() && spec.num_vertices.is_none() {
                    return Err("num_vertices is required for an empty graph".into());
                }
                for &(u, v) in &spec.graph {
                    if u == v {
                        return Err("self-loops are not allowed".into());
                    }
                }
                let inferred = spec
                    .graph
                    .iter()
                    .flat_map(|&(u, v)| [u, v])
                    .max()
                    .map(|v| v.checked_add(1).ok_or("vertex count overflows usize"))
                    .transpose()?
                    .unwrap_or(0);
                let count = spec.num_vertices.unwrap_or(inferred);
                if count < inferred {
                    return Err("num_vertices is too small".into());
                }
                let weights = spec.weights.unwrap_or_else(|| vec![$one; count]);
                if weights.len() != count {
                    return Err("weights length must match num_vertices".into());
                }
                Ok(Self {
                    graph: SimpleGraph::new(count, spec.graph),
                    weights,
                })
            }
        }
    };
}
simple_mis_spec!(MaximumIndependentSetSimpleOneCreateSpec, One, One);
simple_mis_spec!(MaximumIndependentSetSimpleI64CreateSpec, i64, 1_i64);

macro_rules! grid_mis_spec {
    ($name:ident,$graph:ty,$weight:ty,$one:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            positions: Vec<(i64, i64)>,
            #[create(codec = "comma-separated")]
            weights: Option<Vec<$weight>>,
        }
        impl TryFrom<$name> for MaximumIndependentSet<$graph, $weight> {
            type Error = crate::registry::ConstructionError;
            fn try_from(spec: $name) -> Result<Self, crate::registry::ConstructionError> {
                let weights = spec
                    .weights
                    .unwrap_or_else(|| vec![$one; spec.positions.len()]);
                if weights.len() != spec.positions.len() {
                    return Err("weights length must match positions length".into());
                }
                Ok(Self {
                    graph: <$graph>::new(spec.positions),
                    weights,
                })
            }
        }
    };
}
grid_mis_spec!(
    MaximumIndependentSetKingsOneCreateSpec,
    KingsSubgraph,
    One,
    One
);
grid_mis_spec!(
    MaximumIndependentSetKingsI64CreateSpec,
    KingsSubgraph,
    i64,
    1_i64
);
grid_mis_spec!(
    MaximumIndependentSetTriangularI64CreateSpec,
    TriangularSubgraph,
    i64,
    1_i64
);

macro_rules! unit_disk_mis_spec {
    ($name:ident,$weight:ty,$one:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            positions: Vec<(f64, f64)>,
            radius: Option<f64>,
            #[create(codec = "comma-separated")]
            weights: Option<Vec<$weight>>,
        }
        impl TryFrom<$name> for MaximumIndependentSet<UnitDiskGraph, $weight> {
            type Error = ConstructionError;
            fn try_from(spec: $name) -> Result<Self, ConstructionError> {
                let radius = spec.radius.unwrap_or(1.0);
                let weights = spec
                    .weights
                    .unwrap_or_else(|| vec![$one; spec.positions.len()]);
                if weights.len() != spec.positions.len() {
                    return Err(ConstructionError::Conversion(
                        "weights length must match positions length".into(),
                    ));
                }
                Ok(Self {
                    graph: UnitDiskGraph::new(spec.positions, radius)?,
                    weights,
                })
            }
        }
    };
}
unit_disk_mis_spec!(MaximumIndependentSetUnitDiskOneCreateSpec, One, One);
unit_disk_mis_spec!(MaximumIndependentSetUnitDiskI64CreateSpec, i64, 1_i64);

impl<G: Graph, W: Clone + Default> MaximumIndependentSet<G, W> {
    /// Create an Independent Set problem from a graph with given weights.
    pub fn new(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
        Self { graph, weights }
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

    /// Check if a configuration is a valid independent set.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
        is_independent_set_config(&self.graph, config)
    }
}

impl<G: Graph, W: WeightElement> MaximumIndependentSet<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G, W> Problem for MaximumIndependentSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumIndependentSet";
    type Solution = Vec<bool>;
    type Value = Max<W::Sum>;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<Max<W::Sum>, crate::traits::EvaluationError> {
        Ok({
            if solution.len() != self.graph.num_vertices() {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    format!(
                        "solution has {} variables, expected {}",
                        solution.len(),
                        self.graph.num_vertices()
                    ),
                ));
            }
            if !is_independent_set_config(&self.graph, solution) {
                return Ok(Max(None));
            }
            let mut total = W::Sum::zero();
            for (i, &selected) in solution.iter().enumerate() {
                if selected {
                    total = W::checked_add_to_sum(
                        total,
                        self.weights[i].to_sum(),
                        "summing selected independent-set weights",
                    )?;
                }
            }
            Max(Some(total))
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for MaximumIndependentSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }
}

/// Check if a configuration forms a valid independent set.
fn is_independent_set_config<G: Graph>(graph: &G, config: &[bool]) -> bool {
    for (u, v) in graph.edges() {
        if config.get(u).copied().unwrap_or(false) && config.get(v).copied().unwrap_or(false) {
            return false;
        }
    }
    true
}

crate::impl_random_generate!(MaximumIndependentSet<SimpleGraph, i64>, crate::random::SimpleGraphRandomSpec, |spec| {
    Ok(MaximumIndependentSet::new(spec.graph()?, vec![1; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<SimpleGraph, One>, crate::random::SimpleGraphRandomSpec, |spec| {
    Ok(MaximumIndependentSet::new(spec.graph()?, vec![One; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<KingsSubgraph, i64>, crate::random::IntegerGeometryRandomSpec, |spec| {
    let seed = crate::random::seed_to_u64(spec.seed)?;
    Ok(MaximumIndependentSet::new(KingsSubgraph::new(crate::random::create_random_int_positions(spec.num_vertices, seed)), vec![1; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<KingsSubgraph, One>, crate::random::IntegerGeometryRandomSpec, |spec| {
    let seed = crate::random::seed_to_u64(spec.seed)?;
    Ok(MaximumIndependentSet::new(KingsSubgraph::new(crate::random::create_random_int_positions(spec.num_vertices, seed)), vec![One; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<TriangularSubgraph, i64>, crate::random::IntegerGeometryRandomSpec, |spec| {
    let seed = crate::random::seed_to_u64(spec.seed)?;
    Ok(MaximumIndependentSet::new(TriangularSubgraph::new(crate::random::create_random_int_positions(spec.num_vertices, seed)), vec![1; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<UnitDiskGraph, i64>, crate::random::UnitDiskRandomSpec, |spec| {
    let seed = crate::random::seed_to_u64(spec.seed)?;
    Ok(MaximumIndependentSet::new(UnitDiskGraph::new(crate::random::create_random_float_positions(spec.num_vertices, seed), spec.radius.unwrap_or(1.0))?, vec![1; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<UnitDiskGraph, One>, crate::random::UnitDiskRandomSpec, |spec| {
    let seed = crate::random::seed_to_u64(spec.seed)?;
    Ok(MaximumIndependentSet::new(UnitDiskGraph::new(crate::random::create_random_float_positions(spec.num_vertices, seed), spec.radius.unwrap_or(1.0))?, vec![One; spec.num_vertices]))
});

crate::declare_variants! {
    MaximumIndependentSet<SimpleGraph, i64> => "1.1996^num_vertices" create MaximumIndependentSetSimpleI64CreateSpec random,
    default MaximumIndependentSet<SimpleGraph, One> => "1.1996^num_vertices" create MaximumIndependentSetSimpleOneCreateSpec random,
    MaximumIndependentSet<KingsSubgraph, i64> => "2^sqrt(num_vertices)" create MaximumIndependentSetKingsI64CreateSpec random,
    MaximumIndependentSet<KingsSubgraph, One> => "2^sqrt(num_vertices)" create MaximumIndependentSetKingsOneCreateSpec random,
    MaximumIndependentSet<TriangularSubgraph, i64> => "2^sqrt(num_vertices)" create MaximumIndependentSetTriangularI64CreateSpec random,
    MaximumIndependentSet<UnitDiskGraph, i64> => "2^sqrt(num_vertices)" create MaximumIndependentSetUnitDiskI64CreateSpec random,
    MaximumIndependentSet<UnitDiskGraph, One> => "2^sqrt(num_vertices)" create MaximumIndependentSetUnitDiskOneCreateSpec random,
}

crate::register_brute_force! {
    MaximumIndependentSet<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaximumIndependentSet<SimpleGraph, One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaximumIndependentSet<KingsSubgraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaximumIndependentSet<KingsSubgraph, One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaximumIndependentSet<TriangularSubgraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaximumIndependentSet<UnitDiskGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaximumIndependentSet<UnitDiskGraph, One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

impl<G, W> crate::models::decision::DecisionProblemMeta for MaximumIndependentSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
    W::Sum: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned,
{
    const DECISION_NAME: &'static str = "DecisionMaximumIndependentSet";
}

impl crate::models::decision::Decision<MaximumIndependentSet<SimpleGraph, i64>> {
    pub fn num_vertices(&self) -> usize {
        self.inner().num_vertices()
    }

    pub fn num_edges(&self) -> usize {
        self.inner().num_edges()
    }
}

crate::register_decision_variant!(
    MaximumIndependentSet<SimpleGraph, i64>,
    "DecisionMaximumIndependentSet",
    "1.1996^num_vertices",
    &["DMIS", "IndependentSet"],
    "Decision version: does an independent set of weight at least the bound exist?",
    category: crate::registry::ProblemCategory::Graph,
    dims: [
        VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        VariantDimension::new("weight", "i64", &["i64"]),
    ],
    fields: [
        FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        FieldInfo { name: "bound", type_name: "W::Sum", description: "Decision bound (minimum required independent-set weight)" },
    ],
    decode: |_, indices: Vec<usize>| crate::config::config_to_bits(&indices)
);

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![
        crate::example_db::specs::ModelExampleSpec {
            id: "maximum_independent_set_petersen_graph",
            instance: Box::new(MaximumIndependentSet::new(
                SimpleGraph::new(
                    10,
                    vec![
                        (0, 1),
                        (1, 2),
                        (2, 3),
                        (3, 4),
                        (4, 0),
                        (5, 7),
                        (7, 9),
                        (9, 6),
                        (6, 8),
                        (8, 5),
                        (0, 5),
                        (1, 6),
                        (2, 7),
                        (3, 8),
                        (4, 9),
                    ],
                ),
                vec![One; 10],
            )),
            optimal_config: serde_json::json!(vec![
                true, false, true, false, false, false, false, false, true, true
            ]),
            optimal_value: serde_json::json!(4),
        },
        crate::example_db::specs::ModelExampleSpec {
            id: "maximum_independent_set_simplegraph",
            instance: Box::new(MaximumIndependentSet::new(
                SimpleGraph::new(
                    10,
                    vec![
                        (0, 1),
                        (1, 2),
                        (2, 3),
                        (3, 4),
                        (4, 0),
                        (5, 7),
                        (7, 9),
                        (9, 6),
                        (6, 8),
                        (8, 5),
                        (0, 5),
                        (1, 6),
                        (2, 7),
                        (3, 8),
                        (4, 9),
                    ],
                ),
                vec![5, 1, 1, 1, 1, 3, 1, 1, 1, 3],
            )),
            optimal_config: serde_json::json!(vec![
                true, false, true, false, false, false, false, false, true, true
            ]),
            optimal_value: serde_json::json!(10),
        },
    ]
}

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_model_example_specs(
) -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "decision_maximum_independent_set_simplegraph",
        instance: Box::new(crate::models::decision::Decision::new(
            MaximumIndependentSet::new(SimpleGraph::path(4), vec![1i64; 4]),
            2,
        )),
        optimal_config: serde_json::json!(vec![true, false, true, false]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_rule_example_specs(
) -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decision_maximum_independent_set_to_maximum_independent_set",
        build: || {
            use crate::example_db::specs::assemble_rule_example;
            use crate::export::SolutionPair;
            use crate::rules::{AggregateReductionResult, ReduceToAggregate};

            let source = crate::models::decision::Decision::new(
                MaximumIndependentSet::new(SimpleGraph::path(4), vec![1i64; 4]),
                2,
            );
            let result = source
                .reduce_to_aggregate()
                .expect("reduction should succeed");
            let target = result.target_problem();
            let config = vec![true, false, true, false];
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

/// Check if a set of vertices forms an independent set.
///
/// # Arguments
/// * `graph` - The graph
/// * `selected` - Boolean slice indicating which vertices are selected
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
#[cfg(test)]
pub(crate) fn is_independent_set<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );
    for (u, v) in graph.edges() {
        if selected[u] && selected[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_independent_set.rs"]
mod tests;
