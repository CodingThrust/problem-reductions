//! Integer Linear Programming (ILP) problem implementation.
//!
//! ILP optimizes a linear objective over integer variables subject to linear constraints.
//! This is a fundamental "hub" problem that many other NP-hard problems can be reduced to.
//!
//! The type parameter `V` determines the variable domain:
//! - `ILP<bool>`: binary variables (0 or 1)
//! - `ILP<i32>`: non-negative integer variables (0..2^31-1)

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ILP",
        display_name: "ILP",
        aliases: &[],
        dimensions: &[VariantDimension::new("variable", "bool", &["bool", "i32"])],
        module_path: module_path!(),
        description: "Optimize linear objective subject to linear constraints",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of integer variables" },
            FieldInfo { name: "constraints", type_name: "Vec<LinearConstraint>", description: "Linear constraints" },
            FieldInfo { name: "objective", type_name: "Vec<(usize, f64)>", description: "Sparse objective coefficients" },
            FieldInfo { name: "sense", type_name: "ObjectiveSense", description: "Optimization direction" },
        ],
    }
}

/// Sealed trait for ILP variable domains.
///
/// `bool` = binary variables (0 or 1), `i32` = non-negative integers (0..2^31-1).
pub trait VariableDomain: 'static + Clone + std::fmt::Debug + Send + Sync {
    /// Number of possible values per variable (used by `dims()`).
    const DIMS_PER_VAR: usize;
    /// Name for the variant system (e.g., "bool", "i32").
    const NAME: &'static str;
}

impl VariableDomain for bool {
    const DIMS_PER_VAR: usize = 2;
    const NAME: &'static str = "bool";
}

impl VariableDomain for i32 {
    const DIMS_PER_VAR: usize = (i32::MAX as usize) + 1;
    const NAME: &'static str = "i32";
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
    pub(crate) fn new(terms: Vec<(usize, f64)>, cmp: Comparison, rhs: f64) -> Self {
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

/// Integer Linear Programming (ILP) problem.
///
/// An ILP consists of:
/// - A set of integer variables with a domain determined by `V`
/// - Linear constraints on those variables
/// - A linear objective function to optimize
/// - An optimization sense (maximize or minimize)
///
/// # Type Parameter
///
/// - `V = bool`: binary variables (0 or 1)
/// - `V = i32`: non-negative integer variables
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::{ILP, LinearConstraint, ObjectiveSense};
/// use problemreductions::Problem;
///
/// // Create a simple binary ILP: maximize x0 + 2*x1
/// // subject to: x0 + x1 <= 3, x0, x1 binary
/// let ilp = ILP::<bool>::new(
///     2,
///     vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 3.0)],
///     vec![(0, 1.0), (1, 2.0)],
///     ObjectiveSense::Maximize,
/// );
///
/// assert_eq!(ilp.num_variables(), 2);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct ILP<V: VariableDomain = bool> {
    /// Number of variables.
    pub num_vars: usize,
    /// Linear constraints.
    pub constraints: Vec<LinearConstraint>,
    /// Sparse objective coefficients: (var_index, coefficient).
    pub objective: Vec<(usize, f64)>,
    /// Optimization direction.
    pub sense: ObjectiveSense,
    #[serde(skip)]
    _marker: PhantomData<V>,
}

impl<V: VariableDomain> ILP<V> {
    /// Create a new ILP problem.
    pub fn new(
        num_vars: usize,
        constraints: Vec<LinearConstraint>,
        objective: Vec<(usize, f64)>,
        sense: ObjectiveSense,
    ) -> Self {
        Self {
            num_vars,
            constraints,
            objective,
            sense,
            _marker: PhantomData,
        }
    }

    /// Create an empty ILP with no variables.
    pub fn empty() -> Self {
        Self::new(0, vec![], vec![], ObjectiveSense::Minimize)
    }

    /// Evaluate the objective function for given variable values.
    pub fn evaluate_objective(&self, values: &[i64]) -> f64 {
        self.objective
            .iter()
            .map(|&(var, coef)| coef * values.get(var).copied().unwrap_or(0) as f64)
            .sum()
    }

    /// Check if all constraints are satisfied for given variable values.
    pub fn constraints_satisfied(&self, values: &[i64]) -> bool {
        self.constraints.iter().all(|c| c.is_satisfied(values))
    }

    /// Check if a solution is feasible (satisfies constraints).
    pub fn is_feasible(&self, values: &[i64]) -> bool {
        values.len() == self.num_vars && self.constraints_satisfied(values)
    }

    /// Convert a configuration (Vec<usize>) to integer values (Vec<i64>).
    /// For bool: config 0→0, 1→1. For i32: config index = value.
    fn config_to_values(&self, config: &[usize]) -> Vec<i64> {
        config.iter().map(|&c| c as i64).collect()
    }

    /// Get the number of variables.
    pub fn num_variables(&self) -> usize {
        self.num_vars
    }

    /// Get the number of variables.
    pub fn num_vars(&self) -> usize {
        self.num_variables()
    }

    /// Get the number of constraints.
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }
}

impl<V: VariableDomain> Problem for ILP<V> {
    const NAME: &'static str = "ILP";
    type Metric = SolutionSize<f64>;

    fn dims(&self) -> Vec<usize> {
        vec![V::DIMS_PER_VAR; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<f64> {
        let values = self.config_to_values(config);
        if !self.is_feasible(&values) {
            return SolutionSize::Invalid;
        }
        SolutionSize::Valid(self.evaluate_objective(&values))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("variable", V::NAME)]
    }
}

impl<V: VariableDomain> OptimizationProblem for ILP<V> {
    type Value = f64;

    fn direction(&self) -> Direction {
        match self.sense {
            ObjectiveSense::Maximize => Direction::Maximize,
            ObjectiveSense::Minimize => Direction::Minimize,
        }
    }
}

crate::declare_variants! {
    default opt ILP<bool> => "2^num_vars",
    opt ILP<i32> => "num_vars^num_vars",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "ilp_i32",
        build: || {
            let problem = ILP::<i32>::new(
                2,
                vec![
                    LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 5.0),
                    LinearConstraint::le(vec![(0, 4.0), (1, 7.0)], 28.0),
                ],
                vec![(0, -5.0), (1, -6.0)],
                ObjectiveSense::Minimize,
            );
            crate::example_db::specs::explicit_example(problem, vec![vec![0, 4]], vec![vec![3, 2]])
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/ilp.rs"]
mod tests;
