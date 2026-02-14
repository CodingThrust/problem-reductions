//! Core traits for problem reductions.

use crate::traits::Problem;
use std::marker::PhantomData;

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
}

/// Trait for problems that can be reduced to target type T.
///
/// # Example
/// ```text
/// // Example showing reduction workflow
/// use problemreductions::prelude::*;
/// use problemreductions::rules::ReduceTo;
///
/// let sat_problem: Satisfiability = Satisfiability::new(
///     3,  // 3 variables
///     vec![
///         CNFClause::new(vec![0, 1]),     // (x0 OR x1)
///         CNFClause::new(vec![1, 2]),     // (x1 OR x2)
///     ]
/// );
///
/// // Reduce to Independent Set
/// let reduction = sat_problem.reduce_to();
/// let is_problem = reduction.target_problem();
///
/// // Solve and extract solutions
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(is_problem);
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

/// Generic reduction result for natural-edge (subtype) reductions.
///
/// Used when a problem on a specific graph type is trivially reducible to
/// the same problem on a more general graph type (e.g., `MIS<Triangular>` →
/// `MIS<SimpleGraph>`). The solution mapping is identity — vertex indices
/// are preserved.
#[derive(Debug, Clone)]
pub struct ReductionAutoCast<S: Problem, T: Problem> {
    target: T,
    _phantom: PhantomData<S>,
}

impl<S: Problem, T: Problem> ReductionAutoCast<S, T> {
    /// Create a new auto-cast reduction result.
    pub fn new(target: T) -> Self {
        Self {
            target,
            _phantom: PhantomData,
        }
    }
}

impl<S: Problem, T: Problem> ReductionResult for ReductionAutoCast<S, T> {
    type Source = S;
    type Target = T;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/traits.rs"]
mod tests;
