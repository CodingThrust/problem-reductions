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
use crate::rules::sat_helpers::SatVariableAllocator;
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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let n = self.source_num_vars;
        if target_solution.len() != n + 1 {
            return Err(crate::rules::ExtractionError::invalid(format!(
                "expected {} target truth values, got {}",
                n + 1,
                target_solution.len()
            )));
        }
        let sentinel = target_solution[n];
        Ok(target_solution[..n]
            .iter()
            .map(|&value| value ^ sentinel)
            .collect())
    }
}

#[reduction(
    size = exact {
        num_vars = "num_vars + 1",
        num_clauses = "num_clauses",
        num_literals = "num_literals + num_clauses",
    })]
impl ReduceTo<NAESatisfiability> for Satisfiability {
    type Result = ReductionSATToNAESAT;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vars();
        let mut variables = SatVariableAllocator::new("Satisfiability -> NAESatisfiability", n)
            .map_err(
                crate::rules::ReductionError::construction::<Satisfiability, NAESatisfiability>,
            )?;
        let sentinel_lit = variables.allocate().map_err(
            crate::rules::ReductionError::construction::<Satisfiability, NAESatisfiability>,
        )?;

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

        let target = NAESatisfiability::new(variables.num_vars(), nae_clauses);

        Ok(ReductionSATToNAESAT {
            source_num_vars: n,
            target,
        })
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
                    source_config: serde_json::json!(vec![false, true, false]),
                    target_config: serde_json::json!(vec![false, true, false, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/satisfiability_naesatisfiability.rs"]
mod tests;
