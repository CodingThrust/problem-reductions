//! Exact symbolic maps between problem-size vectors.

use crate::expr::{Expr, ExprNode, ExprNodeId, Symbol};
use crate::growth::Growth;
use crate::types::ProblemSize;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Signed, Zero};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// A validated exact mapping from source size fields to target size fields.
#[derive(Clone, Debug)]
pub struct SizeMap {
    edge: Box<str>,
    fields: Vec<SizeMapField>,
}

#[derive(Clone, Debug)]
struct SizeMapField {
    name: Box<str>,
    expression: Expr,
    plan: ExactPlan,
}

#[derive(Clone, Debug)]
struct ExactPlan(Arc<ExactPlanNode>);

#[derive(Debug)]
enum ExactPlanNode {
    Const(BigInt),
    Var(Symbol),
    Add(Box<[ExactPlan]>),
    Mul(Box<[ExactPlan]>),
    Div(ExactPlan, ExactPlan),
    Pow(ExactPlan, BigUint),
}

impl ExactPlan {
    fn identity(&self) -> usize {
        Arc::as_ptr(&self.0) as usize
    }
}

impl SizeMap {
    /// Validate and compile exact expressions for one reduction edge.
    pub fn new<I, N>(edge: impl Into<Box<str>>, fields: I) -> Result<Self, SizeMapError>
    where
        I: IntoIterator<Item = (N, Expr)>,
        N: Into<Box<str>>,
    {
        let edge = edge.into();
        let mut names = HashSet::new();
        let mut plans = HashMap::new();
        let mut compiled_fields = Vec::new();
        for (name, expression) in fields {
            let name = name.into();
            if let Err(error) = Symbol::new(name.clone()) {
                return Err(SizeMapError::InvalidTargetField {
                    edge,
                    field: name,
                    reason: error.to_string().into(),
                });
            }
            if !names.insert(name.clone()) {
                return Err(SizeMapError::DuplicateTargetField { edge, field: name });
            }
            let plan = compile_expression(&expression, &mut plans).map_err(|error| {
                validation_error(edge.clone(), name.clone(), expression.to_string(), error)
            })?;
            compiled_fields.push(SizeMapField {
                name,
                expression,
                plan,
            });
        }
        Ok(Self {
            edge,
            fields: compiled_fields,
        })
    }

    /// The reduction edge named in all errors from this map.
    pub fn edge(&self) -> &str {
        &self.edge
    }

    /// Exact expressions in deterministic target-field order.
    pub fn expressions(&self) -> impl Iterator<Item = (&str, &Expr)> {
        self.fields
            .iter()
            .map(|field| (field.name.as_ref(), &field.expression))
    }

    /// Return the exact expression for a target field.
    pub fn get(&self, target_field: &str) -> Option<&Expr> {
        self.fields
            .iter()
            .find(|field| field.name.as_ref() == target_field)
            .map(|field| &field.expression)
    }

    /// Evaluate every target field exactly and convert each result to `usize`.
    pub fn evaluate(&self, input: &ProblemSize) -> Result<ProblemSize, SizeMapError> {
        let mut memo = HashMap::new();
        let mut output = Vec::with_capacity(self.fields.len());
        for field in &self.fields {
            let value =
                evaluate_plan(&field.plan, input, &mut memo).map_err(|error| match error {
                    EvaluationFailure::MissingInputField(input_field) => {
                        SizeMapError::MissingInputField {
                            edge: self.edge.clone(),
                            target_field: field.name.clone(),
                            input_field,
                        }
                    }
                    EvaluationFailure::DivisionByZero => SizeMapError::DivisionByZero {
                        edge: self.edge.clone(),
                        target_field: field.name.clone(),
                    },
                    EvaluationFailure::NonIntegralDivision {
                        numerator,
                        denominator,
                    } => SizeMapError::NonIntegralResult {
                        edge: self.edge.clone(),
                        target_field: field.name.clone(),
                        value: format!("{numerator}/{denominator}").into(),
                    },
                })?;
            if value.is_negative() {
                return Err(SizeMapError::NegativeResult {
                    edge: self.edge.clone(),
                    target_field: field.name.clone(),
                    value,
                });
            }
            let concrete = usize::try_from(&value).map_err(|_| SizeMapError::OutputOutOfRange {
                edge: self.edge.clone(),
                target_field: field.name.clone(),
                value,
            })?;
            output.push((field.name.as_ref(), concrete));
        }
        Ok(ProblemSize::new(output))
    }

