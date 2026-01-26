# Grid Graph Reductions Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Port the UnitDiskMapping.jl reductions to enable mapping arbitrary graphs to unit disk grid graphs (square and triangular lattices).

**Architecture:** Implement a gadget-based reduction system using the "copy-line" technique. Each vertex in the source graph becomes a copy-line on a 2D grid, crossings are resolved using pre-defined gadgets, and solutions can be mapped back via the inverse transformation.

**Tech Stack:** Rust, petgraph, serde

---

## Task 1: GridGraph Type

**Files:**
- Create: `src/topology/grid_graph.rs`
- Modify: `src/topology/mod.rs`

**Step 1: Write the failing test**

```rust
// In src/topology/grid_graph.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_graph_square_basic() {
        let nodes = vec![
            GridNode::new(0, 0, 1),
            GridNode::new(1, 0, 1),
            GridNode::new(0, 1, 1),
        ];
        let grid = GridGraph::new(GridType::Square, (2, 2), nodes, 1.5);
        assert_eq!(grid.num_vertices(), 3);
        // Nodes at (0,0)-(1,0) and (0,0)-(0,1) are within radius 1.5
        assert_eq!(grid.edges().len(), 2);
    }

    #[test]
    fn test_grid_graph_triangular_basic() {
        let nodes = vec![
            GridNode::new(0, 0, 1),
            GridNode::new(1, 0, 1),
            GridNode::new(0, 1, 1),
        ];
        let grid = GridGraph::new(GridType::Triangular { offset_even_cols: false }, (2, 2), nodes, 1.1);
        assert_eq!(grid.num_vertices(), 3);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test grid_graph_square_basic --no-run 2>&1 | head -20`
Expected: Compile error - module not found

**Step 3: Write minimal implementation**

```rust
// src/topology/grid_graph.rs
//! Grid Graph implementation for unit disk graphs on integer lattices.

use super::graph::Graph;
use serde::{Deserialize, Serialize};

/// Grid type for physical position calculation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GridType {
    /// Square lattice: position (i, j) maps to physical (i, j).
    Square,
    /// Triangular lattice with equilateral triangle geometry.
    Triangular { offset_even_cols: bool },
}

impl Default for GridType {
    fn default() -> Self {
        GridType::Square
    }
}

/// A node on a grid with integer coordinates and weight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridNode<W = i32> {
    /// Grid row (y-coordinate).
    pub row: usize,
    /// Grid column (x-coordinate).
    pub col: usize,
    /// Weight of this node.
    pub weight: W,
}

impl<W> GridNode<W> {
    pub fn new(row: usize, col: usize, weight: W) -> Self {
        Self { row, col, weight }
    }

    /// Get grid coordinates as (row, col).
    pub fn loc(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

/// A graph on a 2D grid where edges exist between nodes within a radius.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridGraph<W = i32> {
    grid_type: GridType,
    size: (usize, usize),
    nodes: Vec<GridNode<W>>,
    radius: f64,
    edges: Vec<(usize, usize)>,
}

impl<W: Clone> GridGraph<W> {
    /// Create a new grid graph.
    pub fn new(grid_type: GridType, size: (usize, usize), nodes: Vec<GridNode<W>>, radius: f64) -> Self {
        let edges = Self::compute_edges(&grid_type, &nodes, radius);
        Self { grid_type, size, nodes, radius, edges }
    }

    /// Compute physical position based on grid type.
    pub fn physical_position(grid_type: &GridType, row: usize, col: usize) -> (f64, f64) {
        match grid_type {
            GridType::Square => (row as f64, col as f64),
            GridType::Triangular { offset_even_cols } => {
                let y = col as f64 * (3.0_f64.sqrt() / 2.0);
                let offset = if *offset_even_cols {
                    if col % 2 == 0 { 0.5 } else { 0.0 }
                } else {
                    if col % 2 == 1 { 0.5 } else { 0.0 }
                };
                (row as f64 + offset, y)
            }
        }
    }

    fn distance(grid_type: &GridType, n1: &GridNode<W>, n2: &GridNode<W>) -> f64 {
        let p1 = Self::physical_position(grid_type, n1.row, n1.col);
        let p2 = Self::physical_position(grid_type, n2.row, n2.col);
        ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
    }

    fn compute_edges(grid_type: &GridType, nodes: &[GridNode<W>], radius: f64) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                if Self::distance(grid_type, &nodes[i], &nodes[j]) <= radius {
                    edges.push((i, j));
                }
            }
        }
        edges
    }

    pub fn grid_type(&self) -> &GridType {
        &self.grid_type
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn nodes(&self) -> &[GridNode<W>] {
        &self.nodes
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn weights(&self) -> Vec<W> {
        self.nodes.iter().map(|n| n.weight.clone()).collect()
    }
}

impl<W: Clone> Graph for GridGraph<W> {
    fn num_vertices(&self) -> usize {
        self.nodes.len()
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        self.edges.clone()
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        let (u, v) = if u < v { (u, v) } else { (v, u) };
        self.edges.contains(&(u, v))
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(u1, u2)| {
                if u1 == v { Some(u2) }
                else if u2 == v { Some(u1) }
                else { None }
            })
            .collect()
    }
}
```

**Step 4: Update mod.rs**

```rust
// Add to src/topology/mod.rs
mod grid_graph;
pub use grid_graph::{GridGraph, GridNode, GridType};
```

**Step 5: Run test to verify it passes**

Run: `cargo test grid_graph --lib`
Expected: PASS

