// Logo for problem-reductions
// Compile: typst compile docs/logo.typ docs/logo.svg
#set page(width: auto, height: auto, margin: 8pt, fill: none)

#import "@preview/cetz:0.4.2": canvas, draw

// Color palette (derived from the reduction graph categories)
#let col-sat = rgb("#5c6bc0")         // indigo - satisfiability

#canvas(length: 2mm, {
  import draw: *

  // === Left shape: complex polygon (source problem) ===
  // An irregular hexagon representing complexity
  let left-cx = 12
  let left-cy = 15
  let left-pts = (
    (left-cx - 8, left-cy + 2),
    (left-cx - 3, left-cy + 9),
    (left-cx + 5, left-cy + 8),
    (left-cx + 9, left-cy + 3),
    (left-cx + 6, left-cy - 5),
    (left-cx - 2, left-cy - 7),
    (left-cx - 7, left-cy - 3),
  )

  // Fill the complex polygon
  line(..left-pts, close: true, fill: col-sat.lighten(70%), stroke: 3pt + col-sat)

  // Draw internal "graph edges" inside the left polygon to show complexity
  let inner-nodes = (
    (left-cx - 4, left-cy + 4),
    (left-cx + 3, left-cy + 5),
    (left-cx + 5, left-cy - 1),
    (left-cx - 1, left-cy - 3),
    (left-cx - 5, left-cy - 1),
  )

  // Edges between inner nodes (showing a complex graph)
  let inner-edges = (
    (0, 1), (1, 2), (2, 3), (3, 4), (4, 0),
  )

  for (i, pt) in inner-nodes.enumerate() {
    circle(pt, radius: 1.2, fill: col-sat, stroke: 1.2pt + white, name: "node-" + str(i))
  }

  for (i, j) in inner-edges {
    line("node-" + str(i), "node-" + str(j), stroke: 1.6pt + col-sat.lighten(20%), mark: (end: "straight"))
  }

  // === Text below ===
  content((12, 16), text(fill: col-sat.darken(20%), size: 24pt)[100])
  content((45, 16), text(fill: col-sat, size: 30pt)[Problem-Reductions])
})
