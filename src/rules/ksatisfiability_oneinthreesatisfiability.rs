//! Reduction from KSatisfiability (3-SAT) to One-In-Three Satisfiability.

use crate::models::formula::{CNFClause, KSatisfiability, OneInThreeSatisfiability};
use crate::reduction;
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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.source_num_vars].to_vec()
    }
}

#[reduction(overhead = {
    num_vars = "num_vars + 2 + 6 * num_clauses",
    num_clauses = "1 + 5 * num_clauses",
})]
impl ReduceTo<OneInThreeSatisfiability> for KSatisfiability<K3> {
    type Result = Reduction3SATToOneInThreeSAT;

    fn reduce_to(&self) -> Self::Result {
        let source_num_vars = self.num_vars();
        let z_false = source_num_vars as i32 + 1;
        let z_true = source_num_vars as i32 + 2;
        let mut next_var = source_num_vars as i32 + 3;

        let mut clauses = Vec::with_capacity(1 + 5 * self.num_clauses());
        clauses.push(CNFClause::new(vec![z_false, z_false, z_true]));

        for clause in self.clauses() {
            let [l1, l2, l3] = clause.literals.as_slice() else {
                unreachable!("K3 clauses must have exactly three literals");
            };
            let a = next_var;
            let b = next_var + 1;
            let c = next_var + 2;
            let d = next_var + 3;
            let e = next_var + 4;
            let f = next_var + 5;
            next_var += 6;

            clauses.push(CNFClause::new(vec![*l1, a, d]));
            clauses.push(CNFClause::new(vec![*l2, b, d]));
            clauses.push(CNFClause::new(vec![a, b, e]));
            clauses.push(CNFClause::new(vec![c, d, f]));
            clauses.push(CNFClause::new(vec![*l3, c, z_false]));
        }

        let target = OneInThreeSatisfiability::new((next_var - 1) as usize, clauses);

        Reduction3SATToOneInThreeSAT {
            source_num_vars,
            target,
        }
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
