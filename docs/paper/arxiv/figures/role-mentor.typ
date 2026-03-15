#import "lib.typ": *
#import "@preview/pixel-family:0.1.0": alice, bolt

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

  let elabel(pos, body) = {
    content(pos, box(fill: white, inset: (x: 3pt, y: 1.5pt), radius: 2pt,
      text(6pt, fill: fg-light, body)))
  }

  // Contributor (human)
  content((0, 1.4), alice(size: 1.8em, baseline: 0pt))
  node((0, 0), [Contributor], "contrib")

  // Mentor (agent)
  content((6, 1.4), bolt(size: 1.8em, baseline: 0pt))
  node((6, 0), [Mentor], "mentor", accented: true)

  // GitHub Issue
  node((3, -2.2), [GitHub Issue], "issue")

  // Contributor ↔ Mentor
  line("contrib.east", "mentor.west",
    stroke: stroke-edge, mark: arrow-both)
  elabel(("contrib", 50%, "mentor"), [interactive])

  // Mentor → Issue
  line("mentor.south", "issue.east",
    stroke: stroke-edge, mark: arrow-end)
})
