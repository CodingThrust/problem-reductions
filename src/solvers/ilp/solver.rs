//! ILP solver implementation using HiGHS.

use super::traits::{ObjectiveSense, ToILP};
use good_lp::{default_solver, variable, ProblemVariables, Solution, SolverModel, Variable};

/// An ILP solver using the HiGHS backend.
///
/// This solver converts problems to Integer Linear Programming formulations
/// and solves them using the HiGHS solver.
///
/// # Example
///
/// ```rust,ignore
/// use problemreductions::prelude::*;
/// use problemreductions::solvers::ILPSolver;
///
/// let problem: IndependentSetT = IndependentSetT::new(4, vec![(0, 1), (1, 2)]);
/// let solver = ILPSolver::new();
///
/// if let Some(solution) = solver.solve(&problem) {
///     println!("Solution: {:?}", solution);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ILPSolver {
    /// Time limit in seconds (None = no limit).
    pub time_limit: Option<f64>,
}

impl Default for ILPSolver {
    fn default() -> Self {
        Self { time_limit: None }
    }
}

impl ILPSolver {
    /// Create a new ILP solver with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an ILP solver with a time limit.
    pub fn with_time_limit(seconds: f64) -> Self {
        Self {
            time_limit: Some(seconds),
        }
    }

    /// Solve a problem and return the optimal solution.
    ///
    /// Returns `None` if the problem is infeasible or the solver fails.
    pub fn solve<P: ToILP>(&self, problem: &P) -> Option<Vec<usize>> {
        let n = problem.num_variables();
        if n == 0 {
            return Some(vec![]);
        }

        // Create binary variables
        let mut vars_builder = ProblemVariables::new();
        let vars: Vec<Variable> = (0..n)
            .map(|_| vars_builder.add(variable().binary()))
            .collect();

        // Get the ILP formulation from the problem
        let formulation = problem.to_ilp(&vars);

        // Build the model with objective
        let unsolved = match formulation.sense {
            ObjectiveSense::Maximize => vars_builder.maximise(&formulation.objective),
            ObjectiveSense::Minimize => vars_builder.minimise(&formulation.objective),
        };

        // Create the solver model and add constraints
        let mut model = unsolved.using(default_solver);
        for constraint in formulation.constraints {
            model = model.with(constraint);
        }

        // Solve
        let solution = model.solve().ok()?;

        // Extract solution values
        let result: Vec<usize> = vars
            .iter()
            .map(|v| {
                let val = solution.value(*v);
                // Round to nearest integer (should be 0 or 1 for binary vars)
                if val > 0.5 {
                    1
                } else {
                    0
                }
            })
            .collect();

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::graph::IndependentSetT;
    use crate::solvers::{BruteForce, Solver};
    use crate::topology::SimpleGraph;
    use crate::traits::Problem;

    #[test]
    fn test_ilp_solver_basic() {
        let problem: IndependentSetT<SimpleGraph, i32> =
            IndependentSetT::new(4, vec![(0, 1), (1, 2), (2, 3)]);

        let solver = ILPSolver::new();
        let solution = solver.solve(&problem);

        assert!(solution.is_some());
        let sol = solution.unwrap();

        // Solution should be valid
        let result = problem.solution_size(&sol);
        assert!(result.is_valid, "ILP solution should be valid");

        // For a path graph of 4 vertices, max IS = 2
        assert_eq!(result.size, 2);
    }

    #[test]
    fn test_ilp_matches_brute_force() {
        let problem: IndependentSetT<SimpleGraph, i32> =
            IndependentSetT::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 5)]);

        let bf = BruteForce::new();
        let ilp = ILPSolver::new();

        let bf_solutions = bf.find_best(&problem);
        let ilp_solution = ilp.solve(&problem).unwrap();

        // Both should find optimal value
        let bf_size = problem.solution_size(&bf_solutions[0]).size;
        let ilp_size = problem.solution_size(&ilp_solution).size;
        assert_eq!(bf_size, ilp_size, "ILP should find optimal solution");
    }

    #[test]
    fn test_ilp_empty_problem() {
        let problem: IndependentSetT<SimpleGraph, i32> = IndependentSetT::new(0, vec![]);
        let solver = ILPSolver::new();
        let solution = solver.solve(&problem);
        assert_eq!(solution, Some(vec![]));
    }

    #[test]
    fn test_ilp_complete_graph() {
        // Complete graph K4 - max IS = 1
        let problem: IndependentSetT<SimpleGraph, i32> =
            IndependentSetT::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);

        let solver = ILPSolver::new();
        let solution = solver.solve(&problem).unwrap();

        let result = problem.solution_size(&solution);
        assert!(result.is_valid);
        assert_eq!(result.size, 1);
    }

    #[test]
    fn test_ilp_vertex_cover() {
        use crate::models::graph::VertexCoverT;

        // Path graph: 0-1-2-3 - min VC = 2 (vertices 1 and 2, or 0,2 or 1,3)
        let problem: VertexCoverT<SimpleGraph, i32> =
            VertexCoverT::new(4, vec![(0, 1), (1, 2), (2, 3)]);

        let solver = ILPSolver::new();
        let solution = solver.solve(&problem).unwrap();

        let result = problem.solution_size(&solution);
        assert!(result.is_valid, "Solution should cover all edges");
        assert_eq!(result.size, 2, "Minimum vertex cover should be 2");
    }

    #[test]
    fn test_ilp_vertex_cover_vs_brute_force() {
        use crate::models::graph::VertexCoverT;

        let problem: VertexCoverT<SimpleGraph, i32> = VertexCoverT::new(
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 5),
                (0, 5),
                (1, 3),
            ],
        );

        let bf = BruteForce::new();
        let ilp = ILPSolver::new();

        let bf_solutions = bf.find_best(&problem);
        let ilp_solution = ilp.solve(&problem).unwrap();

        // Both should find optimal value
        let bf_size = problem.solution_size(&bf_solutions[0]).size;
        let ilp_size = problem.solution_size(&ilp_solution).size;
        assert_eq!(bf_size, ilp_size, "ILP should find optimal solution");
    }
}
