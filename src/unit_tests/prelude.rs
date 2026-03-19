use crate::prelude::*;

#[test]
fn test_prelude_exports_rectilinear_picture_compression() {
    let problem = RectilinearPictureCompression::new(vec![vec![true]], 1);
    assert_eq!(problem.bound_k(), 1);
}
