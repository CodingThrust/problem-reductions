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

// Build node ID from name + variant (new JSON format)
// Format: "Name" for base, "Name/graph/weight" for variants
#let build-node-id(n) = {
  if n.variant == (:) or n.variant.keys().len() == 0 {
    n.name
  } else {
    let parts = (n.name,)
    if "graph" in n.variant and n.variant.graph != "" and n.variant.graph != "SimpleGraph" {
      parts.push(n.variant.graph)
    }
    if "weight" in n.variant and n.variant.weight != "" and n.variant.weight != "Unweighted" {
      parts.push(n.variant.weight)
    }
    parts.join("/")
  }
}

// Build display label from name + variant
#let build-node-label(n) = {
  if n.variant == (:) or n.variant.keys().len() == 0 {
    n.name
  } else {
    // For variants, show abbreviated form
    let suffix = ()
    if "graph" in n.variant and n.variant.graph != "" and n.variant.graph != "SimpleGraph" {
      suffix.push(n.variant.graph)
    }
    if "weight" in n.variant and n.variant.weight != "" and n.variant.weight != "Unweighted" and n.variant.weight != "W" {
      suffix.push(n.variant.weight)
    }
    if suffix.len() > 0 {
      n.name + "/" + suffix.join("/")
    } else {
      n.name
    }
  }
}

// Check if node is a base problem (empty variant)
#let is-base-problem(n) = {
  n.variant == (:) or n.variant.keys().len() == 0
}

// Base problem positions
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

// Get position for a node
#let get-node-position(n) = {
  if is-base-problem(n) {
    // Base problem - use manual position
    base-positions.at(n.name, default: (0, 0))
  } else {
    // Variant - position below parent with horizontal offset
    let parent-pos = base-positions.at(n.name, default: (0, 0))
    // Find variant index among siblings with same base name
    let siblings = graph-data.nodes.filter(x => x.name == n.name and not is-base-problem(x))
    let idx = siblings.position(x => build-node-id(x) == build-node-id(n))
    let offset = if idx == none { 0 } else { idx * 0.4 }
    (parent-pos.at(0) + offset, parent-pos.at(1) + 0.5)
  }
}

// Filter to show only base problems in the main diagram
#let base-nodes = graph-data.nodes.filter(n => is-base-problem(n))

// Filter edges to only those between base problems
#let base-edges = graph-data.edges.filter(e => {
  let src-base = e.source.variant == (:) or e.source.variant.keys().len() == 0
  let tgt-base = e.target.variant == (:) or e.target.variant.keys().len() == 0
  src-base and tgt-base
})

#let reduction-graph(width: 18mm, height: 14mm) = diagram(
  spacing: (width, height),
  node-stroke: 0.6pt,
  edge-stroke: 0.6pt,
  node-corner-radius: 2pt,
  node-inset: 3pt,
  ..base-nodes.map(n => {
    let color = get-color(n.category)
    let pos = get-node-position(n)
    let node-label = build-node-label(n)
    let node-id = build-node-id(n)
    node(pos, text(size: 7pt)[#node-label], fill: color, name: label(node-id))
  }),
  ..base-edges.map(e => {
    let arrow = if e.bidirectional { "<|-|>" } else { "-|>" }
    let src-id = build-node-id(e.source)
    let tgt-id = build-node-id(e.target)
    edge(label(src-id), label(tgt-id), arrow)
  }),
)