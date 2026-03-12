//! Minesweeper Consistency problem implementation.
//!
//! Given a partially revealed Minesweeper grid, determine if there exists a valid
//! mine assignment for unrevealed cells that satisfies all revealed cell constraints.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "Minesweeper",
        module_path: module_path!(),
        description: "Determine if a partially revealed Minesweeper grid has a consistent mine assignment",
        fields: &[
            FieldInfo { name: "rows", type_name: "usize", description: "Number of rows in the grid" },
            FieldInfo { name: "cols", type_name: "usize", description: "Number of columns in the grid" },
            FieldInfo { name: "revealed", type_name: "Vec<(usize, usize, u8)>", description: "Revealed cells (row, col, adjacent mine count)" },
            FieldInfo { name: "unrevealed", type_name: "Vec<(usize, usize)>", description: "Unrevealed cells (row, col)" },
        ],
    }
}

/// The Minesweeper Consistency problem.
///
/// Given a partially revealed Minesweeper grid with `rows x cols` cells,
/// some cells are revealed showing the count of adjacent mines, and some
/// cells are unrevealed (potential mine locations). The problem asks whether
/// there exists an assignment of mines to unrevealed cells such that every
/// revealed cell's count constraint is satisfied.
///
/// This is a satisfaction (decision) problem and is NP-complete.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::Minesweeper;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 3x3 grid, center revealed with count 1
/// let problem = Minesweeper::new(
///     3, 3,
///     vec![(1, 1, 1)],
///     vec![(0,0),(0,1),(0,2),(1,0),(1,2),(2,0),(2,1),(2,2)],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minesweeper {
    /// Number of rows in the grid.
    rows: usize,
    /// Number of columns in the grid.
    cols: usize,
    /// Revealed cells: (row, col, adjacent_mine_count).
    revealed: Vec<(usize, usize, u8)>,
    /// Unrevealed cells: (row, col).
    unrevealed: Vec<(usize, usize)>,
}

impl Minesweeper {
    /// Create a new Minesweeper Consistency problem.
    ///
    /// # Arguments
    /// * `rows` - Number of rows
    /// * `cols` - Number of columns
    /// * `revealed` - Revealed cells with their adjacent mine counts
    /// * `unrevealed` - Unrevealed cells (potential mine locations)
    ///
    /// # Panics
    /// Panics if any cell position is out of bounds, if mine counts exceed 8,
    /// or if revealed and unrevealed positions overlap.
    pub fn new(
        rows: usize,
        cols: usize,
        revealed: Vec<(usize, usize, u8)>,
        unrevealed: Vec<(usize, usize)>,
    ) -> Self {
        let mut all_positions = std::collections::HashSet::new();
        for &(r, c, count) in &revealed {
            assert!(
                r < rows && c < cols,
                "Revealed cell ({r}, {c}) out of bounds for {rows}x{cols} grid"
            );
            assert!(count <= 8, "Mine count {count} exceeds maximum of 8");
            assert!(
                all_positions.insert((r, c)),
                "Duplicate position ({r}, {c}) in revealed cells"
            );
        }
        for &(r, c) in &unrevealed {
            assert!(
                r < rows && c < cols,
                "Unrevealed cell ({r}, {c}) out of bounds for {rows}x{cols} grid"
            );
            assert!(
                all_positions.insert((r, c)),
                "Position ({r}, {c}) appears in both revealed and unrevealed cells"
            );
        }
        Self {
            rows,
            cols,
            revealed,
            unrevealed,
        }
    }

    /// Get the number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of columns.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get the revealed cells.
    pub fn revealed(&self) -> &[(usize, usize, u8)] {
        &self.revealed
    }

    /// Get the unrevealed cells.
    pub fn unrevealed(&self) -> &[(usize, usize)] {
        &self.unrevealed
    }

    /// Get the number of unrevealed cells.
    pub fn num_unrevealed(&self) -> usize {
        self.unrevealed.len()
    }
}

impl Problem for Minesweeper {
    const NAME: &'static str = "Minesweeper";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.unrevealed.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.unrevealed.len() {
            return false;
        }

        // Build position -> index map for unrevealed cells
        let pos_to_idx: HashMap<(usize, usize), usize> = self
            .unrevealed
            .iter()
            .enumerate()
            .map(|(i, &(r, c))| ((r, c), i))
            .collect();

        // Neighbor offsets (8-connected)
        let deltas: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        // Check each revealed cell's constraint
        for &(r, c, count) in &self.revealed {
            let mut mine_count: u8 = 0;
            for &(dr, dc) in &deltas {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.cols as i32 {
                    let pos = (nr as usize, nc as usize);
                    if let Some(&idx) = pos_to_idx.get(&pos) {
                        if config[idx] == 1 {
                            mine_count += 1;
                        }
                    }
                }
            }
            if mine_count != count {
                return false;
            }
        }
        true
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for Minesweeper {}

crate::declare_variants! {
    Minesweeper => "2^num_unrevealed",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minesweeper.rs"]
mod tests;