    /// Compose two exact maps by canonical substitution.
    pub fn compose(
        &self,
        next: &SizeMap,
        composed_edge: impl Into<Box<str>>,
    ) -> Result<SizeMap, SizeMapError> {
        let composed_edge = composed_edge.into();
        let replacements: HashMap<&str, &Expr> = self.expressions().collect();
        let mut fields = Vec::with_capacity(next.fields.len());
        for field in &next.fields {
            let expression = field
                .expression
                .substitute_complete(&replacements)
                .map_err(|error| SizeMapError::MissingCompositionInput {
                    edge: composed_edge.clone(),
                    target_field: field.name.clone(),
                    input_fields: error.missing_variables().map(Box::<str>::from).collect(),
                })?;
            fields.push((field.name.clone(), expression));
        }
        Self::new(composed_edge, fields)
    }

    /// Explicitly project terminal exact expressions into the Growth domain.
    pub fn project_growth(&self) -> Vec<(Box<str>, Growth)> {
        let expressions: Vec<_> = self.fields.iter().map(|field| &field.expression).collect();
        let growth = Growth::from_expr_batch(&expressions);
        self.fields
            .iter()
            .zip(growth)
            .map(|(field, growth)| (field.name.clone(), growth))
            .collect()
    }
}

fn compile_expression(
    expression: &Expr,
    memo: &mut HashMap<ExprNodeId, ExactPlan>,
) -> Result<ExactPlan, ValidationFailure> {
    if let Some(plan) = memo.get(&expression.node_identity()) {
        return Ok(plan.clone());
    }
    let node = match expression.node() {
        ExprNode::Const(value) => {
            if !value.is_integer() {
                return Err(ValidationFailure::NonIntegralConstant(
                    value.to_string().into(),
                ));
            }
            ExactPlanNode::Const(value.to_integer())
        }
        ExprNode::Var(symbol) => ExactPlanNode::Var(symbol.clone()),
        ExprNode::Add(values) => ExactPlanNode::Add(
            values
                .iter()
                .map(|value| compile_expression(value, memo))
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
        ),
        ExprNode::Mul(values) => return compile_product(expression, values, memo),
        ExprNode::Pow(base, exponent) => {
            let ExprNode::Const(exponent) = exponent.node() else {
                return Err(ValidationFailure::NonIntegralConstantExponent(
                    exponent.to_string().into(),
                ));
            };
            if !exponent.is_integer() {
                return Err(ValidationFailure::NonIntegralConstantExponent(
                    exponent.to_string().into(),
                ));
            }
            let exponent = exponent.to_integer();
            if exponent.sign() == Sign::Minus {
                ExactPlanNode::Div(
                    constant_plan(BigInt::one()),
                    power_plan(base, exponent.magnitude().clone(), memo)?,
                )
            } else {
                ExactPlanNode::Pow(
                    compile_expression(base, memo)?,
                    exponent.magnitude().clone(),
                )
            }
        }
        ExprNode::Exp(_) => return Err(ValidationFailure::UnsupportedOperator("exp")),
        ExprNode::Log(_) => return Err(ValidationFailure::UnsupportedOperator("log")),
        ExprNode::Factorial(_) => return Err(ValidationFailure::UnsupportedOperator("factorial")),
    };
    let plan = ExactPlan(Arc::new(node));
    memo.insert(expression.node_identity(), plan.clone());
    Ok(plan)
}

