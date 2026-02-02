# KSG and Triangular Lattice Refactoring Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refactor unitdiskmapping module to use independent gadget implementations organized by lattice type (KSG and Triangular).

**Architecture:** Create `ksg/` and `triangular/` submodules, rename gadgets with proper prefixes (Ksg*, WeightedKsg*, WeightedTri*), split mapping functions, delete old files, and update all imports.

**Tech Stack:** Rust, serde

---

## Overview

| Current File | Lines | Action |
|--------------|-------|--------|
| `gadgets.rs` | 455 | Move Pattern trait to `ksg/traits.rs`, delete rest |
| `gadgets_unweighted.rs` | 1369 | Rename to `ksg/gadgets.rs`, prefix with `Ksg` |
| `map_graph.rs` | 805 | Move to `ksg/mapping.rs` |
| `triangular.rs` | 1620 | Split to `triangular/gadgets.rs` + `triangular/mapping.rs`, prefix with `WeightedTri` |
| `weighted.rs` | 584 | Delete (logic absorbed into weighted gadgets) |

---

## Task 1: Create Directory Structure

**Files:**
- Create: `src/rules/unitdiskmapping/ksg/mod.rs`
- Create: `src/rules/unitdiskmapping/triangular/mod.rs`

**Step 1: Create ksg directory and mod.rs**

```bash
mkdir -p src/rules/unitdiskmapping/ksg
```

Create `src/rules/unitdiskmapping/ksg/mod.rs`:
```rust
//! King's Subgraph (KSG) mapping module.
//!
//! Maps arbitrary graphs to King's Subgraph (8-connected grid graphs).
//! Supports both unweighted and weighted modes.

mod gadgets;
mod gadgets_weighted;
mod mapping;

pub use gadgets::*;
pub use gadgets_weighted::*;
pub use mapping::*;

/// Spacing between copy lines for KSG mapping.
pub const SPACING: usize = 4;

/// Padding around the grid for KSG mapping.
pub const PADDING: usize = 2;
```

**Step 2: Create triangular directory and mod.rs**

```bash
mkdir -p src/rules/unitdiskmapping/triangular
```

Create `src/rules/unitdiskmapping/triangular/mod.rs`:
```rust
//! Triangular lattice mapping module.
//!
//! Maps arbitrary graphs to weighted triangular lattice graphs.

mod gadgets;
mod mapping;

pub use gadgets::*;
pub use mapping::*;

/// Spacing between copy lines for triangular mapping.
pub const SPACING: usize = 6;

/// Padding around the grid for triangular mapping.
pub const PADDING: usize = 2;
```

**Step 3: Verify directories exist**

Run: `ls -la src/rules/unitdiskmapping/ksg/ src/rules/unitdiskmapping/triangular/`

**Step 4: Commit**

```bash
git add src/rules/unitdiskmapping/ksg/ src/rules/unitdiskmapping/triangular/
git commit -m "feat: create ksg/ and triangular/ directory structure"
```

---

## Task 2: Create Shared Traits Module

**Files:**
- Create: `src/rules/unitdiskmapping/traits.rs`
- Modify: `src/rules/unitdiskmapping/mod.rs`

**Step 1: Create traits.rs with Pattern trait and PatternCell**

Extract from `gadgets.rs` lines 22-200 (Pattern trait, PatternCell, pattern_matches, apply_gadget, unapply_gadget) into `src/rules/unitdiskmapping/traits.rs`:

