//! Integer Linear Programming (ILP) intermediate representation.
//!
//! ILP stores integer variables with explicit possibly-unbounded intervals,
//! sparse exact-integer linear constraints, and a finite floating-point
//! objective. The type parameter is a static certificate for either an
//! all-binary model (`bool`) or a general integer model (`i64`).

use crate::registry::{ConstructionError, FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::traits::{EvaluationError, Problem};
use crate::types::{i64_to_exact_f64, Extremum, WeightElement};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ILP",
        display_name: "ILP",
        aliases: &[],
        dimensions: &[VariantDimension::new("variable", "bool", &["bool", "i64"])],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Optimize a linear objective over bounded or unbounded integer variables",
        fields: &[
            FieldInfo { name: "variables", type_name: "Vec<IntegerVariable>", description: "Integer variable bounds; null means an unbounded side" },
            FieldInfo { name: "constraints", type_name: "Vec<LinearConstraint>", description: "Sparse exact linear constraints" },
            FieldInfo { name: "objective", type_name: "Vec<(usize, f64)>", description: "Sparse finite objective coefficients" },
            FieldInfo { name: "sense", type_name: "ObjectiveSense", description: "Optimization direction" },
        ],
    }
}

/// Static certificate for a homogeneous ILP variable domain.
pub trait VariableDomain: 'static + Clone + Debug + Send + Sync {
    /// Name used by the registered variant dimension.
    const NAME: &'static str;

    /// Default stored variable used by homogeneous formulations.
    fn default_variable() -> IntegerVariable;

    /// Validate that stored bounds satisfy this static certificate.
    fn validate_variables(variables: &[IntegerVariable]) -> Result<(), ConstructionError>;
}

impl VariableDomain for bool {
    const NAME: &'static str = "bool";

    fn default_variable() -> IntegerVariable {
        IntegerVariable::binary()
    }

    fn validate_variables(variables: &[IntegerVariable]) -> Result<(), ConstructionError> {
        if variables
            .iter()
            .any(|variable| variable.lower_bound != Some(0) || variable.upper_bound != Some(1))
        {
            return Err(ConstructionError::Conversion(
                "binary ILP variables must have bounds [0, 1]".into(),
            ));
        }
        Ok(())
    }
}

impl VariableDomain for i64 {
    const NAME: &'static str = "i64";

    fn default_variable() -> IntegerVariable {
        IntegerVariable::nonnegative()
    }

    fn validate_variables(_variables: &[IntegerVariable]) -> Result<(), ConstructionError> {
        Ok(())
    }
}

/// Bounds of one mathematical integer variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct IntegerVariable {
    lower_bound: Option<i64>,
    upper_bound: Option<i64>,
}

impl<'de> Deserialize<'de> for IntegerVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Bounds {
            lower_bound: Option<i64>,
            upper_bound: Option<i64>,
        }

        let bounds = Bounds::deserialize(deserializer)?;
        Self::new(bounds.lower_bound, bounds.upper_bound).map_err(serde::de::Error::custom)
    }
}

impl IntegerVariable {
    /// Construct an integer variable. `None` denotes the corresponding
    /// infinite bound.
    pub fn new(
        lower_bound: Option<i64>,
        upper_bound: Option<i64>,
    ) -> Result<Self, ConstructionError> {
        if lower_bound
            .zip(upper_bound)
            .is_some_and(|(lower, upper)| lower > upper)
        {
            return Err(ConstructionError::Conversion(
                "integer variable lower bound exceeds its upper bound".into(),
            ));
        }
        Ok(Self {
            lower_bound,
            upper_bound,
        })
    }

    /// A binary integer variable in `[0, 1]`.
    pub const fn binary() -> Self {
        Self {
            lower_bound: Some(0),
            upper_bound: Some(1),
        }
    }

    /// A non-negative integer variable in `[0, +∞)`.
    pub const fn nonnegative() -> Self {
        Self {
            lower_bound: Some(0),
            upper_bound: None,
        }
    }

    /// A free integer variable in `(-∞, +∞)`.
    pub const fn free() -> Self {
        Self {
            lower_bound: None,
            upper_bound: None,
        }
    }

    /// Finite lower bound, or `None` for negative infinity.
    pub const fn lower_bound(self) -> Option<i64> {
        self.lower_bound
    }

