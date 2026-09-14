//! Reduction from KSatisfiability (3-SAT) to Decision<MinimumVertexCover>.
//!
//! Classical Garey & Johnson reduction (Theorem 3.3). For each variable u_i,
//! add two vertices {u_i, not-u_i} connected by a truth-setting edge. For each
//! clause c_j, add 3 vertices forming a satisfaction-testing triangle. For each
//! literal l_k in clause c_j, add a communication edge from the triangle vertex
//! j_k to the literal vertex l_k.
//!
//! The resulting graph has a vertex cover of size n + 2m if and only if the
//! 3-SAT formula is satisfiable (n = num_vars, m = num_clauses).
//!
//! Reference: Garey & Johnson, "Computers and Intractability", 1979, Theorem 3.3

use crate::models::decision::Decision;
use crate::models::formula::KSatisfiability;
use crate::models::graph::MinimumVertexCover;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::solvers::SolveOutcome;
use crate::topology::SimpleGraph;
use crate::variant::K3;

/// Result of reducing KSatisfiability<K3> to Decision<MinimumVertexCover>.
#[derive(Debug, Clone)]
pub struct Reduction3SATToDecisionMVC {
    target: Decision<MinimumVertexCover<SimpleGraph, i64>>,
    source_num_vars: usize,
}

impl ReductionResult for Reduction3SATToDecisionMVC {
    type Source = KSatisfiability<K3>;
    type Target = Decision<MinimumVertexCover<SimpleGraph, i64>>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a SAT assignment from a vertex cover solution.
    ///
    /// Vertex layout: indices 0..2n are literal vertices (even = positive,
    /// odd = negated). For variable i, vertex 2*i is u_i and vertex 2*i+1
    /// is not-u_i. A cover meeting the n + 2m bound contains exactly one of these two
    /// for each variable. If u_i is in the cover, set x_i = 1;
    /// if not-u_i is in the cover, set x_i = 0.
    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        match target {
            SolveOutcome::Infeasible => Ok(SolveOutcome::Infeasible),
            SolveOutcome::Optimal { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::optimal(source, solution)?)
            }
            SolveOutcome::Feasible { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::feasible(source, solution)?)
            }
        }
    }
}

impl Reduction3SATToDecisionMVC {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        Ok((0..self.source_num_vars)
            .map(|i| target_solution[2 * i])
            .collect())
    }
}

#[reduction(
    transform = exact {
        num_vertices = "2 * num_vars + 3 * num_clauses",
        num_edges = "num_vars + 6 * num_clauses",
    }
)]
impl ReduceTo<Decision<MinimumVertexCover<SimpleGraph, i64>>> for KSatisfiability<K3> {
    type Result = Reduction3SATToDecisionMVC;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vars();
        let m = self.num_clauses();
        let target_bound = m
            .checked_mul(2)
            .and_then(|value| value.checked_add(n))
            .and_then(|value| i64::try_from(value).ok())
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    KSatisfiability<K3>,
                    Decision<MinimumVertexCover<SimpleGraph, i64>>,
                >("computing the target cover bound")
            })?;
        let total_vertices = 2 * n + 3 * m;
        let mut edges: Vec<(usize, usize)> = Vec::with_capacity(n + 6 * m);

        // Step 1: Truth-setting components.
        // For each variable i, add edge (2*i, 2*i+1) connecting u_i and not-u_i.
        for i in 0..n {
            edges.push((2 * i, 2 * i + 1));
        }

        // Step 2: Satisfaction-testing components (triangles) and communication edges.
        // For each clause j, triangle vertices are at indices 2*n + 3*j, 2*n + 3*j + 1, 2*n + 3*j + 2.
        for (j, clause) in self.clauses().iter().enumerate() {
            let base = 2 * n + 3 * j;

            // Triangle edges within clause j
            edges.push((base, base + 1));
            edges.push((base + 1, base + 2));
            edges.push((base, base + 2));

            // Communication edges: connect triangle vertex k to the literal vertex
            for (k, &lit) in clause.literals.iter().enumerate() {
                let var_idx = lit.unsigned_abs() as usize - 1; // 0-indexed variable
                let literal_vertex = if lit > 0 {
                    2 * var_idx // positive literal vertex
                } else {
                    2 * var_idx + 1 // negated literal vertex
                };
                edges.push((base + k, literal_vertex));
            }
        }

        let graph = SimpleGraph::new(total_vertices, edges);
        let weights = vec![1i64; total_vertices];
        let target = MinimumVertexCover::new(graph, weights);

        Ok(Reduction3SATToDecisionMVC {
            target: Decision::new(target, target_bound),
            source_num_vars: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_decisionminimumvertexcover",
        build: || {
            let source = KSatisfiability::<K3>::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 3]),
                    CNFClause::new(vec![-1, -2, 3]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<
                _,
                Decision<MinimumVertexCover<SimpleGraph, i64>>,
            >(
                source,
                SolutionPair {
                    // x1=0, x2=0, x3=1 satisfies both clauses
                    source_config: serde_json::json!(vec![false, false, true]),
                    // Literal vertices: u1(0), ~u1(1), u2(2), ~u2(3), u3(4), ~u3(5)
                    // Clause 0 triangle: v6, v7, v8 (literals x1, x2, x3)
                    // Clause 1 triangle: v9, v10, v11 (literals ~x1, ~x2, x3)
                    // VC: from truth-setting, pick ~u1(1), ~u2(3), u3(4)
                    // Clause 0: u1,u2 not in cover -> pick v6,v7; u3 in cover -> v8 free
                    // Clause 1: ~u1,~u2,u3 all in cover -> pick any 2: v9,v10
                    // Total cover size = 3 + 2 + 2 = 7 = n + 2m
                    target_config: serde_json::json!(vec![
                        false, true, false, true, true, false, true, true, false, true, true, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_decisionminimumvertexcover.rs"]
mod tests;
