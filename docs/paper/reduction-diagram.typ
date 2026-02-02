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

// Base problem positions (variants are auto-positioned below their parent)
#let base-positions = (
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

// Helper to check if a node has a parent (is a variant)
#let has-parent(n) = {
  "parent" in n and n.parent != none
}

// Count variants per parent for horizontal offset
#let variant-counts = {
  let counts = (:)
  for n in graph-data.nodes {
    if has-parent(n) {
      let parent = n.parent
      if parent in counts {
        counts.insert(parent, counts.at(parent) + 1)
      } else {
        counts.insert(parent, 1)
      }
    }
  }
  counts
}

// Get position for a node (base or variant)
#let get-node-position(n) = {
  if not has-parent(n) {
    // Base problem - use manual position
    base-positions.at(n.id, default: (0, 0))
  } else {
    // Variant - position below parent with horizontal offset
    let parent-pos = base-positions.at(n.parent, default: (0, 0))
    // Find variant index among siblings
    let siblings = graph-data.nodes.filter(x => has-parent(x) and x.parent == n.parent)
    let idx = siblings.position(x => x.id == n.id)
    let offset = if idx == none { 0 } else { idx * 0.4 }
    (parent-pos.at(0) + offset, parent-pos.at(1) + 0.5)
  }
}

#let reduction-graph(width: 18mm, height: 14mm) = diagram(
  spacing: (width, height),
  node-stroke: 0.6pt,
  edge-stroke: 0.6pt,
  node-corner-radius: 2pt,
  node-inset: 3pt,
  ..graph-data.nodes.map(n => {
    let color = get-color(n.category)
    let pos = get-node-position(n)
    // Smaller text for variant nodes
    let text-size = if not has-parent(n) { 7pt } else { 6pt }
    node(pos, text(size: text-size)[#n.label], fill: color, name: label(n.id))
  }),
  ..graph-data.edges.map(e => {
    let arrow = if e.bidirectional { "<|-|>" } else { "-|>" }
    edge(label(e.source), label(e.target), arrow)
  }),
)