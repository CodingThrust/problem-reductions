//! Karp's literal-compatibility construction with a universal vertex.
//!
//! For m clauses, create one vertex per literal occurrence plus an anchor
//! (3m+1 vertices) and request a clique of size m+1. The anchor is adjacent to
//! every literal vertex, so the empty formula maps to the one-vertex clique.

use crate::models::formula::KSatisfiability;
use crate::models::graph::KClique;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::variant::K3;

/// Literal vertices carry their validated source variable index and polarity.
#[derive(Debug, Clone)]
pub struct Reduction3SATToKClique {
    target: KClique<SimpleGraph>,
    literal_assignments: Vec<(usize, bool)>,
    source_num_vars: usize,
}

impl ReductionResult for Reduction3SATToKClique {
    type Source = KSatisfiability<K3>;
    type Target = KClique<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0,
            "target selection is not a clique meeting the threshold",
        )?;
        // Variables absent from the selected literals are free; choose false.
        let mut assignment = vec![false; self.source_num_vars];
        for (&selected, &(variable, positive)) in target_solution[..self.literal_assignments.len()]
            .iter()
            .zip(&self.literal_assignments)
        {
            if selected {
                assignment[variable] = positive;
            }
        }
        Ok(assignment)
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for Reduction3SATToKClique {}

#[reduction(
    transform = {
        exact {
            k = "num_clauses + 1",
            num_vertices = "3 * num_clauses + 1",
        },
        upper_bound { num_edges = "9 * num_clauses^2 + 3 * num_clauses" },
    }
)]
impl ReduceTo<KClique<SimpleGraph>> for KSatisfiability<K3> {
    type Result = Reduction3SATToKClique;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let (num_vertices, k) = clique_sizes(self.num_clauses())?;
        // CNFClause::variables performs the formal literal-to-index conversion;
        // the source constructor has already validated its range and polarity.
        let positions: Vec<_> = self
            .clauses()
            .iter()
            .enumerate()
            .flat_map(|(clause, c)| {
                c.variables()
                    .into_iter()
                    .zip(&c.literals)
                    .map(move |(var, &lit)| (clause, var, lit > 0))
            })
            .collect();
        let mut edges = Vec::new();
        for (u, &(cu, vu, pu)) in positions.iter().enumerate() {
            for (v, &(cv, vv, pv)) in positions.iter().enumerate().skip(u + 1) {
                if cu != cv && (vu != vv || pu == pv) {
                    edges.push((u, v));
                }
            }
        }
        let anchor = positions.len();
        edges.extend((0..anchor).map(|v| (v, anchor)));
        let target = KClique::new(SimpleGraph::new(num_vertices, edges), k);
        Ok(Reduction3SATToKClique {
            target,
            literal_assignments: positions.into_iter().map(|(_, v, p)| (v, p)).collect(),
            source_num_vars: self.num_vars(),
        })
    }
}

/// Check the vertex and clique sizes before allocating the compatibility graph.
fn clique_sizes(m: usize) -> Result<(usize, usize), crate::rules::ReductionError> {
    let vertices = m
        .checked_mul(3)
        .and_then(|t| t.checked_add(1))
        .ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                KSatisfiability<K3>,
                KClique<SimpleGraph>,
            >("counting SAT clique literal and anchor vertices")
        })?;
    // m+1 <= 3m+1, whose representability was checked above.
    Ok((vertices, m + 1))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_kclique",
        build: || {
            // (x1 ∨ x2 ∨ x3) ∧ (¬x1 ∨ ¬x2 ∨ x3), n=3, m=2
            let source = KSatisfiability::<K3>::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 3]),
                    CNFClause::new(vec![-1, -2, 3]),
                ],
            );
            // x1=F, x2=F, x3=T satisfies both clauses.
            // Clause 0: pick literal x3 (position 2) → vertex 2
            // Clause 1: pick literal ¬x1 (position 0) → vertex 3
            // Select literal vertices 2 and 3 and the universal vertex 6.
            crate::example_db::specs::rule_example_with_witness::<_, KClique<SimpleGraph>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![false, false, true]),
                    target_config: serde_json::json!(vec![
                        false, false, true, true, false, false, true
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_kclique.rs"]
mod tests;
