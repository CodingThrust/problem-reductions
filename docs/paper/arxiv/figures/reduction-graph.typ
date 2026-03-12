#import "@preview/cetz:0.4.2": canvas, draw

#set page(width: auto, height: auto, margin: 10pt)
#set text(size: 7pt, font: "New Computer Modern")

// Category colors
#let col-graph   = rgb("#4e79a7")
#let col-formula = rgb("#59a14f")
#let col-set     = rgb("#e15759")
#let col-alg     = rgb("#b07aa1")
#let col-misc    = rgb("#999999")

#let hub-r = 0.44
#let node-r = 0.26

// Layout: Semi-circular fan around ILP (the biggest sink with 10 in-edges).
// ILP at center-right; QUBO below it; MIS at center-left.
// Feeder nodes arranged in an arc around ILP so edges don't bundle.
// Bottom: SG <-> MaxCut <-> QUBO cluster.
// Left: SAT cluster fans out to MIS, MinDS, KCol, kSAT, CSAT.
// Isolated nodes in dashed boxes at far left and far right.

#let nodes = (
  // === Hubs ===
  ("MIS",      "MIS",       3.0,   5.0,  col-graph,  hub-r),
  ("ILP",      "ILP",      12.0,   5.0,  col-alg,    hub-r),
  ("QUBO",     "QUBO",     12.0,   1.5,  col-alg,    hub-r),

  // === ILP feeders — spread in a wide arc above/below/left of ILP ===
  ("MaxClq",   "MaxClq",    8.5,   8.5,  col-graph,  node-r),
  ("TSP",      "TSP",      10.0,   8.5,  col-graph,  node-r),
  ("MaxMatch", "MaxM",      5.0,   8.0,  col-graph,  node-r),
  ("MinDS",    "MinDS",     1.0,   7.0,  col-graph,  node-r),

  // === MIS neighbors ===
  ("MinVC",    "MinVC",     1.0,   3.5,  col-graph,  node-r),

  // === Middle band ===
  ("KCol",     "KCol",      8.5,   3.5,  col-graph,  node-r),
  ("SG",       "SG",       10.0,   0.0,  col-graph,  node-r),
  ("MaxCut",   "MaxCut",    8.0,   0.0,  col-graph,  node-r),

  // === Isolated graph ===
  ("MaxIS",    "MaxIS",    -0.8,   5.0,  col-graph,  node-r),
  ("BiClq",    "BiClq",    -0.8,   3.5,  col-graph,  node-r),

  // === Formula ===
  ("SAT",      "SAT",       5.0,   5.0,  col-formula, node-r),
  ("kSAT",     "kSAT",      7.5,   5.0,  col-formula, node-r),
  ("CSAT",     "CSAT",      7.5,   1.5,  col-formula, node-r),

  // === Set ===
  ("MaxSP",    "MaxSP",     5.0,   6.5,  col-set,    node-r),
  ("MinSC",    "MinSC",    10.0,   3.5,  col-set,    node-r),

  // === Isolated algebraic ===
  ("CVP",      "CVP",      14.5,   3.5,  col-alg,    node-r),
  ("BMF",      "BMF",      14.5,   1.5,  col-alg,    node-r),
  ("Knap",     "Knap",     14.5,   5.5,  col-alg,    node-r),

  // === Misc ===
  ("Fact",     "Fact",       5.0,   0.0,  col-misc,   node-r),
  ("BinP",     "BinP",     14.5,   7.5,  col-misc,   node-r),
  ("PS",       "PS",       14.5,   0.0,  col-misc,   node-r),
)

// 32 unique type-level directed edges
#let edges = (
  ("SAT",   "CSAT"),    ("SAT",   "KCol"),    ("SAT",   "kSAT"),
  ("SAT",   "MIS"),     ("SAT",   "MinDS"),
  ("kSAT",  "QUBO"),    ("kSAT",  "SAT"),
  ("CSAT",  "ILP"),     ("CSAT",  "SG"),
  ("Fact",  "CSAT"),    ("Fact",  "ILP"),
  ("MIS",   "MaxSP"),   ("MIS",   "MinVC"),
  ("MinVC", "MIS"),     ("MinVC", "MinSC"),
  ("MaxSP", "ILP"),     ("MaxSP", "MIS"),     ("MaxSP", "QUBO"),
  ("MaxClq","ILP"),
  ("MaxMatch","ILP"),   ("MaxMatch","MaxSP"),
  ("MinDS", "ILP"),
  ("MinSC", "ILP"),
  ("KCol",  "ILP"),     ("KCol",  "QUBO"),
  ("QUBO",  "ILP"),     ("QUBO",  "SG"),
  ("SG",    "MaxCut"),  ("SG",    "QUBO"),
  ("MaxCut","SG"),
  ("ILP",   "QUBO"),
  ("TSP",   "ILP"),
)

