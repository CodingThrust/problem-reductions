//! Reduction from Factoring to CircuitSAT.
//!
//! The reduction constructs a multiplier circuit that computes p × q
//! and constrains the output to equal the target number N.
//! A satisfying assignment to the circuit gives the factorization.
//!
//! The multiplier circuit uses an array multiplier structure with
//! carry propagation, building up partial products row by row.

use crate::models::specialized::{Assignment, BooleanExpr, Circuit, CircuitSAT, Factoring};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
/// Result of reducing Factoring to CircuitSAT.
///
/// This struct contains:
/// - The target CircuitSAT problem (the multiplier circuit)
/// - Variable indices for the first factor p (m bits)
/// - Variable indices for the second factor q (n bits)
/// - Variable indices for the product m (m+n bits)
#[derive(Debug, Clone)]
pub struct ReductionFactoringToCircuit {
    /// The target CircuitSAT problem.
    target: CircuitSAT,
    /// Variable names for the first factor p (bit positions).
    p_vars: Vec<String>,
    /// Variable names for the second factor q (bit positions).
    q_vars: Vec<String>,
    /// Variable names for the product (bit positions).
    m_vars: Vec<String>,
}

impl ReductionResult for ReductionFactoringToCircuit {
    type Source = Factoring;
    type Target = CircuitSAT;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a Factoring solution from a CircuitSAT solution.
    ///
    /// Returns a configuration where the first m bits are the first factor p,
    /// and the next n bits are the second factor q.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let var_names = self.target.variable_names();

        // Build a map from variable name to its value
        let var_map: std::collections::HashMap<&str, usize> = var_names
            .iter()
            .enumerate()
            .map(|(i, name)| (name.as_str(), target_solution.get(i).copied().unwrap_or(0)))
            .collect();

        // Extract p bits
        let p_bits: Vec<usize> = self
            .p_vars
            .iter()
            .map(|name| *var_map.get(name.as_str()).unwrap_or(&0))
            .collect();

        // Extract q bits
        let q_bits: Vec<usize> = self
            .q_vars
            .iter()
            .map(|name| *var_map.get(name.as_str()).unwrap_or(&0))
            .collect();

        // Concatenate p and q bits
        let mut result = p_bits;
        result.extend(q_bits);
        result
    }
}

impl ReductionFactoringToCircuit {
    /// Get the variable names for the first factor.
    pub fn p_vars(&self) -> &[String] {
        &self.p_vars
    }

    /// Get the variable names for the second factor.
    pub fn q_vars(&self) -> &[String] {
        &self.q_vars
    }

    /// Get the variable names for the product.
    pub fn m_vars(&self) -> &[String] {
        &self.m_vars
    }
}

/// Read the i-th bit (1-indexed) of a number (little-endian).
fn read_bit(n: u64, i: usize) -> bool {
    if i == 0 || i > 64 {
        false
    } else {
        ((n >> (i - 1)) & 1) == 1
    }
}

/// Build a single multiplier cell that computes:
/// s + 2*c = p*q + s_pre + c_pre
///
/// This is a full adder that adds three bits: (p AND q), s_pre, and c_pre.
/// Returns the assignments needed and the list of ancilla variable names.
fn build_multiplier_cell(
    s_name: &str,
    c_name: &str,
    p_name: &str,
    q_name: &str,
    s_pre: &BooleanExpr,
    c_pre: &BooleanExpr,
    cell_id: &str,
) -> (Vec<Assignment>, Vec<String>) {
    // Create unique ancilla variable names
    let a_name = format!("a_{}", cell_id);
    let a_xor_s_name = format!("axs_{}", cell_id);
    let a_xor_s_and_c_name = format!("axsc_{}", cell_id);
    let a_and_s_name = format!("as_{}", cell_id);

    let p = BooleanExpr::var(p_name);
    let q = BooleanExpr::var(q_name);
    let a = BooleanExpr::var(&a_name);
    let a_xor_s = BooleanExpr::var(&a_xor_s_name);

    // Build the assignments:
    // a = p & q (AND of the two factor bits)
    let assign_a = Assignment::new(vec![a_name.clone()], BooleanExpr::and(vec![p, q]));

    // a_xor_s = a XOR s_pre
    let assign_a_xor_s = Assignment::new(
        vec![a_xor_s_name.clone()],
        BooleanExpr::xor(vec![a.clone(), s_pre.clone()]),
    );

    // s = a_xor_s XOR c_pre (sum output)
    let assign_s = Assignment::new(
        vec![s_name.to_string()],
        BooleanExpr::xor(vec![a_xor_s.clone(), c_pre.clone()]),
    );

    // a_xor_s_and_c = a_xor_s & c_pre
    let assign_a_xor_s_and_c = Assignment::new(
        vec![a_xor_s_and_c_name.clone()],
        BooleanExpr::and(vec![a_xor_s, c_pre.clone()]),
    );

    // a_and_s = a & s_pre
    let assign_a_and_s = Assignment::new(
        vec![a_and_s_name.clone()],
        BooleanExpr::and(vec![a, s_pre.clone()]),
    );

    // c = a_xor_s_and_c | a_and_s (carry output)
    let assign_c = Assignment::new(
        vec![c_name.to_string()],
        BooleanExpr::or(vec![
            BooleanExpr::var(&a_xor_s_and_c_name),
            BooleanExpr::var(&a_and_s_name),
        ]),
    );

    let assignments = vec![
        assign_a,
        assign_a_xor_s,
        assign_s,
        assign_a_xor_s_and_c,
        assign_a_and_s,
        assign_c,
    ];

    let ancillas = vec![a_name, a_xor_s_name, a_xor_s_and_c_name, a_and_s_name];

    (assignments, ancillas)
}

