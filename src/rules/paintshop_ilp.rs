//! Reduction from PaintShop to ILP (Integer Linear Programming).
//!
//! Binary variable x_i per car (first-occurrence color), binary k_p per
//! sequence position (actual color), binary c_p per adjacent pair (switch
//! indicator). Minimize Σ c_p.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::PaintShop;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionPaintShopToILP {
    target: ILP<bool>,
    num_cars: usize,
}

impl ReductionResult for ReductionPaintShopToILP {
    type Source = PaintShop;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract first-occurrence color bits (x_i) from ILP solution.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_cars].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_cars + 2 * num_sequence",
        num_constraints = "num_sequence + 2 * num_sequence",
    }
)]
impl ReduceTo<ILP<bool>> for PaintShop {
    type Result = ReductionPaintShopToILP;

    fn reduce_to(&self) -> Self::Result {
        let nc = self.num_cars();
        let seq_len = self.sequence_len();

        // Variable layout:
        //   x_i: car first-occurrence color, index i for i in 0..nc
        //   k_p: actual color at position p, index nc + p for p in 0..seq_len
        //   c_p: switch indicator at position p, index nc + seq_len + p
        let k_offset = nc;
        let c_offset = nc + seq_len;
        let num_vars = nc + 2 * seq_len;

        let mut constraints = Vec::new();

        // Determine car index and is_first for each position.
        // With config all-zero: first occ gets color 0, second occ gets color 1.
        let base = self.get_coloring(&vec![0; nc]);

        // For each car i, find its positions by flipping x_i.
        for i in 0..nc {
            let mut config = vec![0; nc];
            config[i] = 1;
            let flipped = self.get_coloring(&config);

            for p in 0..seq_len {
                if flipped[p] != base[p] {
                    // Position p belongs to car i
                    if base[p] == 0 {
                        // First occurrence: k_p = x_i
                        constraints.push(LinearConstraint::eq(
                            vec![(k_offset + p, 1.0), (i, -1.0)],
                            0.0,
                        ));
                    } else {
                        // Second occurrence: k_p = 1 - x_i  =>  k_p + x_i = 1
                        constraints.push(LinearConstraint::eq(
                            vec![(k_offset + p, 1.0), (i, 1.0)],
                            1.0,
                        ));
                    }
                }
            }
        }

        // Switch constraints: c_p >= |k_p - k_{p-1}| for p > 0
        for p in 1..seq_len {
            // c_p >= k_p - k_{p-1}
            constraints.push(LinearConstraint::ge(
                vec![
                    (c_offset + p, 1.0),
                    (k_offset + p, -1.0),
                    (k_offset + p - 1, 1.0),
                ],
                0.0,
            ));
            // c_p >= k_{p-1} - k_p
            constraints.push(LinearConstraint::ge(
                vec![
                    (c_offset + p, 1.0),
                    (k_offset + p - 1, -1.0),
                    (k_offset + p, 1.0),
                ],
                0.0,
            ));
        }

        // Objective: minimize Σ c_p for p in 1..seq_len
        let objective: Vec<(usize, f64)> = (1..seq_len).map(|p| (c_offset + p, 1.0)).collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);
        ReductionPaintShopToILP {
            target,
            num_cars: nc,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "paintshop_to_ilp",
        build: || {
            // Sequence: A, B, A, C, B, C => 3 cars
            let source = PaintShop::new(vec!["A", "B", "A", "C", "B", "C"]);
            let reduction: ReductionPaintShopToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let target_config = {
                let ilp_solver = crate::solvers::ILPSolver::new();
                ilp_solver
                    .solve(reduction.target_problem())
                    .expect("ILP should be solvable")
            };
            let source_config = reduction.extract_solution(&target_config);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/paintshop_ilp.rs"]
mod tests;
