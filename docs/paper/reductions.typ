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
  #text(size: 11pt)[Problem Reductions Library]
  #v(0.3em)
  #text(size: 10pt, style: "italic")[CodingThrust · github.com/CodingThrust/problem-reductions]
  #v(1em)
]

// Abstract
#block(width: 100%, inset: (x: 2em, y: 1em))[
  *Abstract.* This document provides formal mathematical definitions for computational problems and polynomial-time reductions implemented in the `problemreductions` library. We define each problem type, state reduction theorems, and provide constructive proofs that preserve solution structure. All reductions are implemented in Rust with verified correctness through closed-loop testing.
]

#v(1em)


= Introduction

This document serves as a mathematical reference for the reduction rules implemented in the problem reductions library. Each reduction $A arrow.long B$ transforms an instance of problem $A$ into an instance of problem $B$ such that:

+ The transformation is computable in polynomial time.
+ Solutions to $B$ can be efficiently extracted back to solutions of $A$.
+ Optimality is preserved: optimal solutions map to optimal solutions.

== Reduction Graph

The following diagram shows all #graph-data.edges.len() implemented reductions connecting #graph-data.nodes.len() problem types. Bidirectional arrows indicate reductions exist in both directions.

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
  caption: [Reduction graph with automatic layered layout. Colors indicate problem categories.]
) <fig:reduction-graph>

#v(1em)

= Problem Definitions <sec:problems>

== Graph Problems

#definition("Independent Set")[
  Given an undirected graph $G = (V, E)$ with vertex weights $w: V -> RR$, the *Maximum Weighted Independent Set* problem asks to find a subset $S subset.eq V$ such that:
  - No two vertices in $S$ are adjacent: $forall u, v in S: (u, v) in.not E$
  - The total weight $sum_(v in S) w(v)$ is maximized
]

#definition("Vertex Cover")[
  Given an undirected graph $G = (V, E)$ with vertex weights $w: V -> RR$, the *Minimum Weighted Vertex Cover* problem asks to find a subset $S subset.eq V$ such that:
  - Every edge is covered: $forall (u, v) in E: u in S or v in S$
  - The total weight $sum_(v in S) w(v)$ is minimized
]

#definition("Max-Cut")[
  Given an undirected graph $G = (V, E)$ with edge weights $w: E -> RR$, the *Maximum Cut* problem asks to find a partition $(S, overline(S))$ of $V$ that maximizes:
  $ "cut"(S) = sum_((u,v) in E: u in S, v in overline(S)) w(u, v) $
]

#definition("Graph Coloring")[
  Given an undirected graph $G = (V, E)$ and $k$ colors, the *$k$-Coloring* problem asks to find an assignment $c: V -> {1, ..., k}$ that minimizes the number of monochromatic edges:
  $ |{(u, v) in E : c(u) = c(v)}| $
]

#definition("Dominating Set")[
  Given an undirected graph $G = (V, E)$ with vertex weights $w: V -> RR$, the *Minimum Weighted Dominating Set* problem asks to find $S subset.eq V$ such that:
  - Every vertex is dominated: $forall v in V: v in S or exists u in S: (u, v) in E$
  - The total weight $sum_(v in S) w(v)$ is minimized
]

#definition("Matching")[
  Given an undirected graph $G = (V, E)$ with edge weights $w: E -> RR$, the *Maximum Weighted Matching* problem asks to find $M subset.eq E$ such that:
  - No two edges share a vertex: $forall e_1, e_2 in M: e_1 inter e_2 = emptyset$
  - The total weight $sum_(e in M) w(e)$ is maximized
]

== Set Problems

#definition("Set Packing")[
  Given a universe $U$ and a collection $cal(S) = {S_1, ..., S_m}$ where $S_i subset.eq U$, with weights $w: cal(S) -> RR$, the *Maximum Weighted Set Packing* problem asks to find $cal(P) subset.eq cal(S)$ such that:
  - Sets are pairwise disjoint: $forall S_i, S_j in cal(P): S_i inter S_j = emptyset$
  - The total weight $sum_(S in cal(P)) w(S)$ is maximized
]

