//! Reduction from Satisfiability (SAT) to MaximumIndependentSet.
//!
//! The reduction creates one vertex for each literal occurrence in each clause.
//! Edges are added:
//! 1. Between all literals within the same clause (forming a clique per clause)
//! 2. Between complementary literals (x and NOT x) across different clauses
//!
//! A satisfying assignment corresponds to an independent set of size = num_clauses,
//! where we pick exactly one literal from each clause.

use crate::models::decision::Decision;
use crate::models::formula::Satisfiability;
use crate::models::graph::MaximumIndependentSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::solvers::SolveOutcome;
use crate::topology::SimpleGraph;
use crate::types::One;

/// A literal in the SAT problem, representing a variable or its negation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolVar {
    /// The variable name/index (0-indexed).
    pub name: usize,
    /// Whether this literal is negated.
    pub neg: bool,
}

impl BoolVar {
    /// Create a new literal.
    pub fn new(name: usize, neg: bool) -> Self {
        Self { name, neg }
    }

    /// Create a literal from a signed integer (1-indexed, as in DIMACS format).
    /// Positive means the variable, negative means its negation.
    pub fn from_literal(lit: i64) -> Self {
        let name = lit.unsigned_abs() as usize - 1; // Convert to 0-indexed
        let neg = lit < 0;
        Self { name, neg }
    }

    /// Check if this literal is the complement of another.
    pub fn is_complement(&self, other: &BoolVar) -> bool {
        self.name == other.name && self.neg != other.neg
    }
}

/// Result of reducing Satisfiability to MaximumIndependentSet.
///
/// This struct contains:
/// - The target MaximumIndependentSet problem
/// - A mapping from vertex indices to literals
/// - The list of source variable indices
/// - The number of clauses in the original SAT problem
#[derive(Debug, Clone)]
pub struct ReductionSATToIS {
    /// The target MaximumIndependentSet problem.
    target: Decision<MaximumIndependentSet<SimpleGraph, One>>,
    /// Mapping from vertex index to the literal it represents.
    literals: Vec<BoolVar>,
    /// The number of variables in the source SAT problem.
    num_source_variables: usize,
    /// The number of clauses in the source SAT problem.
    num_clauses: usize,
}

impl ReductionResult for ReductionSATToIS {
    type Source = Satisfiability;
    type Target = Decision<MaximumIndependentSet<SimpleGraph, One>>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a SAT solution from an MaximumIndependentSet solution.
    ///
    /// For each selected vertex (representing a literal), we set the corresponding
    /// variable to make that literal true. Variables not covered by any selected
    /// literal default to false.
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

impl ReductionSATToIS {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        let mut assignment = vec![false; self.num_source_variables];
        for (literal, &selected) in self.literals.iter().zip(target_solution) {
            if selected {
                assignment[literal.name] = !literal.neg;
            }
        }
        Ok(assignment)
    }
}

impl ReductionSATToIS {
    /// Get the number of clauses in the source SAT problem.
    pub fn num_clauses(&self) -> usize {
        self.num_clauses
    }

    /// Get a reference to the literals mapping.
    pub fn literals(&self) -> &[BoolVar] {
        &self.literals
    }
}

#[reduction(
    transform = upper_bound {
        num_vertices = "num_literals",
        num_edges = "num_literals^2",
    }
)]
impl ReduceTo<Decision<MaximumIndependentSet<SimpleGraph, One>>> for Satisfiability {
    type Result = ReductionSATToIS;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let target_size =
            <Self as ReduceTo<Decision<MaximumIndependentSet<SimpleGraph, One>>>>::exact_i64(
                self.num_clauses(),
                "representing the satisfying independent-set cardinality",
            )?;
        let mut literals: Vec<BoolVar> = Vec::new();
        let mut edges: Vec<(usize, usize)> = Vec::new();

        // First pass: add vertices for each literal in each clause
        // and add clique edges within each clause
        for clause in self.clauses() {
            let clause_start = literals.len();

            // Add vertices for each literal in this clause
            for &lit in &clause.literals {
                literals.push(BoolVar::from_literal(lit));
            }

            let vertex_count = literals.len();
            // Add clique edges within this clause
            for i in clause_start..vertex_count {
                for j in (i + 1)..vertex_count {
                    edges.push((i, j));
                }
            }
        }

        let vertex_count = literals.len();
        // Add complementary-literal edges. Within a clause these may duplicate
        // clique edges, which does not change independent-set feasibility.
        for i in 0..vertex_count {
            for j in (i + 1)..vertex_count {
                if literals[i].is_complement(&literals[j]) {
                    edges.push((i, j));
                }
            }
        }

        let target = MaximumIndependentSet::new(
            SimpleGraph::new(vertex_count, edges),
            vec![One; vertex_count],
        );

        Ok(ReductionSATToIS {
            target: Decision::new(target, target_size),
            literals,
            num_source_variables: self.num_vars(),
            num_clauses: self.num_clauses(),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    fn sat_seven_clause_example() -> Satisfiability {
        Satisfiability::new(
            5,
            vec![
                CNFClause::new(vec![1, 2, -3]),
                CNFClause::new(vec![-1, 3, 4]),
                CNFClause::new(vec![2, -4, 5]),
                CNFClause::new(vec![-2, 3, -5]),
                CNFClause::new(vec![1, -3, 5]),
                CNFClause::new(vec![-1, -2, 4]),
                CNFClause::new(vec![3, -4, -5]),
            ],
        )
    }

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "satisfiability_to_maximumindependentset",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<
                _,
                Decision<MaximumIndependentSet<SimpleGraph, One>>,
            >(
                sat_seven_clause_example(),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, true, true, true, false]),
                    target_config: serde_json::json!(vec![
                        true, false, false, false, true, false, true, false, false, false, false,
                        true, true, false, false, false, false, true, true, false, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sat_maximumindependentset.rs"]
mod tests;
