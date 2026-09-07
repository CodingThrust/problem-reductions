//! Reduction from Satisfiability to Maximum 2-Satisfiability.

use crate::models::formula::{CNFClause, Maximum2Satisfiability, Satisfiability};
use crate::reduction;
use crate::rules::sat_helpers::SatVariableAllocator;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::{Max, Or};

/// Result of reducing SAT to MAX-2-SAT.
#[derive(Debug, Clone)]
pub struct ReductionSatisfiabilityToMaximum2Satisfiability {
    target: Maximum2Satisfiability,
    source_num_vars: usize,
    target_score: i64,
}

impl ReductionResult for ReductionSatisfiabilityToMaximum2Satisfiability {
    type Source = Satisfiability;
    type Target = Maximum2Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        let certificate = crate::rules::AggregateReductionResult::extract_value(self, value);
        if !certificate.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target assignment does not certify satisfiability",
            ));
        }

        Ok(target_solution[..self.source_num_vars].to_vec())
    }
}

impl crate::rules::AggregateReductionResult for ReductionSatisfiabilityToMaximum2Satisfiability {
    type Source = Satisfiability;
    type Target = Maximum2Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, value: Max<i64>) -> Or {
        Or(value == Max(Some(self.target_score)))
    }
}

fn add_normalized_clause(
    clause: &CNFClause,
    variables: &mut SatVariableAllocator,
    normalized: &mut Vec<CNFClause>,
) -> Result<(), crate::registry::ConstructionError> {
    match clause.len() {
        0 => {
            let y = variables.allocate()?;
            normalized.push(CNFClause::new(vec![y, y, y]));
            normalized.push(CNFClause::new(vec![-y, -y, -y]));
        }
        1 => {
            let l1 = clause.literals[0];
            let allocated = variables.allocate_many(2)?;
            let y = allocated[0];
            let z = allocated[1];
            normalized.push(CNFClause::new(vec![l1, y, z]));
            normalized.push(CNFClause::new(vec![l1, y, -z]));
            normalized.push(CNFClause::new(vec![l1, -y, z]));
            normalized.push(CNFClause::new(vec![l1, -y, -z]));
        }
        2 => {
            let l1 = clause.literals[0];
            let l2 = clause.literals[1];
            let y = variables.allocate()?;
            normalized.push(CNFClause::new(vec![l1, l2, y]));
            normalized.push(CNFClause::new(vec![l1, l2, -y]));
        }
        3 => normalized.push(clause.clone()),
        k => {
            let literals = &clause.literals;
            let y_vars = variables.allocate_many(k - 3)?;

            normalized.push(CNFClause::new(vec![literals[0], literals[1], y_vars[0]]));
            for i in 1..k - 3 {
                normalized.push(CNFClause::new(vec![
                    -y_vars[i - 1],
                    literals[i + 1],
                    y_vars[i],
                ]));
            }
            normalized.push(CNFClause::new(vec![
                -y_vars[y_vars.len() - 1],
                literals[k - 2],
                literals[k - 1],
            ]));
        }
    }
    Ok(())
}

fn add_gjs_gadget(clause: &CNFClause, w: i64, target_clauses: &mut Vec<CNFClause>) {
    let a = clause.literals[0];
    let b = clause.literals[1];
    let c = clause.literals[2];

    target_clauses.push(CNFClause::new(vec![a, a]));
    target_clauses.push(CNFClause::new(vec![b, b]));
    target_clauses.push(CNFClause::new(vec![c, c]));
    target_clauses.push(CNFClause::new(vec![w, w]));
    target_clauses.push(CNFClause::new(vec![-a, -b]));
    target_clauses.push(CNFClause::new(vec![-b, -c]));
    target_clauses.push(CNFClause::new(vec![-a, -c]));
    target_clauses.push(CNFClause::new(vec![a, -w]));
    target_clauses.push(CNFClause::new(vec![b, -w]));
    target_clauses.push(CNFClause::new(vec![c, -w]));
}

#[reduction(
    aggregate = custom,
    transform = upper_bound {
        num_vars = "num_vars + 2 * num_literals + 4 * num_clauses",
        num_clauses = "10 * (num_literals + 3 * num_clauses)",
    }
)]
impl ReduceTo<Maximum2Satisfiability> for Satisfiability {
    type Result = ReductionSatisfiabilityToMaximum2Satisfiability;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let mut normalized = Vec::new();
        let mut variables =
            SatVariableAllocator::new("Satisfiability -> Maximum2Satisfiability", self.num_vars())
                .map_err(
                    crate::rules::ReductionError::construction::<
                        Satisfiability,
                        Maximum2Satisfiability,
                    >,
                )?;

        for clause in self.clauses() {
            add_normalized_clause(clause, &mut variables, &mut normalized).map_err(
                crate::rules::ReductionError::construction::<
                    Satisfiability,
                    Maximum2Satisfiability,
                >,
            )?;
        }

        let capacity =
            normalized.len().checked_mul(10).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    Satisfiability,
                    Maximum2Satisfiability,
                >("computing the target clause count")
            })?;
        let clause_count = <Self as ReduceTo<Maximum2Satisfiability>>::exact_i64(
            capacity,
            "representing every satisfied-clause count",
        )?;
        // capacity=10*M. The target value is at most 10*M, and each
        // normalized clause contributes at most seven satisfied clauses.
        // Dividing first guarantees this exact score stays within i64.
        let target_score = (clause_count / 10) * 7;
        let mut target_clauses = Vec::with_capacity(capacity);
        for clause in &normalized {
            let w =
                variables.allocate().map_err(
                    crate::rules::ReductionError::construction::<
                        Satisfiability,
                        Maximum2Satisfiability,
                    >,
                )?;
            add_gjs_gadget(clause, w, &mut target_clauses);
        }

        let target = Maximum2Satisfiability::try_new(variables.num_vars(), target_clauses)
            .map_err(<Self as ReduceTo<Maximum2Satisfiability>>::target_construction)?;

        Ok(ReductionSatisfiabilityToMaximum2Satisfiability {
            target,
            source_num_vars: self.num_vars(),
            target_score,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "satisfiability_to_maximum2satisfiability",
        build: || {
            let source = Satisfiability::new(
                3,
                vec![CNFClause::new(vec![1, -2, 3]), CNFClause::new(vec![-1, 2])],
            );
            crate::example_db::specs::rule_example_with_witness::<_, Maximum2Satisfiability>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![true, true, true]),
                    target_config: serde_json::json!(vec![
                        true, true, true, false, true, false, true
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/satisfiability_maximum2satisfiability.rs"]
mod tests;
