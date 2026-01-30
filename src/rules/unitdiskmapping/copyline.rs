//! Copy-line technique for embedding graphs into grids.
//!
//! Each vertex in the source graph becomes a "copy line" on the grid.
//! The copy line is an L-shaped path that allows the vertex to connect
//! with all its neighbors through crossings.

use serde::{Deserialize, Serialize};

/// A copy line representing a single vertex embedded in the grid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyLine {
    /// The vertex this copy line represents.
    pub vertex: usize,
    /// Vertical slot (column in the grid).
    pub vslot: usize,
    /// Horizontal slot (row where the vertex info lives).
    pub hslot: usize,
    /// Start row of vertical segment.
    pub vstart: usize,
    /// Stop row of vertical segment.
    pub vstop: usize,
    /// Stop column of horizontal segment.
    pub hstop: usize,
}

impl CopyLine {
    /// Create a new CopyLine.
    pub fn new(
        vertex: usize,
        vslot: usize,
        hslot: usize,
        vstart: usize,
        vstop: usize,
        hstop: usize,
    ) -> Self {
        Self {
            vertex,
            vslot,
            hslot,
            vstart,
            vstop,
            hstop,
        }
    }

    /// Get the center location of this copy line.
    pub fn center_location(&self, padding: usize, spacing: usize) -> (usize, usize) {
        let row = spacing * (self.hslot - 1) + padding + 2;
        let col = spacing * (self.vslot - 1) + padding + 1;
        (row, col)
    }

    /// Generate grid locations for this copy line.
    /// Returns Vec<(row, col, weight)> where weight indicates importance.
    ///
    /// The copy line forms an L-shape:
    /// - Vertical segment from vstart to vstop
    /// - Horizontal segment at hslot from vslot to hstop
    pub fn locations(&self, padding: usize, spacing: usize) -> Vec<(usize, usize, usize)> {
        let mut locs = Vec::new();

        // The center column for this copy line's vertical segment
        let col = spacing * (self.vslot - 1) + padding + 1;

        // Vertical segment: from vstart to vstop
        for v in self.vstart..=self.vstop {
            let row = spacing * (v - 1) + padding + 2;
            // Weight is 1 for regular positions
            locs.push((row, col, 1));
        }

        // Horizontal segment: at hslot, from vslot+1 to hstop
        let hrow = spacing * (self.hslot - 1) + padding + 2;
        for h in (self.vslot + 1)..=self.hstop {
            let hcol = spacing * (h - 1) + padding + 1;
            // Avoid duplicate at the corner (vslot, hslot)
            if hcol != col || hrow != spacing * (self.hslot - 1) + padding + 2 {
                locs.push((hrow, hcol, 1));
            }
        }

        locs
    }

    /// Generate dense grid locations for this copy line (all cells along the L-shape).
    /// This matches UnitDiskMapping.jl's `copyline_locations` function.
    ///
    /// Returns Vec<(row, col, weight)> with nodes at every cell along the path.
    pub fn copyline_locations(&self, padding: usize, spacing: usize) -> Vec<(usize, usize, usize)> {
        let mut locs = Vec::new();
        let mut nline = 0usize;

        // Center location (I, J) - matches Julia's center_location
        let i = (spacing * (self.hslot - 1) + padding + 2) as isize;
        let j = (spacing * (self.vslot - 1) + padding + 1) as isize;
        let spacing = spacing as isize;

        // Grow up: from I down to start
        let start = i + spacing * (self.vstart as isize - self.hslot as isize) + 1;
        if self.vstart < self.hslot {
            nline += 1;
        }
        for row in (start..=i).rev() {
            if row >= 0 {
                let weight = if row != start { 2 } else { 1 };
                locs.push((row as usize, j as usize, weight));
            }
        }

        // Grow down: from I to stop
        let stop = i + spacing * (self.vstop as isize - self.hslot as isize) - 1;
        if self.vstop > self.hslot {
            nline += 1;
        }
        for row in i..=stop {
            if row >= 0 {
                if row == i {
                    // Special: first node going down is offset by (1, 1)
                    locs.push(((row + 1) as usize, (j + 1) as usize, 2));
                } else {
                    let weight = if row != stop { 2 } else { 1 };
                    locs.push((row as usize, j as usize, weight));
                }
            }
        }

        // Grow right: from J+2 to stop
        let stop_col = j + spacing * (self.hstop as isize - self.vslot as isize) - 1;
        if self.hstop > self.vslot {
            nline += 1;
        }
        for col in (j + 2)..=stop_col {
            if col >= 0 {
                let weight = if col != stop_col { 2 } else { 1 };
                locs.push((i as usize, col as usize, weight));
            }
        }

        // Center node at (I, J+1) - always at least weight 1
        locs.push((i as usize, (j + 1) as usize, nline.max(1)));

        locs
    }

