//! Reduction from Factoring to CircuitSAT.
//!
//! The reduction constructs a multiplier circuit that computes p × q
//! and constrains the output to equal the target number N.
//! A satisfying assignment to the circuit gives the factorization.
//!
//! The multiplier circuit uses an array multiplier structure with
//! carry propagation, building up partial products row by row.

use crate::models::specialized::{Assignment, BooleanExpr, Circuit, CircuitSAT, Factoring};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

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
    target: CircuitSAT<i32>,
    /// Variable names for the first factor p (bit positions).
    p_vars: Vec<String>,
    /// Variable names for the second factor q (bit positions).
    q_vars: Vec<String>,
    /// Variable names for the product (bit positions).
    m_vars: Vec<String>,
    /// Size of the source problem.
    source_size: ProblemSize,
}

impl ReductionResult for ReductionFactoringToCircuit {
    type Source = Factoring;
    type Target = CircuitSAT<i32>;

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

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
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

impl ReduceTo<CircuitSAT<i32>> for Factoring {
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
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_read_bit() {
        // 6 = 110 in binary (little-endian: bit1=0, bit2=1, bit3=1)
        assert!(!read_bit(6, 1)); // bit 1 (LSB) = 0
        assert!(read_bit(6, 2)); // bit 2 = 1
        assert!(read_bit(6, 3)); // bit 3 = 1
        assert!(!read_bit(6, 4)); // bit 4 = 0

        // 15 = 1111 in binary
        assert!(read_bit(15, 1));
        assert!(read_bit(15, 2));
        assert!(read_bit(15, 3));
        assert!(read_bit(15, 4));
        assert!(!read_bit(15, 5));
    }

    #[test]
    fn test_reduction_structure() {
        // Factor 6 = 2 * 3 with 2-bit factors
        let factoring = Factoring::new(2, 2, 6);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);

        assert_eq!(reduction.p_vars().len(), 2);
        assert_eq!(reduction.q_vars().len(), 2);
        assert_eq!(reduction.m_vars().len(), 4); // 2 + 2 = 4 bits for product
    }

    #[test]
    fn test_reduction_structure_3x3() {
        // Factor 15 = 3 * 5 with 3-bit factors
        let factoring = Factoring::new(3, 3, 15);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);

