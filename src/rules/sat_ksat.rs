//! Reductions between Satisfiability and K-Satisfiability problems.
//!
//! SAT -> K-SAT: Convert general CNF to K-literal clauses using:
//! - Padding with ancilla variables for clauses with < K literals
//! - Splitting with ancilla variables for clauses with > K literals
//!
//! K-SAT -> SAT: Trivial embedding (K-SAT is a special case of SAT)

use crate::models::satisfiability::{CNFClause, KSatisfiability, Satisfiability};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::{KValue, K2, K3, KN};

/// Result of reducing general SAT to K-SAT.
///
/// This reduction transforms a SAT formula into an equisatisfiable K-SAT formula
/// by introducing ancilla (auxiliary) variables.
#[derive(Debug, Clone)]
pub struct ReductionSATToKSAT<K: KValue> {
    /// Number of original variables in the source problem.
    source_num_vars: usize,
    /// The target K-SAT problem.
    target: KSatisfiability<K>,
}

impl<K: KValue> ReductionResult for ReductionSATToKSAT<K> {
    type Source = Satisfiability;
    type Target = KSatisfiability<K>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Only return the original variables, discarding ancillas
        target_solution[..self.source_num_vars].to_vec()
    }
}

/// Add a clause to the K-SAT formula, splitting or padding as necessary.
///
/// # Algorithm
/// - If clause has exactly K literals: add as-is
/// - If clause has < K literals: pad with ancilla variables (both positive and negative)
/// - If clause has > K literals: split recursively using ancilla variables
///
/// # Arguments
/// * `k` - Target number of literals per clause
/// * `clause` - The clause to add
/// * `result_clauses` - Output vector to append clauses to
/// * `next_var` - Next available variable number (1-indexed)
///
/// # Returns
/// Updated next_var after any ancilla variables are created
fn add_clause_to_ksat(
    k: usize,
    clause: &CNFClause,
    result_clauses: &mut Vec<CNFClause>,
    mut next_var: i32,
) -> i32 {
    let len = clause.len();

    if len == k {
        // Exact size: add as-is
        result_clauses.push(clause.clone());
    } else if len < k {
        // Too few literals: pad with ancilla variables
        // Create both positive and negative versions to maintain satisfiability
        // (a v b) with k=3 becomes (a v b v x) AND (a v b v -x)
        let ancilla = next_var;
        next_var += 1;

        // Add clause with positive ancilla
        let mut lits_pos = clause.literals.clone();
        lits_pos.push(ancilla);
        next_var = add_clause_to_ksat(k, &CNFClause::new(lits_pos), result_clauses, next_var);

        // Add clause with negative ancilla
        let mut lits_neg = clause.literals.clone();
        lits_neg.push(-ancilla);
        next_var = add_clause_to_ksat(k, &CNFClause::new(lits_neg), result_clauses, next_var);
    } else {
        // Too many literals: split using ancilla variable
        // (a v b v c v d) with k=3 becomes (a v b v x) AND (-x v c v d)
        assert!(k >= 3, "K must be at least 3 for splitting");

        let ancilla = next_var;
        next_var += 1;

        // First clause: first k-1 literals + positive ancilla
        let mut first_lits: Vec<i32> = clause.literals[..k - 1].to_vec();
        first_lits.push(ancilla);
        result_clauses.push(CNFClause::new(first_lits));

        // Remaining clause: negative ancilla + remaining literals
        let mut remaining_lits = vec![-ancilla];
        remaining_lits.extend_from_slice(&clause.literals[k - 1..]);
        let remaining_clause = CNFClause::new(remaining_lits);

        // Recursively process the remaining clause
        next_var = add_clause_to_ksat(k, &remaining_clause, result_clauses, next_var);
    }

    next_var
}

/// Implementation of SAT -> K-SAT reduction.
///
/// Note: We implement this for specific K values rather than generic K
/// because the `#[reduction]` proc macro requires concrete types.
macro_rules! impl_sat_to_ksat {
    ($ktype:ty, $k:expr) => {
        #[reduction(overhead = {
                                    ReductionOverhead::new(vec![
                                        ("num_clauses", poly!(num_clauses) + poly!(num_literals)),
                                        ("num_vars", poly!(num_vars) + poly!(num_literals)),
                                    ])
                                })]
        impl ReduceTo<KSatisfiability<$ktype>> for Satisfiability {
            type Result = ReductionSATToKSAT<$ktype>;

            fn reduce_to(&self) -> Self::Result {
                let source_num_vars = self.num_vars();
                let mut result_clauses = Vec::new();
                let mut next_var = (source_num_vars + 1) as i32; // 1-indexed

                for clause in self.clauses() {
                    next_var = add_clause_to_ksat($k, clause, &mut result_clauses, next_var);
                }

                // Calculate total number of variables (original + ancillas)
                let total_vars = (next_var - 1) as usize;

                let target = KSatisfiability::<$ktype>::new(total_vars, result_clauses);

                ReductionSATToKSAT {
                    source_num_vars,
                    target,
                }
            }
        }
    };
}

// Implement for K=3 (the canonical NP-complete case)
impl_sat_to_ksat!(K3, 3);

/// Result of reducing K-SAT to general SAT.
///
/// This is a trivial embedding since K-SAT is a special case of SAT.
#[derive(Debug, Clone)]
pub struct ReductionKSATToSAT<K: KValue> {
    /// The target SAT problem.
    target: Satisfiability,
    _phantom: std::marker::PhantomData<K>,
}

impl<K: KValue> ReductionResult for ReductionKSATToSAT<K> {
    type Source = KSatisfiability<K>;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Direct mapping - no transformation needed
        target_solution.to_vec()
    }
}

/// Helper function for KSAT -> SAT reduction logic (generic over K).
fn reduce_ksat_to_sat<K: KValue>(ksat: &KSatisfiability<K>) -> ReductionKSATToSAT<K> {
    let clauses = ksat.clauses().to_vec();
    let target = Satisfiability::new(ksat.num_vars(), clauses);

    ReductionKSATToSAT {
        target,
        _phantom: std::marker::PhantomData,
    }
}

/// Macro for concrete KSAT -> SAT reduction impls.
/// The `#[reduction]` macro requires concrete types.
macro_rules! impl_ksat_to_sat {
    ($ktype:ty) => {
        #[reduction(overhead = {
                                            ReductionOverhead::new(vec![
                                                ("num_clauses", poly!(num_clauses)),
                                                ("num_vars", poly!(num_vars)),
                                            ])
                                        })]
        impl ReduceTo<Satisfiability> for KSatisfiability<$ktype> {
            type Result = ReductionKSATToSAT<$ktype>;

            fn reduce_to(&self) -> Self::Result {
                reduce_ksat_to_sat(self)
            }
        }
    };
}

// Register KN for the reduction graph (covers all K values as the generic entry)
impl_ksat_to_sat!(KN);
// Register K3 and K2 as concrete entries (used directly in tests and reductions)
impl_ksat_to_sat!(K3);
impl_ksat_to_sat!(K2);

#[cfg(test)]
#[path = "../unit_tests/rules/sat_ksat.rs"]
mod tests;
