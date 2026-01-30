//! Tests comparing Rust mapping output with Julia's UnitDiskMapping.jl traces.
//!
//! Compares three modes:
//! - UnWeighted (square lattice)
//! - Weighted (square lattice with weights)
//! - Triangular (triangular lattice with weights)

use problemreductions::rules::unitdiskmapping::{map_graph, map_graph_triangular_with_order};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Deserialize)]
struct JuliaTrace {
    graph_name: String,
    mode: String,
    num_vertices: usize,
    num_edges: usize,
    edges: Vec<(usize, usize)>,
    grid_size: (usize, usize),
    num_grid_nodes: usize,
    num_grid_nodes_before_simplifiers: usize,
    mis_overhead: i32,
    original_mis_size: f64,
    #[serde(default)]
    mapped_mis_size: Option<f64>,
    padding: usize,
    grid_nodes: Vec<GridNode>,
    copy_lines: Vec<CopyLineInfo>,
    #[serde(default)]
    tape: Vec<JuliaTapeEntry>,
    #[serde(default)]
    grid_nodes_copylines_only: Vec<GridNodeWithState>,
}

#[derive(Debug, Deserialize)]
struct GridNode {
    row: i32,
    col: i32,
    weight: i32,
}

#[derive(Debug, Deserialize)]
struct GridNodeWithState {
    row: i32,
    col: i32,
    state: String,
}

#[derive(Debug, Deserialize)]
struct CopyLineInfo {
    vertex: usize,
    vslot: usize,
    hslot: usize,
    vstart: usize,
    vstop: usize,
    hstop: usize,
    locations: Vec<Location>,
}

#[derive(Debug, Deserialize)]
struct Location {
    row: i32,
    col: i32,
}

#[derive(Debug, Deserialize)]
struct JuliaTapeEntry {
    index: usize,
    #[serde(rename = "type")]
    gadget_type: String,
    row: i32,
    col: i32,
}

fn load_julia_trace(name: &str, mode: &str) -> JuliaTrace {
    let path = format!("tests/julia/{}_{}_trace.json", name, mode);
    let content = fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {}", path));
    serde_json::from_str(&content).unwrap_or_else(|e| panic!("Failed to parse {}: {}", path, e))
}

/// Get edges from Julia trace (converted from 1-indexed to 0-indexed)
fn get_graph_edges(julia: &JuliaTrace) -> Vec<(usize, usize)> {
    julia.edges
        .iter()
        .map(|(u, v)| (u - 1, v - 1))
        .collect()
}

/// Compare Rust and Julia for square lattice (UnWeighted mode)
fn compare_square_unweighted(name: &str) {
    let julia = load_julia_trace(name, "unweighted");
    let edges = get_graph_edges(&julia);
    let num_vertices = julia.num_vertices;

    let rust_result = map_graph(num_vertices, &edges);

    // Collect Rust grid nodes from copyline_locations (0-indexed)
    let rust_nodes: HashSet<(i32, i32)> = rust_result.lines
        .iter()
        .flat_map(|line| {
            line.copyline_locations(rust_result.padding, rust_result.spacing)
                .into_iter()
                .map(|(row, col, _)| (row as i32, col as i32))
        })
        .collect();

    // Collect Julia copyline nodes (convert from 1-indexed to 0-indexed)
    let julia_nodes: HashSet<(i32, i32)> = julia.copy_lines
        .iter()
        .flat_map(|cl| cl.locations.iter().map(|loc| (loc.row - 1, loc.col - 1)))
        .collect();

    println!("\n=== {} (square/unweighted) ===", name);
    print_comparison(&julia, &rust_result.grid_graph.size(), rust_result.mis_overhead,
                     &julia_nodes, &rust_nodes);

    // Compare copy lines
    compare_copy_lines(&julia.copy_lines, &rust_result.lines);

    // Assertions
    assert_eq!(julia.grid_size, rust_result.grid_graph.size(),
        "{} square: Grid size mismatch", name);
    assert_eq!(julia.mis_overhead, rust_result.mis_overhead,
        "{} square: MIS overhead mismatch", name);
    assert_eq!(julia_nodes.len(), rust_nodes.len(),
        "{} square: Node count mismatch (Julia={}, Rust={})", name, julia_nodes.len(), rust_nodes.len());
    assert_eq!(julia_nodes, rust_nodes,
        "{} square: Node positions don't match", name);
}

