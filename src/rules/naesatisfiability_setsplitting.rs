//! Reduction from NAE-Satisfiability to Set Splitting.
//!
//! Given an NAE-SAT instance with n variables and m clauses, we construct a
//! Set Splitting instance on universe {0, ..., 2n-1} where element 2i
//! represents variable x_{i+1} being true and element 2i+1 represents it
//! being false.
//!
//! The reduction adds two kinds of subsets:
//! - Complementarity subsets {2i, 2i+1} for each variable i, ensuring that
//!   each variable and its complement are assigned different colors.
//! - Clause subsets mapping each NAE clause via literal_to_element, ensuring
//!   the NAE condition (not all equal) maps to the set splitting condition
//!   (non-monochromatic).

use crate::models::formula::NAESatisfiability;
use crate::models::set::SetSplitting;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing NAE-Satisfiability to Set Splitting.
#[derive(Debug, Clone)]
pub struct ReductionNAESATToSetSplitting {
    /// Number of variables in the source problem.
    num_vars: usize,
    /// The target Set Splitting problem.
    target: SetSplitting,
}

impl ReductionResult for ReductionNAESATToSetSplitting {
    type Source = NAESatisfiability;
    type Target = SetSplitting;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Variable i is true iff element 2i is in part 0 (color 0).
        // So assignment[i] = 1 - target_solution[2*i].
        (0..self.num_vars)
            .map(|i| 1 - target_solution[2 * i])
            .collect()
    }
}

/// Convert a 1-indexed signed literal to a 0-indexed universe element.
///
/// Positive literal i -> element 2*(i-1).
/// Negative literal -i -> element 2*(i-1)+1.
fn literal_to_element(lit: i32) -> usize {
    let var_idx = (lit.unsigned_abs() as usize) - 1;
    if lit > 0 {
        2 * var_idx
    } else {
        2 * var_idx + 1
    }
}

#[reduction(overhead = {
    universe_size = "2 * num_vars",
    num_subsets = "num_clauses + num_vars",
})]
impl ReduceTo<SetSplitting> for NAESatisfiability {
    type Result = ReductionNAESATToSetSplitting;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let universe_size = 2 * n;

        let mut subsets = Vec::with_capacity(n + self.num_clauses());

        // Complementarity subsets: {2i, 2i+1} for each variable i
        for i in 0..n {
            subsets.push(vec![2 * i, 2 * i + 1]);
        }

        // Clause subsets: map each literal to its universe element
        for clause in self.clauses() {
            let subset: Vec<usize> = clause
                .literals
                .iter()
                .map(|&lit| literal_to_element(lit))
                .collect();
            subsets.push(subset);
        }

        let target = SetSplitting::new(universe_size, subsets);

        ReductionNAESATToSetSplitting {
            num_vars: n,
            target,
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
            // YES instance from test vectors: n=4,
            // clauses=[[1,2,3],[-1,3,4],[2,-3,-4],[1,-2,4]]
            let source = NAESatisfiability::new(
                4,
                vec![
                    CNFClause::new(vec![1, 2, 3]),
                    CNFClause::new(vec![-1, 3, 4]),
                    CNFClause::new(vec![2, -3, -4]),
                    CNFClause::new(vec![1, -2, 4]),
                ],
            );
            // source_solution: [1,0,1,0] means x1=T, x2=F, x3=T, x4=F
            // target_config: element 2i = 1 - assignment[i]
            //   x1=T -> config[0]=0, config[1]=1
            //   x2=F -> config[2]=1, config[3]=0
            //   x3=T -> config[4]=0, config[5]=1
            //   x4=F -> config[6]=1, config[7]=0
            crate::example_db::specs::rule_example_with_witness::<_, SetSplitting>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 1, 0],
                    target_config: vec![0, 1, 1, 0, 0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_setsplitting.rs"]
mod tests;
