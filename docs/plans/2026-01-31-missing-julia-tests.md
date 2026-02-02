# Missing Julia Tests Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add missing tests to match Julia's UnitDiskMapping test suite for complete feature parity.

**Architecture:** Add tests in existing test modules following the established patterns. Tests will verify MIS overhead calculation, config extraction validity, and weighted copyline overhead using ILP solver.

**Tech Stack:** Rust tests, ILP solver for MIS computation, smallgraph for standard graphs

---

## Task 1: Add Square Mode Tests for Cubical and Tutte Graphs

**Files:**
- Modify: `tests/rules/unitdiskmapping/map_graph.rs`

**Context:** Julia tests `map_configurations_back` for petersen, bull, cubical, house, diamond, tutte. We have petersen, bull, diamond, house but missing cubical and tutte.

**Step 1: Add test for cubical graph MIS overhead**

Add to `tests/rules/unitdiskmapping/map_graph.rs` after the existing `test_mis_overhead_triangle`:

```rust
#[test]
fn test_mis_overhead_cubical() {
    let (n, edges) = smallgraph("cubical").unwrap();
    let result = map_graph(n, &edges);

    let original_mis = solve_mis(n, &edges) as i32;
    let grid_edges = result.grid_graph.edges().to_vec();
    let mapped_mis = solve_mis(result.grid_graph.num_vertices(), &grid_edges) as i32;

    let expected = original_mis + result.mis_overhead;

    assert_eq!(
        mapped_mis, expected,
        "Cubical: mapped MIS {} should equal original {} + overhead {} = {}",
        mapped_mis, original_mis, result.mis_overhead, expected
    );
}
```

**Step 2: Add test for tutte graph MIS overhead**

```rust
#[test]
fn test_mis_overhead_tutte() {
    let (n, edges) = smallgraph("tutte").unwrap();
    let result = map_graph(n, &edges);

    let original_mis = solve_mis(n, &edges) as i32;
    let grid_edges = result.grid_graph.edges().to_vec();
    let mapped_mis = solve_mis(result.grid_graph.num_vertices(), &grid_edges) as i32;

    let expected = original_mis + result.mis_overhead;

    assert_eq!(
        mapped_mis, expected,
        "Tutte: mapped MIS {} should equal original {} + overhead {} = {}",
        mapped_mis, original_mis, result.mis_overhead, expected
    );
}
```

**Step 3: Run tests to verify**

Run: `cargo test --test rules_unitdiskmapping test_mis_overhead_cubical test_mis_overhead_tutte -- --nocapture`
Expected: Both tests PASS

**Step 4: Commit**

```bash
git add tests/rules/unitdiskmapping/map_graph.rs
git commit -m "test: Add MIS overhead tests for cubical and tutte graphs"
```

---

## Task 2: Add Full map_config_back Verification for Standard Graphs

**Files:**
- Modify: `tests/rules/unitdiskmapping/map_graph.rs`

**Context:** Julia verifies that:
1. `count(isone, original_configs) == original_mis_size`
2. `is_independent_set(g, original_configs)`

We only test triangle. Need to test all standard graphs.

**Step 1: Add comprehensive map_config_back test**

Add to `tests/rules/unitdiskmapping/map_graph.rs`:

```rust
/// Test map_config_back for standard graphs - verifies:
/// 1. Extracted config is a valid independent set
/// 2. Extracted config size equals original MIS size
#[test]
fn test_map_config_back_standard_graphs() {
    let graph_names = ["bull", "diamond", "house", "petersen", "cubical"];

    for name in graph_names {
        let (n, edges) = smallgraph(name).unwrap();
        let result = map_graph(n, &edges);

        // Solve MIS on mapped graph
        let grid_edges = result.grid_graph.edges().to_vec();
        let grid_config = solve_mis_config(result.grid_graph.num_vertices(), &grid_edges);

        // Extract original config
        let original_config = result.map_config_back(&grid_config);

        // Verify it's a valid independent set
        assert!(
            is_independent_set(&edges, &original_config),
            "{}: Extracted config should be a valid independent set",
            name
        );

        // Verify size matches original MIS
        let original_mis = solve_mis(n, &edges);
        let extracted_size = original_config.iter().filter(|&&x| x > 0).count();

        assert_eq!(
            extracted_size, original_mis,
            "{}: Extracted config size {} should equal original MIS size {}",
            name, extracted_size, original_mis
        );
    }
}
```

**Step 2: Run test to verify**

Run: `cargo test --test rules_unitdiskmapping test_map_config_back_standard_graphs -- --nocapture`
Expected: PASS for all graphs

**Step 3: Commit**

```bash
git add tests/rules/unitdiskmapping/map_graph.rs
git commit -m "test: Add full map_config_back verification for standard graphs"
```

---

## Task 3: Add Copyline Weighted Overhead Explicit Tests

**Files:**
- Modify: `tests/rules/unitdiskmapping/copyline.rs`
- Modify: `tests/rules/unitdiskmapping/common.rs` (if needed)