/// Get MIS overhead for a Julia gadget type string (triangular/weighted mode)
/// Values from Julia's UnitDiskMapping/src/triangular.jl lines 401-413
/// For simplifiers: Julia uses mis_overhead(w::WeightedGadget) = mis_overhead(w.gadget) * 2
fn julia_gadget_overhead(gadget_type: &str) -> i32 {
    // Order matters - check more specific patterns first
    if gadget_type.contains("TriCross{true") { 1 }
    else if gadget_type.contains("TriCross{false") || gadget_type.contains("TriCross}") { 3 }
    else if gadget_type.contains("TriWTurn") { 0 }
    else if gadget_type.contains("TriBranchFixB") { -2 }
    else if gadget_type.contains("TriBranchFix") { -2 }
    else if gadget_type.contains("TriBranch") { 0 }
    else if gadget_type.contains("TriEndTurn") { -2 }
    else if gadget_type.contains("TriTrivialTurn") { 0 }
    else if gadget_type.contains("TriTurn") { 0 }
    else if gadget_type.contains("TriTCon_left") || gadget_type.contains("TriTCon_l") { 4 }
    else if gadget_type.contains("TriTCon") { 0 }  // TriTCon_up, TriTCon_down
    else if gadget_type.contains("DanglingLeg") { -2 }  // weighted overhead = -1 * 2
    else { 0 }
}

/// Get MIS overhead for a Rust triangular gadget index (triangular/weighted mode)
/// Must match Julia's values from triangular.jl
/// For simplifiers: Julia uses mis_overhead(w::WeightedGadget) = mis_overhead(w.gadget) * 2
fn rust_triangular_gadget_overhead(idx: usize) -> i32 {
    match idx {
        0 => 3,   // TriCross<false>
        1 => 1,   // TriCross<true>
        2 => 4,   // TriTConLeft
        3 => 0,   // TriTConUp
        4 => 0,   // TriTConDown
        5 => 0,   // TriTrivialTurnLeft
        6 => 0,   // TriTrivialTurnRight
        7 => -2,  // TriEndTurn
        8 => 0,   // TriTurn
        9 => 0,   // TriWTurn
        10 => -2, // TriBranchFix
        11 => -2, // TriBranchFixB
        12 => 0,  // TriBranch
        idx if idx >= 100 => -2,  // DanglingLeg: weighted overhead = -1 * 2 = -2
        _ => 0,
    }
}

/// Calculate copyline MIS overhead for triangular mode (matches Julia formula)
fn copyline_overhead_triangular(line: &problemreductions::rules::unitdiskmapping::CopyLine, spacing: usize) -> i32 {
    let s = spacing as i32;
    let vertical_up = (line.hslot as i32 - line.vstart as i32) * s;
    let vertical_down = (line.vstop as i32 - line.hslot as i32) * s;
    let horizontal = ((line.hstop as i32 - line.vslot as i32) * s - 2).max(0);
    vertical_up + vertical_down + horizontal
}

/// Extract vertex order from Julia's copy_lines (sorted by vslot)
fn get_vertex_order(julia: &JuliaTrace) -> Vec<usize> {
    let mut lines: Vec<_> = julia.copy_lines.iter().collect();
    lines.sort_by_key(|l| l.vslot);
    lines.iter().map(|l| l.vertex - 1).collect()  // Convert 1-indexed to 0-indexed
}