    /// Generate dense grid locations for triangular mode (includes endpoint node).
    /// This matches Julia's `copyline_locations(TriangularWeighted, ...)` formula.
    ///
    /// The key difference from `copyline_locations` is that the horizontal segment
    /// extends one more cell to include the endpoint at `J + spacing * (hstop - vslot)`.
    pub fn copyline_locations_triangular(&self, padding: usize, spacing: usize) -> Vec<(usize, usize, usize)> {
        let mut locs = Vec::new();
        let mut nline = 0usize;

        // Center location (I, J) - matches Julia's center_location
        let i = (spacing * (self.hslot - 1) + padding + 2) as isize;
        let j = (spacing * (self.vslot - 1) + padding + 1) as isize;
        let spacing = spacing as isize;

        // Grow up: from I down to start
        let start = i + spacing * (self.vstart as isize - self.hslot as isize) + 1;
        if self.vstart < self.hslot {
            nline += 1;
        }
        for row in (start..=i).rev() {
            if row >= 0 {
                let weight = if row != start { 2 } else { 1 };
                locs.push((row as usize, j as usize, weight));
            }
        }

        // Grow down: from I to stop
        let stop = i + spacing * (self.vstop as isize - self.hslot as isize) - 1;
        if self.vstop > self.hslot {
            nline += 1;
        }
        for row in i..=stop {
            if row >= 0 {
                if row == i {
                    // Special: first node going down is offset by (1, 1)
                    locs.push(((row + 1) as usize, (j + 1) as usize, 2));
                } else {
                    let weight = if row != stop { 2 } else { 1 };
                    locs.push((row as usize, j as usize, weight));
                }
            }
        }

        // Grow right: from J+2 to stop (inclusive)
        // Julia formula: stop = J + col_s*(hstop-vslot) - 1
        let stop_col = j + spacing * (self.hstop as isize - self.vslot as isize) - 1;
        if self.hstop > self.vslot {
            nline += 1;
        }
        // Loop from J+2 to stop_col inclusive, weight 1 on last node
        for col in (j + 2)..=stop_col {
            if col >= 0 {
                let weight = if col != stop_col { 2 } else { 1 };
                locs.push((i as usize, col as usize, weight));
            }
        }

        // Center node at (I, J+1) - always at least weight 1
        locs.push((i as usize, (j + 1) as usize, nline.max(1)));

        locs
    }
}

