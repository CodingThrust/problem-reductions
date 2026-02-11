//! Objective value bounds and optimization direction.

use crate::types::EnergyMode;
use serde::{Deserialize, Serialize};

/// Semantic bound for objective value types.
pub trait ObjectiveValue:
    Clone + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign + 'static
{
}

impl<T> ObjectiveValue for T where
    T: Clone + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign + 'static
{
}

/// Objective optimization direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectiveDirection {
    /// Larger objective values are better.
    Maximize,
    /// Smaller objective values are better.
    Minimize,
}

impl ObjectiveDirection {
    /// Returns true if this is a maximization objective.
    pub fn is_maximization(&self) -> bool {
        matches!(self, ObjectiveDirection::Maximize)
    }

    /// Returns true if this is a minimization objective.
    pub fn is_minimization(&self) -> bool {
        matches!(self, ObjectiveDirection::Minimize)
    }

    /// Compare two values according to objective direction.
    pub fn is_better<T: PartialOrd>(&self, a: &T, b: &T) -> bool {
        match self {
            ObjectiveDirection::Maximize => a > b,
            ObjectiveDirection::Minimize => a < b,
        }
    }

    /// Compare two values according to objective direction.
    pub fn is_better_or_equal<T: PartialOrd>(&self, a: &T, b: &T) -> bool {
        match self {
            ObjectiveDirection::Maximize => a >= b,
            ObjectiveDirection::Minimize => a <= b,
        }
    }
}

impl From<EnergyMode> for ObjectiveDirection {
    fn from(value: EnergyMode) -> Self {
        match value {
            EnergyMode::LargerSizeIsBetter => ObjectiveDirection::Maximize,
            EnergyMode::SmallerSizeIsBetter => ObjectiveDirection::Minimize,
        }
    }
}

impl From<ObjectiveDirection> for EnergyMode {
    fn from(value: ObjectiveDirection) -> Self {
        match value {
            ObjectiveDirection::Maximize => EnergyMode::LargerSizeIsBetter,
            ObjectiveDirection::Minimize => EnergyMode::SmallerSizeIsBetter,
        }
    }
}