fn compile_product(
    expression: &Expr,
    values: &[Expr],
    memo: &mut HashMap<ExprNodeId, ExactPlan>,
) -> Result<ExactPlan, ValidationFailure> {
    let mut numerator = Vec::new();
    let mut denominator = Vec::new();
    for value in values {
        if let ExprNode::Pow(base, exponent) = value.node() {
            if let ExprNode::Const(exponent) = exponent.node() {
                if exponent.is_integer() && exponent.is_negative() {
                    denominator.push(power_plan(
                        base,
                        exponent.to_integer().magnitude().clone(),
                        memo,
                    )?);
                    continue;
                }
            }
        }
        numerator.push(compile_expression(value, memo)?);
    }
    let plan = if denominator.is_empty() {
        product_plan(numerator)
    } else {
        ExactPlan(Arc::new(ExactPlanNode::Div(
            product_plan(numerator),
            product_plan(denominator),
        )))
    };
    memo.insert(expression.node_identity(), plan.clone());
    Ok(plan)
}

fn power_plan(
    base: &Expr,
    exponent: BigUint,
    memo: &mut HashMap<ExprNodeId, ExactPlan>,
) -> Result<ExactPlan, ValidationFailure> {
    Ok(ExactPlan(Arc::new(ExactPlanNode::Pow(
        compile_expression(base, memo)?,
        exponent,
    ))))
}

fn product_plan(mut factors: Vec<ExactPlan>) -> ExactPlan {
    match factors.len() {
        0 => constant_plan(BigInt::one()),
        1 => factors.remove(0),
        _ => ExactPlan(Arc::new(ExactPlanNode::Mul(factors.into_boxed_slice()))),
    }
}

fn constant_plan(value: BigInt) -> ExactPlan {
    ExactPlan(Arc::new(ExactPlanNode::Const(value)))
}

fn evaluate_plan(
    plan: &ExactPlan,
    input: &ProblemSize,
    memo: &mut HashMap<usize, BigInt>,
) -> Result<BigInt, EvaluationFailure> {
    if let Some(value) = memo.get(&plan.identity()) {
        return Ok(value.clone());
    }
    let value = match plan.0.as_ref() {
        ExactPlanNode::Const(value) => value.clone(),
        ExactPlanNode::Var(symbol) => BigInt::from(
            input
                .get(symbol.as_str())
                .ok_or_else(|| EvaluationFailure::MissingInputField(symbol.to_string().into()))?,
        ),
        ExactPlanNode::Add(values) => values.iter().try_fold(
            BigInt::zero(),
            |sum, value| -> Result<_, EvaluationFailure> {
                Ok(sum + evaluate_plan(value, input, memo)?)
            },
        )?,
        ExactPlanNode::Mul(values) => values.iter().try_fold(
            BigInt::one(),
            |product, value| -> Result<_, EvaluationFailure> {
                Ok(product * evaluate_plan(value, input, memo)?)
            },
        )?,
        ExactPlanNode::Div(numerator, denominator) => {
            let numerator = evaluate_plan(numerator, input, memo)?;
            let denominator = evaluate_plan(denominator, input, memo)?;
            if denominator.is_zero() {
                return Err(EvaluationFailure::DivisionByZero);
            }
            if (&numerator % &denominator) != BigInt::zero() {
                return Err(EvaluationFailure::NonIntegralDivision {
                    numerator,
                    denominator,
                });
            }
            numerator / denominator
        }
        ExactPlanNode::Pow(base, exponent) => {
            pow_exact(evaluate_plan(base, input, memo)?, exponent)
        }
    };
    memo.insert(plan.identity(), value.clone());
    Ok(value)
}

fn pow_exact(mut base: BigInt, exponent: &BigUint) -> BigInt {
    let mut exponent = exponent.clone();
    let mut result = BigInt::one();
    while !exponent.is_zero() {
        if exponent.bit(0) {
            result *= &base;
        }
        exponent >>= 1usize;
        if !exponent.is_zero() {
            base = &base * &base;
        }
    }
    result
}

