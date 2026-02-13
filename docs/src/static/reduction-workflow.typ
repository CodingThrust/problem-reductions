#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Helvetica Neue")

#let reduction-workflow(dark: false) = {
  let (fg, box-color, box-fill, success-fill) = if dark {
    (rgb("#e2e8f0"), rgb("#94a3b8"), rgb("#1e293b"), rgb("#14532d"))
  } else {
    (rgb("#1e293b"), rgb("#64748b"), rgb("#f8fafc"), rgb("#dcfce7"))
  }

  set text(fill: fg, size: 10pt)

  diagram(
    node-stroke: 1.5pt + box-color,
    edge-stroke: 1.5pt,
    spacing: (20mm, 10mm),

    let accent = rgb("#3b82f6"),
    let success = rgb("#22c55e"),

    // Nodes
    node((0, 0), box(width: 28mm, align(center)[*Problem A*\ #text(size: 8pt)[source problem]]), fill: box-fill, corner-radius: 6pt, inset: 10pt, name: <a>),
    node((1, 0), box(width: 28mm, align(center)[*Problem B*\ #text(size: 8pt)[target problem]]), fill: box-fill, corner-radius: 6pt, inset: 10pt, name: <b>),
    node((2, 0), box(width: 28mm, align(center)[*Solution B*\ #text(size: 8pt)[solver output]]), fill: box-fill, corner-radius: 6pt, inset: 10pt, name: <sol-b>),
    node((1, 1), box(width: 28mm, align(center)[*Solution A*\ #text(size: 8pt)[extracted result]]), fill: success-fill, stroke: 1.5pt + success, corner-radius: 6pt, inset: 10pt, name: <sol-a>),

    // Edges with labels
    edge(<a>, <b>, "->", stroke: 1.5pt + accent, label: text(size: 9pt)[`reduce_to`], label-sep: 5pt, label-pos: 0.5, label-side: left),
    edge(<b>, <sol-b>, "->", stroke: 1.5pt + accent, label: text(size: 9pt)[`find_best`], label-sep: 5pt, label-pos: 0.5, label-side: left),
    edge(<sol-b>, <sol-a>, "->", stroke: 1.5pt + success, label: text(size: 9pt)[`extract_solution`], label-sep: 2pt, label-pos: 0.5, label-side: left),
  )
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#reduction-workflow(dark: standalone-dark)
