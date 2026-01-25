//! Trait for converting problems to ILP formulations.

use crate::traits::Problem;
use crate::types::EnergyMode;
use good_lp::{Constraint, Expression, Variable};

/// Specifies whether to maximize or minimize the objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveSense {
    /// Maximize the objective function.
    Maximize,
    /// Minimize the objective function.
    Minimize,
}

impl From<EnergyMode> for ObjectiveSense {
    fn from(mode: EnergyMode) -> Self {
        match mode {
            EnergyMode::LargerSizeIsBetter => ObjectiveSense::Maximize,
            EnergyMode::SmallerSizeIsBetter => ObjectiveSense::Minimize,
        }
    }
}

/// An ILP formulation of an optimization problem.
///
/// Contains the constraints, objective function, and optimization sense.
pub struct ILPFormulation {
    /// Linear constraints.
    pub constraints: Vec<Constraint>,
    /// The objective function to optimize.
    pub objective: Expression,
    /// Whether to maximize or minimize.
    pub sense: ObjectiveSense,
}

impl ILPFormulation {
    /// Create a new ILP formulation.
    pub fn new(constraints: Vec<Constraint>, objective: Expression, sense: ObjectiveSense) -> Self {
        Self {
            constraints,
            objective,
            sense,
        }
    }

    /// Create a maximization formulation.
    pub fn maximize(constraints: Vec<Constraint>, objective: Expression) -> Self {
        Self::new(constraints, objective, ObjectiveSense::Maximize)
    }

    /// Create a minimization formulation.
    pub fn minimize(constraints: Vec<Constraint>, objective: Expression) -> Self {
        Self::new(constraints, objective, ObjectiveSense::Minimize)
    }
}

/// Trait for problems that can be converted to ILP formulations.
///
/// Implement this trait to enable solving a problem with the ILP solver.
/// The formulation should use binary (0-1) variables, one per problem variable.
///
/// # Example
///
/// ```rust,ignore
/// use problemreductions::solvers::ilp::{ToILP, ILPFormulation, ObjectiveSense};
/// use good_lp::{Variable, Expression, Constraint};
///
/// impl ToILP for MyProblem {
///     fn to_ilp(&self, vars: &[Variable]) -> ILPFormulation {
///         let mut constraints = Vec::new();
///         // Add constraints using vars[i] for variable i
///         // ...
///
///         let objective = vars.iter().sum();
///         ILPFormulation::maximize(constraints, objective)
///     }
/// }
/// ```
pub trait ToILP: Problem {
    /// Convert this problem to an ILP formulation.
    ///
    /// The `vars` slice contains one binary variable for each problem variable.
    /// The implementation should return constraints and an objective function
    /// expressed in terms of these variables.
    fn to_ilp(&self, vars: &[Variable]) -> ILPFormulation;
}
