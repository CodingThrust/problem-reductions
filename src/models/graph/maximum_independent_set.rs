//! Independent Set problem implementation.
//!
//! The Independent Set problem asks for a maximum weight subset of vertices
//! such that no two vertices in the subset are adjacent.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
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
            VariantDimension::new("weight", "One", &["One", "i32"]),
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
/// * `W` - The weight type (e.g., `i32`, `f64`, `One`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumIndependentSet;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a triangle graph (3 vertices, 3 edges)
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = MaximumIndependentSet::new(graph, vec![1; 3]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem);
///
/// // Maximum independent set in a triangle has size 1
/// assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
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
            type Error = String;
            fn try_from(spec: $name) -> Result<Self, String> {
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
simple_mis_spec!(MaximumIndependentSetSimpleI32CreateSpec, i32, 1_i32);

macro_rules! grid_mis_spec {
    ($name:ident,$graph:ty,$weight:ty,$one:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            positions: Vec<(i32, i32)>,
            #[create(codec = "comma-separated")]
            weights: Option<Vec<$weight>>,
        }
        impl TryFrom<$name> for MaximumIndependentSet<$graph, $weight> {
            type Error = String;
            fn try_from(spec: $name) -> Result<Self, String> {
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
    MaximumIndependentSetKingsI32CreateSpec,
    KingsSubgraph,
    i32,
    1_i32
);
grid_mis_spec!(
    MaximumIndependentSetTriangularI32CreateSpec,
    TriangularSubgraph,
    i32,
    1_i32
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
            type Error = String;
            fn try_from(spec: $name) -> Result<Self, String> {
                let radius = spec.radius.unwrap_or(1.0);
                if !radius.is_finite() || radius < 0.0 {
                    return Err("radius must be finite and nonnegative".into());
                }
                if spec
                    .positions
                    .iter()
                    .any(|&(x, y)| !x.is_finite() || !y.is_finite())
                {
                    return Err("positions must be finite".into());
                }
                let weights = spec
                    .weights
                    .unwrap_or_else(|| vec![$one; spec.positions.len()]);
                if weights.len() != spec.positions.len() {
                    return Err("weights length must match positions length".into());
                }
                Ok(Self {
                    graph: UnitDiskGraph::new(spec.positions, radius),
                    weights,
                })
            }
        }
    };
}
unit_disk_mis_spec!(MaximumIndependentSetUnitDiskOneCreateSpec, One, One);
unit_disk_mis_spec!(MaximumIndependentSetUnitDiskI32CreateSpec, i32, 1_i32);

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
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
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
    type Value = Max<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> Max<W::Sum> {
        if !is_independent_set_config(&self.graph, config) {
            return Max(None);
        }
        let mut total = W::Sum::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].to_sum();
            }
        }
        Max(Some(total))
    }
}

/// Check if a configuration forms a valid independent set.
fn is_independent_set_config<G: Graph>(graph: &G, config: &[usize]) -> bool {
    for (u, v) in graph.edges() {
        if config.get(u).copied().unwrap_or(0) == 1 && config.get(v).copied().unwrap_or(0) == 1 {
            return false;
        }
    }
    true
}

crate::impl_random_generate!(MaximumIndependentSet<SimpleGraph, i32>, crate::random::SimpleGraphRandomSpec, |spec| {
    Ok(MaximumIndependentSet::new(spec.graph()?, vec![1; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<SimpleGraph, One>, crate::random::SimpleGraphRandomSpec, |spec| {
    Ok(MaximumIndependentSet::new(spec.graph()?, vec![One; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<KingsSubgraph, i32>, crate::random::IntegerGeometryRandomSpec, |spec| {
    Ok(MaximumIndependentSet::new(KingsSubgraph::new(crate::random::create_random_int_positions(spec.num_vertices, spec.seed)), vec![1; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<KingsSubgraph, One>, crate::random::IntegerGeometryRandomSpec, |spec| {
    Ok(MaximumIndependentSet::new(KingsSubgraph::new(crate::random::create_random_int_positions(spec.num_vertices, spec.seed)), vec![One; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<TriangularSubgraph, i32>, crate::random::IntegerGeometryRandomSpec, |spec| {
    Ok(MaximumIndependentSet::new(TriangularSubgraph::new(crate::random::create_random_int_positions(spec.num_vertices, spec.seed)), vec![1; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<UnitDiskGraph, i32>, crate::random::UnitDiskRandomSpec, |spec| {
    Ok(MaximumIndependentSet::new(UnitDiskGraph::new(crate::random::create_random_float_positions(spec.num_vertices, spec.seed), spec.radius.unwrap_or(1.0)), vec![1; spec.num_vertices]))
});
crate::impl_random_generate!(MaximumIndependentSet<UnitDiskGraph, One>, crate::random::UnitDiskRandomSpec, |spec| {
    Ok(MaximumIndependentSet::new(UnitDiskGraph::new(crate::random::create_random_float_positions(spec.num_vertices, spec.seed), spec.radius.unwrap_or(1.0)), vec![One; spec.num_vertices]))
});

crate::declare_variants! {
    MaximumIndependentSet<SimpleGraph, i32> => "1.1996^num_vertices" create MaximumIndependentSetSimpleI32CreateSpec random,
    default MaximumIndependentSet<SimpleGraph, One> => "1.1996^num_vertices" create MaximumIndependentSetSimpleOneCreateSpec random,
    MaximumIndependentSet<KingsSubgraph, i32> => "2^sqrt(num_vertices)" create MaximumIndependentSetKingsI32CreateSpec random,
    MaximumIndependentSet<KingsSubgraph, One> => "2^sqrt(num_vertices)" create MaximumIndependentSetKingsOneCreateSpec random,
    MaximumIndependentSet<TriangularSubgraph, i32> => "2^sqrt(num_vertices)" create MaximumIndependentSetTriangularI32CreateSpec random,
    MaximumIndependentSet<UnitDiskGraph, i32> => "2^sqrt(num_vertices)" create MaximumIndependentSetUnitDiskI32CreateSpec random,
    MaximumIndependentSet<UnitDiskGraph, One> => "2^sqrt(num_vertices)" create MaximumIndependentSetUnitDiskOneCreateSpec random,
}

impl<G, W> crate::models::decision::DecisionProblemMeta for MaximumIndependentSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
    W::Sum: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned,
{
    const DECISION_NAME: &'static str = "DecisionMaximumIndependentSet";
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![
        crate::example_db::specs::ModelExampleSpec {
            id: "maximum_independent_set_simplegraph_one",
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
            optimal_config: vec![1, 0, 1, 0, 0, 0, 0, 0, 1, 1],
            optimal_value: serde_json::json!(4),
        },
        crate::example_db::specs::ModelExampleSpec {
            id: "maximum_independent_set_simplegraph_i32",
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
            optimal_config: vec![1, 0, 1, 0, 0, 0, 0, 0, 1, 1],
            optimal_value: serde_json::json!(10),
        },
    ]
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
