//! Reduction from Satisfiability to NAE-Satisfiability.
//!
//! Given a SAT instance with n variables and m clauses, we construct an
//! equisatisfiable NAE-SAT instance by adding a fresh sentinel variable s.
//! Each SAT clause C_j = (l_1 ∨ ... ∨ l_k) becomes NAE clause
//! C'_j = (l_1, ..., l_k, s). The sentinel ensures that each NAE clause
//! has at least one false literal (the sentinel itself when s=false, or
//! the complement of the original satisfied literal when s=true).

use crate::models::formula::{CNFClause, NAESatisfiability, Satisfiability};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Satisfiability to NAE-Satisfiability.
#[derive(Debug, Clone)]
pub struct ReductionSATToNAESAT {
    /// Number of original variables in the source problem.
    source_num_vars: usize,
    /// The target NAE-SAT problem.
    target: NAESatisfiability,
}

impl ReductionResult for ReductionSATToNAESAT {
    type Source = Satisfiability;
    type Target = NAESatisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.source_num_vars;
        if target_solution.len() <= n {
            return vec![0; n];
        }
        // The sentinel variable is the last variable (index n).
        let sentinel_value = target_solution[n];
        if sentinel_value == 0 {
            // Sentinel is false: return first n variables as-is.
            target_solution[..n].to_vec()
        } else {
            // Sentinel is true: return complement of first n variables.
            target_solution[..n].iter().map(|&v| 1 - v).collect()
        }
    }
}

#[reduction(overhead = {
    num_vars = "num_vars + 1",
    num_clauses = "num_clauses",
    num_literals = "num_literals + num_clauses",
})]
impl ReduceTo<NAESatisfiability> for Satisfiability {
    type Result = ReductionSATToNAESAT;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        // Sentinel variable has 0-indexed position n, so its 1-indexed literal is n+1.
        let sentinel_lit = (n + 1) as i32;

        let nae_clauses: Vec<CNFClause> = self
            .clauses()
            .iter()
            .map(|clause| {
                if clause.literals.is_empty() {
                    // SAT allows empty clauses, which make the instance unsatisfiable.
                    // Map to a fixed unsatisfiable NAE clause (s, s) of length 2.
                    CNFClause::new(vec![sentinel_lit, sentinel_lit])
                } else {
                    let mut lits = clause.literals.clone();
                    lits.push(sentinel_lit);
                    CNFClause::new(lits)
                }
            })
            .collect();

        let target = NAESatisfiability::new(n + 1, nae_clauses);

        ReductionSATToNAESAT {
            source_num_vars: n,
            target,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "satisfiability_to_naesatisfiability",
        build: || {
            let source = Satisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2]),
                    CNFClause::new(vec![-1, 3]),
                    CNFClause::new(vec![-2, -3]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<_, NAESatisfiability>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 0],
                    target_config: vec![0, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/satisfiability_naesatisfiability.rs"]
mod tests;
