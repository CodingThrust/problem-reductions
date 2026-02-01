//! Reductions between SpinGlass and QUBO problems.
//!
//! QUBO: minimize x^T Q x where x ∈ {0, 1}^n
//! SpinGlass: minimize Σ J_ij s_i s_j + Σ h_i s_i where s ∈ {-1, +1}^n
//!
//! Transformation: s = 2x - 1 (so x=0 → s=-1, x=1 → s=+1)

use crate::models::optimization::{SpinGlass, QUBO};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing QUBO to SpinGlass.
#[derive(Debug, Clone)]
pub struct ReductionQUBOToSG {
    target: SpinGlass<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionQUBOToSG {
    type Source = QUBO<f64>;
    type Target = SpinGlass<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution maps directly (same binary encoding).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl ReduceTo<SpinGlass<f64>> for QUBO<f64> {
    type Result = ReductionQUBOToSG;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let matrix = self.matrix();

        // Convert Q matrix to J interactions and h fields
        // Using substitution s = 2x - 1:
        // x = (s + 1) / 2
        // x_i * x_j = ((s_i + 1)/2) * ((s_j + 1)/2) = (s_i*s_j + s_i + s_j + 1) / 4
        //
        // For off-diagonal terms Q_ij x_i x_j:
        //   Q_ij * (s_i*s_j + s_i + s_j + 1) / 4
        //   = Q_ij/4 * s_i*s_j + Q_ij/4 * s_i + Q_ij/4 * s_j + Q_ij/4
        //
        // For diagonal terms Q_ii x_i:
        //   Q_ii * (s_i + 1) / 2 = Q_ii/2 * s_i + Q_ii/2
        let mut interactions = Vec::new();
        let mut onsite = vec![0.0; n];

        for i in 0..n {
            for j in i..n {
                let q = matrix[i][j];
                if q.abs() < 1e-10 {
                    continue;
                }

                if i == j {
                    // Diagonal: Q_ii * x_i = Q_ii/2 * s_i + Q_ii/2 (constant)
                    onsite[i] += q / 2.0;
                } else {
                    // Off-diagonal: Q_ij * x_i * x_j
                    // J_ij contribution
                    let j_ij = q / 4.0;
                    if j_ij.abs() > 1e-10 {
                        interactions.push(((i, j), j_ij));
                    }
                    // h_i and h_j contributions
                    onsite[i] += q / 4.0;
                    onsite[j] += q / 4.0;
                }
            }
        }

        let target = SpinGlass::new(n, interactions, onsite);

        ReductionQUBOToSG {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing SpinGlass to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionSGToQUBO {
    target: QUBO<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionSGToQUBO {
    type Source = SpinGlass<f64>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl ReduceTo<QUBO<f64>> for SpinGlass<f64> {
    type Result = ReductionSGToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_spins();
        let mut matrix = vec![vec![0.0; n]; n];

        // Convert using s = 2x - 1:
        // s_i * s_j = (2x_i - 1)(2x_j - 1) = 4x_i*x_j - 2x_i - 2x_j + 1
        // s_i = 2x_i - 1
        //
        // J_ij * s_i * s_j = J_ij * (4x_i*x_j - 2x_i - 2x_j + 1)
        //                  = 4*J_ij*x_i*x_j - 2*J_ij*x_i - 2*J_ij*x_j + J_ij
        //
        // h_i * s_i = h_i * (2x_i - 1) = 2*h_i*x_i - h_i
        for &((i, j), j_val) in self.interactions() {
            // Off-diagonal: 4 * J_ij
            matrix[i][j] += 4.0 * j_val;
            // Diagonal contributions: -2 * J_ij
            matrix[i][i] -= 2.0 * j_val;
            matrix[j][j] -= 2.0 * j_val;
        }

        // Convert h fields to diagonal
        for (i, &h) in self.fields().iter().enumerate() {
            // h_i * s_i -> 2*h_i*x_i
            matrix[i][i] += 2.0 * h;
        }

        let target = QUBO::from_matrix(matrix);

        ReductionSGToQUBO {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_qubo_to_spinglass() {
        // Simple 2-variable QUBO: minimize x0 + x1 - 2*x0*x1
        // Optimal at x = [0, 0] (value 0) or x = [1, 1] (value 0)
        let qubo = QUBO::from_matrix(vec![vec![1.0, -2.0], vec![0.0, 1.0]]);
        let reduction = ReduceTo::<SpinGlass<f64>>::reduce_to(&qubo);
        let sg = reduction.target_problem();

        let solver = BruteForce::new();
        let sg_solutions = solver.find_best(sg);
        let qubo_solutions: Vec<_> = sg_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();

        // Verify solutions are valid
        assert!(!qubo_solutions.is_empty());

        // Original QUBO at [0,0]: 0, at [1,1]: 1 + 1 - 2 = 0, at [0,1]: 1, at [1,0]: 1
        // So [0,0] and [1,1] are optimal with value 0
        for sol in &qubo_solutions {
            let val = qubo.solution_size(sol).size;
            assert!(
                val <= 0.0 + 1e-6,
                "Expected optimal value near 0, got {}",
                val
            );
        }
    }

    #[test]
    fn test_spinglass_to_qubo() {
        // Simple SpinGlass: J_01 = -1 (ferromagnetic: prefers aligned spins)
        // Energy: J_01 * s0 * s1 = -s0 * s1
        // Aligned spins give -1, anti-aligned give +1
        // Minimum is -1 at [0,0] or [1,1] (both give s=-1,-1 or s=+1,+1)
        let sg = SpinGlass::new(2, vec![((0, 1), -1.0)], vec![0.0, 0.0]);
        let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
        let qubo = reduction.target_problem();

        let solver = BruteForce::new();
        let qubo_solutions = solver.find_best(qubo);

        // Ferromagnetic: aligned spins are optimal
        for sol in &qubo_solutions {
            assert_eq!(sol[0], sol[1], "Ferromagnetic should have aligned spins");
        }
    }

    #[test]
    fn test_roundtrip_qubo_sg_qubo() {
        let original = QUBO::from_matrix(vec![vec![-1.0, 2.0], vec![0.0, -1.0]]);
        let solver = BruteForce::new();
        let original_solutions = solver.find_best(&original);
        let _original_val = original.solution_size(&original_solutions[0]).size;

        // QUBO -> SG -> QUBO
        let reduction1 = ReduceTo::<SpinGlass<f64>>::reduce_to(&original);
        let sg = reduction1.target_problem().clone();
        let reduction2 = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
        let roundtrip = reduction2.target_problem();

        let roundtrip_solutions = solver.find_best(roundtrip);
        let _roundtrip_val = roundtrip.solution_size(&roundtrip_solutions[0]).size;

        // The solutions should have the same configuration
        // (optimal configs should match)
        let orig_configs: std::collections::HashSet<_> = original_solutions.iter().collect();
        let rt_configs: std::collections::HashSet<_> = roundtrip_solutions.iter().collect();
        assert!(
            orig_configs.intersection(&rt_configs).count() > 0,
            "At least one optimal solution should match"
        );
    }

    #[test]
    fn test_antiferromagnetic() {
        // Antiferromagnetic: J > 0, prefers anti-aligned spins
        let sg = SpinGlass::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);
        let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
        let qubo = reduction.target_problem();

        let solver = BruteForce::new();
        let solutions = solver.find_best(qubo);

        // Anti-ferromagnetic: opposite spins are optimal
        for sol in &solutions {
            assert_ne!(
                sol[0], sol[1],
                "Antiferromagnetic should have opposite spins"
            );
        }
    }

    #[test]
    fn test_with_onsite_fields() {
        // SpinGlass with only on-site field h_0 = 1
        // Energy = h_0 * s_0 = s_0
        // Minimum at s_0 = -1, i.e., x_0 = 0
        let sg = SpinGlass::new(1, vec![], vec![1.0]);
        let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
        let qubo = reduction.target_problem();

        let solver = BruteForce::new();
        let solutions = solver.find_best(qubo);

        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0], "Should prefer x=0 (s=-1)");
    }

    #[test]
    fn test_reduction_sizes() {
        // Test source_size and target_size methods
        let qubo = QUBO::from_matrix(vec![vec![1.0, -2.0], vec![0.0, 1.0]]);
        let reduction = ReduceTo::<SpinGlass<f64>>::reduce_to(&qubo);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert!(!source_size.components.is_empty());
        assert!(!target_size.components.is_empty());

        // Test SG to QUBO sizes
        let sg = SpinGlass::new(3, vec![((0, 1), -1.0)], vec![0.0, 0.0, 0.0]);
        let reduction2 = ReduceTo::<QUBO<f64>>::reduce_to(&sg);

        let source_size2 = reduction2.source_size();
        let target_size2 = reduction2.target_size();

        assert!(!source_size2.components.is_empty());
        assert!(!target_size2.components.is_empty());
    }
}

// Register reductions with inventory for auto-discovery
use crate::poly;
use crate::rules::registry::{ReductionEntry, ReductionOverhead};

inventory::submit! {
    ReductionEntry {
        source_name: "QUBO",
        target_name: "SpinGlass",
        source_graph: "QUBOMatrix",
        target_graph: "SpinGlassGraph",
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_spins", poly!(num_vars)),
        ]),
    }
}

inventory::submit! {
    ReductionEntry {
        source_name: "SpinGlass",
        target_name: "QUBO",
        source_graph: "SpinGlassGraph",
        target_graph: "QUBOMatrix",
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_vars", poly!(num_spins)),
        ]),
    }
}
