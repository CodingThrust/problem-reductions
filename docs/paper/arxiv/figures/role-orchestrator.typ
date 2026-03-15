#import "lib.typ": *
#import "@preview/pixel-family:0.1.0": bob, nova

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

  let gap = 4.5

  // Maintainer (left)
  content((0, 1.4), bob(size: 1.8em, baseline: 0pt))
  node((0, 0), [Maintainer], "maint")

  // Board
  node((gap, 0), [Board], "board")

  // Orchestrator (agent)
  content((2 * gap, 1.4), nova(size: 1.8em, baseline: 0pt))
  node((2 * gap, 0), [Orchestrator], "orch", accented: true)

  // PR
  node((3 * gap, 0), [PR], "pr")

  // Maintainer (right)
  content((4 * gap, 1.4), bob(size: 1.8em, baseline: 0pt))
  node((4 * gap, 0), [Maintainer], "maint2")

  // Edges
  line("maint.east", "board.west", stroke: stroke-edge, mark: arrow-end)
  elabel(("maint", 50%, "board"), [ready])

  line("board.east", "orch.west", stroke: stroke-edge, mark: arrow-end)
  elabel(("board", 50%, "orch"), [pick])

  line("orch.east", "pr.west", stroke: stroke-edge, mark: arrow-end)
  elabel(("orch", 50%, "pr"), [create])

  line("pr.east", "maint2.west", stroke: stroke-edge, mark: arrow-end)
  elabel(("pr", 50%, "maint2"), [merge])
})