#[derive(Debug)]
enum ValidationFailure {
    NonIntegralConstant(Box<str>),
    NonIntegralConstantExponent(Box<str>),
    UnsupportedOperator(&'static str),
}

fn validation_error(
    edge: Box<str>,
    target_field: Box<str>,
    expression: String,
    failure: ValidationFailure,
) -> SizeMapError {
    match failure {
        ValidationFailure::NonIntegralConstant(value) => SizeMapError::NonIntegralConstant {
            edge,
            target_field,
            expression: expression.into(),
            value,
        },
        ValidationFailure::NonIntegralConstantExponent(exponent) => {
            SizeMapError::NonIntegralConstantExponent {
                edge,
                target_field,
                expression: expression.into(),
                exponent,
            }
        }
        ValidationFailure::UnsupportedOperator(operator) => SizeMapError::UnsupportedOperator {
            edge,
            target_field,
            expression: expression.into(),
            operator,
        },
    }
}

#[derive(Debug)]
enum EvaluationFailure {
    MissingInputField(Box<str>),
    DivisionByZero,
    NonIntegralDivision {
        numerator: BigInt,
        denominator: BigInt,
    },
}

/// Validation, composition, or exact-evaluation failure for a [`SizeMap`].
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SizeMapError {
    #[error("size map for edge {edge} has invalid target field {field:?}: {reason}")]
    InvalidTargetField {
        edge: Box<str>,
        field: Box<str>,
        reason: Box<str>,
    },
    #[error("size map for edge {edge} declares target field {field} more than once")]
    DuplicateTargetField { edge: Box<str>, field: Box<str> },
    #[error("size map for edge {edge}, target field {target_field}, contains non-integral constant {value} in {expression}")]
    NonIntegralConstant {
        edge: Box<str>,
        target_field: Box<str>,
        expression: Box<str>,
        value: Box<str>,
    },
    #[error("size map for edge {edge}, target field {target_field}, requires a constant integral exponent, found {exponent} in {expression}")]
    NonIntegralConstantExponent {
        edge: Box<str>,
        target_field: Box<str>,
        expression: Box<str>,
        exponent: Box<str>,
    },
    #[error("size map for edge {edge}, target field {target_field}, does not support {operator} in {expression}")]
    UnsupportedOperator {
        edge: Box<str>,
        target_field: Box<str>,
        expression: Box<str>,
        operator: &'static str,
    },
    #[error("size map composition for edge {edge}, target field {target_field}, is missing intermediate fields {input_fields:?}")]
    MissingCompositionInput {
        edge: Box<str>,
        target_field: Box<str>,
        input_fields: Vec<Box<str>>,
    },
    #[error("size map for edge {edge}, target field {target_field}, is missing input field {input_field}")]
    MissingInputField {
        edge: Box<str>,
        target_field: Box<str>,
        input_field: Box<str>,
    },
    #[error("size map for edge {edge}, target field {target_field}, divides by zero")]
    DivisionByZero {
        edge: Box<str>,
        target_field: Box<str>,
    },
    #[error("size map for edge {edge}, target field {target_field}, produced non-integral value {value}")]
    NonIntegralResult {
        edge: Box<str>,
        target_field: Box<str>,
        value: Box<str>,
    },
    #[error(
        "size map for edge {edge}, target field {target_field}, produced negative value {value}"
    )]
    NegativeResult {
        edge: Box<str>,
        target_field: Box<str>,
        value: BigInt,
    },
    #[error("size map for edge {edge}, target field {target_field}, produced value {value} outside the ProblemSize range")]
    OutputOutOfRange {
        edge: Box<str>,
        target_field: Box<str>,
        value: BigInt,
    },
}

#[cfg(test)]
#[path = "unit_tests/size_map.rs"]
mod tests;
