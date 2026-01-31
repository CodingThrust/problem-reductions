#import "@preview/cetz:0.3.2"

// Load JSON data for different modes
// Paths relative to this document (docs/paper/)
#let load_julia_unweighted(name) = json("../../tests/julia/" + name + "_unweighted_trace.json")
#let load_julia_weighted(name) = json("../../tests/julia/" + name + "_weighted_trace.json")
#let load_julia_triangular(name) = json("../../tests/julia/" + name + "_triangular_trace.json")
#let load_rust_square(name) = json("../../tests/julia/" + name + "_rust_unweighted.json")
#let load_rust_triangular(name) = json("../../tests/julia/" + name + "_rust_triangular.json")

// Color scheme
#let julia_color = rgb("#2196F3")  // Blue
#let rust_color = rgb("#FF5722")   // Orange
#let both_color = rgb("#4CAF50")   // Green

// Create position key for comparison
#let pos_key(r, c) = str(r) + "," + str(c)

// Convert Julia 1-indexed nodes to 0-indexed (Rust is already 0-indexed)
// Preserves state field if present (O=Occupied, D=Doubled, C=Connected)
#let julia_to_0indexed(nodes) = nodes.map(n => (
  row: n.row - 1,
  col: n.col - 1,
  weight: if "weight" in n { n.weight } else { 1 },
  state: if "state" in n { n.state } else { "O" }
))

// Extract copyline nodes from Julia copy_lines (convert to 0-indexed)
#let julia_copylines_to_nodes(copy_lines) = {
  let nodes = ()
  for cl in copy_lines {
    for loc in cl.locations {
      nodes.push((
        row: loc.row - 1,
        col: loc.col - 1,
        weight: if "weight" in loc { loc.weight } else { 1 }
      ))
    }
  }
  nodes
}

// Color for connected cells
#let connected_color = rgb("#E91E63")  // Pink/Magenta for Connected

// Draw grid with nodes - all positions are 0-indexed
// triangular: if true, offset odd rows by 0.5 for triangular lattice
// Shows Connected cells (state="C") with a different color (ring)
#let draw_grid(nodes, grid_size, title, node_color: black, unit: 4pt, triangular: false) = {
  let rows = grid_size.at(0)
  let cols = grid_size.at(1)
  let occupied = nodes.map(n => pos_key(n.row, n.col))

  // Helper to compute x position with optional triangular offset
  let get_x(r, c) = {
    if triangular and calc.rem(r, 2) == 1 { c + 1.0 }  // odd rows offset by 0.5
    else { c + 0.5 }
  }

  cetz.canvas(length: unit, {
    import cetz.draw: *
    content((cols / 2, rows + 2), text(size: 7pt, weight: "bold", title))

    // Draw empty grid sites as small dots
    for r in range(0, rows) {
      for c in range(0, cols) {
        let key = pos_key(r, c)
        if not occupied.contains(key) {
          let y = rows - r - 0.5
          let x = get_x(r, c)
          circle((x, y), radius: 0.08, fill: luma(200), stroke: none)
        }
      }
    }

    // Draw filled nodes - Connected cells shown with different color
    for node in nodes {
      let r = node.row
      let c = node.col
      let w = if "weight" in node { node.weight } else { 1 }
      let s = if "state" in node { node.state } else { "O" }
      let y = rows - r - 0.5
      let x = get_x(r, c)
      let radius = if w == 1 { 0.25 } else if w == 2 { 0.35 } else { 0.45 }
      // Connected cells shown with ring (stroke) instead of fill
      if s == "C" {
        circle((x, y), radius: radius, fill: none, stroke: 1.5pt + connected_color)
      } else {
        circle((x, y), radius: radius, fill: node_color, stroke: none)
      }
    }
  })
}

// Compare two node sets
#let compare_nodes(julia_nodes, rust_nodes) = {
  let julia_keys = julia_nodes.map(n => pos_key(n.row, n.col))
  let rust_keys = rust_nodes.map(n => pos_key(n.row, n.col))
  let both = ()
  let julia_only = ()
  let rust_only = ()

  for n in julia_nodes {
    let key = pos_key(n.row, n.col)
    if rust_keys.contains(key) { both.push((n.row, n.col)) }
    else { julia_only.push((n.row, n.col)) }
  }
  for n in rust_nodes {
    let key = pos_key(n.row, n.col)
    if not julia_keys.contains(key) { rust_only.push((n.row, n.col)) }
  }
  (both: both, julia_only: julia_only, rust_only: rust_only)
}

