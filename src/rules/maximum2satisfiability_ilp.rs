//! Reduction from Maximum 2-Satisfiability (MAX-2-SAT) to ILP.
//!
//! The standard MAX-2-SAT formulation maps directly to a binary ILP:
//! - Variables: one binary variable per Boolean variable (truth assignment)
//!   plus one binary indicator per clause (satisfaction indicator)
//! - Constraints: for each clause, the indicator is at most the sum of its
//!   literal expressions, ensuring z_j = 1 only if the clause is satisfied
//! - Objective: maximize the sum of clause indicators

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::formula::Maximum2Satisfiability;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Maximum2Satisfiability to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMaximum2SatisfiabilityToILP {
    target: ILP<bool>,
    num_vars: usize,
}

impl ReductionResult for ReductionMaximum2SatisfiabilityToILP {
    type Source = Maximum2Satisfiability;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_vars].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vars + num_clauses",
        num_constraints = "num_clauses",
    }
)]
impl ReduceTo<ILP<bool>> for Maximum2Satisfiability {
    type Result = ReductionMaximum2SatisfiabilityToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let m = self.num_clauses();
        let num_ilp_vars = n + m;

        // Build one constraint per clause:
        // For clause j with literals l_1, l_2:
        //   z_{n+j} <= l_1' + l_2'
        // where l_i' = y_{var-1} if positive, or (1 - y_{var-1}) if negative.
        //
        // Rearranged: z_{n+j} - sum(y_i for positive lit i) + sum(y_i for negative lit i) <= k
        // where k = number of negated literals in the clause.
        let constraints: Vec<LinearConstraint> = self
            .clauses()
            .iter()
            .enumerate()
            .map(|(j, clause)| {
                let mut terms: Vec<(usize, f64)> = Vec::new();
                let mut neg_count = 0i32;

                // z_{n+j} has coefficient +1
                terms.push((n + j, 1.0));

                for &lit in &clause.literals {
                    let var_idx = lit.unsigned_abs() as usize - 1;
                    if lit > 0 {
                        // positive literal: subtract y_i
                        terms.push((var_idx, -1.0));
                    } else {
                        // negative literal: add y_i
                        terms.push((var_idx, 1.0));
                        neg_count += 1;
                    }
                }

                LinearConstraint::le(terms, neg_count as f64)
            })
            .collect();

        // Objective: maximize sum of z_j indicators
        let objective: Vec<(usize, f64)> = (0..m).map(|j| (n + j, 1.0)).collect();

        let target = ILP::new(
            num_ilp_vars,
            constraints,
            objective,
            ObjectiveSense::Maximize,
        );

        ReductionMaximum2SatisfiabilityToILP {
            target,
            num_vars: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximum2satisfiability_to_ilp",
        build: || {
            let source = Maximum2Satisfiability::new(
                4,
                vec![
                    CNFClause::new(vec![1, 2]),
                    CNFClause::new(vec![1, -2]),
                    CNFClause::new(vec![-1, 3]),
                    CNFClause::new(vec![-1, -3]),
                    CNFClause::new(vec![2, 4]),
                    CNFClause::new(vec![-3, -4]),
                    CNFClause::new(vec![3, 4]),
                ],
            );
            // Optimal source config: [1,1,0,1] satisfies 6 of 7 clauses.
            // ILP target config: first 4 are truth vars, next 7 are clause indicators.
            // Clause satisfaction with [1,1,0,1] (x1=T, x2=T, x3=F, x4=T):
            //   C0: (x1 OR x2)     = T  -> z4=1
            //   C1: (x1 OR ~x2)    = T  -> z5=1
            //   C2: (~x1 OR x3)    = F  -> z6=0
            //   C3: (~x1 OR ~x3)   = T  -> z7=1
            //   C4: (x2 OR x4)     = T  -> z8=1
            //   C5: (~x3 OR ~x4)   = T  -> z9=1
            //   C6: (x3 OR x4)     = T  -> z10=1
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 0, 1],
                    target_config: vec![1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximum2satisfiability_ilp.rs"]
mod tests;
