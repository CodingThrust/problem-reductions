//! Core traits for problem reductions.

use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing a source problem to a target problem.
///
/// This trait encapsulates the target problem and provides methods
/// to extract solutions back to the source problem space.
pub trait ReductionResult: Clone {
    /// The source problem type.
    type Source: Problem;
    /// The target problem type.
    type Target: Problem;

    /// Get a reference to the target problem.
    fn target_problem(&self) -> &Self::Target;

    /// Extract a solution from target problem space to source problem space.
    ///
    /// # Arguments
    /// * `target_solution` - A solution to the target problem
    ///
    /// # Returns
    /// The corresponding solution in the source problem space
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize>;

    /// Get the size of the source problem (for complexity analysis).
    fn source_size(&self) -> ProblemSize;

    /// Get the size of the target problem (for complexity analysis).
    fn target_size(&self) -> ProblemSize;
}

/// Trait for problems that can be reduced to target type T.
///
/// # Example
/// ```ignore
/// let sat_problem = Satisfiability::new(...);
/// let reduction = sat_problem.reduce_to::<IndependentSet<i32>>();
/// let is_problem = reduction.target_problem();
/// let solutions = solver.find_best(is_problem);
/// let sat_solutions: Vec<_> = solutions.iter()
///     .map(|s| reduction.extract_solution(s))
///     .collect();
/// ```
pub trait ReduceTo<T: Problem>: Problem {
    /// The reduction result type.
    type Result: ReductionResult<Source = Self, Target = T>;

    /// Reduce this problem to the target problem type.
    fn reduce_to(&self) -> Self::Result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_traits_compile() {
        // Traits should compile - actual tests in reduction implementations
    }
}
