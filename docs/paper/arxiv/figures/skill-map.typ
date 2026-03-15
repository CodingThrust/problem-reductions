#import "lib.typ": *

#set page(..fig-page)
#set text(..fig-text)

#canvas(length: 0.55cm, {
  import draw: *

  // Helper: core node (inner ring)
  let core(pos, label, name-id) = {
    let (x, y) = pos
    rect((x - 1.5, y - 0.35 + 0.06), (x + 1.5, y + 0.35 + 0.06),
      radius: 4pt, fill: shadow-col, stroke: none)
    rect((x - 1.5, y - 0.35), (x + 1.5, y + 0.35),
      radius: 4pt, fill: fill-light, stroke: stroke-box, name: name-id)
    content(name-id, text(7pt, weight: "bold", fill: fg, raw(label)))
  }

  // Helper: skill node (outer ring)
  let skill(pos, label, name-id) = {
    let (x, y) = pos
    rect((x - 1.6, y - 0.28), (x + 1.6, y + 0.28),
      radius: 3pt, fill: white, stroke: (thickness: 0.5pt, paint: border), name: name-id)
    content(name-id, text(6pt, fill: fg, raw(label)))
  }

  // Helper: category label
  let cat(pos, label) = {
    content(pos, text(6pt, weight: "bold", fill: fg-light, label))
  }

  let cx = 8
  let cy = 5.5

  // ── Center: CLAUDE.md ──
  rect((cx - 1.8 + 0.08, cy - 0.45 + 0.08), (cx + 1.8 + 0.08, cy + 0.45 + 0.08),
    radius: 5pt, fill: shadow-col, stroke: none)
  rect((cx - 1.8, cy - 0.45), (cx + 1.8, cy + 0.45),
    radius: 5pt, fill: fill-accent, stroke: stroke-accent, name: "claude")
  content("claude", text(9pt, weight: "bold", fill: accent.darken(20%), raw("CLAUDE.md")))

  // ── Inner ring: key project files ──
  core((cx, cy + 2.0), "src/traits.rs", "traits")
  core((cx, cy - 2.0), "Makefile", "make")
  core((cx - 3.0, cy), "src/models/", "models")
  core((cx + 3.0, cy), "src/rules/", "rules")

  // Links from center to inner ring
  line("claude.north", "traits.south", stroke: (thickness: 0.6pt, paint: edge-col, dash: "densely-dashed"), mark: arrow-end)
  line("claude.south", "make.north", stroke: (thickness: 0.6pt, paint: edge-col, dash: "densely-dashed"), mark: arrow-end)
  line("claude.west", "models.east", stroke: (thickness: 0.6pt, paint: edge-col, dash: "densely-dashed"), mark: arrow-end)
  line("claude.east", "rules.west", stroke: (thickness: 0.6pt, paint: edge-col, dash: "densely-dashed"), mark: arrow-end)

  // ── Outer ring: skill groups ──

  // Top-left: Orchestration
  let ol = (cx - 5.5, cy + 4.5)
  cat((ol.at(0), ol.at(1) + 0.5), [orchestration])
  skill((ol.at(0), ol.at(1)), "project-pipeline", "s1")
  skill((ol.at(0), ol.at(1) - 0.7), "review-pipeline", "s2")
  skill((ol.at(0), ol.at(1) - 1.4), "issue-to-pr", "s3")

  // Top-right: Quality gates
  let qr = (cx + 5.5, cy + 4.5)
  cat((qr.at(0), qr.at(1) + 0.5), [quality gates])
  skill((qr.at(0), qr.at(1)), "check-issue", "s4")
  skill((qr.at(0), qr.at(1) - 0.7), "review-impl", "s5")
  skill((qr.at(0), qr.at(1) - 1.4), "fix-pr", "s6")
  skill((qr.at(0), qr.at(1) - 2.1), "topology-check", "s7")

  // Bottom-left: Implementation
  let il = (cx - 5.5, cy - 3.0)
  cat((il.at(0), il.at(1) + 0.5), [implementation])
  skill((il.at(0), il.at(1)), "add-model", "s8")
  skill((il.at(0), il.at(1) - 0.7), "add-rule", "s9")

  // Bottom-right: Docs / community
  let dr = (cx + 5.5, cy - 3.0)
  cat((dr.at(0), dr.at(1) + 0.5), [docs / community])
  skill((dr.at(0), dr.at(1)), "write-in-paper", "s10")
  skill((dr.at(0), dr.at(1) - 0.7), "propose", "s11")
  skill((dr.at(0), dr.at(1) - 1.4), "dev-setup", "s12")

  // Dashed links from skill groups to center
  line("s3.east", "claude.north-west", stroke: stroke-dashed, mark: arrow-end)
  line("s7.west", "claude.north-east", stroke: stroke-dashed, mark: arrow-end)
  line("s9.east", "claude.south-west", stroke: stroke-dashed, mark: arrow-end)
  line("s12.west", "claude.south-east", stroke: stroke-dashed, mark: arrow-end)
})
