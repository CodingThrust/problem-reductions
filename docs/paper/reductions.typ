// Problem Reductions: A Mathematical Reference

#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge

// Page setup with narrow margins
#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

// Theorem counter and environments
#let theorem-counter = counter("theorem")

#let theorem(body) = block(
  width: 100%,
  inset: (x: 0em, y: 0.5em),
  {
    theorem-counter.step()
    [*Theorem #context theorem-counter.display().* ]
    body
  }
)

#let proof(body) = block(
  width: 100%,
  inset: (x: 0em, y: 0.3em),
  [_Proof._ #body #h(1fr) $square$]
)

// Custom definition environment
#let definition(title, body) = block(
  width: 100%,
  inset: (x: 1em, y: 0.8em),
  fill: rgb("#f8f8f8"),
  stroke: (left: 2pt + rgb("#4a86e8")),
  [*#title.* #body]
)

// Load the reduction graph from JSON
#let graph-data = json("reduction_graph.json")

// Category colors
#let category-colors = (
  "graph": rgb("#e0ffe0"),
  "set": rgb("#ffe0e0"),
  "optimization": rgb("#ffffd0"),
  "satisfiability": rgb("#e0e0ff"),
  "specialized": rgb("#ffe0f0"),
  "other": rgb("#f0f0f0"),
)

// Function to get color for a category
#let get-color(category) = {
  category-colors.at(category, default: rgb("#f0f0f0"))
}

// Title
#align(center)[
  #text(size: 16pt, weight: "bold")[Problem Reductions: Models and Transformations]
  #v(0.5em)
  #text(size: 11pt)[Technical Documentation]
  #v(0.3em)
  #text(size: 10pt, style: "italic")[github.com/CodingThrust/problem-reductions]
  #v(1em)
]

// Abstract
#block(width: 100%, inset: (x: 2em, y: 1em))[
  *Abstract.* We present formal definitions for computational problems and polynomial-time reductions implemented in the `problemreductions` library. For each problem type, we state reduction theorems with constructive proofs that preserve solution structure. All reductions are implemented in Rust with correctness verified through closed-loop testing.
]

#v(1em)


= Introduction

This document provides a mathematical reference for the reduction rules in the problem reductions library. A reduction $A arrow.long B$ transforms instances of problem $A$ into instances of problem $B$ satisfying three properties:

+ The transformation is computable in polynomial time.
+ Solutions to $B$ can be efficiently mapped back to solutions of $A$.
+ Optimality is preserved: optimal solutions map to optimal solutions.

== Reduction Graph

@fig:reduction-graph illustrates the #graph-data.edges.len() implemented reductions connecting #graph-data.nodes.len() problem types. Bidirectional arrows indicate that reductions exist in both directions.

// Compute bounding box for auto-scaling
#let x-vals = graph-data.nodes.map(n => n.x)
#let y-vals = graph-data.nodes.map(n => n.y)
#let x-min = calc.min(..x-vals)
#let x-max = calc.max(..x-vals)
#let y-min = calc.min(..y-vals)
#let y-max = calc.max(..y-vals)
#let x-range = x-max - x-min
#let y-range = y-max - y-min

// Normalize positions to [0, 1] range, then scale to fit
#let normalize(val, min-val, range) = {
  if range == 0 { 0.5 } else { (val - min-val) / range }
}

// Target size for the diagram (compact to fit 50% page width)
#let target-width = 4.0  // in diagram units
#let target-height = 3.0

