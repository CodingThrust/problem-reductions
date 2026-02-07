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
    let p = Polynomial::var("n").scale(3.0) + Polynomial::var("m").scale(2.0);

    let size = ProblemSize::new(vec![("n", 10), ("m", 5)]);
    assert_eq!(p.evaluate(&size), 40.0); // 3*10 + 2*5
}

#[test]
fn test_polynomial_complex() {
    // n^2 + 3m
    let p = Polynomial::var_pow("n", 2) + Polynomial::var("m").scale(3.0);

    let size = ProblemSize::new(vec![("n", 4), ("m", 2)]);
    assert_eq!(p.evaluate(&size), 22.0); // 16 + 6
}

#[test]
fn test_poly_macro() {
    let size = ProblemSize::new(vec![("n", 5), ("m", 3)]);

    assert_eq!(poly!(n).evaluate(&size), 5.0);
    assert_eq!(poly!(n ^ 2).evaluate(&size), 25.0);
    assert_eq!(poly!(3 * n).evaluate(&size), 15.0);
    assert_eq!(poly!(2 * m ^ 2).evaluate(&size), 18.0);
}

#[test]
fn test_missing_variable() {
    let p = Polynomial::var("missing");
    let size = ProblemSize::new(vec![("n", 10)]);
    assert_eq!(p.evaluate(&size), 0.0); // missing var = 0
}

#[test]
fn test_polynomial_zero() {
    let p = Polynomial::zero();
    let size = ProblemSize::new(vec![("n", 100)]);
    assert_eq!(p.evaluate(&size), 0.0);
}

#[test]
fn test_polynomial_constant() {
    let p = Polynomial::constant(42.0);
    let size = ProblemSize::new(vec![("n", 100)]);
    assert_eq!(p.evaluate(&size), 42.0);
}

#[test]
fn test_monomial_scale() {
    let m = Monomial::var("n").scale(3.0);
    let size = ProblemSize::new(vec![("n", 10)]);
    assert_eq!(m.evaluate(&size), 30.0);
}

#[test]
fn test_polynomial_scale() {
    let p = Polynomial::var("n").scale(5.0);
    let size = ProblemSize::new(vec![("n", 10)]);
    assert_eq!(p.evaluate(&size), 50.0);
}

#[test]
fn test_monomial_multi_variable() {
    // n * m^2
    let m = Monomial {
        coefficient: 1.0,
        variables: vec![("n", 1), ("m", 2)],
    };
    let size = ProblemSize::new(vec![("n", 2), ("m", 3)]);
    assert_eq!(m.evaluate(&size), 18.0); // 2 * 9
}
