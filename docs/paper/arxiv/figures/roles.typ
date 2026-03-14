#import "@preview/cetz:0.4.2": canvas, draw

#set page(width: auto, height: auto, margin: 10pt)
#set text(size: 7pt, font: "New Computer Modern")

#let col-human = rgb("#f28e2b")
#let col-agent = rgb("#4e79a7")
#let col-code  = rgb("#59a14f")
#let col-skill = rgb("#9c755f")

#canvas(length: 0.55cm, {
  import draw: *

  // Helper: role node with shadow
  let node(pos, label, sub, col, name-id, w: 2.2, h: 0.8) = {
    let (x, y) = pos
    rect((x - w + 0.12, y - h + 0.12), (x + w + 0.12, y + h + 0.12),
      radius: 7pt, fill: luma(230), stroke: none)
    rect((x - w, y - h), (x + w, y + h),
      radius: 7pt, fill: col.lighten(90%), stroke: (thickness: 1.3pt, paint: col),
      name: name-id)
    content((x, y + 0.22), text(10pt, weight: "bold", fill: col.darken(22%), label))
    content((x, y - 0.32), text(6.5pt, fill: col.darken(8%), sub))
  }

  // Helper: edge label with white backing
  let elabel(pos, body) = {
    content(pos, box(fill: white, inset: (x: 3pt, y: 1.5pt), radius: 2pt, body))
  }

  let cx = 8
  let cy = 5.5

  // ── Codebase (center, larger) ──
  rect((cx - 2.7 + 0.12, cy - 1.4 + 0.12), (cx + 2.7 + 0.12, cy + 1.4 + 0.12),
    radius: 8pt, fill: luma(225), stroke: none)
  rect((cx - 2.7, cy - 1.4), (cx + 2.7, cy + 1.4),
    radius: 8pt, fill: col-code.lighten(92%), stroke: (thickness: 1.5pt, paint: col-code),
    name: "code")
  content((cx, cy + 0.45), text(11pt, weight: "bold", fill: col-code.darken(25%), [Codebase]))
  content((cx, cy - 0.3), text(7pt, style: "italic", fill: col-code.darken(8%), [agent-maintained]))

  // ── Three roles ──
  node((3.0, 11.0), [Contributor], [domain expert], col-human, "contrib")
  node((3.0, 0.8), [Maintainer], [no code], col-human, "maint")
  node((13.5, 2.0), [Agent], [implement · test · review], col-agent, "agent", w: 2.5)

  // ── Contributor → Codebase: issue ──
  line((5.2, 11.0 - 0.8), (cx - 0.5, cy + 1.4),
    stroke: (thickness: 1.1pt, paint: col-human),
    mark: (end: "straight", scale: 0.42))
  elabel((6.8, 8.8), text(6.5pt, fill: col-human.darken(15%), [issue (creative elements)]))

  // ── Codebase → Contributor: visual check ──
  line((cx - 2.0, cy + 1.4), (2.2, 11.0 - 0.8),
    stroke: (thickness: 0.9pt, paint: col-code, dash: "densely-dashed"),
    mark: (end: "straight", scale: 0.38))
  elabel((2.5, 8.6), text(6pt, fill: col-code.darken(15%), [generated paper\ (visual check)]))

  // ── Maintainer → Codebase: approve, merge ──
  line((4.5, 0.8 + 0.8), (cx - 2.0, cy - 1.4),
    stroke: (thickness: 0.9pt, paint: col-human),
    mark: (end: "straight", scale: 0.38))
  elabel((3.8, 3.0), text(6pt, fill: col-human.darken(15%), [approve, merge]))

  // ── Agent ↔ Codebase: execute skills ──
  line((13.5 - 2.3, 2.0 + 0.8), (cx + 2.0, cy - 1.4),
    stroke: (thickness: 1.1pt, paint: col-agent),
    mark: (start: "straight", end: "straight", scale: 0.42))
  elabel((12.0, 4.2), text(6pt, fill: col-agent.darken(15%), [execute skills]))

  // ── Maintainer → Agent: author skills ──
  line((3.0 + 2.2, 0.8 + 0.2), (13.5 - 2.5, 2.0 - 0.3),
    stroke: (thickness: 1.1pt, paint: col-skill),
    mark: (end: "straight", scale: 0.42))
  elabel((8.2, 0.4), text(7pt, weight: "bold", fill: col-skill.darken(15%), [author skills]))

  // ── Maintainer ↔ Contributor: community calls ──
  line((3.0 - 1.0, 0.8 + 0.8), (3.0 - 1.0, 11.0 - 0.8),
    stroke: (thickness: 0.7pt, paint: col-human.lighten(25%), dash: "dashed"),
    mark: (start: "straight", end: "straight", scale: 0.28))
  elabel((0.3, 5.9), text(5.5pt, fill: col-human.lighten(5%), [community\ calls]))
})
