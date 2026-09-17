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

/// Check the optional display-form evaluation at the external JSON boundary.
pub(crate) fn check_reported_evaluation<V: std::fmt::Display + 'static>(
    reported: &serde_json::Value,
    actual: &V,
) -> crate::rules::ExtractionResult<()> {
    use crate::types::{Extremum, Max, Min};
    use std::any::TypeId;

    let expected = actual.to_string();
    let matches = reported.as_str().is_some_and(|reported| {
        if reported == expected {
            return true;
        }
        let Some((wrapper, expected_value)) = expected.split_once('(') else {
            return false;
        };
        let Some(value) = reported
            .strip_prefix(wrapper)
            .and_then(|s| s.strip_prefix('('))
            .and_then(|s| s.strip_suffix(')'))
        else {
            return false;
        };
        let expected_value = expected_value.trim_end_matches(')');
        if [
            TypeId::of::<Min<f64>>(),
            TypeId::of::<Max<f64>>(),
            TypeId::of::<Extremum<f64>>(),
        ]
        .contains(&TypeId::of::<V>())
        {
            let (Ok(value), Ok(expected_value)) =
                (value.parse::<f64>(), expected_value.parse::<f64>())
            else {
                return false;
            };
            // Absolute and relative tolerances apply only to floating-point evaluations.
            value.is_finite()
                && expected_value.is_finite()
                && (value - expected_value).abs()
                    <= 1e-9 * value.abs().max(expected_value.abs()).max(1.0)
        } else {
            match (
                value.parse::<num_bigint::BigInt>(),
                expected_value.parse::<num_bigint::BigInt>(),
            ) {
                (Ok(value), Ok(expected_value)) => value == expected_value,
                _ => false,
            }
        }
    });
    if !matches {
        return Err(crate::rules::ExtractionError::invalid(format!(
            "invalid or mismatched evaluation: received {reported}, expected {expected}"
        )));
    }
    Ok(())
}

impl<S, V> SolveOutcome<S, V> {
    /// Evaluate and validate a candidate whose optimality is established by the caller.
    ///
    /// Returns an evaluation error if evaluation fails or the candidate violates
    /// the constraints. This checks feasibility, not optimality.
    ///
    /// The value type must be an [`EvaluationValue`]:
    ///
    /// ```
    /// # use problemreductions::traits::{EvaluationError, Problem};
    /// # use problemreductions::types::{Max, ProblemParameters, Sum};
    /// # use problemreductions::solvers::SolveOutcome;
    /// # #[derive(Clone)]
    /// # struct Constant<V>(V);
    /// # impl<V: Clone> Problem for Constant<V> {
    /// #     const NAME: &'static str = "Constant";
    /// #     type Solution = ();
    /// #     type Value = V;
    /// #     fn parameter_names() -> &'static [&'static str] { &[] }
    /// #     fn parameters(&self) -> ProblemParameters { ProblemParameters::new(vec![]) }
    /// #     fn evaluate(&self, _: &()) -> Result<V, EvaluationError> { Ok(self.0.clone()) }
    /// #     fn variant() -> Vec<(&'static str, &'static str)> { vec![] }
    /// # }
    /// assert!(SolveOutcome::optimal(&Constant(Max(Some(1_u64))), ()).is_ok());
    /// ```
    ///
    /// Fold-only values such as `Sum` and `And` carry no candidate feasibility,
    /// so the same problem with a `Sum` value is rejected at compile time:
    ///
    /// ```compile_fail,E0277
    /// # use problemreductions::traits::{EvaluationError, Problem};
    /// # use problemreductions::types::{Max, ProblemParameters, Sum};
    /// # use problemreductions::solvers::SolveOutcome;
    /// # #[derive(Clone)]
    /// # struct Constant<V>(V);
    /// # impl<V: Clone> Problem for Constant<V> {
    /// #     const NAME: &'static str = "Constant";
    /// #     type Solution = ();
    /// #     type Value = V;
    /// #     fn parameter_names() -> &'static [&'static str] { &[] }
    /// #     fn parameters(&self) -> ProblemParameters { ProblemParameters::new(vec![]) }
    /// #     fn evaluate(&self, _: &()) -> Result<V, EvaluationError> { Ok(self.0.clone()) }
    /// #     fn variant() -> Vec<(&'static str, &'static str)> { vec![] }
    /// # }
    /// assert!(SolveOutcome::optimal(&Constant(Sum(1_u64)), ()).is_ok());
    /// ```
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
