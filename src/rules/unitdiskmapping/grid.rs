//! Mapping grid for intermediate representation during graph embedding.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Cell state in the mapping grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CellState {
    #[default]
    Empty,
    Occupied { weight: i32 },
    Doubled { weight: i32 },
    Connected { weight: i32 },
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
    /// Silently ignores out-of-bounds access.
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
    ///
    /// Silently ignores out-of-bounds access.
    pub fn connect(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            if let CellState::Occupied { weight } = self.content[row][col] {
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
mod tests {
    use super::*;

    #[test]
    fn test_mapping_grid_create() {
        let grid = MappingGrid::new(10, 10, 4);
        assert_eq!(grid.size(), (10, 10));
        assert_eq!(grid.spacing(), 4);
    }

    #[test]
    fn test_mapping_grid_with_padding() {
        let grid = MappingGrid::with_padding(8, 12, 3, 5);
        assert_eq!(grid.size(), (8, 12));
        assert_eq!(grid.spacing(), 3);
        assert_eq!(grid.padding(), 5);
    }

    #[test]
    fn test_mapping_grid_add_node() {
        let mut grid = MappingGrid::new(10, 10, 4);
        grid.add_node(2, 3, 1);
        assert!(grid.is_occupied(2, 3));
        assert!(!grid.is_occupied(2, 4));
    }

    #[test]
    fn test_mapping_grid_get_out_of_bounds() {
        let grid = MappingGrid::new(5, 5, 2);
        assert!(grid.get(0, 0).is_some());
        assert!(grid.get(4, 4).is_some());
        assert!(grid.get(5, 0).is_none());
        assert!(grid.get(0, 5).is_none());
        assert!(grid.get(10, 10).is_none());
    }

    #[test]
    fn test_mapping_grid_add_node_doubled() {
        let mut grid = MappingGrid::new(10, 10, 4);
        grid.add_node(2, 3, 5);
        assert_eq!(
            grid.get(2, 3),
            Some(&CellState::Occupied { weight: 5 })
        );
        grid.add_node(2, 3, 3);
        assert_eq!(
            grid.get(2, 3),
            Some(&CellState::Doubled { weight: 8 })
        );
    }

    #[test]
    fn test_mapping_grid_connect() {
        let mut grid = MappingGrid::new(10, 10, 4);
        grid.add_node(3, 4, 7);
        assert_eq!(
            grid.get(3, 4),
            Some(&CellState::Occupied { weight: 7 })
        );
        grid.connect(3, 4);
        assert_eq!(
            grid.get(3, 4),
            Some(&CellState::Connected { weight: 7 })
        );
    }

    #[test]
    fn test_mapping_grid_connect_empty_cell() {
        let mut grid = MappingGrid::new(10, 10, 4);
        grid.connect(3, 4);
        assert_eq!(grid.get(3, 4), Some(&CellState::Empty));
    }

    #[test]
    fn test_mapping_grid_matches_pattern() {
        let mut grid = MappingGrid::new(10, 10, 4);
        grid.add_node(2, 2, 1);
        grid.add_node(2, 3, 1);
        grid.add_node(3, 2, 1);

        let pattern = vec![(0, 0), (0, 1), (1, 0)];
        assert!(grid.matches_pattern(&pattern, 2, 2));
        assert!(!grid.matches_pattern(&pattern, 0, 0));
    }

    #[test]
    fn test_mapping_grid_matches_pattern_out_of_bounds() {
        let grid = MappingGrid::new(5, 5, 2);
        let pattern = vec![(0, 0), (1, 1)];
        assert!(!grid.matches_pattern(&pattern, 10, 10));
    }

    #[test]
    fn test_mapping_grid_cross_at() {
        let grid = MappingGrid::new(20, 20, 4);
        // Julia's crossat uses larger position for col calculation
        let (row, col) = grid.cross_at(1, 3, 2);
        assert_eq!(row, 4 + 2 + 2); // (hslot - 1) * spacing + 2 + padding
        assert_eq!(col, (3 - 1) * 4 + 1 + 2); // (larger_vslot - 1) * spacing + 1 + padding

        let (row2, col2) = grid.cross_at(3, 1, 2);
        assert_eq!((row, col), (row2, col2));
    }

    #[test]
    fn test_cell_state_weight() {
        assert_eq!(CellState::Empty.weight(), 0);
        assert_eq!(CellState::Occupied { weight: 5 }.weight(), 5);
        assert_eq!(CellState::Doubled { weight: 10 }.weight(), 10);
        assert_eq!(CellState::Connected { weight: 3 }.weight(), 3);
    }

    #[test]
    fn test_cell_state_is_empty() {
        assert!(CellState::Empty.is_empty());
        assert!(!CellState::Occupied { weight: 1 }.is_empty());
        assert!(!CellState::Doubled { weight: 2 }.is_empty());
        assert!(!CellState::Connected { weight: 1 }.is_empty());
    }

    #[test]
    fn test_cell_state_is_occupied() {
        assert!(!CellState::Empty.is_occupied());
        assert!(CellState::Occupied { weight: 1 }.is_occupied());
        assert!(CellState::Doubled { weight: 2 }.is_occupied());
        assert!(CellState::Connected { weight: 1 }.is_occupied());
    }

    #[test]
    fn test_mapping_grid_set() {
        let mut grid = MappingGrid::new(5, 5, 2);
        grid.set(2, 3, CellState::Occupied { weight: 7 });
        assert_eq!(grid.get(2, 3), Some(&CellState::Occupied { weight: 7 }));

        // Out of bounds set should be ignored
        grid.set(10, 10, CellState::Occupied { weight: 1 });
        assert!(grid.get(10, 10).is_none());
    }

    #[test]
    fn test_mapping_grid_get_mut() {
        let mut grid = MappingGrid::new(5, 5, 2);
        grid.add_node(1, 1, 3);

        if let Some(cell) = grid.get_mut(1, 1) {
            *cell = CellState::Connected { weight: 5 };
        }
        assert_eq!(grid.get(1, 1), Some(&CellState::Connected { weight: 5 }));

        // Out of bounds get_mut should return None
        assert!(grid.get_mut(10, 10).is_none());
    }

    #[test]
    fn test_mapping_grid_occupied_coords() {
        let mut grid = MappingGrid::new(5, 5, 2);
        grid.add_node(1, 2, 1);
        grid.add_node(3, 4, 2);
        grid.add_node(0, 0, 1);

        let coords = grid.occupied_coords();
        assert_eq!(coords.len(), 3);
        assert!(coords.contains(&(0, 0)));
        assert!(coords.contains(&(1, 2)));
        assert!(coords.contains(&(3, 4)));
    }

    #[test]
    fn test_mapping_grid_add_node_out_of_bounds() {
        let mut grid = MappingGrid::new(5, 5, 2);
        // Should silently ignore out of bounds
        grid.add_node(10, 10, 1);
        assert!(grid.get(10, 10).is_none());
    }

    #[test]
    fn test_mapping_grid_connect_out_of_bounds() {
        let mut grid = MappingGrid::new(5, 5, 2);
        // Should silently ignore out of bounds
        grid.connect(10, 10);
    }

    #[test]
    fn test_cell_state_display() {
        assert_eq!(format!("{}", CellState::Empty), "⋅");
        assert_eq!(format!("{}", CellState::Occupied { weight: 1 }), "●");
        assert_eq!(format!("{}", CellState::Doubled { weight: 2 }), "◉");
        assert_eq!(format!("{}", CellState::Connected { weight: 1 }), "◇");
    }

    #[test]
    fn test_mapping_grid_display() {
        let mut grid = MappingGrid::new(3, 3, 2);
        grid.add_node(0, 0, 1);
        grid.add_node(1, 1, 1);
        let display = format!("{}", grid);
        assert!(display.contains("●")); // Has occupied nodes
        assert!(display.contains("⋅")); // Has empty cells
    }

    #[test]
    fn test_mapping_grid_format_with_config_none() {
        let mut grid = MappingGrid::new(3, 3, 2);
        grid.add_node(1, 1, 1);
        let output = grid.format_with_config(None);
        assert!(output.contains("●")); // Occupied nodes
    }

    #[test]
    fn test_mapping_grid_format_with_config_some() {
        let mut grid = MappingGrid::new(3, 3, 2);
        grid.add_node(1, 1, 1);
        // Config with node at (1,1) selected
        let config = vec![0, 0, 0, 0, 1, 0, 0, 0, 0]; // 3x3 = 9 cells
        let output = grid.format_with_config(Some(&config));
        // Should have some output
        assert!(!output.is_empty());
    }
}
