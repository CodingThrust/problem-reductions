//! Numerical execution of a native ILP through HiGHS.
//!
//! This module knows only ILP data and backend settings. Registry lookup,
//! type-erased dispatch, and reduction-chain extraction belong to the caller.

use crate::models::algebraic::{Comparison, ILPCoefficient, ObjectiveSense, VariableDomain, ILP};
use crate::types::{i64_to_exact_f64, ExactI64ToF64Error, MAX_EXACT_F64_INTEGER};
use good_lp::highs;
use good_lp::solvers::highs::HighsParallelType;
use good_lp::{
    variable, ProblemVariables, ResolutionError, Solution, SolutionStatus, SolverModel, Variable,
};

/// Internal errors are mapped to the existing public solver errors by orchestration.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub(crate) enum IlpBackendError {
    #[error("the ILP is infeasible")]
    Infeasible,
    #[error("the ILP objective is unbounded")]
    Unbounded,
    #[error("the ILP solver reached its time limit before proving optimality")]
    Timeout,
    #[error("the ILP backend failed: {0}")]
    BackendFailure(String),
    #[error("the ILP backend returned an invalid rounded solution: {0}")]
    InvalidSolution(String),
    #[error(transparent)]
    InexactTransport(#[from] ExactI64ToF64Error),
}

/// Backend representation is an execution concern, not a model capability.
pub(crate) trait BackendCoefficient: ILPCoefficient {
    fn to_backend_number(self) -> Result<f64, IlpBackendError>;
}
impl BackendCoefficient for i64 {
    fn to_backend_number(self) -> Result<f64, IlpBackendError> {
        Ok(i64_to_exact_f64(self)?)
    }
}
impl BackendCoefficient for f64 {
    fn to_backend_number(self) -> Result<f64, IlpBackendError> {
        Ok(self)
    }
}

fn classify_backend_error(error: ResolutionError, time_limit: Option<f64>) -> IlpBackendError {
    match error {
        ResolutionError::Infeasible => IlpBackendError::Infeasible,
        ResolutionError::Unbounded => IlpBackendError::Unbounded,
        ResolutionError::Other("NoSolutionFound") if time_limit.is_some() => {
            IlpBackendError::Timeout
        }
        other => IlpBackendError::BackendFailure(other.to_string()),
    }
}

pub(crate) struct HighsAdapter {
    time_limit: Option<f64>,
}

impl HighsAdapter {
    pub(crate) fn new(time_limit: Option<f64>) -> Self {
        Self { time_limit }
    }
    pub(crate) fn solve<V, C>(&self, problem: &ILP<V, C>) -> Result<Vec<i64>, IlpBackendError>
    where
        V: VariableDomain,
        C: BackendCoefficient,
    {
        if self
            .time_limit
            .is_some_and(|seconds| !seconds.is_finite() || seconds < 0.0)
        {
            return Err(IlpBackendError::BackendFailure(
                "time limit must be finite and nonnegative".into(),
            ));
        }
        self.solve_with_objective(problem, problem.objective())
    }

