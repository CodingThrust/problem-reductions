//! Numerical execution of a native ILP through HiGHS.
//!
//! This module knows only ILP data and backend settings. Registry lookup,
//! type-erased dispatch, and reduction-chain extraction belong to the caller.

use crate::models::algebraic::{Comparison, ILPCoefficient, ObjectiveSense, VariableDomain, ILP};
use crate::types::{i64_to_exact_f64, ExactI64ToF64Error, MAX_EXACT_F64_INTEGER};
use highs::{HighsModelStatus, HighsSolutionStatus, RowProblem, Sense};

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

fn accept_backend_status(status: HighsModelStatus) -> Result<(), IlpBackendError> {
    match status {
        HighsModelStatus::Optimal => Ok(()),
        HighsModelStatus::Infeasible => Err(IlpBackendError::Infeasible),
        HighsModelStatus::Unbounded => Err(IlpBackendError::Unbounded),
        HighsModelStatus::ReachedTimeLimit => Err(IlpBackendError::Timeout),
        other => Err(IlpBackendError::BackendFailure(format!(
            "HiGHS status: {other:?}"
        ))),
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

        if n > i32::MAX as usize || problem.constraints().len() > i32::MAX as usize {
            return Err(IlpBackendError::BackendFailure(
                "ILP dimensions exceed the HiGHS index representation".into(),
            ));
        }
        let mut backend = RowProblem::new();
        let mut costs = vec![0.0; n];
        for &(index, coefficient) in objective_terms {
            costs[index] = coefficient.to_backend_number()?;
        }
        let columns = problem
            .variables()
            .iter()
            .enumerate()
            .map(|(index, bounds)| {
                let lower = bounds
                    .lower_bound()
                    .map(i64_to_exact_f64)
                    .transpose()?
                    .unwrap_or(f64::NEG_INFINITY);
                let upper = bounds
                    .upper_bound()
                    .map(i64_to_exact_f64)
                    .transpose()?
                    .unwrap_or(f64::INFINITY);
                Ok(backend.add_integer_column(costs[index], lower..=upper))
            })
            .collect::<Result<Vec<_>, IlpBackendError>>()?;
        for constraint in problem.constraints() {
            let terms = constraint
                .terms()
                .iter()
                .map(|&(index, coefficient)| Ok((columns[index], coefficient.to_backend_number()?)))
                .collect::<Result<Vec<_>, IlpBackendError>>()?;
            let rhs = constraint.rhs().to_backend_number()?;
            let (lower, upper) = match constraint.comparison() {
                Comparison::Le => (f64::NEG_INFINITY, rhs),
                Comparison::Ge => (rhs, f64::INFINITY),
                Comparison::Eq => (rhs, rhs),
            };
            backend.add_row(lower..=upper, terms);
        }
        let sense = match problem.sense() {
            ObjectiveSense::Minimize => Sense::Minimise,
            ObjectiveSense::Maximize => Sense::Maximise,
        };
        let mut model = backend.try_optimise(sense).map_err(|error| {
            IlpBackendError::BackendFailure(format!("loading HiGHS model: {error:?}"))
        })?;
        model.make_quiet();
        for (option, value) in [("random_seed", 0), ("threads", 1)] {
            model.try_set_option(option, value).map_err(|error| {
                IlpBackendError::BackendFailure(format!("setting {option}: {error:?}"))
            })?;
        }
        for option in ["mip_rel_gap", "mip_abs_gap"] {
            model.try_set_option(option, 0.0).map_err(|error| {
                IlpBackendError::BackendFailure(format!("setting {option}: {error:?}"))
            })?;
        }
        model.try_set_option("parallel", "off").map_err(|error| {
            IlpBackendError::BackendFailure(format!("setting parallel: {error:?}"))
        })?;
        if let Some(seconds) = self.time_limit {
            model
                .try_set_option("time_limit", seconds)
                .map_err(|error| {
                    IlpBackendError::BackendFailure(format!("setting time_limit: {error:?}"))
                })?;
        }
        let solved = model.try_solve().map_err(|error| {
            IlpBackendError::BackendFailure(format!("running HiGHS: {error:?}"))
        })?;
        if solved.status() == HighsModelStatus::UnboundedOrInfeasible && !objective_terms.is_empty()
        {
            // A zero objective cannot be unbounded, so feasibility distinguishes these states.
            self.solve_with_objective(problem, &[])?;
            return Err(IlpBackendError::Unbounded);
        }
        accept_backend_status(solved.status())?;
        let gap = solved.mip_gap();
        if gap.is_finite() && gap > 0.0 {
            return Err(IlpBackendError::BackendFailure(format!(
                "HiGHS returned a nonzero optimality gap: {gap}"
            )));
        }
        if solved.primal_solution_status() != HighsSolutionStatus::Feasible {
            return Err(IlpBackendError::BackendFailure(
                "HiGHS returned no feasible primal solution".into(),
            ));
        }
        decode_and_validate(problem, solved.get_solution().columns().iter().copied())
    }
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
            "the rounded assignment violates the ILP; this may be caused by numerical tolerances. \
             Consider tightening the backend's integer feasibility tolerance"
                .into(),
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
