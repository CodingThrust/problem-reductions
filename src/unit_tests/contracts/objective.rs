use crate::core::ObjectiveDirection;
use crate::types::EnergyMode;

#[test]
fn test_objective_direction_ordering() {
    let max = ObjectiveDirection::Maximize;
    let min = ObjectiveDirection::Minimize;

    assert!(max.is_better(&10, &5));
    assert!(!max.is_better(&5, &10));
    assert!(min.is_better(&5, &10));
    assert!(!min.is_better(&10, &5));

    assert!(max.is_better_or_equal(&10, &10));
    assert!(min.is_better_or_equal(&10, &10));
}

#[test]
fn test_objective_direction_energy_mode_conversion() {
    let max_mode: EnergyMode = ObjectiveDirection::Maximize.into();
    let min_mode: EnergyMode = ObjectiveDirection::Minimize.into();

    assert_eq!(
        ObjectiveDirection::from(max_mode),
        ObjectiveDirection::Maximize
    );
    assert_eq!(
        ObjectiveDirection::from(min_mode),
        ObjectiveDirection::Minimize
    );
}
