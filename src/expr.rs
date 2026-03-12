//! General symbolic expression AST for reduction overhead.

use crate::types::ProblemSize;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// A symbolic math expression over problem size variables.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    /// Numeric constant.
    Const(f64),
    /// Named variable (e.g., "num_vertices").
    Var(&'static str),
    /// Addition: a + b.
    Add(Box<Expr>, Box<Expr>),
    /// Multiplication: a * b.
    Mul(Box<Expr>, Box<Expr>),
    /// Exponentiation: base ^ exponent.
    Pow(Box<Expr>, Box<Expr>),
    /// Exponential function: exp(a).
    Exp(Box<Expr>),
    /// Natural logarithm: log(a).
    Log(Box<Expr>),
    /// Square root: sqrt(a).
    Sqrt(Box<Expr>),
}

impl Expr {
    /// Convenience constructor for exponentiation.
    pub fn pow(base: Expr, exp: Expr) -> Self {
        Expr::Pow(Box::new(base), Box::new(exp))
    }

    /// Multiply expression by a scalar constant.
    pub fn scale(self, c: f64) -> Self {
        Expr::Const(c) * self
    }

    /// Evaluate the expression given concrete variable values.
    pub fn eval(&self, vars: &ProblemSize) -> f64 {
        match self {
            Expr::Const(c) => *c,
            Expr::Var(name) => vars.get(name).unwrap_or(0) as f64,
            Expr::Add(a, b) => a.eval(vars) + b.eval(vars),
            Expr::Mul(a, b) => a.eval(vars) * b.eval(vars),
            Expr::Pow(base, exp) => base.eval(vars).powf(exp.eval(vars)),
            Expr::Exp(a) => a.eval(vars).exp(),
            Expr::Log(a) => a.eval(vars).ln(),
            Expr::Sqrt(a) => a.eval(vars).sqrt(),
        }
    }

