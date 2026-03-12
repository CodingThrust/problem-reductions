#import "@preview/cetz:0.4.2": canvas, draw

#set page(width: auto, height: auto, margin: 5pt)
#set text(size: 7pt, font: "New Computer Modern")

// Layer data: (mechanism, error class caught)
#let layers = (
  ("Type system (Rust compiler)", "API misuse"),
  ("Unit tests (eval, serialization)", "evaluation errors"),
  ("Closed-loop tests (round-trip)", "mapping errors"),
  ("Overhead validation (symbolic exprs)", "formula errors"),
  ("Materialized fixtures (JSON ground truth)", "test gaming"),
  ("Agentic review (parallel subagents)", "convention violations"),
  ("Documentation (proof sketch)", "logical errors"),
)

// Color gradient: blue (automated, bottom) -> gold (human, top)
#let col-auto = rgb("#4e79a7")   // blue
#let col-human = rgb("#e8a838")  // gold

#let lerp-color(t) = {
  color.mix((col-auto, (1 - t) * 100%), (col-human, t * 100%))
}

#canvas(length: 0.55cm, {
  import draw: *

  let n = 7          // number of layers
  let layer-h = 1.1  // height of each layer
  let gap = 0.12     // gap between layers
  let max-w = 14.0   // width of bottom layer
  let min-w = 5.5    // width of top layer
  let cx = 0         // center x
  let right-col-x = max-w / 2 + 0.6  // x position for right-side labels

  // Draw layers from bottom to top
  for i in range(n) {
    let t-bot = i / n
    let t-top = (i + 1) / n

    // Widths: linear interpolation
    let w-bot = max-w - (max-w - min-w) * t-bot
    let w-top = max-w - (max-w - min-w) * t-top

    // Width at midpoint (for label positioning)
    let t-mid = (i + 0.5) / n
    let w-mid = max-w - (max-w - min-w) * t-mid

    // Y coordinates (layer 0 at bottom)
    let y-bot = i * (layer-h + gap)
    let y-top = y-bot + layer-h
    let y-mid = (y-bot + y-top) / 2

    // Color for this layer
    let col = lerp-color(t-bot)
    let col-fill = col.lighten(70%)
    let col-stroke = col.darken(10%)
    let col-text = col-stroke.darken(30%)

    let name-id = "layer" + str(i)

    // Draw trapezoid
    merge-path(
      close: true,
      fill: col-fill,
      stroke: (thickness: 0.8pt, paint: col-stroke),
      name: name-id,
      {
        line(
          (cx - w-bot / 2, y-bot),
          (cx + w-bot / 2, y-bot),
          (cx + w-top / 2, y-top),
          (cx - w-top / 2, y-top),
        )
      },
    )

    let (mechanism, catches) = layers.at(i)

    // Mechanism label centered inside the trapezoid
    content(
      (cx, y-mid),
      anchor: "center",
      text(7.5pt, weight: "bold", fill: col-text,
        [L#(i + 1): #mechanism],
      ),
    )

    // "catches:" label outside on the right, connected by a thin line
    let edge-x = cx + w-mid / 2  // right edge at midpoint height

    // Small connecting line from trapezoid edge to label
    line(
      (edge-x + 0.05, y-mid), (right-col-x - 0.15, y-mid),
      stroke: (thickness: 0.4pt, paint: col-stroke.lighten(30%), dash: "dotted"),
    )

    content(
      (right-col-x, y-mid),
      anchor: "west",
      text(6.5pt, fill: col-text.lighten(20%),
        [#sym.arrow.r #emph(catches)],
      ),
    )
  }

  // Side annotations
  let total-h = n * (layer-h + gap) - gap

  // Left bracket: "Automated" for bottom 4 layers (L1-L4)
  let bx-left = cx - max-w / 2 - 0.8
  let auto-top = 4 * (layer-h + gap) - gap
  line(
    (bx-left + 0.15, 0), (bx-left, 0), (bx-left, auto-top), (bx-left + 0.15, auto-top),
    stroke: (thickness: 0.7pt, paint: col-auto, dash: "dashed"),
  )
  content(
    (bx-left - 0.15, auto-top / 2), anchor: "east",
    text(6pt, fill: col-auto, weight: "bold", [Fully\ automated]),
  )

  // Left bracket: "Human-readable" for top 3 layers (L5-L7)
  let human-bot = 4 * (layer-h + gap)
  let human-top = total-h
  line(
    (bx-left + 0.15, human-bot), (bx-left, human-bot), (bx-left, human-top), (bx-left + 0.15, human-top),
    stroke: (thickness: 0.7pt, paint: col-human, dash: "dashed"),
  )
  content(
    (bx-left - 0.15, (human-bot + human-top) / 2), anchor: "east",
    text(6pt, fill: col-human.darken(10%), weight: "bold", [Human-\ readable]),
  )
})