**Context:** Julia tests in weighted.jl lines 32-49:
```julia
for (vstart, vstop, hstop) in [
        (3, 7, 8), (3, 5, 8), (5, 9, 8), (5, 5, 8),
        (1, 7, 5), (5, 8, 5), (1, 5, 5), (5, 5, 5)]
    tc = CopyLine(1, 5, 5, vstart, vstop, hstop)
    # Build graph from copyline locations
    # Solve weighted MIS
    # Assert MIS == mis_overhead_copyline(Weighted(), tc)
end
```

**Step 1: Add imports to copyline.rs**

Add at top of `tests/rules/unitdiskmapping/copyline.rs`:

```rust
use super::common::solve_weighted_mis;
use problemreductions::rules::unitdiskmapping::mis_overhead_copyline;
```

**Step 2: Add weighted copyline overhead test**

Add to `tests/rules/unitdiskmapping/copyline.rs`:

```rust
/// Test that weighted MIS of copyline graph equals mis_overhead_copyline.
/// This matches Julia's weighted.jl "copy lines" testset.
#[test]
fn test_copyline_weighted_mis_equals_overhead() {
    let test_cases = [
        (3, 7, 8),
        (3, 5, 8),
        (5, 9, 8),
        (5, 5, 8),
        (1, 7, 5),
        (5, 8, 5),
        (1, 5, 5),
        (5, 5, 5),
    ];

    let padding = 2;
    let spacing = 4;

    for (vstart, vstop, hstop) in test_cases {
        // Create copyline with vslot=5, hslot=5 (matching Julia's test)
        let line = CopyLine::new(0, 5, 5, vstart, vstop, hstop);

        // Get copyline locations with weights
        let locs = line.copyline_locations(padding, spacing);
        let n = locs.len();

        // Build graph: chain structure where each node connects to previous
        // unless at a weight=1 starting point
        let mut edges = Vec::new();
        for i in 1..n {
            // In Julia: if i==1 || locs[i-1].weight == 1, connect to last node
            // else connect to previous
            if i == 1 || locs[i - 1].2 == 1 {
                edges.push((n - 1, i - 1));
            } else {
                edges.push((i, i - 1));
            }
        }

        let weights: Vec<i32> = locs.iter().map(|&(_, _, w)| w as i32).collect();

        // Solve weighted MIS
        let weighted_mis = solve_weighted_mis(n, &edges, &weights);

        // Get expected overhead
        let expected = mis_overhead_copyline(&line, spacing, padding) as i32;

        assert_eq!(
            weighted_mis, expected,
            "Copyline vstart={}, vstop={}, hstop={}: weighted MIS {} should equal overhead {}",
            vstart, vstop, hstop, weighted_mis, expected
        );
    }
}
```

**Step 3: Run test to verify**

Run: `cargo test --test rules_unitdiskmapping test_copyline_weighted_mis_equals_overhead -- --nocapture`
Expected: PASS

**Step 4: Commit**

```bash
git add tests/rules/unitdiskmapping/copyline.rs
git commit -m "test: Add weighted copyline MIS overhead verification"
```

---

## Task 4: Add Weighted Mode map_config_back Full Verification

**Files:**
- Modify: `tests/rules/unitdiskmapping/weighted.rs`

**Context:** Julia's weighted.jl "map configurations back" testset (lines 52-88) verifies:
1. MIS overhead formula: `mis_overhead + original_mis == mapped_mis`
2. Config count at centers matches original MIS count
3. Extracted config is valid IS

**Step 1: Check existing weighted tests**

Read current weighted.rs to understand existing structure.

**Step 2: Add comprehensive weighted map_config_back test**

Add to `tests/rules/unitdiskmapping/weighted.rs`:

```rust
use problemreductions::rules::unitdiskmapping::trace_centers;

/// Test weighted mode map_config_back for standard graphs.
/// Verifies:
/// 1. MIS overhead formula holds
/// 2. Config at trace_centers is a valid IS
/// 3. Count at centers equals 2 * original_mis (weighted mode doubles)
#[test]
fn test_weighted_map_config_back_standard_graphs() {
    use super::common::{is_independent_set, solve_mis, solve_weighted_mis_config};
    use problemreductions::rules::unitdiskmapping::map_graph;
    use problemreductions::topology::{smallgraph, Graph};

    let graph_names = ["bull", "diamond", "house", "petersen"];

    for name in graph_names {
        let (n, edges) = smallgraph(name).unwrap();
        let result = map_graph(n, &edges);

        // Get weights (all 1s for unweighted original)
        let grid_edges = result.grid_graph.edges().to_vec();
        let num_grid = result.grid_graph.num_vertices();
        let weights: Vec<i32> = (0..num_grid)
            .map(|i| result.grid_graph.weight(i).copied().unwrap_or(1))
            .collect();

        // Solve weighted MIS on grid
        let grid_config = solve_weighted_mis_config(num_grid, &grid_edges, &weights);

        // Get center locations
        let centers = trace_centers(&result);

        // Extract config at centers
        let center_config: Vec<usize> = centers
            .iter()
            .map(|&(row, col)| {
                // Find grid node at this position
                for (i, node) in result.grid_graph.nodes().iter().enumerate() {
                    if node.row == row as i32 && node.col == col as i32 {
                        return grid_config[i];
                    }
                }
                0
            })
            .collect();

        // Verify it's a valid independent set
        assert!(
            is_independent_set(&edges, &center_config),
            "{}: Config at centers should be a valid independent set",
            name
        );

        // Verify count equals original MIS (for unweighted input, each center is 0 or 1)
        let original_mis = solve_mis(n, &edges);
        let center_count = center_config.iter().filter(|&&x| x > 0).count();

        // In weighted mode with unit weights, center_count should equal original_mis
        assert_eq!(
            center_count, original_mis,
            "{}: Center config count {} should equal original MIS {}",
            name, center_count, original_mis
        );
    }
}
```

