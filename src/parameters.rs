//! Symbolic parameter transformations carried by reduction rules.

use crate::expr::{AlgebraicAnalysis, Expr, ExprNode, ExprNodeId, Symbol};
use crate::types::ProblemParameters;
use num_bigint::{BigInt, BigUint, Sign};
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

/// What a reduction promises about one parameter formula.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterRelation {
    Exact,
    UpperBound,
}

/// Symbolic predictions with independent accuracy and availability per field.
#[derive(Clone, Debug)]
pub struct ParameterTransform {
    edge: Box<str>,
    fields: Vec<ParameterField>,
    unavailable: BTreeMap<Box<str>, ParameterTransformError>,
}

#[derive(Clone, Debug)]
struct ParameterField {
    name: Box<str>,
    expression: Expr,
    relation: ParameterRelation,
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

impl ParameterTransform {
    pub fn new<I, N>(
        edge: impl Into<Box<str>>,
        relation: ParameterRelation,
        fields: I,
    ) -> Result<Self, ParameterTransformError>
    where
        I: IntoIterator<Item = (N, Expr)>,
        N: Into<Box<str>>,
    {
        Self::from_fields(
            edge,
            fields
                .into_iter()
                .map(|(name, expression)| (name, relation, expression)),
        )
    }

    /// Construct predictions whose relations may differ by target field.
    pub fn from_fields<I, N>(
        edge: impl Into<Box<str>>,
        fields: I,
    ) -> Result<Self, ParameterTransformError>
    where
        I: IntoIterator<Item = (N, ParameterRelation, Expr)>,
        N: Into<Box<str>>,
    {
        let edge = edge.into();
        let mut names = HashSet::new();
        let mut raw_fields = Vec::new();
        for (name, relation, expression) in fields {
            let name = name.into();
            if let Err(error) = Symbol::new(name.clone()) {
                return Err(ParameterTransformError::InvalidTargetField {
                    edge,
                    field: name,
                    reason: error.to_string().into(),
                });
            }
            if !names.insert(name.clone()) {
                return Err(ParameterTransformError::DuplicateTargetField { edge, field: name });
            }
            raw_fields.push((name, relation, expression));
        }

        let expressions = raw_fields
            .iter()
            .map(|(_, _, expression)| expression)
            .collect::<Vec<_>>();
        let analysis = AlgebraicAnalysis::new(&expressions);
        let mut plans = HashMap::new();
        let fields = raw_fields
            .into_iter()
            .map(|(name, relation, expression)| {
                let plan = compile(&expression, &analysis, &mut plans).map_err(|failure| {
                    validation_error(edge.clone(), name.clone(), expression.to_string(), failure)
                })?;
                Ok(ParameterField {
                    name,
                    expression,
                    relation,
                    plan,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            edge,
            fields,
            unavailable: BTreeMap::new(),
        })
    }

    pub fn edge(&self) -> &str {
        &self.edge
    }

    pub fn relation(&self, target_field: &str) -> Option<ParameterRelation> {
        self.fields
            .iter()
            .find(|field| field.name.as_ref() == target_field)
            .map(|field| field.relation)
    }

    /// Why this target field could not be predicted, including upstream causes.
    pub fn unavailable(&self, target_field: &str) -> Option<&ParameterTransformError> {
        self.unavailable.get(target_field)
    }

    pub(crate) fn declare_unavailable(&mut self, field: &str, reason: &str) {
        self.unavailable.insert(
            field.into(),
            ParameterTransformError::Unavailable {
                edge: self.edge.clone(),
                field: field.into(),
                reason: reason.into(),
            },
        );
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

    pub fn evaluate(
        &self,
        input: &ProblemParameters,
    ) -> Result<ProblemParameters, ParameterTransformError> {
        let mut memo = HashMap::new();
        let mut output = Vec::with_capacity(self.fields.len());
        for field in &self.fields {
            let value = evaluate_plan(&field.plan, input, &mut memo).map_err(|failure| {
                evaluation_error(self.edge.clone(), field.name.clone(), failure)
            })?;
            if value.is_negative() {
                return Err(ParameterTransformError::NegativeResult {
                    edge: self.edge.clone(),
                    field: field.name.clone(),
                    value,
                });
            }
            let value = if field.relation == ParameterRelation::Exact {
                if !value.is_integer() {
                    return Err(ParameterTransformError::NonIntegralResult {
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
                u64::try_from(&value).map_err(|_| ParameterTransformError::OutputOutOfRange {
                    field: field.name.clone(),
                    value: value.clone(),
                })?;
            output.push((field.name.to_string(), value));
        }
        Ok(ProblemParameters::from_owned(output))
    }

    pub fn compose(
        &self,
        next: &ParameterTransform,
        edge: impl Into<Box<str>>,
    ) -> Result<ParameterTransform, ParameterTransformError> {
        let edge = edge.into();
        let replacements: HashMap<&str, &Expr> = self.expressions().collect();
        let mut fields = Vec::new();
        let mut unavailable = next.unavailable.clone();
        for field in &next.fields {
            let result = (|| {
                let mut bounded_input = false;
                for input in field.expression.variables() {
                    if let Some(cause) = self.unavailable(input) {
                        return Err(ParameterTransformError::UnavailableInput {
                            edge: next.edge.clone(),
                            field: field.name.clone(),
                            input_field: input.into(),
                            cause: Box::new(cause.clone()),
                        });
                    }
                    bounded_input |= self.relation(input) == Some(ParameterRelation::UpperBound);
                }
                let expression = if bounded_input {
                    positive_polynomial_hull(&field.expression).ok_or_else(|| {
                        ParameterTransformError::CannotPropagateUpperBound {
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
                        .map_err(|error| ParameterTransformError::MissingCompositionInput {
                            edge: next.edge.clone(),
                            field: field.name.clone(),
                            input_fields: error.missing_variables().map(Box::<str>::from).collect(),
                        })?;
                let relation = if bounded_input {
                    ParameterRelation::UpperBound
                } else {
                    field.relation
                };
                Ok((field.name.clone(), relation, expression))
            })();
            match result {
                Ok(prediction) => fields.push(prediction),
                Err(error) => {
                    unavailable.insert(field.name.clone(), error);
                }
            }
        }
        let mut composed = Self::from_fields(edge, fields)?;
        composed.unavailable = unavailable;
        Ok(composed)
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
    input: &ProblemParameters,
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
) -> ParameterTransformError {
    match failure {
        ValidationFailure::NonIntegralConstantExponent(exponent) => {
            ParameterTransformError::NonIntegralConstantExponent {
                edge,
                field,
                expression: expression.into(),
                exponent,
            }
        }
        ValidationFailure::UnsupportedOperator(operator) => {
            ParameterTransformError::UnsupportedOperator {
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
) -> ParameterTransformError {
    match failure {
        EvaluationFailure::MissingInputField(input_field) => {
            ParameterTransformError::MissingInputField {
                edge,
                field,
                input_field,
            }
        }
        EvaluationFailure::DivisionByZero => {
            ParameterTransformError::DivisionByZero { edge, field }
        }
    }
}

/// Validation, composition, or evaluation failure for a [`ParameterTransform`].
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ParameterTransformError {
    #[error("reduction `{edge}` target field `{field}` is unavailable: {reason}")]
    Unavailable {
        edge: Box<str>,
        field: Box<str>,
        reason: Box<str>,
    },
    #[error(
        "reduction `{edge}` target field `{field}` depends on unavailable `{input_field}`: {cause}"
    )]
    UnavailableInput {
        edge: Box<str>,
        field: Box<str>,
        input_field: Box<str>,
        #[source]
        cause: Box<ParameterTransformError>,
    },
    #[error("reduction `{edge}` has invalid target parameter field `{field}`: {reason}")]
    InvalidTargetField {
        edge: Box<str>,
        field: Box<str>,
        reason: Box<str>,
    },
    #[error("reduction `{edge}` declares target parameter field `{field}` more than once")]
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
        "reduction `{edge}` target field `{field}` is missing input parameter field `{input_field}`"
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
    #[error("reduction `{edge}` target field `{field}` evaluates to non-integral parameter value `{value}`")]
    NonIntegralResult {
        edge: Box<str>,
        field: Box<str>,
        value: Box<str>,
    },
    #[error(
        "reduction `{edge}` target field `{field}` evaluates to negative parameter value `{value}`"
    )]
    NegativeResult {
        edge: Box<str>,
        field: Box<str>,
        value: BigRational,
    },
    #[error("parameter field `{field}` value `{value}` does not fit u64")]
    OutputOutOfRange { field: Box<str>, value: BigUint },
}

#[cfg(test)]
#[path = "unit_tests/parameters.rs"]
mod tests;