#[reduction(overhead = {
    ReductionOverhead::new(vec![
        ("num_gates", poly!(num_bits_first * num_bits_second)),
    ])
})]
impl ReduceTo<CircuitSAT> for Factoring {
    type Result = ReductionFactoringToCircuit;

    fn reduce_to(&self) -> Self::Result {
        let n1 = self.m(); // bits for first factor
        let n2 = self.n(); // bits for second factor
        let target = self.target();

        // Create input variables for the two factors
        let p_vars: Vec<String> = (1..=n1).map(|i| format!("p{}", i)).collect();
        let q_vars: Vec<String> = (1..=n2).map(|i| format!("q{}", i)).collect();

        // Accumulate assignments and product bits
        let mut assignments = Vec::new();
        let mut m_vars = Vec::new();

        // Initialize s_pre (previous sum signals) with false constants
        // s_pre has n2+1 elements to handle the carry propagation
        let mut s_pre: Vec<BooleanExpr> = (0..=n2).map(|_| BooleanExpr::constant(false)).collect();

        // Build the array multiplier row by row
        for i in 1..=n1 {
            // c_pre is the carry from the previous cell in this row
            let mut c_pre = BooleanExpr::constant(false);

            for j in 1..=n2 {
                // Create signal names for this cell
                let c_name = format!("c{}_{}", i, j);
                let s_name = format!("s{}_{}", i, j);

                // Build the multiplier cell
                let cell_id = format!("{}_{}", i, j);
                let (cell_assignments, _ancillas) = build_multiplier_cell(
                    &s_name,
                    &c_name,
                    &p_vars[i - 1],
                    &q_vars[j - 1],
                    &s_pre[j], // s_pre[j+1] in 0-indexed Julia becomes s_pre[j] in 1-indexed
                    &c_pre,
                    &cell_id,
                );

                assignments.extend(cell_assignments);

                // Update c_pre for the next cell
                c_pre = BooleanExpr::var(&c_name);

                // Update s_pre for the next row
                // s_pre[j-1] (0-indexed) = s (the sum from this cell)
                s_pre[j - 1] = BooleanExpr::var(&s_name);
            }

            // The final carry becomes the last element of s_pre
            s_pre[n2] = c_pre;

            // The first element of s_pre is the i-th bit of the product
            m_vars.push(format!("s{}_{}", i, 1));
        }

        // The remaining bits of the product come from s_pre[1..=n2]
        for j in 2..=n2 {
            m_vars.push(format!("s{}_{}", n1, j));
        }
        // The final carry is the last bit
        m_vars.push(format!("c{}_{}", n1, n2));

        // Constrain the output bits to match the target number
        for (i, m_var) in m_vars.iter().enumerate() {
            let target_bit = read_bit(target, i + 1);
            assignments.push(Assignment::new(
                vec![m_var.clone()],
                BooleanExpr::constant(target_bit),
            ));
        }

        // Build the circuit
        let circuit = Circuit::new(assignments);
        let circuit_sat = CircuitSAT::new(circuit);

        ReductionFactoringToCircuit {
            target: circuit_sat,
            p_vars,
            q_vars,
            m_vars,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/factoring_circuit.rs"]
mod tests;
