//! Big-O asymptotic projection for canonical expressions.
//!
//! Takes the output of `canonical_form()` and projects it into an
//! asymptotic growth class by dropping dominated terms and constant factors.

use crate::canonical::canonical_form;
use crate::expr::{AsymptoticAnalysisError, CanonicalizationError, Expr};

#[derive(Clone, Debug)]
struct ProjectedTerm {
    expr: Expr,
    negative: bool,
}

/// Compute the Big-O normal form of an expression.
///
/// This is a two-phase pipeline:
/// 1. `canonical_form()` — exact symbolic simplification
/// 2. Asymptotic projection — drop dominated terms and constant factors
///
/// Returns an expression representing the asymptotic growth class.
pub fn big_o_normal_form(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    let canonical = canonical_form(expr).map_err(|e| match e {
        CanonicalizationError::Unsupported(s) => AsymptoticAnalysisError::Unsupported(s),
    })?;

    project_big_o(&canonical)
}

/// Project a canonicalized expression into its Big-O growth class.
fn project_big_o(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    // Decompose into additive terms
    let mut terms = Vec::new();
    collect_additive_terms(expr, &mut terms);

    // Project each term: drop constant multiplicative factors
    let mut projected: Vec<ProjectedTerm> = Vec::new();
    for term in &terms {
        if let Some(projected_term) = project_term(term)? {
            projected.push(projected_term);
        }
        // Pure constants are dropped (asymptotically irrelevant)
    }

    // Remove dominated terms
    let survivors = remove_dominated_terms(projected);

    if survivors.is_empty() {
        // All terms were constants → O(1)
        return Ok(Expr::Const(1.0));
    }

    if let Some(negative) = survivors.iter().find(|term| term.negative) {
        return Err(AsymptoticAnalysisError::Unsupported(format!(
            "-1 * {}",
            negative.expr
        )));
    }

    // Deduplicate
    let mut seen = std::collections::BTreeSet::new();
    let mut deduped = Vec::new();
    for term in survivors {
        let key = term.expr.to_string();
        if seen.insert(key) {
            deduped.push(term);
        }
    }

    // Rebuild sum
    let mut result = deduped[0].expr.clone();
    for term in &deduped[1..] {
        result = result + term.expr.clone();
    }

    Ok(result)
}

fn collect_additive_terms(expr: &Expr, out: &mut Vec<Expr>) {
    match expr {
        Expr::Add(a, b) => {
            collect_additive_terms(a, out);
            collect_additive_terms(b, out);
        }
        other => out.push(other.clone()),
    }
}

/// Project a single multiplicative term: strip constant factors.
/// Returns None if the term is a pure constant.
fn project_term(term: &Expr) -> Result<Option<ProjectedTerm>, AsymptoticAnalysisError> {
    if term.constant_value().is_some() {
        return Ok(None); // Pure constant → dropped
    }

    // Collect multiplicative factors
    let mut factors = Vec::new();
    collect_multiplicative_factors(term, &mut factors);

    let mut coeff = 1.0;
    let mut symbolic = Vec::new();
    for factor in &factors {
        if let Some(c) = factor.constant_value() {
            coeff *= c;
            continue;
        }
        if contains_negative_exponent(factor) {
            return Err(AsymptoticAnalysisError::Unsupported(term.to_string()));
        }
        symbolic.push(factor.clone());
    }

    if symbolic.is_empty() {
        return Ok(None);
    }

    let mut result = symbolic[0].clone();
    for f in &symbolic[1..] {
        result = result * f.clone();
    }

    Ok(Some(ProjectedTerm {
        expr: result,
        negative: coeff < 0.0,
    }))
}

fn collect_multiplicative_factors(expr: &Expr, out: &mut Vec<Expr>) {
    match expr {
        Expr::Mul(a, b) => {
            collect_multiplicative_factors(a, out);
            collect_multiplicative_factors(b, out);
        }
        other => out.push(other.clone()),
    }
}