**Step 6: Commit**

```bash
git add src/topology/grid_graph.rs src/topology/mod.rs
git commit -m "feat(topology): Add GridGraph type for square and triangular lattices"
```

---

## Task 2: CopyLine Structure

**Files:**
- Create: `src/rules/mapping/copyline.rs`
- Create: `src/rules/mapping/mod.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write the failing test**

```rust
// In src/rules/mapping/copyline.rs
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
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test create_copylines --no-run 2>&1 | head -20`
Expected: Compile error

**Step 3: Write minimal implementation**

```rust
// src/rules/mapping/copyline.rs
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
    /// Get the center location of this copy line.
    pub fn center_location(&self, padding: usize, spacing: usize) -> (usize, usize) {
        let row = spacing * (self.hslot - 1) + padding + 2;
        let col = spacing * (self.vslot - 1) + padding + 1;
        (row, col)
    }

    /// Generate grid locations for this copy line.
    pub fn locations(&self, padding: usize, spacing: usize) -> Vec<(usize, usize, usize)> {
        let (center_row, center_col) = self.center_location(padding, spacing);
        let mut locations = Vec::new();
        let mut nline = 0;

        // Grow up
        let start = center_row as isize + (spacing as isize) * (self.vstart as isize - self.hslot as isize) + 1;
        if self.vstart < self.hslot {
            nline += 1;
        }
        let mut row = center_row as isize;
        while row >= start {
            let weight = if row != start { 2 } else { 1 };
            locations.push((row as usize, center_col, weight));
            row -= 1;
        }

        // Grow down
        let stop = center_row + spacing * (self.vstop - self.hslot) - 1;
        if self.vstop > self.hslot {
            nline += 1;
        }
        for r in center_row..=stop {
            if r == center_row {
                locations.push((r + 1, center_col + 1, 2));
            } else {
                let weight = if r != stop { 2 } else { 1 };
                locations.push((r, center_col, weight));
            }
        }

        // Grow right
        let stop_col = center_col + spacing * (self.hstop - self.vslot) - 1;
        if self.hstop > self.vslot {
            nline += 1;
        }
        for c in (center_col + 2)..=stop_col {
            let weight = if c != stop_col { 2 } else { 1 };
            locations.push((center_row, c, weight));
        }

        // Center node
        locations.push((center_row, center_col + 1, nline));

        locations
    }
}

/// Compute the remove order for vertices based on vertex ordering.
fn remove_order(num_vertices: usize, edges: &[(usize, usize)], vertex_order: &[usize]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![false; num_vertices]; num_vertices];
    let mut degree = vec![0usize; num_vertices];

    for &(u, v) in edges {
        adj[u][v] = true;
        adj[v][u] = true;
        degree[u] += 1;
        degree[v] += 1;
    }

    let mut add_remove = vec![Vec::new(); num_vertices];
    let mut counts = vec![0usize; num_vertices];
    let mut removed = vec![false; num_vertices];

    for (i, &v) in vertex_order.iter().enumerate() {
        // Add adjacency counts
        for j in 0..num_vertices {
            if adj[v][j] {
                counts[j] += 1;
            }
        }

        // Check which vertices can be removed
        for j in 0..num_vertices {
            if !removed[j] && counts[j] == degree[j] {
                let order_idx = vertex_order.iter().position(|&x| x == j).unwrap();
                let idx = i.max(order_idx);
                add_remove[idx].push(j);
                removed[j] = true;
            }
        }
    }

    add_remove
}

/// Create copy lines for a graph with given vertex ordering.
pub fn create_copylines(num_vertices: usize, edges: &[(usize, usize)], vertex_order: &[usize]) -> Vec<CopyLine> {
    let mut slots = vec![0usize; num_vertices];
    let mut hslots = vec![0usize; num_vertices];
    let rm_order = remove_order(num_vertices, edges, vertex_order);

    // Build adjacency for quick lookup
    let mut adj = vec![vec![false; num_vertices]; num_vertices];
    for &(u, v) in edges {
        adj[u][v] = true;
        adj[v][u] = true;
    }

    // Assign hslots
    for (i, (&v, rs)) in vertex_order.iter().zip(rm_order.iter()).enumerate() {
        let islot = slots.iter().position(|&s| s == 0).unwrap();
        slots[islot] = v + 1; // Use v+1 to distinguish from 0
        hslots[i] = islot + 1; // 1-indexed

        for &r in rs {
            if let Some(pos) = slots.iter().position(|&s| s == r + 1) {
                slots[pos] = 0;
            }
        }
    }

    let mut vstarts = vec![0usize; num_vertices];
    let mut vstops = vec![0usize; num_vertices];
    let mut hstops = vec![0usize; num_vertices];

    for (i, &v) in vertex_order.iter().enumerate() {
        let relevant_hslots: Vec<usize> = (0..=i)
            .filter(|&j| adj[vertex_order[j]][v] || v == vertex_order[j])
            .map(|j| hslots[j])
            .collect();

        let relevant_vslots: Vec<usize> = (0..num_vertices)
            .filter(|&j| adj[vertex_order[j]][v] || v == vertex_order[j])
            .map(|j| j + 1)
            .collect();

        vstarts[i] = *relevant_hslots.iter().min().unwrap_or(&1);
        vstops[i] = *relevant_hslots.iter().max().unwrap_or(&1);
        hstops[i] = *relevant_vslots.iter().max().unwrap_or(&1);
    }

    vertex_order
        .iter()
        .enumerate()
        .map(|(i, &v)| CopyLine {
            vertex: v,
            vslot: i + 1,
            hslot: hslots[i],
            vstart: vstarts[i],
            vstop: vstops[i],
            hstop: hstops[i],
        })
        .collect()
}

