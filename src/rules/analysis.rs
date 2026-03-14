//! Analysis utilities for the reduction graph.
//!
//! Detects primitive reduction rules that are dominated by composite paths,
//! using asymptotic normalization plus monomial-dominance comparison.
//!
//! This analysis is **sound but incomplete**: it reports `Dominated` only when
//! the symbolic comparison is trustworthy, and `Unknown` when metadata is too
//! weak to compare safely.

use crate::canonical::canonical_form;
use crate::expr::Expr;
use crate::rules::graph::{ReductionGraph, ReductionPath};
use crate::rules::registry::ReductionOverhead;
use std::collections::BTreeMap;
use std::fmt;

/// Result of comparing one primitive rule against one composite path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonStatus {
    /// Composite is equal or better on all common fields.
    Dominated,
    /// Composite is worse on at least one common field.
    NotDominated,
    /// Cannot decide: expression not normalizable or path not trustworthy.
    Unknown,
}

/// A primitive reduction rule proven dominated by a composite path.
#[derive(Debug, Clone)]
pub struct DominatedRule {
    pub source_name: &'static str,
    pub source_variant: BTreeMap<String, String>,
    pub target_name: &'static str,
    pub target_variant: BTreeMap<String, String>,
    pub primitive_overhead: ReductionOverhead,
    pub dominating_path: ReductionPath,
    pub composed_overhead: ReductionOverhead,
    pub comparable_fields: Vec<String>,
}

impl DominatedRule {
    pub fn source_display(&self) -> String {
        format_problem_variant(self.source_name, &self.source_variant)
    }

    pub fn target_display(&self) -> String {
        format_problem_variant(self.target_name, &self.target_variant)
    }
}

impl fmt::Display for DominatedRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source_display(), self.target_display())
    }
}

/// A candidate comparison that could not be decided soundly.
#[derive(Debug, Clone)]
pub struct UnknownComparison {
    pub source_name: &'static str,
    pub source_variant: BTreeMap<String, String>,
    pub target_name: &'static str,
    pub target_variant: BTreeMap<String, String>,
    pub candidate_path: ReductionPath,
    pub reason: String,
}

impl UnknownComparison {
    pub fn source_display(&self) -> String {
        format_problem_variant(self.source_name, &self.source_variant)
    }

    pub fn target_display(&self) -> String {
        format_problem_variant(self.target_name, &self.target_variant)
    }
}

impl fmt::Display for UnknownComparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source_display(), self.target_display())
    }
}

