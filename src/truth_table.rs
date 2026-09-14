//! Truth table utilities for logic gadgets.
//!
//! This module provides a `TruthTable` type for representing boolean functions
//! and their truth tables, useful for constructing logic gadgets in reductions.

use crate::registry::ConstructionError;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};

/// A truth table representing a boolean function.
///
/// The truth table stores the output for each possible input combination.
/// For n input variables, there are 2^n rows in the table.
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
        Self::from_outputs(serde_repr.num_inputs, serde_repr.outputs)
            .map_err(serde::de::Error::custom)
    }
}

impl TruthTable {
    /// Create a truth table from a vector of boolean outputs.
    ///
    /// The outputs vector must have exactly 2^num_inputs elements.
    /// Index i corresponds to the input where the j-th bit represents variable j.
    pub fn from_outputs(num_inputs: usize, outputs: Vec<bool>) -> Result<Self, ConstructionError> {
        let expected_len = Self::row_count(num_inputs)?;
        if outputs.len() != expected_len {
            return Err(ConstructionError::InvalidInput(format!(
                "outputs length must be 2^num_inputs = {expected_len}, got {}",
                outputs.len()
            )));
        }
        let mut bits = Self::allocate_outputs(expected_len)?;
        for (mut bit, output) in bits.iter_mut().zip(outputs) {
            *bit = output;
        }
        Ok(Self {
            num_inputs,
            outputs: bits,
        })
    }

    /// Create a truth table by evaluating a function for each input combination.
    pub fn from_function<F>(num_inputs: usize, f: F) -> Result<Self, ConstructionError>
    where
        F: Fn(&[bool]) -> bool,
    {
        let num_rows = Self::row_count(num_inputs)?;
        let mut outputs = Self::allocate_outputs(num_rows)?;
        let mut input = vec![false; num_inputs];
        for i in 0..num_rows {
            for (j, bit) in input.iter_mut().enumerate() {
                *bit = (i >> j) & 1 == 1;
            }
            outputs.set(i, f(&input));
        }
        Ok(Self {
            num_inputs,
            outputs,
        })
    }

    fn row_count(num_inputs: usize) -> Result<usize, ConstructionError> {
        u32::try_from(num_inputs)
            .ok()
            .and_then(|shift| 1usize.checked_shl(shift))
            .filter(|&rows| rows <= BitSlice::<usize, Lsb0>::MAX_BITS)
            .ok_or_else(|| {
                ConstructionError::IntegerOverflow("representing truth-table rows".into())
            })
    }

    fn allocate_outputs(num_rows: usize) -> Result<BitVec, ConstructionError> {
        let words = num_rows.div_ceil(usize::BITS as usize);
        let mut storage = Vec::<usize>::new();
        storage.try_reserve_exact(words).map_err(|error| {
            ConstructionError::Conversion(format!("allocating truth-table storage: {error}"))
        })?;
        storage.resize(words, 0);
        let mut outputs = BitVec::from_vec(storage);
        outputs.truncate(num_rows);
        Ok(outputs)
    }

    /// Get the number of input variables.
    pub fn num_inputs(&self) -> usize {
        self.num_inputs
    }

    /// Get the number of rows (2^num_inputs).
    pub fn num_rows(&self) -> usize {
        self.outputs.len()
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
        (0..self.num_inputs)
            .map(|j| (index >> j) & 1 == 1)
            .collect()
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
    pub fn and(num_inputs: usize) -> Result<Self, ConstructionError> {
        Self::from_function(num_inputs, |input| input.iter().all(|&b| b))
    }

    /// Create an OR gate truth table.
    pub fn or(num_inputs: usize) -> Result<Self, ConstructionError> {
        Self::from_function(num_inputs, |input| input.iter().any(|&b| b))
    }

    /// Create a NOT gate truth table (1 input).
    pub fn not() -> Self {
        Self::from_outputs(1, vec![true, false]).expect("NOT has two rows")
    }

    /// Create an XOR gate truth table.
    pub fn xor(num_inputs: usize) -> Result<Self, ConstructionError> {
        Self::from_function(num_inputs, |input| {
            input.iter().filter(|&&b| b).count() % 2 == 1
        })
    }

    /// Create a NAND gate truth table.
    pub fn nand(num_inputs: usize) -> Result<Self, ConstructionError> {
        Self::from_function(num_inputs, |input| !input.iter().all(|&b| b))
    }

    /// Create a NOR gate truth table.
    pub fn nor(num_inputs: usize) -> Result<Self, ConstructionError> {
        Self::from_function(num_inputs, |input| !input.iter().any(|&b| b))
    }

    /// Create an XNOR gate truth table.
    pub fn xnor(num_inputs: usize) -> Result<Self, ConstructionError> {
        Self::from_function(num_inputs, |input| {
            input.iter().filter(|&&b| b).count().is_multiple_of(2)
        })
    }

    /// Create an implication gate (a -> b = !a OR b).
    /// Input 0 is 'a', input 1 is 'b'.
    pub fn implies() -> Self {
        // Index 0: [F,F] -> T, Index 1: [T,F] -> F, Index 2: [F,T] -> T, Index 3: [T,T] -> T
        Self::from_outputs(2, vec![true, false, true, true]).expect("implication has four rows")
    }

    /// Combine two truth tables using AND.
    pub fn and_with(&self, other: &TruthTable) -> TruthTable {
        assert_eq!(self.num_inputs, other.num_inputs);
        let outputs: BitVec = self
            .outputs
            .iter()
            .zip(other.outputs.iter())
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
        let outputs: BitVec = self
            .outputs
            .iter()
            .zip(other.outputs.iter())
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
#[path = "unit_tests/truth_table.rs"]
mod tests;
