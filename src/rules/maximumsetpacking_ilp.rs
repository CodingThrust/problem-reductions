//! Reduction from MaximumSetPacking to ILP (Integer Linear Programming).
//!
//! The Set Packing problem can be formulated as a binary ILP:
//! - Variables: One binary variable per set (0 = not selected, 1 = selected)
//! - Constraints: For each element e, Σ_{i : e ∈ S_i} x_i ≤ 1
//! - Objective: Maximize the sum of weights of selected sets

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::MaximumSetPacking;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MaximumSetPacking to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each set corresponds to a binary variable
/// - Element constraints ensure at most one set per element is selected
/// - The objective maximizes the total weight of selected sets
#[derive(Debug, Clone)]
pub struct ReductionSPToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionSPToILP {
    type Source = MaximumSetPacking<i32>;
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
        num_vars = "num_sets",
        num_constraints = "universe_size",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumSetPacking<i32> {
    type Result = ReductionSPToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_sets();

        // Build element-to-sets mapping, then create one constraint per element
        let universe = self.universe_size();
        let mut elem_to_sets: Vec<Vec<usize>> = vec![Vec::new(); universe];
        for (i, set) in self.sets().iter().enumerate() {
            for &e in set {
                elem_to_sets[e].push(i);
            }
        }

        let constraints: Vec<LinearConstraint> = elem_to_sets
            .into_iter()
            .filter(|sets| sets.len() > 1)
            .map(|sets| {
                let terms: Vec<(usize, f64)> = sets.into_iter().map(|i| (i, 1.0)).collect();
                LinearConstraint::le(terms, 1.0)
            })
            .collect();

        let objective: Vec<(usize, f64)> = self
            .weights_ref()
            .iter()
            .enumerate()
            .map(|(i, &w)| (i, w as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionSPToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumsetpacking_to_ilp",
        build: || {
            let source = MaximumSetPacking::new(vec![
                vec![0, 1, 2],
                vec![2, 3, 4],
                vec![4, 5, 6],
                vec![6, 7, 0],
                vec![1, 3, 5],
                vec![0, 4, 7],
            ]);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 0, 1, 1, 0],
                    target_config: vec![0, 0, 0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumsetpacking_ilp.rs"]
mod tests;
