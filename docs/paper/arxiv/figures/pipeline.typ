#import "@preview/cetz:0.4.2": canvas, draw

#set page(width: auto, height: auto, margin: 5pt)
#set text(size: 8pt, font: "New Computer Modern")

// Color coding: human = orange, agent = blue
#let col-human = rgb("#f28e2b")
#let col-agent = rgb("#4e79a7")
#let col-bg-human = col-human.lighten(85%)
#let col-bg-agent = col-agent.lighten(85%)
#let col-neutral = luma(240)

// Column card style
#let card-w = 2.4
#let card-h = 0.7
#let gap-y = 1.3  // vertical spacing between cards

#canvas(length: 0.55cm, {
  import draw: *

  // --- Helper: draw a board column card ---
  let board-card(x, y, label, fill-col, stroke-col, name-id) = {
    rect(
      (x - card-w / 2, y - card-h / 2),
      (x + card-w / 2, y + card-h / 2),
      radius: 4pt,
      fill: fill-col,
      stroke: (thickness: 1pt, paint: stroke-col),
      name: name-id,
    )
    content(name-id, text(8pt, weight: "bold", fill: stroke-col.darken(15%), label))
  }

  // --- Layout: vertical pipeline ---
  let cx = 0  // center x for column cards
  let y0 = 0  // top

  // Contributor + Issue at top
  content((cx - 3.5, y0), anchor: "east", text(7pt, fill: luma(100), [Contributor]))
  rect(
    (cx - 3.2, y0 - 0.3), (cx - 1.8, y0 + 0.3),
    radius: 3pt, fill: col-neutral, stroke: 0.5pt + luma(180), name: "issue",
  )
  content("issue", text(7pt, [Issue]))
  line(
    (cx - 1.8, y0), (cx - card-w / 2 - 0.05, y0),
    stroke: 0.6pt + luma(150),
    mark: (end: "straight", scale: 0.35),
  )

  // Backlog
  board-card(cx, y0, "Backlog", col-neutral, luma(130), "backlog")

  // Arrow: Backlog -> Ready (human)
  let y1 = y0 - gap-y
  line(
    (cx, y0 - card-h / 2), (cx, y1 + card-h / 2 + 0.05),
    stroke: (thickness: 1.2pt, paint: col-human),
    mark: (end: "straight", scale: 0.4),
  )
  content(
    (cx + card-w / 2 + 0.2, (y0 + y1) / 2), anchor: "west",
    text(6.5pt, fill: col-human, [Maintainer\ moves card]),
  )

  // Ready
  board-card(cx, y1, "Ready", col-bg-human, col-human, "ready")

  // Arrow: Ready -> In Progress (agent: project-pipeline)
  let y2 = y1 - gap-y
  line(
    (cx, y1 - card-h / 2), (cx, y2 + card-h / 2 + 0.05),
    stroke: (thickness: 1.2pt, paint: col-agent),
    mark: (end: "straight", scale: 0.4),
  )
  content(
    (cx + card-w / 2 + 0.2, (y1 + y2) / 2), anchor: "west",
    text(6.5pt, fill: col-agent, [`project-pipeline`]),
  )

  // In Progress
  board-card(cx, y2, "In Progress", col-bg-agent, col-agent, "inprog")

  // Arrow: In Progress -> review-agentic (agent substeps)
  let y3 = y2 - gap-y
  line(
    (cx, y2 - card-h / 2), (cx, y3 + card-h / 2 + 0.05),
    stroke: (thickness: 1.2pt, paint: col-agent),
    mark: (end: "straight", scale: 0.4),
  )
  // Substep labels
  content(
    (cx + card-w / 2 + 0.2, (y2 + y3) / 2), anchor: "west",
    text(6pt, fill: col-agent, [`issue-to-pr` #sym.arrow `check` #sym.arrow `implement` #sym.arrow `review`]),
  )

  // review-agentic
  board-card(cx, y3, "review-agentic", col-bg-agent, col-agent, "rev-agent")

  // Arrow: review-agentic -> In Review (agent: review-pipeline)
  let y4 = y3 - gap-y
  line(
    (cx, y3 - card-h / 2), (cx, y4 + card-h / 2 + 0.05),
    stroke: (thickness: 1.2pt, paint: col-agent),
    mark: (end: "straight", scale: 0.4),
  )
  content(
    (cx + card-w / 2 + 0.2, (y3 + y4) / 2), anchor: "west",
    text(6.5pt, fill: col-agent, [`review-pipeline`]),
  )

  // In Review
  board-card(cx, y4, "In Review", col-bg-agent, col-agent, "inrev")

  // Arrow: In Review -> Done (human)
  let y5 = y4 - gap-y
  line(
    (cx, y4 - card-h / 2), (cx, y5 + card-h / 2 + 0.05),
    stroke: (thickness: 1.2pt, paint: col-human),
    mark: (end: "straight", scale: 0.4),
  )
  content(
    (cx + card-w / 2 + 0.2, (y4 + y5) / 2), anchor: "west",
    text(6.5pt, fill: col-human, [Maintainer\ merges PR]),
  )

  // Done
  board-card(cx, y5, "Done", col-bg-human, col-human, "done")

  // --- Bracket annotations on the left ---
  // Agent zone bracket (Ready -> In Review)
  let bx = cx - card-w / 2 - 0.6
  let bracket-top = y1 - card-h / 2 - 0.05
  let bracket-bot = y4 + card-h / 2 + 0.05
  line(
    (bx + 0.15, bracket-top), (bx, bracket-top), (bx, bracket-bot), (bx + 0.15, bracket-bot),
    stroke: (thickness: 0.8pt, paint: col-agent, dash: "dashed"),
  )
  content(
    (bx - 0.15, (bracket-top + bracket-bot) / 2), anchor: "east",
    text(6pt, fill: col-agent, weight: "bold", [Agent\ zone]),
  )

  // --- Legend at bottom ---
  let ly = y5 - card-h / 2 - 0.7
  let lx = cx - 2.5
  // Human
  line((lx, ly), (lx + 0.6, ly), stroke: (thickness: 1.2pt, paint: col-human))
  content((lx + 0.8, ly), anchor: "west", text(6pt, [Human decision]))
  // Agent
  line((lx + 3.2, ly), (lx + 3.8, ly), stroke: (thickness: 1.2pt, paint: col-agent))
  content((lx + 4.0, ly), anchor: "west", text(6pt, [Agent action]))
})
