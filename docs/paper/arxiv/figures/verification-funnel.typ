#import "@preview/cetz:0.4.2": canvas, draw

#set page(width: auto, height: auto, margin: 5pt)
#set text(size: 7pt, font: "New Computer Modern")

// Filter layer data: (name, description)
#let filters = (
  ("Type system", "rejects structural errors"),
  ("Round-trip tests", "rejects semantic errors"),
  ("Overhead validation", "rejects incorrect complexity claims"),
  ("Agentic feature tests", "rejects usability issues"),
)

// Color palette: gradient from soft red (top) to green (bottom)
#let col-top = rgb("#d94f4f")     // red — many errors
#let col-bot = rgb("#4ea45e")     // green — correct
#let col-ground = rgb("#4e79a7")  // steel blue — ground truth

#let lerp-color(t) = {
  color.mix((col-top, (1 - t) * 100%), (col-bot, t * 100%))
}

#canvas(length: 0.55cm, {
  import draw: *

  let n = 4              // number of filter layers
  let layer-h = 1.3      // height of each filter layer
  let gap = 0.3          // gap between layers
  let max-w = 13.0       // width at top (agent output)
  let min-w = 5.0        // width at bottom (correct code)
  let cx = 0             // center x
  let cap-h = 1.3        // height of top/bottom cap regions
  let right-x = max-w / 2 + 0.8   // x for right-side descriptions

  // Compute total funnel geometry
  let funnel-top = cap-h + 0.6     // y where first filter starts
  let funnel-bot = -(n * (layer-h + gap) - gap) - 0.4  // y where last filter ends
  let total-h = funnel-top - funnel-bot

  // --- Top cap: "Agent output" ---
  let top-y = funnel-top + cap-h
  let top-w = max-w + 1.0

  // Wide entry region
  merge-path(
    close: true,
    fill: col-top.lighten(85%),
    stroke: (thickness: 0.8pt, paint: col-top.lighten(30%)),
    name: "top-cap",
    {
      line(
        (cx - top-w / 2, top-y),
        (cx + top-w / 2, top-y),
        (cx + max-w / 2, funnel-top),
        (cx - max-w / 2, funnel-top),
      )
    },
  )
  content(
    (cx, (top-y + funnel-top) / 2 + 0.15),
    anchor: "center",
    text(8.5pt, weight: "bold", fill: col-top.darken(20%), [Agent output]),
  )
  content(
    (cx, (top-y + funnel-top) / 2 - 0.4),
    anchor: "center",
    text(6.5pt, fill: col-top.darken(5%), style: "italic", [many candidate implementations]),
  )

  // --- Filter layers (narrowing from top to bottom) ---
  for i in range(n) {
    // t ranges from 0 (top filter) to 1 (bottom filter)
    let t-top = i / n
    let t-bot = (i + 1) / n

    // Widths: linear interpolation from max-w to min-w
    let w-top = max-w - (max-w - min-w) * t-top
    let w-bot = max-w - (max-w - min-w) * t-bot

    // Y coordinates (growing downward from funnel-top)
    let y-top = funnel-top - i * (layer-h + gap)
    let y-bot = y-top - layer-h
    let y-mid = (y-top + y-bot) / 2

    // Width at midpoint
    let t-mid = (i + 0.5) / n
    let w-mid = max-w - (max-w - min-w) * t-mid

    // Color for this layer
    let col = lerp-color(t-mid)
    let col-fill = col.lighten(75%)
    let col-stroke = col.darken(10%)
    let col-text = col.darken(35%)

    let name-id = "filter" + str(i)

    // Draw trapezoid
    merge-path(
      close: true,
      fill: col-fill,
      stroke: (thickness: 1pt, paint: col-stroke),
      name: name-id,
      {
        line(
          (cx - w-top / 2, y-top),
          (cx + w-top / 2, y-top),
          (cx + w-bot / 2, y-bot),
          (cx - w-bot / 2, y-bot),
        )
      },
    )

    let (mechanism, desc) = filters.at(i)

    // Mechanism label inside the trapezoid
    content(
      (cx, y-mid + 0.15),
      anchor: "center",
      text(8pt, weight: "bold", fill: col-text, mechanism),
    )

    // Description below the name
    content(
      (cx, y-mid - 0.35),
      anchor: "center",
      text(6.5pt, fill: col-text.lighten(20%), style: "italic", desc),
    )

    // Right-side connecting dotted line + filter icon
    let edge-x = cx + w-mid / 2
    line(
      (edge-x + 0.05, y-mid), (right-x - 0.15, y-mid),
      stroke: (thickness: 0.5pt, paint: col-stroke.lighten(40%), dash: "dotted"),
    )
    content(
      (right-x, y-mid),
      anchor: "west",
      text(6pt, fill: col-stroke, [#sym.times.o rejected]),
    )
  }

  // --- Bottom cap: "Correct code" ---
  let last-y-bot = funnel-top - (n - 1) * (layer-h + gap) - layer-h
  let bot-y = last-y-bot - 0.4
  let bot-cap-y = bot-y - cap-h
  let bot-w = min-w

  merge-path(
    close: true,
    fill: col-bot.lighten(80%),
    stroke: (thickness: 1pt, paint: col-bot.darken(10%)),
    name: "bot-cap",
    {
      line(
        (cx - bot-w / 2, bot-y),
        (cx + bot-w / 2, bot-y),
        (cx + bot-w / 2 - 0.8, bot-cap-y),
        (cx - bot-w / 2 + 0.8, bot-cap-y),
      )
    },
  )
  content(
    (cx, (bot-y + bot-cap-y) / 2 + 0.1),
    anchor: "center",
    text(8.5pt, weight: "bold", fill: col-bot.darken(30%), [Correct code]),
  )
  content(
    (cx, (bot-y + bot-cap-y) / 2 - 0.45),
    anchor: "center",
    text(6.5pt, fill: col-bot.darken(10%), style: "italic", [matches contributor ground truth]),
  )

  // --- Left side: "Contributor-specified ground truth" vertical arrow ---
  let gt-x = cx - max-w / 2 - 2.2
  let gt-top = funnel-top + cap-h * 0.5
  let gt-bot = bot-cap-y + 0.3

  // Main vertical arrow
  line(
    (gt-x, gt-top), (gt-x, gt-bot),
    stroke: (thickness: 1.4pt, paint: col-ground),
    mark: (end: "straight", scale: 0.5),
  )

  // Label for the vertical arrow
  content(
    (gt-x - 0.25, (gt-top + gt-bot) / 2),
    anchor: "east",
    angle: 90deg,
    text(7pt, weight: "bold", fill: col-ground.darken(10%),
      [Contributor-specified ground truth],
    ),
  )

  // Dashed arrows from ground-truth line into each filter layer
  for i in range(n) {
    let t-mid = (i + 0.5) / n
    let w-mid = max-w - (max-w - min-w) * t-mid

    let y-top = funnel-top - i * (layer-h + gap)
    let y-bot = y-top - layer-h
    let y-mid = (y-top + y-bot) / 2

    let target-x = cx - w-mid / 2

    line(
      (gt-x + 0.1, y-mid), (target-x - 0.1, y-mid),
      stroke: (thickness: 0.7pt, paint: col-ground.lighten(30%), dash: "dashed"),
      mark: (end: "straight", scale: 0.35),
    )
  }

  // Also connect to the bottom cap
  line(
    (gt-x + 0.1, (bot-y + bot-cap-y) / 2),
    (cx - bot-w / 2 + 0.3, (bot-y + bot-cap-y) / 2),
    stroke: (thickness: 0.7pt, paint: col-ground.lighten(30%), dash: "dashed"),
    mark: (end: "straight", scale: 0.35),
  )
})