    /// Finite upper bound, or `None` for positive infinity.
    pub const fn upper_bound(self) -> Option<i64> {
        self.upper_bound
    }

    /// Whether a mathematical value belongs to this interval.
    pub fn contains(self, value: i64) -> bool {
        self.lower_bound.is_none_or(|lower| value >= lower)
            && self.upper_bound.is_none_or(|upper| value <= upper)
    }
}

/// Comparison operator for a linear constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Comparison {
    /// Less than or equal (`<=`).
    Le,
    /// Greater than or equal (`>=`).
    Ge,
    /// Equal (`==`).
    Eq,
}

/// One sparse linear constraint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearConstraint {
    terms: Vec<(usize, i64)>,
    comparison: Comparison,
    rhs: i64,
}

impl LinearConstraint {
    fn new(terms: Vec<(usize, i64)>, comparison: Comparison, rhs: i64) -> Self {
        Self {
            terms,
            comparison,
            rhs,
        }
    }

    /// Create a less-than-or-equal constraint.
    pub fn le(terms: Vec<(usize, i64)>, rhs: i64) -> Self {
        Self::new(terms, Comparison::Le, rhs)
    }

    /// Create a greater-than-or-equal constraint.
    pub fn ge(terms: Vec<(usize, i64)>, rhs: i64) -> Self {
        Self::new(terms, Comparison::Ge, rhs)
    }

    /// Create an equality constraint.
    pub fn eq(terms: Vec<(usize, i64)>, rhs: i64) -> Self {
        Self::new(terms, Comparison::Eq, rhs)
    }

    /// Canonical sparse row terms.
    pub fn terms(&self) -> &[(usize, i64)] {
        &self.terms
    }

    /// Row comparison operator.
    pub const fn comparison(&self) -> Comparison {
        self.comparison
    }

    /// Row right-hand side.
    pub const fn rhs(&self) -> i64 {
        self.rhs
    }

    /// Evaluate the left-hand side exactly.
    pub fn evaluate_lhs(&self, values: &[i64]) -> Result<i64, EvaluationError> {
        self.terms
            .iter()
            .try_fold(0_i64, |sum, &(variable, coefficient)| {
                let value = values.get(variable).copied().ok_or_else(|| {
                    EvaluationError::InvalidConfiguration(format!(
                        "an ILP constraint references variable {variable}, but the assignment has {} values",
                        values.len()
                    ))
                })?;
                let product = coefficient.checked_mul(value).ok_or_else(|| {
                    EvaluationError::IntegerOverflow(
                        "multiplying a term in an ILP constraint".into(),
                    )
                })?;
                sum.checked_add(product).ok_or_else(|| {
                    EvaluationError::IntegerOverflow("summing an ILP constraint".into())
                })
            })
    }

    /// Check whether this row is satisfied.
    pub fn is_satisfied(&self, values: &[i64]) -> Result<bool, EvaluationError> {
        let lhs = self.evaluate_lhs(values)?;
        Ok(match self.comparison {
            Comparison::Le => lhs <= self.rhs,
            Comparison::Ge => lhs >= self.rhs,
            Comparison::Eq => lhs == self.rhs,
        })
    }

    /// Variable indices present in this row.
    pub fn variables(&self) -> impl Iterator<Item = usize> + '_ {
        self.terms.iter().map(|&(variable, _)| variable)
    }
}

/// Optimization direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectiveSense {
    /// Maximize the objective.
    Maximize,
    /// Minimize the objective.
    Minimize,
}

/// Integer Linear Programming model.
#[derive(Debug, Clone, Serialize)]
pub struct ILP<V: VariableDomain = bool> {
    variables: Vec<IntegerVariable>,
    constraints: Vec<LinearConstraint>,
    objective: Vec<(usize, f64)>,
    sense: ObjectiveSense,
    #[serde(skip)]
    marker: PhantomData<V>,
}

#[derive(Deserialize)]
struct ILPData {
    variables: Vec<IntegerVariable>,
    constraints: Vec<LinearConstraint>,
    objective: Vec<(usize, f64)>,
    sense: ObjectiveSense,
}