    /// Collect all variable names referenced in this expression.
    pub fn variables(&self) -> HashSet<&'static str> {
        let mut vars = HashSet::new();
        self.collect_variables(&mut vars);
        vars
    }

    fn collect_variables(&self, vars: &mut HashSet<&'static str>) {
        match self {
            Expr::Const(_) => {}
            Expr::Var(name) => {
                vars.insert(name);
            }
            Expr::Add(a, b) | Expr::Mul(a, b) | Expr::Pow(a, b) => {
                a.collect_variables(vars);
                b.collect_variables(vars);
            }
            Expr::Exp(a) | Expr::Log(a) | Expr::Sqrt(a) => {
                a.collect_variables(vars);
            }
        }
    }

    /// Substitute variables with other expressions.
    pub fn substitute(&self, mapping: &HashMap<&str, &Expr>) -> Expr {
        match self {
            Expr::Const(c) => Expr::Const(*c),
            Expr::Var(name) => {
                if let Some(replacement) = mapping.get(name) {
                    (*replacement).clone()
                } else {
                    Expr::Var(name)
                }
            }
            Expr::Add(a, b) => a.substitute(mapping) + b.substitute(mapping),
            Expr::Mul(a, b) => a.substitute(mapping) * b.substitute(mapping),
            Expr::Pow(a, b) => Expr::pow(a.substitute(mapping), b.substitute(mapping)),
            Expr::Exp(a) => Expr::Exp(Box::new(a.substitute(mapping))),
            Expr::Log(a) => Expr::Log(Box::new(a.substitute(mapping))),
            Expr::Sqrt(a) => Expr::Sqrt(Box::new(a.substitute(mapping))),
        }
    }

    /// Parse an expression string into an `Expr` at runtime.
    ///
    /// **Memory note:** Variable names are leaked to `&'static str` via `Box::leak`
    /// since `Expr::Var` requires static lifetimes. Each unique variable name leaks
    /// a small allocation that is never freed. This is acceptable for testing and
    /// one-time cross-check evaluation, but should not be used in hot loops with
    /// dynamic input.
    ///
    /// # Panics
    /// Panics if the expression string has invalid syntax.
    pub fn parse(input: &str) -> Expr {
        parse_to_expr(input)
            .unwrap_or_else(|e| panic!("failed to parse expression \"{input}\": {e}"))
    }

    /// Check if this expression is a polynomial (no exp/log/sqrt, integer exponents only).
    pub fn is_polynomial(&self) -> bool {
        match self {
            Expr::Const(_) | Expr::Var(_) => true,
            Expr::Add(a, b) | Expr::Mul(a, b) => a.is_polynomial() && b.is_polynomial(),
            Expr::Pow(base, exp) => {
                base.is_polynomial()
                    && matches!(exp.as_ref(), Expr::Const(c) if *c >= 0.0 && (*c - c.round()).abs() < 1e-10)
            }
            Expr::Exp(_) | Expr::Log(_) | Expr::Sqrt(_) => false,
        }
    }

    /// Check whether this expression is suitable for asymptotic complexity notation.
    ///
    /// This is intentionally conservative for symbolic size formulas:
    /// - rejects explicit multiplicative constant factors like `3 * n`
    /// - rejects additive constant terms like `n + 1`
    /// - allows constants used as exponents (e.g. `n^(1/3)`)
    /// - allows constants used as exponential bases (e.g. `2^n`)
    ///
    /// The goal is to accept expressions that already look like reduced
    /// asymptotic notation, rather than exact-count formulas.
    pub fn is_valid_complexity_notation(&self) -> bool {
        self.is_valid_complexity_notation_inner()
    }

    fn is_valid_complexity_notation_inner(&self) -> bool {
        match self {
            Expr::Const(c) => (*c - 1.0).abs() < 1e-10,
            Expr::Var(_) => true,
            Expr::Add(a, b) => {
                a.constant_value().is_none()
                    && b.constant_value().is_none()
                    && a.is_valid_complexity_notation_inner()
                    && b.is_valid_complexity_notation_inner()
            }
            Expr::Mul(a, b) => {
                a.constant_value().is_none()
                    && b.constant_value().is_none()
                    && a.is_valid_complexity_notation_inner()
                    && b.is_valid_complexity_notation_inner()
            }
            Expr::Pow(base, exp) => {
                let base_is_constant = base.constant_value().is_some();
                let exp_is_constant = exp.constant_value().is_some();

                let base_ok = if base_is_constant {
                    base.is_valid_exponential_base()
                } else {
                    base.is_valid_complexity_notation_inner()
                };

                let exp_ok = if exp_is_constant {
                    true
                } else {
                    exp.is_valid_complexity_notation_inner()
                };

                base_ok && exp_ok
            }
            Expr::Exp(a) | Expr::Log(a) | Expr::Sqrt(a) => a.is_valid_complexity_notation_inner(),
        }
    }

    fn is_valid_exponential_base(&self) -> bool {
        self.constant_value().is_some_and(|c| c > 0.0)
    }

    pub(crate) fn constant_value(&self) -> Option<f64> {
        match self {
            Expr::Const(c) => Some(*c),
            Expr::Var(_) => None,
            Expr::Add(a, b) => Some(a.constant_value()? + b.constant_value()?),
            Expr::Mul(a, b) => Some(a.constant_value()? * b.constant_value()?),
            Expr::Pow(base, exp) => Some(base.constant_value()?.powf(exp.constant_value()?)),
            Expr::Exp(a) => Some(a.constant_value()?.exp()),
            Expr::Log(a) => Some(a.constant_value()?.ln()),
            Expr::Sqrt(a) => Some(a.constant_value()?.sqrt()),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Const(c) => {
                let ci = c.round() as i64;
                if (*c - ci as f64).abs() < 1e-10 {
                    write!(f, "{ci}")
                } else {
                    write!(f, "{c}")
                }
            }
            Expr::Var(name) => write!(f, "{name}"),
            Expr::Add(a, b) => write!(f, "{a} + {b}"),
            Expr::Mul(a, b) => {
                let left = if matches!(a.as_ref(), Expr::Add(_, _)) {
                    format!("({a})")
                } else {
                    format!("{a}")
                };
                let right = if matches!(b.as_ref(), Expr::Add(_, _)) {
                    format!("({b})")
                } else {
                    format!("{b}")
                };
                write!(f, "{left} * {right}")
            }
            Expr::Pow(base, exp) => {
                // Special case: x^0.5 → sqrt(x)
                if let Expr::Const(e) = exp.as_ref() {
                    if (*e - 0.5).abs() < 1e-15 {
                        return write!(f, "sqrt({base})");
                    }
                }
                let base_str = if matches!(base.as_ref(), Expr::Add(_, _) | Expr::Mul(_, _)) {
                    format!("({base})")
                } else {
                    format!("{base}")
                };
                let exp_str = if matches!(exp.as_ref(), Expr::Add(_, _) | Expr::Mul(_, _)) {
                    format!("({exp})")
                } else {
                    format!("{exp}")
                };
                write!(f, "{base_str}^{exp_str}")
            }
            Expr::Exp(a) => write!(f, "exp({a})"),
            Expr::Log(a) => write!(f, "log({a})"),
            Expr::Sqrt(a) => write!(f, "sqrt({a})"),
        }
    }
}

impl std::ops::Add for Expr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Expr::Add(Box::new(self), Box::new(other))
    }
}

impl std::ops::Mul for Expr {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Expr::Mul(Box::new(self), Box::new(other))
    }
}

impl std::ops::Sub for Expr {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + Expr::Const(-1.0) * other
    }
}

impl std::ops::Div for Expr {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * Expr::pow(other, Expr::Const(-1.0))
    }
}

impl std::ops::Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self {
        Expr::Const(-1.0) * self
    }
}

/// Error returned when analyzing asymptotic behavior.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AsymptoticAnalysisError {
    Unsupported(String),
}