```rust
//! Shared traits for gadget pattern matching.

use super::grid::{CellState, MappingGrid};

/// Cell type in pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternCell {
    #[default]
    Empty,
    Occupied,
    Doubled,
    Connected,
}

/// A gadget pattern that transforms source configurations to mapped configurations.
#[allow(clippy::type_complexity)]
pub trait Pattern: Clone + std::fmt::Debug {
    /// Size of the gadget pattern (rows, cols).
    fn size(&self) -> (usize, usize);

    /// Cross location within the gadget (1-indexed like Julia).
    fn cross_location(&self) -> (usize, usize);

    /// Whether this gadget involves connected nodes (edge markers).
    fn is_connected(&self) -> bool;

    /// Whether this is a Cross-type gadget where is_connected affects pattern matching.
    fn is_cross_gadget(&self) -> bool {
        false
    }

    /// Connected node indices (for gadgets with edge markers).
    fn connected_nodes(&self) -> Vec<usize> {
        vec![]
    }

    /// Source graph: (locations as (row, col), edges, pin_indices).
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>);

    /// Mapped graph: (locations as (row, col), pin_indices).
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);

    /// MIS overhead when applying this gadget.
    fn mis_overhead(&self) -> i32;

    /// Weights for each node in source graph (for weighted mode).
    fn source_weights(&self) -> Vec<i32> {
        let (locs, _, _) = self.source_graph();
        vec![2; locs.len()]
    }

    /// Weights for each node in mapped graph (for weighted mode).
    fn mapped_weights(&self) -> Vec<i32> {
        let (locs, _) = self.mapped_graph();
        vec![2; locs.len()]
    }

    /// Generate source matrix for pattern matching.
    fn source_matrix(&self) -> Vec<Vec<PatternCell>>;

    /// Generate mapped matrix.
    fn mapped_matrix(&self) -> Vec<Vec<PatternCell>>;
}

/// Check if a pattern matches at position (row, col) in the grid.
pub fn pattern_matches<P: Pattern>(
    pattern: &P,
    grid: &MappingGrid,
    row: usize,
    col: usize,
) -> bool {
    let source = pattern.source_matrix();
    let (prows, pcols) = pattern.size();

    for pr in 0..prows {
        for pc in 0..pcols {
            let gr = row + pr;
            let gc = col + pc;
            let expected = source[pr][pc];
            let actual = grid.get(gr, gc);

            match (expected, actual) {
                (PatternCell::Empty, None) => {}
                (PatternCell::Empty, Some(_)) => return false,
                (PatternCell::Occupied, Some(CellState::Occupied { .. })) => {}
                (PatternCell::Occupied, Some(CellState::Connected { .. })) => {
                    if pattern.is_cross_gadget() && !pattern.is_connected() {
                        return false;
                    }
                }
                (PatternCell::Doubled, Some(CellState::Doubled { .. })) => {}
                (PatternCell::Connected, Some(CellState::Connected { .. })) => {}
                _ => return false,
            }
        }
    }
    true
}

/// Apply a gadget at position (row, col), replacing source pattern with mapped pattern.
pub fn apply_gadget<P: Pattern>(pattern: &P, grid: &mut MappingGrid, row: usize, col: usize) {
    let mapped = pattern.mapped_matrix();
    let mapped_locs_weights: Vec<_> = {
        let (locs, _) = pattern.mapped_graph();
        let weights = pattern.mapped_weights();
        locs.into_iter().zip(weights).collect()
    };
    let (prows, pcols) = pattern.size();

    // Clear the area first
    for pr in 0..prows {
        for pc in 0..pcols {
            grid.clear(row + pr, col + pc);
        }
    }

    // Add mapped nodes
    for ((loc_r, loc_c), weight) in mapped_locs_weights {
        let gr = row + loc_r - 1;
        let gc = col + loc_c - 1;
        grid.add_node(gr, gc, weight);
    }
}

/// Unapply a gadget (reverse transformation).
pub fn unapply_gadget<P: Pattern>(pattern: &P, grid: &mut MappingGrid, row: usize, col: usize) {
    let source_locs_weights: Vec<_> = {
        let (locs, _, _) = pattern.source_graph();
        let weights = pattern.source_weights();
        locs.into_iter().zip(weights).collect()
    };
    let (prows, pcols) = pattern.size();

    // Clear the area first
    for pr in 0..prows {
        for pc in 0..pcols {
            grid.clear(row + pr, col + pc);
        }
    }

    // Add source nodes
    for ((loc_r, loc_c), weight) in source_locs_weights {
        let gr = row + loc_r - 1;
        let gc = col + loc_c - 1;
        grid.add_node(gr, gc, weight);
    }
}
```

**Step 2: Update mod.rs to export traits**

Add to `src/rules/unitdiskmapping/mod.rs`:
```rust
mod traits;
pub use traits::{Pattern, PatternCell, pattern_matches, apply_gadget, unapply_gadget};
```

**Step 3: Verify it compiles**

Run: `cargo check --all-features`

**Step 4: Commit**

```bash
git add src/rules/unitdiskmapping/traits.rs src/rules/unitdiskmapping/mod.rs
git commit -m "feat: extract Pattern trait to shared traits module"
```

---

## Task 3: Create KSG Unweighted Gadgets

**Files:**
- Create: `src/rules/unitdiskmapping/ksg/gadgets.rs`

**Step 1: Create ksg/gadgets.rs with renamed gadgets**

