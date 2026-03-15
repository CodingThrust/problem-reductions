#import "@preview/cetz:0.4.0": canvas, draw
#import "@preview/cetz-plot:0.1.2": plot

#set page(width: auto, height: auto, margin: 10pt)
#set text(size: 8pt, font: "New Computer Modern")

// --- Colors ---
#let col-models = rgb("#4e79a7")       // steel blue for problem types
#let col-rules  = rgb("#59a14f")       // green for reduction rules
// Phase background colors
#let phase1-fill = rgb("#4e79a7").lighten(92%)  // light blue
#let phase2-fill = rgb("#f0d060").lighten(70%)  // light yellow
#let phase3-fill = rgb("#59a14f").lighten(88%)  // light green

// --- Data ---
// Week numbers (Week 1 = Jan 9, 2026)
// Jan 10 = ~week 0.14, Jan 26 = week 2.43, Feb 15 = week 5.29, Mar 13 = week 9.0
#let models-data = ((0.14, 17), (2.43, 20), (5.29, 21), (9.0, 27))
#let rules-data  = ((0.14, 0),  (2.43, 22), (5.29, 44), (9.0, 45))

// Phase boundaries in weeks:
// Phase 1: Jan 9 - Feb 22 = weeks 0 to 6.29
// Phase 2: Feb 22 - Mar 1 = weeks 6.29 to 7.29
// Phase 3: Mar 1 - Mar 13 = weeks 7.29 to 9.0
#let phase1-end = 6.29
#let phase2-end = 7.29
#let week-max   = 9.5

#canvas(length: 0.6cm, {
  import draw: *

  plot.plot(
    size: (12, 7),
    x-label: [Weeks since project start (Jan 9, 2026)],
    y-label: [Cumulative count],
    x-min: 0, x-max: week-max,
    y-min: 0, y-max: 52,
    x-tick-step: 1,
    y-tick-step: 10,
    x-grid: "major",
    y-grid: "major",
    axis-style: "scientific",
    legend: "inner-north-west",
    legend-style: (
      stroke: 0.5pt + luma(200),
      fill: white,
      padding: 0.3,
    ),
    {
      // --- Phase background bands ---
      plot.add-fill-between(
        domain: (0, phase1-end),
        x => 52, x => 0,
        style: (stroke: none, fill: phase1-fill),
        label: none,
      )
      plot.add-fill-between(
        domain: (phase1-end, phase2-end),
        x => 52, x => 0,
        style: (stroke: none, fill: phase2-fill),
        label: none,
      )
      plot.add-fill-between(
        domain: (phase2-end, week-max),
        x => 52, x => 0,
        style: (stroke: none, fill: phase3-fill),
        label: none,
      )

      // --- Data lines ---
      // Problem types (solid blue)
      plot.add(
        models-data,
        mark: "o",
        mark-size: 0.15,
        line: "linear",
        style: (stroke: (paint: col-models, thickness: 1.6pt), fill: col-models),
        label: [Problem types],
      )

      // Reduction rules (dashed green)
      plot.add(
        rules-data,
        mark: "square",
        mark-size: 0.15,
        line: "linear",
        style: (stroke: (paint: col-rules, thickness: 1.6pt, dash: "dashed"), fill: col-rules),
        label: [Reduction rules],
      )

    },
  )
})
