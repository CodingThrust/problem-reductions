# map_config_back for Square Lattice Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement `map_config_back` for square lattice mapping following Julia's UnitDiskMapping exactly.

**Architecture:** Julia's approach has three steps: (1) iterate tape in REVERSE order, (2) for each gadget call `map_config_back!` to convert mapped config to source config in the 2D config matrix, (3) call `map_config_copyback!` to extract original vertex configs from copyline locations. The config is maintained as a 2D matrix during unapply, then final vertex values are extracted.

**Tech Stack:** Rust, existing Pattern trait with `mapped_entry_to_compact` and `source_entry_to_configs`

---

## Background

Julia's `map_config_back` workflow:

```julia
function map_config_back(res::MappingResult, cfg)
    # cfg is 2D matrix indexed by grid coordinates
    cm = cell_matrix(r.grid_graph)
    ug = MappingGrid(r.lines, r.padding, MCell.(cm), r.spacing)
    unapply_gadgets!(ug, r.mapping_history, copy.(configs))[2]
end

function unapply_gadgets!(ug, tape, configurations)
    for (pattern, i, j) in Base.Iterators.reverse(tape)
        for c in configurations
            map_config_back!(pattern, i, j, c)  # modifies c in-place
        end
    end
    cfgs = map(configurations) do c
        map_config_copyback!(ug, c)
    end
    return ug, cfgs
end
```

Key insight: The 2D config matrix is modified gadget-by-gadget, converting mapped patterns back to source patterns. After all gadgets are unapplied, `map_config_copyback!` counts selected nodes along each copyline.

---

### Task 1: Add `mapped_boundary_config` Function

**Files:**
- Modify: `src/rules/mapping/gadgets.rs`
- Test: `src/rules/mapping/gadgets.rs` (existing test module)

**Step 1: Add the function to gadgets.rs**

Add after the Pattern trait definition (around line 120):

```rust
/// Compute binary boundary config from pin values in the mapped graph.
/// Julia: `mapped_boundary_config(p, config)` -> `_boundary_config(pins, config)`
///
/// This computes: sum(config[pin] << (i-1) for i, pin in pins)
pub fn mapped_boundary_config<P: Pattern>(pattern: &P, config: &[usize]) -> usize {
    let (_, pins) = pattern.mapped_graph();
    let mut result = 0usize;
    for (i, &pin_idx) in pins.iter().enumerate() {
        if pin_idx < config.len() && config[pin_idx] > 0 {
            result |= 1 << i;
        }
    }
    result
}
```

**Step 2: Add test**

```rust
#[test]
fn test_mapped_boundary_config_danglinleg() {
    // DanglingLeg has 1 mapped node at (4,2), pins = [0]
    // config[0] = 0 -> boundary = 0
    // config[0] = 1 -> boundary = 1
    assert_eq!(mapped_boundary_config(&DanglingLeg, &[0]), 0);
    assert_eq!(mapped_boundary_config(&DanglingLeg, &[1]), 1);
}

#[test]
fn test_mapped_boundary_config_cross_false() {
    // Cross<false> has multiple pins, test a few cases
    let cross = Cross::<false>;
    // All zeros -> 0
    let config = vec![0; 16];
    assert_eq!(mapped_boundary_config(&cross, &config), 0);
}
```

**Step 3: Run tests**

```bash
cargo test mapped_boundary_config -- --nocapture
```

**Step 4: Commit**

```bash
git add src/rules/mapping/gadgets.rs
git commit -m "feat: add mapped_boundary_config function for config extraction"
```

---

### Task 2: Add `map_config_back_pattern` Function

**Files:**
- Modify: `src/rules/mapping/gadgets.rs`

**Step 1: Add the function**

This implements Julia's `map_config_back!` - converts mapped config to source config at gadget position.

