//! Spin Glass (Ising model) problem implementation.
//!
//! The Spin Glass problem minimizes the Ising Hamiltonian energy.

use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
use serde::{Deserialize, Serialize};

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
/// let solutions = solver.find_best(&problem);
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

impl<G, W> Problem for SpinGlass<G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + std::ops::Mul<Output = W>
        + From<i32>
        + 'static,
{
    const NAME: &'static str = "SpinGlass";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", G::NAME), ("weight", short_type_name::<W>())]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.num_vertices()
    }

    fn num_flavors(&self) -> usize {
        2 // Binary: 0 -> -1 spin, 1 -> +1 spin
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_spins", self.graph.num_vertices()),
            ("num_interactions", self.graph.num_edges()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter // Minimize energy
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let spins = Self::config_to_spins(config);
        let energy = self.compute_energy(&spins);
        SolutionSize::valid(energy)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_spin_glass_creation() {
        let problem = SpinGlass::<SimpleGraph, f64>::new(
            3,
            vec![((0, 1), 1.0), ((1, 2), -1.0)],
            vec![0.0, 0.0, 0.0],
        );
        assert_eq!(problem.num_spins(), 3);
        assert_eq!(problem.interactions().len(), 2);
        assert_eq!(problem.fields().len(), 3);
    }

    #[test]
    fn test_spin_glass_without_fields() {
        let problem = SpinGlass::<SimpleGraph, f64>::without_fields(3, vec![((0, 1), 1.0)]);
        assert_eq!(problem.fields(), &[0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_config_to_spins() {
        assert_eq!(
            SpinGlass::<SimpleGraph, f64>::config_to_spins(&[0, 0]),
            vec![-1, -1]
        );
        assert_eq!(
            SpinGlass::<SimpleGraph, f64>::config_to_spins(&[1, 1]),
            vec![1, 1]
        );
        assert_eq!(
            SpinGlass::<SimpleGraph, f64>::config_to_spins(&[0, 1]),
            vec![-1, 1]
        );
        assert_eq!(
            SpinGlass::<SimpleGraph, f64>::config_to_spins(&[1, 0]),
            vec![1, -1]
        );
    }

    #[test]
    fn test_compute_energy() {
        // Two spins with J = 1 (ferromagnetic prefers aligned)
        let problem =
            SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);

        // Aligned spins: energy = J * s1 * s2 = 1 * 1 * 1 = 1 or 1 * (-1) * (-1) = 1
        assert_eq!(problem.compute_energy(&[1, 1]), 1.0);
        assert_eq!(problem.compute_energy(&[-1, -1]), 1.0);

        // Anti-aligned spins: energy = J * s1 * s2 = 1 * 1 * (-1) = -1
        assert_eq!(problem.compute_energy(&[1, -1]), -1.0);
        assert_eq!(problem.compute_energy(&[-1, 1]), -1.0);
    }

    #[test]
    fn test_compute_energy_with_fields() {
        let problem = SpinGlass::<SimpleGraph, f64>::new(2, vec![], vec![1.0, -1.0]);

        // Energy = h1*s1 + h2*s2 = 1*s1 + (-1)*s2
        assert_eq!(problem.compute_energy(&[1, 1]), 0.0); // 1 - 1 = 0
        assert_eq!(problem.compute_energy(&[-1, -1]), 0.0); // -1 + 1 = 0
        assert_eq!(problem.compute_energy(&[1, -1]), 2.0); // 1 + 1 = 2
        assert_eq!(problem.compute_energy(&[-1, 1]), -2.0); // -1 - 1 = -2
    }

    #[test]
    fn test_solution_size() {
        let problem =
            SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);

        // config [0,0] -> spins [-1,-1] -> energy = 1
        let sol = problem.solution_size(&[0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1.0);

        // config [0,1] -> spins [-1,1] -> energy = -1
        let sol = problem.solution_size(&[0, 1]);
        assert_eq!(sol.size, -1.0);
    }

    #[test]
    fn test_brute_force_ferromagnetic() {
        // Ferromagnetic: J > 0 prefers aligned spins to minimize energy
        // But wait, energy = J*s1*s2, so J>0 with aligned gives positive energy
        // For minimization, we want anti-aligned for J>0
        let problem =
            SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Minimum energy is -1 (anti-aligned)
        for sol in &solutions {
            assert_ne!(sol[0], sol[1]);
            assert_eq!(problem.solution_size(sol).size, -1.0);
        }
    }

    #[test]
    fn test_brute_force_antiferromagnetic() {
        // Antiferromagnetic: J < 0, energy = J*s1*s2
        // J<0 with aligned spins gives negative energy (good for minimization)
        let problem =
            SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), -1.0)], vec![0.0, 0.0]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Minimum energy is -1 (aligned)
        for sol in &solutions {
            assert_eq!(sol[0], sol[1]);
            assert_eq!(problem.solution_size(sol).size, -1.0);
        }
    }

    #[test]
    fn test_energy_mode() {
        let problem = SpinGlass::<SimpleGraph, f64>::without_fields(2, vec![]);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_num_variables_flavors() {
        let problem = SpinGlass::<SimpleGraph, f64>::without_fields(5, vec![]);
        assert_eq!(problem.num_variables(), 5);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_problem_size() {
        let problem = SpinGlass::<SimpleGraph, f64>::new(
            3,
            vec![((0, 1), 1.0), ((1, 2), 1.0)],
            vec![0.0, 0.0, 0.0],
        );
        let size = problem.problem_size();
        assert_eq!(size.get("num_spins"), Some(3));
        assert_eq!(size.get("num_interactions"), Some(2));
    }

    #[test]
    fn test_triangle_frustration() {
        // Triangle with all antiferromagnetic couplings - frustrated system
        let problem = SpinGlass::<SimpleGraph, f64>::new(
            3,
            vec![((0, 1), 1.0), ((1, 2), 1.0), ((0, 2), 1.0)],
            vec![0.0, 0.0, 0.0],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Best we can do is satisfy 2 out of 3 interactions
        // Energy = -1 -1 + 1 = -1 (one frustrated)
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, -1.0);
        }
    }

    #[test]
    fn test_from_graph() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem =
            SpinGlass::<SimpleGraph, f64>::from_graph(graph, vec![1.0, 2.0], vec![0.0, 0.0, 0.0]);
        assert_eq!(problem.num_spins(), 3);
        assert_eq!(problem.couplings(), &[1.0, 2.0]);
        assert_eq!(problem.fields(), &[0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_from_graph_without_fields() {
        let graph = SimpleGraph::new(2, vec![(0, 1)]);
        let problem = SpinGlass::<SimpleGraph, f64>::from_graph_without_fields(graph, vec![1.5]);
        assert_eq!(problem.num_spins(), 2);
        assert_eq!(problem.couplings(), &[1.5]);
        assert_eq!(problem.fields(), &[0.0, 0.0]);
    }

    #[test]
    fn test_graph_accessor() {
        let problem =
            SpinGlass::<SimpleGraph, f64>::new(3, vec![((0, 1), 1.0)], vec![0.0, 0.0, 0.0]);
        let graph = problem.graph();
        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.num_edges(), 1);
    }

    #[test]
    fn test_variant() {
        let variant = SpinGlass::<SimpleGraph, f64>::variant();
        assert_eq!(variant.len(), 2);
        assert_eq!(variant[0], ("graph", "SimpleGraph"));
        assert_eq!(variant[1], ("weight", "f64"));
    }
}
