//! Export Rust mapping process stages to JSON for comparison with Julia.
//!
//! Outputs:
//! - {graph}_rust_stages.json: Contains copylines, each stage's grid nodes, and tape
//!
//! Run with: cargo run --example export_mapping_stages -- diamond
//!           cargo run --example export_mapping_stages -- diamond square
//!           cargo run --example export_mapping_stages -- petersen triangular

use problemreductions::rules::unitdiskmapping::{
    create_copylines, apply_triangular_crossing_gadgets, apply_triangular_simplifier_gadgets,
    apply_crossing_gadgets, apply_simplifier_gadgets,
    MappingGrid, CopyLine, triangular_tape_entry_mis_overhead, tape_entry_mis_overhead,
    TRIANGULAR_SPACING, TRIANGULAR_PADDING, SQUARE_SPACING, SQUARE_PADDING,
    mis_overhead_copyline_triangular, mis_overhead_copyline, TapeEntry, TriangularTapeEntry,
};
use problemreductions::topology::smallgraph;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct GridNodeExport {
    row: i32,
    col: i32,
    weight: i32,
    state: String,  // "O" = Occupied, "D" = Doubled, "C" = Connected
}

#[derive(Serialize)]
struct CopyLineExport {
    vertex: usize,
    vslot: usize,
    hslot: usize,
    vstart: usize,
    vstop: usize,
    hstop: usize,
    locations: Vec<LocationExport>,
}

#[derive(Serialize)]
struct LocationExport {
    row: i32,
    col: i32,
}

#[derive(Serialize)]
struct TapeEntryExport {
    index: usize,
    gadget_type: String,
    gadget_idx: usize,
    row: usize,
    col: usize,
    overhead: i32,
}

#[derive(Serialize)]
struct StageExport {
    name: String,
    grid_nodes: Vec<GridNodeExport>,
    num_nodes: usize,
    grid_size: (usize, usize),
}

#[derive(Serialize)]
struct MappingExport {
    graph_name: String,
    mode: String,
    num_vertices: usize,
    num_edges: usize,
    edges: Vec<(usize, usize)>,
    vertex_order: Vec<usize>,
    padding: usize,
    spacing: usize,
    copy_lines: Vec<CopyLineExport>,
    stages: Vec<StageExport>,
    crossing_tape: Vec<TapeEntryExport>,
    simplifier_tape: Vec<TapeEntryExport>,
    copyline_overhead: i32,
    crossing_overhead: i32,
    simplifier_overhead: i32,
    total_overhead: i32,
}

fn gadget_name(idx: usize) -> String {
    match idx {
        0 => "TriCross<false>".to_string(),
        1 => "TriCross<true>".to_string(),
        2 => "TriTConLeft".to_string(),
        3 => "TriTConUp".to_string(),
        4 => "TriTConDown".to_string(),
        5 => "TriTrivialTurnLeft".to_string(),
        6 => "TriTrivialTurnRight".to_string(),
        7 => "TriEndTurn".to_string(),
        8 => "TriTurn".to_string(),
        9 => "TriWTurn".to_string(),
        10 => "TriBranchFix".to_string(),
        11 => "TriBranchFixB".to_string(),
        12 => "TriBranch".to_string(),
        idx if idx >= 100 => format!("DanglingLeg_{}", idx - 100),
        _ => format!("Unknown_{}", idx),
    }
}

// IMPORTANT: Grid coordinates are exported as 0-indexed (Rust native).
// The Typst script converts to 1-indexed for comparison with Julia.
// DO NOT add +1 here - keep 0-indexed!
fn extract_grid_nodes(grid: &MappingGrid) -> Vec<GridNodeExport> {
    use problemreductions::rules::unitdiskmapping::CellState;
    let mut nodes = Vec::new();
    let (rows, cols) = grid.size();
    for r in 0..rows {
        for c in 0..cols {
            if let Some(cell) = grid.get(r, c) {
                if !cell.is_empty() {
                    let state = match cell {
                        CellState::Occupied { .. } => "O",
                        CellState::Doubled { .. } => "D",
                        CellState::Connected { .. } => "C",
                        CellState::Empty => ".",
                    };
                    nodes.push(GridNodeExport {
                        row: r as i32,  // 0-indexed - DO NOT change!
                        col: c as i32,  // 0-indexed - DO NOT change!
                        weight: cell.weight(),
                        state: state.to_string(),
                    });
                }
            }
        }
    }
    nodes.sort_by_key(|n| (n.row, n.col));
    nodes
}

