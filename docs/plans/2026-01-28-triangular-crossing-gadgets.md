# Triangular Crossing Gadgets Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement `apply_triangular_crossing_gadgets` to complete the triangular mapping pipeline, enabling all ignored triangular MIS tests to pass.

**Architecture:** Add pattern matching and gadget application for triangular lattice in `triangular.rs`. The approach mirrors the square lattice `apply_crossing_gadgets` in `gadgets.rs` but uses `TriangularGadget` trait. Iterate through vertex pairs, find crossings, match patterns, apply transformations.

**Tech Stack:** Rust, existing `TriangularGadget` trait, `MappingGrid`, `CopyLine`.

---

## Overview

The triangular crossing gadget ruleset (13 gadgets):
1. `TriCross<false>` - disconnected crossing (mis_overhead: 3)
2. `TriCross<true>` - connected crossing (mis_overhead: 1)
3. `TriTConLeft` (mis_overhead: 4)
4. `TriTConUp` (mis_overhead: 0)
5. `TriTConDown` (mis_overhead: 0)
6. `TriTrivialTurnLeft` (mis_overhead: 0)
7. `TriTrivialTurnRight` (mis_overhead: 0)
8. `TriEndTurn` (mis_overhead: -2)
9. `TriTurn` (mis_overhead: 0)
10. `TriWTurn` (mis_overhead: 0)
11. `TriBranchFix` (mis_overhead: -2)
12. `TriBranchFixB` (mis_overhead: -2)
13. `TriBranch` (mis_overhead: 0)

---

### Task 1: Add source_matrix and mapped_matrix methods to TriangularGadget

**Files:**
- Modify: `src/rules/mapping/triangular.rs`

**Step 1: Add helper methods to TriangularGadget trait**

Add after line 27 in `triangular.rs`:

```rust
/// Generate source matrix for pattern matching.
fn source_matrix(&self) -> Vec<Vec<bool>> {
    let (rows, cols) = self.size();
    let (locs, _, _) = self.source_graph();
    let mut matrix = vec![vec![false; cols]; rows];
    for (r, c) in locs {
        if r > 0 && c > 0 && r <= rows && c <= cols {
            matrix[r - 1][c - 1] = true;
        }
    }
    matrix
}

/// Generate mapped matrix for gadget application.
fn mapped_matrix(&self) -> Vec<Vec<bool>> {
    let (rows, cols) = self.size();
    let (locs, _) = self.mapped_graph();
    let mut matrix = vec![vec![false; cols]; rows];
    for (r, c) in locs {
        if r > 0 && c > 0 && r <= rows && c <= cols {
            matrix[r - 1][c - 1] = true;
        }
    }
    matrix
}
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: SUCCESS

**Step 3: Commit**

```bash
git add src/rules/mapping/triangular.rs
git commit -m "feat: add source_matrix and mapped_matrix to TriangularGadget"
```

---

### Task 2: Add triangular pattern matching function

**Files:**
- Modify: `src/rules/mapping/triangular.rs`

**Step 1: Add pattern_matches_triangular function**

Add before `map_graph_triangular`:

```rust
/// Check if a triangular gadget pattern matches at position (i, j) in the grid.
/// i, j are 0-indexed row/col offsets.
fn pattern_matches_triangular<G: TriangularGadget>(
    gadget: &G,
    grid: &MappingGrid,
    i: usize,
    j: usize,
) -> bool {
    let source = gadget.source_matrix();
    let (m, n) = gadget.size();

    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;
            let expected_occupied = source[r][c];
            let actual_occupied = grid
                .get(grid_r, grid_c)
                .map(|cell| !cell.is_empty())
                .unwrap_or(false);

            if expected_occupied != actual_occupied {
                return false;
            }
        }
    }
    true
}
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: SUCCESS

**Step 3: Commit**

```bash
git add src/rules/mapping/triangular.rs
git commit -m "feat: add pattern_matches_triangular function"
```

---

### Task 3: Add triangular gadget application function

**Files:**
- Modify: `src/rules/mapping/triangular.rs`

**Step 1: Add apply_triangular_gadget function**

Add after `pattern_matches_triangular`:

