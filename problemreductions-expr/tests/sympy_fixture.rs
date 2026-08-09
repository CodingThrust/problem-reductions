use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use problemreductions_expr::{Expr, ExprNode};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Deserialize)]
struct Fixture {
    oracle: Oracle,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Oracle {
    engine: String,
    version: String,
    parse_evaluate: bool,
    decimal_mode: String,
}

#[derive(Deserialize)]
struct Case {
    name: String,
    source: String,
    variables: Vec<String>,
    bindings: BTreeMap<String, i64>,
    exact_result: String,
    compare_polynomial: bool,
    is_polynomial: bool,
}

#[test]
fn sympy_fixture_matches_expression_semantics() {
    let fixture: Fixture =
        serde_json::from_str(include_str!("fixtures/sympy_oracle.json")).unwrap();
    assert_eq!(fixture.oracle.engine, "SymPy");
    assert_eq!(fixture.oracle.version, "1.14.0");
    assert!(!fixture.oracle.parse_evaluate);
    assert_eq!(fixture.oracle.decimal_mode, "rationalize base-10 spelling");
    assert_eq!(fixture.cases.len(), 50);

    let mut names = std::collections::BTreeSet::new();
    let mut operators = std::collections::BTreeSet::new();
    for case in fixture.cases {
        assert!(
            names.insert(case.name.clone()),
            "duplicate case {}",
            case.name
        );
        let expression = Expr::try_parse(&case.source)
            .unwrap_or_else(|error| panic!("{} failed to parse: {error}", case.name));
        assert_eq!(
            expression.variables(),
            case.variables.iter().map(String::as_str).collect(),
            "{} free variables",
            case.name
        );
        collect_operators(&expression, &mut operators);

        let bindings: BTreeMap<_, _> = case
            .bindings
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str(),
                    BigRational::from_integer(BigInt::from(*value)),
                )
            })
            .collect();
        let actual = evaluate_exact(&expression, &bindings)
            .unwrap_or_else(|| panic!("{} left the exact fixture domain", case.name));
        assert_eq!(
            actual,
            parse_rational(&case.exact_result),
            "{} value",
            case.name
        );

        if case.compare_polynomial {
            assert_eq!(
                expression.is_polynomial(),
                case.is_polynomial,
                "{} polynomial classification",
                case.name
            );
        }
    }
    assert_eq!(
        operators,
        std::collections::BTreeSet::from([
            "Add",
            "Const",
            "Exp",
            "Factorial",
            "Log",
            "Mul",
            "Pow",
            "Var",
        ])
    );
}

fn collect_operators(expression: &Expr, operators: &mut std::collections::BTreeSet<&'static str>) {
    let operator = match expression.node() {
        ExprNode::Const(_) => "Const",
        ExprNode::Var(_) => "Var",
        ExprNode::Add(_) => "Add",
        ExprNode::Mul(_) => "Mul",
        ExprNode::Pow(_, _) => "Pow",
        ExprNode::Exp(_) => "Exp",
        ExprNode::Log(_) => "Log",
        ExprNode::Factorial(_) => "Factorial",
    };
    operators.insert(operator);
    match expression.node() {
        ExprNode::Add(values) | ExprNode::Mul(values) => {
            for value in values {
                collect_operators(value, operators);
            }
        }
        ExprNode::Pow(left, right) => {
            collect_operators(left, operators);
            collect_operators(right, operators);
        }
        ExprNode::Exp(value) | ExprNode::Log(value) | ExprNode::Factorial(value) => {
            collect_operators(value, operators)
        }
        ExprNode::Const(_) | ExprNode::Var(_) => {}
    }
}

fn evaluate_exact(
    expression: &Expr,
    bindings: &BTreeMap<&str, BigRational>,
) -> Option<BigRational> {
    match expression.node() {
        ExprNode::Const(value) => Some(value.clone()),
        ExprNode::Var(name) => bindings.get(name.as_ref()).cloned(),
        ExprNode::Add(values) => values.iter().try_fold(BigRational::zero(), |sum, value| {
            Some(sum + evaluate_exact(value, bindings)?)
        }),
        ExprNode::Mul(values) => values
            .iter()
            .try_fold(BigRational::one(), |product, value| {
                Some(product * evaluate_exact(value, bindings)?)
            }),
        ExprNode::Pow(base, exponent) => {
            let base = evaluate_exact(base, bindings)?;
            let exponent = evaluate_exact(exponent, bindings)?;
            if exponent == BigRational::new(BigInt::one(), BigInt::from(2)) {
                exact_square_root(&base)
            } else if exponent.is_integer() {
                rational_power(base, exponent.to_integer().to_i32()?)
            } else {
                None
            }
        }
        ExprNode::Exp(value) => evaluate_exact(value, bindings)?
            .is_zero()
            .then(BigRational::one),
        ExprNode::Log(value) => {
            (evaluate_exact(value, bindings)? == BigRational::one()).then(BigRational::zero)
        }
        ExprNode::Factorial(value) => {
            let value = evaluate_exact(value, bindings)?;
            if !value.is_integer() || value.is_negative() {
                return None;
            }
            let value = value.to_integer().to_u32()?;
            Some(BigRational::from_integer(
                (2..=value).fold(BigInt::one(), |product, factor| product * factor),
            ))
        }
    }
}

fn rational_power(base: BigRational, exponent: i32) -> Option<BigRational> {
    let reciprocal = exponent.is_negative();
    if reciprocal && base.is_zero() {
        return None;
    }
    let mut remaining = exponent.unsigned_abs();
    let mut factor = base;
    let mut result = BigRational::one();
    while remaining > 0 {
        if remaining % 2 == 1 {
            result *= &factor;
        }
        remaining /= 2;
        if remaining > 0 {
            factor = &factor * &factor;
        }
    }
    if reciprocal {
        Some(result.recip())
    } else {
        Some(result)
    }
}

fn exact_square_root(value: &BigRational) -> Option<BigRational> {
    if value.is_negative() {
        return None;
    }
    Some(BigRational::new(
        perfect_square_root(value.numer())?,
        perfect_square_root(value.denom())?,
    ))
}

fn perfect_square_root(value: &BigInt) -> Option<BigInt> {
    let root = value.sqrt();
    (&root * &root == *value).then_some(root)
}

fn parse_rational(source: &str) -> BigRational {
    let (numerator, denominator) = source.split_once('/').unwrap();
    BigRational::new(
        BigInt::from_str(numerator).unwrap(),
        BigInt::from_str(denominator).unwrap(),
    )
}