#definition("Set Covering")[
  Given a universe $U$ and a collection $cal(S) = {S_1, ..., S_m}$ where $S_i subset.eq U$, with weights $w: cal(S) -> RR$, the *Minimum Weighted Set Cover* problem asks to find $cal(C) subset.eq cal(S)$ such that:
  - The universe is covered: $union.big_(S in cal(C)) S = U$
  - The total weight $sum_(S in cal(C)) w(S)$ is minimized
]

== Optimization Problems

#definition("Spin Glass (Ising Model)")[
  Given $n$ spin variables $s_i in {-1, +1}$, coupling strengths $J_(i j) in RR$, and external fields $h_i in RR$, the *Spin Glass* problem asks to minimize the Hamiltonian:
  $ H(bold(s)) = -sum_((i,j)) J_(i j) s_i s_j - sum_i h_i s_i $
]

#definition("QUBO")[
  Given $n$ binary variables $x_i in {0, 1}$ and a matrix $Q in RR^(n times n)$, the *Quadratic Unconstrained Binary Optimization* problem asks to minimize:
  $ f(bold(x)) = sum_(i,j) Q_(i j) x_i x_j = bold(x)^top Q bold(x) $
]

== Satisfiability Problems

#definition("Boolean Satisfiability (SAT)")[
  Given a CNF formula $phi = C_1 and C_2 and ... and C_m$ over $n$ Boolean variables, where each clause $C_j = (ell_(j 1) or ell_(j 2) or ... or ell_(j k_j))$ is a disjunction of literals, find an assignment $bold(x) in {0, 1}^n$ such that $phi(bold(x)) = 1$.
]

#definition("$k$-SAT")[
  A restriction of SAT where every clause contains exactly $k$ literals.
]

#definition("Circuit-SAT")[
  Given a Boolean circuit $C$ with gates ${and, or, not, xor}$ and input variables $x_1, ..., x_n$, find an assignment such that the circuit outputs TRUE.
]

#definition("Integer Factoring")[
  Given a composite integer $N$ and bit sizes $m, n$, find integers $p in [2, 2^m - 1]$ and $q in [2, 2^n - 1]$ such that $p times q = N$.
]

= Reduction Theorems <sec:reductions>

== Graph Problem Reductions

#theorem[
  *Independent Set $arrow.l.r$ Vertex Cover.* For any graph $G = (V, E)$, a set $S subset.eq V$ is an independent set if and only if $V backslash S$ is a vertex cover. Furthermore:
  $ |"max IS"| + |"min VC"| = |V| $
]

#proof[
  ($arrow.r.double$) Let $S$ be an independent set. For any edge $(u, v) in E$, at most one of $u, v$ is in $S$ (by independence). Thus at least one is in $V backslash S$, so $V backslash S$ covers all edges.

  ($arrow.l.double$) Let $C$ be a vertex cover. For any $u, v in V backslash C$, we must have $(u, v) in.not E$ (otherwise $C$ wouldn't cover $(u, v)$). Thus $V backslash C$ is an independent set.

  The size relation follows immediately from $|S| + |V backslash S| = |V|$.
]

#theorem[
  *Independent Set $arrow.r$ Set Packing.* Given graph $G = (V, E)$, construct set packing instance where:
  - Universe $U = E$ (the edges)
  - For each vertex $v in V$: set $S_v = {e in E : v in e}$ (incident edges)
  - Weight $w(S_v) = w(v)$

  Then $I subset.eq V$ is an independent set iff ${S_v : v in I}$ is a valid packing.
]

#proof[
  ($arrow.r.double$) If $I$ is independent, no two vertices in $I$ share an edge, so their incident edge sets are disjoint.

  ($arrow.l.double$) If ${S_v : v in I}$ is a packing, then for any $u, v in I$ with $u eq.not v$, we have $S_u inter S_v = emptyset$, meaning $(u, v) in.not E$.
]

#theorem[
  *Vertex Cover $arrow.r$ Set Covering.* Given graph $G = (V, E)$, construct set covering instance where:
  - Universe $U = E$ (edges indexed $0, ..., m-1$)
  - For each vertex $v in V$: set $S_v = {i : e_i "is incident to" v}$
  - Weight $w(S_v) = w(v)$

  Then $C subset.eq V$ is a vertex cover iff ${S_v : v in C}$ covers $U$.
]

