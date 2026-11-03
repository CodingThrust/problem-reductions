//! Absolute position encoding of Hamiltonian circuits as CNF satisfiability.

use crate::models::formula::{CNFClause, Satisfiability};
use crate::models::graph::HamiltonianCircuit;
use crate::rules::ReductionError;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// A vertex-major position matrix and its source vertex count.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToSatisfiability {
    target: Satisfiability,
    num_vertices: usize,
}

impl ReductionResult for ReductionHamiltonianCircuitToSatisfiability {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Satisfiability {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<bool>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            solution,
            |value| value.0,
            "target assignment does not satisfy the circuit encoding",
        )?;
        let n = self.num_vertices;
        Ok((0..n)
            .flat_map(|p| (0..n).filter(move |&v| solution[v * n + p]))
            .collect())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionHamiltonianCircuitToSatisfiability {}

fn checked_num_vars(n: usize) -> Result<usize, ReductionError> {
    let overflow = || {
        ReductionError::integer_overflow::<HamiltonianCircuit<SimpleGraph>, Satisfiability>(
            "sizing the position encoding",
        )
    };
    let square = n.checked_mul(n).ok_or_else(overflow)?;
    // Four times n³ bounds every clause and literal count in the main branch.
    square
        .checked_mul(n)
        .and_then(|cube| cube.checked_mul(4))
        .ok_or_else(overflow)?;
    <HamiltonianCircuit<SimpleGraph> as ReduceTo<Satisfiability>>::exact_i64(
        square,
        "numbering SAT variables",
    )?;
    Ok(square)
}

fn literal_index(n: usize, v: usize, p: usize) -> Result<i64, ReductionError> {
    let index = v
        .checked_mul(n)
        .and_then(|offset| offset.checked_add(p))
        .and_then(|offset| offset.checked_add(1))
        .ok_or_else(|| {
            ReductionError::integer_overflow::<HamiltonianCircuit<SimpleGraph>, Satisfiability>(
                "indexing a position literal",
            )
        })?;
    <HamiltonianCircuit<SimpleGraph> as ReduceTo<Satisfiability>>::exact_i64(
        index,
        "representing a position literal",
    )
}

#[crate::reduction(transform = upper_bound {
    num_vars = "num_vertices * num_vertices + 1",
    num_clauses = "2 * num_vertices + num_vertices * num_vertices * (num_vertices - 1) + num_vertices^3 + 2",
    num_literals = "4 * num_vertices^3 + 2",
})]
impl ReduceTo<Satisfiability> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToSatisfiability;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let n = self.num_vertices();
        let (num_vars, clauses) = if n < 3 {
            (1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])])
        } else {
            let num_vars = checked_num_vars(n)?;
            let mut adjacent = vec![false; num_vars];
            for (u, v) in self.graph().edges() {
                adjacent[u * n + v] = true;
                adjacent[v * n + u] = true;
            }
            let lit = |v, p| literal_index(n, v, p);
            let mut clauses = Vec::new();
            for p in 0..n {
                clauses.push(CNFClause::new(
                    (0..n).map(|v| lit(v, p)).collect::<Result<_, _>>()?,
                ));
                for u in 0..n {
                    for v in u + 1..n {
                        clauses.push(CNFClause::new(vec![-lit(u, p)?, -lit(v, p)?]));
                    }
                }
            }
            for v in 0..n {
                clauses.push(CNFClause::new(
                    (0..n).map(|p| lit(v, p)).collect::<Result<_, _>>()?,
                ));
                for p in 0..n {
                    for q in p + 1..n {
                        clauses.push(CNFClause::new(vec![-lit(v, p)?, -lit(v, q)?]));
                    }
                }
            }
            for p in 0..n {
                let q = (p + 1) % n;
                for u in 0..n {
                    for v in 0..n {
                        if u == v || !adjacent[u * n + v] {
                            clauses.push(CNFClause::new(vec![-lit(u, p)?, -lit(v, q)?]));
                        }
                    }
                }
            }
            (num_vars, clauses)
        };
        let target = Satisfiability::try_new(num_vars, clauses)
            .map_err(<Self as ReduceTo<Satisfiability>>::target_construction)?;
        Ok(ReductionHamiltonianCircuitToSatisfiability {
            target,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_satisfiability",
        build: || {
            let source = HamiltonianCircuit::new(SimpleGraph::new(
                5,
                vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (0, 2)],
            ));
            let n = source.num_vertices();
            let order: Vec<_> = (0..n).collect();
            let assignment: Vec<_> = (0..n).flat_map(|v| (0..n).map(move |p| v == p)).collect();
            crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(order),
                    target_config: serde_json::json!(assignment),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_satisfiability.rs"]
mod tests;
