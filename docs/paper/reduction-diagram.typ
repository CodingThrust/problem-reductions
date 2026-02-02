#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#import "@preview/cetz:0.4.2": canvas, draw

#let graph-data = json("reduction_graph.json")

#let category-colors = (
  "graph": rgb("#e0ffe0"),
  "set": rgb("#ffe0e0"),
  "optimization": rgb("#ffffd0"),
  "satisfiability": rgb("#e0e0ff"),
  "specialized": rgb("#ffe0f0"),
  "other": rgb("#f0f0f0"),
)

#let get-color(category) = {
  category-colors.at(category, default: rgb("#f0f0f0"))
}

// Optimized layout: SAT branch (left) + Physics branch (right)
// Node IDs use base names without type parameters
#let node-positions = (
  // Row 0: Root nodes
  "Satisfiability": (-1.5, 0),
  "Factoring": (2.5, 0),
  // Row 1: Direct children of roots
  "KSatisfiability": (-2.5, 1),
  "IndependentSet": (-0.5, 1),
  "Coloring": (0.5, 1),
  "DominatingSet": (-1.5, 1),
  "CircuitSAT": (2.5, 1),
  // Row 2: Next level
  "VertexCovering": (-0.5, 2),
  "Matching": (-2, 2),
  "SpinGlass": (2.5, 2),
  "ILP": (3.5, 1),
  // Row 3: Leaf nodes
  "SetPacking": (-1.5, 3),
  "SetCovering": (0.5, 3),
  "MaxCut": (1.5, 3),
  "QUBO": (3.5, 3),
  "GridGraph": (0.5, 2),
)


#let reduction-graph(width: 18mm, height: 14mm) = diagram(
  spacing: (width, height),
  node-stroke: 0.6pt,
  edge-stroke: 0.6pt,
  node-corner-radius: 2pt,
  node-inset: 3pt,
  ..graph-data.nodes.map(n => {
    let color = get-color(n.category)
    let pos = node-positions.at(n.id)
    node(pos, text(size: 7pt)[#n.label], fill: color, name: label(n.id))
  }),
  ..graph-data.edges.map(e => {
    let arrow = if e.bidirectional { "<|-|>" } else { "-|>" }
    edge(label(e.source), label(e.target), arrow)
  }),
)