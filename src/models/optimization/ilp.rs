//! Integer Linear Programming (ILP) problem implementation.
//!
//! ILP optimizes a linear objective over integer variables subject to linear constraints.
//! This is a fundamental "hub" problem that many other NP-hard problems can be reduced to.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ILP",
        category: "optimization",
        description: "Optimize linear objective subject to linear constraints",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of integer variables" },
            FieldInfo { name: "bounds", type_name: "Vec<VarBounds>", description: "Variable bounds" },
            FieldInfo { name: "constraints", type_name: "Vec<LinearConstraint>", description: "Linear constraints" },
            FieldInfo { name: "objective", type_name: "Vec<(usize, f64)>", description: "Sparse objective coefficients" },
            FieldInfo { name: "sense", type_name: "ObjectiveSense", description: "Optimization direction" },
        ],
    }
}

/// Variable bounds (None = unbounded in that direction).
///
/// Represents the lower and upper bounds for an integer variable.
/// A value of `None` indicates the variable is unbounded in that direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VarBounds {
    /// Lower bound (None = -infinity).
    pub lower: Option<i64>,
    /// Upper bound (None = +infinity).
    pub upper: Option<i64>,
}

impl VarBounds {
    /// Create bounds for a binary variable: 0 <= x <= 1.
    pub fn binary() -> Self {
        Self {
            lower: Some(0),
            upper: Some(1),
        }
    }

    /// Create bounds for a non-negative variable: x >= 0.
    pub fn non_negative() -> Self {
        Self {
            lower: Some(0),
            upper: None,
        }
    }

    /// Create unbounded variable: -infinity < x < +infinity.
    pub fn unbounded() -> Self {
        Self {
            lower: None,
            upper: None,
        }
    }

    /// Create bounds with explicit lower and upper: lo <= x <= hi.
    pub fn bounded(lo: i64, hi: i64) -> Self {
        Self {
            lower: Some(lo),
            upper: Some(hi),
        }
    }

    /// Check if a value satisfies these bounds.
    pub fn contains(&self, value: i64) -> bool {
        if let Some(lo) = self.lower {
            if value < lo {
                return false;
            }
        }
        if let Some(hi) = self.upper {
            if value > hi {
                return false;
            }
        }
        true
    }

    /// Get the number of integer values in this bound range.
    /// Returns None if unbounded in either direction.
    pub fn num_values(&self) -> Option<usize> {
        match (self.lower, self.upper) {
            (Some(lo), Some(hi)) => {
                if hi >= lo {
                    Some((hi - lo + 1) as usize)
                } else {
                    Some(0)
                }
            }
            _ => None,
        }
    }
}

/// Comparison operator for linear constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Comparison {
    /// Less than or equal (<=).
    Le,
    /// Greater than or equal (>=).
    Ge,
    /// Equal (==).
    Eq,
}

impl Comparison {
    /// Check if the comparison holds between lhs and rhs.
    pub fn holds(&self, lhs: f64, rhs: f64) -> bool {
        match self {
            Comparison::Le => lhs <= rhs,
            Comparison::Ge => lhs >= rhs,
            Comparison::Eq => (lhs - rhs).abs() < 1e-9,
        }
    }
}

/// A linear constraint: sum of (coefficient * variable) {<=, >=, ==} rhs.
///
/// The constraint is represented sparsely: only non-zero coefficients are stored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinearConstraint {
    /// Sparse representation: (var_index, coefficient) pairs.
    pub terms: Vec<(usize, f64)>,
    /// Comparison operator.
    pub cmp: Comparison,
    /// Right-hand side constant.
    pub rhs: f64,
}

impl LinearConstraint {
    /// Create a new linear constraint.
    pub fn new(terms: Vec<(usize, f64)>, cmp: Comparison, rhs: f64) -> Self {
        Self { terms, cmp, rhs }
    }

    /// Create a less-than-or-equal constraint.
    pub fn le(terms: Vec<(usize, f64)>, rhs: f64) -> Self {
        Self::new(terms, Comparison::Le, rhs)
    }

