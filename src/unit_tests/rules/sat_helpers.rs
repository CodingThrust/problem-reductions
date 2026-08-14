use super::*;

#[test]
fn test_sat_variable_allocator_numeric_boundaries() {
    let mut allocator = SatVariableAllocator::new("test reduction", i32::MAX as usize - 1)
        .expect("largest valid starting count");
    assert_eq!(allocator.allocate().unwrap(), i32::MAX);
    assert_eq!(allocator.num_vars(), i32::MAX as usize);

    let error = allocator.allocate().unwrap_err();
    assert!(error.contains("test reduction"));
    assert!(error.contains("limited to 2147483647"));
}

#[test]
fn test_sat_variable_allocator_batch_numeric_boundaries() {
    let mut exact = SatVariableAllocator::new("exact batch", i32::MAX as usize - 2).unwrap();
    assert_eq!(
        exact.allocate_many(2).unwrap(),
        vec![i32::MAX - 1, i32::MAX]
    );
    assert_eq!(exact.num_vars(), i32::MAX as usize);

    let mut overflow = SatVariableAllocator::new("overflow batch", i32::MAX as usize - 1).unwrap();
    let error = overflow.allocate_many(2).unwrap_err();
    assert!(error.contains("cannot allocate 2 auxiliary variables"));
    assert_eq!(overflow.num_vars(), i32::MAX as usize - 1);
}
