#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Helvetica Neue")

#let variant-hierarchy(dark: false) = {
  let (fg, box-color, secondary) = if dark {
    (rgb("#e2e8f0"), rgb("#94a3b8"), rgb("#94a3b8"))
  } else {
    (rgb("#1e293b"), rgb("#64748b"), rgb("#6b7280"))
  }

  let (graph-fill, weight-fill, k-fill, label-fill) = if dark {
    (rgb("#1e3a5f"), rgb("#3b1f2b"), rgb("#1a3b2a"), rgb("#334155"))
  } else {
    (rgb("#dbeafe"), rgb("#fce7f3"), rgb("#dcfce7"), rgb("#f1f5f9"))
  }

  set text(fill: fg, size: 9pt)

  // --- Graph type hierarchy ---
  diagram(
    node-stroke: 1.5pt + box-color,
    edge-stroke: 1pt + box-color,
    spacing: (10mm, 8mm),

    // Section labels
    node((-0.3, -0.5), text(size: 10pt, weight: "bold")[Graph Types], stroke: none, fill: none),
    node((3.2, -0.5), text(size: 10pt, weight: "bold")[Weights], stroke: none, fill: none),
    node((5, -0.5), text(size: 10pt, weight: "bold")[K Values], stroke: none, fill: none),

    // Graph hierarchy (tree)
    node((0, 0), [HyperGraph], fill: graph-fill, corner-radius: 5pt, inset: 6pt, name: <hg>),
    node((0, 1), [SimpleGraph], fill: graph-fill, corner-radius: 5pt, inset: 6pt, name: <sg>),
    node((-1, 2), [PlanarGraph], fill: graph-fill, corner-radius: 5pt, inset: 6pt, name: <pg>),
    node((0, 2), [BipartiteGraph], fill: graph-fill, corner-radius: 5pt, inset: 6pt, name: <bg>),
    node((1, 2), [UnitDiskGraph], fill: graph-fill, corner-radius: 5pt, inset: 6pt, name: <udg>),
    node((0.5, 3), [KingsSubgraph], fill: graph-fill, corner-radius: 5pt, inset: 6pt, name: <ksg>),
    node((1.5, 3), [TriangularSubgraph], fill: graph-fill, corner-radius: 5pt, inset: 6pt, name: <tri>),

    edge(<sg>, <hg>, "->"),
    edge(<pg>, <sg>, "->"),
    edge(<bg>, <sg>, "->"),
    edge(<udg>, <sg>, "->"),
    edge(<ksg>, <udg>, "->"),
    edge(<tri>, <udg>, "->"),

    // Weight hierarchy (chain: One → i32 → f64)
    node((3.2, 0), [f64], fill: weight-fill, corner-radius: 5pt, inset: 6pt, name: <w-f64>),
    node((3.2, 1), [i32], fill: weight-fill, corner-radius: 5pt, inset: 6pt, name: <w-i32>),
    node((3.2, 2), [One], fill: weight-fill, corner-radius: 5pt, inset: 6pt, name: <w-one>),

    edge(<w-one>, <w-i32>, "->"),
    edge(<w-i32>, <w-f64>, "->"),

    // K value hierarchy (flat star)
    node((5, 0), [KN], fill: k-fill, corner-radius: 5pt, inset: 6pt, name: <kn>),
    node((4.2, 1), [K1], fill: k-fill, corner-radius: 5pt, inset: 6pt, name: <k1>),
    node((4.6, 1), [K2], fill: k-fill, corner-radius: 5pt, inset: 6pt, name: <k2>),
    node((5, 1), [K3], fill: k-fill, corner-radius: 5pt, inset: 6pt, name: <k3>),
    node((5.4, 1), [K4], fill: k-fill, corner-radius: 5pt, inset: 6pt, name: <k4>),
    node((5.8, 1), [K5], fill: k-fill, corner-radius: 5pt, inset: 6pt, name: <k5>),

    edge(<k1>, <kn>, "->"),
    edge(<k2>, <kn>, "->"),
    edge(<k3>, <kn>, "->"),
    edge(<k4>, <kn>, "->"),
    edge(<k5>, <kn>, "->"),
  )

  v(3mm)
  text(size: 8pt, fill: secondary)[Arrows point from specific to general (subtype direction).]
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#variant-hierarchy(dark: standalone-dark)
