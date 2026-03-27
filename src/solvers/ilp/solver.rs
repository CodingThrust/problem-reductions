//! ILP solver implementation using HiGHS.

use crate::models::algebraic::{Comparison, ObjectiveSense, VariableDomain, ILP};
use crate::models::misc::TimetableDesign;
use crate::rules::{ReduceTo, ReductionMode, ReductionResult};
#[cfg(not(feature = "ilp-highs"))]
use good_lp::default_solver;
#[cfg(feature = "ilp-highs")]
use good_lp::highs;
#[cfg(feature = "ilp-highs")]
use good_lp::solvers::highs::HighsParallelType;
use good_lp::{variable, ProblemVariables, Solution, SolverModel, Variable};

/// An ILP solver using the HiGHS backend.
///
/// This solver solves Integer Linear Programming problems directly using the HiGHS solver.
///
/// # Example
///
/// ```rust,ignore
/// use problemreductions::models::algebraic::{ILP, LinearConstraint, ObjectiveSense};
/// use problemreductions::solvers::ILPSolver;
///
/// // Create a simple binary ILP: maximize x0 + 2*x1 subject to x0 + x1 <= 1
/// let ilp = ILP::<bool>::new(
///     2,
///     vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
///     vec![(0, 1.0), (1, 2.0)],
///     ObjectiveSense::Maximize,
/// );
///
/// let solver = ILPSolver::new();
/// if let Some(solution) = solver.solve(&ilp) {
///     println!("Solution: {:?}", solution);
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct ILPSolver {
    /// Time limit in seconds (None = no limit).
    pub time_limit: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveViaReductionError {
    WitnessPathRequired { name: String },
    NoReductionPath { name: String },
    NoSolution { name: String },
}

impl std::fmt::Display for SolveViaReductionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolveViaReductionError::WitnessPathRequired { name } => write!(
                f,
                "ILP solving requires a witness-capable source problem and reduction path; only aggregate-value solving is available for {}.",
                name
            ),
            SolveViaReductionError::NoReductionPath { name } => {
                write!(f, "No reduction path from {} to ILP", name)
            }
            SolveViaReductionError::NoSolution { name } => {
                write!(f, "ILP solver found no solution for {}", name)
            }
        }
    }
}

