//! Symbolic size transformations carried by reduction rules.

use crate::expr::{AlgebraicAnalysis, Expr, ExprNode, ExprNodeId, Symbol};
use crate::growth::{Growth, GrowthPrecision};
use crate::types::ProblemSize;
use num_bigint::{BigInt, BigUint, Sign};
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// What one reduction rule promises about all of its declared size formulas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SizeRelation {
    Exact,
    UpperBound,
}

impl SizeRelation {
    fn compose(self, next: Self) -> Self {
        if self == Self::Exact && next == Self::Exact {
            Self::Exact
        } else {
            Self::UpperBound
        }
    }
}

/// Arbitrary-precision non-negative values for problem-size fields.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct SizeValues {
    components: Vec<(Box<str>, BigUint)>,
}

impl SizeValues {
    pub fn new<I, N, V>(components: I) -> Self
    where
        I: IntoIterator<Item = (N, V)>,
        N: Into<Box<str>>,
        V: Into<BigUint>,
    {
        Self {
            components: components
                .into_iter()
                .map(|(name, value)| (name.into(), value.into()))
                .collect(),
        }
    }

    pub fn from_problem_size(size: &ProblemSize) -> Self {
        Self::new(
            size.components
                .iter()
                .map(|(name, value)| (name.as_str(), BigUint::from(*value))),
        )
    }

    pub fn get(&self, name: &str) -> Option<&BigUint> {
        self.components
            .iter()
            .find(|(field, _)| field.as_ref() == name)
            .map(|(_, value)| value)
    }

    pub fn components(&self) -> impl Iterator<Item = (&str, &BigUint)> {
        self.components
            .iter()
            .map(|(name, value)| (name.as_ref(), value))
    }

    pub fn try_to_problem_size(&self) -> Result<ProblemSize, SizeTransformError> {
        let mut values = Vec::with_capacity(self.components.len());
        for (name, value) in &self.components {
            let value =
                usize::try_from(value).map_err(|_| SizeTransformError::OutputOutOfRange {
                    field: name.clone(),
                    value: value.clone(),
                })?;
            values.push((name.as_ref(), value));
        }
        Ok(ProblemSize::new(values))
    }
}

/// Concrete size information whose relation is never erased during propagation.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct EvaluatedSize {
    relation: SizeRelation,
    values: SizeValues,
}

/// Asymptotic projection of a size transform with its promise preserved.
#[derive(Clone, Debug, PartialEq)]
pub struct SizeGrowth {
    relation: SizeRelation,
    fields: Vec<(Box<str>, Growth)>,
}

impl SizeGrowth {
    pub fn relation(&self) -> SizeRelation {
        self.relation
    }

    pub fn fields(&self) -> impl Iterator<Item = (&str, &Growth)> {
        self.fields
            .iter()
            .map(|(name, growth)| (name.as_ref(), growth))
    }

    pub fn get(&self, field: &str) -> Option<&Growth> {
        self.fields
            .iter()
            .find(|(name, _)| name.as_ref() == field)
            .map(|(_, growth)| growth)
    }
}

/// Return whether `left` is no larger in every concrete size field and smaller
/// in at least one field.
pub fn problem_size_dominates(left: &ProblemSize, right: &ProblemSize) -> bool {
    left.components.len() == right.components.len()
        && left
            .components
            .iter()
            .all(|(name, value)| right.get(name).is_some_and(|other| *value <= other))
        && left
            .components
            .iter()
            .any(|(name, value)| right.get(name).is_some_and(|other| *value < other))
}

