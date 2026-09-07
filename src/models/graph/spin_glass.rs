//! Spin Glass (Ising model) problem implementation.
//!
//! The Spin Glass problem minimizes the Ising Hamiltonian energy.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use num_traits::{One as _, Zero as _};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SpinGlass",
        display_name: "Spin Glass",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64", "f64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Minimize Ising Hamiltonian on a graph",
        fields: SpinGlassI64CreateSpec::FIELDS,
    }
}

/// The Spin Glass (Ising model) problem.
///
/// Given n spin variables s_i in {-1, +1}, interaction coefficients J_ij,
/// and on-site fields h_i, minimize the Hamiltonian:
///
/// H(s) = sum_{i<j} J_ij * s_i * s_j + sum_i h_i * s_i
///
/// # Representation
///
/// Variables are binary (0 or 1), mapped to spins via: s = 2*x - 1
/// - x = 0 -> s = -1
/// - x = 1 -> s = +1
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `KingsSubgraph`, `UnitDiskGraph`)
/// * `W` - The weight type for couplings (e.g., `i64`, `f64`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::SpinGlass;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Two spins with antiferromagnetic coupling J_01 = 1
/// let problem = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]).unwrap();
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Ground state has opposite spins
/// for sol in &solutions {
///     assert!(sol[0] != sol[1]); // Antiferromagnetic: opposite spins
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SpinGlass<G, W> {
    /// The underlying graph structure.
    graph: G,
    /// Coupling terms J_ij, one per edge in graph.edges() order.
    couplings: Vec<W>,
    /// On-site fields h_i.
    fields: Vec<W>,
}

#[derive(Deserialize)]
struct SpinGlassData<G, W> {
    graph: G,
    couplings: Vec<W>,
    fields: Vec<W>,
}

impl<'de, G, W> Deserialize<'de> for SpinGlass<G, W>
where
    G: Graph + Deserialize<'de>,
    W: WeightElement + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = SpinGlassData::deserialize(deserializer)?;
        Self::from_graph(data.graph, data.couplings, data.fields).map_err(serde::de::Error::custom)
    }
}

macro_rules! spin_glass_create_spec {
    ($name:ident, $weight:ty, $one:expr, $zero:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            /// Undirected interaction graph edges.
            #[create(codec = "edge-list")]
            graph: Vec<(usize, usize)>,
            /// Vertex count, needed to preserve isolated spins.
            num_vertices: Option<usize>,
            /// Pairwise couplings; defaults to one per edge.
            #[create(codec = "comma-separated")]
            couplings: Option<Vec<$weight>>,
            /// On-site fields; defaults to zero per vertex.
            #[create(codec = "comma-separated")]
            fields: Option<Vec<$weight>>,
        }

        impl TryFrom<$name> for SpinGlass<SimpleGraph, $weight> {
            type Error = ConstructionError;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                if spec.graph.is_empty() && spec.num_vertices.is_none() {
                    return Err(ConstructionError::Conversion(
                        "num_vertices is required for an empty graph".into(),
                    ));
                }
                for (index, &(u, v)) in spec.graph.iter().enumerate() {
                    if u == v {
                        return Err(ConstructionError::Conversion(format!(
                            "graph edge {index} is a self-loop at vertex {u}"
                        )));
                    }
                }
                let inferred = spec
                    .graph
                    .iter()
                    .flat_map(|&(u, v)| [u, v])
                    .max()
                    .map(|vertex| {
                        vertex.checked_add(1).ok_or_else(|| {
                            ConstructionError::IntegerOverflow(
                                "inferring the SpinGlass vertex count".into(),
                            )
                        })
                    })
                    .transpose()?
                    .unwrap_or(0);
                let num_vertices = spec.num_vertices.unwrap_or(inferred);
                if num_vertices < inferred {
                    return Err(ConstructionError::Conversion(format!(
                        "num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}"
                    )));
                }
                let couplings = spec
                    .couplings
                    .unwrap_or_else(|| vec![$one; spec.graph.len()]);
                let fields = spec.fields.unwrap_or_else(|| vec![$zero; num_vertices]);
                SpinGlass::from_graph(
                    SimpleGraph::new(num_vertices, spec.graph),
                    couplings,
                    fields,
                )
            }
        }
    };
}