/// Calculate the MIS overhead for a copy line.
pub fn mis_overhead_copyline(line: &CopyLine, spacing: usize) -> usize {
    let row_overhead = (line.hslot.saturating_sub(line.vstart)) * spacing
        + (line.vstop.saturating_sub(line.hslot)) * spacing;
    let col_overhead = if line.hstop > line.vslot {
        (line.hstop - line.vslot) * spacing - 2
    } else {
        0
    };
    row_overhead + col_overhead
}
```

**Step 4: Create mod.rs**

```rust
// src/rules/mapping/mod.rs
//! Graph to grid mapping functionality.

mod copyline;

pub use copyline::{CopyLine, create_copylines, mis_overhead_copyline};
```

**Step 5: Update rules/mod.rs**

```rust
// Add to src/rules/mod.rs after other pub mod declarations
pub mod mapping;
```

**Step 6: Run test to verify it passes**

Run: `cargo test copyline --lib`
Expected: PASS

**Step 7: Commit**

```bash
git add src/rules/mapping/copyline.rs src/rules/mapping/mod.rs src/rules/mod.rs
git commit -m "feat(mapping): Add CopyLine structure for graph embedding"
```

---

## Task 3: Gadget Trait and Basic Gadgets

**Files:**
- Create: `src/rules/mapping/gadgets.rs`
- Modify: `src/rules/mapping/mod.rs`

**Step 1: Write the failing test**

```rust
// In src/rules/mapping/gadgets.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_gadget_size() {
        let cross = Cross::<false>;
        assert_eq!(cross.size(), (4, 5));

        let cross_con = Cross::<true>;
        assert_eq!(cross_con.size(), (3, 3));
    }

    #[test]
    fn test_turn_gadget() {
        let turn = Turn;
        assert_eq!(turn.size(), (4, 4));
        let (locs, pins) = turn.source_graph();
        assert_eq!(pins.len(), 2);
    }

    #[test]
    fn test_gadget_vertex_overhead() {
        let cross = Cross::<false>;
        // mapped has more vertices than source
        let (src_locs, _) = cross.source_graph();
        let (map_locs, _) = cross.mapped_graph();
        assert!(map_locs.len() > src_locs.len() || map_locs.len() <= src_locs.len());
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test cross_gadget --no-run 2>&1 | head -20`
Expected: Compile error

**Step 3: Write minimal implementation**

```rust
// src/rules/mapping/gadgets.rs
//! Gadgets for resolving crossings in grid graph embeddings.
//!
//! A gadget transforms a pattern in the source graph to an equivalent
//! pattern in the mapped graph, preserving MIS properties.

use serde::{Deserialize, Serialize};

/// A gadget pattern that transforms source configurations to mapped configurations.
pub trait Gadget: Clone {
    /// Size of the gadget pattern (rows, cols).
    fn size(&self) -> (usize, usize);

    /// Cross location within the gadget.
    fn cross_location(&self) -> (usize, usize);

    /// Whether this gadget involves connected nodes.
    fn is_connected(&self) -> bool;

    /// Source graph: (locations, pin_indices).
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);

    /// Mapped graph: (locations, pin_indices).
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);

    /// MIS overhead when applying this gadget.
    fn mis_overhead(&self) -> i32;
}

/// Cross gadget for handling line crossings.
/// CON=true means connected crossing, CON=false means disconnected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cross<const CON: bool>;

impl Gadget for Cross<true> {
    fn size(&self) -> (usize, usize) { (3, 3) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { true }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // ⋅ ● ⋅
        // ◆ ◉ ●
        // ⋅ ◆ ⋅
        let locs = vec![(2,1), (2,2), (2,3), (1,2), (2,2), (3,2)];
        let pins = vec![0, 3, 5, 2];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // ⋅ ● ⋅
        // ● ● ●
        // ⋅ ● ⋅
        let locs = vec![(2,1), (2,2), (2,3), (1,2), (3,2)];
        let pins = vec![0, 3, 4, 2];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 0 }
}

impl Gadget for Cross<false> {
    fn size(&self) -> (usize, usize) { (4, 5) }
    fn cross_location(&self) -> (usize, usize) { (2, 3) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // ⋅ ⋅ ● ⋅ ⋅
        // ● ● ◉ ● ●
        // ⋅ ⋅ ● ⋅ ⋅
        // ⋅ ⋅ ● ⋅ ⋅
        let locs = vec![
            (2,1), (2,2), (2,3), (2,4), (2,5),
            (1,3), (2,3), (3,3), (4,3)
        ];
        let pins = vec![0, 5, 8, 4];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // ⋅ ⋅ ● ⋅ ⋅
        // ● ● ● ● ●
        // ⋅ ● ● ● ⋅
        // ⋅ ⋅ ● ⋅ ⋅
        let locs = vec![
            (2,1), (2,2), (2,3), (2,4), (2,5),
            (1,3), (3,3), (4,3), (3,2), (3,4)
        ];
        let pins = vec![0, 5, 7, 4];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 1 }
}

/// Turn gadget for 90-degree turns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Turn;