/// Remove terms dominated by other terms using monomial comparison.
///
/// A term `t` is dominated if there exists another term `s` such that
/// `t` grows no faster than `s` asymptotically.
fn remove_dominated_terms(terms: Vec<ProjectedTerm>) -> Vec<ProjectedTerm> {
    if terms.len() <= 1 {
        return terms;
    }

    let mut survivors = Vec::new();
    for (i, term) in terms.iter().enumerate() {
        let is_dominated = terms
            .iter()
            .enumerate()
            .any(|(j, other)| i != j && term_dominated_by(&term.expr, &other.expr));
        if !is_dominated {
            survivors.push(term.clone());
        }
    }
    survivors
}

/// Check if `small` is asymptotically dominated by `big`.
///
/// Supports three comparison strategies:
/// 1. Polynomial monomial exponent comparison (exact)
/// 2. Exponential vs subexponential / base comparison (structural)
/// 3. Numerical evaluation at two scales (for subexponential cross-class)
fn term_dominated_by(small: &Expr, big: &Expr) -> bool {
    // Case 1: Both pure polynomial monomials — use exponent comparison
    let small_exps = extract_var_exponents(small);
    let big_exps = extract_var_exponents(big);
    if let (Some(ref se), Some(ref be)) = (small_exps, big_exps) {
        return polynomial_dominated(se, be);
    }

    // Cross-class comparison: small's variables must be a subset of big's
    let small_vars = small.variables();
    let big_vars = big.variables();
    if small_vars.is_empty() || big_vars.is_empty() || !small_vars.is_subset(&big_vars) {
        return false;
    }

    // Case 2: Exponential comparison
    let small_has_exp = has_exponential_growth(small);
    let big_has_exp = has_exponential_growth(big);
    match (small_has_exp, big_has_exp) {
        (false, true) => return true,  // exponential dominates subexponential
        (true, false) => return false, // subexponential can't dominate exponential
        (true, true) => {
            // Compare effective exponential bases
            if let (Some(sb), Some(bb)) = (effective_exp_base(small), effective_exp_base(big)) {
                if bb > sb * (1.0 + 1e-10) {
                    return true;
                }
            }
            return false;
        }
        (false, false) => {} // both subexponential, fall through
    }

    // Case 3: Both subexponential, same variables — numerical comparison
    // Handles: poly vs poly*log, log vs log(log), poly vs log, etc.
    if small_vars == big_vars {
        return numerical_dominance_check(small, big, &small_vars);
    }

    false
}

/// Check polynomial dominance: small ≤ big component-wise with at least one strict inequality.
fn polynomial_dominated(
    se: &std::collections::BTreeMap<&'static str, f64>,
    be: &std::collections::BTreeMap<&'static str, f64>,
) -> bool {
    let mut all_leq = true;
    let mut any_strictly_less = false;

    for (var, small_exp) in se {
        let big_exp = be.get(var).copied().unwrap_or(0.0);
        if *small_exp > big_exp + 1e-15 {
            all_leq = false;
            break;
        }
        if *small_exp < big_exp - 1e-15 {
            any_strictly_less = true;
        }
    }

    if all_leq {
        for (var, big_exp) in be {
            if !se.contains_key(var) && *big_exp > 1e-15 {
                any_strictly_less = true;
            }
        }
    }

    all_leq && any_strictly_less
}

/// Extract variable → exponent mapping from a monomial expression.
/// Returns None for non-polynomial terms (exp, log, etc.).
fn extract_var_exponents(expr: &Expr) -> Option<std::collections::BTreeMap<&'static str, f64>> {
    use std::collections::BTreeMap;
    let mut exps = BTreeMap::new();
    extract_var_exponents_inner(expr, &mut exps)?;
    Some(exps)
}

fn extract_var_exponents_inner(
    expr: &Expr,
    exps: &mut std::collections::BTreeMap<&'static str, f64>,
) -> Option<()> {
    match expr {
        Expr::Var(name) => {
            *exps.entry(name).or_insert(0.0) += 1.0;
            Some(())
        }
        Expr::Pow(base, exp) => {
            if let (Expr::Var(name), Some(e)) = (base.as_ref(), exp.constant_value()) {
                if e < 0.0 {
                    return None;
                }
                *exps.entry(name).or_insert(0.0) += e;
                Some(())
            } else {
                None // Non-simple power
            }
        }
        Expr::Mul(a, b) => {
            extract_var_exponents_inner(a, exps)?;
            extract_var_exponents_inner(b, exps)
        }
        Expr::Const(_) => Some(()), // Constants don't affect exponents
        _ => None,                  // exp, log, sqrt → not a polynomial monomial
    }
}

