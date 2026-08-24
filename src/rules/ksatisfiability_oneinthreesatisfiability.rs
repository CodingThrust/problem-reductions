//! Reduction from KSatisfiability (3-SAT) to One-In-Three Satisfiability.

use crate::models::formula::{CNFClause, KSatisfiability, OneInThreeSatisfiability};
use crate::reduction;
use crate::rules::sat_helpers::SatVariableAllocator;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;

#[derive(Debug, Clone)]
pub struct Reduction3SATToOneInThreeSAT {
    source_num_vars: usize,
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
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.source_num_vars].to_vec())
    }
}

#[reduction(
    size = exact {
        num_vars = "num_vars + 2 + 6 * num_clauses",
        num_clauses = "1 + 5 * num_clauses",
    })]
impl ReduceTo<OneInThreeSatisfiability> for KSatisfiability<K3> {
    type Result = Reduction3SATToOneInThreeSAT;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let source_num_vars = self.num_vars();
        let mut variables = SatVariableAllocator::new(
            "KSatisfiability -> OneInThreeSatisfiability",
            source_num_vars,
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
            let [l1, l2, l3] = clause.literals.as_slice() else {
                return Err(crate::rules::ReductionError::invalid_target::<
                    KSatisfiability<K3>,
                    OneInThreeSatisfiability,
                >(
                    "source K3 clause does not contain exactly three literals"
                ));
            };
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

            clauses.push(CNFClause::new(vec![*l1, *a, *d]));
            clauses.push(CNFClause::new(vec![*l2, *b, *d]));
            clauses.push(CNFClause::new(vec![*a, *b, *e]));
            clauses.push(CNFClause::new(vec![*c, *d, *f]));
            clauses.push(CNFClause::new(vec![*l3, *c, z_false]));
        }

        let target = OneInThreeSatisfiability::new(variables.num_vars(), clauses);

        Ok(Reduction3SATToOneInThreeSAT {
            source_num_vars,
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
                    source_config: vec![0, 0, 1],
                    target_config: vec![0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_oneinthreesatisfiability.rs"]
mod tests;