```rust
/// Map configuration back through a single gadget.
/// Julia: `map_config_back!(p, i, j, configuration)`
///
/// This function:
/// 1. Extracts config values at mapped_graph locations
/// 2. Computes boundary config
/// 3. Looks up source configs via mapped_entry_to_compact and source_entry_to_configs
/// 4. Clears the gadget area in the config matrix
/// 5. Writes source config to source_graph locations
///
/// # Arguments
/// * `pattern` - The gadget pattern
/// * `gi, gj` - Position where gadget was applied (0-indexed)
/// * `config` - 2D config matrix (modified in place)
pub fn map_config_back_pattern<P: Pattern>(
    pattern: &P,
    gi: usize,
    gj: usize,
    config: &mut Vec<Vec<usize>>,
) {
    let (m, n) = pattern.size();
    let (mapped_locs, mapped_pins) = pattern.mapped_graph();
    let (source_locs, _, _) = pattern.source_graph();

    // Step 1: Extract config at mapped locations
    let mapped_config: Vec<usize> = mapped_locs
        .iter()
        .map(|&(r, c)| {
            let row = gi + r - 1; // Convert 1-indexed to 0-indexed
            let col = gj + c - 1;
            config.get(row).and_then(|r| r.get(col)).copied().unwrap_or(0)
        })
        .collect();

    // Step 2: Compute boundary config
    let bc = {
        let mut result = 0usize;
        for (i, &pin_idx) in mapped_pins.iter().enumerate() {
            if pin_idx < mapped_config.len() && mapped_config[pin_idx] > 0 {
                result |= 1 << i;
            }
        }
        result
    };

    // Step 3: Look up source config
    let d1 = pattern.mapped_entry_to_compact();
    let d2 = pattern.source_entry_to_configs();

    let compact = d1.get(&bc).copied().unwrap_or(0);
    let source_configs = d2.get(&compact).cloned().unwrap_or_default();

    // Pick first valid config (Julia uses rand, we use first)
    let new_config = if source_configs.is_empty() {
        vec![false; source_locs.len()]
    } else {
        source_configs[0].clone()
    };

    // Step 4: Clear gadget area
    for row in gi..gi + m {
        for col in gj..gj + n {
            if let Some(r) = config.get_mut(row) {
                if let Some(c) = r.get_mut(col) {
                    *c = 0;
                }
            }
        }
    }

    // Step 5: Write source config
    for (k, &(r, c)) in source_locs.iter().enumerate() {
        let row = gi + r - 1;
        let col = gj + c - 1;
        if let Some(rv) = config.get_mut(row) {
            if let Some(cv) = rv.get_mut(col) {
                *cv += if new_config.get(k).copied().unwrap_or(false) { 1 } else { 0 };
            }
        }
    }
}
```

**Step 2: Add test**

```rust
#[test]
fn test_map_config_back_pattern_danglinleg() {
    // DanglingLeg: source (2,2),(3,2),(4,2) -> mapped (4,2)
    // If mapped node is selected (1), source should be [1,0,1]
    // If mapped node is not selected (0), source should be [1,0,0] or [0,1,0]

    let mut config = vec![vec![0; 5]; 6];
    // Place mapped node at (4,2) as selected (gadget at position (1,1))
    config[4][2] = 1;

    map_config_back_pattern(&DanglingLeg, 1, 1, &mut config);

    // After unapply, source nodes at (2,2), (3,2), (4,2) relative to (1,1)
    // which is global (2,2), (3,2), (4,2)
    // Should be [1,0,1]
    assert_eq!(config[2][2], 1);
    assert_eq!(config[3][2], 0);
    assert_eq!(config[4][2], 1);
}

#[test]
fn test_map_config_back_pattern_danglinleg_unselected() {
    let mut config = vec![vec![0; 5]; 6];
    // Mapped node not selected
    config[4][2] = 0;

    map_config_back_pattern(&DanglingLeg, 1, 1, &mut config);

    // Source should be [1,0,0] or [0,1,0]
    let sum = config[2][2] + config[3][2] + config[4][2];
    assert_eq!(sum, 1); // Exactly one node selected
}
```

**Step 3: Run tests**

```bash
cargo test map_config_back_pattern -- --nocapture
```

**Step 4: Commit**

```bash
git add src/rules/mapping/gadgets.rs
git commit -m "feat: add map_config_back_pattern for single gadget config unapply"
```

---

### Task 3: Add `map_config_copyback` Function

**Files:**
- Modify: `src/rules/mapping/map_graph.rs`

**Step 1: Add the function**

This implements Julia's `map_config_copyback!` - extracts vertex configs from copyline locations.

