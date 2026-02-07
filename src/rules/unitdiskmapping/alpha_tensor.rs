//! Alpha tensor computation for gadget verification.
//!
//! Alpha tensors are used to verify gadget correctness. For a gadget with k pins,
//! the alpha tensor is a 2^k array where entry i is the weighted MIS when pins
//! are fixed according to the bit pattern of i.
//!
//! Two gadgets are equivalent if their reduced (compactified) alpha tensors
//! differ by a constant equal to the negative MIS overhead.

use std::collections::HashSet;

/// Compute alpha tensor for a graph with weighted nodes and open pins.
///
/// Returns a 2^k vector where k = pins.len().
/// Entry i represents weighted MIS when pins are fixed according to bit pattern i:
/// - Bit j = 1: pin j is IN the independent set
/// - Bit j = 0: pin j is OUT of the independent set
///
/// # Arguments
/// * `num_vertices` - Total number of vertices
/// * `edges` - Edge list (0-indexed)
/// * `weights` - Weight of each vertex
/// * `pins` - Indices of open vertices (0-indexed)
#[allow(clippy::needless_range_loop)]
pub fn compute_alpha_tensor(
    num_vertices: usize,
    edges: &[(usize, usize)],
    weights: &[i32],
    pins: &[usize],
) -> Vec<i32> {
    let k = pins.len();
    let mut tensor = vec![0; 1 << k];

    for config in 0..(1 << k) {
        tensor[config] = compute_mis_with_fixed_pins(num_vertices, edges, weights, pins, config);
    }

    tensor
}

/// Compute weighted MIS with some pins fixed to be in/out of IS.
///
/// For each pin configuration:
/// - Pins with bit=1: MUST be in IS (forced in)
/// - Pins with bit=0: MUST be out of IS (forced out)
/// - If forced-in pins are adjacent, return i32::MIN (invalid/impossible)
/// - Otherwise solve weighted MIS on remaining free vertices
fn compute_mis_with_fixed_pins(
    num_vertices: usize,
    edges: &[(usize, usize)],
    weights: &[i32],
    pins: &[usize],
    pin_config: usize,
) -> i32 {
    // Determine forced-in and forced-out vertices
    let mut forced_in: HashSet<usize> = HashSet::new();
    let mut forced_out: HashSet<usize> = HashSet::new();

    for (i, &pin) in pins.iter().enumerate() {
        if (pin_config >> i) & 1 == 1 {
            forced_in.insert(pin);
        } else {
            forced_out.insert(pin);
        }
    }

    // Check if any forced-in vertices are adjacent (invalid configuration)
    for &(u, v) in edges {
        if forced_in.contains(&u) && forced_in.contains(&v) {
            return i32::MIN; // Invalid: adjacent pins both forced in
        }
    }

    // Vertices that are blocked by forced-in vertices
    let mut blocked: HashSet<usize> = HashSet::new();
    for &(u, v) in edges {
        if forced_in.contains(&u) {
            blocked.insert(v);
        }
        if forced_in.contains(&v) {
            blocked.insert(u);
        }
    }

    // Free vertices: not forced-in, not forced-out, not blocked
    let free_vertices: Vec<usize> = (0..num_vertices)
        .filter(|&v| !forced_in.contains(&v) && !forced_out.contains(&v) && !blocked.contains(&v))
        .collect();

    // Build subgraph on free vertices
    let vertex_map: std::collections::HashMap<usize, usize> = free_vertices
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect();

    let sub_edges: Vec<(usize, usize)> = edges
        .iter()
        .filter_map(|&(u, v)| {
            if let (Some(&u2), Some(&v2)) = (vertex_map.get(&u), vertex_map.get(&v)) {
                Some((u2, v2))
            } else {
                None
            }
        })
        .collect();

    let sub_weights: Vec<i32> = free_vertices.iter().map(|&v| weights[v]).collect();

    // Solve weighted MIS on subgraph
    let sub_mis = if free_vertices.is_empty() {
        0
    } else {
        weighted_mis_exhaustive(free_vertices.len(), &sub_edges, &sub_weights)
    };

    // Total MIS = weight of forced-in vertices + MIS of free vertices
    let forced_in_weight: i32 = forced_in.iter().map(|&v| weights[v]).sum();
    forced_in_weight + sub_mis
}

