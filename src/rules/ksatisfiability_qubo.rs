//! Reduction from KSatisfiability to QUBO (Max-K-SAT).
//!
//! For K=2 (quadratic penalty), each clause contributes to Q based on literal signs:
//! - (x_i ∨ x_j): penalty (1-x_i)(1-x_j) → Q[i][i]-=1, Q[j][j]-=1, Q[i][j]+=1, const+=1
//! - (¬x_i ∨ x_j): penalty x_i(1-x_j) → Q[i][i]+=1, Q[i][j]-=1
//! - (x_i ∨ ¬x_j): penalty (1-x_i)x_j → Q[j][j]+=1, Q[i][j]-=1
//! - (¬x_i ∨ ¬x_j): penalty x_i·x_j → Q[i][j]+=1
//!
//! For K≥3, we use the Rosenberg quadratization to reduce degree-K penalty terms
//! to quadratic form by introducing auxiliary variables. Each clause of K literals
//! requires K−2 auxiliary variables.
//!
//! CNFClause uses 1-indexed signed integers: positive = variable, negative = negated.

use crate::models::optimization::QUBO;
use crate::models::satisfiability::KSatisfiability;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
/// Result of reducing KSatisfiability to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionKSatToQUBO {
    target: QUBO<f64>,
    source_num_vars: usize,
}

impl ReductionResult for ReductionKSatToQUBO {
    type Source = KSatisfiability<2>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.source_num_vars].to_vec()
    }
}

/// Result of reducing KSatisfiability<3> to QUBO.
#[derive(Debug, Clone)]
pub struct Reduction3SATToQUBO {
    target: QUBO<f64>,
    source_num_vars: usize,
}

impl ReductionResult for Reduction3SATToQUBO {
    type Source = KSatisfiability<3>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.source_num_vars].to_vec()
    }
}

/// Convert a signed literal to (0-indexed variable, is_negated).
fn lit_to_var(lit: i32) -> (usize, bool) {
    let var = (lit.unsigned_abs() as usize) - 1;
    let neg = lit < 0;
    (var, neg)
}

/// Add the quadratic penalty term for a 2-literal clause to the QUBO matrix.
///
/// For clause (l_i ∨ l_j), the penalty for the clause being unsatisfied is
/// the product of the complemented literals.
fn add_2sat_clause_penalty(matrix: &mut [Vec<f64>], lits: &[i32]) {
    assert_eq!(lits.len(), 2, "Expected 2-literal clause");

    let (var_i, neg_i) = lit_to_var(lits[0]);
    let (var_j, neg_j) = lit_to_var(lits[1]);

    // Ensure i <= j for upper-triangular form
    let (i, j, ni, nj) = if var_i <= var_j {
        (var_i, var_j, neg_i, neg_j)
    } else {
        (var_j, var_i, neg_j, neg_i)
    };

    match (ni, nj) {
        (false, false) => {
            // (x_i ∨ x_j): penalty = (1-x_i)(1-x_j) = 1 - x_i - x_j + x_i·x_j
            matrix[i][i] -= 1.0;
            matrix[j][j] -= 1.0;
            matrix[i][j] += 1.0;
        }
        (true, false) => {
            // (¬x_i ∨ x_j): penalty = x_i(1-x_j) = x_i - x_i·x_j
            matrix[i][i] += 1.0;
            matrix[i][j] -= 1.0;
        }
        (false, true) => {
            // (x_i ∨ ¬x_j): penalty = (1-x_i)x_j = x_j - x_i·x_j
            matrix[j][j] += 1.0;
            matrix[i][j] -= 1.0;
        }
        (true, true) => {
            // (¬x_i ∨ ¬x_j): penalty = x_i·x_j
            matrix[i][j] += 1.0;
        }
    }
}

