use super::*;

#[test]
fn test_simple_path_alpha_tensor() {
    // Path graph: 0-1-2, all weight 1, pins = [0, 2]
    let edges = vec![(0, 1), (1, 2)];
    let weights = vec![1, 1, 1];
    let pins = vec![0, 2];

    let tensor = compute_alpha_tensor(3, &edges, &weights, &pins);

    // Config 0b00: neither pin in IS -> MIS can include vertex 1 -> MIS = 1
    // Config 0b01: pin 0 (vertex 0) in -> vertex 1 blocked -> MIS = 1
    // Config 0b10: pin 1 (vertex 2) in -> vertex 1 blocked -> MIS = 1
    // Config 0b11: both pins in -> vertices 0,2 in IS, vertex 1 blocked -> MIS = 2
    assert_eq!(tensor, vec![1, 1, 1, 2]);
}

#[test]
fn test_triangle_alpha_tensor() {
    // Triangle: 0-1, 1-2, 0-2, all weight 1, pins = [0, 1, 2]
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let weights = vec![1, 1, 1];
    let pins = vec![0, 1, 2];

    let tensor = compute_alpha_tensor(3, &edges, &weights, &pins);

    // When all vertices are pins:
    // 0b000: all pins forced OUT -> no vertices available -> MIS = 0
    // 0b001: vertex 0 in, others forced out -> MIS = 1
    // 0b010: vertex 1 in, others forced out -> MIS = 1
    // 0b011: vertices 0,1 in -> INVALID (adjacent) -> i32::MIN
    // 0b100: vertex 2 in, others forced out -> MIS = 1
    // 0b101: vertices 0,2 in -> INVALID (adjacent) -> i32::MIN
    // 0b110: vertices 1,2 in -> INVALID (adjacent) -> i32::MIN
    // 0b111: all in -> INVALID (all adjacent) -> i32::MIN
    assert_eq!(
        tensor,
        vec![0, 1, 1, i32::MIN, 1, i32::MIN, i32::MIN, i32::MIN]
    );
}

#[test]
fn test_mis_compactify_simple() {
    // From path graph test
    let mut tensor = vec![1, 1, 1, 2];
    mis_compactify(&mut tensor);

    // Entry 0b00 (val=1): is it dominated?
    // - By 0b01 (val=1)? (0b01 & 0b00) == 0b00 != 0b01, NO
    // - By 0b10 (val=1)? (0b10 & 0b00) == 0b00 != 0b10, NO
    // - By 0b11 (val=2)? (0b11 & 0b00) == 0b00 != 0b11, NO
    // Entry 0b01 (val=1):
    // - By 0b11 (val=2)? (0b11 & 0b01) == 0b01, but val=1 <= val=2, YES dominated
    // Entry 0b10 (val=1):
    // - By 0b11 (val=2)? (0b11 & 0b10) == 0b10, but val=1 <= val=2, YES dominated

    // After compactify: entries 0b01 and 0b10 should be i32::MIN
    assert_eq!(tensor[0], 1); // 0b00 not dominated
    assert_eq!(tensor[1], i32::MIN); // 0b01 dominated by 0b11
    assert_eq!(tensor[2], i32::MIN); // 0b10 dominated by 0b11
    assert_eq!(tensor[3], 2); // 0b11 not dominated
}

#[test]
fn test_is_diff_by_const() {
    let t1 = vec![3, i32::MIN, i32::MIN, 5];
    let t2 = vec![2, i32::MIN, i32::MIN, 4];

    let (is_equiv, diff) = is_diff_by_const(&t1, &t2);
    assert!(is_equiv);
    assert_eq!(diff, 1); // 3-2 = 1, 5-4 = 1

    let t3 = vec![3, i32::MIN, i32::MIN, 6];
    let (is_equiv2, _) = is_diff_by_const(&t1, &t3);
    assert!(!is_equiv2); // 3-3=0, 5-6=-1, not constant
}

#[test]
fn test_weighted_mis_exhaustive() {
    // Path: 0-1-2, weights [3, 1, 3]
    let edges = vec![(0, 1), (1, 2)];
    let weights = vec![3, 1, 3];

    let mis = weighted_mis_exhaustive(3, &edges, &weights);
    assert_eq!(mis, 6); // Select vertices 0 and 2
}

#[test]
fn test_triangular_unit_disk_edges() {
    // Simple case: two adjacent nodes on triangular lattice
    // Nodes at (1, 1) and (1, 2) should be connected (distance ~0.866)
    let locs = vec![(1, 1), (1, 2)];
    let edges = build_triangular_unit_disk_edges(&locs);
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0], (0, 1));

    // Nodes at (1, 1) and (3, 1) should NOT be connected (distance = 2)
    let locs2 = vec![(1, 1), (3, 1)];
    let edges2 = build_triangular_unit_disk_edges(&locs2);
    assert_eq!(edges2.len(), 0);
}

#[test]
fn test_verify_tri_turn() {
    use super::super::triangular::TriTurn;

    let gadget = TriTurn;
    let result = verify_triangular_gadget(&gadget);
    assert!(result.is_ok(), "TriTurn verification failed: {:?}", result);
}

#[test]
fn test_verify_tri_cross_false() {
    use super::super::triangular::TriCross;

    let gadget = TriCross::<false>;
    let result = verify_triangular_gadget(&gadget);
    assert!(
        result.is_ok(),
        "TriCross<false> verification failed: {:?}",
        result
    );
}

#[test]
fn test_verify_tri_cross_true() {
    use super::super::triangular::TriCross;

    let gadget = TriCross::<true>;
    let result = verify_triangular_gadget(&gadget);
    assert!(
        result.is_ok(),
        "TriCross<true> verification failed: {:?}",
        result
    );
}

#[test]
fn test_verify_tri_branch() {
    use super::super::triangular::TriBranch;

    let gadget = TriBranch;
    let result = verify_triangular_gadget(&gadget);
    assert!(
        result.is_ok(),
        "TriBranch verification failed: {:?}",
        result
    );
}

#[test]
fn test_verify_tri_tcon_left() {
    use super::super::triangular::TriTConLeft;

    let gadget = TriTConLeft;
    let result = verify_triangular_gadget(&gadget);
    assert!(
        result.is_ok(),
        "TriTConLeft verification failed: {:?}",
        result
    );
}