```rust
/// Extract original vertex configurations from copyline locations.
/// Julia: `map_config_copyback!(ug, c)`
///
/// For each copyline, count selected nodes and subtract overhead:
/// `res[vertex] = count - (len(locs) / 2)`
///
/// This works after gadgets have been unapplied, so copyline locations
/// are intact in the config matrix.
pub fn map_config_copyback(
    lines: &[CopyLine],
    padding: usize,
    spacing: usize,
    config: &[Vec<usize>],
) -> Vec<usize> {
    let mut result = vec![0usize; lines.len()];

    for line in lines {
        let locs = line.dense_locations(padding, spacing);
        let mut count = 0usize;

        for &(row, col, _weight) in &locs {
            if let Some(val) = config.get(row).and_then(|r| r.get(col)) {
                count += val;
            }
        }

        // Subtract overhead: MIS overhead for copyline is len/2
        let overhead = locs.len() / 2;
        result[line.vertex] = count.saturating_sub(overhead);
    }

    result
}
```

**Step 2: Add test**

```rust
#[test]
fn test_map_config_copyback_simple() {
    use super::super::copyline::CopyLine;

    // Create a simple copyline
    let line = CopyLine {
        vertex: 0,
        vslot: 1,
        hslot: 1,
        vstart: 1,
        vstop: 1,
        hstop: 3,
    };
    let lines = vec![line];

    // Create config with some nodes selected
    let locs = lines[0].dense_locations(2, 4);
    let (rows, cols) = (20, 20);
    let mut config = vec![vec![0; cols]; rows];

    // Select all nodes in copyline
    for &(row, col, _) in &locs {
        if row < rows && col < cols {
            config[row][col] = 1;
        }
    }

    let result = map_config_copyback(&lines, 2, 4, &config);

    // count = len(locs), overhead = len/2
    // result = count - overhead = len - len/2 = (len+1)/2 for odd len, len/2+1 for even
    let expected = locs.len() - locs.len() / 2;
    assert_eq!(result[0], expected);
}
```

**Step 3: Run tests**

```bash
cargo test map_config_copyback -- --nocapture
```

**Step 4: Commit**

```bash
git add src/rules/mapping/map_graph.rs
git commit -m "feat: add map_config_copyback for extracting vertex configs from copylines"
```

---

### Task 4: Add Pattern Enum for Dynamic Dispatch

**Files:**
- Modify: `src/rules/mapping/gadgets.rs`

**Step 1: Add enum and implementation**

We need to dispatch `map_config_back_pattern` to the correct pattern type based on `pattern_idx`.

