#import "lib.typ": *

#set page(..fig-page)
#set text(..fig-text)

#canvas(length: 0.55cm, {
  import draw: *

  // Helper: box with shadow
  let node(pos, label, sub, name-id, accented: false, w: 2.2, h: 0.8) = {
    let (x, y) = pos
    let s = if accented { stroke-accent } else { stroke-box }
    let f = if accented { fill-accent } else { fill-light }
    rect((x - w + 0.1, y - h + 0.1), (x + w + 0.1, y + h + 0.1),
      radius: 7pt, fill: shadow-col, stroke: none)
    rect((x - w, y - h), (x + w, y + h),
      radius: 7pt, fill: f, stroke: s, name: name-id)
    let c = if accented { accent.darken(20%) } else { fg }
    content((x, y + 0.22), text(10pt, weight: "bold", fill: c, label))
    content((x, y - 0.32), text(6.5pt, fill: fg-light, sub))
  }

  // Helper: edge label
  let elabel(pos, body) = {
    content(pos, box(fill: white, inset: (x: 3pt, y: 1.5pt), radius: 2pt,
      text(6.5pt, fill: fg-light, body)))
  }

  let cx = 8
  let cy = 5.5

  // ── Codebase (center, larger) ──
  rect((cx - 2.7 + 0.12, cy - 1.4 + 0.12), (cx + 2.7 + 0.12, cy + 1.4 + 0.12),
    radius: 8pt, fill: shadow-col, stroke: none)
  rect((cx - 2.7, cy - 1.4), (cx + 2.7, cy + 1.4),
    radius: 8pt, fill: fill-light, stroke: (thickness: 1.5pt, paint: border),
    name: "code")
  content((cx, cy + 0.45), text(11pt, weight: "bold", fill: fg, [Codebase]))
  content((cx, cy - 0.3), text(7pt, style: "italic", fill: fg-light, [agent-maintained]))

  // ── Three roles ──
  node((3.0, 11.0), [Contributor], [domain expert], "contrib")
  node((3.0, 0.8), [Maintainer], [no code], "maint")
  node((13.5, 2.0), [Agent], [implement · test · review], "agent", accented: true, w: 2.5)

  // ── Contributor → Codebase: issue ──
  line((5.2, 11.0 - 0.8), (cx - 0.5, cy + 1.4),
    stroke: stroke-edge, mark: arrow-end)
  elabel((6.8, 8.8), [issue (creative elements)])

  // ── Codebase → Contributor: visual check ──
  line((cx - 2.0, cy + 1.4), (2.2, 11.0 - 0.8),
    stroke: stroke-dotted, mark: arrow-end)
  elabel((2.2, 8.6), [generated paper\ (visual check)])

  // ── Maintainer → Codebase: approve, merge ──
  line((4.5, 0.8 + 0.8), (cx - 2.0, cy - 1.4),
    stroke: stroke-edge, mark: arrow-end)
  elabel((3.8, 3.0), [approve, merge])

  // ── Agent ↔ Codebase: execute skills ──
  line((13.5 - 2.3, 2.0 + 0.8), (cx + 2.0, cy - 1.4),
    stroke: (thickness: 1.1pt, paint: accent), mark: arrow-both)
  elabel((12.0, 4.2), text(fill: accent.darken(15%), [execute skills]))

  // ── Maintainer → Agent: author skills ──
  line((3.0 + 2.2, 0.8 + 0.2), (13.5 - 2.5, 2.0 - 0.3),
    stroke: (thickness: 1.1pt, paint: accent), mark: arrow-end)
  elabel((8.2, 0.35), text(weight: "bold", fill: accent.darken(15%), [author skills]))

  // ── Maintainer ↔ Contributor: community calls ──
  line((3.0 - 1.0, 0.8 + 0.8), (3.0 - 1.0, 11.0 - 0.8),
    stroke: stroke-dashed, mark: arrow-both)
  elabel((0.3, 5.9), [community\ calls])
})