#figure(
  box(
    width: 50%,
    align(center,
      diagram(
        spacing: (18mm, 12mm),
        node-stroke: 0.6pt,
        edge-stroke: 0.6pt,
        node-corner-radius: 2pt,
        node-inset: 3pt,

        // Generate nodes with normalized positions
        ..graph-data.nodes.map(n => {
          let color = get-color(n.category)
          let nx = normalize(n.x, x-min, x-range) * target-width
          let ny = normalize(n.y, y-min, y-range) * target-height
          node((nx, ny), text(size: 7pt)[#n.label], fill: color, name: label(n.id))
        }),

        // Generate edges from JSON
        ..graph-data.edges.map(e => {
          let arrow = if e.bidirectional { "<|-|>" } else { "-|>" }
          edge(label(e.source), label(e.target), arrow)
        }),
      )
    )
  ),
  caption: [Reduction graph. Node colors indicate problem categories: green (graph), red (set), yellow (optimization), blue (satisfiability), pink (specialized).]
) <fig:reduction-graph>

#v(1em)

= Problem Definitions <sec:problems>

== Graph Problems

#definition("Independent Set")[
  Given an undirected graph $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ maximizing $sum_(v in S) w(v)$ subject to $forall u, v in S: (u, v) in.not E$.
]

#definition("Vertex Cover")[
  Given an undirected graph $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ minimizing $sum_(v in S) w(v)$ subject to $forall (u, v) in E: u in S or v in S$.
]

#definition("Max-Cut")[
  Given an undirected graph $G = (V, E)$ with edge weights $w: E -> RR$, find a partition $(S, overline(S))$ of $V$ maximizing $"cut"(S) = sum_((u,v) in E: u in S, v in overline(S)) w(u, v)$.
]

#definition("Graph Coloring")[
  Given an undirected graph $G = (V, E)$ and $k$ colors, find $c: V -> {1, ..., k}$ minimizing $|{(u, v) in E : c(u) = c(v)}|$.
]

#definition("Dominating Set")[
  Given an undirected graph $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ minimizing $sum_(v in S) w(v)$ subject to $forall v in V: v in S or exists u in S: (u, v) in E$.
]

#definition("Matching")[
  Given an undirected graph $G = (V, E)$ with edge weights $w: E -> RR$, find $M subset.eq E$ maximizing $sum_(e in M) w(e)$ subject to $forall e_1, e_2 in M: e_1 inter e_2 = emptyset$.
]

== Set Problems

#definition("Set Packing")[
  Given universe $U$, collection $cal(S) = {S_1, ..., S_m}$ with $S_i subset.eq U$, and weights $w: cal(S) -> RR$, find $cal(P) subset.eq cal(S)$ maximizing $sum_(S in cal(P)) w(S)$ subject to $forall S_i, S_j in cal(P): S_i inter S_j = emptyset$.
]

#definition("Set Covering")[
  Given universe $U$, collection $cal(S) = {S_1, ..., S_m}$ with $S_i subset.eq U$, and weights $w: cal(S) -> RR$, find $cal(C) subset.eq cal(S)$ minimizing $sum_(S in cal(C)) w(S)$ subject to $union.big_(S in cal(C)) S = U$.
]

== Optimization Problems

#definition("Spin Glass (Ising Model)")[
  Given $n$ spins $s_i in {-1, +1}$, couplings $J_(i j) in RR$, and fields $h_i in RR$, minimize the Hamiltonian $H(bold(s)) = -sum_((i,j)) J_(i j) s_i s_j - sum_i h_i s_i$.
]

#definition("QUBO")[
  Given $n$ binary variables $x_i in {0, 1}$ and matrix $Q in RR^(n times n)$, minimize $f(bold(x)) = bold(x)^top Q bold(x)$.
]

== Satisfiability Problems

#definition("Boolean Satisfiability (SAT)")[
  Given CNF formula $phi = and.big_(j=1)^m C_j$ over $n$ variables, where $C_j = or.big_(i) ell_(j i)$ is a disjunction of literals, find $bold(x) in {0, 1}^n$ such that $phi(bold(x)) = 1$.
]

#definition("$k$-SAT")[
  SAT restricted to clauses containing exactly $k$ literals.
]

#definition("Circuit-SAT")[
  Given Boolean circuit $C$ with gates ${and, or, not, xor}$, find inputs $x_1, ..., x_n$ such that $C(bold(x)) = 1$.
]

#definition("Integer Factoring")[
  Given composite $N$ and bit sizes $m, n$, find $p in [2, 2^m - 1]$ and $q in [2, 2^n - 1]$ such that $p times q = N$.
]