impl std::error::Error for SolveViaReductionError {}

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

    /// Solve an ILP problem directly.
    ///
    /// Returns `None` if the problem is infeasible or the solver fails.
    /// The returned solution is a configuration vector where each element
    /// is the variable value (config index = value).
    pub fn solve<V: VariableDomain>(&self, problem: &ILP<V>) -> Option<Vec<usize>> {
        let n = problem.num_vars;
        if n == 0 {
            return problem.is_feasible(&[]).then_some(vec![]);
        }

        // Derive tighter per-variable upper bounds from single-variable ≤ constraints.
        // This avoids giving HiGHS the full domain (e.g. 2^31 for i32), which can
        // cause severe performance degradation even when constraints already bound
        // the variable to a small range.
        let default_ub = (V::DIMS_PER_VAR - 1) as f64;
        let mut upper_bounds = vec![default_ub; n];
        for constraint in &problem.constraints {
            if constraint.cmp == crate::models::algebraic::Comparison::Le
                && constraint.terms.len() == 1
            {
                let (var_idx, coef) = constraint.terms[0];
                if coef > 0.0 && var_idx < n {
                    let ub = constraint.rhs / coef;
                    if ub < upper_bounds[var_idx] {
                        upper_bounds[var_idx] = ub;
                    }
                }
            }
        }

        // Create integer variables with tightened bounds
        let mut vars_builder = ProblemVariables::new();
        let vars: Vec<Variable> = (0..n)
            .map(|i| {
                let mut v = variable().integer();
                v = v.min(0.0);
                v = v.max(upper_bounds[i]);
                vars_builder.add(v)
            })
            .collect();

        // Build objective expression
        let objective: good_lp::Expression = problem
            .objective
            .iter()
            .map(|&(var_idx, coef)| coef * vars[var_idx])
            .sum();

        // Build the model with objective
        let unsolved = match problem.sense {
            ObjectiveSense::Maximize => vars_builder.maximise(&objective),
            ObjectiveSense::Minimize => vars_builder.minimise(&objective),
        };

        // Create the solver model
        #[cfg(feature = "ilp-highs")]
        let mut model = {
            let mut model = unsolved
                .using(highs)
                .set_option("random_seed", 0i32)
                .set_parallel(HighsParallelType::Off)
                .set_threads(1);
            if let Some(seconds) = self.time_limit {
                model = model.set_time_limit(seconds);
            }
            model
        };

        #[cfg(not(feature = "ilp-highs"))]
        let mut model = unsolved.using(default_solver);

        // Add constraints
        for constraint in &problem.constraints {
            // Build left-hand side expression
            let lhs: good_lp::Expression = constraint
                .terms
                .iter()
                .map(|&(var_idx, coef)| coef * vars[var_idx])
                .sum();

            // Create the constraint based on comparison type
            let good_lp_constraint = match constraint.cmp {
                Comparison::Le => lhs.leq(constraint.rhs),
                Comparison::Ge => lhs.geq(constraint.rhs),
                Comparison::Eq => lhs.eq(constraint.rhs),
            };

            model = model.with(good_lp_constraint);
        }

        // Solve
        let solution = model.solve().ok()?;

        // Extract solution: config index = value (no lower bound offset)
        let result: Vec<usize> = vars
            .iter()
            .map(|v| {
                let val = solution.value(*v);
                val.round().max(0.0) as usize
            })
            .collect();

        Some(result)
    }

    /// Solve any problem that reduces to `ILP<bool>`.
    ///
    /// This method first reduces the problem to a binary ILP, solves the ILP,
    /// and then extracts the solution back to the original problem space.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use problemreductions::prelude::*;
    /// use problemreductions::solvers::ILPSolver;
    ///
    /// // Create a problem that reduces directly to ILP.
    /// let problem = MaximumSetPacking::<i32>::new(vec![
    ///     vec![0, 1],
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    ///
    /// // Solve using ILP solver
    /// let solver = ILPSolver::new();
    /// if let Some(solution) = solver.solve_reduced(&problem) {
    ///     println!("Solution: {:?}", solution);
    /// }
    /// ```
    pub fn solve_reduced<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: ReduceTo<ILP<bool>>,
    {
        let reduction = problem.reduce_to();
        let ilp_solution = self.solve(reduction.target_problem())?;
        Some(reduction.extract_solution(&ilp_solution))
    }

    /// Solve a type-erased problem directly when a native solver hook exists.
    ///
    /// Returns `None` if the input type has no direct solver or the solver finds no solution.
    pub fn solve_dyn(&self, any: &dyn std::any::Any) -> Option<Vec<usize>> {
        if let Some(ilp) = any.downcast_ref::<ILP<bool>>() {
            return self.solve(ilp);
        }
        if let Some(ilp) = any.downcast_ref::<ILP<i32>>() {
            return self.solve(ilp);
        }
        if let Some(problem) = any.downcast_ref::<TimetableDesign>() {
            return problem.solve_via_required_assignments();
        }
        None
    }

    fn supports_direct_dyn(&self, any: &dyn std::any::Any) -> bool {
        any.is::<ILP<bool>>() || any.is::<ILP<i32>>() || any.is::<TimetableDesign>()
    }

    /// Two-level path selection:
    /// 1. Dijkstra finds the cheapest path to each ILP variant using
    ///    `MinimizeStepsThenOverhead` (additive edge costs: step count + log overhead).
    /// 2. Across ILP variants, we pick the path whose composed final output size
    ///    is smallest — this is the actual ILP problem size the solver will face.
    fn best_path_to_ilp(
        &self,
        graph: &crate::rules::ReductionGraph,
        name: &str,
        variant: &std::collections::BTreeMap<String, String>,
        mode: ReductionMode,
        instance: &dyn std::any::Any,
    ) -> Option<crate::rules::ReductionPath> {
        let ilp_variants = graph.variants_for("ILP");
        let input_size = crate::rules::ReductionGraph::compute_source_size(name, instance);
        let mut best_path: Option<crate::rules::ReductionPath> = None;
        let mut best_cost = f64::INFINITY;

        for dv in &ilp_variants {
            if let Some(path) = graph.find_cheapest_path_mode(
                name,
                variant,
                "ILP",
                dv,
                mode,
                &input_size,
                &crate::rules::MinimizeStepsThenOverhead,
            ) {
                // Use composed final output size for cross-variant comparison,
                // since this determines the actual ILP problem size.
                let final_size = graph
                    .evaluate_path_overhead(&path, &input_size)
                    .unwrap_or_default();
                let cost = final_size.total() as f64;
                if cost < best_cost {
                    best_cost = cost;
                    best_path = Some(path);
                }
            }
        }

        best_path
    }

    pub fn try_solve_via_reduction(
        &self,
        name: &str,
        variant: &std::collections::BTreeMap<String, String>,
        instance: &dyn std::any::Any,
    ) -> Result<Vec<usize>, SolveViaReductionError> {
        if self.supports_direct_dyn(instance) {
            return self
                .solve_dyn(instance)
                .ok_or_else(|| SolveViaReductionError::NoSolution {
                    name: name.to_string(),
                });
        }

        let graph = crate::rules::ReductionGraph::new();

        let Some(path) =
            self.best_path_to_ilp(&graph, name, variant, ReductionMode::Witness, instance)
        else {
            if self
                .best_path_to_ilp(&graph, name, variant, ReductionMode::Aggregate, instance)
                .is_some()
            {
                return Err(SolveViaReductionError::WitnessPathRequired {
                    name: name.to_string(),
                });
            }

            return Err(SolveViaReductionError::NoReductionPath {
                name: name.to_string(),
            });
        };

        let chain = graph.reduce_along_path(&path, instance).ok_or_else(|| {
            SolveViaReductionError::WitnessPathRequired {
                name: name.to_string(),
            }
        })?;
        let ilp_solution = self.solve_dyn(chain.target_problem_any()).ok_or_else(|| {
            SolveViaReductionError::NoSolution {
                name: name.to_string(),
            }
        })?;
        Ok(chain.extract_solution(&ilp_solution))
    }

    /// Solve a type-erased problem by finding a reduction path to ILP.
    ///
    /// Tries all ILP variants, picks the cheapest path, reduces, solves,
    /// and extracts the solution back. Falls back to direct ILP solve if
    /// the problem is already an ILP type.
    ///
    /// Returns `None` if no path to ILP exists or the solver finds no solution.
    pub fn solve_via_reduction(
        &self,
        name: &str,
        variant: &std::collections::BTreeMap<String, String>,
        instance: &dyn std::any::Any,
    ) -> Option<Vec<usize>> {
        self.try_solve_via_reduction(name, variant, instance).ok()
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/ilp/solver.rs"]
mod tests;
