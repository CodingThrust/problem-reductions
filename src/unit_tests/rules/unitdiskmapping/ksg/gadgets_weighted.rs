use super::*;

#[test]
fn test_weighted_ksg_cross_false_mis_overhead() {
    assert_eq!(WeightedKsgCross::<false>.mis_overhead(), -2);
}

#[test]
fn test_weighted_ksg_cross_true_mis_overhead() {
    assert_eq!(WeightedKsgCross::<true>.mis_overhead(), -2);
}

#[test]
fn test_weighted_ksg_turn_mis_overhead() {
    assert_eq!(WeightedKsgTurn.mis_overhead(), -2);
}

#[test]
fn test_weighted_ksg_branch_weights() {
    let branch = WeightedKsgBranch;
    assert_eq!(branch.source_weights(), vec![2, 2, 2, 3, 2, 2, 2, 2]);
    assert_eq!(branch.mapped_weights(), vec![2, 3, 2, 2, 2, 2]);
}

#[test]
fn test_weighted_ksg_tcon_weights() {
    let tcon = WeightedKsgTCon;
    assert_eq!(tcon.source_weights(), vec![2, 1, 2, 2]);
    assert_eq!(tcon.mapped_weights(), vec![2, 1, 2, 2]);
}

#[test]
fn test_weighted_ksg_trivial_turn_weights() {
    let turn = WeightedKsgTrivialTurn;
    assert_eq!(turn.source_weights(), vec![1, 1]);
    assert_eq!(turn.mapped_weights(), vec![1, 1]);
}

#[test]
fn test_weighted_ksg_pattern_from_tape_idx() {
    assert!(WeightedKsgPattern::from_tape_idx(0).is_some());
    assert!(WeightedKsgPattern::from_tape_idx(12).is_some());
    assert!(WeightedKsgPattern::from_tape_idx(100).is_some());
    assert!(WeightedKsgPattern::from_tape_idx(200).is_none());
}