```rust
/// Apply a triangular gadget pattern at position (i, j).
fn apply_triangular_gadget<G: TriangularGadget>(
    gadget: &G,
    grid: &mut MappingGrid,
    i: usize,
    j: usize,
) {
    let source = gadget.source_matrix();
    let mapped = gadget.mapped_matrix();
    let (m, n) = gadget.size();

    // First, clear source pattern cells
    for r in 0..m {
        for c in 0..n {
            if source[r][c] {
                grid.clear(i + r, j + c);
            }
        }
    }

    // Then, add mapped pattern cells
    for r in 0..m {
        for c in 0..n {
            if mapped[r][c] {
                grid.add_node(i + r, j + c, 1);
            }
        }
    }
}
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: SUCCESS

**Step 3: Commit**

```bash
git add src/rules/mapping/triangular.rs
git commit -m "feat: add apply_triangular_gadget function"
```

---

### Task 4: Add TriangularTapeEntry and crossat_triangular

**Files:**
- Modify: `src/rules/mapping/triangular.rs`

**Step 1: Add tape entry struct and crossat function**

Add after constants at top of file:

```rust
/// Tape entry recording a triangular gadget application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangularTapeEntry {
    /// Index of the gadget in the ruleset (0-12).
    pub gadget_idx: usize,
    /// Row where gadget was applied.
    pub row: usize,
    /// Column where gadget was applied.
    pub col: usize,
}

/// Calculate crossing point for two copylines on triangular lattice.
fn crossat_triangular(
    copylines: &[super::copyline::CopyLine],
    v: usize,
    w: usize,
    spacing: usize,
    padding: usize,
) -> (usize, usize) {
    let line_v = &copylines[v];
    let line_w = &copylines[w];

    // Use vslot to determine order
    let (line_first, line_second) = if line_v.vslot < line_w.vslot {
        (line_v, line_w)
    } else {
        (line_w, line_v)
    };

    let hslot = line_first.hslot;
    let max_vslot = line_second.vslot;

    let row = (hslot - 1) * spacing + 2 + padding;
    let col = (max_vslot - 1) * spacing + 1 + padding;

    (row, col)
}
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: SUCCESS

**Step 3: Commit**

```bash
git add src/rules/mapping/triangular.rs
git commit -m "feat: add TriangularTapeEntry and crossat_triangular"
```

---

### Task 5: Implement apply_triangular_crossing_gadgets

**Files:**
- Modify: `src/rules/mapping/triangular.rs`

**Step 1: Add the main function**

Add after `crossat_triangular`:

