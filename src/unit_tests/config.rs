use super::*;

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
