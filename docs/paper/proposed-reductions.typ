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

#pagebreak()

= NP-Hardness Chain Extensions

== SubsetSum $arrow.r$ Partition <sec:subsetsum-partition>

#theorem[
  Subset Sum reduces to Partition by adding at most one padding element. Given a Subset Sum instance $(S, T)$ with $Sigma = sum S$, the padding element $d = |Sigma - 2T|$ ensures that a balanced partition of $S union {d}$ exists if and only if a subset of $S$ sums to $T$. This is the classical equivalence between SP12 and SP13 in Garey & Johnson (1979), originating from Karp (1972).
] <thm:subsetsum-partition>

#proof[
  _Construction._ Given a Subset Sum instance with elements $S = {s_1, dots, s_n}$ and target $T$:
  + Compute $Sigma = sum_(i=1)^n s_i$.
  + Compute $d = |Sigma - 2T|$.
  + If $d = 0$: output $"Partition"(S)$.
  + If $d > 0$: output $"Partition"(S union {d})$, appending $d$ as the $(n+1)$-th element.

  _Correctness._ Let $Sigma'$ denote the sum of the Partition instance.

  *Case $d = 0$ ($Sigma = 2T$):* $Sigma' = 2T$, half $= T$. A subset summing to $T$ exists $arrow.l.r.double$ a balanced partition exists. $checkmark$

  *Case $Sigma > 2T$ ($d = Sigma - 2T > 0$):* $Sigma' = Sigma + d = 2(Sigma - T)$, half $= Sigma - T$.

  ($arrow.r.double$) If $A subset.eq S$ sums to $T$, place $A union {d}$ on one side: sum $= T + (Sigma - 2T) = Sigma - T =$ half. The other side $S backslash A$ also sums to $Sigma - T$. $checkmark$

  ($arrow.l.double$) Given a balanced partition, $d$ is on one side. The $S$-elements on that side sum to $(Sigma - T) - d = (Sigma - T) - (Sigma - 2T) = T$. $checkmark$

  *Case $Sigma < 2T$ ($d = 2T - Sigma > 0$):* $Sigma' = Sigma + d = 2T$, half $= T$.

  ($arrow.r.double$) If $A subset.eq S$ sums to $T$, place $A$ on one side (sum $= T$) and $(S backslash A) union {d}$ on the other (sum $= (Sigma - T) + (2T - Sigma) = T$). $checkmark$

  ($arrow.l.double$) Given a balanced partition, the side without $d$ has $S$-elements summing to $T$. $checkmark$

  *Infeasible case ($T > Sigma$):* $d = 2T - Sigma > Sigma$, so $d > Sigma' slash 2 = T$. One element exceeds half the total, making partition impossible. $checkmark$

  _Solution extraction._
  - If $d = 0$: the partition config directly gives the subset assignment.
  - If $Sigma > 2T$: $S$-elements on the *same side* as $d$ form the subset summing to $T$.
  - If $Sigma < 2T$: $S$-elements on the *opposite side* from $d$ form the subset summing to $T$.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_elements`], [$n + 1$ (worst case; $n$ when $Sigma = 2T$)],
)

*Example.* $S = {1, 5, 6, 8}$, $T = 11$, $Sigma = 20 < 22 = 2T$, so $d = 2$.

Partition instance: $S' = {1, 5, 6, 8, 2}$, $Sigma' = 22$, half $= 11$.

Balanced partition: ${5, 6}$ (sum 11) vs.~${1, 8, 2}$ (sum 11). Padding $d = 2$ is on the ${1, 8, 2}$ side. Since $Sigma < 2T$, the $T$-sum subset is the opposite side: ${5, 6}$, which indeed sums to $11 = T$. $checkmark$