fn crossat_triangular(
    copylines: &[CopyLine],
    v: usize,
    w: usize,
    spacing: usize,
    padding: usize,
) -> (usize, usize) {
    let line_v = &copylines[v];
    let line_w = &copylines[w];

    let (line_first, line_second) = if line_v.vslot < line_w.vslot {
        (line_v, line_w)
    } else {
        (line_w, line_v)
    };

    let hslot = line_first.hslot;
    let max_vslot = line_second.vslot;

    // 0-indexed coordinates
    let row = (hslot - 1) * spacing + 1 + padding;  // 0-indexed
    let col = (max_vslot - 1) * spacing + padding;  // 0-indexed
    (row, col)
}

fn get_vertex_order_from_julia(graph_name: &str) -> Option<Vec<usize>> {
    let path = format!("tests/julia/{}_triangular_trace.json", graph_name);
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(copy_lines) = data["copy_lines"].as_array() {
                let mut lines: Vec<_> = copy_lines.iter()
                    .filter_map(|cl| {
                        let vertex = cl["vertex"].as_u64()? as usize;
                        let vslot = cl["vslot"].as_u64()? as usize;
                        Some((vertex - 1, vslot))  // Convert to 0-indexed
                    })
                    .collect();
                lines.sort_by_key(|(_, vslot)| *vslot);
                return Some(lines.into_iter().map(|(v, _)| v).collect());
            }
        }
    }
    None
}

fn square_gadget_name(idx: usize) -> String {
    // Must match indices in gadgets_unweighted.rs tape_entry_mis_overhead
    match idx {
        0 => "Cross<false>".to_string(),
        1 => "Turn".to_string(),
        2 => "WTurn".to_string(),
        3 => "Branch".to_string(),
        4 => "BranchFix".to_string(),
        5 => "TCon".to_string(),
        6 => "TrivialTurn".to_string(),
        7 => "RotatedTCon".to_string(),
        8 => "ReflectedCross<true>".to_string(),
        9 => "ReflectedTrivialTurn".to_string(),
        10 => "BranchFixB".to_string(),
        11 => "EndTurn".to_string(),
        12 => "ReflectedRotatedTCon".to_string(),
        idx if idx >= 100 => format!("DanglingLeg_{}", idx - 100),
        _ => format!("Unknown_{}", idx),
    }
}

fn export_triangular(graph_name: &str, n: usize, edges: &[(usize, usize)], vertex_order: &[usize]) -> MappingExport {
    let spacing = TRIANGULAR_SPACING;
    let padding = TRIANGULAR_PADDING;

    let copylines = create_copylines(n, edges, vertex_order);

    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);
    let max_vstop = copylines.iter().map(|l| l.vstop).max().unwrap_or(1);
    let rows = max_hslot.max(max_vstop) * spacing + 2 + 2 * padding;
    let cols = (n - 1) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);
    for line in &copylines {
        for (row, col, weight) in line.copyline_locations_triangular(padding, spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }
    let stage1_nodes = extract_grid_nodes(&grid);

    for &(u, v) in edges {
        let u_line = &copylines[u];
        let v_line = &copylines[v];
        let (smaller_line, larger_line) = if u_line.vslot < v_line.vslot {
            (u_line, v_line)
        } else {
            (v_line, u_line)
        };
        let (row, col) = crossat_triangular(&copylines, smaller_line.vertex, larger_line.vertex, spacing, padding);
        if col > 0 {
            grid.connect(row, col - 1);
        }
        if row > 0 && grid.is_occupied(row - 1, col) {
            grid.connect(row - 1, col);
        } else if row + 1 < grid.size().0 && grid.is_occupied(row + 1, col) {
            grid.connect(row + 1, col);
        }
    }
    let stage2_nodes = extract_grid_nodes(&grid);

    let crossing_tape = apply_triangular_crossing_gadgets(&mut grid, &copylines, spacing, padding);
    let stage3_nodes = extract_grid_nodes(&grid);

    let simplifier_tape = apply_triangular_simplifier_gadgets(&mut grid, 10);
    let stage4_nodes = extract_grid_nodes(&grid);

    let copyline_overhead: i32 = copylines.iter()
        .map(|line| mis_overhead_copyline_triangular(line, spacing))
        .sum();
    let crossing_overhead: i32 = crossing_tape.iter()
        .map(triangular_tape_entry_mis_overhead)
        .sum();
    let simplifier_overhead: i32 = simplifier_tape.iter()
        .map(triangular_tape_entry_mis_overhead)
        .sum();

    let copy_lines_export = export_copylines_triangular(&copylines, padding, spacing);
    let crossing_tape_export = export_triangular_tape(&crossing_tape, 0);
    let simplifier_tape_export = export_triangular_tape(&simplifier_tape, crossing_tape.len());

    create_export(
        graph_name, "TriangularWeighted", n, edges, vertex_order,
        padding, spacing, rows, cols,
        copy_lines_export, stage1_nodes, stage2_nodes, stage3_nodes, stage4_nodes,
        crossing_tape_export, simplifier_tape_export,
        copyline_overhead, crossing_overhead, simplifier_overhead,
    )
}