/// Add the QUBO terms for a 3-literal clause using Rosenberg quadratization.
///
/// For clause (l1 ∨ l2 ∨ l3), the penalty for not satisfying the clause is:
///   P = (1-l1)(1-l2)(1-l3) = y1·y2·y3
/// where yi = 1 - li (complement of literal).
///
/// We introduce one auxiliary variable `a` and quadratize using the substitution
/// a = y1·y2, adding penalty M·(y1·y2 - 2·y1·a - 2·y2·a + 3·a) where M is a
/// sufficiently large penalty (M = 2 suffices for Max-SAT).
///
/// The resulting quadratic form is:
///   H = a·y3 + M·(y1·y2 - 2·y1·a - 2·y2·a + 3·a)
///
/// `aux_var` is the 0-indexed auxiliary variable.
fn add_3sat_clause_penalty(matrix: &mut [Vec<f64>], lits: &[i32], aux_var: usize) {
    assert_eq!(lits.len(), 3, "Expected 3-literal clause");
    let penalty = 2.0; // Rosenberg penalty weight

    let (v1, n1) = lit_to_var(lits[0]);
    let (v2, n2) = lit_to_var(lits[1]);
    let (v3, n3) = lit_to_var(lits[2]);
    let a = aux_var;

    // We need to express yi = (1 - li) in terms of xi:
    //   If literal is positive (li = xi): yi = 1 - xi
    //   If literal is negated (li = 1 - xi): yi = xi
    //
    // So yi = xi if negated, yi = 1 - xi if positive.
    //
    // We compute the QUBO terms for:
    //   H = a·y3 + M·(y1·y2 - 2·y1·a - 2·y2·a + 3·a)
    //
    // Each term is expanded using yi = xi (if negated) or yi = 1-xi (if positive).

    // Helper: add coefficient * yi * yj to the matrix
    // where yi depends on variable vi and negation ni
    let add_yy = |matrix: &mut [Vec<f64>], vi: usize, ni: bool, vj: usize, nj: bool, coeff: f64| {
        // yi = xi if ni (negated literal), yi = 1 - xi if !ni (positive literal)
        // yi * yj expansion:
        if vi == vj {
            // Same variable: yi * yj
            // Both complemented the same way means yi = yj, so yi*yj = yi (binary)
            // If ni == nj: yi*yj = yi^2 = yi (binary), add coeff * yi
            // If ni != nj: yi * yj = xi * (1-xi) = 0 (always), add nothing
            if ni == nj {
                // yi * yi = yi (binary)
                if ni {
                    // yi = xi, add coeff * xi
                    matrix[vi][vi] += coeff;
                } else {
                    // yi = 1 - xi, add coeff * (1 - xi) = coeff - coeff * xi
                    // constant term ignored in QUBO (offset), diagonal:
                    matrix[vi][vi] -= coeff;
                }
            }
            // else: xi * (1-xi) = 0, nothing to add
            return;
        }
        // Different variables: yi * yj
        let (lo, hi, lo_neg, hi_neg) = if vi < vj {
            (vi, vj, ni, nj)
        } else {
            (vj, vi, nj, ni)
        };
        // yi = xi if neg, else 1-xi
        // yj = xj if neg, else 1-xj
        // yi*yj = (xi if neg_i else 1-xi) * (xj if neg_j else 1-xj)
        match (lo_neg, hi_neg) {
            (true, true) => {
                // xi * xj
                matrix[lo][hi] += coeff;
            }
            (true, false) => {
                // xi * (1 - xj) = xi - xi*xj
                matrix[lo][lo] += coeff;
                matrix[lo][hi] -= coeff;
            }
            (false, true) => {
                // (1 - xi) * xj = xj - xi*xj
                matrix[hi][hi] += coeff;
                matrix[lo][hi] -= coeff;
            }
            (false, false) => {
                // (1-xi)(1-xj) = 1 - xi - xj + xi*xj
                // constant 1 ignored (offset)
                matrix[lo][lo] -= coeff;
                matrix[hi][hi] -= coeff;
                matrix[lo][hi] += coeff;
            }
        }
    };

    // Helper: add coefficient * yi * a to the matrix
    // where yi depends on variable vi and negation ni, a is aux variable
    let add_ya = |matrix: &mut [Vec<f64>], vi: usize, ni: bool, a: usize, coeff: f64| {
        // yi = xi if ni (negated literal), yi = 1-xi if !ni (positive literal)
        // yi * a:
        let (lo, hi) = if vi < a { (vi, a) } else { (a, vi) };
        if ni {
            // yi = xi, so yi * a = xi * a
            matrix[lo][hi] += coeff;
        } else {
            // yi = 1 - xi, so yi * a = a - xi * a
            matrix[a][a] += coeff;
            matrix[lo][hi] -= coeff;
        }
    };

    // Helper: add coefficient * yi to the matrix (linear term)
    let add_y = |matrix: &mut [Vec<f64>], vi: usize, ni: bool, coeff: f64| {
        if ni {
            // yi = xi
            matrix[vi][vi] += coeff;
        } else {
            // yi = 1 - xi, linear part: -coeff * xi (constant coeff ignored)
            matrix[vi][vi] -= coeff;
        }
    };

    // Term 1: a * y3 (coefficient = 1.0)
    add_ya(matrix, v3, n3, a, 1.0);

    // Term 2: M * y1 * y2
    add_yy(matrix, v1, n1, v2, n2, penalty);

    // Term 3: -2M * y1 * a
    add_ya(matrix, v1, n1, a, -2.0 * penalty);

    // Term 4: -2M * y2 * a
    add_ya(matrix, v2, n2, a, -2.0 * penalty);

    // Term 5: 3M * a (linear)
    // a is a binary variable, a^2 = a, so linear a → diagonal
    matrix[a][a] += 3.0 * penalty;

    // We also need to add linear terms that come from constant offsets in products
    // Actually, let's verify: the full expansion of
    //   H = a·y3 + M·(y1·y2 - 2·y1·a - 2·y2·a + 3·a)
    // All terms are handled above.
    //
    // However, we need to account for the case where "add_ya" with !ni adds
    // a linear term in `a`. Let me verify the add_ya logic handles this correctly.
    //
    // add_ya with ni=false: yi = 1-xi, yi*a = a - xi*a
    //   matrix[a][a] += coeff (linear in a)
    //   matrix[min(vi,a)][max(vi,a)] -= coeff (quadratic xi*a)
    // This is correct.

    // Note: We ignore constant terms (don't affect QUBO optimization).
    let _ = add_y; // suppress unused warning - linear terms in y handled via products
}

