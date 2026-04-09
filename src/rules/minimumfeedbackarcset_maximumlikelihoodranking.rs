//! Reduction from MinimumFeedbackArcSet to MaximumLikelihoodRanking.
//!
//! On unit-weight instances, a ranking induces exactly the feedback arc set of
//! backward arcs. The target matrix uses the skew-symmetric `c = 0` encoding:
//! one-way arcs become `+/-1`, while bidirectional pairs and missing pairs map
//! to `0`.

use crate::models::graph::MinimumFeedbackArcSet;
use crate::models::misc::MaximumLikelihoodRanking;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[allow(clippy::needless_range_loop)]
fn build_skew_symmetric_matrix(problem: &MinimumFeedbackArcSet<i32>) -> Vec<Vec<i32>> {
    let n = problem.num_vertices();
    let graph = problem.graph();
    let mut matrix = vec![vec![0i32; n]; n];

    for i in 0..n {
        for j in (i + 1)..n {
            let ij = graph.has_arc(i, j);
            let ji = graph.has_arc(j, i);
            if ij && !ji {
                matrix[i][j] = 1;
                matrix[j][i] = -1;
            } else if ji && !ij {
                matrix[i][j] = -1;
                matrix[j][i] = 1;
            }
        }
    }

    matrix
}

/// Result of reducing MinimumFeedbackArcSet to MaximumLikelihoodRanking.
#[derive(Debug, Clone)]
pub struct ReductionFASToMLR {
    target: MaximumLikelihoodRanking,
    source_arcs: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionFASToMLR {
    type Source = MinimumFeedbackArcSet<i32>;
    type Target = MaximumLikelihoodRanking;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.source_arcs
            .iter()
            .map(|&(u, v)| usize::from(target_solution[u] > target_solution[v]))
            .collect()
    }
}

#[reduction(
    overhead = {
        num_items = "num_vertices",
    }
)]
impl ReduceTo<MaximumLikelihoodRanking> for MinimumFeedbackArcSet<i32> {
    type Result = ReductionFASToMLR;

    fn reduce_to(&self) -> Self::Result {
        assert!(
            self.weights().iter().all(|&weight| weight == 1),
            "MinimumFeedbackArcSet -> MaximumLikelihoodRanking requires unit arc weights"
        );

        ReductionFASToMLR {
            target: MaximumLikelihoodRanking::new(build_skew_symmetric_matrix(self)),
            source_arcs: self.graph().arcs(),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::solvers::BruteForce;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumfeedbackarcset_to_maximumlikelihoodranking",
        build: || {
            let source = MinimumFeedbackArcSet::new(
                crate::topology::DirectedGraph::new(
                    5,
                    vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 2), (0, 4)],
                ),
                vec![1i32; 7],
            );
            let reduction = ReduceTo::<MaximumLikelihoodRanking>::reduce_to(&source);
            let target_witness = BruteForce::new()
                .find_witness(reduction.target_problem())
                .expect("target should have an optimum");
            let source_witness = reduction.extract_solution(&target_witness);

            crate::example_db::specs::rule_example_with_witness::<_, MaximumLikelihoodRanking>(
                source,
                SolutionPair {
                    source_config: source_witness,
                    target_config: target_witness,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumfeedbackarcset_maximumlikelihoodranking.rs"]
mod tests;