// Bidirectional pairs for perpendicular offset
#let bidi-pairs = (
  ("MIS", "MinVC"),  ("MIS", "MaxSP"),  ("SAT", "kSAT"),
  ("SG",  "MaxCut"), ("SG",  "QUBO"),   ("ILP", "QUBO"),
)

#canvas(length: 0.55cm, {
  import draw: *

  // Build lookup
  let node-map = (:)
  for n in nodes {
    let (id, abbr, x, y, col, r) = n
    node-map.insert(id, (x: x, y: y, r: r))
  }

  let is-bidi(src, tgt) = {
    let found = false
    for (a, b) in bidi-pairs {
      if (src == a and tgt == b) or (src == b and tgt == a) { found = true }
    }
    found
  }

  let bidi-offset = 0.2

  // --- Edges ---
  for (src, tgt) in edges {
    let s = node-map.at(src)
    let t = node-map.at(tgt)
    let dx = t.x - s.x
    let dy = t.y - s.y
    let dist = calc.sqrt(dx * dx + dy * dy)
    if dist > 0 {
      let ux = dx / dist
      let uy = dy / dist
      let px = -uy
      let py = ux
      let off = 0.0
      if is-bidi(src, tgt) {
        if src < tgt { off = bidi-offset } else { off = -bidi-offset }
      }
      let sx = s.x + px * off
      let sy = s.y + py * off
      let tx = t.x + px * off
      let ty = t.y + py * off
      let x1 = sx + ux * (s.r + 0.06)
      let y1 = sy + uy * (s.r + 0.06)
      let x2 = tx - ux * (t.r + 0.1)
      let y2 = ty - uy * (t.r + 0.1)
      line(
        (x1, y1), (x2, y2),
        stroke: 0.35pt + luma(150),
        mark: (end: "straight", scale: 0.3),
      )
    }
  }

  // --- Nodes ---
  for n in nodes {
    let (id, abbr, x, y, col, r) = n
    let is-hub = r > 0.3
    circle(
      (x, y), radius: r,
      fill: col.lighten(if is-hub { 58% } else { 80% }),
      stroke: (thickness: if is-hub { 1.4pt } else { 0.5pt }, paint: col),
      name: id,
    )
    content(id, text(
      if is-hub { 7.5pt } else { 5.5pt },
      weight: if is-hub { "bold" } else { "regular" },
      fill: col.darken(25%), abbr,
    ))
  }

  // --- Dashed boxes for isolated nodes ---
  rect((-1.4, 2.9), (0.0, 5.7),
    stroke: (thickness: 0.3pt, paint: luma(190), dash: "dashed"), radius: 4pt)
  content((-0.7, 2.55), text(4pt, fill: luma(150), "no reductions"))

  rect((13.85, -0.6), (15.2, 8.1),
    stroke: (thickness: 0.3pt, paint: luma(190), dash: "dashed"), radius: 4pt)
  content((14.5, -0.9), text(4pt, fill: luma(150), "no reductions"))

  // --- Legend ---
  let lx = 1.0
  let ly = -1.3
  rect((lx - 0.3, ly - 0.2), (lx + 10.0, ly + 0.85),
    stroke: 0.3pt + luma(180), fill: white, radius: 3pt)
  let items = (
    ("Graph", col-graph), ("Formula", col-formula), ("Set", col-set),
    ("Algebraic", col-alg), ("Misc", col-misc),
  )
  for (i, (label, col)) in items.enumerate() {
    let ex = lx + 0.25 + i * 2.0
    let ey = ly + 0.33
    circle((ex, ey), radius: 0.15, fill: col.lighten(80%), stroke: 0.5pt + col)
    content((ex + 0.3, ey), anchor: "west", text(5pt, label))
  }
})
