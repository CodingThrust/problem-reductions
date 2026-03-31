// Proposed Reduction Rules — Verification Notes
#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules

#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

#show link: set text(blue)
#show: thmrules.with(qed-symbol: $square$)

#let theorem = thmbox("theorem", "Theorem", fill: rgb("#e8f4f8"))
#let proof = thmproof("proof", "Proof")

#align(center)[
  #text(size: 16pt, weight: "bold")[Proposed Reduction Rules — Verification Notes]

  #v(0.5em)
  #text(size: 11pt)[Mathematical Foundations for Implementation]

  #v(0.5em)
  #text(size: 10pt, style: "italic")[
    Reference document for GitHub issues in the
    #link("https://github.com/CodingThrust/problem-reductions")[problem-reductions] project
  ]
]

#v(1em)

*Abstract.* This document formalizes nine reduction rules between NP-hard problems, providing complete construction algorithms, bidirectional correctness proofs, solution extraction procedures, and worked examples. Two reductions (#sym.section 2) extend the NP-hardness proof chain from 3-SAT, potentially increasing the verified reachable count from 29 to 40+. The remaining seven (#sym.section 3--5) resolve blockers in previously incomplete GitHub issues. Each entry is detailed enough to serve as a direct specification for implementation.

#outline(indent: auto, depth: 2)
#pagebreak()

= Notation and Conventions

Throughout this document we use the following conventions:

- $G = (V, E)$ denotes an undirected graph with vertex set $V$ and edge set $E$
- $n = |V|$, $m = |E|$
- $w: E -> ZZ^+$ denotes positive integer edge weights (when weighted)
- $N[v] = {v} union {u : (u,v) in E}$ is the closed neighbourhood of $v$
- For a set $S subset.eq V$, we write $w(S) = sum_(v in S) w(v)$

Each reduction entry contains:
+ *Theorem statement* --- intuition and citation
+ *Proof* with three subsections:
  - _Construction:_ numbered algorithm steps, all symbols defined before use
  - _Correctness:_ bidirectional ($arrow.r.double$ and $arrow.l.double$)
  - _Solution extraction:_ mapping target solution back to source
+ *Overhead table* --- target size as a function of source size
+ *Worked example* --- concrete instance with full verification
