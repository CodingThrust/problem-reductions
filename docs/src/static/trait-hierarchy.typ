#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Helvetica Neue")

#let trait-hierarchy(dark: false) = {
  let (fg, box-color, secondary) = if dark {
    (rgb("#e2e8f0"), rgb("#94a3b8"), rgb("#94a3b8"))
  } else {
    (rgb("#1e293b"), rgb("#64748b"), rgb("#6b7280"))
  }

  // Trait and type fills - darker for dark mode
  let (trait-fill, type-fill) = if dark {
    (rgb("#1e3a5f"), rgb("#854d0e"))
  } else {
    (rgb("#dbeafe"), rgb("#fef3c7"))
  }

  set text(fill: fg, size: 9pt)

  diagram(
    node-stroke: 1.5pt + box-color,
    edge-stroke: 1.5pt + box-color,
    spacing: (8mm, 12mm),

    // Problem trait (top center)
    node((0.6, 0), box(width: 55mm, align(left)[
      #strong[trait Problem]\
      #text(size: 8pt, fill: secondary)[
        `const NAME: &str`\
        `type Value: Clone`\
        `fn dims() -> Vec<usize>`\
        `fn evaluate(&config) -> Value`\
        `fn variant() -> Vec<(&str, &str)>`
      ]
    ]), fill: trait-fill, corner-radius: 6pt, inset: 10pt, name: <problem>),

    // Aggregate trait (bottom left)
    node((0, 1), box(width: 55mm, align(left)[
      #strong[trait Aggregate]\
      #text(size: 8pt, fill: secondary)[
        `fn identity() -> Self`\
        `fn combine(self, other) -> Self`\
        `fn supports_witnesses() -> bool`\
        `fn contributes_to_witnesses(...)`
      ]
    ]), fill: trait-fill, corner-radius: 6pt, inset: 10pt, name: <aggregate>),

    // Common value types (bottom right)
    node((1.25, 1), box(width: 48mm, align(left)[
      #strong[Common Value Types]\
      #text(size: 8pt, fill: secondary)[
        `Max<V> | Min<V> | Extremum<V>`\
        `Or | Sum<W> | And`\
        #text(style: "italic")[used as `Problem::Value`]
      ]
    ]), fill: type-fill, corner-radius: 6pt, inset: 10pt, name: <values>),

    // Conceptual relationships
    edge(<aggregate>, <problem>, "->", label: text(size: 8pt)[solver-bound on `Value`], label-side: left, label-fill: none),
    edge(<values>, <aggregate>, "->", label: text(size: 8pt)[implements], label-side: right, label-fill: none),
  )
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#trait-hierarchy(dark: standalone-dark)
