#import "@preview/cetz:0.4.2": canvas, draw

#set page(width: auto, height: auto, margin: 10pt)
#set text(size: 7.5pt, font: "New Computer Modern")

// ── Colors ──
#let col-human = rgb("#e15759")     // warm red for human team
#let col-agent = rgb("#59a14f")     // green for agent + verification
#let col-barrier = rgb("#f28e2b")   // orange for barrier bands
#let fg = luma(30)
#let fg-light = luma(100)
#let axis-col = luma(60)

#canvas(length: 0.55cm, {
  import draw: *

  // ── Layout constants ──
  let ox = 0        // origin x
  let oy = 0        // origin y
  let plot-w = 20.0 // plot width (in canvas units)
  let plot-h = 10.0 // plot height

  // Data scaling: x-axis 0..150, y-axis 0..1
  let x-max = 150.0
  let y-max = 1.0
  let sx = plot-w / x-max  // scale x
  let sy = plot-h / y-max  // scale y

  // Helpers: data coords -> canvas coords
  let px(x) = ox + x * sx
  let py(y) = oy + y * sy
  let pt(x, y) = (px(x), py(y))

  // ── Barrier bands (draw first, behind everything) ──
  let barriers = (
    (15, 22, "Convention\ndrift"),
    (32, 42, "Effort\nexhaustion"),
    (52, 65, "Knowledge\ndiscontinuity"),
  )

  for (x0, x1, label) in barriers {
    rect(
      pt(x0, 0), pt(x1, y-max),
      fill: col-barrier.lighten(82%),
      stroke: none,
    )
    // Thin border lines
    line(pt(x0, 0), pt(x0, y-max), stroke: (thickness: 0.4pt, paint: col-barrier.lighten(50%)))
    line(pt(x1, 0), pt(x1, y-max), stroke: (thickness: 0.4pt, paint: col-barrier.lighten(50%)))

    // Barrier label at top
    content(
      pt((x0 + x1) / 2, y-max + 0.06),
      anchor: "south",
      text(6pt, fill: col-barrier.darken(15%), weight: "bold", label),
    )
  }

  // ── Axes ──
  // X-axis
  line(
    pt(0, 0), pt(x-max, 0),
    stroke: (thickness: 1pt, paint: axis-col),
    mark: (end: "straight", scale: 0.35),
  )
  // Y-axis
  line(
    pt(0, 0), pt(0, y-max + 0.08),
    stroke: (thickness: 1pt, paint: axis-col),
    mark: (end: "straight", scale: 0.35),
  )

  // X-axis label
  content(
    pt(x-max / 2, -0.14),
    anchor: "north",
    text(8pt, fill: fg, [Number of problem types]),
  )

  // Y-axis label
  content(
    (ox - 1.8, py(y-max / 2)),
    anchor: "center",
    angle: 90deg,
    text(8pt, fill: fg, [Quality]),
  )

  // X-axis tick marks
  for x in (0, 20, 40, 60, 80, 100, 120, 140) {
    line(pt(x, 0), pt(x, -0.02), stroke: (thickness: 0.6pt, paint: axis-col))
    content(
      pt(x, -0.04),
      anchor: "north",
      text(6pt, fill: fg-light, str(x)),
    )
  }

  // Y-axis: just "Low" and "High" labels (no numeric scale)
  content(
    (ox - 0.4, py(0.05)),
    anchor: "east",
    text(6pt, fill: fg-light, [Low]),
  )
  content(
    (ox - 0.4, py(0.95)),
    anchor: "east",
    text(6pt, fill: fg-light, [High]),
  )

  // ── Human team line ──
  // Hobby spline for the steep descent; straight line for the flat tail.
  let human-pts = (
    (0, 0.92),
    (5, 0.93),
    (10, 0.91),
    (14, 0.88),
    // Hit first barrier: convention drift
    (18, 0.80),
    (20, 0.75),
    (24, 0.68),
    (28, 0.64),
    (30, 0.62),
    // Hit second barrier: effort exhaustion
    (35, 0.50),
    (40, 0.42),
    (45, 0.38),
    (50, 0.34),
    // Hit third barrier: knowledge discontinuity
    (55, 0.28),
    (60, 0.22),
    (65, 0.19),
    (75, 0.15),
    (85, 0.13),
  )

  let human-canvas = human-pts.map(((x, y)) => pt(x, y))
  hobby(
    ..human-canvas,
    stroke: (thickness: 1.8pt, paint: col-human),
  )
  // Flat tail: straight line from where the spline ends
  line(
    pt(85, 0.13), pt(145, 0.12),
    stroke: (thickness: 1.8pt, paint: col-human),
  )

  // ── Agent + verification line ──
  // Starts at same point, maintains quality throughout
  let agent-pts = (
    (0, 0.92),
    (5, 0.93),
    (10, 0.92),
    (15, 0.91),
    (20, 0.91),
    (25, 0.92),
    (27, 0.92),
    (30, 0.91),
    (35, 0.91),
    (40, 0.92),
    (45, 0.91),
    (50, 0.91),
    (55, 0.92),
    (60, 0.91),
    (65, 0.92),
    (70, 0.91),
    (80, 0.92),
    (90, 0.91),
    (100, 0.92),
  )

  let agent-canvas = agent-pts.map(((x, y)) => pt(x, y))
  hobby(
    ..agent-canvas,
    stroke: (thickness: 1.8pt, paint: col-agent),
  )

  // Dashed continuation of agent line beyond data
  line(
    pt(100, 0.92), pt(145, 0.91),
    stroke: (thickness: 1.4pt, paint: col-agent, dash: "dashed"),
  )

  // ── Data points ──

  // This work: x=27 on the agent line
  circle(
    pt(27, 0.92),
    radius: 0.2,
    fill: col-agent,
    stroke: (thickness: 1pt, paint: white),
    name: "this-pt",
  )
  // Label below-right to avoid overlapping with barrier labels at top
  content(
    (rel: (0.4, -0.5), to: "this-pt"),
    anchor: "north-west",
    frame: "rect",
    padding: (x: 0.12, y: 0.06),
    fill: white,
    stroke: (thickness: 0.5pt, paint: col-agent.lighten(40%)),
    text(6.5pt, fill: col-agent.darken(20%), weight: "bold", [This work (9 weeks)]),
  )

  // Vision arrow: from x=100 toward x=140
  line(
    pt(105, 0.80), pt(138, 0.80),
    stroke: (thickness: 1.2pt, paint: col-agent.darken(10%)),
    mark: (end: "straight", scale: 0.4),
    name: "vision-arrow",
  )
  content(
    "vision-arrow.mid",
    anchor: "south",
    padding: 0.12,
    text(7pt, fill: col-agent.darken(20%), weight: "bold", [Vision: 100+]),
  )

  // ── Legend ──
  let lx = px(80)
  let ly = py(0.42)
  let leg-gap = 1.1

  // Legend background (fully opaque to cover the human line behind it)
  rect(
    (lx - 0.4, ly + 0.6),
    (lx + 7.0, ly - leg-gap - 0.4),
    radius: 3pt,
    fill: white,
    stroke: (thickness: 0.5pt, paint: luma(200)),
  )

  // Human line legend
  line(
    (lx, ly), (lx + 1.2, ly),
    stroke: (thickness: 1.8pt, paint: col-human),
  )
  content(
    (lx + 1.5, ly),
    anchor: "west",
    text(6.5pt, fill: fg, [Human team]),
  )

  // Agent line legend
  line(
    (lx, ly - leg-gap), (lx + 1.2, ly - leg-gap),
    stroke: (thickness: 1.8pt, paint: col-agent),
  )
  content(
    (lx + 1.5, ly - leg-gap),
    anchor: "west",
    text(6.5pt, fill: fg, [Agent + verification]),
  )
})