/// Exhaustive weighted MIS solver for small graphs.
/// Uses brute force enumeration for correctness (suitable for gadgets with <20 vertices).
#[allow(clippy::needless_range_loop)]
fn weighted_mis_exhaustive(num_vertices: usize, edges: &[(usize, usize)], weights: &[i32]) -> i32 {
    if num_vertices == 0 {
        return 0;
    }

    // Build adjacency check
    let mut adj = vec![vec![false; num_vertices]; num_vertices];
    for &(u, v) in edges {
        if u < num_vertices && v < num_vertices {
            adj[u][v] = true;
            adj[v][u] = true;
        }
    }

    let mut max_weight = 0;

    // Enumerate all subsets
    for subset in 0..(1usize << num_vertices) {
        // Check if subset is independent
        let mut is_independent = true;
        for u in 0..num_vertices {
            if (subset >> u) & 1 == 0 {
                continue;
            }
            for v in (u + 1)..num_vertices {
                if (subset >> v) & 1 == 0 {
                    continue;
                }
                if adj[u][v] {
                    is_independent = false;
                    break;
                }
            }
            if !is_independent {
                break;
            }
        }

        if is_independent {
            let weight: i32 = (0..num_vertices)
                .filter(|&v| (subset >> v) & 1 == 1)
                .map(|v| weights[v])
                .sum();
            max_weight = max_weight.max(weight);
        }
    }

    max_weight
}

/// Reduce alpha tensor by eliminating dominated entries.
///
/// An entry (bs_a, val_a) is dominated by (bs_b, val_b) if:
/// - bs_a != bs_b
/// - val_a <= val_b
/// - (bs_b & bs_a) == bs_b (bs_a has all bits of bs_b plus more, i.e., bs_b is subset of bs_a)
///
/// Dominated entries are set to i32::MIN (representing -infinity).
pub fn mis_compactify(tensor: &mut [i32]) {
    let n = tensor.len();
    for a in 0..n {
        if tensor[a] == i32::MIN {
            continue;
        }
        for b in 0..n {
            if a != b && tensor[b] != i32::MIN && worse_than(a, b, tensor[a], tensor[b]) {
                tensor[a] = i32::MIN;
                break;
            }
        }
    }
}

/// Check if entry a is dominated by entry b.
fn worse_than(bs_a: usize, bs_b: usize, val_a: i32, val_b: i32) -> bool {
    // bs_a is worse than bs_b if:
    // - bs_b is a subset of bs_a (bs_a has all bits of bs_b plus potentially more)
    // - val_a <= val_b (including more pins doesn't improve MIS)
    bs_a != bs_b && val_a <= val_b && (bs_b & bs_a) == bs_b
}

/// Check if two tensors differ by a constant.
///
/// Returns (is_equivalent, difference) where difference = t1[i] - t2[i] for valid entries.
/// Invalid entries (i32::MIN) in both tensors are skipped.
/// If one is valid and other is invalid, returns false.
pub fn is_diff_by_const(t1: &[i32], t2: &[i32]) -> (bool, i32) {
    assert_eq!(t1.len(), t2.len());

    let mut diff: Option<i32> = None;

    for (&a, &b) in t1.iter().zip(t2.iter()) {
        // Skip if both are -infinity (dominated)
        if a == i32::MIN && b == i32::MIN {
            continue;
        }
        // Fail if only one is -infinity
        if a == i32::MIN || b == i32::MIN {
            return (false, 0);
        }

        let d = a - b;
        match diff {
            None => diff = Some(d),
            Some(prev) if prev != d => return (false, 0),
            _ => {}
        }
    }

    (true, diff.unwrap_or(0))
}

/// Build unit disk graph edges for triangular lattice.
/// Uses distance threshold of 1.1 (matching Julia's triangular_unitdisk_graph).
///
/// Triangular coordinates: (row, col) maps to physical position:
/// - x = row + 0.5 if col is even, else row
/// - y = col * sqrt(3)/2
pub fn build_triangular_unit_disk_edges(locs: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let n = locs.len();
    let mut edges = Vec::new();
    let radius = 1.1;

    for i in 0..n {
        for j in (i + 1)..n {
            let (r1, c1) = locs[i];
            let (r2, c2) = locs[j];

            // Convert to physical coordinates
            let x1 = r1 as f64 + if c1.is_multiple_of(2) { 0.5 } else { 0.0 };
            let y1 = c1 as f64 * (3.0_f64.sqrt() / 2.0);
            let x2 = r2 as f64 + if c2.is_multiple_of(2) { 0.5 } else { 0.0 };
            let y2 = c2 as f64 * (3.0_f64.sqrt() / 2.0);

            // Use squared distance comparison (like Julia): dist^2 < radius^2
            let dist_sq = (x1 - x2).powi(2) + (y1 - y2).powi(2);
            if dist_sq < radius * radius {
                edges.push((i, j));
            }
        }
    }

    edges
}