Copy gadget structs from `gadgets_unweighted.rs` and rename with `Ksg` prefix:
- `Cross` → `KsgCross`
- `Turn` → `KsgTurn`
- `WTurn` → `KsgWTurn`
- `Branch` → `KsgBranch`
- `BranchFix` → `KsgBranchFix`
- `TCon` → `KsgTCon`
- `TrivialTurn` → `KsgTrivialTurn`
- `EndTurn` → `KsgEndTurn`
- `BranchFixB` → `KsgBranchFixB`
- `DanglingLeg` → `KsgDanglingLeg`

Also include:
- `RotatedGadget` → `KsgRotatedGadget`
- `ReflectedGadget` → `KsgReflectedGadget`
- `Mirror` enum
- `KsgPattern` enum (was `SquarePattern`)
- `KsgTapeEntry` (was `TapeEntry`)

The file should be approximately 1200 lines. Use search-replace to rename all occurrences.

**Step 2: Add application functions**

Include `apply_crossing_gadgets`, `apply_simplifier_gadgets` functions renamed appropriately.

**Step 3: Verify it compiles**

Run: `cargo check --all-features`

**Step 4: Commit**

```bash
git add src/rules/unitdiskmapping/ksg/gadgets.rs
git commit -m "feat: create KSG unweighted gadgets with Ksg prefix"
```

---

## Task 4: Create KSG Weighted Gadgets

**Files:**
- Create: `src/rules/unitdiskmapping/ksg/gadgets_weighted.rs`

**Step 1: Create weighted KSG gadgets**

Create independent weighted versions (not wrappers):
- `WeightedKsgCross<const CON: bool>`
- `WeightedKsgTurn`
- `WeightedKsgWTurn`
- `WeightedKsgBranch`
- `WeightedKsgBranchFix`
- `WeightedKsgTCon`
- `WeightedKsgTrivialTurn`
- `WeightedKsgEndTurn`
- `WeightedKsgBranchFixB`
- `WeightedKsgDanglingLeg`

Each struct implements `Pattern` with:
- Same geometry as unweighted version
- `source_weights()` returns actual weight vectors
- `mapped_weights()` returns actual weight vectors
- `mis_overhead()` returns 2x the unweighted value

**Step 2: Add weighted application functions**

- `apply_weighted_ksg_crossing_gadgets`
- `apply_weighted_ksg_simplifier_gadgets`

**Step 3: Verify it compiles**

Run: `cargo check --all-features`

**Step 4: Commit**

```bash
git add src/rules/unitdiskmapping/ksg/gadgets_weighted.rs
git commit -m "feat: create KSG weighted gadgets with WeightedKsg prefix"
```

---

## Task 5: Create KSG Mapping Functions

**Files:**
- Create: `src/rules/unitdiskmapping/ksg/mapping.rs`

**Step 1: Create mapping.rs with renamed functions**

Move from `map_graph.rs`:
- `map_graph` → `map_unweighted`
- `map_graph_with_method` → `map_unweighted_with_method`
- `map_graph_with_order` → `map_unweighted_with_order`

Add new weighted functions:
- `map_weighted`
- `map_weighted_with_method`
- `map_weighted_with_order`

Also move:
- `MappingResult` struct
- `embed_graph` function
- `map_config_copyback` function
- `trace_centers_square` → `trace_centers`

**Step 2: Update imports to use new gadget names**

Replace `Cross` with `KsgCross`, etc.

**Step 3: Verify it compiles**

Run: `cargo check --all-features`

**Step 4: Commit**

```bash
git add src/rules/unitdiskmapping/ksg/mapping.rs
git commit -m "feat: create KSG mapping functions"
```

---

## Task 6: Update KSG mod.rs Exports

**Files:**
- Modify: `src/rules/unitdiskmapping/ksg/mod.rs`

**Step 1: Update exports**

