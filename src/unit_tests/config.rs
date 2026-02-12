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
fn test_config_iterator_zero_variables() {
    // 0 variables means exactly 1 configuration: the empty config
    let iter = ConfigIterator::new(0, 2);
    assert_eq!(iter.total(), 1);
    let configs: Vec<_> = iter.collect();
    let expected: Vec<Vec<usize>> = vec![vec![]];
    assert_eq!(configs, expected); // One config: the empty config
}

#[test]
fn test_config_iterator_zero_flavors() {
    // Non-zero variables with 0 flavors means no valid configs
    let iter = ConfigIterator::new(3, 0);
    assert_eq!(iter.total(), 0);
    let configs: Vec<_> = iter.collect();
    assert!(configs.is_empty());
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
    assert_eq!(
        config_to_bits(&[0, 1, 0, 1]),
        vec![false, true, false, true]
    );
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

// === DimsIterator tests ===

#[test]
fn test_dims_iterator_uniform_binary() {
    let iter = DimsIterator::new(vec![2, 2, 2]);
    assert_eq!(iter.total(), 8);

    let configs: Vec<_> = iter.collect();
    assert_eq!(configs.len(), 8);
    assert_eq!(configs[0], vec![0, 0, 0]);
    assert_eq!(configs[7], vec![1, 1, 1]);
}

#[test]
fn test_dims_iterator_mixed_dims() {
    let iter = DimsIterator::new(vec![2, 3]);
    assert_eq!(iter.total(), 6);

    let configs: Vec<_> = iter.collect();
    assert_eq!(configs.len(), 6);
    assert_eq!(configs[0], vec![0, 0]);
    assert_eq!(configs[1], vec![0, 1]);
    assert_eq!(configs[2], vec![0, 2]);
    assert_eq!(configs[3], vec![1, 0]);
    assert_eq!(configs[4], vec![1, 1]);
    assert_eq!(configs[5], vec![1, 2]);
}

#[test]
fn test_dims_iterator_empty() {
    // Empty dims means exactly 1 configuration: the empty config
    let iter = DimsIterator::new(vec![]);
    assert_eq!(iter.total(), 1);
    let configs: Vec<_> = iter.collect();
    let expected: Vec<Vec<usize>> = vec![vec![]];
    assert_eq!(configs, expected); // One config: the empty config
}

#[test]
fn test_dims_iterator_zero_dimension() {
    // Any dimension being 0 means no valid configs
    let iter = DimsIterator::new(vec![2, 0, 3]);
    assert_eq!(iter.total(), 0);
    assert!(iter.collect::<Vec<_>>().is_empty());
}

#[test]
fn test_dims_iterator_single_variable() {
    let iter = DimsIterator::new(vec![4]);
    assert_eq!(iter.total(), 4);
    let configs: Vec<_> = iter.collect();
    assert_eq!(configs, vec![vec![0], vec![1], vec![2], vec![3]]);
}

#[test]
fn test_dims_iterator_zero_dim() {
    let iter = DimsIterator::new(vec![2, 0, 3]);
    assert_eq!(iter.total(), 0);
    assert_eq!(iter.collect::<Vec<_>>().len(), 0);
}

#[test]
fn test_dims_iterator_exact_size() {
    let mut iter = DimsIterator::new(vec![2, 3]);
    assert_eq!(iter.len(), 6);
    iter.next();
    assert_eq!(iter.len(), 5);
    iter.next();
    iter.next();
    assert_eq!(iter.len(), 3);
}
