//! Mapping grid for intermediate representation during graph embedding.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Cell state in the mapping grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CellState {
    #[default]
    Empty,
    Occupied {
        weight: i32,
    },
    Doubled {
        weight: i32,
    },
    Connected {
        weight: i32,
    },
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
    ///
    /// Silently ignores out-of-bounds access.
    pub fn set(&mut self, row: usize, col: usize, state: CellState) {
        if row < self.rows && col < self.cols {
            self.content[row][col] = state;
        }
    }

    /// Add a node at position.
    ///
    /// For weighted mode (triangular), Julia's add_cell! asserts that when doubling,
    /// the new weight equals the existing weight, and keeps that weight (doesn't add).
    /// For unweighted mode, all weights are 1 so this doesn't matter.
    ///
    /// Silently ignores out-of-bounds access.
    pub fn add_node(&mut self, row: usize, col: usize, weight: i32) {
        if row < self.rows && col < self.cols {
            match self.content[row][col] {
                CellState::Empty => {
                    self.content[row][col] = CellState::Occupied { weight };
                }
                CellState::Occupied { weight: w } => {
                    // Julia: @assert m[i,j].weight == node.weight; keeps same weight
                    // For weighted mode, both should be equal; for unweighted mode, both are 1
                    debug_assert!(
                        w == weight,
                        "When doubling, weights should match: {} != {}",
                        w,
                        weight
                    );
                    self.content[row][col] = CellState::Doubled { weight };
                }
                _ => {}
            }
        }
    }

    /// Mark a cell as connected.
    ///
    /// Julia's connect_cell! converts a plain Occupied cell (MCell()) to a Connected cell.
    /// It errors if the cell is NOT MCell() (i.e., doubled, empty, or already connected).
    /// This matches that behavior - converts Occupied cells to Connected.
    /// Silently ignores out-of-bounds access.
    pub fn connect(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            if let CellState::Occupied { weight } = self.content[row][col] {
                // Julia: converts plain Occupied cell (MCell()) to Connected cell
                self.content[row][col] = CellState::Connected { weight };
            }
        }
    }

    /// Check if a pattern matches at position.
    pub fn matches_pattern(
        &self,
        pattern: &[(usize, usize)],
        offset_row: usize,
        offset_col: usize,
    ) -> bool {
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

    /// Get all doubled cell coordinates.
    /// Returns a set of (row, col) for cells in the Doubled state.
    pub fn doubled_cells(&self) -> std::collections::HashSet<(usize, usize)> {
        let mut cells = std::collections::HashSet::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                if matches!(self.content[r][c], CellState::Doubled { .. }) {
                    cells.insert((r, c));
                }
            }
        }
        cells
    }

    /// Get cross location for two vertices.
    /// Julia's crossat uses smaller position's hslot for row and larger position for col.
    ///
    /// Note: All slot parameters are 1-indexed (must be >= 1).
    /// Returns 0-indexed (row, col) coordinates.
    ///
    /// Julia formula (1-indexed): (hslot-1)*spacing + 2 + padding, (vslot-1)*spacing + 1 + padding
    /// Rust formula (0-indexed): subtract 1 from each coordinate
    pub fn cross_at(&self, v_slot: usize, w_slot: usize, h_slot: usize) -> (usize, usize) {
        debug_assert!(h_slot >= 1, "h_slot must be >= 1 (1-indexed)");
        debug_assert!(v_slot >= 1, "v_slot must be >= 1 (1-indexed)");
        debug_assert!(w_slot >= 1, "w_slot must be >= 1 (1-indexed)");
        let larger_slot = v_slot.max(w_slot);
        // 0-indexed coordinates (Julia's formula minus 1)
        let row = (h_slot - 1) * self.spacing + 1 + self.padding;
        let col = (larger_slot - 1) * self.spacing + self.padding;
        (row, col)
    }

    /// Format the grid as a string matching Julia's UnitDiskMapping format.
    ///
    /// Characters (matching Julia exactly):
    /// - `⋅` = empty cell
    /// - `●` = occupied cell (weight=1 or 2)
    /// - `◉` = doubled cell (two copy lines overlap)
    /// - `◆` = connected cell (weight=2)
    /// - `◇` = connected cell (weight=1)
    /// - `▴` = cell with weight >= 3
    /// - Each cell is followed by a space
    ///
    /// With configuration:
    /// - `●` = selected node (config=1)
    /// - `○` = unselected node (config=0)
    pub fn format_with_config(&self, config: Option<&[usize]>) -> String {
        use std::collections::HashMap;

        // Build position to config index map if config is provided
        let pos_to_idx: HashMap<(usize, usize), usize> = if config.is_some() {
            let mut map = HashMap::new();
            let mut idx = 0;
            for r in 0..self.rows {
                for c in 0..self.cols {
                    if self.content[r][c].is_occupied() {
                        map.insert((r, c), idx);
                        idx += 1;
                    }
                }
            }
            map
        } else {
            HashMap::new()
        };

        let mut lines = Vec::new();

        for r in 0..self.rows {
            let mut line = String::new();
            for c in 0..self.cols {
                let cell = &self.content[r][c];
                let s = if let Some(cfg) = config {
                    if let Some(&idx) = pos_to_idx.get(&(r, c)) {
                        if cfg.get(idx).copied().unwrap_or(0) > 0 {
                            "●" // Selected node
                        } else {
                            "○" // Unselected node
                        }
                    } else {
                        "⋅" // Empty
                    }
                } else {
                    Self::cell_str(cell)
                };
                line.push_str(s);
                line.push(' ');
            }
            // Remove trailing space
            line.pop();
            lines.push(line);
        }

        lines.join("\n")
    }

    /// Get the string representation of a cell (matching Julia's print_cell).
    fn cell_str(cell: &CellState) -> &'static str {
        match cell {
            CellState::Empty => "⋅",
            CellState::Occupied { weight } => {
                if *weight >= 3 {
                    "▴"
                } else {
                    "●"
                }
            }
            CellState::Doubled { .. } => "◉",
            CellState::Connected { weight } => {
                if *weight == 1 {
                    "◇"
                } else {
                    "◆"
                }
            }
        }
    }
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", MappingGrid::cell_str(self))
    }
}

impl fmt::Display for MappingGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_with_config(None))
    }
}

#[cfg(test)]
#[path = "../../tests_unit/rules/unitdiskmapping/grid.rs"]
mod tests;