spin_glass_create_spec!(SpinGlassI64CreateSpec, i64, 1_i64, 0_i64);
spin_glass_create_spec!(SpinGlassF64CreateSpec, f64, 1.0_f64, 0.0_f64);

impl<W: WeightElement> SpinGlass<SimpleGraph, W> {
    /// Create a new Spin Glass problem.
    ///
    /// # Arguments
    /// * `num_spins` - Number of spin variables
    /// * `interactions` - Coupling terms J_ij as ((i, j), value)
    /// * `fields` - On-site fields h_i
    pub fn new(
        num_spins: usize,
        interactions: Vec<((usize, usize), W)>,
        fields: Vec<W>,
    ) -> Result<Self, ConstructionError> {
        for (index, &((u, v), _)) in interactions.iter().enumerate() {
            if u >= num_spins || v >= num_spins {
                return Err(ConstructionError::Conversion(format!(
                    "interaction {index} endpoint exceeds num_spins"
                )));
            }
        }
        let edges = interactions.iter().map(|((u, v), _)| (*u, *v)).collect();
        let couplings = interactions
            .iter()
            .map(|(_, coupling)| coupling.clone())
            .collect();
        let graph = SimpleGraph::new(num_spins, edges);
        Self::from_graph(graph, couplings, fields)
    }

    /// Create a Spin Glass with no on-site fields.
    pub fn without_fields(
        num_spins: usize,
        interactions: Vec<((usize, usize), W)>,
    ) -> Result<Self, ConstructionError>
    where
        W: num_traits::Zero,
    {
        let fields = vec![W::zero(); num_spins];
        Self::new(num_spins, interactions, fields)
    }
}

impl<G: Graph, W: WeightElement> SpinGlass<G, W> {
    /// Create a SpinGlass problem from a graph with specified couplings.
    ///
    /// # Arguments
    /// * `graph` - The underlying graph
    /// * `couplings` - Coupling terms (must match graph.num_edges())
    /// * `fields` - On-site fields h_i
    pub fn from_graph(
        graph: G,
        couplings: Vec<W>,
        fields: Vec<W>,
    ) -> Result<Self, ConstructionError> {
        if couplings.len() != graph.num_edges() {
            return Err(ConstructionError::Conversion(
                "couplings length must match num_edges".into(),
            ));
        }
        if fields.len() != graph.num_vertices() {
            return Err(ConstructionError::Conversion(
                "fields length must match num_vertices".into(),
            ));
        }
        for (index, coupling) in couplings.iter().enumerate() {
            coupling.validate_element(&format!("coupling at index {index}"))?;
        }
        for (index, field) in fields.iter().enumerate() {
            field.validate_element(&format!("field at index {index}"))?;
        }
        Ok(Self {
            graph,
            couplings,
            fields,
        })
    }

    /// Create a SpinGlass problem from a graph with no on-site fields.
    pub fn from_graph_without_fields(graph: G, couplings: Vec<W>) -> Result<Self, ConstructionError>
    where
        W: num_traits::Zero,
    {
        let fields = vec![W::zero(); graph.num_vertices()];
        Self::from_graph(graph, couplings, fields)
    }
}

impl<G: Graph, W: Clone + Default> SpinGlass<G, W> {
    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of spins.
    pub fn num_spins(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of interactions (edges in the interaction graph).
    pub fn num_interactions(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the interactions as ((i, j), weight) pairs.
    ///
    /// Reconstructs from graph.edges() and couplings.
    pub fn interactions(&self) -> Vec<((usize, usize), W)> {
        self.graph
            .edges()
            .into_iter()
            .zip(self.couplings.iter())
            .map(|((i, j), w)| ((i, j), w.clone()))
            .collect()
    }

    /// Get the couplings (J_ij values).
    pub fn couplings(&self) -> &[W] {
        &self.couplings
    }

    /// Get the on-site fields.
    pub fn fields(&self) -> &[W] {
        &self.fields
    }

    /// Convert a binary configuration to implementation-local spin signs.
    pub fn config_to_spins(config: &[usize]) -> Result<Vec<i8>, crate::traits::EvaluationError> {
        config
            .iter()
            .map(|&value| match value {
                0 => Ok(-1),
                1 => Ok(1),
                _ => Err(crate::traits::EvaluationError::InvalidConfiguration(
                    format!("binary spin configuration value must be 0 or 1, got {value}"),
                )),
            })
            .collect()
    }
}

impl<G, W> SpinGlass<G, W>
where
    G: Graph,
    W: WeightElement,
{
    /// Compute the Hamiltonian energy for a spin configuration.
    pub fn compute_energy(&self, spins: &[i8]) -> Result<W::Sum, crate::traits::EvaluationError> {
        if spins.len() != self.graph.num_vertices() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                format!(
                    "expected {} spin values, got {}",
                    self.graph.num_vertices(),
                    spins.len()
                ),
            ));
        }
        let spin_sign = |spin| match spin {
            1 => Ok(W::Sum::one()),
            -1 => Ok(W::Sum::zero() - W::Sum::one()),
            value => Err(crate::traits::EvaluationError::InvalidConfiguration(
                format!("spin value must be -1 or 1, got {value}"),
            )),
        };
        let mut energy = W::Sum::zero();

