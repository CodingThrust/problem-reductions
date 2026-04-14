//! Reduction from Satisfiability to CircuitSAT.
//!
//! Converts a CNF formula into a boolean circuit by creating
//! an OR gate for each clause and a final AND gate.

use crate::models::formula::Satisfiability;
use crate::models::formula::{Assignment, BooleanExpr, Circuit, CircuitSAT};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use std::collections::HashSet;

/// Result of reducing SAT to CircuitSAT.
#[derive(Debug, Clone)]
pub struct ReductionSATToCircuit {
    target: CircuitSAT,
    /// Indices of original SAT variables in the CircuitSAT variable list.
    source_var_indices: Vec<usize>,
}

impl ReductionResult for ReductionSATToCircuit {
    type Source = Satisfiability;
    type Target = CircuitSAT;

    fn target_problem(&self) -> &CircuitSAT {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.source_var_indices
            .iter()
            .map(|&idx| target_solution[idx])
            .collect()
    }
}

#[reduction(
    overhead = {
        num_variables = "num_vars + num_clauses + 1",
        num_assignments = "num_clauses + 2",
    }
)]
impl ReduceTo<CircuitSAT> for Satisfiability {
    type Result = ReductionSATToCircuit;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_variables();
        let clauses = self.clauses();

        let mut assignments = Vec::new();
        let mut clause_outputs = Vec::new();

        for (i, clause) in clauses.iter().enumerate() {
            let clause_output = format!("__clause_{}", i);
            let literal_exprs: Vec<BooleanExpr> = clause
                .literals
                .iter()
                .map(|&lit| {
                    let var_name = format!("x{}", lit.unsigned_abs());
                    let var_expr = BooleanExpr::var(&var_name);
                    if lit < 0 {
                        BooleanExpr::not(var_expr)
                    } else {
                        var_expr
                    }
                })
                .collect();

            let clause_expr = if literal_exprs.len() == 1 {
                literal_exprs.into_iter().next().unwrap()
            } else {
                BooleanExpr::or(literal_exprs)
            };

            assignments.push(Assignment::new(vec![clause_output.clone()], clause_expr));
            clause_outputs.push(clause_output);
        }

        // Final AND gate
        let final_output = "__out".to_string();
        let and_expr = if clause_outputs.len() == 1 {
            BooleanExpr::var(&clause_outputs[0])
        } else {
            BooleanExpr::and(
                clause_outputs
                    .iter()
                    .map(|name| BooleanExpr::var(name))
                    .collect(),
            )
        };
        assignments.push(Assignment::new(vec![final_output.clone()], and_expr));

        // Constrain the final output to be true
        assignments.push(Assignment::new(
            vec![final_output],
            BooleanExpr::constant(true),
        ));

        // Add identity assignments for variables that don't appear in any clause,
        // so they are present in Circuit::variables() for index mapping.
        let used_vars: HashSet<usize> = clauses
            .iter()
            .flat_map(|c| c.literals.iter().map(|&lit| lit.unsigned_abs() as usize))
            .collect();
        for i in 1..=num_vars {
            if !used_vars.contains(&i) {
                let var_name = format!("x{}", i);
                assignments.push(Assignment::new(
                    vec![format!("__unused_{}", i)],
                    BooleanExpr::var(&var_name),
                ));
            }
        }

        let circuit = Circuit::new(assignments);
        let target = CircuitSAT::new(circuit);

        // Map SAT variable indices to CircuitSAT variable indices
        let var_names = target.variable_names();
        let source_var_indices: Vec<usize> = (1..=num_vars)
            .map(|i| {
                let name = format!("x{}", i);
                var_names
                    .iter()
                    .position(|n| n == &name)
                    .unwrap_or_else(|| panic!("Variable {} not found in CircuitSAT", name))
            })
            .collect();

        ReductionSATToCircuit {
            target,
            source_var_indices,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "satisfiability_to_circuitsat",
        build: || {
            let source = Satisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, -2, 3]),
                    CNFClause::new(vec![-1, 2]),
                    CNFClause::new(vec![2, 3]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<_, CircuitSAT>(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 1],
                    target_config: vec![1, 1, 1, 1, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sat_circuitsat.rs"]
mod tests;
