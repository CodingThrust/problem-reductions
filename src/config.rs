//! Configuration utilities for problem solving.

/// Convert a configuration index to a configuration vector.
///
/// The index is treated as a number in base `num_flavors`.
pub fn index_to_config(index: usize, num_variables: usize, num_flavors: usize) -> Vec<usize> {
    let mut config = vec![0; num_variables];
    let mut remaining = index;
    for i in (0..num_variables).rev() {
        config[i] = remaining % num_flavors;
        remaining /= num_flavors;
    }
    config
}

/// Convert a configuration vector to an index.
///
/// The configuration is treated as digits in base `num_flavors`.
pub fn config_to_index(config: &[usize], num_flavors: usize) -> usize {
    let mut index = 0;
    for &value in config {
        index = index * num_flavors + value;
    }
    index
}

/// Convert a binary configuration to a bitvec-style representation.
pub(crate) fn config_to_bits(config: &[usize]) -> Vec<bool> {
    config.iter().map(|&v| v != 0).collect()
}

/// Convert a bitvec-style representation to a binary configuration.
pub(crate) fn bits_to_config(bits: &[bool]) -> Vec<usize> {
    bits.iter().map(|&b| if b { 1 } else { 0 }).collect()
}

#[cfg(test)]
#[path = "unit_tests/config.rs"]
mod tests;
