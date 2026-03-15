// Shared theme for all paper figures.
// Usage: #import "lib.typ": *

#import "@preview/cetz:0.4.2": canvas, draw

// ── Page setup (standalone figures) ──
#let fig-page = (width: auto, height: auto, margin: 10pt)
#let fig-text = (size: 7.5pt, font: "New Computer Modern")

// ── Palette: black + one accent ──
#let accent     = rgb("#4e79a7")          // steel blue — the single accent
#let accent-light = accent.lighten(85%)
#let fg         = luma(30)                // near-black for text & strokes
#let fg-light   = luma(100)               // secondary text
#let border     = luma(60)                // box strokes
#let fill-light = luma(245)               // subtle box fill
#let fill-accent = accent.lighten(90%)    // accent-tinted fill
#let shadow-col = luma(215)               // drop shadow
#let edge-col   = luma(80)                // edge strokes

// ── Stroke presets ──
#let stroke-box     = (thickness: 1.3pt, paint: border)
#let stroke-accent  = (thickness: 1.3pt, paint: accent)
#let stroke-edge    = (thickness: 0.9pt, paint: edge-col)
#let stroke-dashed  = (thickness: 0.8pt, paint: edge-col, dash: "dashed")
#let stroke-dotted  = (thickness: 0.8pt, paint: edge-col, dash: "densely-dashed")

// ── Arrow preset ──
#let arrow-end = (end: "straight", scale: 0.4)
#let arrow-both = (start: "straight", end: "straight", scale: 0.4)
