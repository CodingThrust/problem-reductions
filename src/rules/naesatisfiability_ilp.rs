//! Reduction from NAESatisfiability to ILP (Integer Linear Programming).
//!
//! Binary variable x_i per Boolean variable. For each clause with literals
//! l_1, ..., l_k (using substitution: positive literal contributes +x_i,
//! negative literal contributes -x_i with rhs adjusted by -1 per negative):
//! - At least one true:  Σ coeff_i * x_i ≥ 1 - neg_count
//! - At least one false: Σ coeff_i * x_i ≤ |C| - 1 - neg_count
//!
//! Objective: empty (feasibility problem).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::formula::NAESatisfiability;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionNAESATToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionNAESATToILP {
    type Source = NAESatisfiability;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vars",
        num_constraints = "2 * num_clauses",
    }
)]
impl ReduceTo<ILP<bool>> for NAESatisfiability {
    type Result = ReductionNAESATToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vars();
        let mut constraints = Vec::new();

        for clause in self.clauses() {
            let clause_size = clause.len();
            let mut terms: Vec<(usize, f64)> = Vec::with_capacity(clause_size);
            let mut neg_count: f64 = 0.0;

            for &lit in &clause.literals {
                // Variables are 1-indexed in CNFClause literals.
                let var_idx = lit.unsigned_abs() as usize - 1;
                if lit > 0 {
                    // Positive literal x_i: coefficient +1
                    terms.push((var_idx, 1.0));
                } else {
                    // Negative literal ¬x_i: substitute (1 - x_i), so coefficient -1
                    // and adjust rhs by -1 (accumulated in neg_count).
                    terms.push((var_idx, -1.0));
                    neg_count += 1.0;
                }
            }

            // At least one literal is true: Σ coeff_i * x_i ≥ 1 - neg_count
            constraints.push(LinearConstraint::ge(
                terms.clone(),
                1.0 - neg_count,
            ));

            // At least one literal is false: Σ coeff_i * x_i ≤ |C| - 1 - neg_count
            constraints.push(LinearConstraint::le(
                terms,
                clause_size as f64 - 1.0 - neg_count,
            ));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionNAESATToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "naesatisfiability_to_ilp",
        build: || {
            // NAE-SAT instance: (x1 ∨ x2 ∨ x3) ∧ (¬x1 ∨ ¬x2 ∨ x3)
            // Solution x1=T, x2=F, x3=F: clause1 T,F,F (NAE ✓); clause2 F,T,F (NAE ✓)
            let source = NAESatisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 3]),    // x1 ∨ x2 ∨ x3
                    CNFClause::new(vec![-1, -2, 3]),  // ¬x1 ∨ ¬x2 ∨ x3
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 0],
                    target_config: vec![1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_ilp.rs"]
mod tests;