= Reduction Theorems <sec:reductions>

== Graph Problem Reductions

#theorem[
  *(IS $arrow.l.r$ VC)* $S subset.eq V$ is an independent set iff $V backslash S$ is a vertex cover. Moreover, $|"max IS"| + |"min VC"| = |V|$.
]

#proof[
  ($arrow.r.double$) If $S$ is independent, then for any $(u, v) in E$, at most one endpoint lies in $S$, so $V backslash S$ covers all edges.
  ($arrow.l.double$) If $C$ is a cover, then for any $u, v in V backslash C$, we have $(u, v) in.not E$, so $V backslash C$ is independent.
]

#theorem[
  *(IS $arrow.r$ Set Packing)* Construct: $U = E$, $S_v = {e in E : v in e}$, $w(S_v) = w(v)$. Then $I subset.eq V$ is independent iff ${S_v : v in I}$ is a valid packing.
]

#proof[
  Independence implies disjoint incident edge sets. Conversely, disjoint edge sets imply no shared edges between vertices.
]

#theorem[
  *(VC $arrow.r$ Set Covering)* Construct: $U = {0, ..., |E|-1}$, $S_v = {i : e_i "incident to" v}$, $w(S_v) = w(v)$. Then $C$ is a vertex cover iff ${S_v : v in C}$ covers $U$.
]

#theorem[
  *(Matching $arrow.r$ Set Packing)* Construct: $U = V$, $S_e = {u, v}$ for $e = (u,v)$, $w(S_e) = w(e)$. Then $M$ is a matching iff ${S_e : e in M}$ is a valid packing.
]

== Optimization Reductions

#theorem[
  *(Spin Glass $arrow.l.r$ QUBO)* The substitution $s_i = 2x_i - 1$ transforms between spin and binary representations: $H_"SG"(bold(s)) = H_"QUBO"(bold(x)) + "const"$.
]

#proof[
  Expanding $H_"SG" = -sum_(i,j) J_(i j) (2x_i - 1)(2x_j - 1) - sum_i h_i (2x_i - 1)$ yields QUBO form with $Q_(i j) = -4J_(i j)$ and $Q_(i i) = 2sum_j J_(i j) - 2h_i$.
]

#theorem[
  *(Spin Glass $arrow.l.r$ Max-Cut)* For $h_i = 0$, ground states correspond to maximum cuts. For $h_i != 0$, an ancilla spin with $J_(0 i) = -h_i$ reduces to the zero-field case.
]

== SAT-Based Reductions

#theorem[
  *(SAT $arrow.l.r$ $k$-SAT)* Any SAT formula transforms to $k$-SAT ($k >= 3$) by padding small clauses and splitting large clauses with auxiliary variables.
]

#theorem[
  *(SAT $arrow.r$ IS)* @karp1972 Construct graph with vertices for each literal occurrence, clause edges (forming cliques), and conflict edges (complementary literals). Then $phi$ is satisfiable iff $G$ has an IS of size $m$.
]

#theorem[
  *(SAT $arrow.r$ 3-Coloring)* @garey1979 Construct graph with TRUE/FALSE/AUX triangle, variable vertices, and OR-gadgets enforcing clause satisfaction.
]

#theorem[
  *(SAT $arrow.r$ Dominating Set)* @garey1979 Construct graph with variable triangles and clause vertices connected to literal vertices.
]

== Circuit Reductions

#theorem[
  *(Circuit-SAT $arrow.r$ Spin Glass)* @whitfield2012 @lucas2014 Each gate maps to a gadget whose ground states encode valid I/O configurations.
]

#theorem[
  *(Factoring $arrow.r$ Circuit-SAT)* An array multiplier circuit with output constrained to $N$ is satisfiable iff $N$ factors within the given bit bounds.
]

= Classification <sec:classification>

We classify reductions by conceptual complexity.

== Trivial Reductions

Direct transformations with obvious mappings:

