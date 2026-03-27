//! Reduction from SparseMatrixCompression to ILP.
//!
//! Assign each row one shift value and forbid any pair of shifted 1-entries
//! from colliding in the storage vector.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, SparseMatrixCompression, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSMCToILP {
    target: ILP<bool>,
    num_rows: usize,
    bound_k: usize,
}

impl ReductionResult for ReductionSMCToILP {
    type Source = SparseMatrixCompression;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // For each row r, output the unique zero-based shift g with x_{r,g} = 1
        (0..self.num_rows)
            .map(|r| {
                (0..self.bound_k)
                    .find(|&g| target_solution[r * self.bound_k + g] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_rows * bound_k",
        num_constraints = "num_rows + num_rows * num_rows * bound_k * bound_k",
    }
)]
impl ReduceTo<ILP<bool>> for SparseMatrixCompression {
    type Result = ReductionSMCToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_rows();
        let n = self.num_cols();
        let k = self.bound_k();

        // Variable layout:
        // x_{r,g}: m*K binary variables at [0, m*K)
        //   x_{r*K + g} = 1 iff row r uses shift g (zero-based)
        let num_vars = m * k;
        let mut constraints = Vec::new();

        // Each row assigned exactly one shift
        for r in 0..m {
            let terms: Vec<(usize, f64)> = (0..k).map(|g| (r * k + g, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Collision constraints:
        // x_{r,g} + x_{s,h} <= 1 whenever A_{r,i} = A_{s,j} = 1 and i + g = j + h
        // (for different rows r != s, or same row r = s but different columns i != j)
        for r in 0..m {
            for s in (r + 1)..m {
                for i in 0..n {
                    if !self.matrix()[r][i] {
                        continue;
                    }
                    for j in 0..n {
                        if !self.matrix()[s][j] {
                            continue;
                        }
                        // Collision when i + g = j + h, i.e., g - h = j - i
                        for g in 0..k {
                            // h = g + i - j (must be in [0, k))
                            let gi = g + i;
                            if gi < j {
                                continue;
                            }
                            let h = gi - j;
                            if h >= k {
                                continue;
                            }
                            constraints.push(LinearConstraint::le(
                                vec![(r * k + g, 1.0), (s * k + h, 1.0)],
                                1.0,
                            ));
                        }
                    }
                }
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionSMCToILP {
            target,
            num_rows: m,
            bound_k: k,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sparsematrixcompression_to_ilp",
        build: || {
            let source = SparseMatrixCompression::new(
                vec![
                    vec![true, false, false, true],
                    vec![false, true, false, false],
                    vec![false, false, true, false],
                    vec![true, false, false, false],
                ],
                2,
            );
            let reduction: ReductionSMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let ilp_solver = crate::solvers::ILPSolver::new();
            let target_config = ilp_solver
                .solve(reduction.target_problem())
                .expect("ILP should be solvable");
            let extracted = reduction.extract_solution(&target_config);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: extracted,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sparsematrixcompression_ilp.rs"]
mod tests;