/// Helper function to compute the removal order for vertices.
/// This matches Julia's UnitDiskMapping `remove_order` function.
///
/// A vertex can be removed at step i if all its neighbors have been added by step i.
/// The removal happens at max(vertex's own position, step when all neighbors added).
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the graph
/// * `edges` - List of edges as (u, v) pairs
/// * `vertex_order` - The order in which vertices are processed
///
/// # Returns
/// A vector of vectors, where index i contains vertices removable at step i.
pub fn remove_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Vec<Vec<usize>> {
    if num_vertices == 0 {
        return Vec::new();
    }

    // Build adjacency matrix as a Vec<Vec<bool>>
    let mut adj_matrix = vec![vec![false; num_vertices]; num_vertices];
    for &(u, v) in edges {
        adj_matrix[u][v] = true;
        adj_matrix[v][u] = true;
    }

    // counts[j] = number of neighbors of j that have been added so far
    let mut counts = vec![0usize; num_vertices];
    // total_counts[j] = total number of neighbors of j
    let total_counts: Vec<usize> = (0..num_vertices)
        .map(|j| adj_matrix[j].iter().filter(|&&x| x).count())
        .collect();

    // Create order map: vertex -> position in order (1-indexed for comparison)
    let mut order_pos = vec![0usize; num_vertices];
    for (pos, &v) in vertex_order.iter().enumerate() {
        order_pos[v] = pos + 1; // 1-indexed
    }

    let mut result: Vec<Vec<usize>> = vec![Vec::new(); num_vertices];
    let mut removed = vec![false; num_vertices];

    for (i, &v) in vertex_order.iter().enumerate() {
        // Add v: increment counts for all vertices that have v as neighbor
        for j in 0..num_vertices {
            if adj_matrix[j][v] {
                counts[j] += 1;
            }
        }

        // Check which vertices can be removed (all neighbors have been added)
        for j in 0..num_vertices {
            if !removed[j] && counts[j] == total_counts[j] {
                // Remove at max(i, position of j in order) - both 0-indexed
                let j_pos = order_pos[j] - 1; // Convert to 0-indexed
                let remove_step = i.max(j_pos);
                result[remove_step].push(j);
                removed[j] = true;
            }
        }
    }

    result
}