/// Compare Rust and Julia for triangular lattice
fn compare_triangular(name: &str) {
    let julia = load_julia_trace(name, "triangular");
    let edges = get_graph_edges(&julia);
    let num_vertices = julia.num_vertices;

    // Extract Julia's vertex order from copy_lines
    let vertex_order = get_vertex_order(&julia);
    let rust_result = map_graph_triangular_with_order(num_vertices, &edges, &vertex_order);

    // Collect Rust grid nodes from copyline_locations_triangular (0-indexed)
    let rust_nodes: HashSet<(i32, i32)> = rust_result.lines
        .iter()
        .flat_map(|line| {
            line.copyline_locations_triangular(rust_result.padding, rust_result.spacing)
                .into_iter()
                .map(|(row, col, _)| (row as i32, col as i32))
        })
        .collect();

    // Collect Julia copyline nodes (convert from 1-indexed to 0-indexed)
    let julia_nodes: HashSet<(i32, i32)> = julia.copy_lines
        .iter()
        .flat_map(|cl| cl.locations.iter().map(|loc| (loc.row - 1, loc.col - 1)))
        .collect();

    println!("\n=== {} (triangular) ===", name);
    print_comparison(&julia, &rust_result.grid_graph.size(), rust_result.mis_overhead,
                     &julia_nodes, &rust_nodes);

    // Compare copy lines
    compare_copy_lines(&julia.copy_lines, &rust_result.lines);

    // Calculate and compare MIS overhead breakdown
    let julia_copyline_overhead: i32 = julia.copy_lines.iter().map(|cl| {
        let s = 6i32;
        let vert_up = (cl.hslot as i32 - cl.vstart as i32) * s;
        let vert_down = (cl.vstop as i32 - cl.hslot as i32) * s;
        let horiz = ((cl.hstop as i32 - cl.vslot as i32) * s - 2).max(0);
        vert_up + vert_down + horiz
    }).sum();

    let rust_copyline_overhead: i32 = rust_result.lines.iter()
        .map(|l| copyline_overhead_triangular(l, rust_result.spacing))
        .sum();

    let julia_gadget_overhead_total: i32 = julia.tape.iter()
        .map(|e| julia_gadget_overhead(&e.gadget_type))
        .sum();

    let rust_gadget_overhead_total: i32 = rust_result.tape.iter()
        .map(|e| rust_triangular_gadget_overhead(e.pattern_idx))
        .sum();

    println!("\nMIS overhead breakdown:");
    println!("  Copyline: Julia={}, Rust={}", julia_copyline_overhead, rust_copyline_overhead);
    println!("  Gadgets:  Julia={}, Rust={}", julia_gadget_overhead_total, rust_gadget_overhead_total);
    println!("  Total:    Julia={}, Rust={}",
        julia_copyline_overhead + julia_gadget_overhead_total,
        rust_copyline_overhead + rust_gadget_overhead_total);
    println!("  Reported: Julia={}, Rust={}", julia.mis_overhead, rust_result.mis_overhead);

    // Compare tape entries
    println!("\nTape comparison (first 10 entries):");
    println!("  Julia tape: {} entries", julia.tape.len());
    println!("  Rust tape:  {} entries", rust_result.tape.len());
    for (i, jt) in julia.tape.iter().take(10).enumerate() {
        let j_oh = julia_gadget_overhead(&jt.gadget_type);
        if let Some(rt) = rust_result.tape.get(i) {
            let r_oh = rust_triangular_gadget_overhead(rt.pattern_idx);
            let pos_match = jt.row == rt.row as i32 && jt.col == rt.col as i32;
            println!("  {:2}. Julia: {} at ({},{}) oh={} | Rust: idx={} at ({},{}) oh={} [{}]",
                i+1, &jt.gadget_type[..jt.gadget_type.len().min(40)], jt.row, jt.col, j_oh,
                rt.pattern_idx, rt.row, rt.col, r_oh,
                if pos_match && j_oh == r_oh { "OK" } else { "DIFF" });
        } else {
            println!("  {:2}. Julia: {} at ({},{}) oh={} | Rust: MISSING",
                i+1, &jt.gadget_type[..jt.gadget_type.len().min(40)], jt.row, jt.col, j_oh);
        }
    }

    // Assertions
    assert_eq!(julia.grid_size, rust_result.grid_graph.size(),
        "{} triangular: Grid size mismatch", name);
    assert_eq!(julia_copyline_overhead, rust_copyline_overhead,
        "{} triangular: Copyline overhead mismatch", name);
    assert_eq!(julia.tape.len(), rust_result.tape.len(),
        "{} triangular: Tape length mismatch (Julia={}, Rust={})",
        name, julia.tape.len(), rust_result.tape.len());
    assert_eq!(julia.mis_overhead, rust_result.mis_overhead,
        "{} triangular: MIS overhead mismatch (Julia={}, Rust={})",
        name, julia.mis_overhead, rust_result.mis_overhead);
    assert_eq!(julia_nodes.len(), rust_nodes.len(),
        "{} triangular: Node count mismatch (Julia={}, Rust={})", name, julia_nodes.len(), rust_nodes.len());
    assert_eq!(julia_nodes, rust_nodes,
        "{} triangular: Node positions don't match", name);
}