#proof[
  A vertex cover $C$ covers every edge, meaning every edge has at least one endpoint in $C$. This is equivalent to ${S_v : v in C}$ covering all edge indices.
]

#theorem[
  *Matching $arrow.r$ Set Packing.* Given graph $G = (V, E)$, construct set packing where:
  - Universe $U = V$ (the vertices)
  - For each edge $e = (u, v) in E$: set $S_e = {u, v}$
  - Weight $w(S_e) = w(e)$

  Then $M subset.eq E$ is a matching iff ${S_e : e in M}$ is a valid packing.
]

#proof[
  A matching has no two edges sharing a vertex. In set terms, no two sets $S_(e_1), S_(e_2)$ for $e_1, e_2 in M$ share an element. This is exactly the packing condition.
]

== Optimization Reductions

#theorem[
  *Spin Glass $arrow.l.r$ QUBO.* The substitution $s_i = 2x_i - 1$ (where $x_i in {0, 1}$) transforms between spin ($plus.minus 1$) and binary ($0, 1$) variables:
  $ H_"SG"(bold(s)) = H_"QUBO"(bold(x)) + "const" $
]

#proof[
  Substituting $s_i = 2x_i - 1$ into the spin glass Hamiltonian:
  $ H_"SG" &= -sum_(i,j) J_(i j) s_i s_j - sum_i h_i s_i \
          &= -sum_(i,j) J_(i j) (2x_i - 1)(2x_j - 1) - sum_i h_i (2x_i - 1) $
  Expanding yields QUBO with $Q_(i j) = -4J_(i j)$ and $Q_(i i) = 2sum_j J_(i j) - 2h_i$ plus a constant.
]

#theorem[
  *Spin Glass $arrow.l.r$ Max-Cut.* For zero external fields ($h_i = 0$), the ground state corresponds to the maximum cut. For non-zero fields, an ancilla spin $s_0$ connected to all spins with $J_(0 i) = -h_i$ is added.
]

#proof[
  For antiferromagnetic couplings ($J_(i j) > 0$), minimizing $-sum J_(i j) s_i s_j$ maximizes cut weight since $s_i s_j = -1$ when spins are anti-aligned (different partitions).
]

== SAT-Based Reductions

#theorem[
  *SAT $arrow.l.r$ $k$-SAT.* Any SAT formula can be converted to $k$-SAT ($k >= 3$) in polynomial time by padding small clauses and splitting large clauses using auxiliary variables.
]

#proof[
  For clause $(ell_1 or ... or ell_m)$ with $m > k$: introduce auxiliary variables $y_1, ..., y_(m-k)$ and replace with clauses of size $k$ that preserve satisfiability through propagation.
]

#theorem[
  *SAT $arrow.r$ Independent Set.* Given CNF $phi$ with $m$ clauses, construct graph $G$:
  - For each literal $ell$ in clause $C_j$: vertex $v_(ell, j)$
  - Edges within clauses and between conflicting literals

  Then $phi$ is satisfiable iff $G$ has an independent set of size $m$.
]

#proof[
  A satisfying assignment selects one true literal per clause. The corresponding vertices form an independent set of size $m$: within-clause edges are avoided (one per clause), conflict edges are avoided (consistent assignment).
]

#theorem[
  *SAT $arrow.r$ Graph Coloring.* Using 3-coloring with special vertices (TRUE, FALSE, AUX) and OR-gadgets for clause satisfaction.
]

#theorem[
  *SAT $arrow.r$ Dominating Set.* Variables and clauses become vertices; literal vertices dominate their clause vertices.
]

== Circuit Reductions

#theorem[
  *Circuit-SAT $arrow.r$ Spin Glass.* Each logic gate maps to a spin glass gadget encoding its truth table as energy penalties. Ground states correspond to valid gate configurations.
]

#theorem[
  *Factoring $arrow.r$ Circuit-SAT.* Construct a multiplier circuit computing $p times q$ with output constrained to equal $N$. Satisfiability implies factorization exists.
]

= Classification of Reductions <sec:classification>

We classify the implemented reductions based on their conceptual complexity.

== Trivial Reductions

These reductions are straightforward transformations where the mapping is obvious and direct:

