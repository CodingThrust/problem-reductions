//! Truth table utilities for logic gadgets.
//!
//! This module provides a `TruthTable` type for representing boolean functions
//! and their truth tables, useful for constructing logic gadgets in reductions.

use bitvec::prelude::*;
use serde::{Deserialize, Serialize};

/// A truth table representing a boolean function.
///
/// The truth table stores the output for each possible input combination.
/// For n input variables, there are 2^n rows in the table.
///
/// # Example
///
/// ```
/// use problemreductions::truth_table::TruthTable;
///
/// // Create AND gate truth table
/// let and_gate = TruthTable::from_outputs(2, vec![false, false, false, true]);
/// assert!(!and_gate.evaluate(&[false, false]));
/// assert!(!and_gate.evaluate(&[true, false]));
/// assert!(and_gate.evaluate(&[true, true]));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthTable {
    /// Number of input variables.
    num_inputs: usize,
    /// Output values for each input combination.
    /// Index corresponds to input as binary number (LSB first).
    outputs: BitVec,
}

/// Serialization-friendly representation of a TruthTable.
#[derive(Serialize, Deserialize)]
struct TruthTableSerde {
    num_inputs: usize,
    outputs: Vec<bool>,
}

impl Serialize for TruthTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serde_repr = TruthTableSerde {
            num_inputs: self.num_inputs,
            outputs: self.outputs.iter().by_vals().collect(),
        };
        serde_repr.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TruthTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serde_repr = TruthTableSerde::deserialize(deserializer)?;
        Ok(TruthTable {
            num_inputs: serde_repr.num_inputs,
            outputs: serde_repr.outputs.into_iter().collect(),
        })
    }
}

impl TruthTable {
    /// Create a truth table from a vector of boolean outputs.
    ///
    /// The outputs vector must have exactly 2^num_inputs elements.
    /// Index i corresponds to the input where the j-th bit represents variable j.
    pub fn from_outputs(num_inputs: usize, outputs: Vec<bool>) -> Self {
        let expected_len = 1 << num_inputs;
        assert_eq!(
            outputs.len(),
            expected_len,
            "outputs length must be 2^num_inputs = {}, got {}",
            expected_len,
            outputs.len()
        );

        let bits: BitVec = outputs.into_iter().collect();
        Self { num_inputs, outputs: bits }
    }

    /// Create a truth table from a function.
    ///
    /// The function takes a slice of booleans (the input) and returns the output.
    pub fn from_function<F>(num_inputs: usize, f: F) -> Self
    where
        F: Fn(&[bool]) -> bool,
    {
        let num_rows = 1 << num_inputs;
        let mut outputs = BitVec::with_capacity(num_rows);

        for i in 0..num_rows {
            let input: Vec<bool> = (0..num_inputs).map(|j| (i >> j) & 1 == 1).collect();
            outputs.push(f(&input));
        }

        Self { num_inputs, outputs }
    }

    /// Get the number of input variables.
    pub fn num_inputs(&self) -> usize {
        self.num_inputs
    }

    /// Get the number of rows (2^num_inputs).
    pub fn num_rows(&self) -> usize {
        1 << self.num_inputs
    }

    /// Evaluate the truth table for a given input.
    pub fn evaluate(&self, input: &[bool]) -> bool {
        assert_eq!(
            input.len(),
            self.num_inputs,
            "input length must match num_inputs"
        );

        let index = Self::input_to_index(input);
        self.outputs[index]
    }

    /// Evaluate using a usize config (0/1 values).
    pub fn evaluate_config(&self, config: &[usize]) -> bool {
        let input: Vec<bool> = config.iter().map(|&x| x != 0).collect();
        self.evaluate(&input)
    }

    /// Convert an input to its index in the truth table.
    fn input_to_index(input: &[bool]) -> usize {
        input
            .iter()
            .enumerate()
            .map(|(i, &b)| if b { 1 << i } else { 0 })
            .sum()
    }

    /// Get the output values as a bitvec.
    pub fn outputs(&self) -> &BitVec {
        &self.outputs
    }

    /// Get all outputs as a vector of bools.
    pub fn outputs_vec(&self) -> Vec<bool> {
        self.outputs.iter().by_vals().collect()
    }

    /// Get the input configuration for a given row index.
    pub fn index_to_input(&self, index: usize) -> Vec<bool> {
        (0..self.num_inputs).map(|j| (index >> j) & 1 == 1).collect()
    }

    /// Count the number of true outputs.
    pub fn count_ones(&self) -> usize {
        self.outputs.count_ones()
    }

    /// Count the number of false outputs.
    pub fn count_zeros(&self) -> usize {
        self.outputs.count_zeros()
    }