    /// Create a greater-than-or-equal constraint.
    pub fn ge(terms: Vec<(usize, f64)>, rhs: f64) -> Self {
        Self::new(terms, Comparison::Ge, rhs)
    }

    /// Create an equality constraint.
    pub fn eq(terms: Vec<(usize, f64)>, rhs: f64) -> Self {
        Self::new(terms, Comparison::Eq, rhs)
    }

    /// Evaluate the left-hand side of the constraint for given variable values.
    pub fn evaluate_lhs(&self, values: &[i64]) -> f64 {
        self.terms
            .iter()
            .map(|&(var, coef)| coef * values.get(var).copied().unwrap_or(0) as f64)
            .sum()
    }

    /// Check if the constraint is satisfied by given variable values.
    pub fn is_satisfied(&self, values: &[i64]) -> bool {
        let lhs = self.evaluate_lhs(values);
        self.cmp.holds(lhs, self.rhs)
    }

    /// Get the set of variable indices involved in this constraint.
    pub fn variables(&self) -> Vec<usize> {
        self.terms.iter().map(|&(var, _)| var).collect()
    }
}

/// Optimization direction for the ILP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl From<ObjectiveSense> for EnergyMode {
    fn from(sense: ObjectiveSense) -> Self {
        match sense {
            ObjectiveSense::Maximize => EnergyMode::LargerSizeIsBetter,
            ObjectiveSense::Minimize => EnergyMode::SmallerSizeIsBetter,
        }
    }
}

/// Integer Linear Programming (ILP) problem.
///
/// An ILP consists of:
/// - A set of integer variables with bounds
/// - Linear constraints on those variables
/// - A linear objective function to optimize
/// - An optimization sense (maximize or minimize)
///
/// # Example
///
/// ```
/// use problemreductions::models::optimization::{ILP, VarBounds, Comparison, LinearConstraint, ObjectiveSense};
/// use problemreductions::Problem;
///
/// // Create a simple ILP: maximize x0 + 2*x1
/// // subject to: x0 + x1 <= 3, x0, x1 binary
/// let ilp = ILP::new(
///     2,
///     vec![VarBounds::binary(), VarBounds::binary()],
///     vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 3.0)],
///     vec![(0, 1.0), (1, 2.0)],
///     ObjectiveSense::Maximize,
/// );
///
/// assert_eq!(ilp.num_variables(), 2);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ILP {
    /// Number of variables.
    pub num_vars: usize,
    /// Bounds for each variable.
    pub bounds: Vec<VarBounds>,
    /// Linear constraints.
    pub constraints: Vec<LinearConstraint>,
    /// Sparse objective coefficients: (var_index, coefficient).
    pub objective: Vec<(usize, f64)>,
    /// Optimization direction.
    pub sense: ObjectiveSense,
}

impl ILP {
    /// Create a new ILP problem.
    ///
    /// # Arguments
    /// * `num_vars` - Number of variables
    /// * `bounds` - Bounds for each variable (must have length num_vars)
    /// * `constraints` - List of linear constraints
    /// * `objective` - Sparse objective coefficients
    /// * `sense` - Maximize or minimize
    ///
    /// # Panics
    /// Panics if bounds.len() != num_vars.
    pub fn new(
        num_vars: usize,
        bounds: Vec<VarBounds>,
        constraints: Vec<LinearConstraint>,
        objective: Vec<(usize, f64)>,
        sense: ObjectiveSense,
    ) -> Self {
        assert_eq!(bounds.len(), num_vars, "bounds length must match num_vars");
        Self {
            num_vars,
            bounds,
            constraints,
            objective,
            sense,
        }
    }

    /// Create a binary ILP (all variables are 0-1).
    ///
    /// This is a convenience constructor for common binary optimization problems.
    pub fn binary(
        num_vars: usize,
        constraints: Vec<LinearConstraint>,
        objective: Vec<(usize, f64)>,
        sense: ObjectiveSense,
    ) -> Self {
        let bounds = vec![VarBounds::binary(); num_vars];
        Self::new(num_vars, bounds, constraints, objective, sense)
    }