fn export_square(graph_name: &str, n: usize, edges: &[(usize, usize)], vertex_order: &[usize]) -> MappingExport {
    let spacing = SQUARE_SPACING;
    let padding = SQUARE_PADDING;

    let copylines = create_copylines(n, edges, vertex_order);

    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);
    let max_vstop = copylines.iter().map(|l| l.vstop).max().unwrap_or(1);
    let rows = max_hslot.max(max_vstop) * spacing + 2 + 2 * padding;
    let cols = (n - 1) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);
    for line in &copylines {
        for (row, col, _weight) in line.copyline_locations(padding, spacing) {
            grid.add_node(row, col, 1);  // All weight 1 for square unweighted
        }
    }
    let stage1_nodes = extract_grid_nodes(&grid);

    for &(u, v) in edges {
        let u_line = &copylines[u];
        let v_line = &copylines[v];
        let (smaller_line, larger_line) = if u_line.vslot < v_line.vslot {
            (u_line, v_line)
        } else {
            (v_line, u_line)
        };
        let (row, col) = crossat_square(&copylines, smaller_line.vertex, larger_line.vertex, spacing, padding);
        // Julia's connect logic: always mark (I, J-1), then check (I-1, J) or (I+1, J)
        if col > 0 {
            grid.connect(row, col - 1);
        }
        // Julia: if !isempty(ug.content[I-1, J]) then mark (I-1, J) else mark (I+1, J)
        // Check if there's a copyline node at (row-1, col) to determine direction
        if row > 0 && grid.is_occupied(row - 1, col) {
            grid.connect(row - 1, col);
        } else {
            grid.connect(row + 1, col);
        }
    }
    let stage2_nodes = extract_grid_nodes(&grid);

    let crossing_tape = apply_crossing_gadgets(&mut grid, &copylines);
    let stage3_nodes = extract_grid_nodes(&grid);

    let simplifier_tape = apply_simplifier_gadgets(&mut grid, 2);
    let stage4_nodes = extract_grid_nodes(&grid);

    let copyline_overhead: i32 = copylines.iter()
        .map(|line| mis_overhead_copyline(line, spacing, padding) as i32)
        .sum();
    let crossing_overhead: i32 = crossing_tape.iter()
        .map(tape_entry_mis_overhead)
        .sum();
    let simplifier_overhead: i32 = simplifier_tape.iter()
        .map(tape_entry_mis_overhead)
        .sum();

    let copy_lines_export = export_copylines_square(&copylines, padding, spacing);
    let crossing_tape_export = export_square_tape(&crossing_tape, 0);
    let simplifier_tape_export = export_square_tape(&simplifier_tape, crossing_tape.len());

    create_export(
        graph_name, "UnWeighted", n, edges, vertex_order,
        padding, spacing, rows, cols,
        copy_lines_export, stage1_nodes, stage2_nodes, stage3_nodes, stage4_nodes,
        crossing_tape_export, simplifier_tape_export,
        copyline_overhead, crossing_overhead, simplifier_overhead,
    )
}