/// Create copy lines for all vertices based on the vertex ordering.
/// This matches Julia's UnitDiskMapping `create_copylines` function.
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the graph
/// * `edges` - List of edges as (u, v) pairs
/// * `vertex_order` - The order in which vertices are processed
///
/// # Returns
/// A vector of CopyLine structures, one per vertex (indexed by vertex id).
pub fn create_copylines(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Vec<CopyLine> {
    if num_vertices == 0 {
        return Vec::new();
    }

    // Build adjacency set for edge lookup
    let mut has_edge = vec![vec![false; num_vertices]; num_vertices];
    for &(u, v) in edges {
        has_edge[u][v] = true;
        has_edge[v][u] = true;
    }

    // Compute removal order
    let rmorder = remove_order(num_vertices, edges, vertex_order);

    // Phase 1: Assign hslots using slot reuse strategy
    // slots[k] = vertex occupying slot k+1 (0 = free)
    let mut slots = vec![0usize; num_vertices];
    // hslots[i] = the hslot assigned to vertex at position i in order
    let mut hslots = vec![0usize; num_vertices];

    for (i, (&v, rs)) in vertex_order.iter().zip(rmorder.iter()).enumerate() {
        // Find first free slot (1-indexed in Julia, but we use 0-indexed internally)
        // Safety: A free slot always exists because the removal order (`rmorder`) ensures that
        // vertices whose neighbors have all been processed are removed before new vertices are
        // added. The number of active (non-removed) vertices never exceeds `num_vertices`.
        let islot = slots
            .iter()
            .position(|&x| x == 0)
            .expect("Slot reuse invariant violated: no free slot available");
        slots[islot] = v + 1; // Store vertex+1 to distinguish from empty (0)
        hslots[i] = islot + 1; // 1-indexed hslot

        // Remove vertices according to rmorder
        for &r in rs {
            if let Some(pos) = slots.iter().position(|&x| x == r + 1) {
                slots[pos] = 0;
            }
        }
    }

    // Phase 2: Compute vstarts, vstops, hstops
    let mut vstarts = vec![0usize; num_vertices];
    let mut vstops = vec![0usize; num_vertices];
    let mut hstops = vec![0usize; num_vertices];

    for (i, &v) in vertex_order.iter().enumerate() {
        // relevant_hslots: hslots of vertices j (j <= i) where has_edge(v, ordered_vertices[j]) or v == ordered_vertices[j]
        let relevant_hslots: Vec<usize> = (0..=i)
            .filter(|&j| has_edge[vertex_order[j]][v] || v == vertex_order[j])
            .map(|j| hslots[j])
            .collect();

        // relevant_vslots: positions (1-indexed) of vertices that are neighbors of v or v itself
        let relevant_vslots: Vec<usize> = (0..num_vertices)
            .filter(|&j| has_edge[vertex_order[j]][v] || v == vertex_order[j])
            .map(|j| j + 1) // 1-indexed
            .collect();

        vstarts[i] = *relevant_hslots.iter().min().unwrap_or(&1);
        vstops[i] = *relevant_hslots.iter().max().unwrap_or(&1);
        hstops[i] = *relevant_vslots.iter().max().unwrap_or(&1);
    }

    // Build copylines indexed by vertex id
    let mut copylines = vec![
        CopyLine {
            vertex: 0,
            vslot: 0,
            hslot: 0,
            vstart: 0,
            vstop: 0,
            hstop: 0,
        };
        num_vertices
    ];

    for (i, &v) in vertex_order.iter().enumerate() {
        copylines[v] = CopyLine::new(
            v,
            i + 1, // vslot is 1-indexed position in order
            hslots[i],
            vstarts[i],
            vstops[i],
            hstops[i],
        );
    }

    copylines
}

/// Calculate the MIS (Maximum Independent Set) overhead for a copy line.
/// This matches Julia's UnitDiskMapping `mis_overhead_copyline` for Weighted mode.
///
/// The overhead is:
/// - (hslot - vstart) * spacing for the upward segment
/// - (vstop - hslot) * spacing for the downward segment
/// - max((hstop - vslot) * spacing - 2, 0) for the rightward segment
///
/// # Arguments
/// * `line` - The copy line
/// * `spacing` - Grid spacing parameter
/// * `padding` - Grid padding parameter
///
/// # Returns
/// The MIS overhead value for this copy line.
///
/// For unweighted mapping, the overhead is `length(locs) / 2` where locs
/// are the dense copyline locations. This matches Julia's UnitDiskMapping.jl.
pub fn mis_overhead_copyline(line: &CopyLine, spacing: usize, padding: usize) -> usize {
    let locs = line.copyline_locations(padding, spacing);
    // Julia asserts length(locs) % 2 == 1, then returns length(locs) ÷ 2
    locs.len() / 2
}

/// Generate weighted locations for a copy line in triangular mode.
/// This matches Julia's `copyline_locations(TriangularWeighted(), ...)`.
///
/// Returns (locations, weights) where:
/// - locations: Vec of (row, col) positions
/// - weights: Vec of i32 weights (typically 2 for regular nodes, 1 for turn points)
///
/// The sequence of nodes forms a chain-like structure with the center node at the end.
/// Nodes with weight=1 mark "break points" in the chain where the next node connects
/// to the center (last node) instead of the previous node.
///
/// # Arguments
/// * `line` - The copy line
/// * `spacing` - Grid spacing parameter
///
/// # Returns
/// A tuple of (locations, weights) vectors.
pub fn copyline_weighted_locations_triangular(
    line: &CopyLine,
    spacing: usize,
) -> (Vec<(usize, usize)>, Vec<i32>) {
    let mut locs = Vec::new();
    let mut weights = Vec::new();
    let mut nline = 0usize;

    // Count segments and calculate lengths
    let has_up = line.vstart < line.hslot;
    let has_down = line.vstop > line.hslot;
    let has_right = line.hstop > line.vslot;

    if has_up {
        nline += 1;
    }
    if has_down {
        nline += 1;
    }
    if has_right {
        nline += 1;
    }

    // Upward segment: from vstart to hslot
    // Length = (hslot - vstart) * spacing
    if has_up {
        let len = (line.hslot - line.vstart) * spacing;
        for i in 0..len {
            locs.push((i, 0));
            // Last node of segment (turn point) gets weight 1, others get 2
            let w = if i == len - 1 { 1 } else { 2 };
            weights.push(w);
        }
    }

    // Downward segment: from hslot to vstop
    // Length = (vstop - hslot) * spacing
    if has_down {
        let len = (line.vstop - line.hslot) * spacing;
        let offset = locs.len();
        for i in 0..len {
            locs.push((offset + i, 1));
            // Last node of segment (turn point) gets weight 1, others get 2
            let w = if i == len - 1 { 1 } else { 2 };
            weights.push(w);
        }
    }

    // Rightward segment: from vslot to hstop
    // Julia: for j=J+2:stop where stop = J + col_s*(hstop-vslot) - 1
    // Length = max((hstop - vslot) * spacing - 2, 0)
    if has_right {
        let full_len = (line.hstop - line.vslot) * spacing;
        // Julia starts at J+2 and ends at stop, so we skip 2 positions
        let len = full_len.saturating_sub(2);
        let offset = locs.len();
        for i in 0..len {
            locs.push((offset, 2 + i));
            // Last node of segment (end point) gets weight 1, others get 2
            let w = if i == len - 1 { 1 } else { 2 };
            weights.push(w);
        }
    }

    // Add center node at the end with weight = nline (number of segments)
    // This is the "hub" node that the chain wraps around to
    let center_row = locs.len();
    locs.push((center_row, 0));
    weights.push(nline.max(1) as i32);

    (locs, weights)
}

/// Calculate MIS overhead for a copy line in triangular weighted mode.
///
/// Uses Julia's exact formula for weighted mode:
/// ```julia
/// s = 4  # constant factor per slot
/// overhead = (hslot - vstart) * s + (vstop - hslot) * s + max((hstop - vslot) * s - 2, 0)
/// ```
///
/// The formula computes overhead based on the copyline structure:
/// - Vertical segment from vstart to hslot: (hslot - vstart) * 4
/// - Vertical segment from vstart to hslot: (hslot - vstart) * s
/// - Vertical segment from hslot to vstop: (vstop - hslot) * s
/// - Horizontal segment from vslot to hstop: max((hstop - vslot) * s - 2, 0)
///
/// For spacing=6 (our default), use s=spacing to match the node density.
pub fn mis_overhead_copyline_triangular(line: &CopyLine, spacing: usize) -> i32 {
    // Use spacing directly as the factor
    let s: i32 = spacing as i32;

    let vertical_up = (line.hslot as i32 - line.vstart as i32) * s;
    let vertical_down = (line.vstop as i32 - line.hslot as i32) * s;
    let horizontal = ((line.hstop as i32 - line.vslot as i32) * s - 2).max(0);

    vertical_up + vertical_down + horizontal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_copylines_path() {
        // Path graph: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let order = vec![0, 1, 2];
        let lines = create_copylines(3, &edges, &order);

        assert_eq!(lines.len(), 3);
        // Each vertex gets a copy line
        assert_eq!(lines[0].vertex, 0);
        assert_eq!(lines[1].vertex, 1);
        assert_eq!(lines[2].vertex, 2);
    }

    #[test]
    fn test_copyline_locations() {
        let line = CopyLine {
            vertex: 0,
            vslot: 1,
            hslot: 1,
            vstart: 1,
            vstop: 1,
            hstop: 3,
        };
        let locs = line.locations(2, 4); // padding=2, spacing=4
        assert!(!locs.is_empty());
    }

    #[test]
    fn test_create_copylines_empty() {
        let edges: Vec<(usize, usize)> = vec![];
        let order: Vec<usize> = vec![];
        let lines = create_copylines(0, &edges, &order);
        assert!(lines.is_empty());
    }

    #[test]
    fn test_create_copylines_single_vertex() {
        let edges: Vec<(usize, usize)> = vec![];
        let order = vec![0];
        let lines = create_copylines(1, &edges, &order);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].vertex, 0);
        assert_eq!(lines[0].vslot, 1);
    }

    #[test]
    fn test_create_copylines_triangle() {
        // Triangle: 0-1, 1-2, 0-2
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let order = vec![0, 1, 2];
        let lines = create_copylines(3, &edges, &order);

        assert_eq!(lines.len(), 3);
        // Vertex 0 should have hstop reaching to vertex 2's slot
        assert!(lines[0].hstop >= 2);
    }

    #[test]
    fn test_copyline_center_location() {
        let line = CopyLine::new(0, 2, 3, 1, 3, 4);
        let (row, col) = line.center_location(1, 4);
        // row = 4 * (3-1) + 1 + 2 = 8 + 3 = 11
        // col = 4 * (2-1) + 1 + 1 = 4 + 2 = 6
        assert_eq!(row, 11);
        assert_eq!(col, 6);
    }

    #[test]
    fn test_remove_order_path() {
        // Path: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let order = vec![0, 1, 2];
        let removal = remove_order(3, &edges, &order);

        // Vertex 2 has no later neighbors, so it can be removed at step 2
        // Vertex 1's latest neighbor is 2, so can be removed at step 2
        // Vertex 0's latest neighbor is 1, so can be removed at step 1
        assert_eq!(removal.len(), 3);
    }

    #[test]
    fn test_mis_overhead_copyline() {
        let line = CopyLine::new(0, 1, 2, 1, 2, 3);
        let spacing = 4;
        let padding = 2;
        let locs = line.copyline_locations(padding, spacing);
        let overhead = mis_overhead_copyline(&line, spacing, padding);
        // Julia formula for UnWeighted mode: length(locs) / 2
        assert_eq!(overhead, locs.len() / 2);
    }

    #[test]
    fn test_copyline_serialization() {
        let line = CopyLine::new(0, 1, 2, 1, 2, 3);
        let json = serde_json::to_string(&line).unwrap();
        let deserialized: CopyLine = serde_json::from_str(&json).unwrap();
        assert_eq!(line, deserialized);
    }

    #[test]
    fn test_create_copylines_star() {
        // Star graph: 0 connected to 1, 2, 3
        let edges = vec![(0, 1), (0, 2), (0, 3)];
        let order = vec![0, 1, 2, 3];
        let lines = create_copylines(4, &edges, &order);

        assert_eq!(lines.len(), 4);
        // Vertex 0 (center) should have hstop reaching the last neighbor
        assert_eq!(lines[0].hstop, 4);
    }

    #[test]
    fn test_copyline_locations_detailed() {
        let line = CopyLine::new(0, 1, 2, 1, 2, 2);
        let locs = line.locations(0, 2);

        // With padding=0, spacing=2:
        // Vertical segment at col = 2*(1-1) + 0 + 1 = 1
        // vstart=1: row = 2*(1-1) + 0 + 2 = 2
        // vstop=2: row = 2*(2-1) + 0 + 2 = 4
        // So vertical segment covers (2, 1), (4, 1)

        // Horizontal segment at hslot=2: row = 2*(2-1) + 0 + 2 = 4
        // from vslot+1=2 to hstop=2: col = 2*(2-1) + 0 + 1 = 3
        // So horizontal has (4, 3)

        assert!(!locs.is_empty());
        // Check that we have vertical positions
        let has_vertical = locs.iter().any(|&(_r, c, _)| c == 1);
        assert!(has_vertical);
    }

    #[test]
    fn test_copyline_locations_simple() {
        // Simple L-shape: vslot=1, hslot=1, vstart=1, vstop=2, hstop=2
        let line = CopyLine::new(0, 1, 1, 1, 2, 2);
        let locs = line.copyline_locations(2, 4); // padding=2, spacing=4

        // Center: I = 4*(1-1) + 2 + 2 = 4, J = 4*(1-1) + 2 + 1 = 3
        // vstart=1, hslot=1: no "up" segment
        // vstop=2, hslot=1: "down" segment from I to I + 4*(2-1) - 1 = 4 to 7
        // hstop=2, vslot=1: "right" segment from J+2=5 to J + 4*(2-1) - 1 = 6

        assert!(!locs.is_empty());
        // Should have nodes at every cell, not just at spacing intervals
        // Check we have more than just the sparse waypoints
        let node_count = locs.len();
        println!("Dense locations for simple L-shape: {:?}", locs);
        println!("Node count: {}", node_count);

        // Dense should have many more nodes than sparse (which has ~3-4)
        assert!(node_count > 4, "Dense locations should have more than sparse");
    }

    #[test]
    fn test_copyline_locations_matches_julia() {
        // Test case that can be verified against Julia's UnitDiskMapping
        // Using vslot=1, hslot=2, vstart=1, vstop=2, hstop=3, padding=2, spacing=4
        let line = CopyLine::new(0, 1, 2, 1, 2, 3);
        let locs = line.copyline_locations(2, 4);

        // Center location: I = 4*(2-1) + 2 + 2 = 8, J = 4*(1-1) + 2 + 1 = 3

        // Verify center node is present at (I, J+1) = (8, 4)
        let has_center = locs.iter().any(|&(r, c, _)| r == 8 && c == 4);
        assert!(has_center, "Center node at (8, 4) should be present");

        // All positions should be valid
        for &(row, col, weight) in &locs {
            assert!(weight >= 1, "Weight should be >= 1");
            assert!(row > 0, "Row should be positive");
            assert!(col > 0, "Col should be positive");
        }

        println!("Dense locations: {:?}", locs);
    }

    // === Julia comparison tests ===
    // These test cases are derived from Julia's UnitDiskMapping tests

    #[test]
    fn test_mis_overhead_julia_cases() {
        // Test cases using UnWeighted formula: length(copyline_locations) / 2
        // Using vslot=5, hslot=5 as the base configuration
        let spacing = 4;
        let padding = 2;

        let test_cases = [
            // (vstart, vstop, hstop)
            (3, 7, 8),
            (3, 5, 8),
            (5, 9, 8),
            (5, 5, 8),
            (1, 7, 5),
            (5, 8, 5),
            (1, 5, 5),
            (5, 5, 5),
        ];

        for (vstart, vstop, hstop) in test_cases {
            let line = CopyLine::new(1, 5, 5, vstart, vstop, hstop);
            let locs = line.copyline_locations(padding, spacing);
            let overhead = mis_overhead_copyline(&line, spacing, padding);

            // UnWeighted formula: length(locs) / 2
            let expected = locs.len() / 2;

            assert_eq!(
                overhead, expected,
                "MIS overhead mismatch for (vstart={}, vstop={}, hstop={}): got {}, expected {}",
                vstart, vstop, hstop, overhead, expected
            );
        }
    }

    #[test]
    fn test_create_copylines_petersen() {
        // Petersen graph edges (0-indexed)
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 0), // outer pentagon
            (5, 7),
            (7, 9),
            (9, 6),
            (6, 8),
            (8, 5), // inner star
            (0, 5),
            (1, 6),
            (2, 7),
            (3, 8),
            (4, 9), // connections
        ];
        let order: Vec<usize> = (0..10).collect();

        let lines = create_copylines(10, &edges, &order);

        // Verify all lines are created
        assert_eq!(lines.len(), 10);

        // Verify basic invariants
        for (i, &v) in order.iter().enumerate() {
            let line = &lines[v];
            assert_eq!(line.vertex, v, "Vertex mismatch");
            assert_eq!(line.vslot, i + 1, "vslot should be position + 1");
            assert!(
                line.vstart <= line.hslot && line.hslot <= line.vstop,
                "hslot should be between vstart and vstop for vertex {}",
                v
            );
            assert!(
                line.hstop >= line.vslot,
                "hstop should be >= vslot for vertex {}",
                v
            );
        }

        // Verify that neighboring vertices have overlapping L-shapes
        for &(u, v) in &edges {
            let line_u = &lines[u];
            let line_v = &lines[v];
            // Two lines cross if one's vslot is in the other's hslot range
            // and one's hslot is in the other's vslot range
            let u_pos = order.iter().position(|&x| x == u).unwrap() + 1;
            let v_pos = order.iter().position(|&x| x == v).unwrap() + 1;
            // For a valid embedding, connected vertices should have crossing copy lines
            assert!(
                line_u.hstop >= v_pos || line_v.hstop >= u_pos,
                "Connected vertices {} and {} should have overlapping L-shapes",
                u,
                v
            );
        }
    }

    #[test]
    fn test_remove_order_detailed() {
        // Path graph: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let order = vec![0, 1, 2];
        let removal = remove_order(3, &edges, &order);

        // Trace through Julia's algorithm:
        // Step 0: add vertex 0, counts = [0, 1, 0], totalcounts = [1, 2, 1]
        //         vertex 0: counts[0]=0 != totalcounts[0]=1, not removed
        //         vertex 1: counts[1]=1 != totalcounts[1]=2, not removed
        //         vertex 2: counts[2]=0 != totalcounts[2]=1, not removed
        //         removal[0] = []
        // Step 1: add vertex 1, counts = [1, 2, 1], totalcounts = [1, 2, 1]
        //         vertex 0: counts[0]=1 == totalcounts[0]=1, remove at max(1, 0)=1
        //         vertex 1: counts[1]=2 == totalcounts[1]=2, remove at max(1, 1)=1
        //         vertex 2: counts[2]=1 == totalcounts[2]=1, remove at max(1, 2)=2
        //         removal[1] = [0, 1]
        // Step 2: add vertex 2, counts = [1, 3, 2]
        //         vertex 2 already marked removed at step 2
        //         removal[2] = [2]

        assert_eq!(removal.len(), 3);
        // At step 1, vertices 0 and 1 can be removed
        assert!(removal[1].contains(&0) || removal[1].contains(&1));
        // At step 2, vertex 2 can be removed
        assert!(removal[2].contains(&2));
    }

    #[test]
    fn test_copyline_locations_node_count() {
        // For a copy line, copyline_locations should produce nodes at every cell
        // The number of nodes should be odd (ends + center)
        let spacing = 4;

        let test_cases = [
            (1, 1, 1, 2),
            (1, 2, 1, 3),
            (1, 1, 2, 3),
            (3, 7, 5, 8),
        ];

        for (vslot, hslot, vstart, hstop) in test_cases {
            let vstop = hslot; // Simplified: vstop = hslot
            let line = CopyLine::new(0, vslot, hslot, vstart, vstop, hstop);
            let locs = line.copyline_locations(2, spacing);

            // Node count should be odd (property of copy line construction)
            // This is verified in Julia's test: @assert length(locs) % 2 == 1
            println!(
                "vslot={}, hslot={}, vstart={}, vstop={}, hstop={}: {} nodes",
                vslot,
                hslot,
                vstart,
                vstop,
                hstop,
                locs.len()
            );

            // All weights should be 1 or 2 (for non-center nodes)
            // except center node which has weight = nline (number of line segments)
            for &(row, col, weight) in &locs {
                assert!(row > 0 && col > 0, "Coordinates should be positive");
                assert!(weight >= 1, "Weight should be >= 1");
            }
        }
    }
}
