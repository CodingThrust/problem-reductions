//! Certified monotone bounds between problem-size vectors.

use crate::expr::{Expr, ExprNode, ExprNodeId, Symbol};
use crate::growth::Growth;
use num_bigint::{BigUint, Sign};
use num_traits::{One, Zero};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// An ordered vector of arbitrary-precision non-negative problem-size bounds.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BoundVector {
    components: Vec<(Box<str>, BigUint)>,
}

impl BoundVector {
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
}

/// A validated mapping whose expressions are proven non-negative and monotone.
#[derive(Clone, Debug)]
pub struct SizeBound {
    edge: Box<str>,
    fields: Vec<SizeBoundField>,
}

#[derive(Clone, Debug)]
struct SizeBoundField {
    name: Box<str>,
    expression: Expr,
    plan: BoundPlan,
}

#[derive(Clone, Debug)]
struct BoundPlan(Arc<BoundPlanNode>);

#[derive(Debug)]
enum BoundPlanNode {
    Const(BigUint),
    Var(Symbol),
    Add(Box<[BoundPlan]>),
    Mul(Box<[BoundPlan]>),
    Pow(BoundPlan, BigUint),
}

impl BoundPlan {
    fn identity(&self) -> usize {
        Arc::as_ptr(&self.0) as usize
    }
}

impl SizeBound {
    /// Prove every expression non-negative and monotone, then compile it.
    pub fn new<I, N>(edge: impl Into<Box<str>>, fields: I) -> Result<Self, SizeBoundError>
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
                return Err(SizeBoundError::InvalidTargetField {
                    edge,
                    field: name,
                    reason: error.to_string().into(),
                });
            }
            if !names.insert(name.clone()) {
                return Err(SizeBoundError::DuplicateTargetField { edge, field: name });
            }
            let plan = compile_expression(&expression, &mut plans).map_err(|failure| {
                validation_error(edge.clone(), name.clone(), expression.to_string(), failure)
            })?;
            compiled_fields.push(SizeBoundField {
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

    pub fn edge(&self) -> &str {
        &self.edge
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

    /// Evaluate a certified bound without narrowing to a concrete machine type.
    pub fn evaluate(&self, input: &BoundVector) -> Result<BoundVector, SizeBoundError> {
        let mut memo = HashMap::new();
        let mut output = Vec::with_capacity(self.fields.len());
        for field in &self.fields {
            let value = evaluate_plan(&field.plan, input, &mut memo).map_err(|input_field| {
                SizeBoundError::MissingInputField {
                    edge: self.edge.clone(),
                    target_field: field.name.clone(),
                    input_field,
                }
            })?;
            output.push((field.name.clone(), value));
        }
        Ok(BoundVector { components: output })
    }

    /// Compose two certified bounds and re-prove the substituted expressions.
    pub fn compose(
        &self,
        next: &SizeBound,
        composed_edge: impl Into<Box<str>>,
    ) -> Result<SizeBound, SizeBoundError> {
        let composed_edge = composed_edge.into();
        let replacements: HashMap<&str, &Expr> = self.expressions().collect();
        let mut fields = Vec::with_capacity(next.fields.len());
        for field in &next.fields {
            let expression = field
                .expression
                .substitute_complete(&replacements)
                .map_err(|error| SizeBoundError::MissingCompositionInput {
                    edge: composed_edge.clone(),
                    target_field: field.name.clone(),
                    input_fields: error.missing_variables().map(Box::<str>::from).collect(),
                })?;
            fields.push((field.name.clone(), expression));
        }
        Self::new(composed_edge, fields)
    }

    /// Explicitly project terminal bound expressions into the Growth domain.
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
    memo: &mut HashMap<ExprNodeId, BoundPlan>,
) -> Result<BoundPlan, ValidationFailure> {
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
            let value = value.to_integer();
            if value.sign() == Sign::Minus {
                return Err(ValidationFailure::NegativeCoefficient(value));
            }
            BoundPlanNode::Const(value.magnitude().clone())
        }
        ExprNode::Var(symbol) => BoundPlanNode::Var(symbol.clone()),
        ExprNode::Add(values) => BoundPlanNode::Add(
            values
                .iter()
                .map(|value| compile_expression(value, memo))
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
        ),
        ExprNode::Mul(values) => BoundPlanNode::Mul(
            values
                .iter()
                .map(|value| compile_expression(value, memo))
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
        ),
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
                return Err(ValidationFailure::NegativePower(exponent));
            }
            BoundPlanNode::Pow(
                compile_expression(base, memo)?,
                exponent.magnitude().clone(),
            )
        }
        ExprNode::Exp(_) => return Err(ValidationFailure::UnsupportedOperator("exp")),
        ExprNode::Log(_) => return Err(ValidationFailure::UnsupportedOperator("log")),
        ExprNode::Factorial(_) => return Err(ValidationFailure::UnsupportedOperator("factorial")),
    };
    let plan = BoundPlan(Arc::new(node));
    memo.insert(expression.node_identity(), plan.clone());
    Ok(plan)
}