#table(
  columns: (auto, auto, auto),
  inset: 6pt,
  align: left,
  table.header([*Reduction*], [*Transformation*], [*Complexity*]),
  [IS $arrow.l.r$ VC], [Complement: $S arrow.bar V backslash S$], [$O(|V|)$],
  [IS $arrow.r$ SetPacking], [$v arrow.bar$ incident edges], [$O(|V| + |E|)$],
  [Matching $arrow.r$ SetPacking], [$(u,v) arrow.bar {u, v}$], [$O(|E|)$],
  [VC $arrow.r$ SetCovering], [$v arrow.bar$ incident edge indices], [$O(|V| + |E|)$],
  [$k$-SAT $arrow.r$ SAT], [Identity], [$O(1)$],
  [QUBO $arrow.l.r$ SpinGlass], [$s = 2x - 1$], [$O(n^2)$],
)

== Non-Trivial Reductions

Reductions requiring gadgets or sophisticated constructions:

#table(
  columns: (auto, auto, auto),
  inset: 6pt,
  align: left,
  table.header([*Reduction*], [*Technique*], [*Key Insight*]),
  [SAT $arrow.r$ $k$-SAT], [Clause splitting], [Auxiliary variables preserve satisfiability],
  [SAT $arrow.r$ IS], [Conflict graph], [Clique + complement edges],
  [SAT $arrow.r$ Coloring], [OR-gadgets], [5-vertex gadgets encode disjunction],
  [SAT $arrow.r$ DominatingSet], [Variable triangles], [Literals dominate clauses],
  [CircuitSAT $arrow.r$ SpinGlass], [Logic gadgets], [Ground states = valid I/O],
  [Factoring $arrow.r$ CircuitSAT], [Array multiplier], [Full adder cells],
  [SpinGlass $arrow.l.r$ MaxCut], [Ancilla vertex], [Fields become edges],
)

= Detailed Constructions <sec:survey>

== SAT $arrow.r$ Independent Set

*Reference:* @karp1972

*Construction.* Given CNF $phi = and.big_(j=1)^m C_j$ with $C_j = (ell_(j,1) or ... or ell_(j,k_j))$:

_Vertices:_ For each literal occurrence $ell_(j,i)$ in clause $C_j$, create vertex $v_(j,i)$. Total: $|V| = sum_(j=1)^m k_j$.