// Draw comparison grid
#let draw_comparison(julia_nodes, rust_nodes, grid_size, title, unit: 4pt, triangular: false) = {
  let rows = grid_size.at(0)
  let cols = grid_size.at(1)
  let cmp = compare_nodes(julia_nodes, rust_nodes)
  let all_occupied = cmp.both + cmp.julia_only + cmp.rust_only
  let occupied_keys = all_occupied.map(((r, c)) => pos_key(r, c))

  let get_x(r, c) = {
    if triangular and calc.rem(r, 2) == 1 { c + 1.0 }
    else { c + 0.5 }
  }

  cetz.canvas(length: unit, {
    import cetz.draw: *
    content((cols / 2, rows + 3), text(size: 7pt, weight: "bold", title))
    content((cols / 2, rows + 1.5), text(size: 5pt)[
      #box(fill: both_color, width: 4pt, height: 4pt) #cmp.both.len()
      #box(fill: julia_color, width: 4pt, height: 4pt) #cmp.julia_only.len()
      #box(fill: rust_color, width: 4pt, height: 4pt) #cmp.rust_only.len()
    ])

    for r in range(0, rows) {
      for c in range(0, cols) {
        let key = pos_key(r, c)
        if not occupied_keys.contains(key) {
          let y = rows - r - 0.5
          let x = get_x(r, c)
          circle((x, y), radius: 0.08, fill: luma(200), stroke: none)
        }
      }
    }

    for (r, c) in cmp.both {
      circle((get_x(r, c), rows - r - 0.5), radius: 0.35, fill: both_color, stroke: none)
    }
    for (r, c) in cmp.julia_only {
      circle((get_x(r, c), rows - r - 0.5), radius: 0.35, fill: julia_color, stroke: none)
    }
    for (r, c) in cmp.rust_only {
      circle((get_x(r, c), rows - r - 0.5), radius: 0.35, fill: rust_color, stroke: none)
    }
  })
}

// Compare a single mode
// triangular: if true, use triangular lattice visualization (offset odd rows)
#let compare_mode(name, mode, julia, rust, triangular: false) = {
  let grid_size = julia.grid_size
  // Use grid_nodes_copylines_only if available (has state: O, D, C), else fall back to copy_lines
  let julia_copylines = if "grid_nodes_copylines_only" in julia {
    julia_to_0indexed(julia.grid_nodes_copylines_only)
  } else {
    julia_copylines_to_nodes(julia.copy_lines)
  }
  let julia_before_simp = julia_to_0indexed(julia.at("grid_nodes_before_simplifiers", default: julia.grid_nodes))
  let julia_final = julia_to_0indexed(julia.grid_nodes)
  // Rust stages: 0=copylines only, 1=with connections, 2=after crossing, 3=after simplifiers
  let rust_stage1 = rust.stages.at(1).grid_nodes  // with connections (matches Julia copylines_only)
  let rust_stage2 = rust.stages.at(2).grid_nodes  // after crossing gadgets
  let rust_stage3 = rust.stages.at(3).grid_nodes  // after simplifiers

  [
    = #name - #mode

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

    == Copylines with Connections
    #grid(
      columns: 3,
      gutter: 0.3em,
      draw_grid(julia_copylines, grid_size, "Julia", node_color: julia_color, triangular: triangular),
      draw_grid(rust_stage1, grid_size, "Rust", node_color: rust_color, triangular: triangular),
      draw_comparison(julia_copylines, rust_stage1, grid_size, "Diff", triangular: triangular),
    )

    == After Crossing Gadgets
    #grid(
      columns: 3,
      gutter: 0.3em,
      draw_grid(julia_before_simp, grid_size, "Julia", node_color: julia_color, triangular: triangular),
      draw_grid(rust_stage2, grid_size, "Rust", node_color: rust_color, triangular: triangular),
      draw_comparison(julia_before_simp, rust_stage2, grid_size, "Diff", triangular: triangular),
    )

    == Final
    #grid(
      columns: 3,
      gutter: 0.3em,
      draw_grid(julia_final, grid_size, "Julia", node_color: julia_color, triangular: triangular),
      draw_grid(rust_stage3, grid_size, "Rust", node_color: rust_color, triangular: triangular),
      draw_comparison(julia_final, rust_stage3, grid_size, "Diff", triangular: triangular),
    )

    #pagebreak()
  ]
}

// Compare all 3 modes for a graph
#let compare_graph(name) = {
  let julia_uw = load_julia_unweighted(name)
  let julia_w = load_julia_weighted(name)
  let julia_tri = load_julia_triangular(name)
  let rust_sq = load_rust_square(name)
  let rust_tri = load_rust_triangular(name)

  [
    #compare_mode(name, "UnWeighted (Square)", julia_uw, rust_sq)
    #compare_mode(name, "Weighted (Square)", julia_w, rust_sq)
    #compare_mode(name, "Triangular Weighted", julia_tri, rust_tri, triangular: true)
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
