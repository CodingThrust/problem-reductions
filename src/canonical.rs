//! Exact symbolic canonicalization for `Expr`.
//!
//! Normalizes expressions into a canonical sum-of-terms form with signed
//! coefficients and deterministic ordering, without losing algebraic precision.

use std::collections::BTreeMap;

use crate::expr::{CanonicalizationError, Expr};

/// An opaque non-polynomial factor (exp, log, fractional-power base).
///
/// Stored by its canonical string representation for deterministic ordering.
#[derive(Clone, Debug, PartialEq)]
struct OpaqueFactor {
    /// The canonical string form (used for equality and ordering).
    key: String,
    /// The original `Expr` for reconstruction.
    expr: Expr,
}

impl Eq for OpaqueFactor {}

impl PartialOrd for OpaqueFactor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpaqueFactor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

fn normalized_f64_bits(value: f64) -> u64 {
    if value == 0.0 {
        0.0f64.to_bits()
    } else {
        value.to_bits()
    }
}

/// A single additive term: coefficient × product of canonical factors.
#[derive(Clone, Debug)]
struct CanonicalTerm {
    /// Signed numeric coefficient.
    coeff: f64,
    /// Polynomial variable exponents (variable_name → exponent).
    vars: BTreeMap<&'static str, f64>,
    /// Non-polynomial opaque factors, sorted by key.
    opaque: Vec<OpaqueFactor>,
}

/// Try to merge a new opaque factor into an existing list using transcendental identities.
/// Returns `Some(updated_list)` if a merge happened, `None` if no identity applies.
fn try_merge_opaque(existing: &[OpaqueFactor], new: &OpaqueFactor) -> Option<Vec<OpaqueFactor>> {
    for (i, existing_factor) in existing.iter().enumerate() {
        // exp(a) * exp(b) -> exp(a + b)
        if let (Expr::Exp(a), Expr::Exp(b)) = (&existing_factor.expr, &new.expr) {
            let merged_arg = (**a).clone() + (**b).clone();
            let merged_expr =
                Expr::Exp(Box::new(canonical_form(&merged_arg).unwrap_or(merged_arg)));
            let mut result = existing.to_vec();
            result[i] = OpaqueFactor {
                key: merged_expr.to_string(),
                expr: merged_expr,
            };
            return Some(result);
        }

        // c^a * c^b -> c^(a+b) for matching positive constant base c
        if let (Expr::Pow(base1, exp1), Expr::Pow(base2, exp2)) = (&existing_factor.expr, &new.expr)
        {
            if let (Some(c1), Some(c2)) = (base1.constant_value(), base2.constant_value()) {
                if c1 > 0.0 && c2 > 0.0 && (c1 - c2).abs() < 1e-15 {
                    let merged_exp = (**exp1).clone() + (**exp2).clone();
                    let canon_exp = canonical_form(&merged_exp).unwrap_or(merged_exp);
                    let merged_expr = Expr::Pow(base1.clone(), Box::new(canon_exp));
                    let mut result = existing.to_vec();
                    result[i] = OpaqueFactor {
                        key: merged_expr.to_string(),
                        expr: merged_expr,
                    };
                    return Some(result);
                }
            }
        }
    }
    None
}

/// A canonical sum of terms: the exact normal form of an expression.
#[derive(Clone, Debug)]
pub(crate) struct CanonicalSum {
    terms: Vec<CanonicalTerm>,
}

impl CanonicalTerm {
    fn constant(c: f64) -> Self {
        Self {
            coeff: c,
            vars: BTreeMap::new(),
            opaque: Vec::new(),
        }
    }

    fn variable(name: &'static str) -> Self {
        let mut vars = BTreeMap::new();
        vars.insert(name, 1.0);
        Self {
            coeff: 1.0,
            vars,
            opaque: Vec::new(),
        }
    }

    fn opaque_factor(expr: Expr) -> Self {
        let key = expr.to_string();
        Self {
            coeff: 1.0,
            vars: BTreeMap::new(),
            opaque: vec![OpaqueFactor { key, expr }],
        }
    }

