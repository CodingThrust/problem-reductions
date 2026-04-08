//! Reduction from Satisfiability to Maximum 2-Satisfiability.

use crate::models::formula::{CNFClause, Maximum2Satisfiability, Satisfiability};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SAT to MAX-2-SAT.
#[derive(Debug, Clone)]
pub struct ReductionSatisfiabilityToMaximum2Satisfiability {
    target: Maximum2Satisfiability,
    source_num_vars: usize,
}

impl ReductionResult for ReductionSatisfiabilityToMaximum2Satisfiability {
    type Source = Satisfiability;
    type Target = Maximum2Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.source_num_vars].to_vec()
    }
}

fn add_normalized_clause(clause: &CNFClause, next_var: &mut i32, normalized: &mut Vec<CNFClause>) {
    match clause.len() {
        0 => {
            let y = *next_var;
            *next_var += 1;
            normalized.push(CNFClause::new(vec![y, y, y]));
            normalized.push(CNFClause::new(vec![-y, -y, -y]));
        }
        1 => {
            let l1 = clause.literals[0];
            let y = *next_var;
            let z = *next_var + 1;
            *next_var += 2;
            normalized.push(CNFClause::new(vec![l1, y, z]));
            normalized.push(CNFClause::new(vec![l1, y, -z]));
            normalized.push(CNFClause::new(vec![l1, -y, z]));
            normalized.push(CNFClause::new(vec![l1, -y, -z]));
        }
        2 => {
            let l1 = clause.literals[0];
            let l2 = clause.literals[1];
            let y = *next_var;
            *next_var += 1;
            normalized.push(CNFClause::new(vec![l1, l2, y]));
            normalized.push(CNFClause::new(vec![l1, l2, -y]));
        }
        3 => normalized.push(clause.clone()),
        k => {
            let literals = &clause.literals;
            let y_vars: Vec<i32> = (*next_var..*next_var + (k as i32 - 3)).collect();
            *next_var += k as i32 - 3;

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
}

fn add_gjs_gadget(clause: &CNFClause, w: i32, target_clauses: &mut Vec<CNFClause>) {
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
    overhead = {
        num_vars = "num_vars + 2 * num_literals + 4 * num_clauses",
        num_clauses = "10 * (num_literals + 3 * num_clauses)",
    }
)]
impl ReduceTo<Maximum2Satisfiability> for Satisfiability {
    type Result = ReductionSatisfiabilityToMaximum2Satisfiability;

    fn reduce_to(&self) -> Self::Result {
        let mut normalized = Vec::new();
        let mut next_var = self.num_vars() as i32 + 1;

        for clause in self.clauses() {
            add_normalized_clause(clause, &mut next_var, &mut normalized);
        }

        let mut target_clauses = Vec::with_capacity(normalized.len() * 10);
        for clause in &normalized {
            let w = next_var;
            next_var += 1;
            add_gjs_gadget(clause, w, &mut target_clauses);
        }

        let target = Maximum2Satisfiability::new((next_var - 1) as usize, target_clauses);

        ReductionSatisfiabilityToMaximum2Satisfiability {
            target,
            source_num_vars: self.num_vars(),
        }
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
                    source_config: vec![1, 1, 1],
                    target_config: vec![1, 1, 1, 0, 1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/satisfiability_maximum2satisfiability.rs"]
mod tests;
