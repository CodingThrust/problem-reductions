#import "@preview/cetz:0.4.2": canvas, draw

#set page(width: auto, height: auto, margin: 8pt)
#set text(size: 7pt, font: "New Computer Modern")

// Colors
#let col-ok     = rgb("#4e79a7")   // healthy node
#let col-ok-fill = rgb("#d0ddef")
#let col-warn   = rgb("#e15759")   // problem highlight
#let col-warn-fill = rgb("#fce4e4")
#let col-sat    = rgb("#59a14f")   // 3-SAT / proof chain
#let col-sat-fill = rgb("#ddf0dd")
#let col-edge   = rgb("#5D6D7E")
#let col-redun  = rgb("#e8913a")   // redundant
#let col-ghost  = rgb("#cccccc")

#let node-r = 0.32

#canvas(length: 0.5cm, {
  import draw: *

  // Helper: draw a problem node
  let pnode(pos, label, col: col-ok, fill: col-ok-fill, name: none, r: node-r) = {
    let n = if name != none { name } else { label }
    circle(pos, radius: r, fill: fill, stroke: 0.8pt + col, name: n)
    content(n, text(6pt, weight: "bold", fill: col.darken(30%), label))
  }

  // Helper: draw a directed edge
  let edge(from, to, col: col-edge, thick: 0.5pt, dash: none) = {
    line(from, to,
      stroke: (paint: col, thickness: thick, dash: dash),
      mark: (end: "straight", scale: 0.35))
  }

  // ============================================================
  //  PANEL (a): Orphan node
  // ============================================================
  let ax = 0.0
  let ay = 0.0

  // Title
  content((ax + 2.5, ay + 4.8), text(8pt, weight: "bold", "(a) Orphan node"))

  // Connected subgraph
  pnode((ax + 0.5, ay + 3.5), "SAT", col: col-sat, fill: col-sat-fill, name: "a-sat")
  pnode((ax + 2.5, ay + 2.0), "MIS", name: "a-mis")
  pnode((ax + 0.5, ay + 0.5), "QUBO", name: "a-qubo")
  pnode((ax + 2.5, ay + 3.5), "MVC", name: "a-mvc")
  pnode((ax + 4.0, ay + 0.5), "ILP", name: "a-ilp")

  edge("a-sat.south", "a-mis.north")
  edge("a-mvc.south", "a-mis.north")
  edge("a-mis.south", "a-qubo.north")
  edge("a-mis.south", "a-ilp.north")

  // Orphan node — isolated, no edges
  pnode((ax + 5.0, ay + 3.0), "BMF", col: col-warn, fill: col-warn-fill, name: "a-orphan")

  // Dashed box around orphan
  rect((ax + 4.3, ay + 2.3), (ax + 5.7, ay + 3.7),
    stroke: (thickness: 0.6pt, paint: col-warn, dash: "dashed"), radius: 4pt)

  // Annotation
  content((ax + 5.0, ay + 1.8), text(5.5pt, fill: col-warn,
    [no reductions\ to or from]))

  // ============================================================
  //  PANEL (b): Redundant rule
  // ============================================================
  let bx = 8.0
  let by = 0.0

  content((bx + 2.5, by + 4.8), text(8pt, weight: "bold", "(b) Redundant rule"))

  // Three nodes in a row, with the composite path on top
  pnode((bx + 0.0, by + 3.5), "A", name: "b-a")
  pnode((bx + 2.5, by + 3.5), "B", name: "b-b")
  pnode((bx + 5.0, by + 3.5), "C", name: "b-c")

  // Good composite path: A → B → C (two hops, low overhead)
  edge("b-a.east", "b-b.west", col: col-ok, thick: 0.8pt)
  edge("b-b.east", "b-c.west", col: col-ok, thick: 0.8pt)

  // Cost labels on good path
  content((bx + 1.25, by + 4.1), text(5.5pt, fill: col-ok.darken(20%), $O(n)$))
  content((bx + 3.75, by + 4.1), text(5.5pt, fill: col-ok.darken(20%), $O(n m)$))

  // Redundant direct edge: A → C (higher overhead, curves below)
  bezier("b-a.south", "b-c.south",
    (bx + 1.5, by + 1.2), (bx + 3.5, by + 1.2),
    stroke: (paint: col-warn, thickness: 0.9pt, dash: "densely-dashed"),
    mark: (end: "straight", scale: 0.35))

  // Cost label on redundant edge
  content((bx + 2.5, by + 1.5), text(5.5pt, fill: col-warn,
    [direct: $O(n^2 m)$]))

  // Annotation
  content((bx + 2.5, by + 0.5), text(5.5pt, fill: col-warn,
    [composite $O(n^2 m)$ $lt.eq$ direct\ $arrow.r.double$ rule is dominated]))

  // ============================================================
  //  PANEL (c): Missing NP-hardness proof path
  // ============================================================
  let cx = 16.5
  let cy = 0.0

  content((cx + 2.5, cy + 4.8), text(8pt, weight: "bold", "(c) Missing proof path"))

  // 3-SAT as the NP-hardness source
  pnode((cx + 0.0, cy + 3.5), "3-SAT", col: col-sat, fill: col-sat-fill, name: "c-3sat")
  pnode((cx + 2.0, cy + 3.5), "SAT", col: col-sat, fill: col-sat-fill, name: "c-sat")
  pnode((cx + 4.0, cy + 3.5), "MIS", col: col-sat, fill: col-sat-fill, name: "c-mis")
  pnode((cx + 2.0, cy + 1.5), "ILP", col: col-sat, fill: col-sat-fill, name: "c-ilp")

  // Green proof chain
  edge("c-3sat.east", "c-sat.west", col: col-sat, thick: 0.7pt)
  edge("c-sat.east", "c-mis.west", col: col-sat, thick: 0.7pt)
  edge("c-sat.south", "c-ilp.north", col: col-sat, thick: 0.7pt)

  // Disconnected node — has edges but no path FROM 3-SAT
  pnode((cx + 5.5, cy + 1.5), "TSP", col: col-warn, fill: col-warn-fill, name: "c-tsp")
  pnode((cx + 5.5, cy + 3.5), "BinP", col: col-warn, fill: col-warn-fill, name: "c-binp")

  // TSP has outgoing edge to ILP, but no incoming from 3-SAT
  edge("c-tsp.west", "c-ilp.east", col: col-ghost)
  edge("c-binp.west", "c-mis.east", col: col-ghost)

  // Missing edges shown as dotted with "?"
  line((cx + 3.0, cy + 2.8), (cx + 5.0, cy + 1.8),
    stroke: (paint: col-warn, thickness: 0.5pt, dash: "dotted"),
    mark: (end: "straight", scale: 0.3))
  content((cx + 4.3, cy + 2.6), text(6pt, fill: col-warn, "?"))

  line((cx + 3.0, cy + 3.8), (cx + 5.0, cy + 3.8),
    stroke: (paint: col-warn, thickness: 0.5pt, dash: "dotted"),
    mark: (end: "straight", scale: 0.3))
  content((cx + 4.3, cy + 4.2), text(6pt, fill: col-warn, "?"))

  // Annotation
  content((cx + 5.5, cy + 0.5), text(5.5pt, fill: col-warn,
    [no path from 3-SAT\ $arrow.r.double$ NP-hardness\ unproven in graph]))
})
