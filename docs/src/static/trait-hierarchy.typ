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
    node((0.5, 0), box(width: 55mm, align(left)[
      #strong[trait Problem]\
      #text(size: 8pt, fill: secondary)[
        `const NAME: &str`\
        `type Metric: Clone`\
        `fn dims() -> Vec<usize>`\
        `fn evaluate(&config) -> Metric`\
        `fn variant() -> Vec<(&str, &str)>`
      ]
    ]), fill: trait-fill, corner-radius: 6pt, inset: 10pt, name: <problem>),

    // OptimizationProblem trait (bottom left)
    node((0, 1), box(width: 55mm, align(left)[
      #strong[trait OptimizationProblem]\
      #text(size: 8pt, fill: secondary)[
        `type Value: PartialOrd + Clone`\
        `fn direction() -> Direction`\
        #text(style: "italic")[requires `Metric = SolutionSize<Value>`]

      #strong[SolutionSize\<T\>]\
      #text(size: 8pt, fill: secondary)[`Valid(T) | Invalid`]

      #strong[Direction]\
      #text(size: 8pt, fill: secondary)[`Maximize | Minimize`]

      ]
    ]), fill: trait-fill, corner-radius: 6pt, inset: 10pt, name: <opt>),

    // SatisfactionProblem trait (bottom right)
    node((1.2, 1), box(width: 42mm, align(left)[
      #strong[trait SatisfactionProblem]\
      #text(size: 8pt, fill: secondary)[
        #text(style: "italic")[marker trait]\
        #text(style: "italic")[requires `Metric = bool`]
      ]
    ]), fill: trait-fill, corner-radius: 6pt, inset: 10pt, name: <sat>),

    // Inheritance arrows
    edge(<opt>, <problem>, "->", label: text(size: 8pt)[extends], label-side: left, label-fill: none),
    edge(<sat>, <problem>, "->", label: text(size: 8pt)[extends], label-side: right, label-fill: none),
  )
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#trait-hierarchy(dark: standalone-dark)