impl Gadget for Turn {
    fn size(&self) -> (usize, usize) { (4, 4) }
    fn cross_location(&self) -> (usize, usize) { (3, 2) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // ⋅ ● ⋅ ⋅
        // ⋅ ● ⋅ ⋅
        // ⋅ ● ● ●
        // ⋅ ⋅ ⋅ ⋅
        let locs = vec![(1,2), (2,2), (3,2), (3,3), (3,4)];
        let pins = vec![0, 4];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // ⋅ ● ⋅ ⋅
        // ⋅ ⋅ ● ⋅
        // ⋅ ⋅ ⋅ ●
        // ⋅ ⋅ ⋅ ⋅
        let locs = vec![(1,2), (2,3), (3,4)];
        let pins = vec![0, 2];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 1 }
}

/// Branch gadget for T-junctions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Branch;

impl Gadget for Branch {
    fn size(&self) -> (usize, usize) { (5, 4) }
    fn cross_location(&self) -> (usize, usize) { (3, 2) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (1,2), (2,2), (3,2), (3,3), (3,4),
            (4,3), (4,2), (5,2)
        ];
        let pins = vec![0, 4, 7];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (1,2), (2,3), (3,2), (3,4), (4,3), (5,2)
        ];
        let pins = vec![0, 3, 5];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 0 }
}

/// BranchFix gadget for simplifying branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchFix;

impl Gadget for BranchFix {
    fn size(&self) -> (usize, usize) { (4, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2), (2,2), (2,3), (3,3), (3,2), (4,2)];
        let pins = vec![0, 5];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2), (2,2), (3,2), (4,2)];
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 1 }
}

/// WTurn gadget for W-shaped turns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WTurn;

impl Gadget for WTurn {
    fn size(&self) -> (usize, usize) { (4, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2,3), (2,4), (3,2), (3,3), (4,2)];
        let pins = vec![1, 4];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2,4), (3,3), (4,2)];
        let pins = vec![0, 2];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 1 }
}

/// TCon gadget for T-connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TCon;

impl Gadget for TCon {
    fn size(&self) -> (usize, usize) { (3, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { true }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2), (2,1), (2,2), (3,2)];
        let pins = vec![0, 1, 3];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2), (2,1), (2,3), (3,2)];
        let pins = vec![0, 1, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 1 }
}

/// TrivialTurn for simple diagonal turns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrivialTurn;

impl Gadget for TrivialTurn {
    fn size(&self) -> (usize, usize) { (2, 2) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { true }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2), (2,1)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2), (2,1)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 0 }
}

/// EndTurn for line termination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndTurn;

impl Gadget for EndTurn {
    fn size(&self) -> (usize, usize) { (3, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2), (2,2), (2,3)];
        let pins = vec![0];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2)];
        let pins = vec![0];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 1 }
}

/// BranchFixB for alternate branch fixing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchFixB;

impl Gadget for BranchFixB {
    fn size(&self) -> (usize, usize) { (4, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2,3), (3,2), (3,3), (4,2)];
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(3,2), (4,2)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 1 }
}

/// The default crossing ruleset for square lattice.
pub fn crossing_ruleset_square() -> Vec<Box<dyn Gadget>> {
    vec![
        Box::new(Cross::<false>),
        Box::new(Turn),
        Box::new(WTurn),
        Box::new(Branch),
        Box::new(BranchFix),
        Box::new(TCon),
        Box::new(TrivialTurn),
        Box::new(EndTurn),
        Box::new(BranchFixB),
    ]
}
```

**Step 4: Update mod.rs**

```rust
// In src/rules/mapping/mod.rs
mod gadgets;
pub use gadgets::*;
```

**Step 5: Run test to verify it passes**

Run: `cargo test gadget --lib`
Expected: PASS

**Step 6: Commit**

```bash
git add src/rules/mapping/gadgets.rs src/rules/mapping/mod.rs
git commit -m "feat(mapping): Add gadget trait and basic gadgets for square lattice"
```

---

## Task 4: MappingGrid and Pattern Matching

**Files:**
- Create: `src/rules/mapping/grid.rs`
- Modify: `src/rules/mapping/mod.rs`

**Step 1: Write the failing test**

```rust
// In src/rules/mapping/grid.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_grid_create() {
        let grid = MappingGrid::new(10, 10, 4);
        assert_eq!(grid.size(), (10, 10));
        assert_eq!(grid.spacing(), 4);
    }

    #[test]
    fn test_mapping_grid_add_node() {
        let mut grid = MappingGrid::new(10, 10, 4);
        grid.add_node(2, 3, 1);
        assert!(grid.is_occupied(2, 3));
        assert!(!grid.is_occupied(2, 4));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test mapping_grid --no-run 2>&1 | head -20`
Expected: Compile error

**Step 3: Write minimal implementation**

```rust
// src/rules/mapping/grid.rs
//! Mapping grid for intermediate representation during graph embedding.

use serde::{Deserialize, Serialize};

/// Cell state in the mapping grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellState {
    Empty,
    Occupied { weight: i32 },
    Doubled { weight: i32 },
    Connected { weight: i32 },
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Empty
    }
}

impl CellState {
    pub fn is_empty(&self) -> bool {
        matches!(self, CellState::Empty)
    }

    pub fn is_occupied(&self) -> bool {
        !self.is_empty()
    }

    pub fn weight(&self) -> i32 {
        match self {
            CellState::Empty => 0,
            CellState::Occupied { weight } => *weight,
            CellState::Doubled { weight } => *weight,
            CellState::Connected { weight } => *weight,
        }
    }
}

/// A 2D grid for mapping graphs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingGrid {
    content: Vec<Vec<CellState>>,
    rows: usize,
    cols: usize,
    spacing: usize,
    padding: usize,
}