_Edges:_
$ E_"clause" &= {(v_(j,i), v_(j,i')) : 1 <= i < i' <= k_j} quad "(intra-clause cliques)" \
  E_"conflict" &= {(v_(j,i), v_(j',i')) : j != j', ell_(j,i) = overline(ell_(j',i'))} quad "(complementary literals)" $

*Solution extraction.* For $v_(j,i) in S$ representing $x_k$: set $x_k = 1$. For $v_(j,i) in S$ representing $overline(x_k)$: set $x_k = 0$. Default: $x_k = 0$.

*Correctness.* $phi$ satisfiable $arrow.l.r$ $G$ has IS of size $m$.

== SAT $arrow.r$ 3-Coloring

*Reference:* @garey1979

*Construction.* Given CNF $phi$ with $n$ variables:

_Base structure:_ Three vertices TRUE (0), FALSE (1), AUX (2) forming a triangle.

_Variable gadget:_ For variable $x_i$: create $"pos"_i = 3+i$ and $"neg"_i = 3+n+i$ with edges $("pos"_i, "AUX"), ("neg"_i, "AUX"), ("pos"_i, "neg"_i)$.

_Clause gadget:_ For clause $(ell_1 or ... or ell_k)$, apply OR-gadgets iteratively and force output to TRUE.

_OR-gadget$(a, b) arrow.bar o$:_ Five new vertices with edges encoding $o = a or b$ under 3-coloring constraints.

*Solution extraction.* Set $x_i = 1$ iff $"color"("pos"_i) = "color"("TRUE")$.

== SAT $arrow.r$ Dominating Set

*Reference:* @garey1979

*Construction.* Given CNF $phi$ with $n$ variables and $m$ clauses:

_Variable gadget:_ For $x_i$, create triangle: $"pos"_i = 3i$, $"neg"_i = 3i+1$, $"dum"_i = 3i+2$.

_Clause vertices:_ For clause $C_j$, create $c_j = 3n+j$. Connect $c_j$ to $"pos"_i$ if $x_i in C_j$, to $"neg"_i$ if $overline(x_i) in C_j$.

*Solution extraction.* Set $x_i = 1$ if $"pos"_i$ in dominating set; $x_i = 0$ if $"neg"_i$ in set.

*Correctness.* $phi$ satisfiable $arrow.l.r$ $G$ has dominating set of size $n$.

== SAT $arrow.l.r$ $k$-SAT

*Reference:* @cook1971, @garey1979

*SAT $arrow.r$ $k$-SAT:*

_Small clauses ($|C| < k$):_ Pad with auxiliary variable $y$: $(ell_1 or ... or ell_r or y or overline(y) or ...)$ to length $k$.

_Large clauses ($|C| > k$):_ Split with auxiliaries $y_1, ..., y_(r-k)$:
$ (ell_1 or ... or ell_(k-1) or y_1) and (overline(y_1) or ell_k or ... or y_2) and ... and (overline(y_(r-k)) or ell_(r-k+2) or ... or ell_r) $

*$k$-SAT $arrow.r$ SAT:* Identity embedding.

== CircuitSAT $arrow.r$ Spin Glass

*Reference:* @whitfield2012, @lucas2014, @nguyen2023

*Spin mapping:* $sigma in {0,1} arrow.bar s = 2sigma - 1 in {-1, +1}$.

*Gate gadgets:* Hamiltonians with ground states matching truth tables.

#table(
  columns: (auto, auto, auto),
  inset: 5pt,
  align: left,
  table.header([*Gate*], [*Couplings $J$*], [*Fields $h$*]),
  [AND], [$J_(01)=1, J_(02)=J_(12)=-2$], [$h_0=h_1=-1, h_2=2$],
  [OR], [$J_(01)=1, J_(02)=J_(12)=-2$], [$h_0=h_1=1, h_2=-2$],
  [NOT], [$J_(01)=1$], [$h_0=h_1=0$],
  [XOR], [$J_(01)=1, J_(02)=J_(12)=-1, J_(03)=J_(13)=-2, J_(23)=2$], [$h_0=h_1=-1, h_2=1, h_3=2$],
)

*Compilation:* Allocate spins per variable, instantiate gadgets, merge Hamiltonians additively.

== Factoring $arrow.r$ Circuit-SAT

*Note:* This is a folklore construction standard in hardware verification. No canonical reference exists.

*Construction.* Build array multiplier computing $p times q$:

_Full adder cell $(i,j)$:_ Computes $s_(i,j) + 2c_(i,j) = (p_i and q_j) + s_"prev" + c_"prev"$ via:
$ a := p_i and q_j, quad t_1 := a xor s_"prev", quad s_(i,j) := t_1 xor c_"prev" $
$ t_2 := t_1 and c_"prev", quad t_3 := a and s_"prev", quad c_(i,j) := t_2 or t_3 $

_Output constraint:_ Force product bits to match $N$: $M_k := "bit"_k(N)$.

*Solution extraction.* $p = sum_i p_i 2^(i-1)$, $q = sum_j q_j 2^(j-1)$.

== SpinGlass $arrow.l.r$ MaxCut

*Reference:* @barahona1982, @lucas2014

*MaxCut $arrow.r$ SpinGlass:* Set $J_(i j) = w_(i j)$, $h_i = 0$. Relationship: $"MaxCut" = frac(1,2) sum w_(i j) - frac(1,2) H_"SG"$.

*SpinGlass $arrow.r$ MaxCut:*
- If $h_i = 0$ for all $i$: direct mapping $w_(i j) = J_(i j)$.
- Otherwise: add ancilla $a$ with $w_(i,a) = h_i$ for each $h_i != 0$.

*Solution extraction.* Without ancilla: identity. With ancilla: if $sigma_a = 1$, flip all spins before removing ancilla.

#bibliography("references.bib", style: "ieee")
