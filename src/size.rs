//! Symbolic size transformations carried by reduction rules.

use crate::expr::{AlgebraicAnalysis, Expr, ExprNode, ExprNodeId, Symbol};
use crate::growth::Growth;
use crate::types::ProblemSize;
use num_bigint::{BigInt, BigUint, Sign};
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::collections::{BTreeMap, HashMap, HashSet};
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

/// Big-O projection of the target size fields used by symbolic cost models.
#[derive(Clone, Debug, PartialEq)]
pub struct SizeGrowth {
    fields: Vec<(Box<str>, Growth)>,
}

impl SizeGrowth {
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

/// Return whether `left` has no faster Big-O growth in every symbolic size
/// field and strictly slower growth in at least one field.
pub fn size_growth_dominates(left: &SizeGrowth, right: &SizeGrowth) -> bool {
    if left.fields.len() != right.fields.len() {
        return false;
    }
    let mut strictly_smaller = false;
    for (name, growth) in left.fields() {
        let Some(other) = right.get(name) else {
            return false;
        };
        let left_at_most = other.dominates(growth);
        if !left_at_most {
            return false;
        }
        strictly_smaller |= !growth.dominates(other);
    }
    strictly_smaller
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
                Ok(SizeField {
                    name,
                    expression,
                    plan,
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

    pub fn evaluate(&self, input: &ProblemSize) -> Result<ProblemSize, SizeTransformError> {
        let mut memo = HashMap::new();
        let mut output = Vec::with_capacity(self.fields.len());
        for field in &self.fields {
            let value = evaluate_plan(&field.plan, input, &mut memo).map_err(|failure| {
                evaluation_error(self.edge.clone(), field.name.clone(), failure)
            })?;
            if value.is_negative() {
                return Err(SizeTransformError::NegativeResult {
                    edge: self.edge.clone(),
                    field: field.name.clone(),
                    value,
                });
            }
            let value = if self.relation == SizeRelation::Exact {
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
            let value =
                u64::try_from(&value).map_err(|_| SizeTransformError::OutputOutOfRange {
                    field: field.name.clone(),
                    value: value.clone(),
                })?;
            output.push((field.name.to_string(), value));
        }
        Ok(ProblemSize { components: output })
    }

    pub fn compose(
        &self,
        next: &SizeTransform,
        edge: impl Into<Box<str>>,
    ) -> Result<SizeTransform, SizeTransformError> {
        let edge = edge.into();
        let replacements: HashMap<&str, &Expr> = self.expressions().collect();
        let fields = next
            .fields
            .iter()
            .map(|field| {
                let expression = if self.relation == SizeRelation::UpperBound {
                    positive_polynomial_hull(&field.expression).ok_or_else(|| {
                        SizeTransformError::CannotPropagateUpperBound {
                            edge: next.edge.clone(),
                            field: field.name.clone(),
                            expression: field.expression.to_string().into(),
                        }
                    })?
                } else {
                    field.expression.clone()
                };
                let expression =
                    expression
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
        SizeGrowth { fields }
    }
}

type Monomial = BTreeMap<Symbol, BigUint>;
type Polynomial = BTreeMap<Monomial, BigRational>;

fn positive_polynomial_hull(expression: &Expr) -> Option<Expr> {
    let polynomial = polynomial(expression)?;
    let terms = polynomial
        .into_iter()
        .filter(|(_, coefficient)| coefficient.is_positive())
        .map(|(monomial, coefficient)| {
            monomial
                .into_iter()
                .fold(Expr::constant(coefficient), |term, (variable, exponent)| {
                    term * Expr::pow(
                        Expr::variable(variable.as_str()),
                        Expr::integer(BigInt::from(exponent)),
                    )
                })
        });
    Some(terms.fold(Expr::integer(0), |sum, term| sum + term))
}

fn polynomial(expression: &Expr) -> Option<Polynomial> {
    match expression.node() {
        ExprNode::Const(value) => Some(BTreeMap::from([(BTreeMap::new(), value.clone())])),
        ExprNode::Var(variable) => Some(BTreeMap::from([(
            BTreeMap::from([(variable.clone(), BigUint::one())]),
            BigRational::one(),
        )])),
        ExprNode::Add(values) => values.iter().try_fold(BTreeMap::new(), |sum, value| {
            Some(add_polynomials(sum, polynomial(value)?))
        }),
        ExprNode::Mul(values) => values.iter().try_fold(
            BTreeMap::from([(BTreeMap::new(), BigRational::one())]),
            |product, value| Some(multiply_polynomials(product, polynomial(value)?)),
        ),
        ExprNode::Pow(base, exponent) => {
            let ExprNode::Const(exponent) = exponent.node() else {
                return None;
            };
            if !exponent.is_integer() {
                return None;
            }
            if exponent.is_negative() {
                let ExprNode::Const(base) = base.node() else {
                    return None;
                };
                if base.is_zero() {
                    return None;
                }
                return Some(BTreeMap::from([(
                    BTreeMap::new(),
                    pow_rational(base.clone(), &exponent.to_integer()),
                )]));
            }
            let mut exponent = exponent.to_integer().magnitude().clone();
            let mut base = polynomial(base)?;
            let mut result = BTreeMap::from([(BTreeMap::new(), BigRational::one())]);
            while !exponent.is_zero() {
                if exponent.bit(0) {
                    result = multiply_polynomials(result, base.clone());
                }
                exponent >>= 1usize;
                if !exponent.is_zero() {
                    base = multiply_polynomials(base.clone(), base);
                }
            }
            Some(result)
        }
        ExprNode::Exp(_) | ExprNode::Log(_) | ExprNode::Factorial(_) => None,
    }
}

fn add_polynomials(mut left: Polynomial, right: Polynomial) -> Polynomial {
    for (monomial, right_coefficient) in right {
        *left.entry(monomial).or_insert_with(BigRational::zero) += right_coefficient;
    }
    left.retain(|_, coefficient| !coefficient.is_zero());
    left
}

fn multiply_polynomials(left: Polynomial, right: Polynomial) -> Polynomial {
    let mut product = Polynomial::new();
    for (left_monomial, left_coefficient) in left {
        for (right_monomial, right_coefficient) in &right {
            let mut monomial = left_monomial.clone();
            for (variable, exponent) in right_monomial {
                *monomial.entry(variable.clone()).or_default() += exponent;
            }
            *product.entry(monomial).or_insert_with(BigRational::zero) +=
                &left_coefficient * right_coefficient;
        }
    }
    product.retain(|_, coefficient| !coefficient.is_zero());
    product
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

fn evaluate_plan(
    plan: &Plan,
    input: &ProblemSize,
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
                .ok_or_else(|| EvaluationFailure::MissingInputField(symbol.to_string().into()))?,
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
    #[error("size field `{field}` value `{value}` does not fit u64")]
    OutputOutOfRange { field: Box<str>, value: BigUint },
}

#[cfg(test)]
#[path = "unit_tests/size.rs"]
mod tests;
