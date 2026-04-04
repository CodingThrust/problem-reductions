//! Reduction from NAE-Satisfiability to Set Splitting.
//!
//! Create one universe element for each positive literal `x_i` and one for each
//! negative literal `¬x_i`. Add a 2-element subset `{x_i, ¬x_i}` for every
//! variable to force opposite colors. Each clause becomes a subset of its
//! literal-elements, so non-monochromatic subsets correspond exactly to
//! not-all-equal clauses.

use crate::models::formula::NAESatisfiability;
use crate::models::set::SetSplitting;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionNAESATToSetSplitting {
    target: SetSplitting,
    num_source_variables: usize,
}

impl ReductionResult for ReductionNAESATToSetSplitting {
    type Source = NAESatisfiability;
    type Target = SetSplitting;

    fn target_problem(&self) -> &SetSplitting {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        assert!(
            target_solution.len() >= self.num_source_variables,
            "SetSplitting solution has {} variables but source requires {}",
            target_solution.len(),
            self.num_source_variables,
        );
        target_solution[..self.num_source_variables].to_vec()
    }
}

fn literal_element_index(lit: i32, num_vars: usize) -> usize {
    let var_index = lit.unsigned_abs() as usize - 1;
    if lit > 0 {
        var_index
    } else {
        num_vars + var_index
    }
}

#[reduction(
    overhead = {
        universe_size = "2 * num_vars",
        num_subsets = "num_vars + num_clauses",
    }
)]
impl ReduceTo<SetSplitting> for NAESatisfiability {
    type Result = ReductionNAESATToSetSplitting;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vars();
        let mut subsets = Vec::with_capacity(num_vars + self.num_clauses());

        for var_index in 0..num_vars {
            subsets.push(vec![var_index, num_vars + var_index]);
        }

        for clause in self.clauses() {
            subsets.push(
                clause
                    .literals
                    .iter()
                    .map(|&lit| literal_element_index(lit, num_vars))
                    .collect(),
            );
        }

        ReductionNAESATToSetSplitting {
            target: SetSplitting::new(2 * num_vars, subsets),
            num_source_variables: num_vars,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "naesatisfiability_to_setsplitting",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, SetSplitting>(
                NAESatisfiability::new(
                    3,
                    vec![
                        CNFClause::new(vec![1, -2, 3]),
                        CNFClause::new(vec![-1, 2, -3]),
                    ],
                ),
                SolutionPair {
                    source_config: vec![1, 1, 1],
                    target_config: vec![1, 1, 1, 0, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_setsplitting.rs"]
mod tests;