    fn solve_with_objective<V, C>(
        &self,
        problem: &ILP<V, C>,
        objective_terms: &[(usize, C)],
    ) -> Result<Vec<i64>, IlpBackendError>
    where
        V: VariableDomain,
        C: BackendCoefficient,
    {
        let n = problem.num_vars();
        if n == 0 {
            return if problem
                .is_feasible(&[])
                .map_err(|error| IlpBackendError::InvalidSolution(error.to_string()))?
            {
                Ok(vec![])
            } else {
                Err(IlpBackendError::Infeasible)
            };
        }

        let mut vars_builder = ProblemVariables::new();
        let vars: Vec<Variable> = problem
            .variables()
            .iter()
            .map(|variable_bounds| {
                let mut definition = variable().integer();
                if let Some(lower) = variable_bounds.lower_bound() {
                    definition = definition.min(i64_to_exact_f64(lower)?);
                }
                if let Some(upper) = variable_bounds.upper_bound() {
                    definition = definition.max(i64_to_exact_f64(upper)?);
                }
                Ok(vars_builder.add(definition))
            })
            .collect::<Result<_, IlpBackendError>>()?;

        let objective = backend_expression(objective_terms, &vars)?;

        // Build the model with objective
        let unsolved = match problem.sense() {
            ObjectiveSense::Maximize => vars_builder.maximise(&objective),
            ObjectiveSense::Minimize => vars_builder.minimise(&objective),
        };

        // Create the solver model
        let mut model = {
            let mut model = unsolved
                .using(highs)
                .set_option("random_seed", 0i32)
                .set_option("mip_rel_gap", 0.0)
                .set_option("mip_abs_gap", 0.0)
                .set_parallel(HighsParallelType::Off)
                .set_threads(1);
            if let Some(seconds) = self.time_limit {
                model = model.set_time_limit(seconds);
            }
            model
        };

        // Add constraints
        for constraint in problem.constraints() {
            let lhs = backend_expression(constraint.terms(), &vars)?;
            let rhs = constraint.rhs().to_backend_number()?;

            // Create the constraint based on comparison type
            let good_lp_constraint = match constraint.comparison() {
                Comparison::Le => lhs.leq(rhs),
                Comparison::Ge => lhs.geq(rhs),
                Comparison::Eq => lhs.eq(rhs),
            };

            model = model.with(good_lp_constraint);
        }

        // Solve
        let solution = match model.solve() {
            Ok(solution) => solution,
            Err(ResolutionError::Infeasible)
                if !objective_terms.is_empty()
                    && problem.variables().iter().any(|variable| {
                        variable.lower_bound().is_none() || variable.upper_bound().is_none()
                    }) =>
            {
                // A zero objective cannot be unbounded, so feasibility distinguishes the two states.
                self.solve_with_objective(problem, &[])?;
                return Err(IlpBackendError::Unbounded);
            }
            Err(error) => return Err(classify_backend_error(error, self.time_limit)),
        };

        match solution.status() {
            SolutionStatus::Optimal => {}
            SolutionStatus::TimeLimit => return Err(IlpBackendError::Timeout),
            SolutionStatus::GapLimit => {
                return Err(IlpBackendError::BackendFailure(
                    "the backend stopped at its gap limit before proving optimality".to_string(),
                ));
            }
        }

        decode_and_validate(
            problem,
            vars.iter().map(|variable| solution.value(*variable)),
        )
    }
}

fn backend_expression<C: BackendCoefficient>(
    terms: &[(usize, C)],
    variables: &[Variable],
) -> Result<good_lp::Expression, IlpBackendError> {
    terms.iter().try_fold(
        good_lp::Expression::with_capacity(terms.len()),
        |mut expression, &(index, coefficient)| {
            expression.add_mul(coefficient.to_backend_number()?, variables[index]);
            Ok(expression)
        },
    )
}

fn decode_and_validate<V: VariableDomain, C: ILPCoefficient>(
    problem: &ILP<V, C>,
    values: impl IntoIterator<Item = f64>,
) -> Result<Vec<i64>, IlpBackendError> {
    let result = values
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            if !value.is_finite() {
                return Err(IlpBackendError::InvalidSolution(format!(
                    "variable {index} is non-finite"
                )));
            }
            let rounded = value.round();
            if (value - rounded).abs() > 1e-6 {
                return Err(IlpBackendError::InvalidSolution(format!(
                    "variable {index} has non-integral value {value}"
                )));
            }
            if rounded.abs() > MAX_EXACT_F64_INTEGER as f64 {
                return Err(IlpBackendError::InvalidSolution(format!(
                    "variable {index} value {rounded} exceeds exact f64 integer transport"
                )));
            }
            Ok(rounded as i64)
        })
        .collect::<Result<Vec<_>, _>>()?;
    if !problem
        .is_feasible(&result)
        .map_err(|error| IlpBackendError::InvalidSolution(error.to_string()))?
    {
        return Err(IlpBackendError::InvalidSolution(
            "the rounded assignment violates the ILP".into(),
        ));
    }
    problem
        .evaluate_objective(&result)
        .map_err(|error| IlpBackendError::InvalidSolution(error.to_string()))?;
    Ok(result)
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/ilp/adapter.rs"]
mod tests;
