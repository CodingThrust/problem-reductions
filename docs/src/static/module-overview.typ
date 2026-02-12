#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Noto Sans CJK SC")

#let module-overview(dark: false) = {
  let (fg, box-fill) = if dark {
    (rgb("#e2e8f0"), rgb("#1e293b"))
  } else {
    (rgb("#1e293b"), rgb("#f8fafc"))
  }

  set text(fill: fg, size: 10pt)

  diagram(
    node-stroke: 1.5pt + fg,
    edge-stroke: 1.5pt,
    spacing: (25mm, 15mm),

    let model-color = rgb("#c8f0c8"),
    let rule-color = rgb("#c8c8f0"),
    let registry-color = rgb("#f0f0a0"),
    let solver-color = rgb("#f0c8c8"),

    // Module nodes
    node((0, 0), box(width: 30mm, align(center)[#strong[models/]\ #text(size: 8pt)[Problem types]]), fill: model-color, corner-radius: 6pt, inset: 10pt, name: <models>),
    node((1, 0), box(width: 30mm, align(center)[#strong[rules/]\ #text(size: 8pt)[Reductions]]), fill: rule-color, corner-radius: 6pt, inset: 10pt, name: <rules>),
    node((2, 0), box(width: 30mm, align(center)[#strong[registry/]\ #text(size: 8pt)[Graph metadata]]), fill: registry-color, corner-radius: 6pt, inset: 10pt, name: <registry>),
    node((1, 1), box(width: 30mm, align(center)[#strong[solvers/]\ #text(size: 8pt)[BruteForce, ILP]]), fill: solver-color, corner-radius: 6pt, inset: 10pt, name: <solvers>),

    // Relationships
    edge(<models>, <rules>, "<->", label: text(size: 8pt)[imports], label-side: center),
    edge(<rules>, <registry>, "->", label: text(size: 8pt)[registers], label-side: center),
    edge(<solvers>, <models>, "->", label: text(size: 8pt)[solves], label-side: center),
  )
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#module-overview(dark: standalone-dark)