impl<'de, V: VariableDomain> Deserialize<'de> for ILP<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = ILPData::deserialize(deserializer)?;
        Self::with_variables(data.variables, data.constraints, data.objective, data.sense)
            .map_err(serde::de::Error::custom)
    }
}

impl<V: VariableDomain> ILP<V> {
    /// Construct a homogeneous model using the domain certificate's standard
    /// variable interval: binary `[0, 1]` or integer `[0, +∞)`.
    pub fn new(
        num_variables: usize,
        constraints: Vec<LinearConstraint>,
        objective: Vec<(usize, f64)>,
        sense: ObjectiveSense,
    ) -> Result<Self, ConstructionError> {
        Self::with_variables(
            vec![V::default_variable(); num_variables],
            constraints,
            objective,
            sense,
        )
    }

    /// Construct a model with explicit possibly-unbounded variable intervals.
    pub fn with_variables(
        variables: Vec<IntegerVariable>,
        constraints: Vec<LinearConstraint>,
        objective: Vec<(usize, f64)>,
        sense: ObjectiveSense,
    ) -> Result<Self, ConstructionError> {
        V::validate_variables(&variables)?;
        let num_variables = variables.len();
        let constraints = constraints
            .into_iter()
            .enumerate()
            .map(|(index, constraint)| normalize_constraint(constraint, num_variables, index))
            .collect::<Result<_, _>>()?;
        let objective = normalize_objective(objective, num_variables)?;
        Ok(Self {
            variables,
            constraints,
            objective,
            sense,
            marker: PhantomData,
        })
    }

    /// Empty model.
    pub fn empty() -> Self {
        Self::new(0, vec![], vec![], ObjectiveSense::Minimize)
            .expect("the empty ILP satisfies all construction invariants")
    }

    /// Stored variables and their bounds.
    pub fn variables(&self) -> &[IntegerVariable] {
        &self.variables
    }

    /// Canonical sparse constraints.
    pub fn constraints(&self) -> &[LinearConstraint] {
        &self.constraints
    }

    /// Canonical sparse objective.
    pub fn objective(&self) -> &[(usize, f64)] {
        &self.objective
    }

    /// Optimization direction.
    pub const fn sense(&self) -> ObjectiveSense {
        self.sense
    }

    /// Number of variables.
    pub fn num_variables(&self) -> usize {
        self.variables.len()
    }

    /// Canonical size getter alias.
    pub fn num_vars(&self) -> usize {
        self.num_variables()
    }

    /// Number of constraints.
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    /// Number of non-zero row coefficients.
    pub fn num_nonzeros(&self) -> usize {
        self.constraints
            .iter()
            .map(|constraint| constraint.terms.len())
            .sum()
    }

    /// Evaluate the finite floating-point objective.
    pub fn evaluate_objective(&self, values: &[i64]) -> Result<f64, EvaluationError> {
        self.objective
            .iter()
            .try_fold(0.0_f64, |sum, &(variable, coefficient)| {
                let integer = values.get(variable).copied().ok_or_else(|| {
                    EvaluationError::InvalidConfiguration(format!(
                        "the ILP objective references variable {variable}, but the assignment has {} values",
                        values.len()
                    ))
                })?;
                let value = i64_to_exact_f64(integer).map_err(|_| {
                    EvaluationError::InexactFloatConversion(
                        "transporting an integer variable into the ILP objective".into(),
                    )
                })?;
                let product = <f64 as WeightElement>::checked_mul_sum(
                    coefficient,
                    value,
                    "multiplying a term in the ILP objective",
                )?;
                <f64 as WeightElement>::checked_add_to_sum(
                    sum,
                    product,
                    "summing the ILP objective",
                )
            })
    }

