#import "@preview/cetz:0.4.2": canvas, draw

#set page(width: auto, height: auto, margin: 8pt)
#set text(size: 8pt, font: "New Computer Modern")

// Color palette (matching NSFC figure's feel, adapted for English paper)
#let col-platform = rgb("#1A5276")
#let col-platform-fill = rgb("#D4E6F1")
#let col-human = rgb("#1E8449")
#let col-human-fill = rgb("#D5F5E3")
#let col-ai = rgb("#7D3C98")
#let col-ai-fill = rgb("#E8DAEF")
#let col-edge = rgb("#5D6D7E")
#let col-dash = rgb("#ABB2B9")

#canvas(length: 0.5cm, {
  import draw: *

  // ============================================================
  //  LEVEL 0 — Hardware native problems (solver backends)
  // ============================================================
  let plat-w = 3.4
  let plat-h = 0.9

  rect((-4.5, -0.45), (-4.5 + plat-w, 0.45),
    fill: col-platform-fill, stroke: 1.2pt + col-platform, radius: 4pt, name: "udmis")
  content("udmis", text(9pt, weight: "bold", fill: col-platform, "UD-MIS on grids"))
  content((-2.8, -0.85), text(6.5pt, fill: col-platform.lighten(20%), "(Rydberg atom arrays)"))

  rect((1.1, -0.45), (1.1 + plat-w, 0.45),
    fill: col-platform-fill, stroke: 1.2pt + col-platform, radius: 4pt, name: "qubo")
  content("qubo", text(9pt, weight: "bold", fill: col-platform, "QUBO"))
  content((2.8, -0.85), text(6.5pt, fill: col-platform.lighten(20%), "(D-Wave annealers)"))

  // Also show ILP as a third backend
  rect((6.5, -0.45), (6.5 + plat-w, 0.45),
    fill: col-platform-fill, stroke: 1.2pt + col-platform, radius: 4pt, name: "ilp")
  content("ilp", text(9pt, weight: "bold", fill: col-platform, "ILP"))
  content((8.2, -0.85), text(6.5pt, fill: col-platform.lighten(20%), "(Gurobi, CPLEX)"))

  // ============================================================
  //  LEVEL 1 — Human-implemented reductions (~40 rules)
  // ============================================================
  let hnode(pos, label, name: none) = {
    let n = if name != none { name } else { label }
    rect(
      (pos.at(0) - 0.9, pos.at(1) - 0.28),
      (pos.at(0) + 0.9, pos.at(1) + 0.28),
      fill: col-human-fill, stroke: 0.7pt + col-human.lighten(20%),
      radius: 3pt, name: n,
    )
    content(n, text(6.5pt, weight: "bold", fill: col-human.darken(30%), label))
  }

  // Left subtree → UD-MIS
  hnode((-6.0, 2.0), "Set Packing", name: "sp")
  hnode((-4.8, 3.2), "Vertex Cover", name: "vc")
  hnode((-3.4, 1.7), "MIS", name: "mis")
  hnode((-1.6, 3.2), "3-SAT", name: "sat")
  hnode((-0.4, 2.2), "Clique", name: "clq")

  // Middle/Right subtree → QUBO / ILP
  hnode((1.0, 1.7), "MAX-CUT", name: "mc")
  hnode((2.8, 3.4), "Graph Coloring", name: "gc")
  hnode((3.6, 2.0), "Set Cover", name: "sc")
  hnode((5.8, 3.0), "Hamilton Cycle", name: "hc")
  hnode((7.2, 1.9), "Num. Partition", name: "np")
  hnode((8.8, 3.2), "Factoring", name: "fac")

  // Reduction edges (downward = reduction direction)
  let redge(from, to) = {
    line(from, to,
      stroke: (paint: col-edge, thickness: 0.5pt),
      mark: (end: "straight", scale: 0.35))
  }

  redge("mis.south", "udmis.north")
  redge("clq.south", "udmis.north")
  redge("sp.south", "udmis.north")
  redge("sat.south", "mis.north")
  redge("vc.south", "mis.north")

  redge("mc.south", "qubo.north")
  redge("gc.south", "qubo.north")
  redge("sc.south", "qubo.north")
  redge("hc.south", "qubo.north")
  redge("np.south", "ilp.north")
  redge("fac.south", "ilp.north")

  // Cross-reductions
  let dredge(from, to) = {
    line(from, to,
      stroke: (paint: col-edge.lighten(30%), thickness: 0.4pt, dash: "densely-dashed"),
      mark: (end: "straight", scale: 0.3))
  }
  dredge("vc.south", "qubo.north")
  dredge("sat.south", "ilp.north")
  dredge("gc.south", "ilp.north")
  dredge("mc.south", "ilp.north")
  dredge("clq.south", "ilp.north")

  // ============================================================
  //  DIVIDER — boundary between current and AI-scaled
  // ============================================================
  line((-7.0, 4.1), (10.5, 4.1),
    stroke: (paint: col-dash, thickness: 0.7pt, dash: "dashed"))

  // ============================================================
  //  LEVEL 2 — AI-synthesized reductions (~100+ rules)
  //  Staggered dot grid forming a canopy shape
  // ============================================================
  let ai-dot(x, y) = {
    circle((x, y), radius: 0.08,
      fill: col-ai.lighten(60%), stroke: 0.2pt + col-ai.lighten(40%))
  }

  // Row 1 (y=4.6)
  for x in (-5.5, -4.0, -2.5, -1.0, 0.5, 2.0, 3.5, 5.0, 6.5, 8.0) { ai-dot(x, 4.6) }
  // Row 2 (y=5.0)
  for x in (-6.2, -4.7, -3.2, -1.7, -0.2, 1.3, 2.8, 4.3, 5.8, 7.3, 8.8) { ai-dot(x, 5.0) }
  // Row 3 (y=5.4)
  for x in (-6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, 7.5, 8.5) { ai-dot(x, 5.4) }
  // Row 4 (y=5.8)
  for x in (-5.5, -4.0, -2.5, -1.0, 0.5, 2.0, 3.5, 5.0, 6.5, 8.0) { ai-dot(x, 5.8) }
  // Row 5 (y=6.2)
  for x in (-4.8, -3.3, -1.8, -0.3, 1.2, 2.7, 4.2, 5.7, 7.2) { ai-dot(x, 6.2) }
  // Row 6 (y=6.6)
  for x in (-3.8, -2.3, -0.8, 0.7, 2.2, 3.7, 5.2, 6.5) { ai-dot(x, 6.6) }
  // Row 7 (y=7.0)
  for x in (-2.5, -1.0, 0.5, 2.0, 3.5, 5.0) { ai-dot(x, 7.0) }
  // Row 8 (y=7.4)
  for x in (-1.0, 0.5, 2.0, 3.5) { ai-dot(x, 7.4) }

  // Representative labeled problems in AI layer
  let ai-label(pos, label, n) = {
    rect(
      (pos.at(0) - 0.85, pos.at(1) - 0.2),
      (pos.at(0) + 0.85, pos.at(1) + 0.2),
      fill: col-ai-fill, stroke: 0.3pt + col-ai.lighten(30%),
      radius: 2pt, name: n,
    )
    content(n, text(5pt, weight: "bold", fill: col-ai.lighten(-20%), label))
  }

  ai-label((-5.2, 4.8), "Scheduling", "ai-sched")
  ai-label((7.5, 4.8), "TSP", "ai-tsp")
  ai-label((-2.5, 5.5), [$k$-SAT], "ai-ksat")
  ai-label((3.0, 5.5), "Steiner Tree", "ai-steiner")
  ai-label((0.5, 6.8), "Bin Packing", "ai-binp")
  ai-label((-4.0, 6.4), "Dom. Set", "ai-domset")
  ai-label((5.5, 6.4), [Max $k$-Cut], "ai-mkcut")

  // Faint edges from AI layer down to human layer
  let aedge(from, to) = {
    line(from, to,
      stroke: (paint: col-ai.lighten(50%), thickness: 0.3pt),
      mark: (end: "straight", scale: 0.2))
  }
  aedge((-4.0, 4.35), "sat.north")
  aedge((-1.0, 4.35), "vc.north")
  aedge((1.3, 4.35), "gc.north")
  aedge((2.8, 4.35), "mc.north")
  aedge((5.8, 4.35), "hc.north")
  aedge((8.0, 4.35), "np.north")

  // Ellipsis
  content((1.5, 7.7), text(12pt, fill: col-ai.lighten(30%), $dots.c$))

  // ============================================================
  //  ANNOTATIONS — right-side braces
  // ============================================================
  // Hardware native
  on-layer(-1, {
    // Use simple bracket lines instead of decorations
    let bx = 10.8

    // Hardware brace
    line((bx, -0.5), (bx + 0.3, -0.5), stroke: 0.6pt + col-platform.lighten(20%))
    line((bx + 0.3, -0.5), (bx + 0.3, 0.5), stroke: 0.6pt + col-platform.lighten(20%))
    line((bx, 0.5), (bx + 0.3, 0.5), stroke: 0.6pt + col-platform.lighten(20%))
    content((bx + 0.6, 0.0), anchor: "west", text(7pt, fill: col-platform, weight: "bold",
      [Hardware-native\ problems]))

    // Human brace
    line((bx, 1.0), (bx + 0.3, 1.0), stroke: 0.6pt + col-human.lighten(20%))
    line((bx + 0.3, 1.0), (bx + 0.3, 3.8), stroke: 0.6pt + col-human.lighten(20%))
    line((bx, 3.8), (bx + 0.3, 3.8), stroke: 0.6pt + col-human.lighten(20%))
    content((bx + 0.6, 2.4), anchor: "west", text(7pt, fill: col-human.darken(10%), weight: "bold",
      [Human-implemented\ #text(6pt)[$tilde.op$40 reduction rules]]))

    // AI brace
    line((bx, 4.3), (bx + 0.3, 4.3), stroke: 0.6pt + col-ai.lighten(20%))
    line((bx + 0.3, 4.3), (bx + 0.3, 7.5), stroke: 0.6pt + col-ai.lighten(20%))
    line((bx, 7.5), (bx + 0.3, 7.5), stroke: 0.6pt + col-ai.lighten(20%))
    content((bx + 0.6, 5.9), anchor: "west", text(7pt, fill: col-ai.darken(10%), weight: "bold",
      [Agent-synthesized\ #text(6pt)[$tilde.op$100+ new rules]]))
  })
})
