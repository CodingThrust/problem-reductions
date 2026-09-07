//! Reductions between Satisfiability and K-Satisfiability problems.
//!
//! SAT -> K-SAT: Convert general CNF to K-literal clauses using:
//! - Padding with ancilla variables for clauses with < K literals
//! - Splitting with ancilla variables for clauses with > K literals
//!
//! K-SAT -> SAT: Trivial embedding (K-SAT is a special case of SAT)

use crate::models::formula::{CNFClause, KSatisfiability, Satisfiability};
use crate::reduction;
use crate::rules::sat_helpers::SatVariableAllocator;
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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            // Only return the original variables, discarding ancillas
            target_solution[..self.source_num_vars].to_vec()
        })
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
fn add_clause_to_ksat(
    k: usize,
    clause: &CNFClause,
    result_clauses: &mut Vec<CNFClause>,
    variables: &mut SatVariableAllocator,
) -> Result<(), crate::registry::ConstructionError> {
    let len = clause.len();

    if len == k {
        // Exact size: add as-is
        result_clauses.push(clause.clone());
    } else if len < k {
        // Too few literals: pad with ancilla variables
        // Create both positive and negative versions to maintain satisfiability
        // (a v b) with k=3 becomes (a v b v x) AND (a v b v -x)
        let ancilla = variables.allocate()?;

        // Add clause with positive ancilla
        let mut lits_pos = clause.literals.clone();
        lits_pos.push(ancilla);
        add_clause_to_ksat(k, &CNFClause::new(lits_pos), result_clauses, variables)?;

        // Add clause with negative ancilla
        let mut lits_neg = clause.literals.clone();
        lits_neg.push(-ancilla);
        add_clause_to_ksat(k, &CNFClause::new(lits_neg), result_clauses, variables)?;
    } else {
        // Too many literals: split using ancilla variable
        // (a v b v c v d) with k=3 becomes (a v b v x) AND (-x v c v d)
        if k < 3 {
            return Err(format!(
                "cannot split a clause with {} literals into {k}-literal clauses",
                clause.len()
            )
            .into());
        }

        let ancilla = variables.allocate()?;

        // First clause: first k-1 literals + positive ancilla
        let mut first_lits: Vec<i64> = clause.literals[..k - 1].to_vec();
        first_lits.push(ancilla);
        result_clauses.push(CNFClause::new(first_lits));

        // Remaining clause: negative ancilla + remaining literals
        let mut remaining_lits = vec![-ancilla];
        remaining_lits.extend_from_slice(&clause.literals[k - 1..]);
        let remaining_clause = CNFClause::new(remaining_lits);

        // Recursively process the remaining clause
        add_clause_to_ksat(k, &remaining_clause, result_clauses, variables)?;
    }

    Ok(())
}

/// Implementation of SAT -> K-SAT reduction.
///
/// Note: We implement this for specific K values rather than generic K
/// because the `#[reduction]` proc macro requires concrete types.
macro_rules! impl_sat_to_ksat {
    ($ktype:ty, $k:expr) => {
        #[rustfmt::skip]
        #[reduction(
    transform = upper_bound {
        num_clauses = "4 * num_clauses + num_literals",
        num_vars = "num_vars + 3 * num_clauses + num_literals",
    },
    unavailable = {
        num_literals = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
        impl ReduceTo<KSatisfiability<$ktype>> for Satisfiability {
            type Result = ReductionSATToKSAT<$ktype>;

            fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
                let source_num_vars = self.num_vars();
                let mut result_clauses = Vec::new();
                let mut variables = SatVariableAllocator::new(
                    "Satisfiability -> KSatisfiability",
                    source_num_vars,
                ).map_err(crate::rules::ReductionError::construction::<
                    Satisfiability,
                    KSatisfiability<$ktype>,
                >)?;

                for clause in self.clauses() {
                    add_clause_to_ksat($k, clause, &mut result_clauses, &mut variables)
                        .map_err(crate::rules::ReductionError::construction::<
                            Satisfiability,
                            KSatisfiability<$ktype>,
                        >)?;
                }

                let target = KSatisfiability::<$ktype>::new(variables.num_vars(), result_clauses);

                Ok(ReductionSATToKSAT {
                    source_num_vars,
                    target,
                })
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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            // Direct mapping - no transformation needed
            target_solution.to_vec()
        })
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
#[rustfmt::skip]
        #[reduction(
    transform = exact {
        num_clauses = "num_clauses",
        num_vars = "num_vars",
        num_literals = "num_literals",
    })]
        impl ReduceTo<Satisfiability> for KSatisfiability<$ktype> {
            type Result = ReductionKSATToSAT<$ktype>;

            fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
                Ok(reduce_ksat_to_sat(self))
            }
        }
    };
}

// Register KN for the reduction graph (covers all K values as the generic entry)
impl_ksat_to_sat!(KN);

// K3 and K2 keep their ReduceTo<Satisfiability> impls for typed use,
// but are NOT registered as separate primitive graph edges (KN covers them).
impl ReduceTo<Satisfiability> for KSatisfiability<K3> {
    type Result = ReductionKSATToSAT<K3>;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(reduce_ksat_to_sat(self))
    }
}

impl ReduceTo<Satisfiability> for KSatisfiability<K2> {
    type Result = ReductionKSATToSAT<K2>;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(reduce_ksat_to_sat(self))
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "satisfiability_to_ksatisfiability",
            build: || {
                let source = Satisfiability::new(
                    5,
                    vec![
                        CNFClause::new(vec![1]),
                        CNFClause::new(vec![2, -3]),
                        CNFClause::new(vec![-1, 3, 4]),
                        CNFClause::new(vec![2, -4, 5]),
                        CNFClause::new(vec![1, -2, 3, -5]),
                        CNFClause::new(vec![-1, 2, -3, 4, 5]),
                    ],
                );
                crate::example_db::specs::rule_example_with_witness::<_, KSatisfiability<K3>>(
                    source,
                    SolutionPair {
                        source_config: serde_json::json!(vec![true, true, true, false, true]),
                        target_config: serde_json::json!(vec![
                            true, true, true, false, true, false, false, false, false, true, true,
                            true
                        ]),
                    },
                )
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "ksatisfiability_to_satisfiability",
            build: || {
                let source = KSatisfiability::<KN>::new(
                    4,
                    vec![
                        CNFClause::new(vec![1, -2, 3]),
                        CNFClause::new(vec![-1, 3, 4]),
                        CNFClause::new(vec![2, -3, -4]),
                    ],
                );
                crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                    source,
                    SolutionPair {
                        source_config: serde_json::json!(vec![true, true, true, false]),
                        target_config: serde_json::json!(vec![true, true, true, false]),
                    },
                )
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sat_ksat.rs"]
mod tests;
