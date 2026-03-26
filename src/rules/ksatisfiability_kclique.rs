//! Reduction from KSatisfiability (3-SAT) to KClique.
//!
//! Classical Karp (1972) reduction. Given a 3-SAT formula with `m` clauses over
//! `n` variables, construct a graph with `3m` vertices — one per literal position
//! in each clause. Connect two vertices `(j1, p1)` and `(j2, p2)` if and only if
//! they belong to different clauses (`j1 ≠ j2`) and their literals are not
//! contradictory (not `x_i` and `¬x_i`). Set `k = m`.
//!
//! A satisfying assignment selects one true literal per clause, forming a clique
//! of size `m`. Conversely, any `m`-clique picks one non-contradictory literal
//! per clause, which can be extended to a satisfying assignment.
//!
//! Reference: Karp, "Reducibility among combinatorial problems", 1972.

use crate::models::formula::KSatisfiability;
use crate::models::graph::KClique;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::variant::K3;

/// Result of reducing KSatisfiability<K3> to KClique<SimpleGraph>.
#[derive(Debug, Clone)]
pub struct Reduction3SATToKClique {
    target: KClique<SimpleGraph>,
    /// Clauses from the source problem, needed for solution extraction.
    source_clauses: Vec<Vec<i32>>,
    source_num_vars: usize,
}

impl ReductionResult for Reduction3SATToKClique {
    type Source = KSatisfiability<K3>;
    type Target = KClique<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.source_num_vars;
        // Start with all variables unset (false = 0).
        let mut assignment = vec![0usize; n];
        // Track which variables have been explicitly set by a clique vertex.
        let mut set = vec![false; n];

        for (v, &val) in target_solution.iter().enumerate() {
            if val != 1 {
                continue;
            }
            // Vertex v corresponds to clause j, position p.
            let j = v / 3;
            let p = v % 3;
            let lit = self.source_clauses[j][p];
            let var_idx = (lit.unsigned_abs() as usize) - 1; // 0-indexed
            if !set[var_idx] {
                assignment[var_idx] = if lit > 0 { 1 } else { 0 };
                set[var_idx] = true;
            }
        }
        assignment
    }
}

/// Check whether two literals are contradictory (one is the negation of the other).
fn literals_contradict(lit1: i32, lit2: i32) -> bool {
    lit1 == -lit2
}

#[reduction(
    overhead = {
        num_vertices = "3 * num_clauses",
        num_edges = "9 * num_clauses * (num_clauses - 1) / 2",
        k = "num_clauses",
    }
)]
impl ReduceTo<KClique<SimpleGraph>> for KSatisfiability<K3> {
    type Result = Reduction3SATToKClique;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_clauses();
        let num_verts = 3 * m;

        // Collect literals for each clause for easy access.
        let clause_lits: Vec<Vec<i32>> =
            self.clauses().iter().map(|c| c.literals.clone()).collect();

        // Build edges: connect (j1,p1) and (j2,p2) if j1 != j2 and literals
        // are not contradictory.
        let mut edges = Vec::new();
        for j1 in 0..m {
            for j2 in (j1 + 1)..m {
                for p1 in 0..3 {
                    for p2 in 0..3 {
                        let lit1 = clause_lits[j1][p1];
                        let lit2 = clause_lits[j2][p2];
                        if !literals_contradict(lit1, lit2) {
                            let v1 = 3 * j1 + p1;
                            let v2 = 3 * j2 + p2;
                            edges.push((v1, v2));
                        }
                    }
                }
            }
        }

        let graph = SimpleGraph::new(num_verts, edges);
        let target = KClique::new(graph, m);

        Reduction3SATToKClique {
            target,
            source_clauses: clause_lits,
            source_num_vars: self.num_vars(),
        }
    }
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
            // Target config: 6 vertices, vertices 2 and 3 selected.
            crate::example_db::specs::rule_example_with_witness::<_, KClique<SimpleGraph>>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 1],
                    target_config: vec![0, 0, 1, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_kclique.rs"]
mod tests;
