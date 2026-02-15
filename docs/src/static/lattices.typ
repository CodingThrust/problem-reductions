// Demonstration of graph types used in the problem-reductions library.
// Compile:
//   typst compile lattices.typ --input dark=false lattices.svg
//   typst compile lattices.typ --input dark=true  lattices-dark.svg
#import "@preview/cetz:0.4.2": canvas, draw
#import "../../paper/lib.typ": g-node, g-edge

#set page(width: auto, height: auto, margin: 12pt, fill: none)

#let lattices(dark: false) = {
  // ── Theme colors ────────────────────────────────────────────────
  let (fg, edge-color, secondary) = if dark {
    (rgb("#e2e8f0"), rgb("#94a3b8"), rgb("#94a3b8"))
  } else {
    (rgb("#1e293b"), rgb("#64748b"), rgb("#64748b"))
  }

  let (node-fill, node-highlight) = if dark {
    (rgb("#1e3a5f"), rgb("#2563eb"))
  } else {
    (rgb("#dbeafe"), rgb("#93c5fd"))
  }

  let hyper-colors = if dark {
    (
      (fill: rgb("#1e3a5f").transparentize(30%), stroke: rgb("#60a5fa")),
      (fill: rgb("#7f1d1d").transparentize(30%), stroke: rgb("#f87171")),
      (fill: rgb("#064e3b").transparentize(30%), stroke: rgb("#34d399")),
    )
  } else {
    (
      (fill: rgb("#dbeafe").transparentize(40%), stroke: rgb("#4e79a7")),
      (fill: rgb("#fecaca").transparentize(40%), stroke: rgb("#e15759")),
      (fill: rgb("#d1fae5").transparentize(40%), stroke: rgb("#059669")),
    )
  }

  let hyper-node-fill = if dark { rgb("#1e293b") } else { white }

  let disk-fill = if dark {
    rgb("#1e3a5f").transparentize(70%)
  } else {
    rgb("#dbeafe").transparentize(70%)
  }

  set text(fill: fg, size: 9pt)

  // ── (a) SimpleGraph ──────────────────────────────────────────────
  let simple-graph-fig() = {
    import draw: *
    let vs = ((0, 0), (2, 0), (0, 1.5), (2, 1.5), (1, 2.5))
    let es = ((0,1),(0,2),(1,3),(2,3),(2,4),(3,4))
    for (u, v) in es { g-edge(vs.at(u), vs.at(v), stroke: 1pt + edge-color) }
    for (k, pos) in vs.enumerate() {
      g-node(pos, name: "s" + str(k), fill: node-fill, stroke: 0.5pt + edge-color, label: str(k))
    }
  }

  // ── (b) HyperGraph ──────────────────────────────────────────────
  let hypergraph-fig() = {
    import draw: *
    let vs = ((0, 0), (1.5, 0.3), (2.5, 0), (0.5, 1.5), (2, 1.5), (1.2, 2.5))

    // Hyperedge A: {0, 1, 3}
    draw.hobby(
      (-.3, -.3), (0.8, -.2), (1.9, 0.2), (0.8, 1.8), (-.2, 1.8), (-.5, 0.5),
      close: true,
      fill: hyper-colors.at(0).fill,
      stroke: 0.8pt + hyper-colors.at(0).stroke,
    )
    // Hyperedge B: {1, 2, 4}
    draw.hobby(
      (1.1, -.2), (2.8, -.3), (2.6, 1.8), (1.6, 1.8), (0.9, 0.8),
      close: true,
      fill: hyper-colors.at(1).fill,
      stroke: 0.8pt + hyper-colors.at(1).stroke,
    )
    // Hyperedge C: {3, 4, 5}
    draw.hobby(
      (0.1, 1.2), (1.6, 1.0), (2.4, 1.8), (1.5, 2.9), (0.1, 2.2),
      close: true,
      fill: hyper-colors.at(2).fill,
      stroke: 0.8pt + hyper-colors.at(2).stroke,
    )

    for (k, pos) in vs.enumerate() {
      g-node(pos, name: "h" + str(k), fill: hyper-node-fill, stroke: 1pt + edge-color, label: str(k))
    }
  }

  // ── (c) UnitDiskGraph ────────────────────────────────────────────
  let unit-disk-fig() = {
    import draw: *
    let vs = ((0.2, 0.2), (1.0, 0.0), (2.2, 0.3), (0.0, 1.2), (1.2, 1.5), (2.0, 1.1), (0.8, 2.3))
    let r = 1.25

    // Radius disk around vertex 4
    draw.circle(vs.at(4), radius: r, fill: disk-fill, stroke: (dash: "dashed", paint: edge-color, thickness: 0.6pt))

    // Compute edges: connect pairs within distance r
    let es = ()
    for i in range(vs.len()) {
      for j in range(i + 1, vs.len()) {
        let dx = vs.at(i).at(0) - vs.at(j).at(0)
        let dy = vs.at(i).at(1) - vs.at(j).at(1)
        if calc.sqrt(dx * dx + dy * dy) <= r {
          es.push((i, j))
        }
      }
    }

    for (u, v) in es { g-edge(vs.at(u), vs.at(v), stroke: 0.8pt + edge-color) }
    for (k, pos) in vs.enumerate() {
      let fill = if k == 4 { node-highlight } else { node-fill }
      g-node(pos, name: "u" + str(k), fill: fill, stroke: 0.5pt + edge-color, label: str(k))
    }

    // Radius label
    draw.content((vs.at(4).at(0) + r + 0.15, vs.at(4).at(1) + 0.1), text(7pt, fill: secondary)[$r$])
  }

  // ── (d) KingsSubgraph ────────────────────────────────────────────
  let kings-fig() = {
    import draw: *
    let rows = 4
    let cols = 5
    let sp = 0.6
    let vs = ()
    for row in range(rows) {
      for col in range(cols) {
        vs.push((col * sp, -row * sp))
      }
    }

    let es = ()
    for row in range(rows) {
      for col in range(cols) {
        let i = row * cols + col
        if col + 1 < cols { es.push((i, i + 1)) }
        if row + 1 < rows { es.push((i, i + cols)) }
        if row + 1 < rows and col + 1 < cols { es.push((i, i + cols + 1)) }
        if row + 1 < rows and col > 0 { es.push((i, i + cols - 1)) }
      }
    }

    for (u, v) in es { g-edge(vs.at(u), vs.at(v), stroke: 0.6pt + edge-color) }
    for (k, pos) in vs.enumerate() {
      g-node(pos, name: "k" + str(k), radius: 0.12, fill: node-fill, stroke: 0.5pt + edge-color)
    }
  }

  // ── (e) TriangularSubgraph ───────────────────────────────────────
  let triangular-fig() = {
    import draw: *
    let rows = 5
    let cols = 6
    let sp = 0.6
    let sqrt3_2 = calc.sqrt(3) / 2
    let vs = ()
    for row in range(rows) {
      let offset = if calc.rem(row, 2) == 0 { 0 } else { 0.5 * sp }
      for col in range(cols) {
        vs.push((col * sp + offset, -row * sqrt3_2 * sp))
      }
    }

    let es = ()
    for row in range(rows) {
      for col in range(cols) {
        let i = row * cols + col
        if col + 1 < cols { es.push((i, i + 1)) }
        if row + 1 < rows {
          if calc.rem(row, 2) == 0 {
            if col > 0 { es.push((i, (row + 1) * cols + col - 1)) }
            es.push((i, (row + 1) * cols + col))
          } else {
            es.push((i, (row + 1) * cols + col))
            if col + 1 < cols { es.push((i, (row + 1) * cols + col + 1)) }
          }
        }
      }
    }

    for (u, v) in es { g-edge(vs.at(u), vs.at(v), stroke: 0.5pt + edge-color) }
    for (k, pos) in vs.enumerate() {
      g-node(pos, name: "t" + str(k), radius: 0.1, fill: node-fill, stroke: 0.4pt + edge-color)
    }
  }

  // ── Layout ───────────────────────────────────────────────────────
  let font-size = 12pt
  canvas({
    import draw: *
    simple-graph-fig()
    content((1, -1), text(font-size, [(a) SimpleGraph]))
    set-origin((5, 0))
    hypergraph-fig()
    content((1, -1), text(font-size, [(b) HyperGraph]))
    set-origin((5, 0))
    unit-disk-fig()
    content((1, -1), text(font-size, [(c) UnitDiskGraph]))
    set-origin((-10, -2))
    kings-fig()
    content((1, -2.6), text(font-size, [(d) KingsSubgraph]))
    set-origin((5, 0))
    triangular-fig()
    content((1, -2.6), text(font-size, [(e) TriangularSubgraph]))
  })
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#lattices(dark: standalone-dark)
