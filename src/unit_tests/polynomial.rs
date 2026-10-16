use super::*;

fn poly(s: &str) -> Poly {
    Poly::from_expr(&Expr::parse(s)).unwrap()
}

#[test]
fn canonical_terms_remove_zero_powers_and_combine_constant_terms() {
    let n = Symbol::new("n").unwrap();
    let p = Poly::from_terms([
        (
            Monomial::from([(n, BigUint::zero())]),
            BigRational::from_integer(2.into()),
        ),
        (Monomial::new(), BigRational::from_integer(3.into())),
    ]);
    assert_eq!(p, Poly::constant(BigRational::from_integer(5.into())));
    assert!(p.variables().is_empty());
    assert_eq!(p.substitute(&BTreeMap::new()), Some(p.clone()));
}

#[test]
fn native_substitution_expands_signed_formulas_and_reports_missing_inputs() {
    let p = poly("n^2+n*m-m^2");
    let replacements = BTreeMap::from([
        (Symbol::new("n").unwrap(), poly("a+b")),
        (Symbol::new("m").unwrap(), poly("a-b")),
    ]);
    assert_eq!(p.substitute(&replacements), Some(poly("a^2+4*a*b-b^2")));
    assert!(p.substitute(&BTreeMap::new()).is_none());
    assert_eq!(
        Poly::zero().substitute(&BTreeMap::new()),
        Some(Poly::zero())
    );
}

#[test]
fn sparse_arithmetic_preserves_exact_coefficients_and_hull_signs() {
    assert_eq!(
        poly("n/2+m/3+1").mul(&poly("n/5-m/7")),
        poly("n^2/10-n*m/210+n/5-m^2/21-m/7")
    );
    assert_eq!(
        poly("n-1").pow(&5u8.into()),
        poly("n^5-5*n^4+10*n^3-10*n^2+5*n-1")
    );
    assert_eq!(poly("n-1").pow(&0u8.into()), Poly::one());
    assert_eq!(poly("n-1").add(&poly("1-n")), Poly::zero());
    assert_eq!(poly("n-m-1").hull(|m| m.is_empty()), poly("n-1"));
    assert_eq!(poly("n-m-1").positive_hull(), poly("n"));
    assert_eq!(
        pow_rational(BigRational::new(3.into(), 2.into()), &(-5).into()),
        BigRational::new(32.into(), 243.into())
    );
    assert_eq!(
        pow_rational(BigRational::new(3.into(), 2.into()), &0.into()),
        BigRational::one()
    );
}

#[test]
fn multivariate_powers_match_signed_multinomial_coefficients() {
    let factorial = |n: u64| (1..=n).product::<u64>();
    let n = Symbol::new("n").unwrap();
    let m = Symbol::new("m").unwrap();
    let expected = Poly::from_terms((0..=8u64).flat_map(|a| {
        let n = n.clone();
        let m = m.clone();
        (0..=8 - a).map(move |b| {
            let coefficient =
                BigInt::from(factorial(8) / (factorial(a) * factorial(b) * factorial(8 - a - b)));
            (
                Monomial::from([(n.clone(), a.into()), (m.clone(), b.into())]),
                BigRational::from_integer(if b % 2 == 0 {
                    coefficient
                } else {
                    -coefficient
                }),
            )
        })
    }));
    assert_eq!(poly("(n-m+1)^8"), expected);
    assert_eq!(
        poly("n^18446744073709551616+m").mul(&poly("n+m")),
        poly("n^18446744073709551617+m*n^18446744073709551616+n*m+m^2")
    );
    assert_eq!(
        poly("n^1000+m^1000").mul(&poly("n^1000-m^1000")),
        poly("n^2000-m^2000")
    );
}

#[test]
fn polynomial_conversion_rejects_unsupported_functions_and_exponents() {
    for expression in ["n/m", "sqrt(n)", "2^n", "log(n)", "exp(n)", "factorial(n)"] {
        assert!(
            Poly::from_expr(&Expr::parse(expression)).is_none(),
            "{expression}"
        );
    }
    assert_eq!(poly("(n+m)^2/2"), poly("n^2/2+n*m+m^2/2"));
    assert_eq!(
        poly("2^-3"),
        Poly::constant(BigRational::new(1.into(), 8.into()))
    );
    assert!(Poly::from_expr(&Expr::pow(Expr::integer(0), Expr::integer(-1))).is_none());
}