    /// Create an empty ILP with no variables.
    pub fn empty() -> Self {
        Self {
            num_vars: 0,
            bounds: vec![],
            constraints: vec![],
            objective: vec![],
            sense: ObjectiveSense::Minimize,
        }
    }

    /// Evaluate the objective function for given variable values.
    pub fn evaluate_objective(&self, values: &[i64]) -> f64 {
        self.objective
            .iter()
            .map(|&(var, coef)| coef * values.get(var).copied().unwrap_or(0) as f64)
            .sum()
    }

    /// Check if all bounds are satisfied for given variable values.
    pub fn bounds_satisfied(&self, values: &[i64]) -> bool {
        if values.len() != self.num_vars {
            return false;
        }
        for (i, &value) in values.iter().enumerate() {
            if !self.bounds[i].contains(value) {
                return false;
            }
        }
        true
    }

    /// Check if all constraints are satisfied for given variable values.
    pub fn constraints_satisfied(&self, values: &[i64]) -> bool {
        self.constraints.iter().all(|c| c.is_satisfied(values))
    }

    /// Check if a solution is feasible (satisfies bounds and constraints).
    pub fn is_feasible(&self, values: &[i64]) -> bool {
        self.bounds_satisfied(values) && self.constraints_satisfied(values)
    }

    /// Convert a configuration (Vec<usize>) to integer values (Vec<i64>).
    /// The configuration encodes variable values as offsets from lower bounds.
    fn config_to_values(&self, config: &[usize]) -> Vec<i64> {
        config
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let lo = self.bounds.get(i).and_then(|b| b.lower).unwrap_or(0);
                lo + c as i64
            })
            .collect()
    }
}

impl Problem for ILP {
    const NAME: &'static str = "ILP";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "f64")]
    }

    type Size = f64;

    fn num_variables(&self) -> usize {
        self.num_vars
    }

    fn num_flavors(&self) -> usize {
        // Return the maximum number of values any variable can take.
        // For unbounded variables, return usize::MAX.
        self.bounds
            .iter()
            .map(|b| b.num_values().unwrap_or(usize::MAX))
            .max()
            .unwrap_or(2)
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vars", self.num_vars),
            ("num_constraints", self.constraints.len()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        match self.sense {
            ObjectiveSense::Maximize => EnergyMode::LargerSizeIsBetter,
            ObjectiveSense::Minimize => EnergyMode::SmallerSizeIsBetter,
        }
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<f64> {
        // Convert config to actual integer values
        let values = self.config_to_values(config);

        // Check bounds validity
        let bounds_ok = self.bounds_satisfied(&values);

        // Check constraints satisfaction
        let constraints_ok = self.constraints_satisfied(&values);

        let is_valid = bounds_ok && constraints_ok;

        // Compute objective value
        let obj = self.evaluate_objective(&values);

        SolutionSize::new(obj, is_valid)
    }
}

// === ProblemV2 / OptimizationProblemV2 implementations ===

impl crate::traits::ProblemV2 for ILP {
    const NAME: &'static str = "ILP";
    type Metric = f64;

    fn dims(&self) -> Vec<usize> {
        self.bounds
            .iter()
            .map(|b| b.num_values().unwrap_or(2))
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> f64 {
        let values = self.config_to_values(config);
        if !self.is_feasible(&values) {
            return match self.sense {
                ObjectiveSense::Maximize => f64::MIN,
                ObjectiveSense::Minimize => f64::MAX,
            };
        }
        self.evaluate_objective(&values)
    }
}

impl crate::traits::OptimizationProblemV2 for ILP {
    fn direction(&self) -> crate::types::Direction {
        match self.sense {
            ObjectiveSense::Maximize => crate::types::Direction::Maximize,
            ObjectiveSense::Minimize => crate::types::Direction::Minimize,
        }
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/optimization/ilp.rs"]
mod tests;
