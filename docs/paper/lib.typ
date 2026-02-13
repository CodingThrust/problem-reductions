// Graph visualization library for the problem-reductions paper
#import "@preview/cetz:0.4.2": canvas, draw

// Color palette for k-coloring visualizations
#let graph-colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"))

// Weight-based color map for grid graph nodes
#let weight-color(w) = if w == 1 { blue } else if w == 2 { red } else { green }

// ── Core drawing function ──────────────────────────────────────

// Draw a graph from vertex positions and edge index pairs.
//   vertices: array of (x, y) positions
//   edges: array of (i, j) index pairs
//   node-fill: single color or array of per-vertex colors
//   node-labels: none, "index", or array of label strings
#let draw-graph(
  vertices,
  edges,
  radius: 0.12,
  node-stroke: 0.5pt,
  edge-stroke: 0.6pt + gray,
  node-fill: white,
  node-labels: none,
  label-size: 6pt,
) = {
  import draw: *
  // Draw edges first (behind nodes)
  for (u, v) in edges {
    line(vertices.at(u), vertices.at(v), stroke: edge-stroke)
  }
  // Draw nodes
  for (k, pos) in vertices.enumerate() {
    let fill = if type(node-fill) == array { node-fill.at(k) } else { node-fill }
    circle(pos, radius: radius, fill: fill, stroke: node-stroke, name: str(k))
    if node-labels != none {
      let label = if node-labels == "index" { str(k) } else { node-labels.at(k) }
      content(str(k), text(label-size, label))
    }
  }
}

// ── Pre-defined graph layouts ──────────────────────────────────

// Petersen graph: outer pentagon (0-4) + inner star (5-9)
#let petersen-graph() = {
  let r-outer = 1.2
  let r-inner = 0.6
  let vertices = ()
  for i in range(5) {
    let angle = 90deg - i * 72deg
    vertices.push((calc.cos(angle) * r-outer, calc.sin(angle) * r-outer))
  }
  for i in range(5) {
    let angle = 90deg - i * 72deg
    vertices.push((calc.cos(angle) * r-inner, calc.sin(angle) * r-inner))
  }
  let edges = (
    (0,1),(0,4),(0,5),(1,2),(1,6),(2,3),(2,7),(3,4),(3,8),(4,9),
    (5,7),(5,8),(6,8),(6,9),(7,9),
  )
  (vertices: vertices, edges: edges)
}

// House graph: square base (0-1-3-2) + triangle roof (2-3-4)
#let house-graph() = {
  let vertices = ((0, 0), (1, 0), (0, 1), (1, 1), (0.5, 1.7))
  let edges = ((0,1),(0,2),(1,3),(2,3),(2,4),(3,4))
  (vertices: vertices, edges: edges)
}

// Octahedral graph (K_{2,2,2}): 6 vertices, 12 edges
// Layout: top/bottom poles with 4 equatorial vertices
#let octahedral-graph() = {
  let vertices = (
    (0, -1.2),   // 0: bottom pole
    (-1.0, 0),   // 1: left
    (0, 0.5),    // 2: upper-center
    (0, -0.5),   // 3: lower-center
    (1.0, 0),    // 4: right
    (0, 1.2),    // 5: top pole
  )
  let edges = (
    (0,1),(0,2),(0,3),(0,4),(1,2),(1,3),(1,5),(2,4),(2,5),(3,4),(3,5),(4,5),
  )
  (vertices: vertices, edges: edges)
}

// ── Grid graph functions (JSON-driven) ─────────────────────────

// King's subgraph from JSON with weight-based coloring
#let draw-grid-graph(data, cell-size: 0.2) = canvas(length: 1cm, {
  import draw: *
  let grid-data = data.grid_graph
  let positions = grid-data.nodes.map(n => (n.col * cell-size, -n.row * cell-size))
  let weights = grid-data.nodes.map(n => n.weight)

  for edge in grid-data.edges {
    line(positions.at(edge.at(0)), positions.at(edge.at(1)), stroke: 0.4pt + gray)
  }
  for (k, pos) in positions.enumerate() {
    circle(pos, radius: 0.04, fill: weight-color(weights.at(k)), stroke: none)
  }
})

// Triangular lattice from JSON with weight-based coloring
// Matches Rust GridGraph::physical_position_static for Triangular (offset_even_cols=true)
#let draw-triangular-graph(data, cell-size: 0.2) = canvas(length: 1cm, {
  import draw: *
  let grid-data = data.grid_graph
  let sqrt3_2 = calc.sqrt(3) / 2
  let positions = grid-data.nodes.map(n => {
    let offset = if calc.rem(n.col, 2) == 0 { 0.5 } else { 0.0 }
    ((n.row + offset) * cell-size, -n.col * sqrt3_2 * cell-size)
  })
  let weights = grid-data.nodes.map(n => n.weight)

  for edge in grid-data.edges {
    line(positions.at(edge.at(0)), positions.at(edge.at(1)), stroke: 0.3pt + gray)
  }
  for (k, pos) in positions.enumerate() {
    circle(pos, radius: 0.025, fill: weight-color(weights.at(k)), stroke: none)
  }
})
