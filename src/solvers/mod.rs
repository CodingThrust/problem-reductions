//! Solvers for computational problems.

mod brute_force;

#[cfg(feature = "ilp")]
pub mod ilp;

pub use brute_force::{BruteForce, BruteForceFloat};

#[cfg(feature = "ilp")]
pub use ilp::ILPSolver;

use crate::traits::Problem;
use crate::types::SolutionSize;

/// Trait for problem solvers.
pub trait Solver {
    /// Find the best solution(s) for a problem.
    ///
    /// Returns all configurations that achieve the optimal objective value.
    fn find_best<P: Problem>(&self, problem: &P) -> Vec<Vec<usize>>;

    /// Find the best solution(s) along with their solution sizes.
    fn find_best_with_size<P: Problem>(
        &self,
        problem: &P,
    ) -> Vec<(Vec<usize>, SolutionSize<P::Size>)>;
}

// === V2 solver trait ===

use crate::traits::{OptimizationProblemV2, ProblemV2};

/// Solver trait for the V2 trait system.
pub trait SolverV2 {
    /// Find best solution(s) for an optimization problem.
    ///
    /// Returns all configurations that achieve the optimal metric value.
    fn find_best_v2<P: OptimizationProblemV2>(
        &self,
        problem: &P,
    ) -> Vec<Vec<usize>>
    where
        P::Metric: crate::types::NumericSize;

    /// Find any satisfying solution for a satisfaction problem (Metric = bool).
    fn find_satisfying<P: ProblemV2<Metric = bool>>(
        &self,
        problem: &P,
    ) -> Option<Vec<usize>>;

    /// Find all satisfying solutions for a satisfaction problem (Metric = bool).
    fn find_all_satisfying<P: ProblemV2<Metric = bool>>(
        &self,
        problem: &P,
    ) -> Vec<Vec<usize>>;
}

