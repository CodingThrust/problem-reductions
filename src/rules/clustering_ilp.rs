//! Reduction from Clustering to ILP (Integer Linear Programming).
//!
//! Use one binary assignment variable `x_{i,c}` for each element `i` and
//! cluster `c`. Equality constraints force every element into exactly one
//! cluster, and conflict constraints forbid any pair with distance above the
//! diameter bound from sharing a cluster.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::Clustering;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Clustering to ILP.
#[derive(Debug, Clone)]
pub struct ReductionClusteringToILP {
    target: ILP<bool>,
    num_elements: usize,
    num_clusters: usize,
}

impl ReductionClusteringToILP {
    fn var_index(&self, element: usize, cluster: usize) -> usize {
        element * self.num_clusters + cluster
    }
}

impl ReductionResult for ReductionClusteringToILP {
    type Source = Clustering;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_elements)
            .map(|element| {
                (0..self.num_clusters)
                    .find(|&cluster| {
                        let idx = self.var_index(element, cluster);
                        idx < target_solution.len() && target_solution[idx] == 1
                    })
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_elements * num_clusters",
        num_constraints = "num_elements + num_elements^2 * num_clusters",
    }
)]
impl ReduceTo<ILP<bool>> for Clustering {
    type Result = ReductionClusteringToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_elements = self.num_elements();
        let num_clusters = self.num_clusters();
        let num_vars = num_elements * num_clusters;
        let mut constraints = Vec::new();

        let var_index =
            |element: usize, cluster: usize| -> usize { element * num_clusters + cluster };

        for element in 0..num_elements {
            let terms: Vec<(usize, f64)> = (0..num_clusters)
                .map(|cluster| (var_index(element, cluster), 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        let distances = self.distances();
        let diameter_bound = self.diameter_bound();
        for (i, row) in distances.iter().enumerate() {
            for (j, &distance) in row.iter().enumerate().skip(i + 1) {
                if distance > diameter_bound {
                    for cluster in 0..num_clusters {
                        constraints.push(LinearConstraint::le(
                            vec![(var_index(i, cluster), 1.0), (var_index(j, cluster), 1.0)],
                            1.0,
                        ));
                    }
                }
            }
        }

        ReductionClusteringToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_elements,
            num_clusters,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "clustering_to_ilp",
        build: || {
            let source = Clustering::new(
                vec![
                    vec![0, 1, 3, 3],
                    vec![1, 0, 3, 3],
                    vec![3, 3, 0, 1],
                    vec![3, 3, 1, 0],
                ],
                2,
                1,
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 1, 1],
                    target_config: vec![1, 0, 1, 0, 0, 1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/clustering_ilp.rs"]
mod tests;
