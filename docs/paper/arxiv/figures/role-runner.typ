#import "lib.typ": *
#import "@preview/pixel-family:0.1.0": crank

#set page(..fig-page)
#set text(..fig-text)

#canvas(length: 0.55cm, {
  import draw: *

  // Helper: auto-sized rounded box
  let node(pos, label, name-id, accented: false) = {
    let s = if accented { stroke-accent } else { stroke-box }
    let f = if accented { fill-accent } else { fill-light }
    let c = if accented { accent.darken(20%) } else { fg }
    content(pos,
      box(fill: f, stroke: s, inset: (x: 8pt, y: 4pt), radius: 5pt,
        text(8pt, weight: "bold", fill: c, label)),
      name: name-id)
  }

  // Runner (agent, top center)
  content((3, 2.1), crank(size: 1.8em, baseline: 0pt))
  node((3, 0.7), [Runner], "runner", accented: true)

  // Code + Tests (bottom left)
  node((0.5, -0.7), [Code + Tests], "code")

  // Paper Entry (bottom right)
  node((5.5, -0.7), [Paper Entry], "paper")

  // Edges
  line("runner.south-west", "code.north-east",
    stroke: stroke-edge, mark: arrow-end)
  line("runner.south-east", "paper.north-west",
    stroke: stroke-edge, mark: arrow-end)
})
