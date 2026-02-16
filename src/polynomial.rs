//! Polynomial representation for reduction overhead.

use crate::types::ProblemSize;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Add;

/// A monomial: coefficient × Π(variable^exponent)
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Monomial {
    pub coefficient: f64,
    pub variables: Vec<(&'static str, u8)>,
}

impl Monomial {
    pub fn constant(c: f64) -> Self {
        Self {
            coefficient: c,
            variables: vec![],
        }
    }

    pub fn var(name: &'static str) -> Self {
        Self {
            coefficient: 1.0,
            variables: vec![(name, 1)],
        }
    }

    pub fn var_pow(name: &'static str, exp: u8) -> Self {
        Self {
            coefficient: 1.0,
            variables: vec![(name, exp)],
        }
    }

    pub fn scale(mut self, c: f64) -> Self {
        self.coefficient *= c;
        self
    }

    pub fn evaluate(&self, size: &ProblemSize) -> f64 {
        let var_product: f64 = self
            .variables
            .iter()
            .map(|(name, exp)| {
                let val = size.get(name).unwrap_or(0) as f64;
                val.powi(*exp as i32)
            })
            .product();
        self.coefficient * var_product
    }

    /// Multiply two monomials.
    pub fn mul(&self, other: &Monomial) -> Monomial {
        let mut variables = self.variables.clone();
        variables.extend_from_slice(&other.variables);
        Monomial {
            coefficient: self.coefficient * other.coefficient,
            variables,
        }
    }

    /// Normalize: sort variables by name, merge duplicate entries.
    pub fn normalize(&mut self) {
        self.variables.sort_by_key(|(name, _)| *name);
        let mut merged: Vec<(&'static str, u8)> = Vec::new();
        for &(name, exp) in &self.variables {
            if let Some(last) = merged.last_mut() {
                if last.0 == name {
                    last.1 += exp;
                    continue;
                }
            }
            merged.push((name, exp));
        }
        // Remove zero-exponent variables
        merged.retain(|&(_, exp)| exp > 0);
        self.variables = merged;
    }

    /// Variable signature for like-term comparison (after normalization).
    fn var_signature(&self) -> &[(&'static str, u8)] {
        &self.variables
    }
}

/// A polynomial: Σ monomials
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Polynomial {
    pub terms: Vec<Monomial>,
}

impl Polynomial {
    pub fn zero() -> Self {
        Self { terms: vec![] }
    }

    pub fn constant(c: f64) -> Self {
        Self {
            terms: vec![Monomial::constant(c)],
        }
    }

    pub fn var(name: &'static str) -> Self {
        Self {
            terms: vec![Monomial::var(name)],
        }
    }

    pub fn var_pow(name: &'static str, exp: u8) -> Self {
        Self {
            terms: vec![Monomial::var_pow(name, exp)],
        }
    }

    /// Create a polynomial with a single monomial that is a product of two variables.
    pub fn var_product(a: &'static str, b: &'static str) -> Self {
        Self {
            terms: vec![Monomial {
                coefficient: 1.0,
                variables: vec![(a, 1), (b, 1)],
            }],
        }
    }

    pub fn scale(mut self, c: f64) -> Self {
        for term in &mut self.terms {
            term.coefficient *= c;
        }
        self
    }

    pub fn evaluate(&self, size: &ProblemSize) -> f64 {
        self.terms.iter().map(|m| m.evaluate(size)).sum()
    }

    /// Collect all variable names referenced by this polynomial.
    pub fn variable_names(&self) -> HashSet<&'static str> {
        self.terms
            .iter()
            .flat_map(|m| m.variables.iter().map(|(name, _)| *name))
            .collect()
    }

    /// Multiply two polynomials.
    pub fn mul(&self, other: &Polynomial) -> Polynomial {
        let mut terms = Vec::new();
        for a in &self.terms {
            for b in &other.terms {
                terms.push(a.mul(b));
            }
        }
        let mut result = Polynomial { terms };
        result.normalize();
        result
    }

    /// Raise to a non-negative integer power.
    pub fn pow(&self, n: u8) -> Polynomial {
        match n {
            0 => Polynomial::constant(1.0),
            1 => self.clone(),
            _ => {
                let mut result = self.clone();
                for _ in 1..n {
                    result = result.mul(self);
                }
                result
            }
        }
    }

    /// Substitute variables with polynomials.
    ///
    /// Each variable in the polynomial is replaced by the corresponding
    /// polynomial from the mapping. Variables not in the mapping are left as-is.
    pub fn substitute(&self, mapping: &HashMap<&str, &Polynomial>) -> Polynomial {
        let mut result = Polynomial::zero();
        for mono in &self.terms {
            // Start with the coefficient
            let mut term_poly = Polynomial::constant(mono.coefficient);
            // Multiply by each variable's substitution raised to its exponent
            for &(name, exp) in &mono.variables {
                let var_poly = if let Some(&replacement) = mapping.get(name) {
                    replacement.pow(exp)
                } else {
                    Polynomial::var_pow(name, exp)
                };
                term_poly = term_poly.mul(&var_poly);
            }
            result = result + term_poly;
        }
        result.normalize();
        result
    }

    /// Normalize: normalize all monomials, then combine like terms.
    pub fn normalize(&mut self) {
        for term in &mut self.terms {
            term.normalize();
        }
        // Combine like terms
        let mut combined: Vec<Monomial> = Vec::new();
        for term in &self.terms {
            if let Some(existing) = combined
                .iter_mut()
                .find(|m| m.var_signature() == term.var_signature())
            {
                existing.coefficient += term.coefficient;
            } else {
                combined.push(term.clone());
            }
        }
        // Remove zero-coefficient terms
        combined.retain(|m| m.coefficient.abs() > 1e-15);
        self.terms = combined;
    }

    /// Return a normalized copy.
    pub fn normalized(&self) -> Polynomial {
        let mut p = self.clone();
        p.normalize();
        p
    }
}

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let coeff_i = self.coefficient.round() as i64;
        let is_int = (self.coefficient - coeff_i as f64).abs() < 1e-10;
        if self.variables.is_empty() {
            if is_int {
                write!(f, "{coeff_i}")
            } else {
                write!(f, "{}", self.coefficient)
            }
        } else {
            let has_coeff = if is_int {
                match coeff_i {
                    1 => false,
                    -1 => {
                        write!(f, "-")?;
                        false
                    }
                    _ => {
                        write!(f, "{coeff_i}")?;
                        true
                    }
                }
            } else {
                write!(f, "{}", self.coefficient)?;
                true
            };
            for (i, (name, exp)) in self.variables.iter().enumerate() {
                if has_coeff || i > 0 {
                    write!(f, " * ")?;
                }
                write!(f, "{name}")?;
                if *exp > 1 {
                    write!(f, "^{exp}")?;
                }
            }
            Ok(())
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.terms.is_empty() {
            write!(f, "0")
        } else {
            for (i, term) in self.terms.iter().enumerate() {
                if i > 0 {
                    if term.coefficient < 0.0 {
                        write!(f, " - ")?;
                        let negated = Monomial {
                            coefficient: -term.coefficient,
                            variables: term.variables.clone(),
                        };
                        write!(f, "{negated}")?;
                    } else {
                        write!(f, " + ")?;
                        write!(f, "{term}")?;
                    }
                } else {
                    write!(f, "{term}")?;
                }
            }
            Ok(())
        }
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.terms.extend(other.terms);
        self
    }
}

/// Convenience macro for building polynomials.
#[macro_export]
macro_rules! poly {
    // Single variable: poly!(n)
    ($name:ident) => {
        $crate::polynomial::Polynomial::var(stringify!($name))
    };
    // Variable with exponent: poly!(n^2)
    ($name:ident ^ $exp:literal) => {
        $crate::polynomial::Polynomial::var_pow(stringify!($name), $exp)
    };
    // Constant: poly!(5)
    ($c:literal) => {
        $crate::polynomial::Polynomial::constant($c as f64)
    };
    // Scaled variable: poly!(3 * n)
    ($c:literal * $name:ident) => {
        $crate::polynomial::Polynomial::var(stringify!($name)).scale($c as f64)
    };
    // Scaled variable with exponent: poly!(9 * n^2)
    ($c:literal * $name:ident ^ $exp:literal) => {
        $crate::polynomial::Polynomial::var_pow(stringify!($name), $exp).scale($c as f64)
    };
    // Product of two variables: poly!(a * b)
    ($a:ident * $b:ident) => {
        $crate::polynomial::Polynomial::var_product(stringify!($a), stringify!($b))
    };
    // Scaled product of two variables: poly!(3 * a * b)
    ($c:literal * $a:ident * $b:ident) => {
        $crate::polynomial::Polynomial::var_product(stringify!($a), stringify!($b)).scale($c as f64)
    };
}

#[cfg(test)]
#[path = "unit_tests/polynomial.rs"]
mod tests;
