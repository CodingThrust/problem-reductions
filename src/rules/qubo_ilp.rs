//! Reduction from QUBO to ILP via McCormick linearization.
//!
//! QUBO minimizes x^T Q x where x ∈ {0,1}^n and Q is upper-triangular.
//!
//! ## Linearization
//! - Diagonal: Q_ii · x_i² = Q_ii · x_i (linear for binary x)
//! - Off-diagonal: For each non-zero Q_ij (i < j), introduce y_ij = x_i · x_j
//!   with McCormick constraints: y_ij ≤ x_i, y_ij ≤ x_j, y_ij ≥ x_i + x_j - 1
//!
//! ## Variables
//! - x_i ∈ {0,1} for i = 0..n-1 (original QUBO variables)
//! - y_k ∈ {0,1} for each non-zero off-diagonal Q_ij (auxiliary products)
//!
//! ## Objective
//! minimize Σ_i Q_ii · x_i + Σ_{i<j} Q_ij · y_{ij}

use crate::models::algebraic::{ILPCoefficient, ObjectiveSense, ILP, QUBO};
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing QUBO to ILP.
#[derive(Debug, Clone)]
pub struct ReductionQUBOToILP<C: ILPCoefficient = i64> {
    target: ILP<bool, C>,
    num_original: usize,
}

impl<C> ReductionResult for ReductionQUBOToILP<C>
where
    C: ILPCoefficient + crate::variant::VariantParam,
{
    type Source = QUBO<C>;
    type Target = ILP<bool, C>;

    fn target_problem(&self) -> &ILP<bool, C> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_original]
            .iter()
            .map(|&value| value == 1)
            .collect())
    }
}

fn reduce_qubo<C>(source: &QUBO<C>) -> Result<ReductionQUBOToILP<C>, crate::rules::ReductionError>
where
    C: ILPCoefficient + crate::variant::VariantParam + From<i8>,
{
    let n = source.num_vars();
    let matrix = source.matrix();

    // Collect non-zero off-diagonal entries (i < j)
    let mut off_diag: Vec<(usize, usize, C)> = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, &q_ij) in row.iter().enumerate().skip(i + 1) {
            if q_ij != C::zero() {
                off_diag.push((i, j, q_ij));
            }
        }
    }

    let m = off_diag.len();
    let total_vars = n + m;

    // Objective: minimize Σ Q_ii · x_i + Σ Q_ij · y_k
    let mut objective: Vec<(usize, C)> = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        let q_ii = row[i];
        if q_ii != C::zero() {
            objective.push((i, q_ii));
        }
    }
    for (k, &(_, _, q_ij)) in off_diag.iter().enumerate() {
        objective.push((n + k, q_ij));
    }

    // McCormick constraints: 3 per auxiliary variable
    let mut constraints: Vec<crate::models::algebraic::LinearConstraint<C>> =
        Vec::with_capacity(3 * m);
    for (k, &(i, j, _)) in off_diag.iter().enumerate() {
        let y_k = n + k;
        constraints.extend(mccormick_product(y_k, i, j));
    }

    let target = ILP::new(total_vars, constraints, objective, ObjectiveSense::Minimize)
        .map_err(crate::rules::ReductionError::construction::<QUBO<C>, ILP<bool, C>>)?;
    Ok(ReductionQUBOToILP {
        target,
        num_original: n,
    })
}

macro_rules! impl_qubo_to_ilp {
    ($coefficient:ty) => {
        #[reduction(
            transform = upper_bound {
                num_vars = "num_vars^2 + num_vars",
                num_constraints = "3 * num_vars^2",
            },
            unavailable = {
                num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
            }
        )]
        impl ReduceTo<ILP<bool, $coefficient>> for QUBO<$coefficient> {
            type Result = ReductionQUBOToILP<$coefficient>;

            fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
                reduce_qubo(self)
            }
        }
    }
}

impl_qubo_to_ilp!(i64);
impl_qubo_to_ilp!(f64);

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "qubo_to_ilp",
            build: || {
                let mut matrix = vec![vec![0.0; 4]; 4];
                matrix[0][0] = -2.0;
                matrix[1][1] = -3.0;
                matrix[2][2] = -1.0;
                matrix[3][3] = -4.0;
                matrix[0][1] = 1.0;
                matrix[1][2] = 2.0;
                matrix[2][3] = -1.0;
                let source = QUBO::from_matrix(matrix).unwrap();
                crate::example_db::specs::rule_example_via_float_ilp::<_, bool>(source)
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "integer_qubo_to_ilp",
            build: || {
                let source = QUBO::from_matrix(vec![vec![2, 1], vec![0, -3]]).unwrap();
                crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/qubo_ilp.rs"]
mod tests;