#table(
  columns: (auto, auto, auto),
  inset: 6pt,
  align: left,
  [*Reduction*], [*Transformation*], [*Complexity*],
  [IS $arrow.l.r$ VC], [Complement: $S$ is IS iff $V backslash S$ is VC], [$O(|V|)$],
  [IS $arrow.r$ SetPacking], [Vertex $v arrow.bar$ set of incident edges], [$O(|V| + |E|)$],
  [Matching $arrow.r$ SetPacking], [Edge $(u,v) arrow.bar$ set ${u, v}$], [$O(|E|)$],
  [VC $arrow.r$ SetCovering], [Vertex $v arrow.bar$ set of incident edge indices], [$O(|V| + |E|)$],
  [$k$-SAT $arrow.r$ SAT], [Direct embedding (trivial)], [$O(1)$],
  [QUBO $arrow.l.r$ SpinGlass], [Variable substitution $s = 2x - 1$], [$O(n^2)$],
)

== Non-Trivial Reductions

These reductions require sophisticated constructions, gadgets, or non-obvious insights:

#table(
  columns: (auto, auto, auto),
  inset: 6pt,
  align: left,
  [*Reduction*], [*Technique*], [*Key Insight*],
  [SAT $arrow.r$ $k$-SAT], [Clause splitting], [Auxiliary variables preserve satisfiability],
  [SAT $arrow.r$ IS], [Conflict graph], [Clique + complement edges],
  [SAT $arrow.r$ Coloring], [OR-gadgets], [5-vertex gadgets encode disjunction],
  [SAT $arrow.r$ DominatingSet], [Variable triangles], [Literal vertices dominate clauses],
  [CircuitSAT $arrow.r$ SpinGlass], [Logic gadgets], [Ground states = valid gate I/O],
  [Factoring $arrow.r$ CircuitSAT], [Multiplier circuit], [Array multiplier with full adders],
  [SpinGlass $arrow.l.r$ MaxCut], [Ancilla vertex], [External fields become edges],
)

= Detailed Survey of Non-Trivial Reductions <sec:survey>

