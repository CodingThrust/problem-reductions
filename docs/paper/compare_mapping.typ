#import "@preview/cetz:0.3.2"

// Load JSON data (paths relative to project root with --root .)
#let load_julia(name) = json("/tests/julia/" + name + "_triangular_trace.json")
#let load_rust(name) = json("/tests/julia/" + name + "_rust_stages.json")

// Color scheme
#let julia_color = rgb("#2196F3")  // Blue
#let rust_color = rgb("#FF5722")   // Orange
#let both_color = rgb("#4CAF50")   // Green

// Create position key for comparison
#let pos_key(r, c) = str(r) + "," + str(c)

// Convert Julia 1-indexed nodes to 0-indexed (Rust is already 0-indexed)
// IMPORTANT: Rust exports 0-indexed coordinates, Julia exports 1-indexed
#let julia_to_0indexed(nodes) = nodes.map(n => (
  row: n.row - 1,
  col: n.col - 1,
  weight: if "weight" in n { n.weight } else { 1 }
))

// Convert Julia tape entry to 0-indexed
#let julia_tape_to_0indexed(entry) = (
  row: entry.row - 1,
  col: entry.col - 1,
  gadget_type: if "type" in entry { entry.type } else { "?" },
)

// Draw grid with nodes - grid_size is [rows, cols] array
// All positions are 0-indexed (Rust native format)
#let draw_grid(nodes, grid_size, title, node_color: black, unit: 4pt) = {
  let rows = grid_size.at(0)
  let cols = grid_size.at(1)

  // Create set of occupied positions (0-indexed)
  let occupied = nodes.map(n => pos_key(n.row, n.col))

  cetz.canvas(length: unit, {
    import cetz.draw: *

    // Title
    content((cols / 2, rows + 2), text(size: 7pt, weight: "bold", title))

    // Draw empty grid sites as small dots (0-indexed)
    for r in range(0, rows) {
      for c in range(0, cols) {
        let key = pos_key(r, c)
        if not occupied.contains(key) {
          let y = rows - r - 0.5
          let x = c + 0.5
          circle((x, y), radius: 0.08, fill: luma(200), stroke: none)
        }
      }
    }

    // Draw filled nodes (0-indexed)
    for node in nodes {
      let r = node.row
      let c = node.col
      let w = if "weight" in node { node.weight } else { 1 }
      let y = rows - r - 0.5
      let x = c + 0.5
      let radius = if w == 1 { 0.25 } else if w == 2 { 0.35 } else { 0.45 }
      circle((x, y), radius: radius, fill: node_color, stroke: none)
    }
  })
}

// Compare two node sets - both are 0-indexed
#let compare_nodes(julia_nodes, rust_nodes) = {
  let julia_keys = julia_nodes.map(n => pos_key(n.row, n.col))
  let rust_keys = rust_nodes.map(n => pos_key(n.row, n.col))

  let both = ()
  let julia_only = ()
  let rust_only = ()

  for n in julia_nodes {
    let key = pos_key(n.row, n.col)
    if rust_keys.contains(key) {
      both.push((n.row, n.col))
    } else {
      julia_only.push((n.row, n.col))
    }
  }

  for n in rust_nodes {
    let key = pos_key(n.row, n.col)
    if not julia_keys.contains(key) {
      rust_only.push((n.row, n.col))
    }
  }

  (both: both, julia_only: julia_only, rust_only: rust_only)
}

// Draw comparison grid - all positions are 0-indexed
#let draw_comparison(julia_nodes, rust_nodes, grid_size, title, unit: 4pt) = {
  let rows = grid_size.at(0)
  let cols = grid_size.at(1)
  let cmp = compare_nodes(julia_nodes, rust_nodes)

  // Create set of all occupied positions
  let all_occupied = cmp.both + cmp.julia_only + cmp.rust_only
  let occupied_keys = all_occupied.map(((r, c)) => pos_key(r, c))

  cetz.canvas(length: unit, {
    import cetz.draw: *

    // Title and legend
    content((cols / 2, rows + 3), text(size: 7pt, weight: "bold", title))
    content((cols / 2, rows + 1.5), text(size: 5pt)[
      #box(fill: both_color, width: 4pt, height: 4pt) #cmp.both.len()
      #box(fill: julia_color, width: 4pt, height: 4pt) #cmp.julia_only.len()
      #box(fill: rust_color, width: 4pt, height: 4pt) #cmp.rust_only.len()
    ])

    // Draw empty grid sites as small dots (0-indexed)
    for r in range(0, rows) {
      for c in range(0, cols) {
        let key = pos_key(r, c)
        if not occupied_keys.contains(key) {
          let y = rows - r - 0.5
          let x = c + 0.5
          circle((x, y), radius: 0.08, fill: luma(200), stroke: none)
        }
      }
    }

    // All positions are 0-indexed
    for (r, c) in cmp.both {
      let y = rows - r - 0.5
      let x = c + 0.5
      circle((x, y), radius: 0.35, fill: both_color, stroke: none)
    }

    for (r, c) in cmp.julia_only {
      let y = rows - r - 0.5
      let x = c + 0.5
      circle((x, y), radius: 0.35, fill: julia_color, stroke: none)
    }

    for (r, c) in cmp.rust_only {
      let y = rows - r - 0.5
      let x = c + 0.5
      circle((x, y), radius: 0.35, fill: rust_color, stroke: none)
    }
  })
}

