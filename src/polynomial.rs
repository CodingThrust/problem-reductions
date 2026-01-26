//! Polynomial representation for reduction overhead.

use crate::types::ProblemSize;

/// A monomial: coefficient × Π(variable^exponent)
#[derive(Clone, Debug, PartialEq)]
pub struct Monomial {
    pub coefficient: f64,
    pub variables: Vec<(&'static str, u8)>,
}

impl Monomial {
    pub fn constant(c: f64) -> Self {
        Self { coefficient: c, variables: vec![] }
    }

    pub fn var(name: &'static str) -> Self {
        Self { coefficient: 1.0, variables: vec![(name, 1)] }
    }

    pub fn var_pow(name: &'static str, exp: u8) -> Self {
        Self { coefficient: 1.0, variables: vec![(name, exp)] }
    }

    pub fn scale(mut self, c: f64) -> Self {
        self.coefficient *= c;
        self
    }

    pub fn evaluate(&self, size: &ProblemSize) -> f64 {
        let var_product: f64 = self.variables.iter()
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
        Self { terms: vec![Monomial::constant(c)] }
    }

    pub fn var(name: &'static str) -> Self {
        Self { terms: vec![Monomial::var(name)] }
    }

    pub fn var_pow(name: &'static str, exp: u8) -> Self {
        Self { terms: vec![Monomial::var_pow(name, exp)] }
    }

    pub fn scale(mut self, c: f64) -> Self {
        for term in &mut self.terms {
            term.coefficient *= c;
        }
        self
    }

    pub fn add(mut self, other: Self) -> Self {
        self.terms.extend(other.terms);
        self
    }

    pub fn evaluate(&self, size: &ProblemSize) -> f64 {
        self.terms.iter().map(|m| m.evaluate(size)).sum()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monomial_constant() {
        let m = Monomial::constant(5.0);
        let size = ProblemSize::new(vec![("n", 10)]);
        assert_eq!(m.evaluate(&size), 5.0);
    }

    #[test]
    fn test_monomial_variable() {
        let m = Monomial::var("n");
        let size = ProblemSize::new(vec![("n", 10)]);
        assert_eq!(m.evaluate(&size), 10.0);
    }

    #[test]
    fn test_monomial_var_pow() {
        let m = Monomial::var_pow("n", 2);
        let size = ProblemSize::new(vec![("n", 5)]);
        assert_eq!(m.evaluate(&size), 25.0);
    }

    #[test]
    fn test_polynomial_add() {
        // 3n + 2m
        let p = Polynomial::var("n").scale(3.0)
            .add(Polynomial::var("m").scale(2.0));

        let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
        assert_eq!(p.evaluate(&size), 40.0);  // 3*10 + 2*5
    }

    #[test]
    fn test_polynomial_complex() {
        // n^2 + 3m
        let p = Polynomial::var_pow("n", 2)
            .add(Polynomial::var("m").scale(3.0));

        let size = ProblemSize::new(vec![("n", 4), ("m", 2)]);
        assert_eq!(p.evaluate(&size), 22.0);  // 16 + 6
    }

    #[test]
    fn test_poly_macro() {
        let size = ProblemSize::new(vec![("n", 5), ("m", 3)]);

        assert_eq!(poly!(n).evaluate(&size), 5.0);
        assert_eq!(poly!(n^2).evaluate(&size), 25.0);
        assert_eq!(poly!(3 * n).evaluate(&size), 15.0);
        assert_eq!(poly!(2 * m^2).evaluate(&size), 18.0);
    }

    #[test]
    fn test_missing_variable() {
        let p = Polynomial::var("missing");
        let size = ProblemSize::new(vec![("n", 10)]);
        assert_eq!(p.evaluate(&size), 0.0);  // missing var = 0
    }
}