/// Return whether the available bounds prove that `left` has no faster growth
/// in every symbolic size field and strictly slower growth in at least one.
///
/// The right-hand path must be exact with tight growth projections because it
/// is the path being removed. The left-hand path may itself be an upper bound:
/// proving that upper bound smaller than the right-hand tight class is enough.
pub fn size_growth_dominates(left: &SizeGrowth, right: &SizeGrowth) -> bool {
    if left.fields.len() != right.fields.len() {
        return false;
    }
    if right.relation != SizeRelation::Exact {
        return false;
    }
    let mut strictly_smaller = false;
    for (name, growth) in left.fields() {
        let Some(other) = right.get(name) else {
            return false;
        };
        if growth.failures().is_some() || other.precision() != Some(GrowthPrecision::Tight) {
            return false;
        }
        let left_at_most = other.bound_dominates(growth);
        if !left_at_most {
            return false;
        }
        strictly_smaller |= !growth.bound_dominates(other);
    }
    strictly_smaller
}

impl EvaluatedSize {
    pub fn exact(values: SizeValues) -> Self {
        Self {
            relation: SizeRelation::Exact,
            values,
        }
    }

    pub fn from_problem_size(size: &ProblemSize) -> Self {
        Self::exact(SizeValues::from_problem_size(size))
    }

    pub fn relation(&self) -> SizeRelation {
        self.relation
    }

    pub fn values(&self) -> &SizeValues {
        &self.values
    }
}

/// One rule-level symbolic transformation. Its relation applies to every formula.
#[derive(Clone, Debug)]
pub struct SizeTransform {
    edge: Box<str>,
    relation: SizeRelation,
    fields: Vec<SizeField>,
    analysis: AlgebraicAnalysis,
}

#[derive(Clone, Debug)]
struct SizeField {
    name: Box<str>,
    expression: Expr,
    plan: Plan,
    monotone: bool,
}

#[derive(Clone, Debug)]
struct Plan(Arc<PlanNode>);

#[derive(Debug)]
enum PlanNode {
    Const(BigRational),
    Var(Symbol),
    Add(Box<[Plan]>),
    Mul(Box<[Plan]>),
    Pow(Plan, BigInt),
}

impl Plan {
    fn identity(&self) -> usize {
        Arc::as_ptr(&self.0) as usize
    }
}