fn export_weighted(graph_name: &str, n: usize, edges: &[(usize, usize)], vertex_order: &[usize]) -> MappingExport {
    let spacing = SQUARE_SPACING;
    let padding = SQUARE_PADDING;

    let copylines = create_copylines(n, edges, vertex_order);

    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);
    let max_vstop = copylines.iter().map(|l| l.vstop).max().unwrap_or(1);
    let rows = max_hslot.max(max_vstop) * spacing + 2 + 2 * padding;
    let cols = (n - 1) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);
    for line in &copylines {
        for (row, col, _weight) in line.copyline_locations(padding, spacing) {
            grid.add_node(row, col, 2);  // Weight 2 for weighted mode
        }
    }
    let stage1_nodes = extract_grid_nodes(&grid);

    for &(u, v) in edges {
        let u_line = &copylines[u];
        let v_line = &copylines[v];
        let (smaller_line, larger_line) = if u_line.vslot < v_line.vslot {
            (u_line, v_line)
        } else {
            (v_line, u_line)
        };
        let (row, col) = crossat_square(&copylines, smaller_line.vertex, larger_line.vertex, spacing, padding);
        if col > 0 {
            grid.connect(row, col - 1);
        }
        if row > 0 && grid.is_occupied(row - 1, col) {
            grid.connect(row - 1, col);
        } else {
            grid.connect(row + 1, col);
        }
    }
    let stage2_nodes = extract_grid_nodes(&grid);

    let crossing_tape = apply_crossing_gadgets(&mut grid, &copylines);
    let stage3_nodes = extract_grid_nodes(&grid);

    let simplifier_tape = apply_simplifier_gadgets(&mut grid, 2);
    let stage4_nodes = extract_grid_nodes(&grid);

    // Weighted mode: overhead = unweighted_overhead * 2
    let copyline_overhead: i32 = copylines.iter()
        .map(|line| mis_overhead_copyline(line, spacing, padding) as i32 * 2)
        .sum();
    let crossing_overhead: i32 = crossing_tape.iter()
        .map(|e| tape_entry_mis_overhead(e) * 2)
        .sum();
    let simplifier_overhead: i32 = simplifier_tape.iter()
        .map(|e| tape_entry_mis_overhead(e) * 2)
        .sum();

    let copy_lines_export = export_copylines_square(&copylines, padding, spacing);
    let crossing_tape_export = export_square_tape(&crossing_tape, 0);
    let simplifier_tape_export = export_square_tape(&simplifier_tape, crossing_tape.len());

    create_export(
        graph_name, "Weighted", n, edges, vertex_order,
        padding, spacing, rows, cols,
        copy_lines_export, stage1_nodes, stage2_nodes, stage3_nodes, stage4_nodes,
        crossing_tape_export, simplifier_tape_export,
        copyline_overhead, crossing_overhead, simplifier_overhead,
    )
}

fn crossat_square(
    copylines: &[CopyLine],
    v: usize,
    w: usize,
    spacing: usize,
    padding: usize,
) -> (usize, usize) {
    let line_v = &copylines[v];
    let line_w = &copylines[w];

    let (line_first, line_second) = if line_v.vslot < line_w.vslot {
        (line_v, line_w)
    } else {
        (line_w, line_v)
    };

    let hslot = line_first.hslot;
    let max_vslot = line_second.vslot;

    // 0-indexed coordinates (matches center_location formula)
    let row = (hslot - 1) * spacing + 1 + padding;  // 0-indexed
    let col = (max_vslot - 1) * spacing + padding;  // 0-indexed
    (row, col)
}

// IMPORTANT: Locations are 0-indexed. Vertex is 1-indexed for display only.
// DO NOT add +1 to row/col - keep 0-indexed!
fn export_copylines_triangular(copylines: &[CopyLine], padding: usize, spacing: usize) -> Vec<CopyLineExport> {
    copylines.iter().map(|cl| {
        let locs = cl.copyline_locations_triangular(padding, spacing);
        CopyLineExport {
            vertex: cl.vertex + 1,  // 1-indexed for display
            vslot: cl.vslot,
            hslot: cl.hslot,
            vstart: cl.vstart,
            vstop: cl.vstop,
            hstop: cl.hstop,
            locations: locs.iter().map(|(r, c, _)| LocationExport {
                row: *r as i32,  // 0-indexed - DO NOT change!
                col: *c as i32,  // 0-indexed - DO NOT change!
            }).collect(),
        }
    }).collect()
}

// IMPORTANT: Locations are 0-indexed. DO NOT add +1 to row/col!
fn export_copylines_square(copylines: &[CopyLine], padding: usize, spacing: usize) -> Vec<CopyLineExport> {
    copylines.iter().map(|cl| {
        let locs = cl.copyline_locations(padding, spacing);
        CopyLineExport {
            vertex: cl.vertex + 1,  // 1-indexed for display
            vslot: cl.vslot,
            hslot: cl.hslot,
            vstart: cl.vstart,
            vstop: cl.vstop,
            hstop: cl.hstop,
            locations: locs.iter().map(|(r, c, _)| LocationExport {
                row: *r as i32,  // 0-indexed - DO NOT change!
                col: *c as i32,  // 0-indexed - DO NOT change!
            }).collect(),
        }
    }).collect()
}

// IMPORTANT: Tape positions are 0-indexed. DO NOT add +1 to row/col!
fn export_triangular_tape(tape: &[TriangularTapeEntry], offset: usize) -> Vec<TapeEntryExport> {
    tape.iter().enumerate()
        .map(|(i, e)| TapeEntryExport {
            index: offset + i + 1,  // 1-indexed for display
            gadget_type: gadget_name(e.gadget_idx),
            gadget_idx: e.gadget_idx,
            row: e.row,  // 0-indexed - DO NOT change!
            col: e.col,  // 0-indexed - DO NOT change!
            overhead: triangular_tape_entry_mis_overhead(e),
        }).collect()
}

