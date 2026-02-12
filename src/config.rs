//! Configuration utilities for problem solving.

/// Iterator over all possible configurations for a given number of variables and flavors.
///
/// Generates configurations in lexicographic order, from [0, 0, ..., 0] to
/// [num_flavors-1, num_flavors-1, ..., num_flavors-1].
pub struct ConfigIterator {
    num_variables: usize,
    num_flavors: usize,
    current: Option<Vec<usize>>,
    total_configs: usize,
    current_index: usize,
}

impl ConfigIterator {
    /// Create a new configuration iterator.
    pub fn new(num_variables: usize, num_flavors: usize) -> Self {
        let total_configs = if num_variables == 0 || num_flavors == 0 {
            0
        } else {
            num_flavors.pow(num_variables as u32)
        };
        let current = if total_configs == 0 {
            None
        } else {
            Some(vec![0; num_variables])
        };
        Self {
            num_variables,
            num_flavors,
            current,
            total_configs,
            current_index: 0,
        }
    }

    /// Returns the total number of configurations.
    pub fn total(&self) -> usize {
        self.total_configs
    }
}

impl Iterator for ConfigIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take()?;
        let result = current.clone();

        // Advance to next configuration
        let mut next = current;
        let mut carry = true;
        for i in (0..self.num_variables).rev() {
            if carry {
                next[i] += 1;
                if next[i] >= self.num_flavors {
                    next[i] = 0;
                } else {
                    carry = false;
                }
            }
        }

        self.current_index += 1;
        if self.current_index < self.total_configs {
            self.current = Some(next);
        }

        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_configs - self.current_index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for ConfigIterator {}

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
pub fn config_to_bits(config: &[usize]) -> Vec<bool> {
    config.iter().map(|&v| v != 0).collect()
}

/// Convert a bitvec-style representation to a binary configuration.
pub fn bits_to_config(bits: &[bool]) -> Vec<usize> {
    bits.iter().map(|&b| if b { 1 } else { 0 }).collect()
}

/// Iterator over all configurations for per-variable dimension sizes.
///
/// Unlike `ConfigIterator` which assumes uniform flavors, this supports
/// different cardinalities per variable (e.g., `dims = [2, 3, 2]`).
pub struct DimsIterator {
    dims: Vec<usize>,
    current: Option<Vec<usize>>,
    total_configs: usize,
    current_index: usize,
}

impl DimsIterator {
    /// Create a new iterator from per-variable dimensions.
    pub fn new(dims: Vec<usize>) -> Self {
        let total_configs = if dims.is_empty() {
            0
        } else {
            dims.iter().copied().try_fold(1usize, |acc, d| {
                if d == 0 { None } else { acc.checked_mul(d) }
            }).unwrap_or(0)
        };
        let current = if total_configs == 0 {
            None
        } else {
            Some(vec![0; dims.len()])
        };
        Self {
            dims,
            current,
            total_configs,
            current_index: 0,
        }
    }

    /// Returns the total number of configurations.
    pub fn total(&self) -> usize {
        self.total_configs
    }
}

impl Iterator for DimsIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take()?;
        let result = current.clone();

        // Advance to next configuration
        let mut next = current;
        let mut carry = true;
        for i in (0..self.dims.len()).rev() {
            if carry {
                next[i] += 1;
                if next[i] >= self.dims[i] {
                    next[i] = 0;
                } else {
                    carry = false;
                }
            }
        }

        self.current_index += 1;
        if self.current_index < self.total_configs {
            self.current = Some(next);
        }

        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_configs - self.current_index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for DimsIterator {}

#[cfg(test)]
#[path = "unit_tests/config.rs"]
mod tests;
