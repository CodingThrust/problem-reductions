//! Evaluation result for assignments.

use crate::types::SolutionSize;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Objective evaluation paired with feasibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evaluation<T> {
    /// Objective value for this assignment.
    pub objective: T,
    /// Whether assignment satisfies hard constraints.
    pub feasible: bool,
}

impl<T> Evaluation<T> {
    /// Create a feasible evaluation.
    pub fn feasible(objective: T) -> Self {
        Self {
            objective,
            feasible: true,
        }
    }

    /// Create an infeasible evaluation.
    pub fn infeasible(objective: T) -> Self {
        Self {
            objective,
            feasible: false,
        }
    }

    /// Create evaluation with explicit feasibility.
    pub fn new(objective: T, feasible: bool) -> Self {
        Self {
            objective,
            feasible,
        }
    }

    /// Transform objective type while preserving feasibility.
    pub fn map<U, F>(self, mut f: F) -> Evaluation<U>
    where
        F: FnMut(T) -> U,
    {
        Evaluation {
            objective: f(self.objective),
            feasible: self.feasible,
        }
    }
}

impl<T> From<SolutionSize<T>> for Evaluation<T> {
    fn from(value: SolutionSize<T>) -> Self {
        Self {
            objective: value.size,
            feasible: value.is_valid,
        }
    }
}

impl<T> From<Evaluation<T>> for SolutionSize<T> {
    fn from(value: Evaluation<T>) -> Self {
        SolutionSize::new(value.objective, value.feasible)
    }
}

impl<T: fmt::Display> fmt::Display for Evaluation<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Evaluation({}, {})",
            self.objective,
            if self.feasible {
                "feasible"
            } else {
                "infeasible"
            }
        )
    }
}