fn print_comparison(julia: &JuliaTrace, rust_size: &(usize, usize), rust_overhead: i32,
                    julia_nodes: &HashSet<(i32, i32)>, rust_nodes: &HashSet<(i32, i32)>) {
    println!("Julia: {} vertices, {} edges", julia.num_vertices, julia.num_edges);
    println!("Grid size: Julia {:?}, Rust {:?}", julia.grid_size, rust_size);
    println!("Nodes: Julia {}, Rust {}", julia_nodes.len(), rust_nodes.len());
    println!("MIS overhead: Julia {}, Rust {}", julia.mis_overhead, rust_overhead);

    let only_julia: Vec<_> = julia_nodes.difference(rust_nodes).collect();
    let only_rust: Vec<_> = rust_nodes.difference(julia_nodes).collect();

    if !only_julia.is_empty() {
        println!("Nodes only in Julia ({}):", only_julia.len());
        for &(r, c) in only_julia.iter().take(5) {
            println!("  ({}, {})", r, c);
        }
    }
    if !only_rust.is_empty() {
        println!("Nodes only in Rust ({}):", only_rust.len());
        for &(r, c) in only_rust.iter().take(5) {
            println!("  ({}, {})", r, c);
        }
    }
}

fn compare_copy_lines(julia_lines: &[CopyLineInfo], rust_lines: &[problemreductions::rules::unitdiskmapping::CopyLine]) {
    println!("Copy lines:");
    for jl in julia_lines {
        let julia_vertex_0idx = jl.vertex - 1;
        if let Some(rl) = rust_lines.iter().find(|l| l.vertex == julia_vertex_0idx) {
            let matches = rl.vslot == jl.vslot && rl.hslot == jl.hslot
                && rl.vstart == jl.vstart && rl.vstop == jl.vstop && rl.hstop == jl.hstop;
            if matches {
                println!("  v{} OK", julia_vertex_0idx);
            } else {
                println!("  v{} MISMATCH: Julia({},{},{},{},{}) Rust({},{},{},{},{})",
                    julia_vertex_0idx,
                    jl.vslot, jl.hslot, jl.vstart, jl.vstop, jl.hstop,
                    rl.vslot, rl.hslot, rl.vstart, rl.vstop, rl.hstop);
            }
        } else {
            println!("  v{} missing in Rust!", julia_vertex_0idx);
        }
    }
}

// ============================================================================
// Square Lattice (UnWeighted) Tests
// ============================================================================

#[test]
fn test_square_unweighted_bull() {
    compare_square_unweighted("bull");
}

#[test]
fn test_square_unweighted_diamond() {
    compare_square_unweighted("diamond");
}

#[test]
fn test_square_unweighted_house() {
    compare_square_unweighted("house");
}

#[test]
fn test_square_unweighted_petersen() {
    compare_square_unweighted("petersen");
}

// ============================================================================
// Connected Cell Tests - Verify connect() marks cells correctly
// ============================================================================