pub fn format_problem_variant(name: &str, variant: &BTreeMap<String, String>) -> String {
    if variant.is_empty() {
        return name.to_string();
    }

    let vars = variant
        .iter()
        .map(|(k, v)| format!("{k}: {v:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{name} {{{vars}}}")
}

// ────────── Polynomial normalization ──────────

/// A monomial: coefficient × ∏(variable ^ exponent).
#[derive(Debug, Clone)]
struct Monomial {
    coeff: f64,
    /// Variable name → exponent. Only non-zero exponents stored.
    vars: BTreeMap<&'static str, f64>,
}

impl Monomial {
    fn constant(c: f64) -> Self {
        Self {
            coeff: c,
            vars: BTreeMap::new(),
        }
    }

    fn variable(name: &'static str) -> Self {
        let mut vars = BTreeMap::new();
        vars.insert(name, 1.0);
        Self { coeff: 1.0, vars }
    }

    /// Multiply two monomials.
    fn mul(&self, other: &Monomial) -> Monomial {
        let coeff = self.coeff * other.coeff;
        let mut vars = self.vars.clone();
        for (&v, &e) in &other.vars {
            *vars.entry(v).or_insert(0.0) += e;
        }
        Monomial { coeff, vars }
    }
}

/// A polynomial (sum of monomials) in normal form.
#[derive(Debug, Clone)]
struct NormalizedPoly {
    terms: Vec<Monomial>,
}

impl NormalizedPoly {
    fn add(mut self, other: NormalizedPoly) -> NormalizedPoly {
        self.terms.extend(other.terms);
        self
    }

    fn mul(&self, other: &NormalizedPoly) -> NormalizedPoly {
        let mut terms = Vec::new();
        for a in &self.terms {
            for b in &other.terms {
                terms.push(a.mul(b));
            }
        }
        NormalizedPoly { terms }
    }

    /// True if any monomial has a negative coefficient.
    fn has_negative_coefficients(&self) -> bool {
        self.terms.iter().any(|m| m.coeff < -1e-15)
    }
}

/// Normalize an expression into a sum of monomials.
///
/// Supports: constants, variables, addition, multiplication,
/// and powers with non-negative constant exponents.
/// Returns `Err` for exp, log, sqrt, division, and negative exponents.
fn normalize_polynomial(expr: &Expr) -> Result<NormalizedPoly, String> {
    match expr {
        Expr::Const(c) => Ok(NormalizedPoly {
            terms: vec![Monomial::constant(*c)],
        }),
        Expr::Var(v) => Ok(NormalizedPoly {
            terms: vec![Monomial::variable(v)],
        }),
        Expr::Add(a, b) => {
            let pa = normalize_polynomial(a)?;
            let pb = normalize_polynomial(b)?;
            Ok(pa.add(pb))
        }
        Expr::Mul(a, b) => {
            let pa = normalize_polynomial(a)?;
            let pb = normalize_polynomial(b)?;
            Ok(pa.mul(&pb))
        }
        Expr::Pow(base, exp) => {
            if let Expr::Const(c) = exp.as_ref() {
                if *c < 0.0 {
                    return Err(format!("negative exponent: {c}"));
                }
                let pb = normalize_polynomial(base)?;
                // Single monomial: multiply exponents
                if pb.terms.len() == 1 {
                    let m = &pb.terms[0];
                    let coeff = m.coeff.powf(*c);
                    let vars: BTreeMap<_, _> = m.vars.iter().map(|(&v, &e)| (v, e * c)).collect();
                    return Ok(NormalizedPoly {
                        terms: vec![Monomial { coeff, vars }],
                    });
                }
                // Multi-term polynomial raised to non-negative integer power
                let n = *c as usize;
                if c.fract().abs() < 1e-10 {
                    if n == 0 {
                        return Ok(NormalizedPoly {
                            terms: vec![Monomial::constant(1.0)],
                        });
                    }
                    let mut result = pb.clone();
                    for _ in 1..n {
                        result = result.mul(&pb);
                    }
                    return Ok(result);
                }
                Err(format!(
                    "non-integer power of multi-term polynomial: ({base})^{c}"
                ))
            } else {
                Err(format!("variable exponent: ({base})^({exp})"))
            }
        }
        Expr::Exp(_) => Err("exp() not supported".into()),
        Expr::Log(_) => Err("log() not supported".into()),
        Expr::Sqrt(_) => Err("sqrt() not supported".into()),
        Expr::Factorial(_) => Err("factorial() not supported".into()),
    }
}

fn prepare_expr_for_comparison(expr: &Expr) -> Expr {
    canonical_form(expr).unwrap_or_else(|_| expr.clone())
}

// ────────── Monomial-dominance comparison ──────────

/// Check if monomial `small` is asymptotically dominated by monomial `big`.
///
/// True iff for every variable in `small`, `big` has at least as large an exponent.
/// This means `small` grows no faster than `big` as all variables → ∞.
fn monomial_dominated_by(small: &Monomial, big: &Monomial) -> bool {
    for (&var, &exp_small) in &small.vars {
        let exp_big = big.vars.get(var).copied().unwrap_or(0.0);
        if exp_small > exp_big + 1e-10 {
            return false;
        }
    }
    true
}

/// Check if polynomial `a` is asymptotically ≤ polynomial `b`.
///
/// True iff every positive-coefficient monomial in `a` is dominated by
/// some positive-coefficient monomial in `b`.
fn poly_leq(a: &NormalizedPoly, b: &NormalizedPoly) -> bool {
    let b_positive: Vec<&Monomial> = b.terms.iter().filter(|m| m.coeff > 1e-15).collect();

    for a_term in &a.terms {
        if a_term.coeff <= 1e-15 {
            continue; // zero or negative — can only make `a` smaller
        }
        let dominated = b_positive
            .iter()
            .any(|b_term| monomial_dominated_by(a_term, b_term));
        if !dominated {
            return false;
        }
    }
    true
}

// ────────── Overhead comparison ──────────

/// Compare two overheads across all common fields.
///
/// Returns `Dominated` if composite ≤ primitive on all common fields.
/// Returns `NotDominated` if composite is worse on any common field.
/// Returns `Unknown` if any common field's expressions cannot be normalized
/// into a comparable polynomial form or contain negative coefficients.
pub fn compare_overhead(
    primitive: &ReductionOverhead,
    composite: &ReductionOverhead,
) -> ComparisonStatus {
    let comp_map: std::collections::HashMap<&str, &Expr> = composite
        .output_size
        .iter()
        .map(|(name, expr)| (*name, expr))
        .collect();

    let mut any_common = false;

    for (field, prim_expr) in &primitive.output_size {
        let Some(comp_expr) = comp_map.get(field) else {
            continue;
        };
        any_common = true;

        let primitive_prepared = prepare_expr_for_comparison(prim_expr);
        let composite_prepared = prepare_expr_for_comparison(comp_expr);

        if primitive_prepared == composite_prepared {
            continue;
        }

        let primitive_poly = match normalize_polynomial(&primitive_prepared) {
            Ok(p) => p,
            Err(_) => return ComparisonStatus::Unknown,
        };
        let composite_poly = match normalize_polynomial(&composite_prepared) {
            Ok(p) => p,
            Err(_) => return ComparisonStatus::Unknown,
        };

        // Reject expressions with negative coefficients
        if primitive_poly.has_negative_coefficients() || composite_poly.has_negative_coefficients()
        {
            return ComparisonStatus::Unknown;
        }

        // Check: composite ≤ primitive on this field
        if !poly_leq(&composite_poly, &primitive_poly) {
            return ComparisonStatus::NotDominated;
        }
    }

    if any_common {
        ComparisonStatus::Dominated
    } else {
        ComparisonStatus::NotDominated
    }
}

// ────────── Main analysis ──────────

/// Find all primitive reduction rules dominated by composite paths.
///
/// Returns a tuple of:
/// - `Vec<DominatedRule>`: rules proven dominated by a composite path
/// - `Vec<UnknownComparison>`: candidates that could not be decided
///
/// For each primitive rule (direct edge), enumerates all alternative paths,
/// validates trustworthiness, composes overheads, and compares.
/// Keeps only the best (shortest) dominating path per primitive rule.
///
/// Note: iterates the graph's coalesced edges rather than raw `inventory` entries.
/// This is sound because `test_no_duplicate_primitive_rules_per_variant_pair` guards
/// the invariant that at most one registration exists per (source_variant, target_variant) pair.
pub fn find_dominated_rules(
    graph: &ReductionGraph,
) -> (Vec<DominatedRule>, Vec<UnknownComparison>) {
    let mut dominated = Vec::new();
    let mut unknown = Vec::new();

    for edge_info in all_edges(graph) {
        let paths = graph.find_all_paths(
            edge_info.source_name,
            &edge_info.source_variant,
            edge_info.target_name,
            &edge_info.target_variant,
        );

        let mut best_dominating: Option<(ReductionPath, ReductionOverhead, Vec<String>)> = None;

        for path in paths {
            if path.len() <= 1 {
                continue; // skip the direct edge itself
            }

            let composed = graph.compose_path_overhead(&path);

            match compare_overhead(&edge_info.overhead, &composed) {
                ComparisonStatus::Dominated => {
                    let comparable_fields = common_fields(&edge_info.overhead, &composed);
                    let is_better = match &best_dominating {
                        None => true,
                        Some((best_path, _, _)) => path.len() < best_path.len(),
                    };
                    if is_better {
                        best_dominating = Some((path, composed, comparable_fields));
                    }
                }
                ComparisonStatus::Unknown => {
                    unknown.push(UnknownComparison {
                        source_name: edge_info.source_name,
                        source_variant: edge_info.source_variant.clone(),
                        target_name: edge_info.target_name,
                        target_variant: edge_info.target_variant.clone(),
                        candidate_path: path,
                        reason: "expression comparison returned Unknown".into(),
                    });
                }
                ComparisonStatus::NotDominated => {}
            }
        }

        if let Some((path, composed, fields)) = best_dominating {
            dominated.push(DominatedRule {
                source_name: edge_info.source_name,
                source_variant: edge_info.source_variant.clone(),
                target_name: edge_info.target_name,
                target_variant: edge_info.target_variant.clone(),
                primitive_overhead: edge_info.overhead.clone(),
                dominating_path: path,
                composed_overhead: composed,
                comparable_fields: fields,
            });
        }
    }

    // Deterministic output
    dominated.sort_by(|a, b| {
        (
            format_problem_variant(a.source_name, &a.source_variant),
            format_problem_variant(a.target_name, &a.target_variant),
            a.dominating_path.len(),
        )
            .cmp(&(
                format_problem_variant(b.source_name, &b.source_variant),
                format_problem_variant(b.target_name, &b.target_variant),
                b.dominating_path.len(),
            ))
    });
    unknown.sort_by(|a, b| {
        (
            format_problem_variant(a.source_name, &a.source_variant),
            format_problem_variant(a.target_name, &a.target_variant),
        )
            .cmp(&(
                format_problem_variant(b.source_name, &b.source_variant),
                format_problem_variant(b.target_name, &b.target_variant),
            ))
    });

    (dominated, unknown)
}

/// Fields present in both overheads.
fn common_fields(a: &ReductionOverhead, b: &ReductionOverhead) -> Vec<String> {
    let b_fields: std::collections::HashSet<&str> = b.output_size.iter().map(|(n, _)| *n).collect();
    a.output_size
        .iter()
        .filter(|&(f, _)| b_fields.contains(f))
        .map(|(f, _)| f.to_string())
        .collect()
}

/// Collect all edges from the reduction graph.
fn all_edges(graph: &ReductionGraph) -> Vec<crate::rules::graph::ReductionEdgeInfo> {
    let mut edges = Vec::new();
    for name in graph.problem_types() {
        edges.extend(graph.outgoing_reductions(name));
    }
    edges
}

#[cfg(test)]
#[path = "../unit_tests/rules/analysis.rs"]
mod tests;