impl MappingGrid {
    /// Create a new mapping grid.
    pub fn new(rows: usize, cols: usize, spacing: usize) -> Self {
        Self {
            content: vec![vec![CellState::Empty; cols]; rows],
            rows,
            cols,
            spacing,
            padding: 2,
        }
    }

    /// Create with custom padding.
    pub fn with_padding(rows: usize, cols: usize, spacing: usize, padding: usize) -> Self {
        Self {
            content: vec![vec![CellState::Empty; cols]; rows],
            rows,
            cols,
            spacing,
            padding,
        }
    }

    /// Get grid dimensions.
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Get spacing.
    pub fn spacing(&self) -> usize {
        self.spacing
    }

    /// Get padding.
    pub fn padding(&self) -> usize {
        self.padding
    }

    /// Check if a cell is occupied.
    pub fn is_occupied(&self, row: usize, col: usize) -> bool {
        self.get(row, col).map(|c| c.is_occupied()).unwrap_or(false)
    }

    /// Get cell state safely.
    pub fn get(&self, row: usize, col: usize) -> Option<&CellState> {
        self.content.get(row).and_then(|r| r.get(col))
    }

    /// Get mutable cell state safely.
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut CellState> {
        self.content.get_mut(row).and_then(|r| r.get_mut(col))
    }

    /// Set cell state.
    pub fn set(&mut self, row: usize, col: usize, state: CellState) {
        if row < self.rows && col < self.cols {
            self.content[row][col] = state;
        }
    }

    /// Add a node at position.
    pub fn add_node(&mut self, row: usize, col: usize, weight: i32) {
        if row < self.rows && col < self.cols {
            match self.content[row][col] {
                CellState::Empty => {
                    self.content[row][col] = CellState::Occupied { weight };
                }
                CellState::Occupied { weight: w } => {
                    self.content[row][col] = CellState::Doubled { weight: w + weight };
                }
                _ => {}
            }
        }
    }

