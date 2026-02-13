//! Spin Glass (Ising model) problem implementation.
//!
//! The Spin Glass problem minimizes the Ising Hamiltonian energy.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SpinGlass",
        module_path: module_path!(),
        description: "Minimize Ising Hamiltonian on a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The interaction graph" },
            FieldInfo { name: "couplings", type_name: "Vec<W>", description: "Pairwise couplings J_ij" },
            FieldInfo { name: "fields", type_name: "Vec<W>", description: "On-site fields h_i" },
        ],
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
/// * `G` - The graph type (e.g., `SimpleGraph`, `GridGraph`, `UnitDiskGraph`)
/// * `W` - The weight type for couplings (e.g., `i32`, `f64`)
///
/// # Example
///
/// ```
/// use problemreductions::models::optimization::SpinGlass;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Two spins with antiferromagnetic coupling J_01 = 1
/// let problem = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Ground state has opposite spins
/// for sol in &solutions {
///     assert!(sol[0] != sol[1]); // Antiferromagnetic: opposite spins
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinGlass<G, W> {
    /// The underlying graph structure.
    graph: G,
    /// Coupling terms J_ij, one per edge in graph.edges() order.
    couplings: Vec<W>,
    /// On-site fields h_i.
    fields: Vec<W>,
}

impl<W: Clone + Default> SpinGlass<SimpleGraph, W> {
    /// Create a new Spin Glass problem.
    ///
    /// # Arguments
    /// * `num_spins` - Number of spin variables
    /// * `interactions` - Coupling terms J_ij as ((i, j), value)
    /// * `fields` - On-site fields h_i
    pub fn new(num_spins: usize, interactions: Vec<((usize, usize), W)>, fields: Vec<W>) -> Self {
        assert_eq!(fields.len(), num_spins);
        let edges: Vec<_> = interactions.iter().map(|((i, j), _)| (*i, *j)).collect();
        let couplings: Vec<_> = interactions.iter().map(|(_, w)| w.clone()).collect();
        let graph = SimpleGraph::new(num_spins, edges);
        Self {
            graph,
            couplings,
            fields,
        }
    }

    /// Create a Spin Glass with no on-site fields.
    pub fn without_fields(num_spins: usize, interactions: Vec<((usize, usize), W)>) -> Self
    where
        W: num_traits::Zero,
    {
        let fields = vec![W::zero(); num_spins];
        Self::new(num_spins, interactions, fields)
    }
}

impl<G: Graph, W: Clone + Default> SpinGlass<G, W> {
    /// Create a SpinGlass problem from a graph with specified couplings.
    ///
    /// # Arguments
    /// * `graph` - The underlying graph
    /// * `couplings` - Coupling terms (must match graph.num_edges())
    /// * `fields` - On-site fields h_i
    pub fn from_graph(graph: G, couplings: Vec<W>, fields: Vec<W>) -> Self {
        assert_eq!(
            couplings.len(),
            graph.num_edges(),
            "couplings length must match num_edges"
        );
        assert_eq!(
            fields.len(),
            graph.num_vertices(),
            "fields length must match num_vertices"
        );
        Self {
            graph,
            couplings,
            fields,
        }
    }

    /// Create a SpinGlass problem from a graph with no on-site fields.
    pub fn from_graph_without_fields(graph: G, couplings: Vec<W>) -> Self
    where
        W: num_traits::Zero,
    {
        let fields = vec![W::zero(); graph.num_vertices()];
        Self::from_graph(graph, couplings, fields)
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of spins.
    pub fn num_spins(&self) -> usize {
        self.graph.num_vertices()
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

    /// Convert binary config (0,1) to spin config (-1,+1).
    pub fn config_to_spins(config: &[usize]) -> Vec<i32> {
        config.iter().map(|&x| 2 * x as i32 - 1).collect()
    }
}

impl<G, W> SpinGlass<G, W>
where
    G: Graph,
    W: Clone + num_traits::Zero + std::ops::AddAssign + std::ops::Mul<Output = W> + From<i32>,
{
    /// Compute the Hamiltonian energy for a spin configuration.
    pub fn compute_energy(&self, spins: &[i32]) -> W {
        let mut energy = W::zero();

        // Interaction terms: sum J_ij * s_i * s_j
        for ((i, j), j_val) in self.graph.edges().iter().zip(self.couplings.iter()) {
            let s_i = spins.get(*i).copied().unwrap_or(1);
            let s_j = spins.get(*j).copied().unwrap_or(1);
            let product: i32 = s_i * s_j;
            energy += j_val.clone() * W::from(product);
        }

        // On-site terms: sum h_i * s_i
        for (i, h_val) in self.fields.iter().enumerate() {
            let s_i = spins.get(i).copied().unwrap_or(1);
            energy += h_val.clone() * W::from(s_i);
        }

        energy
    }
}

impl<G, W> Problem for SpinGlass<G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + std::ops::Mul<Output = W>
        + From<i32>
        + 'static,
{
    const NAME: &'static str = "SpinGlass";
    type Metric = SolutionSize<W>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W> {
        let spins = Self::config_to_spins(config);
        SolutionSize::Valid(self.compute_energy(&spins))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", G::NAME),
            ("weight", crate::variant::short_type_name::<W>()),
        ]
    }
}

impl<G, W> OptimizationProblem for SpinGlass<G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + std::ops::Mul<Output = W>
        + From<i32>
        + 'static,
{
    type Value = W;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/optimization/spin_glass.rs"]
mod tests;