```rust
/// Enum wrapping all square lattice patterns for dynamic dispatch during unapply.
#[derive(Debug, Clone)]
pub enum SquarePattern {
    CrossFalse(Cross<false>),
    CrossTrue(Cross<true>),
    Turn(Turn),
    WTurn(WTurn),
    Branch(Branch),
    BranchFix(BranchFix),
    TCon(TCon),
    TrivialTurn(TrivialTurn),
    EndTurn(EndTurn),
    BranchFixB(BranchFixB),
    DanglingLeg(DanglingLeg),
    // Rotated and reflected variants
    RotatedTCon1(RotatedGadget<TCon>),
    ReflectedCrossTrue(ReflectedGadget<Cross<true>>),
    ReflectedTrivialTurn(ReflectedGadget<TrivialTurn>),
    ReflectedRotatedTCon1(ReflectedGadget<RotatedGadget<TCon>>),
    // DanglingLeg rotations/reflections (6 variants, indices 100-105)
    DanglingLegRot1(RotatedGadget<DanglingLeg>),
    DanglingLegRot2(RotatedGadget<DanglingLeg>),
    DanglingLegRot3(RotatedGadget<DanglingLeg>),
    DanglingLegReflX(ReflectedGadget<DanglingLeg>),
    DanglingLegReflY(ReflectedGadget<DanglingLeg>),
}

impl SquarePattern {
    /// Get pattern from tape index.
    /// Crossing gadgets: 0-12
    /// Simplifier gadgets: 100-105 (DanglingLeg variants)
    pub fn from_tape_idx(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::CrossFalse(Cross::<false>)),
            1 => Some(Self::Turn(Turn)),
            2 => Some(Self::WTurn(WTurn)),
            3 => Some(Self::Branch(Branch)),
            4 => Some(Self::BranchFix(BranchFix)),
            5 => Some(Self::TCon(TCon)),
            6 => Some(Self::TrivialTurn(TrivialTurn)),
            7 => Some(Self::RotatedTCon1(RotatedGadget::new(TCon, 1))),
            8 => Some(Self::ReflectedCrossTrue(ReflectedGadget::new(Cross::<true>, Mirror::Y))),
            9 => Some(Self::ReflectedTrivialTurn(ReflectedGadget::new(TrivialTurn, Mirror::Y))),
            10 => Some(Self::BranchFixB(BranchFixB)),
            11 => Some(Self::EndTurn(EndTurn)),
            12 => Some(Self::ReflectedRotatedTCon1(ReflectedGadget::new(RotatedGadget::new(TCon, 1), Mirror::Y))),
            // Simplifier gadgets
            100 => Some(Self::DanglingLeg(DanglingLeg)),
            101 => Some(Self::DanglingLegRot1(RotatedGadget::new(DanglingLeg, 1))),
            102 => Some(Self::DanglingLegRot2(RotatedGadget::new(DanglingLeg, 2))),
            103 => Some(Self::DanglingLegRot3(RotatedGadget::new(DanglingLeg, 3))),
            104 => Some(Self::DanglingLegReflX(ReflectedGadget::new(DanglingLeg, Mirror::X))),
            105 => Some(Self::DanglingLegReflY(ReflectedGadget::new(DanglingLeg, Mirror::Y))),
            _ => None,
        }
    }

    /// Apply map_config_back_pattern for this pattern.
    pub fn map_config_back(&self, gi: usize, gj: usize, config: &mut Vec<Vec<usize>>) {
        match self {
            Self::CrossFalse(p) => map_config_back_pattern(p, gi, gj, config),
            Self::CrossTrue(p) => map_config_back_pattern(p, gi, gj, config),
            Self::Turn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::WTurn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::Branch(p) => map_config_back_pattern(p, gi, gj, config),
            Self::BranchFix(p) => map_config_back_pattern(p, gi, gj, config),
            Self::TCon(p) => map_config_back_pattern(p, gi, gj, config),
            Self::TrivialTurn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::EndTurn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::BranchFixB(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLeg(p) => map_config_back_pattern(p, gi, gj, config),
            Self::RotatedTCon1(p) => map_config_back_pattern(p, gi, gj, config),
            Self::ReflectedCrossTrue(p) => map_config_back_pattern(p, gi, gj, config),
            Self::ReflectedTrivialTurn(p) => map_config_back_pattern(p, gi, gj, config),
            Self::ReflectedRotatedTCon1(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegRot1(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegRot2(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegRot3(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegReflX(p) => map_config_back_pattern(p, gi, gj, config),
            Self::DanglingLegReflY(p) => map_config_back_pattern(p, gi, gj, config),
        }
    }
}
```

**Step 2: Add test**

```rust
#[test]
fn test_square_pattern_from_tape_idx() {
    assert!(SquarePattern::from_tape_idx(0).is_some()); // CrossFalse
    assert!(SquarePattern::from_tape_idx(11).is_some()); // EndTurn
    assert!(SquarePattern::from_tape_idx(100).is_some()); // DanglingLeg
    assert!(SquarePattern::from_tape_idx(105).is_some()); // DanglingLeg ReflY
    assert!(SquarePattern::from_tape_idx(200).is_none()); // Invalid
}
```

**Step 3: Run tests**

```bash
cargo test square_pattern -- --nocapture
```

**Step 4: Commit**

```bash
git add src/rules/mapping/gadgets.rs
git commit -m "feat: add SquarePattern enum for dynamic dispatch during config unapply"
```

---

### Task 5: Implement `unapply_gadgets` and Update `map_config_back`

**Files:**
- Modify: `src/rules/mapping/map_graph.rs`

**Step 1: Add unapply_gadgets function**

```rust
/// Unapply gadgets from tape in reverse order, converting mapped configs to source configs.
/// Julia: `unapply_gadgets!(ug, tape, configurations)`
///
/// # Arguments
/// * `tape` - Vector of TapeEntry recording applied gadgets
/// * `config` - 2D config matrix (modified in place)
pub fn unapply_gadgets(tape: &[TapeEntry], config: &mut Vec<Vec<usize>>) {
    use super::gadgets::SquarePattern;

    // Iterate tape in REVERSE order
    for entry in tape.iter().rev() {
        if let Some(pattern) = SquarePattern::from_tape_idx(entry.pattern_idx) {
            pattern.map_config_back(entry.row, entry.col, config);
        }
    }
}
```

**Step 2: Update map_config_back in MappingResult**

Replace the existing `map_config_back` method:

