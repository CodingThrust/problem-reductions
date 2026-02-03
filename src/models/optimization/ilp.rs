//! Integer Linear Programming (ILP) problem implementation.
//!
//! ILP optimizes a linear objective over integer variables subject to linear constraints.
//! This is a fundamental "hub" problem that many other NP-hard problems can be reduced to.

use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    // ============================================================
    // VarBounds tests
    // ============================================================

    #[test]
    fn test_varbounds_binary() {
        let bounds = VarBounds::binary();
        assert_eq!(bounds.lower, Some(0));
        assert_eq!(bounds.upper, Some(1));
        assert!(bounds.contains(0));
        assert!(bounds.contains(1));
        assert!(!bounds.contains(-1));
        assert!(!bounds.contains(2));
        assert_eq!(bounds.num_values(), Some(2));
    }

    #[test]
    fn test_varbounds_non_negative() {
        let bounds = VarBounds::non_negative();
        assert_eq!(bounds.lower, Some(0));
        assert_eq!(bounds.upper, None);
        assert!(bounds.contains(0));
        assert!(bounds.contains(100));
        assert!(!bounds.contains(-1));
        assert_eq!(bounds.num_values(), None);
    }

    #[test]
    fn test_varbounds_unbounded() {
        let bounds = VarBounds::unbounded();
        assert_eq!(bounds.lower, None);
        assert_eq!(bounds.upper, None);
        assert!(bounds.contains(-1000));
        assert!(bounds.contains(0));
        assert!(bounds.contains(1000));
        assert_eq!(bounds.num_values(), None);
    }

    #[test]
    fn test_varbounds_bounded() {
        let bounds = VarBounds::bounded(-5, 10);
        assert_eq!(bounds.lower, Some(-5));
        assert_eq!(bounds.upper, Some(10));
        assert!(bounds.contains(-5));
        assert!(bounds.contains(0));
        assert!(bounds.contains(10));
        assert!(!bounds.contains(-6));
        assert!(!bounds.contains(11));
        assert_eq!(bounds.num_values(), Some(16)); // -5 to 10 inclusive
    }

    #[test]
    fn test_varbounds_default() {
        let bounds = VarBounds::default();
        assert_eq!(bounds.lower, None);
        assert_eq!(bounds.upper, None);
    }

    #[test]
    fn test_varbounds_empty_range() {
        let bounds = VarBounds::bounded(5, 3); // Invalid: lo > hi
        assert_eq!(bounds.num_values(), Some(0));
    }

    // ============================================================
    // Comparison tests
    // ============================================================

    #[test]
    fn test_comparison_le() {
        let cmp = Comparison::Le;
        assert!(cmp.holds(5.0, 10.0));
        assert!(cmp.holds(10.0, 10.0));
        assert!(!cmp.holds(11.0, 10.0));
    }

    #[test]
    fn test_comparison_ge() {
        let cmp = Comparison::Ge;
        assert!(cmp.holds(10.0, 5.0));
        assert!(cmp.holds(10.0, 10.0));
        assert!(!cmp.holds(4.0, 5.0));
    }

    #[test]
    fn test_comparison_eq() {
        let cmp = Comparison::Eq;
        assert!(cmp.holds(10.0, 10.0));
        assert!(!cmp.holds(10.0, 10.1));
        assert!(!cmp.holds(9.9, 10.0));
        // Test tolerance
        assert!(cmp.holds(10.0, 10.0 + 1e-10));
    }

    // ============================================================
    // LinearConstraint tests
    // ============================================================

    #[test]
    fn test_linear_constraint_le() {
        // x0 + 2*x1 <= 5
        let constraint = LinearConstraint::le(vec![(0, 1.0), (1, 2.0)], 5.0);
        assert_eq!(constraint.cmp, Comparison::Le);
        assert_eq!(constraint.rhs, 5.0);

        // x0=1, x1=2 => 1 + 4 = 5 <= 5 (satisfied)
        assert!(constraint.is_satisfied(&[1, 2]));
        // x0=2, x1=2 => 2 + 4 = 6 > 5 (not satisfied)
        assert!(!constraint.is_satisfied(&[2, 2]));
    }

    #[test]
    fn test_linear_constraint_ge() {
        // x0 + x1 >= 3
        let constraint = LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 3.0);
        assert_eq!(constraint.cmp, Comparison::Ge);

        assert!(constraint.is_satisfied(&[2, 2])); // 4 >= 3
        assert!(constraint.is_satisfied(&[1, 2])); // 3 >= 3
        assert!(!constraint.is_satisfied(&[1, 1])); // 2 < 3
    }

    #[test]
    fn test_linear_constraint_eq() {
        // x0 + x1 == 2
        let constraint = LinearConstraint::eq(vec![(0, 1.0), (1, 1.0)], 2.0);
        assert_eq!(constraint.cmp, Comparison::Eq);

        assert!(constraint.is_satisfied(&[1, 1])); // 2 == 2
        assert!(!constraint.is_satisfied(&[1, 2])); // 3 != 2
        assert!(!constraint.is_satisfied(&[0, 1])); // 1 != 2
    }

    #[test]
    fn test_linear_constraint_evaluate_lhs() {
        let constraint = LinearConstraint::le(vec![(0, 3.0), (2, -1.0)], 10.0);
        // 3*x0 - 1*x2 with x=[2, 5, 7] => 3*2 - 1*7 = -1
        assert!((constraint.evaluate_lhs(&[2, 5, 7]) - (-1.0)).abs() < 1e-9);
    }

    #[test]
    fn test_linear_constraint_variables() {
        let constraint = LinearConstraint::le(vec![(0, 1.0), (3, 2.0), (5, -1.0)], 10.0);
        assert_eq!(constraint.variables(), vec![0, 3, 5]);
    }

    #[test]
    fn test_linear_constraint_out_of_bounds() {
        // Constraint references variable 5, but values only has 3 elements
        let constraint = LinearConstraint::le(vec![(5, 1.0)], 10.0);
        // Missing variable defaults to 0, so 0 <= 10 is satisfied
        assert!(constraint.is_satisfied(&[1, 2, 3]));
    }

    // ============================================================
    // ObjectiveSense tests
    // ============================================================

    #[test]
    fn test_objective_sense_from_energy_mode() {
        assert_eq!(
            ObjectiveSense::from(EnergyMode::LargerSizeIsBetter),
            ObjectiveSense::Maximize
        );
        assert_eq!(
            ObjectiveSense::from(EnergyMode::SmallerSizeIsBetter),
            ObjectiveSense::Minimize
        );
    }

    #[test]
    fn test_energy_mode_from_objective_sense() {
        assert_eq!(
            EnergyMode::from(ObjectiveSense::Maximize),
            EnergyMode::LargerSizeIsBetter
        );
        assert_eq!(
            EnergyMode::from(ObjectiveSense::Minimize),
            EnergyMode::SmallerSizeIsBetter
        );
    }

    // ============================================================
    // ILP tests
    // ============================================================

    #[test]
    fn test_ilp_new() {
        let ilp = ILP::new(
            2,
            vec![VarBounds::binary(), VarBounds::binary()],
            vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0), (1, 2.0)],
            ObjectiveSense::Maximize,
        );
        assert_eq!(ilp.num_vars, 2);
        assert_eq!(ilp.bounds.len(), 2);
        assert_eq!(ilp.constraints.len(), 1);
        assert_eq!(ilp.objective.len(), 2);
        assert_eq!(ilp.sense, ObjectiveSense::Maximize);
    }

    #[test]
    #[should_panic(expected = "bounds length must match num_vars")]
    fn test_ilp_new_mismatched_bounds() {
        ILP::new(
            3,
            vec![VarBounds::binary(), VarBounds::binary()], // Only 2 bounds for 3 vars
            vec![],
            vec![],
            ObjectiveSense::Minimize,
        );
    }

    #[test]
    fn test_ilp_binary() {
        let ilp = ILP::binary(
            3,
            vec![],
            vec![(0, 1.0), (1, 1.0), (2, 1.0)],
            ObjectiveSense::Minimize,
        );
        assert_eq!(ilp.num_vars, 3);
        assert!(ilp.bounds.iter().all(|b| *b == VarBounds::binary()));
    }

    #[test]
    fn test_ilp_empty() {
        let ilp = ILP::empty();
        assert_eq!(ilp.num_vars, 0);
        assert!(ilp.bounds.is_empty());
        assert!(ilp.constraints.is_empty());
        assert!(ilp.objective.is_empty());
    }

    #[test]
    fn test_ilp_evaluate_objective() {
        let ilp = ILP::binary(
            3,
            vec![],
            vec![(0, 2.0), (1, 3.0), (2, -1.0)],
            ObjectiveSense::Maximize,
        );
        // 2*1 + 3*1 + (-1)*0 = 5
        assert!((ilp.evaluate_objective(&[1, 1, 0]) - 5.0).abs() < 1e-9);
        // 2*0 + 3*0 + (-1)*1 = -1
        assert!((ilp.evaluate_objective(&[0, 0, 1]) - (-1.0)).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_bounds_satisfied() {
        let ilp = ILP::new(
            2,
            vec![VarBounds::bounded(0, 5), VarBounds::bounded(-2, 2)],
            vec![],
            vec![],
            ObjectiveSense::Minimize,
        );
        assert!(ilp.bounds_satisfied(&[0, 0]));
        assert!(ilp.bounds_satisfied(&[5, 2]));
        assert!(ilp.bounds_satisfied(&[3, -2]));
        assert!(!ilp.bounds_satisfied(&[6, 0])); // x0 > 5
        assert!(!ilp.bounds_satisfied(&[0, 3])); // x1 > 2
        assert!(!ilp.bounds_satisfied(&[0])); // Wrong length
    }

    #[test]
    fn test_ilp_constraints_satisfied() {
        let ilp = ILP::binary(
            3,
            vec![
                LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0), // x0 + x1 <= 1
                LinearConstraint::ge(vec![(2, 1.0)], 0.0),           // x2 >= 0
            ],
            vec![],
            ObjectiveSense::Minimize,
        );
        assert!(ilp.constraints_satisfied(&[0, 0, 1]));
        assert!(ilp.constraints_satisfied(&[1, 0, 0]));
        assert!(ilp.constraints_satisfied(&[0, 1, 1]));
        assert!(!ilp.constraints_satisfied(&[1, 1, 0])); // x0 + x1 = 2 > 1
    }

    #[test]
    fn test_ilp_is_feasible() {
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Maximize,
        );
        assert!(ilp.is_feasible(&[0, 0]));
        assert!(ilp.is_feasible(&[1, 0]));
        assert!(ilp.is_feasible(&[0, 1]));
        assert!(!ilp.is_feasible(&[1, 1])); // Constraint violated
        assert!(!ilp.is_feasible(&[2, 0])); // Bounds violated
    }

    // ============================================================
    // Problem trait tests
    // ============================================================

    #[test]
    fn test_ilp_num_variables() {
        let ilp = ILP::binary(5, vec![], vec![], ObjectiveSense::Minimize);
        assert_eq!(ilp.num_variables(), 5);
    }

    #[test]
    fn test_ilp_num_flavors_binary() {
        let ilp = ILP::binary(3, vec![], vec![], ObjectiveSense::Minimize);
        assert_eq!(ilp.num_flavors(), 2);
    }

    #[test]
    fn test_ilp_num_flavors_mixed() {
        let ilp = ILP::new(
            3,
            vec![
                VarBounds::binary(),
                VarBounds::bounded(0, 5),
                VarBounds::bounded(-1, 1),
            ],
            vec![],
            vec![],
            ObjectiveSense::Minimize,
        );
        assert_eq!(ilp.num_flavors(), 6); // Max is 6 (from 0-5)
    }

    #[test]
    fn test_ilp_num_flavors_unbounded() {
        let ilp = ILP::new(
            2,
            vec![VarBounds::binary(), VarBounds::unbounded()],
            vec![],
            vec![],
            ObjectiveSense::Minimize,
        );
        assert_eq!(ilp.num_flavors(), usize::MAX);
    }

    #[test]
    fn test_ilp_num_flavors_empty() {
        let ilp = ILP::empty();
        assert_eq!(ilp.num_flavors(), 2); // Default when empty
    }

    #[test]
    fn test_ilp_problem_size() {
        let ilp = ILP::binary(
            4,
            vec![
                LinearConstraint::le(vec![(0, 1.0)], 1.0),
                LinearConstraint::le(vec![(1, 1.0)], 1.0),
            ],
            vec![],
            ObjectiveSense::Minimize,
        );
        let size = ilp.problem_size();
        assert_eq!(size.get("num_vars"), Some(4));
        assert_eq!(size.get("num_constraints"), Some(2));
    }

    #[test]
    fn test_ilp_energy_mode() {
        let max_ilp = ILP::binary(2, vec![], vec![], ObjectiveSense::Maximize);
        let min_ilp = ILP::binary(2, vec![], vec![], ObjectiveSense::Minimize);

        assert!(max_ilp.energy_mode().is_maximization());
        assert!(min_ilp.energy_mode().is_minimization());
    }

    #[test]
    fn test_ilp_solution_size_valid() {
        // Maximize x0 + 2*x1 subject to x0 + x1 <= 1
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0), (1, 2.0)],
            ObjectiveSense::Maximize,
        );

        // Config [0, 1] means x0=0, x1=1 => obj = 2, valid
        let sol = ilp.solution_size(&[0, 1]);
        assert!(sol.is_valid);
        assert!((sol.size - 2.0).abs() < 1e-9);

        // Config [1, 0] means x0=1, x1=0 => obj = 1, valid
        let sol = ilp.solution_size(&[1, 0]);
        assert!(sol.is_valid);
        assert!((sol.size - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_solution_size_invalid() {
        // x0 + x1 <= 1
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0), (1, 2.0)],
            ObjectiveSense::Maximize,
        );

        // Config [1, 1] means x0=1, x1=1 => obj = 3, but invalid (1+1 > 1)
        let sol = ilp.solution_size(&[1, 1]);
        assert!(!sol.is_valid);
        assert!((sol.size - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_solution_size_with_offset_bounds() {
        // Variables with non-zero lower bounds
        let ilp = ILP::new(
            2,
            vec![VarBounds::bounded(1, 3), VarBounds::bounded(-1, 1)],
            vec![],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Maximize,
        );

        // Config [0, 0] maps to x0=1, x1=-1 => obj = 0
        let sol = ilp.solution_size(&[0, 0]);
        assert!(sol.is_valid);
        assert!((sol.size - 0.0).abs() < 1e-9);

        // Config [2, 2] maps to x0=3, x1=1 => obj = 4
        let sol = ilp.solution_size(&[2, 2]);
        assert!(sol.is_valid);
        assert!((sol.size - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_ilp_brute_force_maximization() {
        // Maximize x0 + 2*x1 subject to x0 + x1 <= 1, x0, x1 binary
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0), (1, 2.0)],
            ObjectiveSense::Maximize,
        );

        let solver = BruteForce::new();
        let solutions = solver.find_best(&ilp);

        // Optimal: x1=1, x0=0 => objective = 2
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1]);
    }

    #[test]
    fn test_ilp_brute_force_minimization() {
        // Minimize x0 + x1 subject to x0 + x1 >= 1, x0, x1 binary
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Minimize,
        );

        let solver = BruteForce::new();
        let solutions = solver.find_best(&ilp);

        // Optimal: x0=1,x1=0 or x0=0,x1=1 => objective = 1
        assert_eq!(solutions.len(), 2);
        for sol in &solutions {
            let size = ilp.solution_size(sol);
            assert!(size.is_valid);
            assert!((size.size - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn test_ilp_brute_force_no_feasible() {
        // x0 >= 1 AND x0 <= 0 (infeasible)
        let ilp = ILP::binary(
            1,
            vec![
                LinearConstraint::ge(vec![(0, 1.0)], 1.0),
                LinearConstraint::le(vec![(0, 1.0)], 0.0),
            ],
            vec![(0, 1.0)],
            ObjectiveSense::Minimize,
        );

        let solver = BruteForce::new();
        let solutions = solver.find_best(&ilp);

        // No feasible solutions
        assert!(solutions.is_empty());
    }

    #[test]
    fn test_ilp_unconstrained() {
        // Maximize x0 + x1, no constraints, binary vars
        let ilp = ILP::binary(
            2,
            vec![],
            vec![(0, 1.0), (1, 1.0)],
            ObjectiveSense::Maximize,
        );

        let solver = BruteForce::new();
        let solutions = solver.find_best(&ilp);

        // Optimal: both = 1
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 1]);
    }

    #[test]
    fn test_ilp_equality_constraint() {
        // Minimize x0 subject to x0 + x1 == 1, binary vars
        let ilp = ILP::binary(
            2,
            vec![LinearConstraint::eq(vec![(0, 1.0), (1, 1.0)], 1.0)],
            vec![(0, 1.0)],
            ObjectiveSense::Minimize,
        );

        let solver = BruteForce::new();
        let solutions = solver.find_best(&ilp);

        // Optimal: x0=0, x1=1 => objective = 0
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1]);
    }

    #[test]
    fn test_ilp_multiple_constraints() {
        // Maximize x0 + x1 + x2 subject to:
        //   x0 + x1 <= 1
        //   x1 + x2 <= 1
        // Binary vars
        let ilp = ILP::binary(
            3,
            vec![
                LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
                LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
            ],
            vec![(0, 1.0), (1, 1.0), (2, 1.0)],
            ObjectiveSense::Maximize,
        );

        let solver = BruteForce::new();
        let solutions = solver.find_best(&ilp);

        // Optimal: x0=1, x1=0, x2=1 => objective = 2
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 0, 1]);
    }

    #[test]
    fn test_ilp_config_to_values() {
        let ilp = ILP::new(
            3,
            vec![
                VarBounds::bounded(0, 2),  // 0,1,2
                VarBounds::bounded(-1, 1), // -1,0,1
                VarBounds::bounded(5, 7),  // 5,6,7
            ],
            vec![],
            vec![],
            ObjectiveSense::Minimize,
        );

        // Config [0,0,0] => [0, -1, 5]
        assert_eq!(ilp.config_to_values(&[0, 0, 0]), vec![0, -1, 5]);
        // Config [2,2,2] => [2, 1, 7]
        assert_eq!(ilp.config_to_values(&[2, 2, 2]), vec![2, 1, 7]);
        // Config [1,1,1] => [1, 0, 6]
        assert_eq!(ilp.config_to_values(&[1, 1, 1]), vec![1, 0, 6]);
    }

    #[test]
    fn test_ilp_variant() {
        let v = ILP::variant();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], ("graph", "SimpleGraph"));
        assert_eq!(v[1], ("weight", "f64"));
    }
}
