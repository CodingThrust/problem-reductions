//! K-Satisfiability (K-SAT) problem implementation.
//!
//! K-SAT is a special case of SAT where each clause has exactly K literals.
//! Common variants include 3-SAT (K=3) and 2-SAT (K=2). This is the decision
//! version - for the optimization variant (MAX-K-SAT), see the separate
//! MaxKSatisfiability type (if available).

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::variant::{KValue, K2, K3, KN};
use serde::{Deserialize, Serialize};

use super::CNFClause;

pub(crate) fn first_n_odd_primes(count: usize) -> Vec<u64> {
    let mut primes = Vec::with_capacity(count);
    let mut candidate = 3u64;

    while primes.len() < count {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate += 2;
    }

    primes
}

fn is_prime(candidate: u64) -> bool {
    if candidate < 2 {
        return false;
    }
    if candidate == 2 {
        return true;
    }
    if candidate.is_multiple_of(2) {
        return false;
    }

    let mut divisor = 3u64;
    while divisor * divisor <= candidate {
        if candidate.is_multiple_of(divisor) {
            return false;
        }
        divisor += 2;
    }

    true
}

inventory::submit! {
    ProblemSchemaEntry {
        name: "KSatisfiability",
        display_name: "K-Satisfiability",
        aliases: &["KSAT"],
        dimensions: &[VariantDimension::new("k", "KN", &["KN", "K2", "K3"])],
        module_path: module_path!(),
        description: "SAT with exactly k literals per clause",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "Clauses each with exactly K literals" },
        ],
    }
}

/// K-Satisfiability problem where each clause has exactly K literals.
///
/// This is a restricted form of SAT where every clause must contain
/// exactly K literals. The most famous variant is 3-SAT (K=3), which
/// is NP-complete, while 2-SAT (K=2) is solvable in polynomial time.
/// This is the decision version of the problem.
///
/// # Type Parameters
/// * `K` - A type implementing `KValue` that specifies the number of literals per clause
///
/// # Example
///
/// ```
/// use problemreductions::models::formula::{KSatisfiability, CNFClause};
/// use problemreductions::variant::K3;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 3-SAT formula: (x1 OR x2 OR x3) AND (NOT x1 OR x2 OR NOT x3)
/// let problem = KSatisfiability::<K3>::new(
///     3,
///     vec![
///         CNFClause::new(vec![1, 2, 3]),       // x1 OR x2 OR x3
///         CNFClause::new(vec![-1, 2, -3]),     // NOT x1 OR x2 OR NOT x3
///     ],
/// );
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem);
/// assert!(!solutions.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct KSatisfiability<K: KValue> {
    /// Number of variables.
    num_vars: usize,
    /// Clauses in CNF, each with exactly K literals.
    clauses: Vec<CNFClause>,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<K>,
}

impl<K: KValue> KSatisfiability<K> {
    /// Create a new K-SAT problem.
    ///
    /// # Panics
    /// Panics if any clause does not have exactly K literals (when K is a
    /// concrete value like K2, K3). When K is KN (arbitrary), no clause-length
    /// validation is performed.
    pub fn new(num_vars: usize, clauses: Vec<CNFClause>) -> Self {
        if let Some(k) = K::K {
            for (i, clause) in clauses.iter().enumerate() {
                assert!(
                    clause.len() == k,
                    "Clause {} has {} literals, expected {}",
                    i,
                    clause.len(),
                    k
                );
            }
        }
        Self {
            num_vars,
            clauses,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new K-SAT problem allowing clauses with fewer than K literals.
    ///
    /// This is useful when the reduction algorithm produces clauses with
    /// fewer literals (e.g., when allow_less is true in the Julia implementation).
    ///
    /// # Panics
    /// Panics if any clause has more than K literals (when K is a concrete
    /// value like K2, K3). When K is KN (arbitrary), no clause-length
    /// validation is performed.
    pub fn new_allow_less(num_vars: usize, clauses: Vec<CNFClause>) -> Self {
        if let Some(k) = K::K {
            for (i, clause) in clauses.iter().enumerate() {
                assert!(
                    clause.len() <= k,
                    "Clause {} has {} literals, expected at most {}",
                    i,
                    clause.len(),
                    k
                );
            }
        }
        Self {
            num_vars,
            clauses,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get the number of variables.
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// Get the number of clauses.
    pub fn num_clauses(&self) -> usize {
        self.clauses.len()
    }

    /// Get the clauses.
    pub fn clauses(&self) -> &[CNFClause] {
        &self.clauses
    }

    /// Get a specific clause.
    pub fn get_clause(&self, index: usize) -> Option<&CNFClause> {
        self.clauses.get(index)
    }

    /// Get the total number of literals across all clauses.
    pub fn num_literals(&self) -> usize {
        self.clauses().iter().map(|c| c.len()).sum()
    }

    /// Padding term used by Sethi's Register Sufficiency reduction.
    pub fn register_sufficiency_padding(&self) -> usize {
        (2 * self.num_vars).saturating_sub(self.num_clauses())
    }

    pub fn simultaneous_incongruences_num_incongruences(&self) -> usize {
        first_n_odd_primes(self.num_vars)
            .into_iter()
            .map(|prime| usize::try_from(prime - 2).expect("prime fits in usize"))
            .sum::<usize>()
            + self.num_clauses()
    }

    pub fn simultaneous_incongruences_bound(&self) -> usize {
        first_n_odd_primes(self.num_vars)
            .into_iter()
            .try_fold(1usize, |product, prime| {
                product.checked_mul(usize::try_from(prime).expect("prime fits in usize"))
            })
            .expect("simultaneous incongruences bound must fit in usize")
    }

    /// Count satisfied clauses for an assignment.
    pub fn count_satisfied(&self, assignment: &[bool]) -> usize {
        self.clauses
            .iter()
            .filter(|c| c.is_satisfied(assignment))
            .count()
    }

    /// Check if an assignment satisfies all clauses.
    pub fn is_satisfying(&self, assignment: &[bool]) -> bool {
        self.clauses.iter().all(|c| c.is_satisfied(assignment))
    }
}

impl<K: KValue> Problem for KSatisfiability<K> {
    const NAME: &'static str = "KSatisfiability";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            let assignment = super::config_to_assignment(config);
            self.is_satisfying(&assignment)
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![K]
    }
}

crate::declare_variants! {
    default KSatisfiability<KN> => "2^num_variables",
    KSatisfiability<K2> => "num_variables + num_clauses",
    KSatisfiability<K3> => "1.307^num_variables",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use super::CNFClause;
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "ksatisfiability_k3",
        instance: Box::new(KSatisfiability::<K3>::new(
            3,
            vec![
                CNFClause::new(vec![1, 2, 3]),
                CNFClause::new(vec![-1, -2, 3]),
                CNFClause::new(vec![1, -2, -3]),
            ],
        )),
        optimal_config: vec![0, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/ksat.rs"]
mod tests;
