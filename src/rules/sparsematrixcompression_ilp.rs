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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::one_hot_decode_rows(
            target_solution,
            self.num_rows,
            self.bound_k,
            0,
        )
    }
}

#[reduction(
    transform = upper_bound {
        num_vars = "num_rows * bound_k",
        num_constraints = "num_rows + num_rows * num_rows * bound_k * bound_k",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for SparseMatrixCompression {
    type Result = ReductionSMCToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
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
            let terms: Vec<(usize, i64)> = (0..k).map(|g| (r * k + g, 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
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
                                vec![(r * k + g, 1), (s * k + h, 1)],
                                1,
                            ));
                        }
                    }
                }
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionSMCToILP {
            target,
            num_rows: m,
            bound_k: k,
        })
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
            let reduction: ReductionSMCToILP =
                ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
            let ilp_solver = crate::solvers::ILPSolver::new();
            let target_config = ilp_solver
                .solve(reduction.target_problem())
                .expect("ILP should be solvable");
            let extracted = reduction.extract_solution(&target_config).unwrap();
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(extracted),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sparsematrixcompression_ilp.rs"]
mod tests;