/// Test that Connected cells are marked at the correct positions.
/// This tests the fix for the bug where connect() was incorrectly implemented.
/// Julia's connect_cell! converts plain Occupied cells to Connected at crossing points.
fn compare_connected_cells(name: &str) {
    use problemreductions::rules::unitdiskmapping::CellState;

    let julia = load_julia_trace(name, "unweighted");
    let edges = get_graph_edges(&julia);
    let num_vertices = julia.num_vertices;

    // Get Julia's Connected cell positions (convert 1-indexed to 0-indexed)
    let julia_connected: HashSet<(i32, i32)> = julia.grid_nodes_copylines_only
        .iter()
        .filter(|n| n.state == "C")
        .map(|n| (n.row - 1, n.col - 1))
        .collect();

    // Run Rust mapping and get Connected cells from the grid after applying connections
    let rust_result = map_graph(num_vertices, &edges);

    // Re-create the grid with connections to check Connected cell positions
    let mut grid = problemreductions::rules::unitdiskmapping::MappingGrid::with_padding(
        rust_result.grid_graph.size().0,
        rust_result.grid_graph.size().1,
        rust_result.spacing,
        rust_result.padding,
    );

    // Add copyline nodes
    for line in &rust_result.lines {
        for (row, col, weight) in line.copyline_locations(rust_result.padding, rust_result.spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Apply connections (this is what we're testing)
    for &(u, v) in &edges {
        let u_line = &rust_result.lines[u];
        let v_line = &rust_result.lines[v];
        let (smaller_line, larger_line) = if u_line.vslot < v_line.vslot {
            (u_line, v_line)
        } else {
            (v_line, u_line)
        };
        let (row, col) = grid.cross_at(smaller_line.vslot, larger_line.vslot, smaller_line.hslot);
        if col > 0 {
            grid.connect(row, col - 1);
        }
        if row > 0 && grid.is_occupied(row - 1, col) {
            grid.connect(row - 1, col);
        } else {
            grid.connect(row + 1, col);
        }
    }

    // Collect Rust's Connected cell positions
    let rust_connected: HashSet<(i32, i32)> = {
        let (rows, cols) = grid.size();
        let mut connected = HashSet::new();
        for r in 0..rows {
            for c in 0..cols {
                if let Some(CellState::Connected { .. }) = grid.get(r, c) {
                    connected.insert((r as i32, c as i32));
                }
            }
        }
        connected
    };

    println!("\n=== {} Connected Cells Test ===", name);
    println!("Julia Connected: {} cells", julia_connected.len());
    println!("Rust Connected: {} cells", rust_connected.len());

    // Find differences
    let julia_only: Vec<_> = julia_connected.difference(&rust_connected).collect();
    let rust_only: Vec<_> = rust_connected.difference(&julia_connected).collect();

    if !julia_only.is_empty() {
        println!("Julia-only positions: {:?}", julia_only);
    }
    if !rust_only.is_empty() {
        println!("Rust-only positions: {:?}", rust_only);
    }

    assert_eq!(julia_connected.len(), rust_connected.len(),
        "{}: Connected cell count mismatch (Julia={}, Rust={})",
        name, julia_connected.len(), rust_connected.len());
    assert_eq!(julia_connected, rust_connected,
        "{}: Connected cell positions don't match", name);
}

#[test]
fn test_connected_cells_diamond() {
    compare_connected_cells("diamond");
}

#[test]
fn test_connected_cells_bull() {
    compare_connected_cells("bull");
}

#[test]
fn test_connected_cells_house() {
    compare_connected_cells("house");
}

#[test]
fn test_connected_cells_petersen() {
    compare_connected_cells("petersen");
}

// ============================================================================
// Triangular Lattice Tests
// ============================================================================

#[test]
fn test_triangular_bull() {
    compare_triangular("bull");
}

#[test]
fn test_triangular_diamond() {
    compare_triangular("diamond");
}

#[test]
fn test_triangular_house() {
    compare_triangular("house");
}

#[test]
fn test_triangular_petersen() {
    compare_triangular("petersen");
}
