//! Polynomial representation for reduction overhead.

use crate::types::ProblemSize;
use std::fmt;
use std::ops::Add;

/// A monomial: coefficient × Π(variable^exponent)
#[derive(Clone, Debug, PartialEq)]
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
}

/// A polynomial: Σ monomials
#[derive(Clone, Debug, PartialEq)]
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