```rust
//! King's Subgraph (KSG) mapping module.

mod gadgets;
mod gadgets_weighted;
mod mapping;

// Re-export all public items
pub use gadgets::{
    KsgCross, KsgTurn, KsgWTurn, KsgBranch, KsgBranchFix, KsgTCon,
    KsgTrivialTurn, KsgEndTurn, KsgBranchFixB, KsgDanglingLeg,
    KsgRotatedGadget, KsgReflectedGadget, Mirror, KsgPattern, KsgTapeEntry,
    apply_crossing_gadgets, apply_simplifier_gadgets,
    crossing_ruleset_indices, tape_entry_mis_overhead,
};

pub use gadgets_weighted::{
    WeightedKsgCross, WeightedKsgTurn, WeightedKsgWTurn, WeightedKsgBranch,
    WeightedKsgBranchFix, WeightedKsgTCon, WeightedKsgTrivialTurn,
    WeightedKsgEndTurn, WeightedKsgBranchFixB, WeightedKsgDanglingLeg,
    apply_weighted_crossing_gadgets, apply_weighted_simplifier_gadgets,
};

pub use mapping::{
    map_unweighted, map_unweighted_with_method, map_unweighted_with_order,
    map_weighted, map_weighted_with_method, map_weighted_with_order,
    embed_graph, map_config_copyback, trace_centers, MappingResult,
};

pub const SPACING: usize = 4;
pub const PADDING: usize = 2;
```

**Step 2: Verify it compiles**

Run: `cargo check --all-features`

**Step 3: Commit**

```bash
git add src/rules/unitdiskmapping/ksg/mod.rs
git commit -m "feat: update KSG module exports"
```

---

## Task 7: Create Triangular Weighted Gadgets

**Files:**
- Create: `src/rules/unitdiskmapping/triangular/gadgets.rs`

**Step 1: Create gadgets.rs with renamed gadgets**

Copy from `triangular.rs` and rename with `WeightedTri` prefix:
- `TriCross` → `WeightedTriCross`
- `TriTurn` → `WeightedTriTurn`
- `TriBranch` → `WeightedTriBranch`
- `TriTConLeft` → `WeightedTriTConLeft`
- `TriTConDown` → `WeightedTriTConDown`
- `TriTConUp` → `WeightedTriTConUp`
- `TriTrivialTurnLeft` → `WeightedTriTrivialTurnLeft`
- `TriTrivialTurnRight` → `WeightedTriTrivialTurnRight`
- `TriEndTurn` → `WeightedTriEndTurn`
- `TriWTurn` → `WeightedTriWTurn`
- `TriBranchFix` → `WeightedTriBranchFix`
- `TriBranchFixB` → `WeightedTriBranchFixB`

Also rename:
- `TriangularGadget` → `WeightedTriangularGadget` trait
- `TriangularTapeEntry` → `WeightedTriTapeEntry`
- `SourceCell` enum (keep name)

**Step 2: Include application functions**

- `apply_triangular_crossing_gadgets` → `apply_crossing_gadgets`
- `apply_triangular_simplifier_gadgets` → `apply_simplifier_gadgets`
- `triangular_tape_entry_mis_overhead` → `tape_entry_mis_overhead`

**Step 3: Verify it compiles**

Run: `cargo check --all-features`

**Step 4: Commit**

```bash
git add src/rules/unitdiskmapping/triangular/gadgets.rs
git commit -m "feat: create triangular weighted gadgets with WeightedTri prefix"
```

---

## Task 8: Create Triangular Mapping Functions

**Files:**
- Create: `src/rules/unitdiskmapping/triangular/mapping.rs`

**Step 1: Create mapping.rs**

Move from `triangular.rs`:
- `map_graph_triangular` → `map_weighted`
- `map_graph_triangular_with_method` → `map_weighted_with_method`
- `map_graph_triangular_with_order` → `map_weighted_with_order`

Also move from `weighted.rs`:
- `triangular_weighted_ruleset` → `weighted_ruleset`
- `trace_centers` (triangular version)
- `map_weights`

**Step 2: Update imports**

Replace `TriCross` with `WeightedTriCross`, etc.

**Step 3: Verify it compiles**

Run: `cargo check --all-features`

**Step 4: Commit**

```bash
git add src/rules/unitdiskmapping/triangular/mapping.rs
git commit -m "feat: create triangular mapping functions"
```

---

## Task 9: Update Triangular mod.rs Exports

**Files:**
- Modify: `src/rules/unitdiskmapping/triangular/mod.rs`

**Step 1: Update exports**

```rust
//! Triangular lattice mapping module.

mod gadgets;
mod mapping;

pub use gadgets::{
    WeightedTriCross, WeightedTriTurn, WeightedTriBranch,
    WeightedTriTConLeft, WeightedTriTConDown, WeightedTriTConUp,
    WeightedTriTrivialTurnLeft, WeightedTriTrivialTurnRight,
    WeightedTriEndTurn, WeightedTriWTurn,
    WeightedTriBranchFix, WeightedTriBranchFixB,
    WeightedTriangularGadget, WeightedTriTapeEntry, SourceCell,
    apply_crossing_gadgets, apply_simplifier_gadgets, tape_entry_mis_overhead,
};

pub use mapping::{
    map_weighted, map_weighted_with_method, map_weighted_with_order,
    weighted_ruleset, trace_centers, map_weights,
};

pub const SPACING: usize = 6;
pub const PADDING: usize = 2;
```

