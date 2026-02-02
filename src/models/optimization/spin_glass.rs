//! Spin Glass (Ising model) problem implementation.
//!
//! The Spin Glass problem minimizes the Ising Hamiltonian energy.

use crate::traits::Problem;
use crate::variant::short_type_name;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};

/// The Spin Glass (Ising model) problem.
///
/// Given n spin variables s_i ∈ {-1, +1}, interaction coefficients J_ij,
/// and on-site fields h_i, minimize the Hamiltonian:
///
/// H(s) = Σ_{i<j} J_ij * s_i * s_j + Σ_i h_i * s_i
///
/// # Representation
///
/// Variables are binary (0 or 1), mapped to spins via: s = 2*x - 1
/// - x = 0 → s = -1
/// - x = 1 → s = +1
///
/// # Example
///
/// ```
/// use problemreductions::models::optimization::SpinGlass;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Two spins with antiferromagnetic coupling J_01 = 1
/// let problem = SpinGlass::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);
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
pub struct SpinGlass<W = f64> {
    /// Number of spins.
    num_spins: usize,
    /// Interaction terms J_ij as ((i, j), value).
    interactions: Vec<((usize, usize), W)>,
    /// On-site fields h_i.
    fields: Vec<W>,
}

impl<W: Clone + Default> SpinGlass<W> {
    /// Create a new Spin Glass problem.
    ///
    /// # Arguments
    /// * `num_spins` - Number of spin variables
    /// * `interactions` - Coupling terms J_ij as ((i, j), value)
    /// * `fields` - On-site fields h_i
    pub fn new(num_spins: usize, interactions: Vec<((usize, usize), W)>, fields: Vec<W>) -> Self {
        assert_eq!(fields.len(), num_spins);
        Self {
            num_spins,
            interactions,
            fields,
        }
    }

    /// Create a Spin Glass with no on-site fields.
    pub fn without_fields(num_spins: usize, interactions: Vec<((usize, usize), W)>) -> Self
    where
        W: num_traits::Zero,
    {
        let fields = vec![W::zero(); num_spins];
        Self {
            num_spins,
            interactions,
            fields,
        }
    }

    /// Get the number of spins.
    pub fn num_spins(&self) -> usize {
        self.num_spins
    }

    /// Get the interactions.
    pub fn interactions(&self) -> &[((usize, usize), W)] {
        &self.interactions
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

impl<W> Problem for SpinGlass<W>
where
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
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.num_spins
    }

    fn num_flavors(&self) -> usize {
        2 // Binary: 0 → -1 spin, 1 → +1 spin
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_spins", self.num_spins),
            ("num_interactions", self.interactions.len()),
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

impl<W> SpinGlass<W>
where
    W: Clone + num_traits::Zero + std::ops::AddAssign + std::ops::Mul<Output = W> + From<i32>,
{
    /// Compute the Hamiltonian energy for a spin configuration.
    pub fn compute_energy(&self, spins: &[i32]) -> W {
        let mut energy = W::zero();

        // Interaction terms: Σ J_ij * s_i * s_j
        for ((i, j), j_val) in &self.interactions {
            let s_i = spins.get(*i).copied().unwrap_or(1);
            let s_j = spins.get(*j).copied().unwrap_or(1);
            let product: i32 = s_i * s_j;
            energy += j_val.clone() * W::from(product);
        }

        // On-site terms: Σ h_i * s_i
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
        let problem = SpinGlass::new(3, vec![((0, 1), 1.0), ((1, 2), -1.0)], vec![0.0, 0.0, 0.0]);
        assert_eq!(problem.num_spins(), 3);
        assert_eq!(problem.interactions().len(), 2);
        assert_eq!(problem.fields().len(), 3);
    }

    #[test]
    fn test_spin_glass_without_fields() {
        let problem = SpinGlass::<f64>::without_fields(3, vec![((0, 1), 1.0)]);
        assert_eq!(problem.fields(), &[0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_config_to_spins() {
        assert_eq!(SpinGlass::<f64>::config_to_spins(&[0, 0]), vec![-1, -1]);
        assert_eq!(SpinGlass::<f64>::config_to_spins(&[1, 1]), vec![1, 1]);
        assert_eq!(SpinGlass::<f64>::config_to_spins(&[0, 1]), vec![-1, 1]);
        assert_eq!(SpinGlass::<f64>::config_to_spins(&[1, 0]), vec![1, -1]);
    }

    #[test]
    fn test_compute_energy() {
        // Two spins with J = 1 (ferromagnetic prefers aligned)
        let problem = SpinGlass::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);

        // Aligned spins: energy = J * s1 * s2 = 1 * 1 * 1 = 1 or 1 * (-1) * (-1) = 1
        assert_eq!(problem.compute_energy(&[1, 1]), 1.0);
        assert_eq!(problem.compute_energy(&[-1, -1]), 1.0);

        // Anti-aligned spins: energy = J * s1 * s2 = 1 * 1 * (-1) = -1
        assert_eq!(problem.compute_energy(&[1, -1]), -1.0);
        assert_eq!(problem.compute_energy(&[-1, 1]), -1.0);
    }

    #[test]
    fn test_compute_energy_with_fields() {
        let problem = SpinGlass::new(2, vec![], vec![1.0, -1.0]);

        // Energy = h1*s1 + h2*s2 = 1*s1 + (-1)*s2
        assert_eq!(problem.compute_energy(&[1, 1]), 0.0); // 1 - 1 = 0
        assert_eq!(problem.compute_energy(&[-1, -1]), 0.0); // -1 + 1 = 0
        assert_eq!(problem.compute_energy(&[1, -1]), 2.0); // 1 + 1 = 2
        assert_eq!(problem.compute_energy(&[-1, 1]), -2.0); // -1 - 1 = -2
    }

    #[test]
    fn test_solution_size() {
        let problem = SpinGlass::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);

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
        let problem = SpinGlass::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);
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
        let problem = SpinGlass::new(2, vec![((0, 1), -1.0)], vec![0.0, 0.0]);
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
        let problem = SpinGlass::<f64>::without_fields(2, vec![]);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_num_variables_flavors() {
        let problem = SpinGlass::<f64>::without_fields(5, vec![]);
        assert_eq!(problem.num_variables(), 5);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_problem_size() {
        let problem = SpinGlass::new(3, vec![((0, 1), 1.0), ((1, 2), 1.0)], vec![0.0, 0.0, 0.0]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_spins"), Some(3));
        assert_eq!(size.get("num_interactions"), Some(2));
    }

    #[test]
    fn test_triangle_frustration() {
        // Triangle with all antiferromagnetic couplings - frustrated system
        let problem = SpinGlass::new(
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
}