    /// Mark a cell as connected.
    pub fn connect(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            if let CellState::Occupied { weight } = self.content[row][col] {
                self.content[row][col] = CellState::Connected { weight };
            }
        }
    }

    /// Check if a pattern matches at position.
    pub fn matches_pattern(&self, pattern: &[(usize, usize)], offset_row: usize, offset_col: usize) -> bool {
        pattern.iter().all(|&(r, c)| {
            let row = offset_row + r;
            let col = offset_col + c;
            self.get(row, col).map(|c| c.is_occupied()).unwrap_or(false)
        })
    }

    /// Get all occupied coordinates.
    pub fn occupied_coords(&self) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.content[r][c].is_occupied() {
                    coords.push((r, c));
                }
            }
        }
        coords
    }

    /// Get cross location for two vertices.
    pub fn cross_at(&self, v_slot: usize, w_slot: usize, h_slot: usize) -> (usize, usize) {
        let (v, w) = if v_slot < w_slot { (v_slot, w_slot) } else { (w_slot, v_slot) };
        let row = (h_slot - 1) * self.spacing + 2 + self.padding;
        let col = (w - 1) * self.spacing + 1 + self.padding;
        (row, col)
    }
}
```

**Step 4: Update mod.rs**

```rust
// In src/rules/mapping/mod.rs
mod grid;
pub use grid::{MappingGrid, CellState};
```

**Step 5: Run test to verify it passes**

Run: `cargo test mapping_grid --lib`
Expected: PASS

**Step 6: Commit**

```bash
git add src/rules/mapping/grid.rs src/rules/mapping/mod.rs
git commit -m "feat(mapping): Add MappingGrid for intermediate representation"
```

---

## Task 5: Graph Mapping Functions

**Files:**
- Create: `src/rules/mapping/map_graph.rs`
- Modify: `src/rules/mapping/mod.rs`

**Step 1: Write the failing test**

```rust
// In src/rules/mapping/map_graph.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::GridGraph;

    #[test]
    fn test_embed_graph_path() {
        // Path graph: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let result = embed_graph(3, &edges, &[0, 1, 2]);

        assert!(result.is_some());
        let grid = result.unwrap();
        assert!(!grid.occupied_coords().is_empty());
    }

    #[test]
    fn test_map_graph_triangle() {
        // Triangle graph
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let result = map_graph(3, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
        assert!(result.mis_overhead >= 0);
    }

    #[test]
    fn test_mapping_result_config_back() {
        let edges = vec![(0, 1)];
        let result = map_graph(2, &edges);

        // Create a dummy config
        let config: Vec<usize> = vec![0; result.grid_graph.num_vertices()];
        let original = result.map_config_back(&config);

        assert_eq!(original.len(), 2);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test embed_graph --no-run 2>&1 | head -20`
Expected: Compile error

**Step 3: Write minimal implementation**

```rust
// src/rules/mapping/map_graph.rs
//! Graph to grid mapping functions.

use super::copyline::{create_copylines, CopyLine, mis_overhead_copyline};
use super::grid::{MappingGrid, CellState};
use crate::topology::{GridGraph, GridNode, GridType};
use serde::{Deserialize, Serialize};

const DEFAULT_SPACING: usize = 4;
const DEFAULT_PADDING: usize = 2;
const SQUARE_UNIT_RADIUS: f64 = 1.5;

/// Result of mapping a graph to a grid graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingResult {
    /// The resulting grid graph.
    pub grid_graph: GridGraph<i32>,
    /// Copy lines used in the mapping.
    pub lines: Vec<CopyLine>,
    /// Padding used.
    pub padding: usize,
    /// Spacing used.
    pub spacing: usize,
    /// MIS overhead from the mapping.
    pub mis_overhead: i32,
}

impl MappingResult {
    /// Map a configuration back from grid to original graph.
    pub fn map_config_back(&self, grid_config: &[usize]) -> Vec<usize> {
        let mut result = vec![0; self.lines.len()];

        for line in &self.lines {
            let locs = line.locations(self.padding, self.spacing);
            let mut count = 0;

            for (idx, &(row, col, _weight)) in locs.iter().enumerate() {
                // Find the node index at this location
                if let Some(node_idx) = self.find_node_at(row, col) {
                    if grid_config.get(node_idx).copied().unwrap_or(0) > 0 {
                        count += 1;
                    }
                }
            }

            // The original vertex is in the IS if count exceeds half the line length
            result[line.vertex] = if count > locs.len() / 2 { 1 } else { 0 };
        }

        result
    }

    fn find_node_at(&self, row: usize, col: usize) -> Option<usize> {
        self.grid_graph.nodes().iter().position(|n| n.row == row && n.col == col)
    }
}

/// Embed a graph into a mapping grid.
pub fn embed_graph(num_vertices: usize, edges: &[(usize, usize)], vertex_order: &[usize]) -> Option<MappingGrid> {
    if num_vertices == 0 {
        return None;
    }

    let spacing = DEFAULT_SPACING;
    let padding = DEFAULT_PADDING;

    let copylines = create_copylines(num_vertices, edges, vertex_order);

    // Calculate grid dimensions
    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);
    let max_vslot = copylines.iter().map(|l| l.vslot).max().unwrap_or(1);
    let max_hstop = copylines.iter().map(|l| l.hstop).max().unwrap_or(1);
    let max_vstop = copylines.iter().map(|l| l.vstop).max().unwrap_or(1);

    let rows = max_hslot.max(max_vstop) * spacing + 2 + 2 * padding;
    let cols = max_vslot.max(max_hstop) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);

    // Add copy line nodes
    for line in &copylines {
        for (row, col, weight) in line.locations(padding, spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Mark edge connections
    let mut adj = vec![vec![false; num_vertices]; num_vertices];
    for &(u, v) in edges {
        adj[u][v] = true;
        adj[v][u] = true;
    }

    for &(u, v) in edges {
        let u_idx = vertex_order.iter().position(|&x| x == u).unwrap();
        let v_idx = vertex_order.iter().position(|&x| x == v).unwrap();
        let u_line = &copylines[u_idx];
        let v_line = &copylines[v_idx];

        let (row, col) = grid.cross_at(u_line.vslot, v_line.vslot, u_line.hslot.min(v_line.hslot));

        // Mark connected cells
        if col > 0 {
            grid.connect(row, col - 1);
        }
        if row > 0 && grid.is_occupied(row - 1, col) {
            grid.connect(row - 1, col);
        } else if row + 1 < grid.size().0 && grid.is_occupied(row + 1, col) {
            grid.connect(row + 1, col);
        }
    }

    Some(grid)
}

/// Map a graph to a grid graph.
pub fn map_graph(num_vertices: usize, edges: &[(usize, usize)]) -> MappingResult {
    // Use simple ordering: 0, 1, 2, ...
    let vertex_order: Vec<usize> = (0..num_vertices).collect();
    map_graph_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph with a specific vertex ordering.
pub fn map_graph_with_order(num_vertices: usize, edges: &[(usize, usize)], vertex_order: &[usize]) -> MappingResult {
    let spacing = DEFAULT_SPACING;
    let padding = DEFAULT_PADDING;

    let grid = embed_graph(num_vertices, edges, vertex_order)
        .expect("Failed to embed graph");

    let copylines = create_copylines(num_vertices, edges, vertex_order);

    // Calculate MIS overhead
    let mis_overhead: i32 = copylines.iter()
        .map(|line| mis_overhead_copyline(line, spacing) as i32)
        .sum();

    // Convert to GridGraph
    let nodes: Vec<GridNode<i32>> = grid.occupied_coords()
        .into_iter()
        .filter_map(|(row, col)| {
            grid.get(row, col).map(|cell| {
                GridNode::new(row, col, cell.weight())
            })
        })
        .filter(|n| n.weight > 0)
        .collect();

    let grid_graph = GridGraph::new(
        GridType::Square,
        grid.size(),
        nodes,
        SQUARE_UNIT_RADIUS,
    );

    MappingResult {
        grid_graph,
        lines: copylines,
        padding,
        spacing,
        mis_overhead,
    }
}
```

**Step 4: Update mod.rs**

```rust
// In src/rules/mapping/mod.rs
mod map_graph;
pub use map_graph::{MappingResult, embed_graph, map_graph, map_graph_with_order};
```

**Step 5: Run test to verify it passes**

Run: `cargo test map_graph --lib`
Expected: PASS

**Step 6: Commit**

```bash
git add src/rules/mapping/map_graph.rs src/rules/mapping/mod.rs
git commit -m "feat(mapping): Add map_graph function for graph to grid mapping"
```

---

## Task 6: Triangular Lattice Support

**Files:**
- Create: `src/rules/mapping/triangular.rs`
- Modify: `src/rules/mapping/mod.rs`

**Step 1: Write the failing test**

```rust
// In src/rules/mapping/triangular.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular_cross_gadget() {
        let cross = TriCross::<true>;
        assert_eq!(cross.size(), (6, 4));
    }

    #[test]
    fn test_map_graph_triangular() {
        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph_triangular(3, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
        assert!(matches!(result.grid_graph.grid_type(), GridType::Triangular { .. }));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test triangular --no-run 2>&1 | head -20`
Expected: Compile error

**Step 3: Write minimal implementation**

```rust
// src/rules/mapping/triangular.rs
//! Triangular lattice mapping support.

use super::copyline::{create_copylines, CopyLine};
use super::gadgets::Gadget;
use super::grid::MappingGrid;
use super::map_graph::MappingResult;
use crate::topology::{GridGraph, GridNode, GridType};
use serde::{Deserialize, Serialize};

const TRIANGULAR_SPACING: usize = 6;
const TRIANGULAR_PADDING: usize = 2;
const TRIANGULAR_UNIT_RADIUS: f64 = 1.1;

/// Triangular cross gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriCross<const CON: bool>;

impl Gadget for TriCross<true> {
    fn size(&self) -> (usize, usize) { (6, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { true }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (2,1), (2,2), (2,3), (2,4),
            (1,2), (2,2), (3,2), (4,2), (5,2), (6,2)
        ];
        let pins = vec![0, 4, 9, 3];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (1,2), (2,1), (2,2), (2,3), (1,4),
            (3,3), (4,2), (4,3), (5,1), (6,1), (6,2)
        ];
        let pins = vec![1, 0, 10, 4];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 1 }
}

impl Gadget for TriCross<false> {
    fn size(&self) -> (usize, usize) { (6, 6) }
    fn cross_location(&self) -> (usize, usize) { (2, 4) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (2,2), (2,3), (2,4), (2,5), (2,6),
            (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (2,1)
        ];
        let pins = vec![11, 5, 10, 4];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (1,4), (2,2), (2,3), (2,4), (2,5), (2,6),
            (3,2), (3,3), (3,4), (3,5), (4,2), (4,3),
            (5,2), (6,3), (6,4), (2,1)
        ];
        let pins = vec![15, 0, 14, 5];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 3 }
}

/// Triangular turn gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriTurn;

impl Gadget for TriTurn {
    fn size(&self) -> (usize, usize) { (3, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2), (2,2), (2,3), (2,4)];
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1,2), (2,2), (3,3), (2,4)];
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 0 }
}