    /// Multiply two terms, applying transcendental identities:
    /// - `exp(a) * exp(b) -> exp(a + b)`
    /// - `c^a * c^b -> c^(a + b)` for matching constant base `c`
    fn mul(&self, other: &CanonicalTerm) -> CanonicalTerm {
        let coeff = self.coeff * other.coeff;
        let mut vars = self.vars.clone();
        for (&v, &e) in &other.vars {
            *vars.entry(v).or_insert(0.0) += e;
        }
        // Remove zero-exponent variables
        vars.retain(|_, e| e.abs() > 1e-15);

        // Merge opaque factors with transcendental identities
        let mut opaque = self.opaque.clone();
        for other_factor in &other.opaque {
            if let Some(merged) = try_merge_opaque(&opaque, other_factor) {
                opaque = merged;
            } else {
                opaque.push(other_factor.clone());
            }
        }
        opaque.sort();
        CanonicalTerm {
            coeff,
            vars,
            opaque,
        }
    }

    /// Deterministic sort key for ordering terms in a sum.
    fn sort_key(&self) -> (Vec<(&'static str, u64)>, Vec<String>) {
        let vars: Vec<_> = self
            .vars
            .iter()
            .map(|(&k, &v)| (k, normalized_f64_bits(v)))
            .collect();
        let opaque: Vec<_> = self.opaque.iter().map(|o| o.key.clone()).collect();
        (vars, opaque)
    }
}

impl CanonicalSum {
    fn from_term(term: CanonicalTerm) -> Self {
        Self { terms: vec![term] }
    }

    fn add(mut self, other: CanonicalSum) -> Self {
        self.terms.extend(other.terms);
        self
    }

    fn mul(&self, other: &CanonicalSum) -> CanonicalSum {
        let mut terms = Vec::new();
        for a in &self.terms {
            for b in &other.terms {
                terms.push(a.mul(b));
            }
        }
        CanonicalSum { terms }
    }

