//! Reduction from HamiltonianPath to IsomorphicSpanningTree.
//!
//! A Hamiltonian path is exactly a spanning copy of the path graph `P_n`.

use crate::models::graph::{HamiltonianPath, IsomorphicSpanningTree};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing HamiltonianPath to IsomorphicSpanningTree.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianPathToIsomorphicSpanningTree {
    target: IsomorphicSpanningTree<SimpleGraph>,
}

impl ReductionResult for ReductionHamiltonianPathToIsomorphicSpanningTree {
    type Source = HamiltonianPath<SimpleGraph>;
    type Target = IsomorphicSpanningTree<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<IsomorphicSpanningTree<SimpleGraph>> for HamiltonianPath<SimpleGraph> {
    type Result = ReductionHamiltonianPathToIsomorphicSpanningTree;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let tree = SimpleGraph::new(n, (0..n.saturating_sub(1)).map(|i| (i, i + 1)).collect());
        let target = IsomorphicSpanningTree::new(self.graph().clone(), tree);
        ReductionHamiltonianPathToIsomorphicSpanningTree { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    fn source_example() -> HamiltonianPath<SimpleGraph> {
        HamiltonianPath::new(SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (3, 4),
                (3, 5),
                (4, 2),
                (5, 1),
            ],
        ))
    }

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpath_to_isomorphicspanningtree",
        build: || {
            let source_config = vec![0, 2, 4, 3, 1, 5];
            crate::example_db::specs::rule_example_with_witness::<
                _,
                IsomorphicSpanningTree<SimpleGraph>,
            >(
                source_example(),
                SolutionPair {
                    source_config: source_config.clone(),
                    target_config: source_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpath_isomorphicspanningtree.rs"]
mod tests;
