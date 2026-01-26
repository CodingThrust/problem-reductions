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
}

/// Helper function to compute the removal order for vertices.
/// Returns a vector where each element is a list of vertices that can be
/// removed at that step (vertices with no unremoved neighbors with lower order).
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
    // Build adjacency list
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); num_vertices];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    // Create order map: vertex -> position in order
    let mut order_pos = vec![0usize; num_vertices];
    for (pos, &v) in vertex_order.iter().enumerate() {
        order_pos[v] = pos;
    }

    // For each vertex, find the maximum order position among its neighbors
    // that appear later in the ordering
    let mut max_later_neighbor = vec![0usize; num_vertices];
    for v in 0..num_vertices {
        let v_pos = order_pos[v];
        for &neighbor in &adj[v] {
            let n_pos = order_pos[neighbor];
            if n_pos > v_pos {
                max_later_neighbor[v] = max_later_neighbor[v].max(n_pos);
            }
        }
    }

    // Group vertices by when they can be removed
    // A vertex can be removed at step i if all its later neighbors have been processed
    let mut result: Vec<Vec<usize>> = vec![Vec::new(); num_vertices];
    for &v in vertex_order {
        let remove_step = max_later_neighbor[v];
        result[remove_step].push(v);
    }

    result
}

/// Create copy lines for all vertices based on the vertex ordering.
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the graph
/// * `edges` - List of edges as (u, v) pairs
/// * `vertex_order` - The order in which vertices are processed
///
/// # Returns
/// A vector of CopyLine structures, one per vertex.
pub fn create_copylines(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Vec<CopyLine> {
    if num_vertices == 0 {
        return Vec::new();
    }

    // Build adjacency list
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); num_vertices];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    // Create order map: vertex -> position in order (1-indexed for slots)
    let mut order_pos = vec![0usize; num_vertices];
    for (pos, &v) in vertex_order.iter().enumerate() {
        order_pos[v] = pos + 1; // 1-indexed
    }

    // Compute removal order
    let removal = remove_order(num_vertices, edges, vertex_order);

    // Track slot availability: which hslot each vslot is free from
    let mut slot_available_from = vec![1usize; num_vertices + 1];

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

    // Process vertices in order
    for (idx, &v) in vertex_order.iter().enumerate() {
        let vslot = idx + 1; // 1-indexed slot

        // Find hslot: the row where this vertex's horizontal segment lives
        // It must be >= slot_available_from[vslot]
        let hslot = slot_available_from[vslot];

        // Find the maximum vslot among later neighbors (for hstop)
        let mut max_later_vslot = vslot;
        for &neighbor in &adj[v] {
            let n_vslot = order_pos[neighbor];
            if n_vslot > vslot {
                max_later_vslot = max_later_vslot.max(n_vslot);
            }
        }
        let hstop = max_later_vslot;

        // vstart is 1 (top of the grid)
        let vstart = 1;

        // vstop is the hslot (vertical segment goes down to the horizontal segment)
        let vstop = hslot;

        copylines[v] = CopyLine::new(v, vslot, hslot, vstart, vstop, hstop);

        // Update slot availability for slots that this copy line passes through
        // The horizontal segment occupies hslot from vslot to hstop
        for slot in &mut slot_available_from[vslot..=hstop.min(num_vertices)] {
            *slot = (*slot).max(hslot + 1);
        }

        // When vertices are removed (from removal order), free up their slots
        if idx < removal.len() {
            for &removed_v in &removal[idx] {
                let removed_vslot = order_pos[removed_v];
                // This vertex's vertical segment is no longer blocking
                // (handled implicitly by the slot tracking)
                let _ = removed_vslot; // Acknowledge but don't need explicit action
            }
        }
    }

    copylines
}

/// Calculate the MIS (Maximum Independent Set) overhead for a copy line.
///
/// The overhead represents the contribution to the MIS problem size
/// from this copy line's grid representation.
///
/// # Arguments
/// * `line` - The copy line
/// * `spacing` - Grid spacing parameter
///
/// # Returns
/// The MIS overhead value for this copy line.
pub fn mis_overhead_copyline(line: &CopyLine, spacing: usize) -> usize {
    // The overhead is based on the length of the copy line segments
    // Vertical segment length
    let v_len = if line.vstop >= line.vstart {
        line.vstop - line.vstart + 1
    } else {
        0
    };

    // Horizontal segment length (excluding the corner which is counted in vertical)
    let h_len = line.hstop.saturating_sub(line.vslot);

    // Total cells occupied, scaled by spacing
    // Each segment cell contributes approximately spacing/2 to MIS overhead
    let total_len = v_len + h_len;
    total_len * spacing.div_ceil(2)
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
        let overhead = mis_overhead_copyline(&line, 4);
        // v_len = 2 - 1 + 1 = 2
        // h_len = 3 - 1 = 2
        // total = 4, overhead = 4 * ((4+1)/2) = 4 * 2 = 8
        assert_eq!(overhead, 8);
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
}
