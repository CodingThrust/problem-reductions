//! Lossless symbolic expressions shared by the runtime library and proc macros.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::str::FromStr;

/// A symbolic expression over named problem-size variables.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    Const(BigRational),
    Var(Box<str>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Exp(Box<Expr>),
    Log(Box<Expr>),
    Sqrt(Box<Expr>),
    Factorial(Box<Expr>),
}

impl Expr {
    pub fn integer(value: impl Into<BigInt>) -> Self {
        Self::Const(BigRational::from_integer(value.into()))
    }

    pub fn rational(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>) -> Self {
        Self::Const(BigRational::new(numerator.into(), denominator.into()))
    }

    pub fn variable(name: impl Into<Box<str>>) -> Self {
        Self::Var(name.into())
    }

    pub fn pow(base: Expr, exponent: Expr) -> Self {
        Self::Pow(Box::new(base), Box::new(exponent))
    }

    pub fn parse(input: &str) -> Self {
        Self::try_parse(input)
            .unwrap_or_else(|error| panic!("failed to parse expression {input:?}: {error}"))
    }

    pub fn try_parse(input: &str) -> Result<Self, ParseError> {
        Parser::new(tokenize(input)?).parse()
    }

    pub fn variables(&self) -> BTreeSet<&str> {
        let mut variables = BTreeSet::new();
        self.collect_variables(&mut variables);
        variables
    }

    fn collect_variables<'a>(&'a self, variables: &mut BTreeSet<&'a str>) {
        match self {
            Self::Const(_) => {}
            Self::Var(name) => {
                variables.insert(name);
            }
            Self::Add(left, right)
            | Self::Sub(left, right)
            | Self::Mul(left, right)
            | Self::Div(left, right)
            | Self::Pow(left, right) => {
                left.collect_variables(variables);
                right.collect_variables(variables);
            }
            Self::Neg(value)
            | Self::Exp(value)
            | Self::Log(value)
            | Self::Sqrt(value)
            | Self::Factorial(value) => value.collect_variables(variables),
        }
    }

    pub fn substitute(&self, replacements: &HashMap<&str, &Expr>) -> Expr {
        match self {
            Self::Const(value) => Self::Const(value.clone()),
            Self::Var(name) => replacements
                .get(name.as_ref())
                .map_or_else(|| self.clone(), |replacement| (*replacement).clone()),
            Self::Add(left, right) => {
                left.substitute(replacements) + right.substitute(replacements)
            }
            Self::Sub(left, right) => {
                left.substitute(replacements) - right.substitute(replacements)
            }
            Self::Mul(left, right) => {
                left.substitute(replacements) * right.substitute(replacements)
            }
            Self::Div(left, right) => {
                left.substitute(replacements) / right.substitute(replacements)
            }
            Self::Pow(base, exponent) => Self::pow(
                base.substitute(replacements),
                exponent.substitute(replacements),
            ),
            Self::Neg(value) => -value.substitute(replacements),
            Self::Exp(value) => Self::Exp(Box::new(value.substitute(replacements))),
            Self::Log(value) => Self::Log(Box::new(value.substitute(replacements))),
            Self::Sqrt(value) => Self::Sqrt(Box::new(value.substitute(replacements))),
            Self::Factorial(value) => Self::Factorial(Box::new(value.substitute(replacements))),
        }
    }

    pub fn is_constant(&self) -> bool {
        match self {
            Self::Const(_) => true,
            Self::Var(_) => false,
            Self::Add(left, right)
            | Self::Sub(left, right)
            | Self::Mul(left, right)
            | Self::Div(left, right)
            | Self::Pow(left, right) => left.is_constant() && right.is_constant(),
            Self::Neg(value)
            | Self::Exp(value)
            | Self::Log(value)
            | Self::Sqrt(value)
            | Self::Factorial(value) => value.is_constant(),
        }
    }

    pub fn is_polynomial(&self) -> bool {
        match self {
            Self::Const(_) | Self::Var(_) => true,
            Self::Add(left, right) | Self::Sub(left, right) | Self::Mul(left, right) => {
                left.is_polynomial() && right.is_polynomial()
            }
            Self::Pow(base, exponent) => {
                base.is_polynomial()
                    && matches!(exponent.as_ref(), Self::Const(value) if value.is_integer() && !value.is_negative())
            }
            Self::Div(_, _)
            | Self::Neg(_)
            | Self::Exp(_)
            | Self::Log(_)
            | Self::Sqrt(_)
            | Self::Factorial(_) => false,
        }
    }

    pub fn is_valid_complexity_notation(&self) -> bool {
        match self {
            Self::Const(value) => value.is_one(),
            Self::Var(_) => true,
            Self::Add(left, right) | Self::Mul(left, right) => {
                !left.is_constant()
                    && !right.is_constant()
                    && left.is_valid_complexity_notation()
                    && right.is_valid_complexity_notation()
            }
            Self::Pow(base, exponent) => {
                let base_valid = if let Self::Const(value) = base.as_ref() {
                    value.is_positive()
                } else {
                    base.is_valid_complexity_notation()
                };
                base_valid && (exponent.is_constant() || exponent.is_valid_complexity_notation())
            }
            Self::Exp(value) | Self::Log(value) | Self::Sqrt(value) | Self::Factorial(value) => {
                value.is_valid_complexity_notation()
            }
            Self::Sub(_, _) | Self::Div(_, _) | Self::Neg(_) => false,
        }
    }
}