fn evaluate_plan(
    plan: &BoundPlan,
    input: &BoundVector,
    memo: &mut HashMap<usize, BigUint>,
) -> Result<BigUint, Box<str>> {
    if let Some(value) = memo.get(&plan.identity()) {
        return Ok(value.clone());
    }
    let value = match plan.0.as_ref() {
        BoundPlanNode::Const(value) => value.clone(),
        BoundPlanNode::Var(symbol) => input
            .get(symbol.as_str())
            .cloned()
            .ok_or_else(|| Box::<str>::from(symbol.as_str()))?,
        BoundPlanNode::Add(values) => values
            .iter()
            .try_fold(BigUint::zero(), |sum, value| -> Result<_, Box<str>> {
                Ok(sum + evaluate_plan(value, input, memo)?)
            })?,
        BoundPlanNode::Mul(values) => {
            values
                .iter()
                .try_fold(BigUint::one(), |product, value| -> Result<_, Box<str>> {
                    Ok(product * evaluate_plan(value, input, memo)?)
                })?
        }
        BoundPlanNode::Pow(base, exponent) => {
            pow_biguint(evaluate_plan(base, input, memo)?, exponent)
        }
    };
    memo.insert(plan.identity(), value.clone());
    Ok(value)
}

fn pow_biguint(mut base: BigUint, exponent: &BigUint) -> BigUint {
    let mut exponent = exponent.clone();
    let mut result = BigUint::one();
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
    NegativeCoefficient(num_bigint::BigInt),
    NonIntegralConstant(Box<str>),
    NegativePower(num_bigint::BigInt),
    NonIntegralConstantExponent(Box<str>),
    UnsupportedOperator(&'static str),
}

fn validation_error(
    edge: Box<str>,
    target_field: Box<str>,
    expression: String,
    failure: ValidationFailure,
) -> SizeBoundError {
    match failure {
        ValidationFailure::NegativeCoefficient(value) => SizeBoundError::NegativeCoefficient {
            edge,
            target_field,
            expression: expression.into(),
            value,
        },
        ValidationFailure::NonIntegralConstant(value) => SizeBoundError::NonIntegralConstant {
            edge,
            target_field,
            expression: expression.into(),
            value,
        },
        ValidationFailure::NegativePower(exponent) => SizeBoundError::NegativePower {
            edge,
            target_field,
            expression: expression.into(),
            exponent,
        },
        ValidationFailure::NonIntegralConstantExponent(exponent) => {
            SizeBoundError::NonIntegralConstantExponent {
                edge,
                target_field,
                expression: expression.into(),
                exponent,
            }
        }
        ValidationFailure::UnsupportedOperator(operator) => SizeBoundError::UnsupportedOperator {
            edge,
            target_field,
            expression: expression.into(),
            operator,
        },
    }
}

/// Validation, composition, or evaluation failure for a [`SizeBound`].
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SizeBoundError {
    #[error("size bound for edge {edge} has invalid target field {field:?}: {reason}")]
    InvalidTargetField {
        edge: Box<str>,
        field: Box<str>,
        reason: Box<str>,
    },
    #[error("size bound for edge {edge} declares target field {field} more than once")]
    DuplicateTargetField { edge: Box<str>, field: Box<str> },
    #[error("size bound for edge {edge}, target field {target_field}, contains negative coefficient {value} in {expression}")]
    NegativeCoefficient {
        edge: Box<str>,
        target_field: Box<str>,
        expression: Box<str>,
        value: num_bigint::BigInt,
    },
    #[error("size bound for edge {edge}, target field {target_field}, contains non-integral constant {value} in {expression}")]
    NonIntegralConstant {
        edge: Box<str>,
        target_field: Box<str>,
        expression: Box<str>,
        value: Box<str>,
    },
    #[error("size bound for edge {edge}, target field {target_field}, contains negative power {exponent} in {expression}")]
    NegativePower {
        edge: Box<str>,
        target_field: Box<str>,
        expression: Box<str>,
        exponent: num_bigint::BigInt,
    },
    #[error("size bound for edge {edge}, target field {target_field}, requires a constant integral exponent, found {exponent} in {expression}")]
    NonIntegralConstantExponent {
        edge: Box<str>,
        target_field: Box<str>,
        expression: Box<str>,
        exponent: Box<str>,
    },
    #[error("size bound for edge {edge}, target field {target_field}, does not support {operator} in {expression}")]
    UnsupportedOperator {
        edge: Box<str>,
        target_field: Box<str>,
        expression: Box<str>,
        operator: &'static str,
    },
    #[error("size bound composition for edge {edge}, target field {target_field}, is missing intermediate fields {input_fields:?}")]
    MissingCompositionInput {
        edge: Box<str>,
        target_field: Box<str>,
        input_fields: Vec<Box<str>>,
    },
    #[error("size bound for edge {edge}, target field {target_field}, is missing input field {input_field}")]
    MissingInputField {
        edge: Box<str>,
        target_field: Box<str>,
        input_field: Box<str>,
    },
}

#[cfg(test)]
#[path = "unit_tests/size_bound.rs"]
mod tests;