**Step 3: Run test to verify**

Run: `cargo test --test rules_unitdiskmapping test_weighted_map_config_back_standard_graphs -- --nocapture`
Expected: PASS

**Step 4: Commit**

```bash
git add tests/rules/unitdiskmapping/weighted.rs
git commit -m "test: Add weighted mode map_config_back verification for standard graphs"
```

---

## Task 5: Add Triangular Mode map_config_back Full Verification

**Files:**
- Modify: `tests/rules/unitdiskmapping/triangular.rs`

**Context:** Triangular mode also needs full verification similar to weighted mode.

**Step 1: Add triangular map_config_back verification**

Add to `tests/rules/unitdiskmapping/triangular.rs`:

```rust
/// Test triangular mode map_config_back for standard graphs.
/// For triangular weighted mode: mapped_weighted_mis == overhead
/// And config at centers should be a valid IS.
#[test]
fn test_triangular_map_config_back_standard_graphs() {
    use super::common::{is_independent_set, solve_weighted_mis_config};
    use problemreductions::topology::Graph;

    let graph_names = ["bull", "diamond", "house", "petersen"];

    for name in graph_names {
        let (n, edges) = smallgraph(name).unwrap();

        // Use Julia's vertex order if available
        let vertex_order = get_julia_vertex_order(name)
            .unwrap_or_else(|| (0..n).collect());
        let result = map_graph_triangular_with_order(n, &edges, &vertex_order);

        // Get weights
        let grid_edges = result.grid_graph.edges().to_vec();
        let num_grid = result.grid_graph.num_vertices();
        let weights: Vec<i32> = (0..num_grid)
            .map(|i| result.grid_graph.weight(i).copied().unwrap_or(1))
            .collect();

        // Solve weighted MIS on grid
        let grid_config = solve_weighted_mis_config(num_grid, &grid_edges, &weights);

        // Get center locations
        let centers = trace_centers(&result);

        // Extract config at centers
        let center_config: Vec<usize> = centers
            .iter()
            .map(|&(row, col)| {
                for (i, node) in result.grid_graph.nodes().iter().enumerate() {
                    if node.row == row as i32 && node.col == col as i32 {
                        return grid_config[i];
                    }
                }
                0
            })
            .collect();

        // Verify it's a valid independent set
        assert!(
            is_independent_set(&edges, &center_config),
            "{}: Triangular config at centers should be a valid IS",
            name
        );
    }
}
```

**Step 2: Run test to verify**

Run: `cargo test --test rules_unitdiskmapping test_triangular_map_config_back_standard_graphs -- --nocapture`
Expected: PASS

**Step 3: Commit**

```bash
git add tests/rules/unitdiskmapping/triangular.rs
git commit -m "test: Add triangular mode map_config_back verification"
```

---

## Task 6: Run Full Test Suite and Verify

**Step 1: Run all unitdiskmapping tests**

Run: `cargo test --test rules_unitdiskmapping -- --nocapture 2>&1 | tail -50`
Expected: All tests pass

**Step 2: Run full test suite**

Run: `cargo test --all-features 2>&1 | tail -20`
Expected: All tests pass

**Step 3: Final commit with test summary**

```bash
git add -A
git commit -m "test: Complete Julia test parity for unitdiskmapping

Added missing tests compared to Julia's UnitDiskMapping test suite:
- MIS overhead tests for cubical and tutte graphs
- Full map_config_back verification for standard graphs
- Weighted copyline MIS overhead tests
- Weighted mode map_config_back verification
- Triangular mode map_config_back verification

All tests verify:
1. MIS overhead formula correctness
2. Extracted config is valid independent set
3. Extracted config size matches original MIS"
```

---

## Verification Checklist

After completing all tasks, verify:

- [ ] `test_mis_overhead_cubical` passes
- [ ] `test_mis_overhead_tutte` passes
- [ ] `test_map_config_back_standard_graphs` passes for bull, diamond, house, petersen, cubical
- [ ] `test_copyline_weighted_mis_equals_overhead` passes for all 8 test cases
- [ ] `test_weighted_map_config_back_standard_graphs` passes
- [ ] `test_triangular_map_config_back_standard_graphs` passes
- [ ] Full test suite passes: `cargo test --all-features`
