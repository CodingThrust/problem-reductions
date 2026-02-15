// Graph visualization library for the problem-reductions paper
#import "@preview/cetz:0.4.2": canvas, draw

// ── Style defaults ─────────────────────────────────────────────

// Color palette for k-coloring visualizations
#let graph-colors = (rgb("#4e79a7"), rgb("#e15759"), rgb("#76b7b2"))

// Weight-based fill colors for grid graph nodes
#let weight-color(w) = if w == 1 { blue } else if w == 2 { red } else { green }

// ── Primitives: g-node, g-edge ─────────────────────────────────
// All graph drawing goes through these two functions.
// They define the standard style; callers can override any parameter.

// Draw a single graph node.
//   pos: (x, y) position
//   name: CetZ element name (for edge references)
//   label: none or content to place inside the node
#let g-node(
  pos,
  name: none,
  radius: 0.2,
  fill: white,
  stroke: 0.5pt,
  label: none,
  label-size: 8pt,
) = {
  draw.circle(pos, radius: radius, fill: fill, stroke: stroke, name: name)
  if label != none {
    draw.content(name, text(label-size, label))
  }
}

// Draw a single graph edge between two named nodes or positions.
#let g-edge(
  from,
  to,
  stroke: 1pt + black,
) = {
  draw.line(from, to, stroke: stroke)
}

// ── Pre-defined graph layouts ──────────────────────────────────
// Each returns (vertices: [...], edges: [...])

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
// Extract positions from JSON, draw with dense styling via g-node/g-edge.

// King's subgraph from JSON with weight-based coloring
#let draw-grid-graph(data, cell-size: 0.2) = canvas(length: 1cm, {
  let positions = data.nodes.map(n => (n.col * cell-size, -n.row * cell-size))
  let fills = data.nodes.map(n => weight-color(n.weight))
  let edges = data.edges.map(e => (e.at(0), e.at(1)))
  for (u, v) in edges { g-edge(positions.at(u), positions.at(v), stroke: 0.4pt + gray) }
  for (k, pos) in positions.enumerate() {
    g-node(pos, radius: 0.04, stroke: none, fill: fills.at(k))
  }
})

// Triangular lattice from JSON with weight-based coloring
// Physical positions use triangular lattice transform (offset_even_cols=true)
#let draw-triangular-graph(data, cell-size: 0.2) = canvas(length: 1cm, {
  let sqrt3_2 = calc.sqrt(3) / 2
  let positions = data.nodes.map(n => {
    let offset = if calc.rem(n.col, 2) == 0 { 0.5 } else { 0.0 }
    ((n.row + offset) * cell-size, -n.col * sqrt3_2 * cell-size)
  })
  let fills = data.nodes.map(n => weight-color(n.weight))
  let edges = data.edges.map(e => (e.at(0), e.at(1)))
  for (u, v) in edges { g-edge(positions.at(u), positions.at(v), stroke: 0.3pt + gray) }
  for (k, pos) in positions.enumerate() {
    g-node(pos, radius: 0.025, stroke: none, fill: fills.at(k))
  }
})