    /// Check stored variable intervals and all rows.
    pub fn is_feasible(&self, values: &[i64]) -> Result<bool, EvaluationError> {
        if values.len() != self.variables.len() {
            return Err(EvaluationError::InvalidConfiguration(
                "variable assignment length does not match the ILP".into(),
            ));
        }
        if self
            .variables
            .iter()
            .zip(values)
            .any(|(&variable, &value)| !variable.contains(value))
        {
            return Ok(false);
        }
        for constraint in &self.constraints {
            if !constraint.is_satisfied(values)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

fn normalize_constraint(
    constraint: LinearConstraint,
    num_variables: usize,
    constraint_index: usize,
) -> Result<LinearConstraint, ConstructionError> {
    let mut terms = constraint.terms;
    for &(variable, _) in &terms {
        if variable >= num_variables {
            return Err(ConstructionError::Conversion(format!(
                "ILP constraint {constraint_index} references variable {variable}, but the model has {num_variables} variables"
            )));
        }
    }
    terms.sort_by_key(|&(variable, _)| variable);
    let mut normalized: Vec<(usize, i64)> = Vec::with_capacity(terms.len());
    for (variable, coefficient) in terms {
        if let Some((previous_variable, previous_coefficient)) = normalized.last_mut() {
            if *previous_variable == variable {
                *previous_coefficient =
                    previous_coefficient.checked_add(coefficient).ok_or_else(|| {
                        ConstructionError::IntegerOverflow(format!(
                            "merging duplicate variable {variable} in ILP constraint {constraint_index}"
                        ))
                    })?;
                continue;
            }
        }
        normalized.push((variable, coefficient));
    }
    normalized.retain(|&(_, coefficient)| coefficient != 0);
    Ok(LinearConstraint::new(
        normalized,
        constraint.comparison,
        constraint.rhs,
    ))
}

fn normalize_objective(
    mut objective: Vec<(usize, f64)>,
    num_variables: usize,
) -> Result<Vec<(usize, f64)>, ConstructionError> {
    for &(variable, coefficient) in &objective {
        if variable >= num_variables {
            return Err(ConstructionError::Conversion(format!(
                "ILP objective references variable {variable}, but the model has {num_variables} variables"
            )));
        }
        if !coefficient.is_finite() {
            return Err(ConstructionError::NonFiniteFloat(format!(
                "objective coefficient of variable {variable}"
            )));
        }
    }
    objective.sort_by_key(|&(variable, _)| variable);
    let mut normalized: Vec<(usize, f64)> = Vec::with_capacity(objective.len());
    for (variable, coefficient) in objective {
        if let Some((previous_variable, previous_coefficient)) = normalized.last_mut() {
            if *previous_variable == variable {
                let sum = *previous_coefficient + coefficient;
                if !sum.is_finite() {
                    return Err(ConstructionError::NonFiniteFloat(format!(
                        "merged objective coefficient of variable {variable}"
                    )));
                }
                *previous_coefficient = sum;
                continue;
            }
        }
        normalized.push((variable, coefficient));
    }
    normalized.retain(|&(_, coefficient)| coefficient != 0.0);
    Ok(normalized)
}

impl<V: VariableDomain> Problem for ILP<V> {
    const NAME: &'static str = "ILP";
    type Solution = Vec<i64>;
    type Value = Extremum<f64>;

    crate::problem_parameters![
        ("num_constraints", num_constraints),
        ("num_nonzeros", num_nonzeros),
        ("num_vars", num_vars),
    ];

    fn evaluate(&self, solution: &Self::Solution) -> Result<Self::Value, EvaluationError> {
        if !self.is_feasible(solution)? {
            return Ok(match self.sense {
                ObjectiveSense::Maximize => Extremum::maximize(None),
                ObjectiveSense::Minimize => Extremum::minimize(None),
            });
        }
        let objective = self.evaluate_objective(solution)?;
        Ok(match self.sense {
            ObjectiveSense::Maximize => Extremum::maximize(Some(objective)),
            ObjectiveSense::Minimize => Extremum::minimize(Some(objective)),
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("variable", V::NAME)]
    }
}

crate::declare_variants! {
    default ILP<bool> => "2^num_vars",
    ILP<i64> => "num_vars^num_vars",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "ilp",
        instance: Box::new(
            ILP::<i64>::new(
                2,
                vec![
                    LinearConstraint::le(vec![(0, 1), (1, 1)], 5),
                    LinearConstraint::le(vec![(0, 4), (1, 7)], 28),
                ],
                vec![(0, -5.0), (1, -6.0)],
                ObjectiveSense::Minimize,
            )
            .expect("canonical ILP construction must succeed"),
        ),
        optimal_config: serde_json::json!(vec![3, 2]),
        optimal_value: serde_json::json!({
            "sense": "Minimize",
            "value": -27.0,
        }),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/ilp.rs"]
mod tests;