```rust
/// Apply all triangular crossing gadgets to resolve crossings.
/// Returns the tape of applied gadgets.
pub fn apply_triangular_crossing_gadgets(
    grid: &mut MappingGrid,
    copylines: &[super::copyline::CopyLine],
    spacing: usize,
    padding: usize,
) -> Vec<TriangularTapeEntry> {
    use std::collections::HashSet;

    let mut tape = Vec::new();
    let mut processed = HashSet::new();
    let n = copylines.len();

    // Triangular crossing ruleset (order matters - try in this order)
    let gadgets: Vec<Box<dyn Fn() -> (usize, usize, i32, Box<dyn Fn(&MappingGrid, usize, usize) -> bool>, Box<dyn Fn(&mut MappingGrid, usize, usize)>)>> = vec![
        // We'll use a simpler approach - try each gadget type directly
    ];

    // Iterate through all pairs of vertices
    for j in 0..n {
        for i in 0..n {
            let (cross_row, cross_col) = crossat_triangular(copylines, i, j, spacing, padding);

            if processed.contains(&(cross_row, cross_col)) {
                continue;
            }

            // Try each gadget in the ruleset
            if let Some(entry) = try_match_triangular_gadget(grid, cross_row, cross_col) {
                tape.push(entry);
                processed.insert((cross_row, cross_col));
            }
        }
    }

    tape
}

/// Try to match and apply a triangular gadget at the crossing point.
fn try_match_triangular_gadget(
    grid: &mut MappingGrid,
    cross_row: usize,
    cross_col: usize,
) -> Option<TriangularTapeEntry> {
    // Macro to reduce repetition
    macro_rules! try_gadget {
        ($gadget:expr, $idx:expr) => {{
            let g = $gadget;
            let (cr, cc) = g.cross_location();
            if cross_row >= cr && cross_col >= cc {
                let x = cross_row - cr + 1;
                let y = cross_col - cc + 1;
                if pattern_matches_triangular(&g, grid, x, y) {
                    apply_triangular_gadget(&g, grid, x, y);
                    return Some(TriangularTapeEntry {
                        gadget_idx: $idx,
                        row: x,
                        col: y,
                    });
                }
            }
        }};
    }

    // Try gadgets in order (matching Julia's triangular_crossing_ruleset)
    try_gadget!(TriCross::<false>, 0);
    try_gadget!(TriCross::<true>, 1);
    try_gadget!(TriTConLeft, 2);
    try_gadget!(TriTConUp, 3);
    try_gadget!(TriTConDown, 4);
    try_gadget!(TriTrivialTurnLeft, 5);
    try_gadget!(TriTrivialTurnRight, 6);
    try_gadget!(TriEndTurn, 7);
    try_gadget!(TriTurn, 8);
    try_gadget!(TriWTurn, 9);
    try_gadget!(TriBranchFix, 10);
    try_gadget!(TriBranchFixB, 11);
    try_gadget!(TriBranch, 12);

    None
}

/// Get MIS overhead for a triangular tape entry.
pub fn triangular_tape_entry_mis_overhead(entry: &TriangularTapeEntry) -> i32 {
    match entry.gadget_idx {
        0 => TriCross::<false>.mis_overhead(),
        1 => TriCross::<true>.mis_overhead(),
        2 => TriTConLeft.mis_overhead(),
        3 => TriTConUp.mis_overhead(),
        4 => TriTConDown.mis_overhead(),
        5 => TriTrivialTurnLeft.mis_overhead(),
        6 => TriTrivialTurnRight.mis_overhead(),
        7 => TriEndTurn.mis_overhead(),
        8 => TriTurn.mis_overhead(),
        9 => TriWTurn.mis_overhead(),
        10 => TriBranchFix.mis_overhead(),
        11 => TriBranchFixB.mis_overhead(),
        12 => TriBranch.mis_overhead(),
        _ => 0,
    }
}
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: SUCCESS

**Step 3: Commit**

```bash
git add src/rules/mapping/triangular.rs
git commit -m "feat: add apply_triangular_crossing_gadgets function"
```

---

### Task 6: Integrate into map_graph_triangular_with_order

**Files:**
- Modify: `src/rules/mapping/triangular.rs`

**Step 1: Update map_graph_triangular_with_order to use gadgets**

Replace the section after "Add copy line nodes" (around line 689) with:

```rust
    // Add copy line nodes
    for line in &copylines {
        for (row, col, weight) in line.locations(padding, spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Apply crossing gadgets
    let tape = apply_triangular_crossing_gadgets(&mut grid, &copylines, spacing, padding);

    // Calculate MIS overhead from copylines
    let copyline_overhead: i32 = copylines
        .iter()
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

    // Add gadget overhead
    let gadget_overhead: i32 = tape.iter().map(triangular_tape_entry_mis_overhead).sum();
    let mis_overhead = copyline_overhead + gadget_overhead;
```

Also update `MappingResult` creation to include tape (need to convert to generic tape format or update MappingResult).

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: SUCCESS

**Step 3: Commit**

```bash
git add src/rules/mapping/triangular.rs
git commit -m "feat: integrate apply_triangular_crossing_gadgets into mapping"
```

---

### Task 7: Run tests and verify ignored tests now pass

**Files:**
- Test: `tests/grid_mapping_tests.rs`

**Step 1: Run the triangular MIS tests**

Run: `cargo test --features ilp triangular_mis_verification -- --include-ignored --nocapture`

Check which tests pass now. The key tests:
- `test_triangular_map_configurations_back`
- `test_triangular_interface_full`
- `test_triangular_config_count_preserved`

**Step 2: Remove #[ignore] from passing tests**

If tests pass, remove the `#[ignore]` attributes.

**Step 3: Fix any failing tests**

Debug pattern matching or gadget application if tests fail.

**Step 4: Commit**

```bash
git add tests/grid_mapping_tests.rs src/rules/mapping/triangular.rs
git commit -m "feat: enable triangular MIS verification tests"
```

---

### Task 8: Export new functions in mod.rs

**Files:**
- Modify: `src/rules/mapping/mod.rs`

**Step 1: Add exports**

Add to the triangular exports:

```rust
pub use triangular::{
    ...,
    apply_triangular_crossing_gadgets, triangular_tape_entry_mis_overhead, TriangularTapeEntry,
};
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: SUCCESS

**Step 3: Commit**

```bash
git add src/rules/mapping/mod.rs
git commit -m "feat: export triangular crossing gadget functions"
```

---

### Task 9: Run full test suite

**Step 1: Run all tests**

```bash
cargo test --features ilp
```

Expected: All tests pass (previously ignored triangular tests should now pass)

**Step 2: Verify test count**

```bash
cargo test --features ilp 2>&1 | grep "test result"
```

Expected: More passing tests, fewer ignored tests

**Step 3: Final commit**

```bash
git add -A
git commit -m "feat: complete triangular crossing gadget implementation"
```

---

## Summary

After completing this plan:
- `apply_triangular_crossing_gadgets` resolves crossings on triangular lattice
- `map_graph_triangular` produces correct grid graphs with proper MIS overhead
- Previously ignored tests (`test_triangular_map_configurations_back`, etc.) should pass
- Full parity with Julia's UnitDiskMapping.jl for triangular mode
