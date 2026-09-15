//! Mathematical solve results, shared by solvers and reduction recovery.

use crate::traits::{EvaluationError, EvaluationValue, Problem};
use serde::{Deserialize, Serialize};
use std::any::Any;

/// A completed solve or a feasible incumbent whose optimality is not established.
/// Execution failures are returned separately as errors.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SolveOutcome<S = serde_json::Value, V = String> {
    Optimal { solution: S, evaluation: V },
    Feasible { solution: S, evaluation: V },
    Infeasible,
}

/// A result retaining the model's concrete solution and value types.
pub type ProblemOutcome<P> = SolveOutcome<<P as Problem>::Solution, <P as Problem>::Value>;

impl<S, V> SolveOutcome<S, V> {
    /// Evaluate and validate a candidate whose optimality is established by the caller.
    ///
    /// Returns an evaluation error if evaluation fails or the candidate violates
    /// the constraints. This checks feasibility, not optimality.
    pub fn optimal<P: Problem<Solution = S, Value = V>>(
        problem: &P,
        solution: S,
    ) -> Result<Self, EvaluationError>
    where
        V: EvaluationValue,
    {
        let evaluation = problem.evaluate(&solution)?;
        if !evaluation.is_valid() {
            return Err(EvaluationError::ConstraintViolation);
        }
        Ok(Self::Optimal {
            solution,
            evaluation,
        })
    }

    /// Evaluate and validate a candidate without claiming optimality.
    ///
    /// Returns an evaluation error if evaluation fails or the candidate violates
    /// the constraints; this does not establish problem infeasibility.
    pub fn feasible<P: Problem<Solution = S, Value = V>>(
        problem: &P,
        solution: S,
    ) -> Result<Self, EvaluationError>
    where
        V: EvaluationValue,
    {
        let evaluation = problem.evaluate(&solution)?;
        if !evaluation.is_valid() {
            return Err(EvaluationError::ConstraintViolation);
        }
        Ok(Self::Feasible {
            solution,
            evaluation,
        })
    }

    pub fn solution(&self) -> Option<&S> {
        match self {
            Self::Optimal { solution, .. } | Self::Feasible { solution, .. } => Some(solution),
            Self::Infeasible => None,
        }
    }

    pub fn into_solution(self) -> Option<S> {
        match self {
            Self::Optimal { solution, .. } | Self::Feasible { solution, .. } => Some(solution),
            Self::Infeasible => None,
        }
    }
}

/// Serialize a model-owned result without evaluating its solution again.
pub(crate) fn outcome_to_json<S: Serialize, V: EvaluationValue + std::fmt::Display>(
    outcome: &SolveOutcome<S, V>,
) -> crate::rules::ExtractionResult<SolveOutcome> {
    if let SolveOutcome::Optimal { evaluation, .. } | SolveOutcome::Feasible { evaluation, .. } =
        outcome
    {
        if !evaluation.is_valid() {
            return Err(EvaluationError::ConstraintViolation.into());
        }
    }
    let encode = |solution| {
        serde_json::to_value(solution).map_err(|error| {
            crate::rules::ExtractionError::invalid(format!(
                "solution serialization failed: {error}"
            ))
        })
    };
    Ok(match outcome {
        SolveOutcome::Optimal {
            solution,
            evaluation,
        } => SolveOutcome::Optimal {
            solution: encode(solution)?,
            evaluation: evaluation.to_string(),
        },
        SolveOutcome::Feasible {
            solution,
            evaluation,
        } => SolveOutcome::Feasible {
            solution: encode(solution)?,
            evaluation: evaluation.to_string(),
        },
        SolveOutcome::Infeasible => SolveOutcome::Infeasible,
    })
}

pub(crate) type ErasedOutcome = SolveOutcome<Box<dyn Any>, Box<dyn Any>>;

pub(crate) fn erase_outcome<S: 'static, V: 'static>(outcome: SolveOutcome<S, V>) -> ErasedOutcome {
    match outcome {
        SolveOutcome::Optimal {
            solution,
            evaluation,
        } => SolveOutcome::Optimal {
            solution: Box::new(solution),
            evaluation: Box::new(evaluation),
        },
        SolveOutcome::Feasible {
            solution,
            evaluation,
        } => SolveOutcome::Feasible {
            solution: Box::new(solution),
            evaluation: Box::new(evaluation),
        },
        SolveOutcome::Infeasible => SolveOutcome::Infeasible,
    }
}

pub(crate) fn downcast_outcome<S: 'static, V: 'static>(
    outcome: ErasedOutcome,
) -> crate::rules::ExtractionResult<SolveOutcome<S, V>> {
    let convert = |solution: Box<dyn Any>, evaluation: Box<dyn Any>| {
        let solution = *solution
            .downcast::<S>()
            .map_err(|_| crate::rules::ExtractionError::invalid("result solution type mismatch"))?;
        let evaluation = *evaluation.downcast::<V>().map_err(|_| {
            crate::rules::ExtractionError::invalid("result evaluation type mismatch")
        })?;
        Ok::<_, crate::rules::ExtractionError>((solution, evaluation))
    };
    Ok(match outcome {
        SolveOutcome::Optimal {
            solution,
            evaluation,
        } => {
            let (solution, evaluation) = convert(solution, evaluation)?;
            SolveOutcome::Optimal {
                solution,
                evaluation,
            }
        }
        SolveOutcome::Feasible {
            solution,
            evaluation,
        } => {
            let (solution, evaluation) = convert(solution, evaluation)?;
            SolveOutcome::Feasible {
                solution,
                evaluation,
            }
        }
        SolveOutcome::Infeasible => SolveOutcome::Infeasible,
    })
}

#[cfg(test)]
#[path = "../unit_tests/solvers/outcome.rs"]
mod tests;
