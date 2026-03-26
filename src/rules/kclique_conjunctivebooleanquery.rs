//! Reduction from KClique to ConjunctiveBooleanQuery.
//!
//! Given a KClique instance (G=(V,E), k):
//! 1. Domain D = V (n elements, indexed 0..n-1).
//! 2. Single binary relation R with tuples: for each edge {u,v}∈E, include both
//!    (u,v) and (v,u). So 2|E| tuples, each of arity 2.
//! 3. Query: k existential variables, with conjuncts R(y_i, y_j) for all 0≤i<j<k.
//!    That's k*(k-1)/2 conjuncts.
//! 4. G has a k-clique if and only if the query evaluates to true.

use crate::models::graph::KClique;
use crate::models::misc::{CbqRelation, ConjunctiveBooleanQuery, QueryArg};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing KClique to ConjunctiveBooleanQuery.
#[derive(Debug, Clone)]
pub struct ReductionKCliqueToCBQ {
    target: ConjunctiveBooleanQuery,
    num_vertices: usize,
}

impl ReductionResult for ReductionKCliqueToCBQ {
    type Source = KClique<SimpleGraph>;
    type Target = ConjunctiveBooleanQuery;

    fn target_problem(&self) -> &ConjunctiveBooleanQuery {
        &self.target
    }

    /// Extract solution from CBQ back to KClique.
    ///
    /// CBQ config: vec of length k, each value is a domain element (vertex index).
    /// KClique config: binary vec of length n; set config[v]=1 for each v in
    /// the CBQ assignment.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        KClique::<SimpleGraph>::config_from_vertices(self.num_vertices, target_solution)
    }
}

#[reduction(
    overhead = {
        domain_size = "num_vertices",
        num_relations = "1",
        num_variables = "k",
        num_conjuncts = "k * (k - 1) / 2",
    }
)]
impl ReduceTo<ConjunctiveBooleanQuery> for KClique<SimpleGraph> {
    type Result = ReductionKCliqueToCBQ;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let k = self.k();

        // Build the single binary relation: for each edge {u,v}, include (u,v) and (v,u).
        let mut tuples = Vec::with_capacity(self.num_edges() * 2);
        for (u, v) in self.graph().edges() {
            tuples.push(vec![u, v]);
            tuples.push(vec![v, u]);
        }
        let relation = CbqRelation { arity: 2, tuples };

        // Build conjuncts: R(y_i, y_j) for all 0 <= i < j < k.
        let mut conjuncts = Vec::with_capacity(k * k.saturating_sub(1) / 2);
        for i in 0..k {
            for j in (i + 1)..k {
                conjuncts.push((0, vec![QueryArg::Variable(i), QueryArg::Variable(j)]));
            }
        }

        let target = ConjunctiveBooleanQuery::new(n, vec![relation], k, conjuncts);

        ReductionKCliqueToCBQ {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kclique_to_conjunctivebooleanquery",
        build: || {
            // Triangle (0,1,2) in a 5-vertex graph, k=3 → has 3-clique
            let source = KClique::new(
                SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 2), (2, 3), (3, 4)]),
                3,
            );
            crate::example_db::specs::rule_example_with_witness::<_, ConjunctiveBooleanQuery>(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 1, 0, 0],
                    target_config: vec![0, 1, 2],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kclique_conjunctivebooleanquery.rs"]
mod tests;
