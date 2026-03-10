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
    /// Convenience constructor for addition.
    pub fn add(a: Expr, b: Expr) -> Self {
        Expr::Add(Box::new(a), Box::new(b))
    }

    /// Convenience constructor for multiplication.
    pub fn mul(a: Expr, b: Expr) -> Self {
        Expr::Mul(Box::new(a), Box::new(b))
    }

    /// Convenience constructor for exponentiation.
    pub fn pow(base: Expr, exp: Expr) -> Self {
        Expr::Pow(Box::new(base), Box::new(exp))
    }

    /// Multiply expression by a scalar constant.
    pub fn scale(self, c: f64) -> Self {
        Expr::mul(Expr::Const(c), self)
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
            Expr::Add(a, b) => Expr::add(a.substitute(mapping), b.substitute(mapping)),
            Expr::Mul(a, b) => Expr::mul(a.substitute(mapping), b.substitute(mapping)),
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

    fn constant_value(&self) -> Option<f64> {
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

/// Return a normalized `Expr` representing the asymptotic behavior of `expr`.
///
/// Normalization includes:
/// - commutativity/associativity of `+` and `*`
/// - removal of positive constant factors
/// - removal of additive constant terms
/// - normalization of `sqrt(x)` into `x^(1/2)`
/// - combination of repeated multiplicative factors
/// - canonical identities like `exp(a) * exp(b) = exp(a + b)`
pub fn asymptotic_normal_form(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    match expr {
        Expr::Const(c) => {
            if *c >= 0.0 {
                Ok(Expr::Const(1.0))
            } else {
                Err(AsymptoticAnalysisError::Unsupported(expr.to_string()))
            }
        }
        Expr::Var(name) => Ok(Expr::Var(name)),
        Expr::Add(a, b) => {
            let mut terms = Vec::new();
            collect_sum_term(a, &mut terms)?;
            collect_sum_term(b, &mut terms)?;
            Ok(build_sum(terms))
        }
        Expr::Mul(a, b) => {
            let mut factors = Vec::new();
            collect_product_factor(a, &mut factors)?;
            collect_product_factor(b, &mut factors)?;
            Ok(build_product(factors))
        }
        Expr::Pow(base, exp) => normalize_pow(base, exp, expr),
        Expr::Exp(arg) => Ok(build_exp(asymptotic_normal_form(arg)?)),
        Expr::Log(arg) => Ok(build_log(asymptotic_normal_form(arg)?)),
        Expr::Sqrt(arg) => Ok(build_pow(asymptotic_normal_form(arg)?, 0.5)),
    }
}

fn normalize_pow(base: &Expr, exp: &Expr, whole: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    match (base.constant_value(), exp.constant_value()) {
        (Some(c), Some(_)) => {
            if c >= 0.0 {
                Ok(Expr::Const(1.0))
            } else {
                Err(AsymptoticAnalysisError::Unsupported(whole.to_string()))
            }
        }
        (Some(base_const), None) => {
            if base_const <= 0.0 || (base_const - 1.0).abs() < 1e-10 {
                return Err(AsymptoticAnalysisError::Unsupported(whole.to_string()));
            }
            Ok(build_exp_base(base_const, asymptotic_normal_form(exp)?))
        }
        (None, Some(exp_const)) => {
            if exp_const < 0.0 {
                return Err(AsymptoticAnalysisError::Unsupported(whole.to_string()));
            }
            Ok(build_pow(asymptotic_normal_form(base)?, exp_const))
        }
        (None, None) => Err(AsymptoticAnalysisError::Unsupported(whole.to_string())),
    }
}

fn collect_sum_term(expr: &Expr, out: &mut Vec<Expr>) -> Result<(), AsymptoticAnalysisError> {
    if let Some(c) = expr.constant_value() {
        if c >= 0.0 {
            return Ok(());
        }
        return Err(AsymptoticAnalysisError::Unsupported(expr.to_string()));
    }
    out.push(asymptotic_normal_form(expr)?);
    Ok(())
}

fn collect_product_factor(expr: &Expr, out: &mut Vec<Expr>) -> Result<(), AsymptoticAnalysisError> {
    if let Some(c) = expr.constant_value() {
        if c > 0.0 {
            return Ok(());
        }
        return Err(AsymptoticAnalysisError::Unsupported(expr.to_string()));
    }
    out.push(asymptotic_normal_form(expr)?);
    Ok(())
}

fn build_sum(terms: Vec<Expr>) -> Expr {
    let mut flat = Vec::new();
    for term in terms {
        match term {
            Expr::Const(c) if (c - 1.0).abs() < 1e-10 => {}
            Expr::Add(a, b) => {
                flat.push(*a);
                flat.push(*b);
            }
            other => flat.push(other),
        }
    }

    if flat.is_empty() {
        return Expr::Const(1.0);
    }

    let mut dedup = HashMap::<String, Expr>::new();
    for term in flat {
        dedup.entry(term.to_string()).or_insert(term);
    }

    let mut values: Vec<_> = dedup.into_values().collect();
    values.sort_by_key(|term| term.to_string());
    combine_add_chain(values)
}

fn build_product(factors: Vec<Expr>) -> Expr {
    let mut flat = Vec::new();
    for factor in factors {
        match factor {
            Expr::Const(c) if (c - 1.0).abs() < 1e-10 => {}
            Expr::Mul(a, b) => {
                flat.push(*a);
                flat.push(*b);
            }
            other => flat.push(other),
        }
    }

    if flat.is_empty() {
        return Expr::Const(1.0);
    }

    let mut power_terms: HashMap<String, (f64, Expr)> = HashMap::new();
    let mut natural_exp_args = Vec::new();
    let mut base_exp_args: HashMap<String, (f64, Vec<Expr>)> = HashMap::new();

    for factor in flat {
        match factor {
            Expr::Exp(arg) => natural_exp_args.push(*arg),
            Expr::Pow(base, exp)
                if base.constant_value().is_some() && exp.constant_value().is_none() =>
            {
                let base_const = base.constant_value().unwrap();
                let key = format_float(base_const);
                base_exp_args
                    .entry(key)
                    .or_insert_with(|| (base_const, Vec::new()))
                    .1
                    .push(*exp);
            }
            other => {
                let (base, exp) = into_base_and_exponent(other);
                let key = base.to_string();
                power_terms
                    .entry(key)
                    .and_modify(|(total, _)| *total += exp)
                    .or_insert((exp, base));
            }
        }
    }

    let mut result = Vec::new();

    for (_key, (exp, base)) in power_terms {
        if exp.abs() < 1e-10 {
            continue;
        }
        result.push(build_pow(base, exp));
    }

    if !natural_exp_args.is_empty() {
        result.push(build_exp(build_sum(natural_exp_args)));
    }

    for (_key, (base, args)) in base_exp_args {
        result.push(build_exp_base(base, build_sum(args)));
    }

    if result.is_empty() {
        return Expr::Const(1.0);
    }

    result.sort_by_key(|factor| factor.to_string());
    combine_mul_chain(result)
}

fn build_pow(base: Expr, exp: f64) -> Expr {
    if exp.abs() < 1e-10 {
        return Expr::Const(1.0);
    }
    if (exp - 1.0).abs() < 1e-10 {
        return base;
    }

    match base {
        Expr::Const(c) if (c - 1.0).abs() < 1e-10 => Expr::Const(1.0),
        Expr::Pow(inner, inner_exp) => {
            if let Expr::Const(inner_exp_value) = inner_exp.as_ref() {
                build_pow(*inner, inner_exp_value * exp)
            } else {
                Expr::Pow(
                    Box::new(Expr::Pow(inner, inner_exp)),
                    Box::new(Expr::Const(exp)),
                )
            }
        }
        Expr::Mul(a, b) => build_product(vec![build_pow(*a, exp), build_pow(*b, exp)]),
        other => Expr::Pow(Box::new(other), Box::new(Expr::Const(exp))),
    }
}

fn build_exp(arg: Expr) -> Expr {
    match arg {
        Expr::Log(inner) => *inner,
        other => Expr::Exp(Box::new(other)),
    }
}

fn build_exp_base(base: f64, arg: Expr) -> Expr {
    if (base - std::f64::consts::E).abs() < 1e-10 {
        return build_exp(arg);
    }
    Expr::Pow(Box::new(Expr::Const(base)), Box::new(arg))
}

fn build_log(arg: Expr) -> Expr {
    match arg {
        Expr::Const(_) => Expr::Const(1.0),
        Expr::Pow(base, exp) if exp.constant_value().is_some() => build_log(*base),
        Expr::Pow(base, exp) if base.constant_value().is_some() => *exp,
        Expr::Exp(inner) => *inner,
        other => Expr::Log(Box::new(other)),
    }
}

fn into_base_and_exponent(expr: Expr) -> (Expr, f64) {
    match expr {
        Expr::Pow(base, exp) => match *exp {
            Expr::Const(exp_value) => (*base, exp_value),
            other => (Expr::Pow(base, Box::new(other)), 1.0),
        },
        other => (other, 1.0),
    }
}

fn combine_add_chain(mut terms: Vec<Expr>) -> Expr {
    if terms.is_empty() {
        return Expr::Const(1.0);
    }
    let mut expr = terms.remove(0);
    for term in terms {
        expr = Expr::add(expr, term);
    }
    expr
}

fn combine_mul_chain(mut factors: Vec<Expr>) -> Expr {
    if factors.is_empty() {
        return Expr::Const(1.0);
    }
    let mut expr = factors.remove(0);
    for factor in factors {
        expr = Expr::mul(expr, factor);
    }
    expr
}

fn format_float(value: f64) -> String {
    let rounded = value.round();
    if (value - rounded).abs() < 1e-10 {
        return format!("{}", rounded as i64);
    }

    let mut s = format!("{value:.10}");
    while s.contains('.') && s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    s
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
                ExprToken::Plus => Expr::add(left, right),
                ExprToken::Minus => Expr::add(left, Expr::mul(Expr::Const(-1.0), right)),
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
                ExprToken::Star => Expr::mul(left, right),
                ExprToken::Slash => Expr::mul(left, Expr::pow(right, Expr::Const(-1.0))),
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
            Ok(Expr::mul(Expr::Const(-1.0), expr))
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