// Format tape entry for display (all 1-indexed)
#let format_tape(entry) = {
  let typ = if "gadget_type" in entry { entry.gadget_type } else if "type" in entry { entry.type } else { "?" }
  let short_typ = if typ.len() > 20 { typ.slice(0, 20) + ".." } else { typ }
  [(#entry.row,#entry.col) #text(size: 5pt)[#short_typ]]
}

// Extract copyline nodes from Julia copy_lines (convert to 0-indexed)
#let julia_copylines_to_nodes(copy_lines) = {
  let nodes = ()
  for cl in copy_lines {
    for loc in cl.locations {
      nodes.push((
        row: loc.row - 1,  // Convert 1-indexed to 0-indexed
        col: loc.col - 1,
        weight: if "weight" in loc { loc.weight } else { 1 }
      ))
    }
  }
  nodes
}

// Main comparison for a graph
#let compare_graph(name) = {
  let julia = load_julia(name)
  let rust = load_rust(name)
  let grid_size = julia.grid_size

  // Convert Julia 1-indexed to 0-indexed (Rust is already 0-indexed)
  let julia_copylines = julia_copylines_to_nodes(julia.copy_lines)
  let julia_before_simp = julia_to_0indexed(julia.at("grid_nodes_before_simplifiers", default: julia.grid_nodes))
  let julia_final = julia_to_0indexed(julia.grid_nodes)
  let julia_tape = julia.tape.map(julia_tape_to_0indexed)

  // Rust is already 0-indexed - use directly
  let rust_stage0 = rust.stages.at(0).grid_nodes  // copylines only
  let rust_stage2 = rust.stages.at(2).grid_nodes  // after crossing gadgets
  let rust_stage3 = rust.stages.at(3).grid_nodes  // after simplifiers
  let all_rust_tape = rust.crossing_tape + rust.simplifier_tape

  [
    = #name Graph

    == Overview
    #table(
      columns: 3,
      stroke: 0.5pt,
      [*Metric*], [*Julia*], [*Rust*],
      [Vertices], [#julia.num_vertices], [#rust.num_vertices],
      [Grid], [#grid_size.at(0)×#grid_size.at(1)], [#rust.stages.at(0).grid_size.at(0)×#rust.stages.at(0).grid_size.at(1)],
      [Final Nodes], [#julia.num_grid_nodes], [#rust.stages.at(3).num_nodes],
      [Before Simpl], [#julia.num_grid_nodes_before_simplifiers], [#rust.stages.at(2).num_nodes],
      [Overhead], [#julia.mis_overhead], [#rust.total_overhead],
      [Tape], [#julia.tape.len()], [#(rust.crossing_tape.len() + rust.simplifier_tape.len())],
    )

    == Tape (first 10)
    #{
      let max_entries = calc.min(10, calc.max(julia_tape.len(), all_rust_tape.len()))
      table(
        columns: 4,
        stroke: 0.5pt,
        [*\#*], [*Julia*], [*Rust*], [*OK*],
        ..range(0, max_entries).map(i => {
          let jt = if i < julia_tape.len() { julia_tape.at(i) } else { none }
          let rt = if i < all_rust_tape.len() { all_rust_tape.at(i) } else { none }
          let j_str = if jt != none { format_tape(jt) } else { [-] }
          let r_str = if rt != none { format_tape(rt) } else { [-] }
          // Both are 0-indexed now (Julia converted)
          let pos_ok = if jt != none and rt != none and jt.row == rt.row and jt.col == rt.col { [✓] } else { [✗] }
          ([#(i+1)], j_str, r_str, pos_ok)
        }).flatten()
      )
    }

    == Copylines Only (before crossing gadgets)
    #grid(
      columns: 3,
      gutter: 0.3em,
      draw_grid(julia_copylines, grid_size, "Julia", node_color: julia_color),
      draw_grid(rust_stage0, grid_size, "Rust", node_color: rust_color),
      draw_comparison(julia_copylines, rust_stage0, grid_size, "Diff"),
    )

    == After Crossing Gadgets
    #grid(
      columns: 3,
      gutter: 0.3em,
      draw_grid(julia_before_simp, grid_size, "Julia", node_color: julia_color),
      draw_grid(rust_stage2, grid_size, "Rust", node_color: rust_color),
      draw_comparison(julia_before_simp, rust_stage2, grid_size, "Diff"),
    )

    == Final
    #grid(
      columns: 3,
      gutter: 0.3em,
      draw_grid(julia_final, grid_size, "Julia", node_color: julia_color),
      draw_grid(rust_stage3, grid_size, "Rust", node_color: rust_color),
      draw_comparison(julia_final, rust_stage3, grid_size, "Diff"),
    )

    #pagebreak()
  ]
}

// Document setup
#set page(margin: 0.6cm, paper: "a4")
#set text(size: 6pt)

#align(center, text(size: 12pt, weight: "bold")[Julia vs Rust Mapping Comparison])
#v(0.3em)

#compare_graph("diamond")
#compare_graph("bull")
#compare_graph("house")
#compare_graph("petersen")