fn contains_negative_exponent(expr: &Expr) -> bool {
    match expr {
        Expr::Pow(_, exp) => exp.constant_value().is_some_and(|e| e < 0.0),
        Expr::Mul(a, b) | Expr::Add(a, b) => {
            contains_negative_exponent(a) || contains_negative_exponent(b)
        }
        Expr::Exp(arg) | Expr::Log(arg) | Expr::Sqrt(arg) | Expr::Factorial(arg) => {
            contains_negative_exponent(arg)
        }
        Expr::Const(_) | Expr::Var(_) => false,
    }
}

/// Check if an expression has exponential growth.
///
/// Returns true if the expression contains `exp(var_expr)` or `c^(var_expr)` where c > 1.
fn has_exponential_growth(expr: &Expr) -> bool {
    match expr {
        Expr::Exp(arg) => !arg.variables().is_empty(),
        Expr::Pow(base, exp) => {
            base.constant_value().is_some_and(|c| c > 1.0) && !exp.variables().is_empty()
        }
        Expr::Mul(a, b) => has_exponential_growth(a) || has_exponential_growth(b),
        _ => false,
    }
}

/// Compute the effective exponential base for growth rate comparison.
///
/// For `c^(f(n))`, approximates the effective base as `c^(f(1))`.
/// This works correctly for linear exponents (the common case in complexity expressions).
fn effective_exp_base(expr: &Expr) -> Option<f64> {
    match expr {
        Expr::Exp(arg) => {
            let vars = arg.variables();
            if vars.is_empty() {
                None
            } else {
                let size = unit_problem_size(&vars);
                let rate = arg.eval(&size);
                Some(std::f64::consts::E.powf(rate))
            }
        }
        Expr::Pow(base, exp) => {
            if let Some(c) = base.constant_value() {
                let vars = exp.variables();
                if c > 1.0 && !vars.is_empty() {
                    let size = unit_problem_size(&vars);
                    let exp_at_1 = exp.eval(&size);
                    Some(c.powf(exp_at_1))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Expr::Mul(a, b) => match (effective_exp_base(a), effective_exp_base(b)) {
            (Some(ba), Some(bb)) => Some(ba * bb),
            (Some(b), None) | (None, Some(b)) => Some(b),
            (None, None) => None,
        },
        _ => None,
    }
}

/// Create a `ProblemSize` with all variables set to the given value.
fn make_problem_size(
    vars: &std::collections::HashSet<&'static str>,
    val: usize,
) -> crate::types::ProblemSize {
    crate::types::ProblemSize::new(vars.iter().map(|&v| (v, val)).collect())
}

/// Create a `ProblemSize` with all variables set to 1.
fn unit_problem_size(vars: &std::collections::HashSet<&'static str>) -> crate::types::ProblemSize {
    make_problem_size(vars, 1)
}

/// Check dominance numerically by evaluating at two scales.
///
/// Returns true if `big/small` ratio is > 1 and increasing between the two
/// evaluation points, indicating `big` grows asymptotically faster.
fn numerical_dominance_check(
    small: &Expr,
    big: &Expr,
    vars: &std::collections::HashSet<&'static str>,
) -> bool {
    let size1 = make_problem_size(vars, 100);
    let size2 = make_problem_size(vars, 10_000);

    let s1 = small.eval(&size1);
    let b1 = big.eval(&size1);
    let s2 = small.eval(&size2);
    let b2 = big.eval(&size2);

    // Both must be finite and positive at both points
    if !s1.is_finite() || !b1.is_finite() || !s2.is_finite() || !b2.is_finite() {
        return false;
    }
    if s1 <= 1e-300 || b1 <= 1e-300 || s2 <= 1e-300 || b2 <= 1e-300 {
        return false;
    }

    let ratio1 = b1 / s1;
    let ratio2 = b2 / s2;

    // Dominance: ratio is > 1 at both points and strictly increasing
    ratio1 > 1.0 + 1e-10 && ratio2 > ratio1 * (1.0 + 1e-6)
}

#[cfg(test)]
#[path = "unit_tests/big_o.rs"]
mod tests;