        // Interaction terms: sum J_ij * s_i * s_j
        for ((i, j), j_val) in self.graph.edges().iter().zip(self.couplings.iter()) {
            let s_i = spins[*i];
            let s_j = spins[*j];
            let product = s_i * s_j;
            let term = W::checked_mul_sum(
                j_val.to_sum(),
                spin_sign(product)?,
                "multiplying a SpinGlass coupling by its spin sign",
            )?;
            energy = W::checked_add_to_sum(energy, term, "summing SpinGlass interaction energy")?;
        }

        // On-site terms: sum h_i * s_i
        for (i, h_val) in self.fields.iter().enumerate() {
            let term = W::checked_mul_sum(
                h_val.to_sum(),
                spin_sign(spins[i])?,
                "multiplying a SpinGlass field by its spin sign",
            )?;
            energy = W::checked_add_to_sum(energy, term, "summing SpinGlass field energy")?;
        }

        Ok(energy)
    }
}

impl<G, W> Problem for SpinGlass<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement
        + crate::variant::VariantParam
        + PartialOrd
        + num_traits::Zero
        + num_traits::Bounded,
{
    const NAME: &'static str = "SpinGlass";
    type Solution = Vec<i8>;
    type Value = Min<W::Sum>;

    crate::problem_parameters![
        ("num_interactions", num_interactions),
        ("num_spins", num_spins),
    ];

    fn evaluate(
        &self,
        spins: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        Ok(Min(Some(self.compute_energy(spins)?)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }
}

impl<G, W> crate::solvers::BruteForceProblem for SpinGlass<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement
        + crate::variant::VariantParam
        + PartialOrd
        + num_traits::Zero
        + num_traits::Bounded,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }
}

crate::impl_random_generate!(SpinGlass<SimpleGraph, i64>, crate::random::SimpleGraphRandomSpec, |spec| {
    let graph = spec.graph()?;
    let num_edges = graph.num_edges();
    SpinGlass::from_graph(
        graph,
        vec![1; num_edges],
        vec![0; spec.num_vertices],
    )
});

crate::declare_variants! {
    default SpinGlass<SimpleGraph, i64> => "2^num_spins" create SpinGlassI64CreateSpec random,
    SpinGlass<SimpleGraph, f64> => "2^num_spins" create SpinGlassF64CreateSpec,
}

crate::register_brute_force! {
    SpinGlass<SimpleGraph, i64> decode |_, indices: Vec<usize>| SpinGlass::<SimpleGraph, i64>::config_to_spins(&indices).expect("enumerated spin bits are valid"),
    SpinGlass<SimpleGraph, f64> decode |_, indices: Vec<usize>| SpinGlass::<SimpleGraph, f64>::config_to_spins(&indices).expect("enumerated spin bits are valid"),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "spin_glass_simplegraph",
        instance: Box::new(
            SpinGlass::<SimpleGraph, i64>::without_fields(
                5,
                vec![
                    ((0, 1), 1),
                    ((1, 2), 1),
                    ((3, 4), 1),
                    ((0, 3), 1),
                    ((1, 3), 1),
                    ((1, 4), 1),
                    ((2, 4), 1),
                ],
            )
            .unwrap(),
        ),
        optimal_config: serde_json::json!(vec![1, -1, 1, 1, -1]),
        optimal_value: serde_json::json!(-3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/spin_glass.rs"]
mod tests;