**Step 2: Verify it compiles**

Run: `cargo check --all-features`

**Step 3: Commit**

```bash
git add src/rules/unitdiskmapping/triangular/mod.rs
git commit -m "feat: update triangular module exports"
```

---

## Task 10: Update Main mod.rs

**Files:**
- Modify: `src/rules/unitdiskmapping/mod.rs`

**Step 1: Update to export new modules**

```rust
//! Graph to grid graph mapping.
//!
//! This module implements reductions from arbitrary graphs to unit disk grid graphs
//! using the copy-line technique from UnitDiskMapping.jl.
//!
//! # Modules
//!
//! - `ksg`: King's Subgraph (8-connected square grid) mapping
//! - `triangular`: Triangular lattice mapping
//!
//! # Example
//!
//! ```rust
//! use problemreductions::rules::unitdiskmapping::{ksg, triangular};
//!
//! let edges = vec![(0, 1), (1, 2), (0, 2)];
//!
//! // Map to King's Subgraph (unweighted)
//! let result = ksg::map_unweighted(3, &edges);
//!
//! // Map to triangular lattice (weighted)
//! let tri_result = triangular::map_weighted(3, &edges);
//! ```

pub mod alpha_tensor;
mod copyline;
mod grid;
pub mod ksg;
pub mod pathdecomposition;
mod traits;
pub mod triangular;

// Re-export shared types
pub use copyline::{create_copylines, mis_overhead_copyline, remove_order, CopyLine};
pub use grid::{CellState, MappingGrid};
pub use pathdecomposition::{pathwidth, Layout, PathDecompositionMethod};
pub use traits::{apply_gadget, pattern_matches, unapply_gadget, Pattern, PatternCell};

// Re-export commonly used items from submodules for convenience
pub use ksg::MappingResult;
```

**Step 2: Verify it compiles**

Run: `cargo check --all-features`

**Step 3: Commit**

```bash
git add src/rules/unitdiskmapping/mod.rs
git commit -m "feat: update main mod.rs to export ksg and triangular modules"
```

---

## Task 11: Update Test Files

**Files:**
- Modify: `tests/rules/unitdiskmapping/mod.rs`
- Modify: `tests/rules/unitdiskmapping/gadgets.rs`
- Modify: `tests/rules/unitdiskmapping/map_graph.rs`
- Modify: `tests/rules/unitdiskmapping/triangular.rs`
- Modify: `tests/rules/unitdiskmapping/weighted.rs`
- Modify: `tests/rules/unitdiskmapping/julia_comparison.rs`
- Modify: `tests/rules/unitdiskmapping/gadgets_ground_truth.rs`

**Step 1: Update imports in all test files**

Replace old imports with new:
```rust
// Old
use problemreductions::rules::unitdiskmapping::{map_graph, Cross, Turn, ...};

// New
use problemreductions::rules::unitdiskmapping::{ksg, triangular};
use problemreductions::rules::unitdiskmapping::ksg::{KsgCross, KsgTurn, ...};
```

**Step 2: Update function calls**

```rust
// Old
let result = map_graph(n, &edges);
let tri_result = map_graph_triangular(n, &edges);