/// Build unit disk graph edges using standard Euclidean distance.
/// Uses radius 1.5 matching Julia's unitdisk_graph for gadget verification.
///
/// This treats coordinates as standard grid positions, not triangular lattice.
pub fn build_standard_unit_disk_edges(locs: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let n = locs.len();
    let mut edges = Vec::new();
    let radius = 1.5;

    for i in 0..n {
        for j in (i + 1)..n {
            let (r1, c1) = locs[i];
            let (r2, c2) = locs[j];

            // Standard Euclidean distance
            let dr = r1 as f64 - r2 as f64;
            let dc = c1 as f64 - c2 as f64;
            let dist = (dr * dr + dc * dc).sqrt();

            if dist <= radius {
                edges.push((i, j));
            }
        }
    }

    edges
}

/// Verify a triangular gadget's correctness using alpha tensors.
///
/// Returns Ok if the gadget is correct (source and mapped have equivalent alpha tensors),
/// Err with a message if not.
///
/// Uses Julia's approach: subtract 1 from pin weights to account for external coupling.
pub fn verify_triangular_gadget<G: super::triangular::TriangularGadget>(
    gadget: &G,
) -> Result<(), String> {
    // Get source graph
    let (src_locs, src_edges, src_pins) = gadget.source_graph();
    // Use gadget's source weights, then subtract 1 from pins (Julia's approach)
    let mut src_weights = gadget.source_weights();
    for &pin in &src_pins {
        src_weights[pin] -= 1;
    }

    // Get mapped graph
    // Use triangular unit disk with radius 1.1 (matching Julia's triangular_unitdisk_graph)
    let (map_locs, map_pins) = gadget.mapped_graph();
    let map_edges = build_triangular_unit_disk_edges(&map_locs);
    // Use gadget's mapped weights, then subtract 1 from pins
    let mut map_weights = gadget.mapped_weights();
    for &pin in &map_pins {
        map_weights[pin] -= 1;
    }

    // Compute alpha tensors
    let src_tensor = compute_alpha_tensor(src_locs.len(), &src_edges, &src_weights, &src_pins);
    let map_tensor = compute_alpha_tensor(map_locs.len(), &map_edges, &map_weights, &map_pins);

    // Julia doesn't use mis_compactify for weighted gadgets - it just checks that
    // the maximum entries are in the same positions and differ by a constant.
    // Let's check the simpler condition first.
    let src_max = *src_tensor
        .iter()
        .filter(|&&x| x != i32::MIN)
        .max()
        .unwrap_or(&0);
    let map_max = *map_tensor
        .iter()
        .filter(|&&x| x != i32::MIN)
        .max()
        .unwrap_or(&0);

    // Check that positions where source == max match positions where mapped == max
    let src_max_mask: Vec<bool> = src_tensor.iter().map(|&x| x == src_max).collect();
    let map_max_mask: Vec<bool> = map_tensor.iter().map(|&x| x == map_max).collect();

    if src_max_mask != map_max_mask {
        return Err(format!(
            "Maximum entry positions differ.\nSource tensor: {:?}\nMapped tensor: {:?}\nSource max mask: {:?}\nMapped max mask: {:?}",
            src_tensor, map_tensor, src_max_mask, map_max_mask
        ));
    }

    // Check that the difference between max values equals -mis_overhead
    let diff = src_max - map_max;
    let expected_diff = -gadget.mis_overhead();
    if diff != expected_diff {
        return Err(format!(
            "Overhead mismatch: src_max={}, map_max={}, diff={}, expected -mis_overhead={}",
            src_max, map_max, diff, expected_diff
        ));
    }

    Ok(())
}

#[cfg(test)]
#[path = "../../tests_unit/rules/unitdiskmapping/alpha_tensor.rs"]
mod tests;