    /// Merge terms with the same signature and drop zero-coefficient terms.
    /// Sort the result deterministically.
    fn simplify(self) -> Self {
        type SortKey = (Vec<(&'static str, u64)>, Vec<String>);
        let mut groups: BTreeMap<SortKey, CanonicalTerm> = BTreeMap::new();

        for term in self.terms {
            let key = term.sort_key();
            groups
                .entry(key)
                .and_modify(|existing| existing.coeff += term.coeff)
                .or_insert(term);
        }

        let mut terms: Vec<_> = groups
            .into_values()
            .filter(|t| t.coeff.abs() > 1e-15)
            .collect();

        terms.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));

        CanonicalSum { terms }
    }
}

/// Normalize an expression into its exact canonical sum-of-terms form.
///
/// This performs exact symbolic simplification:
/// - Flattens nested Add/Mul
/// - Merges duplicate additive terms by summing coefficients
/// - Merges repeated multiplicative factors into powers
/// - Preserves signed coefficients (supports subtraction)
/// - Preserves transcendental identities: exp(a)*exp(b)=exp(a+b), etc.
/// - Produces deterministic ordering
///
/// Does NOT drop terms or constant factors — use `big_o_normal_form()` for that.
pub fn canonical_form(expr: &Expr) -> Result<Expr, CanonicalizationError> {
    let sum = expr_to_canonical(expr)?;
    let simplified = sum.simplify();
    Ok(canonical_sum_to_expr(&simplified))
}

fn expr_to_canonical(expr: &Expr) -> Result<CanonicalSum, CanonicalizationError> {
    match expr {
        Expr::Const(c) => Ok(CanonicalSum::from_term(CanonicalTerm::constant(*c))),
        Expr::Var(name) => Ok(CanonicalSum::from_term(CanonicalTerm::variable(name))),
        Expr::Add(a, b) => {
            let ca = expr_to_canonical(a)?;
            let cb = expr_to_canonical(b)?;
            Ok(ca.add(cb))
        }
        Expr::Mul(a, b) => {
            let ca = expr_to_canonical(a)?;
            let cb = expr_to_canonical(b)?;
            Ok(ca.mul(&cb))
        }
        Expr::Pow(base, exp) => canonicalize_pow(base, exp),
        Expr::Exp(arg) => {
            // Treat exp(canonicalized_arg) as an opaque factor
            let inner = canonical_form(arg)?;
            Ok(CanonicalSum::from_term(CanonicalTerm::opaque_factor(
                Expr::Exp(Box::new(inner)),
            )))
        }
        Expr::Log(arg) => {
            let inner = canonical_form(arg)?;
            Ok(CanonicalSum::from_term(CanonicalTerm::opaque_factor(
                Expr::Log(Box::new(inner)),
            )))
        }
        Expr::Sqrt(arg) => {
            // sqrt(x) = x^0.5 — canonicalize as power
            canonicalize_pow(arg, &Expr::Const(0.5))
        }
    }
}

fn canonicalize_pow(base: &Expr, exp: &Expr) -> Result<CanonicalSum, CanonicalizationError> {
    match (base, exp) {
        // Constant base, constant exp → numeric constant
        (_, _) if base.constant_value().is_some() && exp.constant_value().is_some() => {
            let b = base.constant_value().unwrap();
            let e = exp.constant_value().unwrap();
            Ok(CanonicalSum::from_term(CanonicalTerm::constant(b.powf(e))))
        }
        // Variable ^ constant exponent → vars map (supports fractional/negative exponents)
        (Expr::Var(name), _) if exp.constant_value().is_some() => {
            let e = exp.constant_value().unwrap();
            if e.abs() < 1e-15 {
                return Ok(CanonicalSum::from_term(CanonicalTerm::constant(1.0)));
            }
            let mut vars = BTreeMap::new();
            vars.insert(*name, e);
            Ok(CanonicalSum::from_term(CanonicalTerm {
                coeff: 1.0,
                vars,
                opaque: Vec::new(),
            }))
        }
        // Polynomial base ^ constant integer exponent → expand
        (_, _) if exp.constant_value().is_some() => {
            let e = exp.constant_value().unwrap();
            if e >= 0.0 && (e - e.round()).abs() < 1e-10 {
                let n = e.round() as usize;
                let base_sum = expr_to_canonical(base)?;
                if n == 0 {
                    return Ok(CanonicalSum::from_term(CanonicalTerm::constant(1.0)));
                }
                let mut result = base_sum.clone();
                for _ in 1..n {
                    result = result.mul(&base_sum);
                }
                Ok(result)
            } else {
                // Fractional exponent with non-variable base → opaque
                let canon_base = canonical_form(base)?;
                Ok(CanonicalSum::from_term(CanonicalTerm::opaque_factor(
                    Expr::Pow(Box::new(canon_base), Box::new(Expr::Const(e))),
                )))
            }
        }
        // Constant base ^ variable exponent → opaque (exponential growth)
        (_, _) if base.constant_value().is_some() => {
            let c = base.constant_value().unwrap();
            if (c - 1.0).abs() < 1e-15 {
                return Ok(CanonicalSum::from_term(CanonicalTerm::constant(1.0)));
            }
            if c <= 0.0 {
                return Err(CanonicalizationError::Unsupported(format!(
                    "{}^{}",
                    base, exp
                )));
            }
            let canon_exp = canonical_form(exp)?;
            Ok(CanonicalSum::from_term(CanonicalTerm::opaque_factor(
                Expr::Pow(Box::new(base.clone()), Box::new(canon_exp)),
            )))
        }
        // Variable base ^ variable exponent → unsupported
        _ => Err(CanonicalizationError::Unsupported(format!(
            "{}^{}",
            base, exp
        ))),
    }
}

fn canonical_sum_to_expr(sum: &CanonicalSum) -> Expr {
    if sum.terms.is_empty() {
        return Expr::Const(0.0);
    }

    let term_exprs: Vec<Expr> = sum.terms.iter().map(canonical_term_to_expr).collect();

    let mut result = term_exprs[0].clone();
    for term in &term_exprs[1..] {
        result = result + term.clone();
    }
    result
}

fn canonical_term_to_expr(term: &CanonicalTerm) -> Expr {
    let mut factors: Vec<Expr> = Vec::new();

    // Add coefficient if not 1.0 (or -1.0, handled specially)
    let (coeff_factor, sign) = if term.coeff < 0.0 {
        (term.coeff.abs(), true)
    } else {
        (term.coeff, false)
    };

    let has_other_factors = !term.vars.is_empty() || !term.opaque.is_empty();

    if (coeff_factor - 1.0).abs() > 1e-15 || !has_other_factors {
        factors.push(Expr::Const(coeff_factor));
    }

    // Add variable powers
    for (&var, &exp) in &term.vars {
        if (exp - 1.0).abs() < 1e-15 {
            factors.push(Expr::Var(var));
        } else {
            factors.push(Expr::pow(Expr::Var(var), Expr::Const(exp)));
        }
    }

    // Add opaque factors
    for opaque in &term.opaque {
        factors.push(opaque.expr.clone());
    }

    let mut result = if factors.is_empty() {
        Expr::Const(1.0)
    } else {
        let mut r = factors[0].clone();
        for f in &factors[1..] {
            r = r * f.clone();
        }
        r
    };

    if sign {
        result = -result;
    }

    result
}

#[cfg(test)]
#[path = "unit_tests/canonical.rs"]
mod tests;