impl SizeTransform {
    pub fn new<I, N>(
        edge: impl Into<Box<str>>,
        relation: SizeRelation,
        fields: I,
    ) -> Result<Self, SizeTransformError>
    where
        I: IntoIterator<Item = (N, Expr)>,
        N: Into<Box<str>>,
    {
        let edge = edge.into();
        let mut names = HashSet::new();
        let mut raw_fields = Vec::new();
        for (name, expression) in fields {
            let name = name.into();
            if let Err(error) = Symbol::new(name.clone()) {
                return Err(SizeTransformError::InvalidTargetField {
                    edge,
                    field: name,
                    reason: error.to_string().into(),
                });
            }
            if !names.insert(name.clone()) {
                return Err(SizeTransformError::DuplicateTargetField { edge, field: name });
            }
            raw_fields.push((name, expression));
        }

        let expressions = raw_fields
            .iter()
            .map(|(_, expression)| expression)
            .collect::<Vec<_>>();
        let analysis = AlgebraicAnalysis::new(&expressions);
        let mut plans = HashMap::new();
        let fields = raw_fields
            .into_iter()
            .map(|(name, expression)| {
                let plan = compile(&expression, &analysis, &mut plans).map_err(|failure| {
                    validation_error(edge.clone(), name.clone(), expression.to_string(), failure)
                })?;
                let monotone = is_nonnegative_monotone(&expression, &analysis);
                if relation == SizeRelation::UpperBound && !monotone {
                    return Err(SizeTransformError::NonMonotoneUpperBound {
                        edge: edge.clone(),
                        field: name,
                        expression: expression.to_string().into(),
                    });
                }
                Ok(SizeField {
                    name,
                    expression,
                    plan,
                    monotone,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            edge,
            relation,
            fields,
            analysis,
        })
    }

    pub fn edge(&self) -> &str {
        &self.edge
    }

    pub fn relation(&self) -> SizeRelation {
        self.relation
    }

    pub fn expressions(&self) -> impl Iterator<Item = (&str, &Expr)> {
        self.fields
            .iter()
            .map(|field| (field.name.as_ref(), &field.expression))
    }

    pub fn get(&self, target_field: &str) -> Option<&Expr> {
        self.fields
            .iter()
            .find(|field| field.name.as_ref() == target_field)
            .map(|field| &field.expression)
    }

    pub fn evaluate(&self, input: &EvaluatedSize) -> Result<EvaluatedSize, SizeTransformError> {
        if input.relation == SizeRelation::UpperBound {
            if let Some(field) = self.fields.iter().find(|field| !field.monotone) {
                return Err(SizeTransformError::CannotPropagateUpperBound {
                    edge: self.edge.clone(),
                    field: field.name.clone(),
                    expression: field.expression.to_string().into(),
                });
            }
        }

        let relation = input.relation.compose(self.relation);
        let mut memo = HashMap::new();
        let mut output = Vec::with_capacity(self.fields.len());
        for field in &self.fields {
            let value =
                evaluate_plan(&field.plan, &input.values, &mut memo).map_err(|failure| {
                    evaluation_error(self.edge.clone(), field.name.clone(), failure)
                })?;
            if value.is_negative() {
                return Err(SizeTransformError::NegativeResult {
                    edge: self.edge.clone(),
                    field: field.name.clone(),
                    value,
                });
            }
            let value = if relation == SizeRelation::Exact {
                if !value.is_integer() {
                    return Err(SizeTransformError::NonIntegralResult {
                        edge: self.edge.clone(),
                        field: field.name.clone(),
                        value: value.to_string().into(),
                    });
                }
                value.to_integer().magnitude().clone()
            } else {
                ceil_nonnegative(&value)
            };
            output.push((field.name.clone(), value));
        }
        Ok(EvaluatedSize {
            relation,
            values: SizeValues { components: output },
        })
    }

    pub fn compose(
        &self,
        next: &SizeTransform,
        edge: impl Into<Box<str>>,
    ) -> Result<SizeTransform, SizeTransformError> {
        if self.relation == SizeRelation::UpperBound {
            if let Some(field) = next.fields.iter().find(|field| !field.monotone) {
                return Err(SizeTransformError::CannotPropagateUpperBound {
                    edge: next.edge.clone(),
                    field: field.name.clone(),
                    expression: field.expression.to_string().into(),
                });
            }
        }
        let edge = edge.into();
        let replacements: HashMap<&str, &Expr> = self.expressions().collect();
        let fields = next
            .fields
            .iter()
            .map(|field| {
                let expression = field
                    .expression
                    .substitute_complete(&replacements)
                    .map_err(|error| SizeTransformError::MissingCompositionInput {
                        edge: edge.clone(),
                        field: field.name.clone(),
                        input_fields: error.missing_variables().map(Box::<str>::from).collect(),
                    })?;
                Ok((field.name.clone(), expression))
            })
            .collect::<Result<Vec<_>, SizeTransformError>>()?;
        Self::new(edge, self.relation.compose(next.relation), fields)
    }

    pub fn project_growth(&self) -> SizeGrowth {
        let fields = self
            .fields
            .iter()
            .map(|field| {
                (
                    field.name.clone(),
                    Growth::from_analysis(&field.expression, &self.analysis),
                )
            })
            .collect();
        SizeGrowth {
            relation: self.relation,
            fields,
        }
    }
}

fn compile(
    expression: &Expr,
    analysis: &AlgebraicAnalysis,
    memo: &mut HashMap<ExprNodeId, Plan>,
) -> Result<Plan, ValidationFailure> {
    if let Some(plan) = memo.get(&expression.node_identity()) {
        return Ok(plan.clone());
    }
    let node = match expression.node() {
        ExprNode::Const(value) => PlanNode::Const(value.clone()),
        ExprNode::Var(symbol) => PlanNode::Var(symbol.clone()),
        ExprNode::Add(values) => PlanNode::Add(
            values
                .iter()
                .map(|value| compile(value, analysis, memo))
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
        ),
        ExprNode::Mul(values) => PlanNode::Mul(
            values
                .iter()
                .map(|value| compile(value, analysis, memo))
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
        ),
        ExprNode::Pow(base, exponent) => {
            let Some(exponent) = analysis.facts(exponent).exact_rational.as_ref() else {
                return Err(ValidationFailure::NonIntegralConstantExponent(
                    exponent.to_string().into(),
                ));
            };
            if !exponent.is_integer() {
                return Err(ValidationFailure::NonIntegralConstantExponent(
                    exponent.to_string().into(),
                ));
            }
            PlanNode::Pow(compile(base, analysis, memo)?, exponent.to_integer())
        }
        ExprNode::Exp(_) => return Err(ValidationFailure::UnsupportedOperator("exp")),
        ExprNode::Log(_) => return Err(ValidationFailure::UnsupportedOperator("log")),
        ExprNode::Factorial(_) => {
            return Err(ValidationFailure::UnsupportedOperator("factorial"));
        }
    };
    let plan = Plan(Arc::new(node));
    memo.insert(expression.node_identity(), plan.clone());
    Ok(plan)
}

fn is_nonnegative_monotone(expression: &Expr, analysis: &AlgebraicAnalysis) -> bool {
    if analysis
        .facts(expression)
        .exact_rational
        .as_ref()
        .is_some_and(|value| !value.is_negative())
    {
        return true;
    }
    match expression.node() {
        ExprNode::Const(value) => !value.is_negative(),
        ExprNode::Var(_) => true,
        ExprNode::Add(values) | ExprNode::Mul(values) => values
            .iter()
            .all(|value| is_nonnegative_monotone(value, analysis)),
        ExprNode::Pow(base, exponent) => {
            analysis
                .facts(exponent)
                .exact_rational
                .as_ref()
                .is_some_and(|exponent| exponent.is_integer() && !exponent.is_negative())
                && is_nonnegative_monotone(base, analysis)
        }
        ExprNode::Exp(_) | ExprNode::Log(_) | ExprNode::Factorial(_) => false,
    }
}

fn evaluate_plan(
    plan: &Plan,
    input: &SizeValues,
    memo: &mut HashMap<usize, BigRational>,
) -> Result<BigRational, EvaluationFailure> {
    if let Some(value) = memo.get(&plan.identity()) {
        return Ok(value.clone());
    }
    let value = match plan.0.as_ref() {
        PlanNode::Const(value) => value.clone(),
        PlanNode::Var(symbol) => BigRational::from_integer(BigInt::from(
            input
                .get(symbol.as_str())
                .ok_or_else(|| EvaluationFailure::MissingInputField(symbol.to_string().into()))?
                .clone(),
        )),
        PlanNode::Add(values) => values.iter().try_fold(BigRational::zero(), |sum, value| {
            Ok(sum + evaluate_plan(value, input, memo)?)
        })?,
        PlanNode::Mul(values) => values
            .iter()
            .try_fold(BigRational::one(), |product, value| {
                Ok(product * evaluate_plan(value, input, memo)?)
            })?,
        PlanNode::Pow(base, exponent) => {
            let base = evaluate_plan(base, input, memo)?;
            if exponent.sign() == Sign::Minus && base.is_zero() {
                return Err(EvaluationFailure::DivisionByZero);
            }
            pow_rational(base, exponent)
        }
    };
    memo.insert(plan.identity(), value.clone());
    Ok(value)
}

fn pow_rational(mut base: BigRational, exponent: &BigInt) -> BigRational {
    let negative = exponent.sign() == Sign::Minus;
    let mut exponent = exponent.magnitude().clone();
    let mut result = BigRational::one();
    while !exponent.is_zero() {
        if exponent.bit(0) {
            result *= &base;
        }
        exponent >>= 1usize;
        if !exponent.is_zero() {
            base = &base * &base;
        }
    }
    if negative {
        result.recip()
    } else {
        result
    }
}

fn ceil_nonnegative(value: &BigRational) -> BigUint {
    ((value.numer() + value.denom() - BigInt::one()) / value.denom())
        .magnitude()
        .clone()
}

#[derive(Debug)]
enum ValidationFailure {
    NonIntegralConstantExponent(Box<str>),
    UnsupportedOperator(&'static str),
}

#[derive(Debug)]
enum EvaluationFailure {
    MissingInputField(Box<str>),
    DivisionByZero,
}

fn validation_error(
    edge: Box<str>,
    field: Box<str>,
    expression: String,
    failure: ValidationFailure,
) -> SizeTransformError {
    match failure {
        ValidationFailure::NonIntegralConstantExponent(exponent) => {
            SizeTransformError::NonIntegralConstantExponent {
                edge,
                field,
                expression: expression.into(),
                exponent,
            }
        }
        ValidationFailure::UnsupportedOperator(operator) => {
            SizeTransformError::UnsupportedOperator {
                edge,
                field,
                expression: expression.into(),
                operator,
            }
        }
    }
}

fn evaluation_error(
    edge: Box<str>,
    field: Box<str>,
    failure: EvaluationFailure,
) -> SizeTransformError {
    match failure {
        EvaluationFailure::MissingInputField(input_field) => {
            SizeTransformError::MissingInputField {
                edge,
                field,
                input_field,
            }
        }
        EvaluationFailure::DivisionByZero => SizeTransformError::DivisionByZero { edge, field },
    }
}

/// Validation, composition, or evaluation failure for a [`SizeTransform`].
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SizeTransformError {
    #[error("reduction `{edge}` has invalid target size field `{field}`: {reason}")]
    InvalidTargetField {
        edge: Box<str>,
        field: Box<str>,
        reason: Box<str>,
    },
    #[error("reduction `{edge}` declares target size field `{field}` more than once")]
    DuplicateTargetField { edge: Box<str>, field: Box<str> },
    #[error("reduction `{edge}` target field `{field}` has non-integral constant exponent `{exponent}` in `{expression}`")]
    NonIntegralConstantExponent {
        edge: Box<str>,
        field: Box<str>,
        expression: Box<str>,
        exponent: Box<str>,
    },
    #[error("reduction `{edge}` target field `{field}` uses unsupported operator `{operator}` in `{expression}`")]
    UnsupportedOperator {
        edge: Box<str>,
        field: Box<str>,
        expression: Box<str>,
        operator: &'static str,
    },
    #[error("reduction `{edge}` target field `{field}` has a non-monotone upper-bound formula `{expression}`")]
    NonMonotoneUpperBound {
        edge: Box<str>,
        field: Box<str>,
        expression: Box<str>,
    },
    #[error("reduction `{edge}` target field `{field}` cannot propagate an upper bound through `{expression}`")]
    CannotPropagateUpperBound {
        edge: Box<str>,
        field: Box<str>,
        expression: Box<str>,
    },
    #[error(
        "reduction `{edge}` target field `{field}` is missing input size field `{input_field}`"
    )]
    MissingInputField {
        edge: Box<str>,
        field: Box<str>,
        input_field: Box<str>,
    },
    #[error(
        "reduction `{edge}` target field `{field}` is missing composition inputs {input_fields:?}"
    )]
    MissingCompositionInput {
        edge: Box<str>,
        field: Box<str>,
        input_fields: Vec<Box<str>>,
    },
    #[error("reduction `{edge}` target field `{field}` divides by zero")]
    DivisionByZero { edge: Box<str>, field: Box<str> },
    #[error("reduction `{edge}` target field `{field}` evaluates to non-integral size `{value}`")]
    NonIntegralResult {
        edge: Box<str>,
        field: Box<str>,
        value: Box<str>,
    },
    #[error("reduction `{edge}` target field `{field}` evaluates to negative size `{value}`")]
    NegativeResult {
        edge: Box<str>,
        field: Box<str>,
        value: BigRational,
    },
    #[error("size field `{field}` value `{value}` does not fit usize")]
    OutputOutOfRange { field: Box<str>, value: BigUint },
}

#[cfg(test)]
#[path = "unit_tests/size.rs"]
mod tests;
