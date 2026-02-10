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

#[test]
fn test_display_monomial_constant_int() {
    assert_eq!(format!("{}", Monomial::constant(5.0)), "5");
}

#[test]
fn test_display_monomial_constant_float() {
    assert_eq!(format!("{}", Monomial::constant(3.5)), "3.5");
}

#[test]
fn test_display_monomial_single_var() {
    assert_eq!(format!("{}", Monomial::var("n")), "n");
}

#[test]
fn test_display_monomial_neg_one_coeff() {
    assert_eq!(format!("{}", Monomial::var("n").scale(-1.0)), "-n");
}

#[test]
fn test_display_monomial_scaled_var() {
    assert_eq!(format!("{}", Monomial::var("n").scale(3.0)), "3 * n");
}

#[test]
fn test_display_monomial_var_pow() {
    assert_eq!(format!("{}", Monomial::var_pow("n", 2)), "n^2");
}

#[test]
fn test_display_monomial_multi_var() {
    let m = Monomial {
        coefficient: 2.0,
        variables: vec![("n", 1), ("m", 2)],
    };
    assert_eq!(format!("{m}"), "2 * n * m^2");
}

#[test]
fn test_display_monomial_float_coeff_var() {
    let m = Monomial {
        coefficient: 1.5,
        variables: vec![("n", 1)],
    };
    assert_eq!(format!("{m}"), "1.5 * n");
}

#[test]
fn test_display_polynomial_zero() {
    assert_eq!(format!("{}", Polynomial::zero()), "0");
}

#[test]
fn test_display_polynomial_single_term() {
    assert_eq!(format!("{}", Polynomial::var("n").scale(3.0)), "3 * n");
}

#[test]
fn test_display_polynomial_addition() {
    let p = Polynomial::var("n").scale(3.0) + Polynomial::var("m").scale(2.0);
    assert_eq!(format!("{p}"), "3 * n + 2 * m");
}

#[test]
fn test_display_polynomial_subtraction() {
    let p = Polynomial::var("n").scale(3.0) + Polynomial::var("m").scale(-2.0);
    assert_eq!(format!("{p}"), "3 * n - 2 * m");
}

#[test]
fn test_poly_macro_product() {
    let size = ProblemSize::new(vec![("a", 3), ("b", 4)]);
    assert_eq!(poly!(a * b).evaluate(&size), 12.0);
    assert_eq!(format!("{}", poly!(a * b)), "a * b");
}

#[test]
fn test_poly_macro_scaled_product() {
    let size = ProblemSize::new(vec![("a", 3), ("b", 4)]);
    assert_eq!(poly!(5 * a * b).evaluate(&size), 60.0);
    assert_eq!(format!("{}", poly!(5 * a * b)), "5 * a * b");
}