// IMPORTANT: Tape positions are 0-indexed. DO NOT add +1 to row/col!
fn export_square_tape(tape: &[TapeEntry], offset: usize) -> Vec<TapeEntryExport> {
    tape.iter().enumerate()
        .map(|(i, e)| TapeEntryExport {
            index: offset + i + 1,  // 1-indexed for display
            gadget_type: square_gadget_name(e.pattern_idx),
            gadget_idx: e.pattern_idx,
            row: e.row,  // 0-indexed - DO NOT change!
            col: e.col,  // 0-indexed - DO NOT change!
            overhead: tape_entry_mis_overhead(e),
        }).collect()
}

#[allow(clippy::too_many_arguments)]
fn create_export(
    graph_name: &str, mode: &str, n: usize, edges: &[(usize, usize)], vertex_order: &[usize],
    padding: usize, spacing: usize, rows: usize, cols: usize,
    copy_lines: Vec<CopyLineExport>,
    stage1: Vec<GridNodeExport>, stage2: Vec<GridNodeExport>,
    stage3: Vec<GridNodeExport>, stage4: Vec<GridNodeExport>,
    crossing_tape: Vec<TapeEntryExport>, simplifier_tape: Vec<TapeEntryExport>,
    copyline_overhead: i32, crossing_overhead: i32, simplifier_overhead: i32,
) -> MappingExport {
    let mut export = MappingExport {
        graph_name: graph_name.to_string(),
        mode: mode.to_string(),
        num_vertices: n,
        num_edges: edges.len(),
        edges: edges.iter().map(|(u, v)| (*u + 1, *v + 1)).collect(),
        vertex_order: vertex_order.iter().map(|v| v + 1).collect(),
        padding,
        spacing,
        copy_lines,
        stages: vec![
            StageExport { name: "copylines_only".to_string(), grid_nodes: stage1, num_nodes: 0, grid_size: (rows, cols) },
            StageExport { name: "with_connections".to_string(), grid_nodes: stage2, num_nodes: 0, grid_size: (rows, cols) },
            StageExport { name: "after_crossing_gadgets".to_string(), grid_nodes: stage3, num_nodes: 0, grid_size: (rows, cols) },
            StageExport { name: "after_simplifiers".to_string(), grid_nodes: stage4, num_nodes: 0, grid_size: (rows, cols) },
        ],
        crossing_tape,
        simplifier_tape,
        copyline_overhead,
        crossing_overhead,
        simplifier_overhead,
        total_overhead: copyline_overhead + crossing_overhead + simplifier_overhead,
    };
    for stage in &mut export.stages {
        stage.num_nodes = stage.grid_nodes.len();
    }
    export
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let graph_name = args.get(1).map(|s| s.as_str()).unwrap_or("diamond");
    let mode = args.get(2).map(|s| s.as_str()).unwrap_or("triangular");

    let (n, edges) = smallgraph(graph_name).expect("Unknown graph");

    let vertex_order = get_vertex_order_from_julia(graph_name)
        .unwrap_or_else(|| (0..n).collect());

    let (export, suffix) = match mode {
        "unweighted" | "square" => (export_square(graph_name, n, &edges, &vertex_order), "_rust_unweighted"),
        "weighted" => (export_weighted(graph_name, n, &edges, &vertex_order), "_rust_weighted"),
        "triangular" | _ => (export_triangular(graph_name, n, &edges, &vertex_order), "_rust_triangular"),
    };

    let output_path = format!("tests/julia/{}{}.json", graph_name, suffix);
    let json = serde_json::to_string_pretty(&export).unwrap();
    fs::write(&output_path, &json).expect("Failed to write JSON");
    println!("Exported to: {}", output_path);

    println!("\n=== {} {} Mapping Summary ===", graph_name, export.mode);
    println!("Vertices: {}, Edges: {}", n, edges.len());
    println!("Grid size: {}x{}", export.stages[0].grid_size.0, export.stages[0].grid_size.1);
    println!("\nStages:");
    for stage in &export.stages {
        println!("  {}: {} nodes", stage.name, stage.num_nodes);
    }
    println!("\nTape: {} crossing + {} simplifier", export.crossing_tape.len(), export.simplifier_tape.len());
    println!("Overhead: copyline={} crossing={} simplifier={} total={}",
        export.copyline_overhead, export.crossing_overhead, export.simplifier_overhead, export.total_overhead);
}
