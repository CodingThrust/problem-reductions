//! Big-O asymptotic projection for canonical expressions.
//!
//! Takes the output of `canonical_form()` and projects it into an
//! asymptotic growth class by dropping dominated terms and constant factors.

use crate::canonical::canonical_form;
use crate::expr::{AsymptoticAnalysisError, CanonicalizationError, Expr};

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
    let mut projected: Vec<Expr> = Vec::new();
    for term in &terms {
        if let Some(projected_term) = project_term(term) {
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

    // Deduplicate
    let mut seen = std::collections::BTreeSet::new();
    let mut deduped = Vec::new();
    for term in survivors {
        let key = term.to_string();
        if seen.insert(key) {
            deduped.push(term);
        }
    }

    // Rebuild sum
    let mut result = deduped[0].clone();
    for term in &deduped[1..] {
        result = result + term.clone();
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
fn project_term(term: &Expr) -> Option<Expr> {
    if term.constant_value().is_some() {
        return None; // Pure constant → dropped
    }

    // Collect multiplicative factors
    let mut factors = Vec::new();
    collect_multiplicative_factors(term, &mut factors);

    // Remove constant factors, keep symbolic ones
    let symbolic: Vec<&Expr> = factors
        .iter()
        .filter(|f| f.constant_value().is_none())
        .collect();

    if symbolic.is_empty() {
        return None;
    }

    let mut result = symbolic[0].clone();
    for f in &symbolic[1..] {
        result = result * (*f).clone();
    }
    Some(result)
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
fn remove_dominated_terms(terms: Vec<Expr>) -> Vec<Expr> {
    if terms.len() <= 1 {
        return terms;
    }

    let mut survivors = Vec::new();
    for (i, term) in terms.iter().enumerate() {
        let is_dominated = terms
            .iter()
            .enumerate()
            .any(|(j, other)| i != j && term_dominated_by(term, other));
        if !is_dominated {
            survivors.push(term.clone());
        }
    }
    survivors
}

/// Check if `small` is asymptotically dominated by `big`.
///
/// Conservative: only returns true when dominance is provable
/// via monomial exponent comparison.
fn term_dominated_by(small: &Expr, big: &Expr) -> bool {
    // Extract monomial exponents for comparison
    let small_exps = extract_var_exponents(small);
    let big_exps = extract_var_exponents(big);

    // Both must be pure polynomial monomials for comparison
    let (Some(se), Some(be)) = (small_exps, big_exps) else {
        return false; // Can't compare non-polynomial terms
    };

    // small ≤ big if: for every variable in small, big has ≥ exponent
    // AND big has at least one strictly greater exponent or has a variable small doesn't
    let mut all_leq = true;
    let mut any_strictly_less = false;

    for (var, small_exp) in &se {
        let big_exp = be.get(var).copied().unwrap_or(0.0);
        if *small_exp > big_exp + 1e-15 {
            all_leq = false;
            break;
        }
        if *small_exp < big_exp - 1e-15 {
            any_strictly_less = true;
        }
    }

    // Also check variables in big not in small (those have implicit exponent 0 in small)
    if all_leq {
        for (var, big_exp) in &be {
            if !se.contains_key(var) && *big_exp > 1e-15 {
                any_strictly_less = true;
            }
        }
    }

    // Dominated if all exponents ≤ AND at least one is strictly less.
    // Equal terms are NOT dominated — they get deduped in a separate step.
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

#[cfg(test)]
#[path = "unit_tests/big_o.rs"]
mod tests;