    /// Check if the function is satisfiable (has at least one true output).
    pub fn is_satisfiable(&self) -> bool {
        self.outputs.any()
    }

    /// Check if the function is a tautology (all outputs are true).
    pub fn is_tautology(&self) -> bool {
        self.outputs.all()
    }

    /// Check if the function is a contradiction (all outputs are false).
    pub fn is_contradiction(&self) -> bool {
        self.outputs.not_any()
    }

    /// Get all satisfying assignments.
    pub fn satisfying_assignments(&self) -> Vec<Vec<bool>> {
        (0..self.num_rows())
            .filter(|&i| self.outputs[i])
            .map(|i| self.index_to_input(i))
            .collect()
    }

    /// Create an AND gate truth table.
    pub fn and(num_inputs: usize) -> Self {
        Self::from_function(num_inputs, |input| input.iter().all(|&b| b))
    }

    /// Create an OR gate truth table.
    pub fn or(num_inputs: usize) -> Self {
        Self::from_function(num_inputs, |input| input.iter().any(|&b| b))
    }

    /// Create a NOT gate truth table (1 input).
    pub fn not() -> Self {
        Self::from_outputs(1, vec![true, false])
    }

    /// Create an XOR gate truth table.
    pub fn xor(num_inputs: usize) -> Self {
        Self::from_function(num_inputs, |input| {
            input.iter().filter(|&&b| b).count() % 2 == 1
        })
    }

    /// Create a NAND gate truth table.
    pub fn nand(num_inputs: usize) -> Self {
        Self::from_function(num_inputs, |input| !input.iter().all(|&b| b))
    }

    /// Create a NOR gate truth table.
    pub fn nor(num_inputs: usize) -> Self {
        Self::from_function(num_inputs, |input| !input.iter().any(|&b| b))
    }

    /// Create an XNOR gate truth table.
    pub fn xnor(num_inputs: usize) -> Self {
        Self::from_function(num_inputs, |input| {
            input.iter().filter(|&&b| b).count() % 2 == 0
        })
    }

    /// Create an implication gate (a -> b = !a OR b).
    /// Input 0 is 'a', input 1 is 'b'.
    pub fn implies() -> Self {
        // Index 0: [F,F] -> T, Index 1: [T,F] -> F, Index 2: [F,T] -> T, Index 3: [T,T] -> T
        Self::from_outputs(2, vec![true, false, true, true])
    }

    /// Combine two truth tables using AND.
    pub fn and_with(&self, other: &TruthTable) -> TruthTable {
        assert_eq!(self.num_inputs, other.num_inputs);
        let outputs: BitVec = self.outputs.iter().zip(other.outputs.iter())
            .map(|(a, b)| *a && *b)
            .collect();
        TruthTable {
            num_inputs: self.num_inputs,
            outputs,
        }
    }

    /// Combine two truth tables using OR.
    pub fn or_with(&self, other: &TruthTable) -> TruthTable {
        assert_eq!(self.num_inputs, other.num_inputs);
        let outputs: BitVec = self.outputs.iter().zip(other.outputs.iter())
            .map(|(a, b)| *a || *b)
            .collect();
        TruthTable {
            num_inputs: self.num_inputs,
            outputs,
        }
    }