        assert_eq!(reduction.p_vars().len(), 3);
        assert_eq!(reduction.q_vars().len(), 3);
        assert_eq!(reduction.m_vars().len(), 6); // 3 + 3 = 6 bits for product
    }

    /// Helper function to evaluate a circuit with given inputs.
    /// Returns a HashMap of all variable assignments after propagation.
    fn evaluate_multiplier_circuit(
        reduction: &ReductionFactoringToCircuit,
        p_val: u64,
        q_val: u64,
    ) -> HashMap<String, bool> {
        let circuit = reduction.target_problem().circuit();
        let mut assignments: HashMap<String, bool> = HashMap::new();

        // Set input variables for p
        for (i, var_name) in reduction.p_vars().iter().enumerate() {
            let bit = ((p_val >> i) & 1) == 1;
            assignments.insert(var_name.clone(), bit);
        }

        // Set input variables for q
        for (i, var_name) in reduction.q_vars().iter().enumerate() {
            let bit = ((q_val >> i) & 1) == 1;
            assignments.insert(var_name.clone(), bit);
        }

        // Evaluate the circuit assignments in order
        for assign in &circuit.assignments {
            let result = assign.expr.evaluate(&assignments);
            for out in &assign.outputs {
                assignments.insert(out.clone(), result);
            }
        }

        assignments
    }

    /// Check if inputs satisfying the circuit give correct factorization.
    /// This tests the core functionality: given p and q, does the circuit
    /// correctly identify when p * q = target?
    fn check_factorization_satisfies(
        factoring: &Factoring,
        reduction: &ReductionFactoringToCircuit,
        p_val: u64,
        q_val: u64,
    ) -> bool {
        let assignments = evaluate_multiplier_circuit(reduction, p_val, q_val);
        let circuit = reduction.target_problem().circuit();

        // Check if all assignments are satisfied
        for assign in &circuit.assignments {
            if !assign.is_satisfied(&assignments) {
                return false;
            }
        }

        // Also verify the product equals target (redundant but explicit)
        p_val * q_val == factoring.target()
    }

    #[test]
    fn test_factorization_6_satisfies_circuit() {
        let factoring = Factoring::new(2, 2, 6);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);

        // 2 * 3 = 6 should satisfy the circuit
        assert!(
            check_factorization_satisfies(&factoring, &reduction, 2, 3),
            "2 * 3 = 6 should satisfy the circuit"
        );

        // 3 * 2 = 6 should also satisfy
        assert!(
            check_factorization_satisfies(&factoring, &reduction, 3, 2),
            "3 * 2 = 6 should satisfy the circuit"
        );

        // 1 * 1 = 1 != 6 should NOT satisfy (product constraint fails)
        assert!(
            !check_factorization_satisfies(&factoring, &reduction, 1, 1),
            "1 * 1 != 6 should not satisfy the circuit"
        );

        // 2 * 2 = 4 != 6 should NOT satisfy
        assert!(
            !check_factorization_satisfies(&factoring, &reduction, 2, 2),
            "2 * 2 != 6 should not satisfy the circuit"
        );
    }

    #[test]
    fn test_factorization_15_satisfies_circuit() {
        let factoring = Factoring::new(4, 4, 15);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);

        // Valid factorizations of 15
        assert!(
            check_factorization_satisfies(&factoring, &reduction, 3, 5),
            "3 * 5 = 15 should satisfy"
        );
        assert!(
            check_factorization_satisfies(&factoring, &reduction, 5, 3),
            "5 * 3 = 15 should satisfy"
        );
        assert!(
            check_factorization_satisfies(&factoring, &reduction, 1, 15),
            "1 * 15 = 15 should satisfy"
        );
        assert!(
            check_factorization_satisfies(&factoring, &reduction, 15, 1),
            "15 * 1 = 15 should satisfy"
        );

        // Invalid: 2 * 7 = 14 != 15
        assert!(
            !check_factorization_satisfies(&factoring, &reduction, 2, 7),
            "2 * 7 != 15 should not satisfy"
        );
    }

    #[test]
    fn test_factorization_21_satisfies_circuit() {
        let factoring = Factoring::new(3, 3, 21);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);

        // 3 * 7 = 21
        assert!(
            check_factorization_satisfies(&factoring, &reduction, 3, 7),
            "3 * 7 = 21 should satisfy"
        );
        assert!(
            check_factorization_satisfies(&factoring, &reduction, 7, 3),
            "7 * 3 = 21 should satisfy"
        );

        // Invalid: 3 * 5 = 15 != 21
        assert!(
            !check_factorization_satisfies(&factoring, &reduction, 3, 5),
            "3 * 5 != 21 should not satisfy"
        );
    }

    #[test]
    fn test_source_and_target_size() {
        let factoring = Factoring::new(3, 4, 15);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_bits_first"), Some(3));
        assert_eq!(source_size.get("num_bits_second"), Some(4));
        assert!(target_size.get("num_variables").unwrap() > 0);
        assert!(target_size.get("num_assignments").unwrap() > 0);
    }

    #[test]
    fn test_extract_solution() {
        let factoring = Factoring::new(2, 2, 6);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);
        let circuit_sat = reduction.target_problem();

        // Create a solution where p=2 (binary: 01) and q=3 (binary: 11)
        // We need to find the indices of p1, p2, q1, q2 in the variable list
        let var_names = circuit_sat.variable_names();
        let mut sol = vec![0usize; var_names.len()];

        // Now evaluate the circuit to set all internal variables correctly
        let assignments = evaluate_multiplier_circuit(&reduction, 2, 3);
        for (i, name) in var_names.iter().enumerate() {
            if let Some(&val) = assignments.get(name) {
                sol[i] = if val { 1 } else { 0 };
            }
        }

        let factoring_sol = reduction.extract_solution(&sol);
        assert_eq!(factoring_sol.len(), 4, "Should have 4 bits (2 for p, 2 for q)");

        let (p, q) = factoring.read_factors(&factoring_sol);
        assert_eq!(p, 2, "p should be 2");
        assert_eq!(q, 3, "q should be 3");
        assert_eq!(p * q, 6, "Product should equal target");
    }

    #[test]
    fn test_prime_7_only_trivial_factorizations() {
        let factoring = Factoring::new(3, 3, 7);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);

        // Check that only trivial factorizations satisfy
        for p in 0..8u64 {
            for q in 0..8u64 {
                let satisfies = check_factorization_satisfies(&factoring, &reduction, p, q);
                let is_valid_factorization = p * q == 7;

                if is_valid_factorization {
                    assert!(
                        satisfies,
                        "{}*{}=7 should satisfy the circuit",
                        p,
                        q
                    );
                    // Check it's a trivial factorization (1*7 or 7*1)
                    assert!(
                        (p == 1 && q == 7) || (p == 7 && q == 1),
                        "7 is prime, so only 1*7 or 7*1 should work"
                    );
                } else if p > 0 && q > 0 {
                    // Non-zero products that don't equal 7 should not satisfy
                    assert!(
                        !satisfies,
                        "{}*{}={} != 7 should not satisfy the circuit",
                        p,
                        q,
                        p * q
                    );
                }
            }
        }
    }

    #[test]
    fn test_all_2bit_factorizations() {
        // Test all possible 2-bit * 2-bit multiplications for target 6
        let factoring = Factoring::new(2, 2, 6);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);

        let mut valid_factorizations = Vec::new();
        for p in 0..4u64 {
            for q in 0..4u64 {
                if check_factorization_satisfies(&factoring, &reduction, p, q) {
                    valid_factorizations.push((p, q));
                }
            }
        }

        // Only 2*3 and 3*2 should satisfy (both give 6)
        assert_eq!(valid_factorizations.len(), 2, "Should find exactly 2 factorizations of 6");
        assert!(valid_factorizations.contains(&(2, 3)), "Should find 2*3");
        assert!(valid_factorizations.contains(&(3, 2)), "Should find 3*2");
    }

    #[test]
    fn test_factorization_1_trivial() {
        // Factor 1 = 1 * 1
        let factoring = Factoring::new(2, 2, 1);
        let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);

        assert!(
            check_factorization_satisfies(&factoring, &reduction, 1, 1),
            "1 * 1 = 1 should satisfy"
        );
        assert!(
            !check_factorization_satisfies(&factoring, &reduction, 2, 1),
            "2 * 1 = 2 != 1 should not satisfy"
        );
    }
}

// Register reduction with inventory for auto-discovery
use crate::poly;
use crate::rules::registry::{ReductionEntry, ReductionOverhead};

inventory::submit! {
    ReductionEntry {
        source_name: "Factoring",
        target_name: "CircuitSAT",
        source_graph: "Factoring",
        target_graph: "Circuit",
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_gates", poly!(num_bits_first^2)),
        ]),
    }
}
