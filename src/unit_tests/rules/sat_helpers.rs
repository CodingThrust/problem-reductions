use super::*;

#[test]
fn test_sat_variable_allocator_numeric_boundaries() {
    let mut allocator = SatVariableAllocator::new("test reduction", i64::MAX as usize - 1)
        .expect("largest valid starting count");
    assert_eq!(allocator.allocate().unwrap(), i64::MAX);
    assert_eq!(allocator.num_vars(), i64::MAX as usize);

    let error = allocator.allocate().unwrap_err();
    let error = error.to_string();
    assert!(error.contains("test reduction"));
    assert!(error.contains(&format!("limited to {}", i64::MAX)));
}

#[test]
fn test_sat_variable_allocator_batch_numeric_boundaries() {
    let mut exact = SatVariableAllocator::new("exact batch", i64::MAX as usize - 2).unwrap();
    assert_eq!(
        exact.allocate_many(2).unwrap(),
        vec![i64::MAX - 1, i64::MAX]
    );
    assert_eq!(exact.num_vars(), i64::MAX as usize);

    let mut overflow = SatVariableAllocator::new("overflow batch", i64::MAX as usize - 1).unwrap();
    let error = overflow.allocate_many(2).unwrap_err();
    let error = error.to_string();
    assert!(error.contains("cannot allocate 2 auxiliary variables"));
    assert_eq!(overflow.num_vars(), i64::MAX as usize - 1);
}
