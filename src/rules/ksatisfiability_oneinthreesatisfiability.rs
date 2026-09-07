//! Reduction from KSatisfiability (3-SAT) to One-In-Three Satisfiability.
//!
//! Schaefer's Lemma 3.5 (STOC 1978) expresses a three-input disjunction
//! using five one-in-three constraints. Missing native clause positions
//! use the forced false variable; appearing source variables are compacted
//! and restored through an inverse map during extraction.

use crate::models::formula::{CNFClause, KSatisfiability, OneInThreeSatisfiability};
use crate::reduction;
use crate::rules::sat_helpers::SatVariableAllocator;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Reduction3SATToOneInThreeSAT {
    source_num_vars: usize,
    source_variables: Vec<usize>,
    target: OneInThreeSatisfiability,
}

impl ReductionResult for Reduction3SATToOneInThreeSAT {
    type Source = KSatisfiability<K3>;
    type Target = OneInThreeSatisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !value.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target assignment does not satisfy every one-in-three clause",
            ));
        }
        let mut assignment = vec![false; self.source_num_vars];
        for (compact, &original) in self.source_variables.iter().enumerate() {
            assignment[original] = target_solution[compact];
        }
        Ok(assignment)
    }
}

#[reduction(
    transform = upper_bound {
        num_vars = "num_vars + 2 + 6 * num_clauses",
        num_clauses = "1 + 5 * num_clauses",
    })]
impl ReduceTo<OneInThreeSatisfiability> for KSatisfiability<K3> {
    type Result = Reduction3SATToOneInThreeSAT;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let source_num_vars = self.num_vars();
        let source_variables: Vec<_> = self
            .clauses()
            .iter()
            .flat_map(|clause| clause.literals.iter())
            .map(|literal| {
                usize::try_from(literal.unsigned_abs()).expect("native SAT indices fit usize") - 1
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut variables = SatVariableAllocator::new(
            "KSatisfiability -> OneInThreeSatisfiability",
            source_variables.len(),
        )
        .map_err(
            crate::rules::ReductionError::construction::<
                KSatisfiability<K3>,
                OneInThreeSatisfiability,
            >,
        )?;
        let sentinels = variables.allocate_many(2).map_err(
            crate::rules::ReductionError::construction::<
                KSatisfiability<K3>,
                OneInThreeSatisfiability,
            >,
        )?;
        let z_false = sentinels[0];
        let z_true = sentinels[1];

        let capacity = self
            .num_clauses()
            .checked_mul(5)
            .and_then(|count| count.checked_add(1))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    KSatisfiability<K3>,
                    OneInThreeSatisfiability,
                >("computing the target clause count")
            })?;
        let mut clauses = Vec::with_capacity(capacity);
        clauses.push(CNFClause::new(vec![z_false, z_false, z_true]));

        for clause in self.clauses() {
            // Adding false disjuncts preserves every native clause, including
            // the empty disjunction, while using the same three-input gadget.
            let mut literals = [z_false; 3];
            for (position, &literal) in clause.literals.iter().enumerate() {
                let original = usize::try_from(literal.unsigned_abs())
                    .expect("native SAT indices fit usize")
                    - 1;
                let compact = source_variables
                    .binary_search(&original)
                    .expect("all appearing variables were collected");
                let variable = i64::try_from(compact + 1).expect("compact SAT indices fit i64");
                literals[position] = if literal > 0 { variable } else { -variable };
            }
            let [l1, l2, l3] = literals;
            let allocated = variables.allocate_many(6).map_err(
                crate::rules::ReductionError::construction::<
                    KSatisfiability<K3>,
                    OneInThreeSatisfiability,
                >,
            )?;
            let [a, b, c, d, e, f] = allocated.as_slice() else {
                return Err(crate::rules::ReductionError::invalid_target::<
                    KSatisfiability<K3>,
                    OneInThreeSatisfiability,
                >(
                    "SAT allocator returned an unexpected variable count"
                ));
            };

            clauses.push(CNFClause::new(vec![l1, *a, *d]));
            clauses.push(CNFClause::new(vec![l2, *b, *d]));
            clauses.push(CNFClause::new(vec![*a, *b, *e]));
            clauses.push(CNFClause::new(vec![*c, *d, *f]));
            clauses.push(CNFClause::new(vec![l3, *c, z_false]));
        }

        let target = OneInThreeSatisfiability::try_new(variables.num_vars(), clauses).map_err(
            crate::rules::ReductionError::construction::<Self, OneInThreeSatisfiability>,
        )?;

        Ok(Reduction3SATToOneInThreeSAT {
            source_num_vars,
            source_variables,
            target,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_oneinthreesatisfiability",
        build: || {
            let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
            crate::example_db::specs::rule_example_with_witness::<_, OneInThreeSatisfiability>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![false, false, true]),
                    target_config: serde_json::json!(vec![
                        false, false, true, false, true, false, false, false, true, true, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_oneinthreesatisfiability.rs"]
mod tests;