impl fmt::Display for AsymptoticAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(expr) => write!(f, "unsupported asymptotic expression: {expr}"),
        }
    }
}

impl std::error::Error for AsymptoticAnalysisError {}

/// Error returned when exact canonicalization fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CanonicalizationError {
    /// Expression cannot be canonicalized (e.g., variable in both base and exponent).
    Unsupported(String),
}

impl fmt::Display for CanonicalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(expr) => {
                write!(f, "unsupported expression for canonicalization: {expr}")
            }
        }
    }
}

impl std::error::Error for CanonicalizationError {}

/// Return a normalized `Expr` representing the asymptotic behavior of `expr`.
///
/// This is now a compatibility wrapper for `big_o_normal_form()`.
pub fn asymptotic_normal_form(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    crate::big_o::big_o_normal_form(expr)
}

// --- Runtime expression parser ---

/// Parse an expression string into an `Expr`.
///
/// Uses the same grammar as the proc macro parser. Variable names are leaked
/// to `&'static str` for compatibility with `Expr::Var`.
fn parse_to_expr(input: &str) -> Result<Expr, String> {
    let tokens = tokenize_expr(input)?;
    let mut parser = ExprParser::new(tokens);
    let expr = parser.parse_additive()?;
    if parser.pos != parser.tokens.len() {
        return Err(format!("trailing tokens at position {}", parser.pos));
    }
    Ok(expr)
}

#[derive(Debug, Clone, PartialEq)]
enum ExprToken {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

fn tokenize_expr(input: &str) -> Result<Vec<ExprToken>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '+' => {
                chars.next();
                tokens.push(ExprToken::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(ExprToken::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(ExprToken::Star);
            }
            '/' => {
                chars.next();
                tokens.push(ExprToken::Slash);
            }
            '^' => {
                chars.next();
                tokens.push(ExprToken::Caret);
            }
            '(' => {
                chars.next();
                tokens.push(ExprToken::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(ExprToken::RParen);
            }
            c if c.is_ascii_digit() || c == '.' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(ExprToken::Number(
                    num.parse().map_err(|_| format!("invalid number: {num}"))?,
                ));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(ExprToken::Ident(ident));
            }
            _ => return Err(format!("unexpected character: '{ch}'")),
        }
    }
    Ok(tokens)
}

struct ExprParser {
    tokens: Vec<ExprToken>,
    pos: usize,
}

impl ExprParser {
    fn new(tokens: Vec<ExprToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&ExprToken> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<ExprToken> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &ExprToken) -> Result<(), String> {
        match self.advance() {
            Some(ref tok) if tok == expected => Ok(()),
            Some(tok) => Err(format!("expected {expected:?}, got {tok:?}")),
            None => Err(format!("expected {expected:?}, got end of input")),
        }
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;
        while matches!(self.peek(), Some(ExprToken::Plus) | Some(ExprToken::Minus)) {
            let op = self.advance().unwrap();
            let right = self.parse_multiplicative()?;
            left = match op {
                ExprToken::Plus => left + right,
                ExprToken::Minus => left - right,
                _ => unreachable!(),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;
        while matches!(self.peek(), Some(ExprToken::Star) | Some(ExprToken::Slash)) {
            let op = self.advance().unwrap();
            let right = self.parse_power()?;
            left = match op {
                ExprToken::Star => left * right,
                ExprToken::Slash => left / right,
                _ => unreachable!(),
            };
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let base = self.parse_unary()?;
        if matches!(self.peek(), Some(ExprToken::Caret)) {
            self.advance();
            let exp = self.parse_power()?; // right-associative
            Ok(Expr::pow(base, exp))
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), Some(ExprToken::Minus)) {
            self.advance();
            let expr = self.parse_unary()?;
            Ok(-expr)
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(ExprToken::Number(n)) => Ok(Expr::Const(n)),
            Some(ExprToken::Ident(name)) => {
                if matches!(self.peek(), Some(ExprToken::LParen)) {
                    self.advance();
                    let arg = self.parse_additive()?;
                    self.expect(&ExprToken::RParen)?;
                    match name.as_str() {
                        "exp" => Ok(Expr::Exp(Box::new(arg))),
                        "log" => Ok(Expr::Log(Box::new(arg))),
                        "sqrt" => Ok(Expr::Sqrt(Box::new(arg))),
                        _ => Err(format!("unknown function: {name}")),
                    }
                } else {
                    // Leak the string to get &'static str for Expr::Var
                    let leaked: &'static str = Box::leak(name.into_boxed_str());
                    Ok(Expr::Var(leaked))
                }
            }
            Some(ExprToken::LParen) => {
                let expr = self.parse_additive()?;
                self.expect(&ExprToken::RParen)?;
                Ok(expr)
            }
            Some(tok) => Err(format!("unexpected token: {tok:?}")),
            None => Err("unexpected end of input".to_string()),
        }
    }
}

#[cfg(test)]
#[path = "unit_tests/expr.rs"]
mod tests;