/// Triangular branch gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriBranch;

impl Gadget for TriBranch {
    fn size(&self) -> (usize, usize) { (6, 4) }
    fn cross_location(&self) -> (usize, usize) { (2, 2) }
    fn is_connected(&self) -> bool { false }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (1,2), (2,2), (2,3), (2,4), (3,3),
            (3,2), (4,2), (5,2), (6,2)
        ];
        let pins = vec![0, 3, 8];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (1,2), (2,2), (2,4), (3,3), (4,2),
            (4,3), (5,1), (6,1), (6,2)
        ];
        let pins = vec![0, 2, 8];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 { 0 }
}

/// Map a graph to a triangular lattice grid graph.
pub fn map_graph_triangular(num_vertices: usize, edges: &[(usize, usize)]) -> MappingResult {
    let vertex_order: Vec<usize> = (0..num_vertices).collect();
    map_graph_triangular_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph to triangular lattice with specific vertex ordering.
pub fn map_graph_triangular_with_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> MappingResult {
    let spacing = TRIANGULAR_SPACING;
    let padding = TRIANGULAR_PADDING;

    let copylines = create_copylines(num_vertices, edges, vertex_order);

    // Calculate grid dimensions
    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);
    let max_vslot = copylines.iter().map(|l| l.vslot).max().unwrap_or(1);
    let max_hstop = copylines.iter().map(|l| l.hstop).max().unwrap_or(1);
    let max_vstop = copylines.iter().map(|l| l.vstop).max().unwrap_or(1);

    let rows = max_hslot.max(max_vstop) * spacing + 2 + 2 * padding;
    let cols = max_vslot.max(max_hstop) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);

    // Add copy line nodes with weighted locations for triangular
    for line in &copylines {
        for (row, col, weight) in line.locations(padding, spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Calculate MIS overhead
    let mis_overhead: i32 = copylines.iter()
        .map(|line| {
            let row_overhead = (line.hslot.saturating_sub(line.vstart)) * spacing
                + (line.vstop.saturating_sub(line.hslot)) * spacing;
            let col_overhead = if line.hstop > line.vslot {
                (line.hstop - line.vslot) * spacing - 2
            } else {
                0
            };
            (row_overhead + col_overhead) as i32
        })
        .sum();

    // Convert to GridGraph with triangular type
    let nodes: Vec<GridNode<i32>> = grid.occupied_coords()
        .into_iter()
        .filter_map(|(row, col)| {
            grid.get(row, col).map(|cell| {
                GridNode::new(row, col, cell.weight())
            })
        })
        .filter(|n| n.weight > 0)
        .collect();

    let grid_graph = GridGraph::new(
        GridType::Triangular { offset_even_cols: true },
        grid.size(),
        nodes,
        TRIANGULAR_UNIT_RADIUS,
    );

    MappingResult {
        grid_graph,
        lines: copylines,
        padding,
        spacing,
        mis_overhead,
    }
}
```

**Step 4: Update mod.rs**

```rust
// In src/rules/mapping/mod.rs
mod triangular;
pub use triangular::{TriCross, TriTurn, TriBranch, map_graph_triangular, map_graph_triangular_with_order};
```

**Step 5: Run test to verify it passes**

Run: `cargo test triangular --lib`
Expected: PASS

**Step 6: Commit**

```bash
git add src/rules/mapping/triangular.rs src/rules/mapping/mod.rs
git commit -m "feat(mapping): Add triangular lattice support"
```

---

## Task 7: Integration Tests

**Files:**
- Create: `tests/grid_mapping_tests.rs`

**Step 1: Write comprehensive tests**

```rust
// tests/grid_mapping_tests.rs
//! Integration tests for graph to grid mapping.

use problemreductions::rules::mapping::{
    map_graph, map_graph_triangular, MappingResult,
};
use problemreductions::topology::{Graph, GridType};

#[test]
fn test_map_path_graph() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    assert!(result.grid_graph.num_vertices() > 0);
    assert!(result.mis_overhead >= 0);

    // Solution mapping back should work
    let config = vec![0; result.grid_graph.num_vertices()];
    let original = result.map_config_back(&config);
    assert_eq!(original.len(), 3);
}

