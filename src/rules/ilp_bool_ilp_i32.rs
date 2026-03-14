//! Natural embedding of binary ILP into general integer ILP.
//!
//! Every binary (0-1) variable is a valid non-negative integer variable.
//! The constraints carry over unchanged. Additional upper-bound constraints
//! (x_i <= 1) are added to preserve binary semantics.
//!
//! This is a same-name variant cast (ILP → ILP), so by convention it does not
//! have an example file or a paper `reduction-rule` entry.

use crate::models::algebraic::{LinearConstraint, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionBinaryILPToIntILP {
    target: ILP<i32>,
}

impl ReductionResult for ReductionBinaryILPToIntILP {
    type Source = ILP<bool>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(id = "ilp_to_ilp_bool_i32", overhead = {
    num_vars = "num_vars",
    num_constraints = "num_constraints + num_vars",
})]
impl ReduceTo<ILP<i32>> for ILP<bool> {
    type Result = ReductionBinaryILPToIntILP;

    fn reduce_to(&self) -> Self::Result {
        let mut constraints = self.constraints.clone();
        // Add x_i <= 1 for each variable to preserve binary domain
        for i in 0..self.num_vars {
            constraints.push(LinearConstraint::le(vec![(i, 1.0)], 1.0));
        }
        ReductionBinaryILPToIntILP {
            target: ILP::<i32>::new(
                self.num_vars,
                constraints,
                self.objective.clone(),
                self.sense,
            ),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_bool_ilp_i32.rs"]
mod tests;