== SAT → Independent Set (Karp's Reduction)

*Reference:* @karp1972

*Input:* CNF formula $phi = C_1 and C_2 and ... and C_m$ over variables $x_1, ..., x_n$, where each clause $C_j = (ell_(j,1) or ell_(j,2) or ... or ell_(j,k_j))$.

*Output:* Graph $G = (V, E)$ with unit weights.

*Construction:*

_Vertices:_ For each literal $ell_(j,i)$ in clause $C_j$, create vertex $v_(j,i)$. Total: $|V| = sum_(j=1)^m k_j$.

_Edges (two types):_
1. *Clause edges:* For each clause $C_j$, add edges between all pairs of vertices in that clause:
   $ E_"clause" = {(v_(j,i), v_(j,i')) : 1 <= i < i' <= k_j, 1 <= j <= m} $
2. *Conflict edges:* For vertices representing complementary literals in different clauses:
   $ E_"conflict" = {(v_(j,i), v_(j',i')) : j != j', ell_(j,i) = not ell_(j',i')} $

*Solution extraction:* Given IS solution $S subset.eq V$:
- For each $v_(j,i) in S$ representing literal $x_k$: set $x_k = 1$
- For each $v_(j,i) in S$ representing literal $not x_k$: set $x_k = 0$
- Unassigned variables default to 0

*Correctness:* $phi$ is satisfiable iff $G$ has an independent set of size $m$.

== SAT → Graph 3-Coloring

*Reference:* @garey1979

*Input:* CNF formula $phi$ with $n$ variables and $m$ clauses.

*Output:* Graph $G = (V, E)$ for 3-coloring.

*Construction:*

_Base vertices (indices 0, 1, 2):_
- Vertex 0: TRUE, Vertex 1: FALSE, Vertex 2: AUX
- Triangle edges: $(0,1), (0,2), (1,2)$

_Variable vertices:_ For each variable $x_i$ ($i = 0, ..., n-1$):
- Positive literal vertex: $"pos"_i = 3 + i$
- Negative literal vertex: $"neg"_i = 3 + n + i$
- Edges: $("pos"_i, 2), ("neg"_i, 2), ("pos"_i, "neg"_i)$

_Clause gadgets:_ For clause $(ell_1 or ell_2 or ... or ell_k)$:
- Let $v_1$ = vertex for $ell_1$
- For $i = 2, ..., k$: let $v_i$ = vertex for $ell_i$, then $v_1 := "OR-gadget"(v_1, v_i)$
- Force output TRUE: add edges $(v_1, 1), (v_1, 2)$

_OR-gadget$(a, b)$:_ Creates 5 new vertices and returns output vertex $o$:
- New vertices: $"anc"_1, "anc"_2, "ent"_1, "ent"_2, o$
- Edges: $("anc"_1, 0), ("anc"_1, "anc"_2), ("anc"_2, a), ("anc"_2, b),$
  $("ent"_1, "ent"_2), (o, "anc"_1), (a, "ent"_2), (b, "ent"_1), ("ent"_1, o), ("ent"_2, o), (o, 2)$

*Solution extraction:* Let $c_"TRUE"$ = color of vertex 0. For variable $x_i$: set $x_i = 1$ if color of $"pos"_i$ equals $c_"TRUE"$, else $x_i = 0$.

== SAT → Dominating Set

*Reference:* @garey1979

*Input:* CNF formula $phi$ with $n$ variables and $m$ clauses.

*Output:* Graph $G = (V, E)$ with unit weights.

*Construction:*

_Variable gadgets:_ For each variable $x_i$ ($i = 0, ..., n-1$), create triangle:
- $"pos"_i = 3i$: positive literal vertex
- $"neg"_i = 3i + 1$: negative literal vertex
- $"dum"_i = 3i + 2$: dummy vertex
- Edges: $("pos"_i, "neg"_i), ("neg"_i, "dum"_i), ("dum"_i, "pos"_i)$

_Clause vertices:_ For each clause $C_j$ ($j = 0, ..., m-1$):
- Create vertex $c_j = 3n + j$
- For each literal $ell$ in $C_j$: if $ell = x_i$, add edge $("pos"_i, c_j)$; if $ell = not x_i$, add edge $("neg"_i, c_j)$

*Solution extraction:* For each variable $x_i$: if $"pos"_i$ is in dominating set, set $x_i = 1$; if $"neg"_i$ is in dominating set, set $x_i = 0$.

*Correctness:* $phi$ is satisfiable iff $G$ has dominating set of size $n$.

== SAT ↔ $k$-SAT (Clause Transformation)

*Reference:* @cook1971, @garey1979

*SAT → $k$-SAT transformation:*

_Case 1: Small clause_ $C = (ell_1 or ... or ell_r)$ with $r < k$:

Introduce auxiliary variable $y$. Replace $C$ with:
$ (ell_1 or ... or ell_r or y or not y or y or ... ) $
padding to exactly $k$ literals by repeating $y$ and $not y$ alternately.

_Case 2: Large clause_ $C = (ell_1 or ... or ell_r)$ with $r > k$:

Introduce auxiliary variables $y_1, y_2, ..., y_(r-k)$. Replace $C$ with:
$ &(ell_1 or ... or ell_(k-1) or y_1) and \
  &(not y_1 or ell_k or ... or ell_(2k-3) or y_2) and \
  &(not y_2 or ell_(2k-2) or ... or ell_(3k-5) or y_3) and \
  &... and \
  &(not y_(r-k) or ell_(r-k+2) or ... or ell_r) $

*Solution extraction:* Return only the original variable assignments; auxiliary variables are discarded.

*$k$-SAT → SAT:* Direct embedding (identity transformation).

== CircuitSAT → Spin Glass (Logic Gadgets)

*Reference:* @whitfield2012, @lucas2014, @nguyen2023

*Spin convention:* Configuration $sigma in {0, 1}$ maps to spin $s = 2sigma - 1 in {-1, +1}$.

*Hamiltonian:* $H(bold(s)) = -sum_((i,j)) J_(i j) s_i s_j - sum_i h_i s_i$

*Gate gadgets:* Each gadget has ground states exactly matching the gate's truth table.

#table(
  columns: (auto, auto, auto, auto),
  inset: 5pt,
  align: left,
  [*Gate*], [*Spins*], [*$J$ (edges)*], [*$h$ (fields)*],
  [AND], [0,1→2], [$J_(01)=1, J_(02)=-2, J_(12)=-2$], [$h_0=-1, h_1=-1, h_2=2$],
  [OR], [0,1→2], [$J_(01)=1, J_(02)=-2, J_(12)=-2$], [$h_0=1, h_1=1, h_2=-2$],
  [NOT], [0→1], [$J_(01)=1$], [$h_0=0, h_1=0$],
  [XOR], [0,1→2 (aux:3)], [$J_(01)=1, J_(02)=-1, J_(03)=-2,$], [$h_0=-1, h_1=-1,$],
  [], [], [$J_(12)=-1, J_(13)=-2, J_(23)=2$], [$h_2=1, h_3=2$],
)

*Circuit compilation:*
1. Allocate spin index for each circuit variable
2. For each gate, instantiate gadget with mapped spin indices
3. Merge Hamiltonians: $J_"total" = sum_"gadgets" J$, $h_"total" = sum_"gadgets" h$
4. For output constraints $y = "expr"$: add ferromagnetic coupling $J_(y,"expr") = -4$

*Solution extraction:* Map spin configuration back: $sigma_i = (s_i + 1) / 2$.

== Factoring → Circuit-SAT (Multiplier Circuit)

*Note:* This reduction uses a classical array multiplier circuit construction. It is a folklore result based on the observation that multiplication circuits can be inverted via SAT solving. The construction is standard in hardware verification and cryptographic research. No single canonical reference exists for this approach.

*Input:* Target $N$, bit sizes $m$ (for $p$) and $n$ (for $q$).

*Output:* Circuit with variables $p_1...p_m$, $q_1...q_n$, and internal signals.

*Construction (array multiplier):*

_Variables:_ Input bits $p_i, q_j$ (little-endian), sum signals $s_(i,j)$, carry signals $c_(i,j)$, ancillas.

_Full adder cell_ at position $(i,j)$: Computes $s_(i,j) + 2 c_(i,j) = (p_i and q_j) + s_"prev" + c_"prev"$

For each cell, create 6 assignments:
$ a &:= p_i and q_j \
  t_1 &:= a "xor" s_"prev" \
  s_(i,j) &:= t_1 "xor" c_"prev" \
  t_2 &:= t_1 and c_"prev" \
  t_3 &:= a and s_"prev" \
  c_(i,j) &:= t_2 or t_3 $

_Output constraints:_ For each product bit $M_k$ ($k = 1, ..., m+n$):
$ M_k := "bit"_k (N) $
where $"bit"_k (N)$ is the $k$-th bit of $N$ (constant TRUE or FALSE).

*Solution extraction:* Read $p = sum_(i=1)^m p_i dot 2^(i-1)$ and $q = sum_(j=1)^n q_j dot 2^(j-1)$.

== SpinGlass ↔ MaxCut (Ancilla Technique)

*Reference:* @barahona1982, @lucas2014

*MaxCut → SpinGlass:*

Given graph $G = (V, E)$ with weights $w_(i j)$:
- $n = |V|$ spins
- $J_(i j) = w_(i j)$ for $(i,j) in E$
- $h_i = 0$ for all $i$

MaxCut: $max sum_((i,j) in E) w_(i j) [s_i != s_j]$

SpinGlass: $min -sum_((i,j)) J_(i j) s_i s_j$ where $s_i s_j = -1$ when $s_i != s_j$

Relationship: $"MaxCut" = 1/2 sum_((i,j)) w_(i j) - 1/2 H_"SG"$

*SpinGlass → MaxCut:*

Given SpinGlass with $J_(i j)$ and $h_i$:

_Case 1:_ All $h_i = 0$. Direct mapping: $w_(i j) = J_(i j)$, $n' = n$.

_Case 2:_ Some $h_i != 0$. Add ancilla vertex $a = n$:
- $n' = n + 1$ vertices
- $w_(i j) = J_(i j)$ for original edges
- $w_(i,a) = h_i$ for each $i$ with $h_i != 0$

*Solution extraction:*
- If no ancilla: return solution directly
- If ancilla present: let $sigma_a$ = ancilla value. If $sigma_a = 1$, flip all bits: $sigma'_i = 1 - sigma_i$. Return $sigma'_0, ..., sigma'_(n-1)$ (excluding ancilla).

#bibliography("references.bib", style: "ieee")
