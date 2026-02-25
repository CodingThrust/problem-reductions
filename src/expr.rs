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
                write!(f, "{base_str}^{exp}")
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

impl From<crate::polynomial::Polynomial> for Expr {
    fn from(poly: crate::polynomial::Polynomial) -> Self {
        let terms: Vec<Expr> = poly
            .terms
            .iter()
            .map(|mono| {
                // Build monomial: coefficient * Π(var^exp)
                let mut expr = Expr::Const(mono.coefficient);
                for &(name, exp) in &mono.variables {
                    let var_expr = if exp == 1 {
                        Expr::Var(name)
                    } else {
                        Expr::pow(Expr::Var(name), Expr::Const(exp as f64))
                    };
                    expr = Expr::mul(expr, var_expr);
                }
                // Simplify `1.0 * x` to just `x` for single-variable monomials
                if let Expr::Mul(ref a, ref b) = expr {
                    if matches!(a.as_ref(), Expr::Const(c) if (*c - 1.0).abs() < 1e-15) {
                        return b.as_ref().clone();
                    }
                }
                expr
            })
            .collect();

        if terms.is_empty() {
            return Expr::Const(0.0);
        }

        terms.into_iter().reduce(Expr::add).unwrap()
    }
}

#[cfg(test)]
#[path = "unit_tests/expr.rs"]
mod tests;