/// Build a QUBO matrix from a KSatisfiability instance.
///
/// For K=2, directly encodes quadratic penalties.
/// For K=3, uses Rosenberg quadratization with one auxiliary variable per clause.
///
/// Returns (matrix, num_source_vars) where matrix is (n + aux) x (n + aux).
fn build_qubo_matrix(
    num_vars: usize,
    clauses: &[crate::models::satisfiability::CNFClause],
    k: usize,
) -> Vec<Vec<f64>> {
    match k {
        2 => {
            let mut matrix = vec![vec![0.0; num_vars]; num_vars];
            for clause in clauses {
                add_2sat_clause_penalty(&mut matrix, &clause.literals);
            }
            matrix
        }
        3 => {
            let num_aux = clauses.len(); // one auxiliary per clause
            let total = num_vars + num_aux;
            let mut matrix = vec![vec![0.0; total]; total];
            for (idx, clause) in clauses.iter().enumerate() {
                let aux_var = num_vars + idx;
                add_3sat_clause_penalty(&mut matrix, &clause.literals, aux_var);
            }
            matrix
        }
        _ => unimplemented!("KSatisfiability to QUBO only supports K=2 and K=3"),
    }
}

#[reduction(
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vars))]) }
)]
impl ReduceTo<QUBO<f64>> for KSatisfiability<2> {
    type Result = ReductionKSatToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let matrix = build_qubo_matrix(n, self.clauses(), 2);

        ReductionKSatToQUBO {
            target: QUBO::from_matrix(matrix),
            source_num_vars: n,
        }
    }
}

#[reduction(
    overhead = { ReductionOverhead::new(vec![
        ("num_vars", poly!(num_vars) + poly!(num_clauses)),
    ]) }
)]
impl ReduceTo<QUBO<f64>> for KSatisfiability<3> {
    type Result = Reduction3SATToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let matrix = build_qubo_matrix(n, self.clauses(), 3);

        Reduction3SATToQUBO {
            target: QUBO::from_matrix(matrix),
            source_num_vars: n,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_qubo.rs"]
mod tests;
