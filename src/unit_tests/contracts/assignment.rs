use crate::core::Assignment;
use crate::error::ProblemError;

#[test]
fn test_assignment_validates_shape_and_flavors() {
    let assignment = Assignment::from(vec![0, 1, 1, 0]);
    assert!(assignment.validate(4, 2).is_ok());
    assert!(assignment.is_valid(4, 2));
}

#[test]
fn test_assignment_reports_invalid_size() {
    let assignment = Assignment::from(vec![0, 1, 1]);
    let err = assignment.validate(4, 2).unwrap_err();
    assert_eq!(
        err,
        ProblemError::InvalidConfigSize {
            expected: 4,
            got: 3
        }
    );
}

#[test]
fn test_assignment_reports_invalid_flavor() {
    let assignment = Assignment::from(vec![0, 1, 2, 0]);
    let err = assignment.validate(4, 2).unwrap_err();
    assert_eq!(
        err,
        ProblemError::InvalidFlavor {
            index: 2,
            value: 2,
            num_flavors: 2
        }
    );
}
