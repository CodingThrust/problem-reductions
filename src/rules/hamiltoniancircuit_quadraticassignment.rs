//! Reduction from HamiltonianCircuit to QuadraticAssignment.
//!
//! Uses an exact-decision normalization of the Sahni & Gonzalez (1976)
//! cycle-position construction. The QAP cost counts missing edges in a cyclic
//! permutation, so zero cost certifies a Hamiltonian circuit. Sources with fewer
//! than three vertices map to a fixed positive-cost instance.

use crate::models::algebraic::QuadraticAssignment;
use crate::models::decision::Decision;
use crate::models::graph::HamiltonianCircuit;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::solvers::SolveOutcome;
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to QuadraticAssignment.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToQuadraticAssignment {
    target: Decision<QuadraticAssignment>,
}

impl ReductionResult for ReductionHamiltonianCircuitToQuadraticAssignment {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = Decision<QuadraticAssignment>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

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

impl ReductionHamiltonianCircuitToQuadraticAssignment {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        // Zero cost makes this permutation itself a Hamiltonian circuit.
        Ok(target_solution.to_vec())
    }
}

#[reduction(
    transform = upper_bound {
        num_facilities = "num_vertices + 3",
        num_locations = "num_vertices + 3",
    }
)]
impl ReduceTo<Decision<QuadraticAssignment>> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToQuadraticAssignment;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let source_n = self.num_vertices();
        // The source requires at least three vertices. For smaller graphs use
        // three positions with every distinct pair charged, hence optimum 3.
        let n = source_n.max(3);
        let cost_matrix: Vec<Vec<i64>> = (0..n)
            .map(|i| (0..n).map(|j| i64::from(j == (i + 1) % n)).collect())
            .collect();
        let distance_matrix: Vec<Vec<i64>> = (0..n)
            .map(|k| {
                (0..n)
                    .map(|l| i64::from(k != l && (source_n < 3 || !self.graph().has_edge(k, l))))
                    .collect()
            })
            .collect();

        let target = QuadraticAssignment::new(cost_matrix, distance_matrix);
        Ok(ReductionHamiltonianCircuitToQuadraticAssignment {
            target: Decision::new(target, 0),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_quadraticassignment",
        build: || {
            let source = HamiltonianCircuit::new(SimpleGraph::cycle(4));
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QuadraticAssignment>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![0, 1, 2, 3]),
                    target_config: serde_json::json!(vec![0, 1, 2, 3]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_quadraticassignment.rs"]
mod tests;
