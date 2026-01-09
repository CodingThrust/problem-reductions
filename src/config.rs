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
        let total_configs = num_flavors.pow(num_variables as u32);
        let current = if num_variables == 0 || num_flavors == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_iterator_binary() {
        let iter = ConfigIterator::new(3, 2);
        assert_eq!(iter.total(), 8);

        let configs: Vec<_> = iter.collect();
        assert_eq!(configs.len(), 8);
        assert_eq!(configs[0], vec![0, 0, 0]);
        assert_eq!(configs[1], vec![0, 0, 1]);
        assert_eq!(configs[2], vec![0, 1, 0]);
        assert_eq!(configs[3], vec![0, 1, 1]);
        assert_eq!(configs[4], vec![1, 0, 0]);
        assert_eq!(configs[5], vec![1, 0, 1]);
        assert_eq!(configs[6], vec![1, 1, 0]);
        assert_eq!(configs[7], vec![1, 1, 1]);
    }

    #[test]
    fn test_config_iterator_ternary() {
        let iter = ConfigIterator::new(2, 3);
        assert_eq!(iter.total(), 9);

        let configs: Vec<_> = iter.collect();
        assert_eq!(configs.len(), 9);
        assert_eq!(configs[0], vec![0, 0]);
        assert_eq!(configs[1], vec![0, 1]);
        assert_eq!(configs[2], vec![0, 2]);
        assert_eq!(configs[3], vec![1, 0]);
        assert_eq!(configs[8], vec![2, 2]);
    }

    #[test]
    fn test_config_iterator_empty() {
        let iter = ConfigIterator::new(0, 2);
        assert_eq!(iter.total(), 1);
        let configs: Vec<_> = iter.collect();
        assert_eq!(configs.len(), 0); // Empty because num_variables is 0
    }

    #[test]
    fn test_config_iterator_single_variable() {
        let iter = ConfigIterator::new(1, 4);
        assert_eq!(iter.total(), 4);

        let configs: Vec<_> = iter.collect();
        assert_eq!(configs, vec![vec![0], vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_index_to_config() {
        assert_eq!(index_to_config(0, 3, 2), vec![0, 0, 0]);
        assert_eq!(index_to_config(1, 3, 2), vec![0, 0, 1]);
        assert_eq!(index_to_config(7, 3, 2), vec![1, 1, 1]);
        assert_eq!(index_to_config(5, 3, 2), vec![1, 0, 1]);
    }

    #[test]
    fn test_config_to_index() {
        assert_eq!(config_to_index(&[0, 0, 0], 2), 0);
        assert_eq!(config_to_index(&[0, 0, 1], 2), 1);
        assert_eq!(config_to_index(&[1, 1, 1], 2), 7);
        assert_eq!(config_to_index(&[1, 0, 1], 2), 5);
    }

    #[test]
    fn test_index_config_roundtrip() {
        for i in 0..27 {
            let config = index_to_config(i, 3, 3);
            let back = config_to_index(&config, 3);
            assert_eq!(i, back);
        }
    }

    #[test]
    fn test_config_to_bits() {
        assert_eq!(config_to_bits(&[0, 1, 0, 1]), vec![false, true, false, true]);
        assert_eq!(config_to_bits(&[0, 0, 0]), vec![false, false, false]);
        assert_eq!(config_to_bits(&[1, 1, 1]), vec![true, true, true]);
    }

    #[test]
    fn test_bits_to_config() {
        assert_eq!(
            bits_to_config(&[false, true, false, true]),
            vec![0, 1, 0, 1]
        );
        assert_eq!(bits_to_config(&[true, true, true]), vec![1, 1, 1]);
    }

    #[test]
    fn test_exact_size_iterator() {
        let mut iter = ConfigIterator::new(3, 2);
        assert_eq!(iter.len(), 8);
        iter.next();
        assert_eq!(iter.len(), 7);
        iter.next();
        iter.next();
        assert_eq!(iter.len(), 5);
    }
}