// New
let result = ksg::map_unweighted(n, &edges);
let tri_result = triangular::map_weighted(n, &edges);
```

**Step 3: Update gadget names in tests**

Replace all `Cross` with `KsgCross`, `TriCross` with `WeightedTriCross`, etc.

**Step 4: Run tests**

Run: `cargo test --all-features`

**Step 5: Commit**

```bash
git add tests/
git commit -m "test: update imports for ksg and triangular modules"
```

---

## Task 12: Delete Old Files

**Files:**
- Delete: `src/rules/unitdiskmapping/gadgets.rs`
- Delete: `src/rules/unitdiskmapping/gadgets_unweighted.rs`
- Delete: `src/rules/unitdiskmapping/map_graph.rs`
- Delete: `src/rules/unitdiskmapping/triangular.rs`
- Delete: `src/rules/unitdiskmapping/weighted.rs`

**Step 1: Remove old files**

```bash
git rm src/rules/unitdiskmapping/gadgets.rs
git rm src/rules/unitdiskmapping/gadgets_unweighted.rs
git rm src/rules/unitdiskmapping/map_graph.rs
git rm src/rules/unitdiskmapping/triangular.rs
git rm src/rules/unitdiskmapping/weighted.rs
```

**Step 2: Verify build**

Run: `cargo build --all-features`

**Step 3: Run all tests**

Run: `cargo test --all-features`

**Step 4: Commit**

```bash
git commit -m "chore: delete old gadget and mapping files"
```

---

## Task 13: Update Documentation

**Files:**
- Modify: `src/rules/unitdiskmapping/ksg/mod.rs` (add module docs)
- Modify: `src/rules/unitdiskmapping/triangular/mod.rs` (add module docs)

**Step 1: Add comprehensive module documentation**

Add examples showing the new API usage.

**Step 2: Update any doc references**

Search for references to old function names in docs/ and update.

**Step 3: Commit**

```bash
git add src/ docs/
git commit -m "docs: update documentation for ksg and triangular modules"
```

---

## Task 14: Post Julia Comparison to Issue #8

**Step 1: Add comment to issue #8**

Run:
```bash
gh issue comment 8 --body "$(cat <<'EOF'
## Julia vs Rust Naming Comparison

The Rust implementation uses different naming from Julia's UnitDiskMapping.jl to better reflect the underlying graph structures.

| Concept | Julia (UnitDiskMapping.jl) | Rust |
|---------|---------------------------|------|
| **Mapping Modes** | | |
| Square unweighted | `UnWeighted()` | `ksg::map_unweighted()` |
| Square weighted | `Weighted()` | `ksg::map_weighted()` |
| Triangular weighted | `TriangularWeighted()` | `triangular::map_weighted()` |
| **Lattice Types** | | |
| Square lattice | `SquareGrid` | King's Subgraph (KSG) |
| Triangular lattice | `TriangularGrid` | Triangular |
| **Square Gadgets** | | |
| Crossing | `Cross{CON}` | `KsgCross<CON>` / `WeightedKsgCross<CON>` |
| Turn | `Turn` | `KsgTurn` / `WeightedKsgTurn` |
| Branch | `Branch` | `KsgBranch` / `WeightedKsgBranch` |
| Weighted wrapper | `WeightedGadget{T}` | *(separate types)* |
| **Triangular Gadgets** | | |
| Crossing | `TriCross{CON}` | `WeightedTriCross<CON>` |
| Turn | `TriTurn` | `WeightedTriTurn` |
| Branch | `TriBranch` | `WeightedTriBranch` |

**Key difference:** Julia uses a `WeightedGadget{T}` wrapper pattern. Rust uses independent weighted types for cleaner separation.

See `docs/plans/2026-01-31-ksg-triangular-refactor-design.md` for full comparison.
EOF
)"
```

**Step 2: Verify comment posted**

Run: `gh issue view 8 --comments`

**Step 3: Commit design docs if not already**

```bash
git add docs/plans/
git commit -m "docs: add Julia naming comparison reference" --allow-empty
```

---

## Task 15: Final Verification

**Step 1: Run full test suite**

Run: `cargo test --all-features -- --include-ignored`

**Step 2: Run clippy**

Run: `cargo clippy --all-features -- -D warnings`

**Step 3: Check documentation builds**

Run: `cargo doc --all-features --no-deps`

**Step 4: Final commit if any fixes needed**

```bash
git add -A
git commit -m "fix: address any remaining issues from refactoring"
```

---

## Summary

After completing all tasks:

| Module | Contents |
|--------|----------|
| `ksg/` | `KsgCross`, `KsgTurn`, ..., `WeightedKsgCross`, ..., `map_unweighted()`, `map_weighted()` |
| `triangular/` | `WeightedTriCross`, `WeightedTriTurn`, ..., `map_weighted()` |
| `traits.rs` | `Pattern` trait, `PatternCell`, shared functions |
| `copyline.rs` | Shared copy line creation |
| `grid.rs` | Shared grid representation |

**Files deleted:** `gadgets.rs`, `gadgets_unweighted.rs`, `map_graph.rs`, `triangular.rs`, `weighted.rs`

**New API:**
```rust
use problemreductions::rules::unitdiskmapping::{ksg, triangular};

// King's Subgraph mapping
let result = ksg::map_unweighted(n, &edges);
let weighted_result = ksg::map_weighted(n, &edges);

// Triangular mapping
let tri_result = triangular::map_weighted(n, &edges);
```