impl std::ops::Add for Expr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Add(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::Sub for Expr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sub(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::Mul for Expr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Mul(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::Div for Expr {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Div(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::Neg for Expr {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Neg(Box::new(self))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_precedence(formatter, 0, false)
    }
}

impl Expr {
    fn precedence(&self) -> u8 {
        match self {
            Self::Add(_, _) | Self::Sub(_, _) => 1,
            Self::Mul(_, _) | Self::Div(_, _) => 2,
            Self::Neg(_) => 3,
            Self::Pow(_, _) => 4,
            _ => 5,
        }
    }

    fn fmt_with_precedence(
        &self,
        formatter: &mut fmt::Formatter<'_>,
        parent_precedence: u8,
        right_child: bool,
    ) -> fmt::Result {
        let precedence = self.precedence();
        let needs_parentheses = precedence < parent_precedence
            || (right_child
                && precedence == parent_precedence
                && matches!(
                    self,
                    Self::Add(_, _) | Self::Sub(_, _) | Self::Mul(_, _) | Self::Div(_, _)
                ))
            || (!right_child && precedence == parent_precedence && matches!(self, Self::Pow(_, _)));
        if needs_parentheses {
            write!(formatter, "(")?;
        }
        match self {
            Self::Const(value) => fmt_rational(value, formatter)?,
            Self::Var(name) => write!(formatter, "{name}")?,
            Self::Add(left, right) => {
                left.fmt_with_precedence(formatter, precedence, false)?;
                write!(formatter, " + ")?;
                right.fmt_with_precedence(formatter, precedence, true)?;
            }
            Self::Sub(left, right) => {
                left.fmt_with_precedence(formatter, precedence, false)?;
                write!(formatter, " - ")?;
                right.fmt_with_precedence(formatter, precedence, true)?;
            }
            Self::Mul(left, right) => {
                left.fmt_with_precedence(formatter, precedence, false)?;
                write!(formatter, " * ")?;
                right.fmt_with_precedence(formatter, precedence, true)?;
            }
            Self::Div(left, right) => {
                left.fmt_with_precedence(formatter, precedence, false)?;
                write!(formatter, " / ")?;
                right.fmt_with_precedence(formatter, precedence, true)?;
            }
            Self::Pow(base, exponent) => {
                base.fmt_with_precedence(formatter, precedence, false)?;
                write!(formatter, "^")?;
                exponent.fmt_with_precedence(formatter, precedence, true)?;
            }
            Self::Neg(value) => {
                write!(formatter, "-")?;
                value.fmt_with_precedence(formatter, precedence, true)?;
            }
            Self::Exp(value) => write!(formatter, "exp({value})")?,
            Self::Log(value) => write!(formatter, "log({value})")?,
            Self::Sqrt(value) => write!(formatter, "sqrt({value})")?,
            Self::Factorial(value) => write!(formatter, "factorial({value})")?,
        }
        if needs_parentheses {
            write!(formatter, ")")?;
        }
        Ok(())
    }
}

fn fmt_rational(value: &BigRational, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if value.is_integer() {
        return write!(formatter, "{}", value.to_integer());
    }
    let negative = value.is_negative();
    let numerator = value.numer().abs();
    let mut denominator = value.denom().clone();
    let mut twos = 0usize;
    let mut fives = 0usize;
    while (&denominator % 2u8).is_zero() {
        denominator /= 2u8;
        twos += 1;
    }
    while (&denominator % 5u8).is_zero() {
        denominator /= 5u8;
        fives += 1;
    }
    if !denominator.is_one() {
        return write!(formatter, "{}/{}", value.numer(), value.denom());
    }
    let scale = twos.max(fives);
    let scaled = numerator
        * BigInt::from(2u8).pow((scale - twos) as u32)
        * BigInt::from(5u8).pow((scale - fives) as u32);
    let digits = scaled.to_string();
    let sign = if negative { "-" } else { "" };
    if digits.len() <= scale {
        write!(formatter, "{sign}0.{:0>width$}", digits, width = scale)
    } else {
        let split = digits.len() - scale;
        write!(formatter, "{sign}{}.{}", &digits[..split], &digits[split..])
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{message} at byte {position}")]
pub struct ParseError {
    position: usize,
    message: String,
}

impl ParseError {
    fn new(position: usize, message: impl Into<String>) -> Self {
        Self {
            position,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Token {
    position: usize,
    kind: TokenKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TokenKind {
    Number(BigRational),
    Ident(Box<str>),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut position = 0;
    while position < bytes.len() {
        match bytes[position] {
            b' ' | b'\t' | b'\n' | b'\r' => position += 1,
            b'+' => push_token(&mut tokens, &mut position, TokenKind::Plus),
            b'-' => push_token(&mut tokens, &mut position, TokenKind::Minus),
            b'*' => push_token(&mut tokens, &mut position, TokenKind::Star),
            b'/' => push_token(&mut tokens, &mut position, TokenKind::Slash),
            b'^' => push_token(&mut tokens, &mut position, TokenKind::Caret),
            b'(' => push_token(&mut tokens, &mut position, TokenKind::LeftParen),
            b')' => push_token(&mut tokens, &mut position, TokenKind::RightParen),
            byte if byte.is_ascii_digit() || byte == b'.' => {
                let start = position;
                while position < bytes.len()
                    && (bytes[position].is_ascii_digit() || bytes[position] == b'.')
                {
                    position += 1;
                }
                let spelling = &input[start..position];
                let value = parse_decimal(spelling).ok_or_else(|| {
                    ParseError::new(start, format!("invalid number {spelling:?}"))
                })?;
                tokens.push(Token {
                    position: start,
                    kind: TokenKind::Number(value),
                });
            }
            byte if byte.is_ascii_alphabetic() || byte == b'_' => {
                let start = position;
                while position < bytes.len()
                    && (bytes[position].is_ascii_alphanumeric() || bytes[position] == b'_')
                {
                    position += 1;
                }
                tokens.push(Token {
                    position: start,
                    kind: TokenKind::Ident(input[start..position].into()),
                });
            }
            _ => {
                let character = input[position..].chars().next().unwrap();
                return Err(ParseError::new(
                    position,
                    format!("unexpected character {character:?}"),
                ));
            }
        }
    }
    Ok(tokens)
}

fn push_token(tokens: &mut Vec<Token>, position: &mut usize, kind: TokenKind) {
    tokens.push(Token {
        position: *position,
        kind,
    });
    *position += 1;
}

fn parse_decimal(spelling: &str) -> Option<BigRational> {
    let mut parts = spelling.split('.');
    let integer = parts.next()?;
    let fractional = parts.next();
    if parts.next().is_some() || (integer.is_empty() && fractional.is_none()) {
        return None;
    }
    match fractional {
        None => BigInt::from_str(integer)
            .ok()
            .map(BigRational::from_integer),
        Some(fractional) if !integer.is_empty() || !fractional.is_empty() => {
            let combined = format!("{integer}{fractional}");
            let numerator = BigInt::from_str(&combined).ok()?;
            let denominator = BigInt::from(10u8).pow(fractional.len() as u32);
            Some(BigRational::new(numerator, denominator))
        }
        Some(_) => None,
    }
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn parse(mut self) -> Result<Expr, ParseError> {
        if self.tokens.is_empty() {
            return Err(ParseError::new(0, "expected expression"));
        }
        let expression = self.parse_additive()?;
        if let Some(token) = self.peek() {
            return Err(ParseError::new(token.position, "unexpected trailing token"));
        }
        Ok(expression)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += usize::from(token.is_some());
        token
    }

    fn consume(&mut self, kind: &TokenKind) -> bool {
        if self.peek().is_some_and(|token| &token.kind == kind) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut expression = self.parse_multiplicative()?;
        loop {
            if self.consume(&TokenKind::Plus) {
                expression = expression + self.parse_multiplicative()?;
            } else if self.consume(&TokenKind::Minus) {
                expression = expression - self.parse_multiplicative()?;
            } else {
                return Ok(expression);
            }
        }
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut expression = self.parse_unary()?;
        loop {
            if self.consume(&TokenKind::Star) {
                expression = expression * self.parse_unary()?;
            } else if self.consume(&TokenKind::Slash) {
                expression = expression / self.parse_unary()?;
            } else {
                return Ok(expression);
            }
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.consume(&TokenKind::Minus) {
            Ok(-self.parse_unary()?)
        } else {
            self.parse_power()
        }
    }

    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let base = self.parse_primary()?;
        if self.consume(&TokenKind::Caret) {
            Ok(Expr::pow(base, self.parse_unary()?))
        } else {
            Ok(base)
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self
            .advance()
            .ok_or_else(|| ParseError::new(self.end_position(), "expected expression"))?;
        match token.kind {
            TokenKind::Number(value) => Ok(Expr::Const(value)),
            TokenKind::Ident(name) => {
                if !self.consume(&TokenKind::LeftParen) {
                    return Ok(Expr::Var(name));
                }
                let argument = self.parse_additive()?;
                self.expect_right_paren()?;
                match name.as_ref() {
                    "exp" => Ok(Expr::Exp(Box::new(argument))),
                    "log" => Ok(Expr::Log(Box::new(argument))),
                    "sqrt" => Ok(Expr::Sqrt(Box::new(argument))),
                    "factorial" => Ok(Expr::Factorial(Box::new(argument))),
                    _ => Err(ParseError::new(
                        token.position,
                        format!("unknown function {name:?}"),
                    )),
                }
            }
            TokenKind::LeftParen => {
                let expression = self.parse_additive()?;
                self.expect_right_paren()?;
                Ok(expression)
            }
            _ => Err(ParseError::new(token.position, "expected expression")),
        }
    }

    fn expect_right_paren(&mut self) -> Result<(), ParseError> {
        if self.consume(&TokenKind::RightParen) {
            Ok(())
        } else {
            Err(ParseError::new(
                self.end_position(),
                "expected closing parenthesis",
            ))
        }
    }

    fn end_position(&self) -> usize {
        self.peek().map_or_else(
            || self.tokens.last().map_or(0, |token| token.position + 1),
            |token| token.position,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_literals_are_exact() {
        assert_eq!(Expr::parse("2.372"), Expr::rational(593, 250));
    }

    #[test]
    fn parser_preserves_source_operators() {
        let expression = Expr::parse("n * (n - 1) / 2 - m");
        assert!(matches!(expression, Expr::Sub(_, _)));
        let Expr::Sub(left, _) = expression else {
            unreachable!()
        };
        assert!(matches!(left.as_ref(), Expr::Div(_, _)));
    }

    #[test]
    fn variables_are_owned() {
        let name = String::from("dynamic_size");
        let expression = Expr::parse(&name);
        drop(name);
        assert_eq!(expression.variables(), BTreeSet::from(["dynamic_size"]));
    }

    #[test]
    fn exponentiation_precedes_unary_minus() {
        assert_eq!(
            Expr::parse("-n^2"),
            -Expr::pow(Expr::variable("n"), Expr::integer(2))
        );
        assert_eq!(
            Expr::parse("2^-3"),
            Expr::pow(Expr::integer(2), -Expr::integer(3))
        );
    }

    #[test]
    fn display_preserves_grouping() {
        let expression = Expr::parse("n * (n - 1) / 2 - m");
        assert_eq!(expression.to_string(), "n * (n - 1) / 2 - m");
        assert_eq!(Expr::parse(&expression.to_string()), expression);
    }

    #[test]
    fn serialization_preserves_every_operator() {
        let expression = Expr::parse("-factorial(n - 1) + exp(m) / log(sqrt(k))^2");
        let encoded = serde_json::to_string(&expression).unwrap();
        let decoded: Expr = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, expression);
    }

    #[test]
    fn display_does_not_normalize_half_power_to_sqrt() {
        let power = Expr::pow(Expr::variable("n"), Expr::rational(1, 2));
        assert_eq!(power.to_string(), "n^0.5");
        assert_eq!(Expr::parse(&power.to_string()), power);
    }
}
