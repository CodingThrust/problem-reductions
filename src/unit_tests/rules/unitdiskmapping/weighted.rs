use super::*;

#[test]
fn trace_centers_returns_one_center_per_source_vertex() {
    let result =
        crate::rules::unitdiskmapping::triangular::map_weighted(3, &[(0, 1), (1, 2)]).unwrap();
    let centers = trace_centers(&result).unwrap();

    assert_eq!(centers.len(), 3);
    assert!(centers.iter().all(|&(row, column)| row > 0 && column > 0));
}

#[test]
fn map_weights_adds_one_weight_per_source_vertex() {
    let result =
        crate::rules::unitdiskmapping::triangular::map_weighted(3, &[(0, 1), (1, 2)]).unwrap();
    let mapped = map_weights(&result, &[0.5, 0.3, 0.7]).unwrap();

    assert_eq!(mapped.len(), result.positions.len());
    assert!(mapped
        .iter()
        .all(|weight| weight.is_finite() && *weight > 0.0));
}

#[test]
fn map_weights_rejects_invalid_source_weight() {
    let result = crate::rules::unitdiskmapping::triangular::map_weighted(2, &[(0, 1)]).unwrap();

    assert!(matches!(
        map_weights(&result, &[1.5, 0.3]),
        Err(crate::rules::ReductionError::InvalidTarget { .. })
    ));
}