```rust
impl MappingResult {
    /// Map a configuration back from grid to original graph.
    ///
    /// This follows Julia's exact algorithm:
    /// 1. Convert flat grid config to 2D matrix
    /// 2. Unapply gadgets in reverse order (modifying config matrix)
    /// 3. Extract vertex configs from copyline locations
    ///
    /// # Arguments
    /// * `grid_config` - Configuration on the grid graph (0 = not selected, 1 = selected)
    ///
    /// # Returns
    /// A vector where `result[v]` is 1 if vertex `v` is selected, 0 otherwise.
    pub fn map_config_back(&self, grid_config: &[usize]) -> Vec<usize> {
        // Step 1: Convert flat config to 2D matrix
        let (rows, cols) = self.grid_graph.size();
        let mut config_2d = vec![vec![0usize; cols]; rows];

        for (idx, node) in self.grid_graph.nodes().iter().enumerate() {
            let row = node.row as usize;
            let col = node.col as usize;
            if row < rows && col < cols {
                config_2d[row][col] = grid_config.get(idx).copied().unwrap_or(0);
            }
        }

        // Step 2: Unapply gadgets in reverse order
        unapply_gadgets(&self.tape, &mut config_2d);

        // Step 3: Extract vertex configs from copylines
        map_config_copyback(&self.lines, self.padding, self.spacing, &config_2d)
    }
}
```

**Step 3: Add imports at top of map_graph.rs**

```rust
use super::gadgets::TapeEntry;
```

**Step 4: Run tests**

```bash
cargo test test_map_config_back -- --nocapture
```

**Step 5: Commit**

```bash
git add src/rules/mapping/map_graph.rs
git commit -m "feat: implement proper map_config_back following Julia's unapply algorithm"
```

---

### Task 6: Add Julia Ground Truth Test

**Files:**
- Modify: `tests/rules/mapping/map_graph.rs`

**Step 1: Update the failing test**

The test `test_map_config_back_all_standard_graphs` should now pass. Run it:

```bash
cargo test test_map_config_back_all_standard_graphs -- --nocapture
```

**Step 2: If test passes, commit**

```bash
git add tests/rules/mapping/map_graph.rs
git commit -m "test: verify map_config_back matches Julia for standard graphs"
```

**Step 3: If test fails, debug**

Add debug output to understand the issue:

```rust
#[test]
fn test_map_config_back_debug() {
    let (n, edges) = smallgraph("petersen").unwrap();
    let result = map_graph(n, &edges);

    // Solve MIS on grid
    let grid_edges = result.grid_graph.edges().to_vec();
    let grid_config = solve_mis_config(result.grid_graph.num_vertices(), &grid_edges);

    eprintln!("Grid MIS size: {}", grid_config.iter().sum::<usize>());
    eprintln!("Tape entries: {}", result.tape.len());

    // Map back
    let original_config = result.map_config_back(&grid_config);
    eprintln!("Original config: {:?}", original_config);
    eprintln!("Original MIS size: {}", original_config.iter().sum::<usize>());

    // Expected
    let expected_mis = solve_mis(n, &edges);
    eprintln!("Expected MIS: {}", expected_mis);
}
```

---

### Task 7: Verify All Tests Pass

**Step 1: Run full test suite**

```bash
cargo test --test rules_mapping
```

**Step 2: Run with Julia comparison**

```bash
cargo test test_standard_graphs_match_julia -- --nocapture
cargo test test_mis_overhead_correctness -- --nocapture
cargo test test_map_config_back_all_standard_graphs -- --nocapture
```

**Step 3: Commit final changes**

```bash
git add -A
git commit -m "feat: complete map_config_back implementation matching Julia exactly"
```

---

## Verification Checklist

After implementation, verify:

- [ ] `map_config_back` returns valid independent set for all standard graphs
- [ ] `map_config_back` returns correct MIS size (matches `solve_mis`)
- [ ] All existing tests still pass
- [ ] No panics on edge cases (empty graph, single vertex)

## Notes

1. **Index convention**: Julia uses 1-indexed positions, Rust uses 0-indexed. Be careful with conversions.
2. **Pattern indices**: Crossing gadgets are 0-12, simplifier gadgets are 100-105.
3. **Random selection**: Julia uses `rand()` to pick from multiple valid source configs. Rust uses first valid config.
4. **Export functions**: Remember to export new public functions in `mod.rs`.