    /// Negate the truth table.
    pub fn negate(&self) -> TruthTable {
        let outputs: BitVec = self.outputs.iter().map(|b| !*b).collect();
        TruthTable {
            num_inputs: self.num_inputs,
            outputs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_gate() {
        let and = TruthTable::and(2);
        assert!(!and.evaluate(&[false, false]));
        assert!(!and.evaluate(&[true, false]));
        assert!(!and.evaluate(&[false, true]));
        assert!(and.evaluate(&[true, true]));
    }

    #[test]
    fn test_or_gate() {
        let or = TruthTable::or(2);
        assert!(!or.evaluate(&[false, false]));
        assert!(or.evaluate(&[true, false]));
        assert!(or.evaluate(&[false, true]));
        assert!(or.evaluate(&[true, true]));
    }

    #[test]
    fn test_not_gate() {
        let not = TruthTable::not();
        assert!(not.evaluate(&[false]));
        assert!(!not.evaluate(&[true]));
    }

    #[test]
    fn test_xor_gate() {
        let xor = TruthTable::xor(2);
        assert!(!xor.evaluate(&[false, false]));
        assert!(xor.evaluate(&[true, false]));
        assert!(xor.evaluate(&[false, true]));
        assert!(!xor.evaluate(&[true, true]));
    }

    #[test]
    fn test_nand_gate() {
        let nand = TruthTable::nand(2);
        assert!(nand.evaluate(&[false, false]));
        assert!(nand.evaluate(&[true, false]));
        assert!(nand.evaluate(&[false, true]));
        assert!(!nand.evaluate(&[true, true]));
    }

    #[test]
    fn test_implies() {
        let imp = TruthTable::implies();
        assert!(imp.evaluate(&[false, false])); // F -> F = T
        assert!(imp.evaluate(&[false, true]));  // F -> T = T
        assert!(!imp.evaluate(&[true, false])); // T -> F = F
        assert!(imp.evaluate(&[true, true]));   // T -> T = T
    }

    #[test]
    fn test_from_function() {
        let majority = TruthTable::from_function(3, |input| {
            input.iter().filter(|&&b| b).count() >= 2
        });
        assert!(!majority.evaluate(&[false, false, false]));
        assert!(!majority.evaluate(&[true, false, false]));
        assert!(majority.evaluate(&[true, true, false]));
        assert!(majority.evaluate(&[true, true, true]));
    }

    #[test]
    fn test_evaluate_config() {
        let and = TruthTable::and(2);
        assert!(!and.evaluate_config(&[0, 0]));
        assert!(!and.evaluate_config(&[1, 0]));
        assert!(and.evaluate_config(&[1, 1]));
    }

    #[test]
    fn test_satisfiable() {
        let or = TruthTable::or(2);
        assert!(or.is_satisfiable());

        let contradiction = TruthTable::from_outputs(2, vec![false, false, false, false]);
        assert!(!contradiction.is_satisfiable());
        assert!(contradiction.is_contradiction());
    }

    #[test]
    fn test_tautology() {
        let tautology = TruthTable::from_outputs(2, vec![true, true, true, true]);
        assert!(tautology.is_tautology());

        let or = TruthTable::or(2);
        assert!(!or.is_tautology());
    }

    #[test]
    fn test_satisfying_assignments() {
        let xor = TruthTable::xor(2);
        let sat = xor.satisfying_assignments();
        assert_eq!(sat.len(), 2);
        assert!(sat.contains(&vec![true, false]));
        assert!(sat.contains(&vec![false, true]));
    }

    #[test]
    fn test_count() {
        let and = TruthTable::and(2);
        assert_eq!(and.count_ones(), 1);
        assert_eq!(and.count_zeros(), 3);
    }

    #[test]
    fn test_index_to_input() {
        let tt = TruthTable::and(3);
        assert_eq!(tt.index_to_input(0), vec![false, false, false]);
        assert_eq!(tt.index_to_input(1), vec![true, false, false]);
        assert_eq!(tt.index_to_input(7), vec![true, true, true]);
    }

    #[test]
    fn test_outputs_vec() {
        let and = TruthTable::and(2);
        assert_eq!(and.outputs_vec(), vec![false, false, false, true]);
    }

    #[test]
    fn test_and_with() {
        let a = TruthTable::from_outputs(1, vec![false, true]);
        let b = TruthTable::from_outputs(1, vec![true, false]);
        let result = a.and_with(&b);
        assert_eq!(result.outputs_vec(), vec![false, false]);
    }

    #[test]
    fn test_or_with() {
        let a = TruthTable::from_outputs(1, vec![false, true]);
        let b = TruthTable::from_outputs(1, vec![true, false]);
        let result = a.or_with(&b);
        assert_eq!(result.outputs_vec(), vec![true, true]);
    }

    #[test]
    fn test_negate() {
        let and = TruthTable::and(2);
        let nand = and.negate();
        assert_eq!(nand.outputs_vec(), vec![true, true, true, false]);
    }

    #[test]
    fn test_num_rows() {
        let tt = TruthTable::and(3);
        assert_eq!(tt.num_rows(), 8);
    }

    #[test]
    fn test_3_input_and() {
        let and3 = TruthTable::and(3);
        assert!(!and3.evaluate(&[true, true, false]));
        assert!(and3.evaluate(&[true, true, true]));
    }

    #[test]
    fn test_xnor() {
        let xnor = TruthTable::xnor(2);
        assert!(xnor.evaluate(&[false, false]));
        assert!(!xnor.evaluate(&[true, false]));
        assert!(!xnor.evaluate(&[false, true]));
        assert!(xnor.evaluate(&[true, true]));
    }

    #[test]
    fn test_nor() {
        let nor = TruthTable::nor(2);
        assert!(nor.evaluate(&[false, false]));
        assert!(!nor.evaluate(&[true, false]));
        assert!(!nor.evaluate(&[false, true]));
        assert!(!nor.evaluate(&[true, true]));
    }
}