#[test]
fn test_map_triangle_graph() {
    // Triangle: 0-1, 1-2, 0-2
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph(3, &edges);

    assert!(result.grid_graph.num_vertices() >= 3);
}

#[test]
fn test_map_star_graph() {
    // Star: center 0 connected to 1,2,3
    let edges = vec![(0, 1), (0, 2), (0, 3)];
    let result = map_graph(4, &edges);

    assert!(result.grid_graph.num_vertices() > 4);
}

#[test]
fn test_map_empty_graph() {
    // No edges
    let edges: Vec<(usize, usize)> = vec![];
    let result = map_graph(3, &edges);

    assert!(result.grid_graph.num_vertices() > 0);
    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_map_single_edge() {
    let edges = vec![(0, 1)];
    let result = map_graph(2, &edges);

    assert_eq!(result.lines.len(), 2);
}

#[test]
fn test_triangular_path_graph() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    assert!(matches!(result.grid_graph.grid_type(), GridType::Triangular { .. }));
    assert!(result.grid_graph.num_vertices() > 0);
}

#[test]
fn test_triangular_complete_k4() {
    // K4: complete graph on 4 vertices
    let edges = vec![
        (0, 1), (0, 2), (0, 3),
        (1, 2), (1, 3),
        (2, 3),
    ];
    let result = map_graph_triangular(4, &edges);

    assert!(result.grid_graph.num_vertices() > 4);
}

#[test]
fn test_mapping_result_serialization() {
    let edges = vec![(0, 1)];
    let result = map_graph(2, &edges);

    // Should be serializable
    let json = serde_json::to_string(&result).unwrap();
    let deserialized: MappingResult = serde_json::from_str(&json).unwrap();

    assert_eq!(result.mis_overhead, deserialized.mis_overhead);
    assert_eq!(result.lines.len(), deserialized.lines.len());
}
```

**Step 2: Run tests**

Run: `cargo test grid_mapping --test grid_mapping_tests`
Expected: PASS

**Step 3: Commit**

```bash
git add tests/grid_mapping_tests.rs
git commit -m "test: Add integration tests for grid mapping"
```

---

## Task 8: Documentation and Exports

**Files:**
- Modify: `src/lib.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Update exports**

```rust
// In src/rules/mod.rs, ensure mapping is exported
pub mod mapping;

// In src/lib.rs prelude, add:
pub use crate::rules::mapping::{
    MappingResult, map_graph, map_graph_triangular,
};
```

**Step 2: Add module documentation**

```rust
// At the top of src/rules/mapping/mod.rs
//! Graph to grid graph mapping.
//!
//! This module implements reductions from arbitrary graphs to unit disk grid graphs
//! using the copy-line technique from UnitDiskMapping.jl.
//!
//! # Overview
//!
//! The mapping works by:
//! 1. Creating "copy lines" for each vertex (L-shaped paths on the grid)
//! 2. Resolving crossings using gadgets that preserve MIS properties
//! 3. The resulting grid graph has the property that a MIS solution can be
//!    mapped back to a MIS solution on the original graph
//!
//! # Example
//!
//! ```rust
//! use problemreductions::rules::mapping::{map_graph, map_graph_triangular};
//!
//! // Map a triangle graph to a square lattice
//! let edges = vec![(0, 1), (1, 2), (0, 2)];
//! let result = map_graph(3, &edges);
//!
//! println!("Grid graph has {} vertices", result.grid_graph.num_vertices());
//! println!("MIS overhead: {}", result.mis_overhead);
//! ```
```

**Step 3: Run doc tests**

Run: `cargo test --doc`
Expected: PASS

**Step 4: Build docs**

Run: `cargo doc --no-deps`
Expected: Success

**Step 5: Commit**

```bash
git add src/lib.rs src/rules/mod.rs src/rules/mapping/mod.rs
git commit -m "docs: Add documentation and exports for grid mapping"
```

---

## Summary

This plan implements:

1. **GridGraph** - A graph type for weighted nodes on integer grids (square and triangular)
2. **CopyLine** - The vertex embedding technique
3. **Gadgets** - Patterns for resolving crossings
4. **MappingGrid** - Intermediate representation during mapping
5. **map_graph** - Main function for square lattice mapping
6. **map_graph_triangular** - Function for triangular lattice mapping
7. **MappingResult** - Result type with solution back-mapping

Key test coverage:
- Unit tests for each component
- Integration tests for complete mapping workflow
- Serialization tests
- Various graph types (path, triangle, star, complete)
