#import "@preview/cetz:0.4.2": canvas, draw

#set page(width: auto, height: auto, margin: 5pt)
#set text(size: 8pt, font: "New Computer Modern")

#let col-trait = rgb("#4e79a7")       // blue
#let col-reduction = rgb("#59a14f")   // green
#let col-compile = rgb("#e8a838")     // gold

#canvas(length: 0.55cm, {
  import draw: *

  let box-w = 12.0
  let box1-h = 3.0   // Problem trait box (taller: has two sub-boxes)
  let box2-h = 1.6   // ReductionResult box
  let box3-h = 2.4   // Compile-time validation box
  let arrow-gap = 1.4
  let cx = 0

  // --- Box 1: Problem trait (top) ---
  let y1-top = 0
  let y1-bot = -box1-h

  rect(
    (cx - box-w / 2, y1-top), (cx + box-w / 2, y1-bot),
    radius: 4pt,
    fill: col-trait.lighten(88%),
    stroke: (thickness: 1pt, paint: col-trait),
    name: "box1",
  )

  // Title
  content(
    (cx, y1-top - 0.4), anchor: "center",
    text(9pt, weight: "bold", fill: col-trait.darken(20%),
      [`Problem` trait],
    ),
  )

  // Method list
  content(
    (cx, y1-top - 1.0), anchor: "center",
    text(7.5pt, fill: luma(60),
      [`NAME`#h(4pt)#sym.dot.c#h(4pt)`Metric`#h(4pt)#sym.dot.c#h(4pt)`dims()`#h(4pt)#sym.dot.c#h(4pt)`evaluate()`],
    ),
  )

  // Divider line
  line(
    (cx - box-w / 2 + 0.4, y1-top - 1.4),
    (cx + box-w / 2 - 0.4, y1-top - 1.4),
    stroke: (thickness: 0.5pt, paint: col-trait.lighten(40%)),
  )

  // Sub-boxes for Optimization and Satisfaction
  let sub-margin = 0.35  // margin from parent box edge
  let sub-gap = 0.3      // gap between sub-boxes
  let sub-w = (box-w - 2 * sub-margin - sub-gap) / 2  // = 5.525 each
  let sub-h = 1.0
  let sub-y-top = y1-top - 1.6
  let sub-y-bot = sub-y-top - sub-h

  // Optimization sub-box (left)
  let opt-left = cx - box-w / 2 + sub-margin
  let opt-right = opt-left + sub-w
  rect(
    (opt-left, sub-y-top), (opt-right, sub-y-bot),
    radius: 3pt,
    fill: col-trait.lighten(78%),
    stroke: (thickness: 0.6pt, paint: col-trait.lighten(20%)),
    name: "opt",
  )
  content(
    "opt", anchor: "center",
    {
      text(7.5pt, weight: "bold", fill: col-trait.darken(10%), [`OptimizationProblem`])
      linebreak()
      text(6pt, fill: luma(80), [`SolutionSize<W>` #sym.dot.c `direction()`])
    },
  )

  // Satisfaction sub-box (right)
  let sat-left = opt-right + sub-gap
  let sat-right = sat-left + sub-w
  rect(
    (sat-left, sub-y-top), (sat-right, sub-y-bot),
    radius: 3pt,
    fill: col-trait.lighten(78%),
    stroke: (thickness: 0.6pt, paint: col-trait.lighten(20%)),
    name: "sat",
  )
  content(
    "sat", anchor: "center",
    {
      text(7.5pt, weight: "bold", fill: col-trait.darken(10%), [`SatisfactionProblem`])
      linebreak()
      text(6pt, fill: luma(80), [`Metric = bool`])
    },
  )

  // --- Arrow 1: Box 1 -> Box 2 ---
  let a1-top = y1-bot
  let a1-bot = y1-bot - arrow-gap
  line(
    (cx, a1-top), (cx, a1-bot + 0.05),
    stroke: (thickness: 1.2pt, paint: col-reduction.darken(10%)),
    mark: (end: "straight", scale: 0.45),
  )
  content(
    (cx + 0.3, (a1-top + a1-bot) / 2), anchor: "west",
    text(7.5pt, weight: "bold", fill: col-reduction.darken(10%),
      [`ReduceTo<T>`],
    ),
  )

  // --- Box 2: ReductionResult ---
  let y2-top = a1-bot
  let y2-bot = y2-top - box2-h

  rect(
    (cx - box-w / 2, y2-top), (cx + box-w / 2, y2-bot),
    radius: 4pt,
    fill: col-reduction.lighten(88%),
    stroke: (thickness: 1pt, paint: col-reduction),
    name: "box2",
  )

  content(
    (cx, y2-top - 0.45), anchor: "center",
    text(9pt, weight: "bold", fill: col-reduction.darken(20%),
      [`ReductionResult<T>`],
    ),
  )

  content(
    (cx, y2-top - 1.1), anchor: "center",
    text(7.5pt, fill: luma(60),
      [`target_problem()`#h(4pt)#sym.dot.c#h(4pt)`extract_solution()`],
    ),
  )

  // --- Arrow 2: Box 2 -> Box 3 ---
  let a2-top = y2-bot
  let a2-bot = y2-bot - arrow-gap
  line(
    (cx, a2-top), (cx, a2-bot + 0.05),
    stroke: (thickness: 1.2pt, paint: col-compile.darken(10%)),
    mark: (end: "straight", scale: 0.45),
  )
  content(
    (cx + 0.3, (a2-top + a2-bot) / 2), anchor: "west",
    text(7pt, fill: col-compile.darken(10%),
      [`#[reduction(overhead = {...})]`],
    ),
  )

  // --- Box 3: Compile-time validation ---
  let y3-top = a2-bot
  let y3-bot = y3-top - box3-h

  rect(
    (cx - box-w / 2, y3-top), (cx + box-w / 2, y3-bot),
    radius: 4pt,
    fill: col-compile.lighten(88%),
    stroke: (thickness: 1pt, paint: col-compile),
    name: "box3",
  )

  content(
    (cx, y3-top - 0.45), anchor: "center",
    text(9pt, weight: "bold", fill: col-compile.darken(20%),
      [Compile-time validation],
    ),
  )

  // Bullet points
  let bullet-x = cx - box-w / 2 + 1.2
  let bullet-y = y3-top - 1.1

  content(
    (bullet-x, bullet-y), anchor: "west",
    text(7.5pt, fill: luma(60),
      [#sym.bullet#h(3pt)Variable names #sym.arrow getter methods],
    ),
  )
  content(
    (bullet-x, bullet-y - 0.55), anchor: "west",
    text(7.5pt, fill: luma(60),
      [#sym.bullet#h(3pt)`Expr` AST: symbolic overhead expressions],
    ),
  )
  content(
    (bullet-x, bullet-y - 1.1), anchor: "west",
    text(7.5pt, fill: luma(60),
      [#sym.bullet#h(3pt)`declare_variants!` #sym.arrow compile-time registry],
    ),
  )
})
