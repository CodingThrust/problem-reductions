//! Reduction from MinimumFaultDetectionTestSet to ILP.
//!
//! Each input-output pair becomes a binary decision variable. For every
//! internal vertex, the ILP requires at least one selected pair whose coverage
//! set contains that vertex. Minimizing the sum of the pair variables therefore
//! recovers the minimum-size covering test set.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MinimumFaultDetectionTestSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::collections::VecDeque;

/// Result of reducing MinimumFaultDetectionTestSet to `ILP<bool>`.
#[derive(Debug, Clone)]
pub struct ReductionMFDTSToILP {
    target: ILP<bool>,
    num_inputs: usize,
    num_outputs: usize,
}

impl ReductionResult for ReductionMFDTSToILP {
    type Source = MinimumFaultDetectionTestSet;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok((0..self.num_inputs)
            .map(|input| {
                (0..self.num_outputs)
                    .map(|output| target_solution[input * self.num_outputs + output] == 1)
                    .collect()
            })
            .collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_inputs * num_outputs",
        num_constraints = "num_vertices - num_inputs - num_outputs",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumFaultDetectionTestSet {
    type Result = ReductionMFDTSToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        fn reachable(adj: &[Vec<usize>], start: usize) -> Vec<bool> {
            let mut seen = vec![false; adj.len()];
            let mut queue = VecDeque::new();
            seen[start] = true;
            queue.push_back(start);

            while let Some(vertex) = queue.pop_front() {
                for &next in &adj[vertex] {
                    if !seen[next] {
                        seen[next] = true;
                        queue.push_back(next);
                    }
                }
            }

            seen
        }

        let mut adj = vec![Vec::new(); self.num_vertices()];
        let mut rev_adj = vec![Vec::new(); self.num_vertices()];
        for &(tail, head) in self.arcs() {
            adj[tail].push(head);
            rev_adj[head].push(tail);
        }

        let input_reachability: Vec<Vec<bool>> = self
            .inputs()
            .iter()
            .map(|&input| reachable(&adj, input))
            .collect();
        let output_reachability: Vec<Vec<bool>> = self
            .outputs()
            .iter()
            .map(|&output| reachable(&rev_adj, output))
            .collect();

        let mut boundary = vec![false; self.num_vertices()];
        for &input in self.inputs() {
            boundary[input] = true;
        }
        for &output in self.outputs() {
            boundary[output] = true;
        }

        let num_pairs = self.num_inputs() * self.num_outputs();
        let internal_vertices: Vec<usize> = (0..self.num_vertices())
            .filter(|&vertex| !boundary[vertex])
            .collect();

        let constraints: Vec<LinearConstraint> = internal_vertices
            .into_iter()
            .map(|vertex| {
                let mut terms = Vec::new();
                for (input_idx, input_cov) in input_reachability.iter().enumerate() {
                    for (output_idx, output_cov) in output_reachability.iter().enumerate() {
                        if input_cov[vertex] && output_cov[vertex] {
                            let pair_idx = input_idx * self.num_outputs() + output_idx;
                            terms.push((pair_idx, 1));
                        }
                    }
                }
                LinearConstraint::ge(terms, 1)
            })
            .collect();

        let objective = (0..num_pairs).map(|pair_idx| (pair_idx, 1)).collect();

        Ok(ReductionMFDTSToILP {
            target: ILP::new(num_pairs, constraints, objective, ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_inputs: self.num_inputs(),
            num_outputs: self.num_outputs(),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumfaultdetectiontestset_to_ilp",
        build: || {
            let source = MinimumFaultDetectionTestSet::new(
                7,
                vec![
                    (0, 2),
                    (0, 3),
                    (1, 3),
                    (1, 4),
                    (2, 5),
                    (3, 5),
                    (3, 6),
                    (4, 6),
                ],
                vec![0, 1],
                vec![5, 6],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumfaultdetectiontestset_ilp.rs"]
mod tests;
