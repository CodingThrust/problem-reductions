//! Reduction from ShortestCommonSupersequence to ILP (Integer Linear Programming).
//!
//! One-hot symbol variables x_{p,a} for each position p and symbol a, plus
//! matching variables m_{s,j,p} indicating that the j-th character of string s
//! is matched to position p. Monotonicity forces strictly increasing match
//! positions per string.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::ShortestCommonSupersequence;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSCSToILP {
    target: ILP<bool>,
    max_length: usize,
    alphabet_size: usize,
}

impl ReductionResult for ReductionSCSToILP {
    type Source = ShortestCommonSupersequence;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// At each position p, output the unique symbol a with x_{p,a} = 1.
    /// Uses alphabet_size + 1 symbols (last = padding).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let b = self.max_length;
        let k = self.alphabet_size + 1; // includes padding symbol
        (0..b)
            .map(|p| {
                (0..k)
                    .find(|&a| target_solution[p * k + a] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "max_length * (alphabet_size + 1) + total_length * max_length",
        num_constraints = "max_length + total_length + total_length * max_length + total_length + max_length",
    }
)]
impl ReduceTo<ILP<bool>> for ShortestCommonSupersequence {
    type Result = ReductionSCSToILP;

    fn reduce_to(&self) -> Self::Result {
        let b = self.max_length();
        let alpha = self.alphabet_size();
        let k = alpha + 1; // alphabet + padding symbol
        let strings = self.strings();
        let pad = alpha; // padding symbol index

        // Variable layout:
        //   x_{p,a}: position p carries symbol a, index p*k + a  for p in 0..b, a in 0..k
        //   m_{s,j,p}: j-th char of string s matched to position p
        //     We flatten (s,j) into a global character index.
        let x_count = b * k;

        // Build global char index: for string s, char j, the global index is sum of lengths before s + j
        let mut char_offsets = Vec::with_capacity(strings.len());
        let mut total_chars = 0usize;
        for s_str in strings {
            char_offsets.push(total_chars);
            total_chars += s_str.len();
        }

        // m_{global_char, p}: index x_count + global_char * b + p
        let m_offset = x_count;
        let num_vars = x_count + total_chars * b;

        let mut constraints = Vec::new();

        // 1. One-hot symbol at each position: Σ_a x_{p,a} = 1  ∀ p
        for p in 0..b {
            let terms: Vec<(usize, f64)> = (0..k).map(|a| (p * k + a, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2. Each character matched to exactly one position: Σ_p m_{gc,p} = 1
        for gc in 0..total_chars {
            let terms: Vec<(usize, f64)> = (0..b).map(|p| (m_offset + gc * b + p, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 3. Symbol consistency: m_{gc,p} <= x_{p,a} where a is the symbol at gc
        for (s_idx, s_str) in strings.iter().enumerate() {
            for (j, &sym) in s_str.iter().enumerate() {
                let gc = char_offsets[s_idx] + j;
                for p in 0..b {
                    // m_{gc,p} <= x_{p,sym}
                    constraints.push(LinearConstraint::le(
                        vec![(m_offset + gc * b + p, 1.0), (p * k + sym, -1.0)],
                        0.0,
                    ));
                }
            }
        }

        // 4. Monotonicity: matching positions strictly increase within each string.
        //    For consecutive chars j and j+1 of string s:
        //    Σ_p p * m_{gc_j,p} < Σ_p p * m_{gc_{j+1},p}
        //    i.e., Σ_p p * m_{gc_{j+1},p} - Σ_p p * m_{gc_j,p} >= 1
        for (s_idx, s_str) in strings.iter().enumerate() {
            for j in 0..s_str.len().saturating_sub(1) {
                let gc_j = char_offsets[s_idx] + j;
                let gc_next = char_offsets[s_idx] + j + 1;
                let mut terms = Vec::new();
                for p in 0..b {
                    terms.push((m_offset + gc_next * b + p, p as f64));
                    terms.push((m_offset + gc_j * b + p, -(p as f64)));
                }
                constraints.push(LinearConstraint::ge(terms, 1.0));
            }
        }

        // 5. Contiguous padding: if position p is padding, then p+1 must also be padding.
        //    x_{p,pad} <= x_{p+1,pad}  for p in 0..b-1
        for p in 0..b.saturating_sub(1) {
            constraints.push(LinearConstraint::le(
                vec![(p * k + pad, 1.0), ((p + 1) * k + pad, -1.0)],
                0.0,
            ));
        }

        // Objective: minimize non-padding positions = maximize padding positions
        let objective: Vec<(usize, f64)> = (0..b).map(|p| (p * k + pad, 1.0)).collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);
        ReductionSCSToILP {
            target,
            max_length: b,
            alphabet_size: alpha,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "shortestcommonsupersequence_to_ilp",
        build: || {
            // Alphabet {0,1}, strings [0,1] and [1,0]
            let source = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]);
            let reduction: ReductionSCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
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
#[path = "../unit_tests/rules/shortestcommonsupersequence_ilp.rs"]
mod tests;
