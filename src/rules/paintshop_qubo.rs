//! Reduction from PaintShop to QUBO.
//!
//! One binary QUBO variable per car. Variable x_i = 0 means car i gets color A
//! at its first occurrence and color B at its second; x_i = 1 reverses.
//! Adjacent pairs in the sequence contribute to the Q matrix based on their
//! parity (first/second occurrence).
//!
//! Reference: Streif et al., 2021, Physical Review A 104, 012403.

use crate::models::algebraic::QUBO;
use crate::models::misc::PaintShop;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing PaintShop to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionPaintShopToQUBO {
    target: QUBO<i64>,
}

impl ReductionResult for ReductionPaintShopToQUBO {
    type Source = PaintShop;
    type Target = QUBO<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// The QUBO solution maps directly back: car i's first occurrence gets
    /// color x_i, second gets 1 - x_i.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.to_vec())
    }
}

#[reduction(transform = exact {
    num_vars = "num_cars",
})]
impl ReduceTo<QUBO<i64>> for PaintShop {
    type Result = ReductionPaintShopToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_cars();
        let seq = self.sequence_indices();
        let is_first = self.is_first();
        let seq_len = seq.len();

        let mut matrix = vec![vec![0i64; n]; n];
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<PaintShop, QUBO<i64>>(operation)
        };

        // For each adjacent pair in the sequence
        for pos in 0..seq_len.saturating_sub(1) {
            let a = seq[pos];
            let b = seq[pos + 1];

            // Skip if same car (always a color change, constant term)
            if a == b {
                continue;
            }

            let parity_a = is_first[pos];
            let parity_b = is_first[pos + 1];

            // Ensure we write to upper triangular: smaller index first
            let (lo, hi) = if a < b { (a, b) } else { (b, a) };

            if parity_a == parity_b {
                // Same parity: color change when x_a != x_b
                // Contribution: +1 to Q[a][a], +1 to Q[b][b], -2 to Q[lo][hi]
                matrix[a][a] = matrix[a][a]
                    .checked_add(1)
                    .ok_or_else(|| overflow("adding a PaintShop diagonal coefficient"))?;
                matrix[b][b] = matrix[b][b]
                    .checked_add(1)
                    .ok_or_else(|| overflow("adding a PaintShop diagonal coefficient"))?;
                matrix[lo][hi] = matrix[lo][hi]
                    .checked_sub(2)
                    .ok_or_else(|| overflow("adding a PaintShop interaction coefficient"))?;
            } else {
                // Different parity: color change when x_a == x_b
                // Contribution: -1 to Q[a][a], -1 to Q[b][b], +2 to Q[lo][hi]
                matrix[a][a] = matrix[a][a]
                    .checked_sub(1)
                    .ok_or_else(|| overflow("adding a PaintShop diagonal coefficient"))?;
                matrix[b][b] = matrix[b][b]
                    .checked_sub(1)
                    .ok_or_else(|| overflow("adding a PaintShop diagonal coefficient"))?;
                matrix[lo][hi] = matrix[lo][hi]
                    .checked_add(2)
                    .ok_or_else(|| overflow("adding a PaintShop interaction coefficient"))?;
            }
        }

        Ok(ReductionPaintShopToQUBO {
            target: QUBO::from_matrix(matrix).map_err(|message| {
                crate::rules::ReductionError::construction::<PaintShop, QUBO<i64>>(message)
            })?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "paintshop_to_qubo",
        build: || {
            // Issue example: Sequence [A, B, C, A, D, B, D, C], 4 cars
            let source = PaintShop::new(vec!["A", "B", "C", "A", "D", "B", "D", "C"]);
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false, false]),
                    target_config: serde_json::json!(vec![true, false, false, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/paintshop_qubo.rs"]
mod tests;
