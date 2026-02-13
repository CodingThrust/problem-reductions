#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Helvetica Neue")

#let module-overview(dark: false) = {
  let (fg, box-color) = if dark {
    (rgb("#e2e8f0"), rgb("#94a3b8"))
  } else {
    (rgb("#1e293b"), rgb("#64748b"))
  }

  // Module colors - darker versions for dark mode
  let (model-color, rule-color, registry-color, solver-color) = if dark {
    (rgb("#166534"), rgb("#1e3a5f"), rgb("#854d0e"), rgb("#7f1d1d"))
  } else {
    (rgb("#dcfce7"), rgb("#dbeafe"), rgb("#fef3c7"), rgb("#fee2e2"))
  }

  set text(fill: fg, size: 10pt)

  diagram(
    node-stroke: 1.5pt + box-color,
    edge-stroke: 1.5pt + box-color,
    spacing: (25mm, 15mm),

    // Module nodes
    node((0, 0), box(width: 30mm, align(center)[#strong[models/]\ #text(size: 10pt)[Problem types]]), fill: model-color, corner-radius: 6pt, inset: 10pt, name: <models>),
    node((1, 0), box(width: 30mm, align(center)[#strong[rules/]\ #text(size: 10pt)[Reductions]]), fill: rule-color, corner-radius: 6pt, inset: 10pt, name: <rules>),
    node((2, 0), box(width: 30mm, align(center)[#strong[registry/]\ #text(size: 10pt)[Graph metadata]]), fill: registry-color, corner-radius: 6pt, inset: 10pt, name: <registry>),
    node((1, 1), box(width: 30mm, align(center)[#strong[solvers/]\ #text(size: 8pt)[BruteForce, ILP]]), fill: solver-color, corner-radius: 6pt, inset: 10pt, name: <solvers>),

    // Relationships
    edge(<models>, <rules>, "->", label: text(size: 10pt)[imports], label-sep: 2pt, label-pos: 0.5, label-side: left),
    edge(<rules>, <registry>, "->", label: text(size: 10pt)[registers], label-sep: 2pt, label-pos: 0.5, label-side: left),
    edge(<solvers>, <models>, "<-", label: text(size: 10pt)[solves], label-sep: 2pt, label-pos: 0.5, label-side: left),
  )
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#module-overview(dark: standalone-dark)
