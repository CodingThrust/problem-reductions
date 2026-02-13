#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: (top: 5pt, bottom: 5pt, left: 5pt, right: 5pt), fill: none)
#set text(font: "Helvetica Neue")

// Parameterized workflow-loop diagram
// Usage: #workflow-loop(lang: "zh", label-size: 12pt, scale: 1.2)
// Compile: typst compile workflow-loop.typ --input dark=false
//          typst compile workflow-loop.typ --input dark=true -o workflow-loop-dark.svg
#let workflow-loop(
  lang: "en",           // "en" or "zh"
  dark: false,          // dark mode colors
  label-size: 10pt,     // font size for phase labels
  text-size: 10pt,      // font size for box content  
  fix-label-size: 9pt,  // font size for "fix" label
  scale: 1.0,           // overall scale factor
  spacing: (25mm, 12mm), // (horizontal, vertical) spacing
) = {
  // Translation helper
  let t(en, zh) = if lang == "zh" { zh } else { en }
  
  // Apply scale to spacing
  let sp = (spacing.at(0) * scale, spacing.at(1) * scale)
  
  // Theme colors
  let (fg, box-color, box-fill, merge-fill) = if dark {
    (rgb("#e2e8f0"), rgb("#94a3b8"), rgb("#1e293b"), rgb("#14532d"))
  } else {
    (rgb("#1e293b"), rgb("#64748b"), rgb("#f8fafc"), rgb("#dcfce7"))
  }
  
  set text(fill: fg)

  diagram(
    node-stroke: 1pt + fg,
    edge-stroke: 1.5pt,
    spacing: sp,

    let accent = rgb("#3b82f6"),
    let success = rgb("#22c55e"),
    let feedback = rgb("#ef4444"),

    // Phase labels at top
    node((0, -0.5), text(weight: "bold", size: label-size)[#t("PLANNING", "规划")], stroke: none, fill: none),
    node((1, -0.5), text(weight: "bold", size: label-size)[#t("ACTION", "执行")], stroke: none, fill: none),
    node((2, -0.5), text(weight: "bold", size: label-size)[#t("REVIEW", "审核")], stroke: none, fill: none),

    // Main boxes: circle -> rect -> rect
    let min-width = 30mm * scale,
    node((0, 0.5), box(width: 22mm * scale, align(center, text(size: text-size)[👤 *#t("Human", "用户")*\ _issues_ &\ _milestones_])), fill: box-fill, stroke: 2pt + box-color, shape: "circle", inset: 12pt * scale, name: <plan>),
    node((1, 0.5), box(width: min-width, align(center, text(size: text-size)[🤖 *#t("AI Agent", "AI 助手")*\ _commits_ &\ _pull requests_])), fill: box-fill, stroke: 2pt + box-color, corner-radius: 8pt, inset: 12pt * scale, width: 35mm * scale, name: <action>),
    node((2, 0.5), box(width: min-width, align(center, text(size: text-size)[🤝 *#t("AI + Human", "AI + 用户")*\ _CI/CD_ &\ _reviews_])), fill: box-fill, stroke: 2pt + box-color, corner-radius: 8pt, inset: 12pt * scale, width: 35mm * scale, name: <review>),

    // Merge node
    node((2, 1.5), text(size: text-size)[✅ *Merge*], fill: merge-fill, stroke: 2pt + success, corner-radius: 6pt, inset: 10pt * scale, name: <merge>),

    // Main flow
    edge(<plan>, <action>, "->", stroke: 1.5pt + accent),
    edge(<action>, <review>, "->", stroke: 1.5pt + accent),
    edge(<review>, <merge>, "->", stroke: 1.5pt + success),

    // Fix loop back to action (dashed)
    edge(<review>, <action>, "->", stroke: (paint: feedback, thickness: 1.5pt, dash: "dashed"), bend: -40deg, label: text(size: fix-label-size, fill: feedback)[#t("fix", "修复")]),
  )
}

// Default render when compiled standalone
#let standalone-lang = sys.inputs.at("lang", default: "en")
#let standalone-dark = sys.inputs.at("dark", default: "false") == "true"
#workflow-loop(lang: standalone-lang, dark: standalone-dark)
