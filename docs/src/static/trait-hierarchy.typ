#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Noto Sans CJK SC")

#let trait-hierarchy(dark: false) = {
  let (fg, box-fill) = if dark {
    (rgb("#e2e8f0"), rgb("#1e293b"))
  } else {
    (rgb("#1e293b"), rgb("#f8fafc"))
  }

  set text(fill: fg, size: 9pt)

  diagram(
    node-stroke: 1.5pt + fg,
    edge-stroke: 1.5pt,
    spacing: (8mm, 12mm),

    let trait-fill = rgb("#e0e7ff"),
    let type-fill = rgb("#fef3c7"),

    // Problem trait (main)
    node((0, 0), box(width: 55mm, align(left)[
      #strong[trait Problem]\
      #text(size: 8pt, fill: rgb("#6b7280"))[
        `const NAME: &str`\
        `type Metric: Clone`\
        `fn dims() -> Vec<usize>`\
        `fn evaluate(&config) -> Metric`\
        `fn variant() -> Vec<(&str, &str)>`
      ]
    ]), fill: trait-fill, corner-radius: 6pt, inset: 10pt, name: <problem>),

    // OptimizationProblem trait
    node((0, 1), box(width: 55mm, align(left)[
      #strong[trait OptimizationProblem]\
      #text(size: 8pt, fill: rgb("#6b7280"))[
        `type Value: PartialOrd + Clone`\
        `fn direction() -> Direction`\
        #text(style: "italic")[requires `Metric = SolutionSize<Value>`]
      ]
    ]), fill: trait-fill, corner-radius: 6pt, inset: 10pt, name: <opt>),

    // Type boxes on the right
    node((1.3, 0), box(width: 38mm, align(left)[
      #strong[SolutionSize\<T\>]\
      #text(size: 8pt, fill: rgb("#6b7280"))[`Valid(T) | Invalid`]
    ]), fill: type-fill, corner-radius: 6pt, inset: 8pt, name: <solsize>),

    node((1.3, 1), box(width: 38mm, align(left)[
      #strong[Direction]\
      #text(size: 8pt, fill: rgb("#6b7280"))[`Maximize | Minimize`]
    ]), fill: type-fill, corner-radius: 6pt, inset: 8pt, name: <dir>),

    // Inheritance arrow
    edge(<opt>, <problem>, "->", stroke: 1.5pt + fg, label: text(size: 8pt)[extends], label-side: center),

    // Type associations (dashed)
    edge(<problem>, <solsize>, "-->", stroke: (paint: fg, dash: "dashed")),
    edge(<opt>, <dir>, "-->", stroke: (paint: fg, dash: "dashed")),
  )
}

#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#trait-hierarchy(dark: standalone-dark)
