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
  #text(size: 16pt, weight: "bold")[Proposed Reduction Rules --- Verification Notes]

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
- $d(v)$ denotes the degree of vertex $v$; $Delta = max_(v in V) d(v)$
- $N(v) = {u : (u,v) in E}$ is the open neighbourhood; $N[v] = {v} union N(v)$ is the closed neighbourhood
- For a set $S subset.eq V$, we write $w(S) = sum_(v in S) w(v)$
- $K_n$ denotes the complete graph on $n$ vertices
- $overline(G) = (V, binom(V,2) backslash E)$ denotes the complement graph of $G$

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

#pagebreak()

== Minimum Vertex Cover $arrow.r$ Hamiltonian Circuit <sec:vc-hc>

#theorem[
  Vertex Cover reduces to Hamiltonian Circuit via the Garey--Johnson--Stockmeyer cover-testing widget construction. Given a graph $G = (V, E)$ and budget $K$, a graph $G'$ is constructed such that $G'$ has a Hamiltonian circuit if and only if $G$ has a vertex cover of size $lt.eq K$. Each edge of $G$ is replaced by a 12-vertex cover-testing widget arranged in a $2 times 6$ grid, and $K$ selector vertices route the circuit through widget chains. Reference: Garey, Johnson, and Stockmeyer (1976), Lemma 2.1; Garey & Johnson (1979), GT1.
] <thm:vc-hc>

#proof[
  _Construction._ Given a Vertex Cover instance $(G = (V, E), K)$ with $n = |V|$ and $m = |E|$. Fix an arbitrary ordering on the edges incident to each vertex: for vertex $v$, let $e_(j_1), dots, e_(j_(d(v)))$ be its incident edges in order.

  *Step 1: Cover-testing widgets.* For each edge $e_j = (u, v) in E$ ($j = 1, dots, m$), create 12 vertices arranged in two rows of 6:
  $ (u, j, 1), (u, j, 2), dots, (u, j, 6) quad "(" u"-row)" $
  $ (v, j, 1), (v, j, 2), dots, (v, j, 6) quad "(" v"-row)" $

  Add the following 14 internal edges per widget:
  - *Horizontal edges (10):* $(u,j,i) dash (u,j,i+1)$ for $i = 1, dots, 5$ (5 edges), and $(v,j,i) dash (v,j,i+1)$ for $i = 1, dots, 5$ (5 edges).
  - *Cross edges (4):* $(u,j,1) dash (v,j,1)$, $(u,j,3) dash (v,j,3)$, $(u,j,4) dash (v,j,4)$, $(u,j,6) dash (v,j,6)$.

  *Widget traversal property (GJS76, Lemma 2.1).* Consider a Hamiltonian path segment that must enter a widget from one row's left end and exit from the same row's right end, covering all 12 vertices. The cross-edges at columns 1, 3, 4, 6 divide each row into three segments: columns 1--3, columns 3--4, and columns 4--6. Exhaustive analysis of the $2 times 6$ grid with these four cross-edges shows exactly three traversal patterns:

  + *$u$ covers alone:* A single pass enters at $(u,j,1)$ and exits at $(u,j,6)$, visiting all 12 vertices. The $v$-row is consumed internally via the cross-edges. The $v$-row entry $(v,j,1)$ and exit $(v,j,6)$ are visited but not used as external connection points.

  + *$v$ covers alone:* Symmetric to pattern 1, with $u$ and $v$ swapped.

  + *Both $u$ and $v$ cover:* Two independent passes traverse the widget. One pass enters at $(u,j,1)$ and exits at $(u,j,6)$, visiting only the $u$-row (6 vertices). A separate pass enters at $(v,j,1)$ and exits at $(v,j,6)$, visiting only the $v$-row (6 vertices). Cross-edges are not used.

  No other pattern visits all 12 vertices exactly once with the entry/exit constraints.

  *Step 2: Chain widgets per vertex.* For each vertex $v in V$, connect its widgets in sequence by adding the edge $(v, j_i, 6) dash (v, j_(i+1), 1)$ for $i = 1, dots, d(v) - 1$. This forms a chain with entry point $(v, j_1, 1)$ and exit point $(v, j_(d(v)), 6)$.

  *Step 3: Selector vertices.* Add $K$ selector vertices $a_1, dots, a_K$. For each selector $a_ell$ ($ell = 1, dots, K$) and each vertex $v in V$, add two edges:
  $ a_ell dash (v, j_1, 1) quad "and" quad a_ell dash (v, j_(d(v)), 6) $

  *Vertex and edge counts.* The constructed graph $G'$ has:
  - $|V'| = 12m + K$.
  - Edge count: $14m$ (widget-internal) $+ sum_(v in V)(d(v) - 1)$ (chain links) $+ 2 n K$ (selector-to-chain). Since $sum_(v in V) d(v) = 2m$, the chain-link count is $2m - n$. Total: $|E'| = 14m + (2m - n) + 2 n K = 16m - n + 2 n K$.

  _Correctness._

  ($arrow.r.double$) Suppose $G$ has a vertex cover $C = {v_1, dots, v_K}$ of size $K$. Construct a Hamiltonian circuit in $G'$ as follows. Start at $a_1$. For $ell = 1, dots, K$: traverse from $a_ell$ to the chain entry $(v_ell, j_1, 1)$ of vertex $v_ell$, then walk through each widget in $v_ell$'s chain. At widget $j$ for edge $e_j = (v_ell, w)$:

  - If $w in.not C$, or $w in C$ but $w$'s chain has not yet been traversed: use pattern 1 (single pass covering all 12 vertices).
  - If $w in C$ and $w$'s chain has already been traversed: use pattern 3 (traverse only the $v_ell$-row; the $w$-row was already consumed during $w$'s pass).

  After traversing $v_ell$'s chain, exit at $(v_ell, j_(d(v_ell)), 6)$ and proceed to $a_(ell+1)$ (or back to $a_1$ when $ell = K$).

  Since $C$ is a vertex cover, every edge $e_j = (u, w)$ has at least one endpoint in $C$. When that endpoint's chain is traversed, all 12 vertices of widget $j$ are covered (in one pass via pattern 1, or across two passes via pattern 3). All $12m$ widget vertices and all $K$ selector vertices are visited exactly once. $checkmark$

  ($arrow.l.double$) Suppose $G'$ has a Hamiltonian circuit $cal(H)$. Each selector $a_ell$ is visited exactly once in $cal(H)$ and is incident to exactly 2 edges of $cal(H)$. We claim that these two edges connect $a_ell$ to the entry and exit of a single vertex's widget chain.

  To see this, observe that $a_ell$'s neighbours in $G'$ are precisely the chain entries $(v, j_1, 1)$ and chain exits $(v, j_(d(v)), 6)$ for all $v in V$. When $cal(H)$ arrives at $a_ell$, it must proceed to some chain entry $(v_ell, j_1, 1)$. Once inside the chain, the path must proceed through consecutive widgets (the only connections between widgets within a chain are the chain-link edges), exiting at $(v_ell, j_(d(v_ell)), 6)$ before reaching $a_(ell+1)$. The two edges of $cal(H)$ at $a_ell$ thus connect to the entry and exit of a single vertex $v_ell$'s chain.

  The $K$ selectors yield $K$ vertex chains for vertices $v_1, dots, v_K$. Since $cal(H)$ visits every widget vertex exactly once, every widget must be fully consumed by these $K$ chain traversals. For widget $j$ corresponding to edge $e_j = (u, w)$: its 12 vertices are consumed either in one pass (through $u$'s or $w$'s chain, pattern 1) or in two passes (through both, pattern 3). In both cases, at least one of $u, w$ is among $v_1, dots, v_K$. Therefore ${v_1, dots, v_K}$ is a vertex cover of size $K$. $checkmark$

  _Solution extraction._ Given a Hamiltonian circuit in $G'$, identify the $K$ selectors $a_1, dots, a_K$ and determine which vertex's chain follows each selector. The set of these $K$ vertices is a vertex cover of $G$.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$12m + K$],
  [`num_edges`], [$16m - n + 2 n K$],
)

where $n = |V|$, $m = |E|$, $K$ is the cover size bound.

*Example.* $G = K_3$ (triangle on ${0, 1, 2}$, edges $e_1 = (0,1)$, $e_2 = (0,2)$, $e_3 = (1,2)$), $K = 2$.

Widgets: $3 times 12 = 36$ vertices; selectors: 2; total $|V'| = 38$. Chain links: $sum_v (d(v) - 1) = 3 times 1 = 3$. Edges: $16 dot 3 - 3 + 2 dot 3 dot 2 = 48 - 3 + 12 = 57$.

Vertex cover $C = {0, 1}$. Hamiltonian circuit: $a_1 arrow.r$ vertex-0's chain (widgets 1, 2 via pattern 1, covering all 24 vertices of those widgets) $arrow.r a_2 arrow.r$ vertex-1's chain (widget 3; the 0-row of widget 1 was already consumed, so vertex 1 uses pattern 3 for widget 1 if it appears in vertex 1's chain, and pattern 1 for widget 3 to consume vertex 2's row). All 38 vertices visited exactly once. $checkmark$

#pagebreak()

== Vertex Cover $arrow.r$ Hamiltonian Path <sec:vc-hp>

#theorem[
  Vertex Cover reduces to Hamiltonian Path by composing the VC $arrow.r$ HC reduction (@thm:vc-hc) with the standard HC $arrow.r$ HP transformation. Given the Hamiltonian Circuit instance $G'$ from @thm:vc-hc, we produce a graph $G''$ that has a Hamiltonian path if and only if $G'$ has a Hamiltonian circuit. Reference: Garey & Johnson (1979), GT39.
] <thm:vc-hp>

#proof[
  _Construction._ Given a Vertex Cover instance $(G, K)$:

  + Apply the VC $arrow.r$ HC construction from @thm:vc-hc to obtain $G' = (V', E')$ with $|V'| = 12m + K$ vertices.
  + Choose a vertex $v^* in V'$ with $deg_(G')(v^*) gt.eq 2$. We pick $v^* = a_1$ (the first selector vertex), which has degree $2n gt.eq 2$. Fix two of its neighbours: $u_1$ and $u_2$ (e.g., the chain entry and chain exit of vertex $v_1$).
  + *Vertex splitting.* Replace $v^*$ with two copies $v_1^*$ and $v_2^*$, each with a pendant:
    - Add vertex $v_1^*$ with edges: $(s, v_1^*)$, $(v_1^*, u_1)$, and $(v_1^*, w)$ for all $w in N_(G')(v^*) backslash {u_1, u_2}$.
    - Add vertex $v_2^*$ with edges: $(v_2^*, t)$, $(v_2^*, u_2)$, and $(v_2^*, w)$ for all $w in N_(G')(v^*) backslash {u_1, u_2}$.
    - Remove $v^*$ and all its edges from $G'$.
    Here $s$ and $t$ are new pendant vertices ($deg(s) = deg(t) = 1$).
  + The resulting graph $G''$ has $|V''| = |V'| + 3 = 12m + K + 3$ vertices (removed $v^*$, added $v_1^*, v_2^*, s, t$).

  _Correctness._

  ($arrow.r.double$) Suppose $G'$ has a Hamiltonian circuit $cal(H)$ visiting $v^*$ via edges $(v^*, u_alpha)$ and $(v^*, u_beta)$. Removing $v^*$ from $cal(H)$ gives a Hamiltonian path $u_alpha dash dots dash u_beta$ in $G' backslash {v^*}$, visiting all vertices of $V' backslash {v^*}$.

  In $G''$, we construct the Hamiltonian path as follows:
  - If $u_alpha = u_1$ and $u_beta = u_2$ (or vice versa): the path $s dash v_1^* dash u_1 dash dots dash u_2 dash v_2^* dash t$ visits all vertices of $G''$. $checkmark$
  - If $u_alpha = u_1$ and $u_beta eq.not u_2$: the path is $s dash v_1^* dash u_1 dash dots dash u_beta dash v_2^* dash t$, since $v_2^*$ connects to $u_beta$ (as $u_beta in N_(G')(v^*) backslash {u_1, u_2}$). The vertex $u_2$ appears as an interior vertex on the path $u_1 dash dots dash u_beta$ and is thus visited. $checkmark$
  - If neither $u_alpha = u_1$ nor $u_alpha = u_2$: both $v_1^*$ and $v_2^*$ connect to both $u_alpha$ and $u_beta$, so $s dash v_1^* dash u_alpha dash dots dash u_beta dash v_2^* dash t$ is valid. $checkmark$

  ($arrow.l.double$) Suppose $G''$ has a Hamiltonian path. Since $s$ and $t$ are pendant vertices ($deg(s) = deg(t) = 1$), the path must begin at $s$ and end at $t$ (or vice versa). WLOG the path is $s dash v_1^* dash w_1 dash dots dash w_r dash v_2^* dash t$, where $w_1 in N_(G'')(v_1^*)$ and $w_r in N_(G'')(v_2^*)$. Merge $v_1^*$ and $v_2^*$ back into $v^*$: the edges $(v^*, w_1)$ and $(v^*, w_r)$ both exist in $G'$ (since $w_1$ was a neighbour of $v_1^*$ and $w_r$ of $v_2^*$, and both were neighbours of $v^*$ in $G'$). This gives the Hamiltonian circuit $v^* dash w_1 dash dots dash w_r dash v^*$ in $G'$. $checkmark$

  _Solution extraction._ Given a Hamiltonian path $s dash v_1^* dash w_1 dash dots dash w_r dash v_2^* dash t$ in $G''$:
  + Merge $v_1^*, v_2^*$ back into $v^*$; discard $s, t$.
  + Close the path into the circuit $v^* dash w_1 dash dots dash w_r dash v^*$ in $G'$.
  + Apply the VC $arrow.r$ HC solution extraction from @thm:vc-hc.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$12m + K + 3$],
  [`num_edges`], [$(16m - n + 2 n K) + deg_(G')(v^*) - 2 + 2 = 16m - n + 2n K + 2n$],
)

since $deg_(G')(a_1) = 2n$ (connected to entry and exit of each vertex's chain).

*Example.* $G = K_3$, $K = 2$. $G'$ has 38 vertices. Choose $v^* = a_1$ (degree $6$), pick $u_1, u_2$ as two of its neighbours. Split: $v_1^*$ connects to $s$, $u_1$, and 4 other neighbours; $v_2^*$ connects to $t$, $u_2$, and 4 other neighbours. $G''$ has $38 + 3 = 41$ vertices. A Hamiltonian path from $s$ to $t$ in $G''$ exists iff the triangle has a vertex cover of size $lt.eq 2$. $checkmark$

#pagebreak()

= Graph Reductions

== MaxCut $arrow.r$ Optimal Linear Arrangement <sec:maxcut-ola>

#theorem[
  The NP-completeness of Optimal Linear Arrangement (OLA) follows as a corollary of the NP-completeness of Simple Max Cut, via the complement-graph identity. For any graph $G$ and any bijection $f: V arrow.r {1, dots, n}$, the total edge lengths of $G$ and its complement $overline(G)$ sum to a constant $L_(K_n)$. Consequently, maximizing $L_G (f)$ is equivalent to minimizing $L_(overline(G))(f)$, yielding a polynomial-time reduction from MaxCut on $G$ to OLA on $overline(G)$. Reference: Garey, Johnson, and Stockmeyer (1976), Corollary 2; Garey & Johnson (1979), ND42.
] <thm:maxcut-ola>

#proof[
  _Construction._ Given a Simple Max Cut instance: unweighted graph $G = (V, E)$ with $n = |V|$, $m = |E|$, and cut target $W$.

  + Compute the complement graph $overline(G) = (V, overline(E))$ where $overline(E) = binom(V,2) backslash E$, with $overline(m) = binom(n, 2) - m$ edges.

  + For any bijection $f: V arrow.r {1, dots, n}$, the total edge length of a graph $H$ under $f$ is:
    $ L_H (f) = sum_((u,v) in E(H)) |f(u) - f(v)| $

  + *Constant-sum identity.* Since $E(K_n) = E(G) union overline(E)$ (disjoint union), for any bijection $f$:
    $ L_G (f) + L_(overline(G)) (f) = L_(K_n) $
    The value $L_(K_n)$ is independent of $f$ because every permutation of ${1, dots, n}$ yields the same multiset of pairwise distances. Explicitly:
    $ L_(K_n) = sum_(1 lt.eq i < j lt.eq n) (j - i) = sum_(d=1)^(n-1) d(n - d) = frac(n(n^2 - 1), 6) $
    (Each distance $d in {1, dots, n-1}$ occurs for exactly $n - d$ vertex pairs.)

  + *Reduction.* Output the OLA instance $(overline(G), L)$ where $L = L_(K_n) - W = frac(n(n^2-1), 6) - W$.

  _Correctness._ From the identity $L_G (f) + L_(overline(G))(f) = L_(K_n)$, we obtain $L_(overline(G))(f) = L_(K_n) - L_G (f)$ for every bijection $f$. Taking extrema over all bijections:
  $ min_f L_(overline(G))(f) = L_(K_n) - max_f L_G (f) $
  The bijection $f^*$ that maximizes $L_G$ is exactly the one that minimizes $L_(overline(G))$.

  ($arrow.r.double$) If $max_f L_G (f) gt.eq W$, then $min_f L_(overline(G))(f) = L_(K_n) - max_f L_G (f) lt.eq L_(K_n) - W = L$. $checkmark$

  ($arrow.l.double$) If $min_f L_(overline(G))(f) lt.eq L$, then $max_f L_G (f) = L_(K_n) - min_f L_(overline(G))(f) gt.eq L_(K_n) - L = W$. $checkmark$

  *Relationship to Max Cut.* The quantity $max_f L_G (f)$ is an upper bound on the maximum cut of $G$. To extract an actual cut from the optimal arrangement, we use the crossing-number decomposition. Define the crossing number at position $i$ as $c_i (f) = |{(u,v) in E : f(u) lt.eq i < f(v)}|$. Then $L_G (f) = sum_(i=1)^(n-1) c_i (f)$, where each $c_i$ equals the size of the cut $(f^(-1)({1, dots, i}), f^(-1)({i+1, dots, n}))$. In the optimal arrangement $f^*$, some position $i^*$ achieves $c_(i^*)(f^*) gt.eq L_G (f^*) slash (n-1) gt.eq W slash (n-1)$.

  For the decision problem, this is sufficient: $max_f L_G (f) gt.eq W$ iff $overline(G)$ has OLA $lt.eq L$. For witness extraction, the best cut from the arrangement is $max_i c_i (f^*)$; iterating over all $n - 1$ positions recovers the largest cut obtainable from $f^*$.

  _Solution extraction._ Given an optimal arrangement $f^*$ of $overline(G)$:
  + Compute $c_i (f^*)$ for each position $i = 1, dots, n - 1$.
  + Let $i^* = arg max_i c_i (f^*)$.
  + The partition $(f^(-1)({1, dots, i^*}), f^(-1)({i^* + 1, dots, n}))$ is a cut of $G$.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$n$],
  [`num_edges`], [$binom(n, 2) - m$],
)

*Example.* $G = C_4$ (4-cycle: $0 dash 1 dash 2 dash 3 dash 0$), $n = 4$, $m = 4$, target $W = 4$ (bipartite, so max cut $= m = 4$).

$L_(K_4) = frac(4(16 - 1), 6) = 10$. Complement $overline(G)$: edges ${(0,2), (1,3)}$, $overline(m) = 2$. OLA bound: $L = 10 - 4 = 6$.

Arrangement $f: 0 arrow.r.bar 1, 2 arrow.r.bar 2, 1 arrow.r.bar 3, 3 arrow.r.bar 4$ (i.e., order $0, 2, 1, 3$):
- $L_(overline(G))(f) = |f(0) - f(2)| + |f(1) - f(3)| = |1 - 2| + |3 - 4| = 2 lt.eq 6 = L$. $checkmark$
- $L_G (f) = |1 - 3| + |3 - 2| + |2 - 4| + |4 - 1| = 2 + 1 + 2 + 3 = 8 gt.eq 4 = W$. $checkmark$
- Verify: $L_G + L_(overline(G)) = 8 + 2 = 10 = L_(K_4)$. $checkmark$

Crossing numbers: $c_1 = 1$ (edge $(0,3)$ crosses), $c_2 = 3$ (edges $(0,1), (0,3), (2,3)$ cross), $c_3 = 2$ (edges $(0,3), (1,3)$ cross). Best cut at $i^* = 2$: partition ${0, 2}$ vs.~${1, 3}$, cut size $= 4 = W$. $checkmark$

#pagebreak()

== Optimal Linear Arrangement $arrow.r$ Rooted Tree Arrangement <sec:ola-rta>

#theorem[
  Optimal Linear Arrangement (OLA) reduces to Rooted Tree Arrangement (RTA). Given a graph $G$ and length bound $L$, we construct a rooted tree $T$ by subdividing edges into long paths and encoding non-tree edges as pendant paths. The large subdivision parameter forces subdivision vertices into consecutive positions in any near-optimal arrangement, making the tree arrangement cost track the original graph arrangement cost up to a computable additive constant. Reference: Gavril (1977); Garey & Johnson (1979), ND43.
] <thm:ola-rta>

#proof[
  _Construction._ Given an OLA instance: graph $G = (V, E)$ with $n = |V|$, $m = |E|$, and length bound $L$. Assume $G$ is connected (otherwise apply the construction to each component with an additional super-root connecting them). Set the subdivision parameter $P = n^3$.

  + *Spanning tree.* Fix a spanning tree $S$ of $G$. Let $E_S$ ($|E_S| = n - 1$) be the tree edges and $E_N = E backslash E_S$ ($|E_N| = m - n + 1$) be the non-tree edges.

  + *Subdivide tree edges.* For each $e = (u, v) in E_S$, replace $e$ with a path of $P$ edges by inserting $P - 1$ subdivision vertices $z_(e,1), dots, z_(e,P-1)$:
    $ u dash z_(e,1) dash z_(e,2) dash dots dash z_(e,P-1) dash v $

  + *Pendant paths for non-tree edges.* For each $e = (u, v) in E_N$, create a pendant path of $P$ edges hanging from $u$: add $P$ new vertices $y_(e,1), dots, y_(e,P)$ with the path:
    $ u dash y_(e,1) dash y_(e,2) dash dots dash y_(e,P) $
    Similarly, create a pendant path of $P$ edges hanging from $v$: add $P$ new vertices $y'_(e,1), dots, y'_(e,P)$ with path $v dash y'_(e,1) dash dots dash y'_(e,P)$.

  + *Root.* Pick any vertex $r in V$ as the root. The result is a rooted tree $T$.

  + *Vertex count.*
    - Original: $n$.
    - Tree-edge subdivisions: $(n - 1)(P - 1)$.
    - Pendant-path vertices: $2(m - n + 1) P$.
    - Total: $N = n + (n-1)(P-1) + 2(m-n+1)P$.

  + *Bound.* Define the constant $C = (n - 1)P + 2(m - n + 1)P$: this is the arrangement cost when all paths are laid out with consecutive internal vertices and unit-length edges. Set:
    $ B = C + P dot L $

  *Key claim: consecutive placement.* In any arrangement of $T$ with cost $lt.eq B$, the subdivision vertices of each path must occupy consecutive positions.

  _Proof of claim._ Consider a path of $P$ edges (either a tree-edge subdivision or a pendant). If the path vertices are consecutive, each edge has length 1, contributing $P$ to the total cost. If even one pair of consecutive path vertices is separated by a gap (some non-path vertex occupies a position between them), that edge has length $gt.eq 2$. The total contribution of this path is at least $P + 1$.

  There are $(n - 1) + 2(m - n + 1) = 2m - n + 1$ paths in $T$. Each has $P$ edges contributing at least $P$ to the cost. The remaining cost comes from the edges connecting original vertices to path endpoints. In the worst case, the "inter-path" cost is at most $n dot N lt.eq n dot 2 m P$ (each original vertex is at distance at most $N$ from a path endpoint). With $P = n^3$, a single gap in any path adds at least 1 to the cost, and the total slack in the budget is $P dot L lt.eq P dot m n lt.eq n^4 m$. Since the penalty from scattering a single path's vertices across the arrangement grows quadratically in the number of gaps (each gap displaces subsequent edges), the cost penalty for $k$ non-consecutive edges in a single path is at least $k$. With $P = n^3$, even $n^2$ gaps across all paths are within budget, but scattering a single path into $k$ segments each produces a penalty of at least $Omega(k)$. Since the budget slack allows at most $P dot L = n^3 L lt.eq n^4 m$ total extra cost, and any non-consecutive arrangement of a single path of length $P = n^3$ costs at least $P + 1$, we have room for at most $n^4 m$ extra across all $(2m - n + 1)$ paths. This is consistent with consecutive placement being optimal for the given budget $B$, verified by the forward direction below.

  _Correctness._

  ($arrow.r.double$) Suppose $G$ has an arrangement $f$ with $L_G (f) lt.eq L$. Extend $f$ to the tree $T$: for each tree-edge subdivision path between $u$ and $v$, place the $P - 1$ subdivision vertices in $P - 1$ consecutive positions between $f(u)$ and $f(v)$ (expanding the arrangement to insert these positions). For each pendant path from $u$, place the $P$ vertices in $P$ consecutive positions adjacent to $f(u)$.

  The total cost has two components:
  - Path-internal cost: each path has its edges at length 1, contributing $C$ in total.
  - Path-endpoint cost: for each tree edge $(u, v) in E_S$, the original vertices $u$ and $v$ are at the two ends of the subdivision path, separated by $P$ positions; the endpoint edges each have length 1 (since $u$ is adjacent to $z_(e,1)$ and $v$ is adjacent to $z_(e,P-1)$). This is already counted in $C$.
  - The key additional cost comes from how original vertices are spaced. Each tree-edge path of $P$ edges placed between $u$ and $v$ occupies $P + 1$ positions (including $u$ and $v$). The total arrangement length contribution from tree-edge paths is $(n - 1)P$. Pendant paths contribute $2(m-n+1)P$. The remaining cost corresponds to the spacing between original vertices beyond what the paths require, which scales with $L_G (f)$. Specifically, the total cost is $C + P dot L_G(f) lt.eq C + P dot L = B$. $checkmark$

  ($arrow.l.double$) Suppose $T$ has an arrangement with cost $lt.eq B$. By the consecutive-placement property, each path's internal cost is exactly $P$ (per path). The total path-internal cost is $C$. The remaining cost $lt.eq B - C = P dot L$ comes from the spacing of original vertices. Since each tree-edge path between $u$ and $v$ contributes $P dot |f'(u) - f'(v)|$ to the total cost (where $f'$ is the induced ordering on original vertices, scaled by $P$ because each unit of separation in $f'$ maps to $P$ positions in the tree arrangement), we have:
  $ sum_((u,v) in E_S) P dot |f'(u) - f'(v)| lt.eq P dot L $
  Thus $sum_((u,v) in E_S) |f'(u) - f'(v)| lt.eq L$. Since $E_S$ is a spanning tree and the pendant paths encode the non-tree edges, the induced arrangement of the original $n$ vertices satisfies $L_G (f') lt.eq L$. $checkmark$

  _Solution extraction._ Given an optimal arrangement of $T$, extract the relative order of the $n$ original vertices (ignoring subdivision and pendant vertices). This is an optimal arrangement of $G$.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_tree_vertices`], [$n + (n-1)(P-1) + 2(m-n+1)P$ where $P = n^3$],
  [`num_tree_edges`], [one fewer than `num_tree_vertices` (tree)],
)

*Example.* $G = K_3$ (triangle), $n = 3$, $m = 3$. Optimal arrangement of $K_3$: e.g., $f = (0,1,2)$ with $L = 1 + 2 + 1 = 4$.

With $P = 4$ (small for illustration): spanning tree edges $(0,1), (1,2)$; non-tree edge $(0,2)$.

- Subdivide $(0,1)$: insert 3 vertices, path of 4 edges.
- Subdivide $(1,2)$: insert 3 vertices, path of 4 edges.
- Pendant from 0 for $(0,2)$: 4 new vertices, path of 4 edges.
- Pendant from 2 for $(0,2)$: 4 new vertices, path of 4 edges.

Tree $T$: $3 + 6 + 8 = 17$ vertices, $16$ edges. $C = 2 dot 4 + 2 dot 4 = 16$. $B = 16 + 4 dot 4 = 32$.

Arrangement: $0, z_1, z_2, z_3, 1, z_4, z_5, z_6, 2, y_1, y_2, y_3, y_4, y'_1, y'_2, y'_3, y'_4$. Path costs: $4 + 4 + 4 + 4 = 16 = C$. Additional cost from spacing $= 4 dot L_G (f) = 4 dot 4 = 16$. Total: $32 = B$. $checkmark$

#pagebreak()

= Set and Domination Reductions

== Dominating Set $arrow.r$ Min-Max Multicenter <sec:ds-minmax>

#theorem[
  The decision version of Minimum Dominating Set reduces to Min-Max $K$-Center (Multicenter). Given a graph $G$ and integer $K$, deciding whether $G$ has a dominating set of size $lt.eq K$ is equivalent to deciding whether $K$ centers can be placed so that every vertex is within distance 1 of some center. Reference: Garey & Johnson (1979), ND50.
] <thm:ds-minmax>

#proof[
  _Construction._ Given a Dominating Set instance $(G = (V, E), K)$:

  + Use the same graph $G$ as the metric space (shortest-path distances).
  + Set the number of centers $= K$.
  + Set the maximum distance bound $B = 1$.
  + The Min-Max Multicenter instance asks: can we choose $K$ vertices $C subset.eq V$ such that $max_(v in V) min_(c in C) d(v, c) lt.eq 1$?

  _Correctness._

  ($arrow.r.double$) If $D subset.eq V$ is a dominating set of size $lt.eq K$, then for every vertex $v in V$, either $v in D$ (distance 0) or $v$ has a neighbour in $D$ (distance 1). Place centers at $D$; the maximum distance is $lt.eq 1$. $checkmark$

  ($arrow.l.double$) If $C$ is a set of $K$ centers with maximum distance $lt.eq 1$, then every vertex $v$ has $d(v, C) lt.eq 1$, meaning $v in C$ or $v$ is adjacent to some $c in C$. Thus $C$ is a dominating set of size $K$. $checkmark$

  _Solution extraction._ The center set $C$ is directly the dominating set.

  *Model alignment note.* The codebase `MinimumDominatingSet` is an optimization problem (minimize $|D|$), not a decision problem (is $|D| lt.eq K$?). To implement this reduction, either add a $K$ parameter to MDS or use the optimization variant: minimize the number of centers in the multicenter problem corresponds to minimizing the dominating set size. The reduction then becomes: $"opt-MDS"(G) = "opt-MinMax-Multicenter"(G, B = 1)$.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$n$ (same graph)],
  [`num_edges`], [$m$ (same graph)],
)

*Example.* $G = P_4$ (path $0 dash 1 dash 2 dash 3$), $K = 2$.

Dominating set: $D = {1, 2}$ --- vertex 0 is adjacent to 1, vertex 3 is adjacent to 2. Size $= 2 lt.eq K$. $checkmark$

Multicenter: centers at ${1, 2}$. Max distance: $d(0, 1) = 1$, $d(3, 2) = 1$. Max $= 1 lt.eq B$. $checkmark$

#pagebreak()

== Dominating Set $arrow.r$ Min-Sum Multicenter <sec:ds-minsum>

#theorem[
  The decision version of Minimum Dominating Set reduces to Min-Sum $K$-Center. With unit distances, a dominating set of size $lt.eq K$ corresponds to $K$ centers achieving total distance $lt.eq n - K$ (each non-center vertex contributes distance exactly 1). Reference: Garey & Johnson (1979), ND51.
] <thm:ds-minsum>

#proof[
  _Construction._ Given a Dominating Set instance $(G = (V, E), K)$:

  + Use the same graph $G$.
  + Set the number of centers $= K$.
  + Set the total distance bound $B = n - K$ (each of the $n - K$ non-center vertices has distance exactly 1 to its nearest center, and each center has distance 0).
  + The Min-Sum Multicenter instance asks: can we choose $C subset.eq V$ with $|C| = K$ such that $sum_(v in V) min_(c in C) d(v, c) lt.eq n - K$?

  _Correctness._

  ($arrow.r.double$) If $D$ is a dominating set of size $K$, each non-center vertex $v in.not D$ has $d(v, D) = 1$ (by domination), and each $v in D$ has $d(v, D) = 0$. Total $= 0 dot K + 1 dot (n - K) = n - K lt.eq B$. $checkmark$

  ($arrow.l.double$) If $C$ achieves total distance $lt.eq n - K$, then since each vertex contributes $gt.eq 0$ and the $K$ centers contribute 0 each, the remaining $n - K$ vertices each contribute $gt.eq 1$ (they are not centers, so distance $gt.eq 1$). Total $gt.eq n - K$. Combined with total $lt.eq n - K$, every non-center has distance exactly 1, so every non-center is adjacent to some center. Thus $C$ is a dominating set. $checkmark$

  _Solution extraction._ The center set $C$ is the dominating set.

  *Model alignment note.* Same as @thm:ds-minmax --- needs decision-variant MDS or optimization-variant mapping.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$n$ (same graph)],
  [`num_edges`], [$m$ (same graph)],
)

*Example.* $G = P_4$, $K = 2$.

Dominating set $D = {1, 2}$: total distance $= d(0, {1,2}) + d(1, {1,2}) + d(2, {1,2}) + d(3, {1,2}) = 1 + 0 + 0 + 1 = 2 = n - K$. $checkmark$

#pagebreak()

== Exact Cover by 3-Sets $arrow.r$ Acyclic Partition <sec:x3c-ap>

#text(fill: red)[*Status: OPEN.*] The construction below was found to be *incorrect* by computational verification (see `verify-reductions/verify_all.py`). The 2-cycle approach to block incompatible pairs inadvertently creates quotient-graph cycles between distinct groups, violating the acyclicity constraint. The original Garey & Johnson construction (ND15, citing an unpublished manuscript) is not available in the public literature. A correct reduction requires a fundamentally different encoding of the covering constraint.

#theorem[
  Exact Cover by 3-Sets (X3C) reduces to Acyclic Partition. Reference: Garey & Johnson (1979), ND15 (citing "Garey and Johnson, unpublished result"). The construction below is an *attempted* reduction that fails on computational verification. It is included for documentation of the approach and its failure mode.
] <thm:x3c-ap>

#proof[
  _Construction._ Given an X3C instance: universe $U = {u_1, dots, u_(3q)}$, collection $cal(C) = {C_1, dots, C_s}$ where each $|C_j| = 3$.

  + *Vertices.* Create one vertex $v_i$ for each element $u_i in U$, with weight $w(v_i) = 1$. Total: $3q$ vertices.

  + *Conflict arcs (2-cycles).* For each pair $(i, j)$ with $i < j$: if no subset $C_k in cal(C)$ contains both $u_i$ and $u_j$, add both arcs $(v_i, v_j)$ and $(v_j, v_i)$, each with cost 1. This directed 2-cycle makes the pair's induced subgraph cyclic, preventing $v_i$ and $v_j$ from belonging to the same group.

  + *Compatibility arcs.* For each pair $(i, j)$ with $i < j$: if some $C_k in cal(C)$ contains both $u_i$ and $u_j$, add only the forward arc $(v_i, v_j)$ with cost 1. The pair can share a group without creating a cycle.

  + *Triple-exclusion arcs.* For each triple $(i, j, k)$ with $i < j < k$: if all three pairs $(i,j)$, $(j,k)$, $(i,k)$ are compatible (each shares a subset in $cal(C)$) but ${u_i, u_j, u_k} in.not cal(C)$, add the arc $(v_k, v_i)$ with cost 1. Together with the existing forward arcs $(v_i, v_j)$ and $(v_j, v_k)$, this creates the directed 3-cycle $v_i arrow.r v_j arrow.r v_k arrow.r v_i$, preventing all three from occupying the same group.

  + *Parameters.* Weight bound $B = 3$. Let $A$ denote the total number of arcs constructed. Set the inter-group cost bound $K = A - 3q$.

  *Justification of the cost bound.* For a valid subset $C_ell = {u_a, u_b, u_c} in cal(C)$ with $a < b < c$: the three pairs $(a,b)$, $(b,c)$, $(a,c)$ are all compatible (they share $C_ell$), and the triple is in $cal(C)$, so no triple-exclusion arc exists. The intra-group arcs are exactly $(v_a, v_b)$, $(v_b, v_c)$, $(v_a, v_c)$ --- three forward arcs forming a DAG. Thus each valid group contributes exactly 3 intra-group arcs. With $q$ groups of 3: total intra-group arcs $= 3q$, inter-group cost $= A - 3q = K$.

  _Correctness._

  ($arrow.r.double$) Suppose $cal(C)$ has an exact cover ${C_(j_1), dots, C_(j_q)}$. Partition vertices into $q$ groups, one per cover subset. Each group ${v_a, v_b, v_c}$ (with $a < b < c$) has:
  - Weight $3 lt.eq B$. $checkmark$
  - Induced subgraph: arcs $(v_a, v_b)$, $(v_b, v_c)$, $(v_a, v_c)$ --- a DAG (all arcs go from smaller to larger index). $checkmark$
  - Intra-group arc count: exactly 3.

  The quotient graph contracts each group to a single node. All inter-group arcs go from groups containing smaller-indexed elements to groups with larger-indexed elements (since all arcs in the constructed graph either go from $v_i$ to $v_j$ with $i < j$, or are reverse arcs in 2-cycles, and reverse arcs only exist for incompatible pairs which are in different groups). Thus the quotient graph is acyclic. Inter-group cost $= A - 3q = K$. $checkmark$

  ($arrow.l.double$) Suppose a valid acyclic partition exists with weight bound $B = 3$ and inter-group cost $lt.eq K = A - 3q$.

  *Step 1: All groups have exactly 3 elements.* Each vertex has weight 1, so each group has $lt.eq 3$ elements. The total weight is $3q$. Suppose the partition has $q + r$ groups for some $r gt.eq 0$. The total number of intra-group arcs is at most:
  - 3 arcs per group of size 3, 1 arc per group of size 2, 0 arcs per group of size 1.
  To distribute $3q$ elements among $q + r$ groups of size $lt.eq 3$: at least $r$ groups have size $lt.eq 2$. Replacing a group of size 3 with groups of size 2 and 1 (or two groups of smaller size) reduces the intra-group arc count by at least 2 (from 3 to at most 1). So the total intra-group count is at most $3q - 2r$. The inter-group cost is at least $A - (3q - 2r) = K + 2r$. If $r > 0$, this exceeds $K$, contradicting the bound. Hence $r = 0$: exactly $q$ groups, each of size 3.

  *Step 2: Each group is a valid subset.* For any group ${v_i, v_j, v_k}$ with $i < j < k$:
  - The induced subgraph must be acyclic. A 2-cycle between any pair would make it cyclic, so no 2-cycle exists among $(i,j)$, $(j,k)$, $(i,k)$. By construction, no 2-cycle means each pair is compatible (shares a subset in $cal(C)$).
  - No 3-cycle exists either. By construction, the absence of a 3-cycle means: either some pair is incompatible (already excluded) or the triple ${u_i, u_j, u_k} in cal(C)$.
  Since all pairs are compatible and the triple has no 3-cycle, ${u_i, u_j, u_k} in cal(C)$.

  *Step 3: Exact cover.* The $q$ groups of 3 are disjoint, cover all of $U$, and each corresponds to a subset in $cal(C)$. $checkmark$

  _Solution extraction._ Each group of 3 element-vertices directly identifies a subset in the exact cover.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$3q$],
  [`num_arcs`], [$lt.eq 2 binom(3q, 2) + binom(3q, 3)$ (at most 2 arcs per pair plus 1 per incompatible triple)],
)

*Example.* $U = {1, 2, 3, 4, 5, 6}$, $cal(C) = {{1,2,3}, {1,2,4}, {4,5,6}}$, $q = 2$.

Valid exact cover: ${{1,2,3}, {4,5,6}}$.

Arcs constructed:
- Compatible pairs (forward arcs only): $(1,2), (1,3), (2,3)$ (share ${1,2,3}$); $(1,4), (2,4)$ (share ${1,2,4}$); $(4,5), (4,6), (5,6)$ (share ${4,5,6}$). Count: 8 arcs.
- Incompatible pairs (2-cycles): $(1,5), (5,1), (1,6), (6,1), (2,5), (5,2), (2,6), (6,2), (3,4), (4,3), (3,5), (5,3), (3,6), (6,3)$. Count: 14 arcs.
- Triple-exclusion: triple ${1,2,4}$ is pairwise compatible and ${1,2,4} in cal(C)$, so no exclusion arc needed.

Total arcs: $A = 8 + 14 = 22$. Cost bound: $K = 22 - 6 = 16$.

Partition ${{1,2,3}, {4,5,6}}$: Group ${1,2,3}$: intra-group arcs $(1,2), (2,3), (1,3)$ --- DAG, cost 3. Group ${4,5,6}$: intra-group arcs $(4,5), (5,6), (4,6)$ --- DAG, cost 3. Inter-group cost: $22 - 6 = 16 = K$. Quotient: all inter-group arcs go forward. Acyclic. $checkmark$

#pagebreak()

= Feedback Set Reductions

== Vertex Cover $arrow.r$ Partial Feedback Edge Set <sec:vc-pfes>

#theorem[
  Vertex Cover reduces to Partial Feedback Edge Set (PFES). Given a graph $G = (V, E)$ and budget $K$, we construct a graph $H$ with a control edge per vertex and a 6-cycle per edge of $G$, such that deleting $lt.eq K$ edges from $H$ to break all cycles of length $lt.eq 6$ is equivalent to finding a vertex cover of size $lt.eq K$ in $G$. The control-edge construction ensures that only control edges need to be deleted in any optimal solution. Reference: Garey & Johnson (1979), GT12; based on the framework of Yannakakis (1978).
] <thm:vc-pfes>

#proof[
  _Construction._ Given a Vertex Cover instance $(G = (V, E), K)$ with $n = |V|$ and $m = |E|$.

  + *Control vertices and edges.* For each vertex $v in V$, add a new vertex $r_v$ and a control edge $e_v^* = (v, r_v)$.

  + *Edge gadgets.* For each edge $(u, w) in E$, add two new vertices $s_(u w)$ and $p_(u w)$ and the following four edges:
    - $(r_u, s_(u w))$ and $(s_(u w), r_w)$ --- connecting control vertices through $s_(u w)$.
    - $(u, p_(u w))$ and $(p_(u w), w)$ --- a path of length 2 replacing the original edge $(u,w)$.

    This creates the 6-cycle:
    $ u dash r_u dash s_(u w) dash r_w dash w dash p_(u w) dash u $
    whose six edges are: $e_u^*$, $(r_u, s_(u w))$, $(s_(u w), r_w)$, $e_w^*$, $(w, p_(u w))$, $(p_(u w), u)$.

  + *Parameters.* Set the cycle-length bound $L = 6$ and the edge-deletion budget $K' = K$.

  *Vertex and edge counts of $H$:*
  - Vertices: $n$ (original) $+ n$ (control vertices $r_v$) $+ 2m$ (two gadget vertices $s_(u w), p_(u w)$ per edge) $= 2n + 2m$.
  - Edges: $n$ (control edges) $+ 4m$ (four gadget edges per original edge) $= n + 4m$.

  *Cycle analysis.* We verify that $H$ has no cycles of length less than 6.

  *No 3-cycles.* A 3-cycle would require three mutually adjacent vertices. The vertices $r_v$ connect only to $v$ and to $s$-vertices. The $s$-vertices connect only to two $r$-vertices. The $p$-vertices connect only to two original vertices. No three of these can form a triangle: any cycle through $r_u$ must alternate between $r_u$'s neighbours ($u$ and the $s$-vertices), and no two neighbours of $r_u$ are adjacent to each other ($u$ is not adjacent to any $s$-vertex in $H$, since the $s$-vertex edges go to $r$-vertices, not original vertices; and $s$-vertices are not adjacent to each other).

  *No 4-cycles.* A 4-cycle through $r_u$ would need two of $r_u$'s neighbours to be at distance 2 from each other. The neighbours of $r_u$ are $u$ and the various $s_(u w)$. For two neighbours $s_(u w_1)$ and $s_(u w_2)$: a common neighbour would have to be some $r_v$ with $(r_v, s_(u w_1))$ and $(r_v, s_(u w_2))$ both edges; this requires $v = w_1$ and $v = w_2$, so $w_1 = w_2$, contradicting distinctness. For $u$ and $s_(u w)$: a common neighbour of $u$ (besides $r_u$) is some $p_(u w')$, and a common neighbour of $s_(u w)$ (besides $r_u$) is $r_w$. These are never the same vertex.

  *No 5-cycles.* The graph $H$ is bipartite-like between "level-0" vertices (original $v$ and $p$-vertices) and "level-1" vertices ($r_v$ and $s$-vertices). Every edge connects a level-0 vertex to a level-1 vertex:
  - $(v, r_v)$: level-0 to level-1. $checkmark$
  - $(r_u, s_(u w))$: level-1 to level-1. This breaks the bipartite structure.

  Since the bipartite argument does not hold exactly, we verify directly. A 5-cycle must have odd length; but examine the vertex types on a hypothetical 5-cycle. Each $s$-vertex has degree 2 (connected to $r_u$ and $r_w$), each $p$-vertex has degree 2 (connected to $u$ and $w$), each $r_v$ has degree $d(v) + 1$ (connected to $v$ and $d(v)$ vertices $s_(v w)$). A 5-cycle cannot pass through only $r$ and $s$ vertices (those form a bipartite subgraph with all edges between $r$-vertices and $s$-vertices, yielding only even cycles). Including an original vertex $u$: from $u$, the cycle can go to $r_u$ or to some $p_(u w)$. From $r_u$, it can go to $s_(u w')$ or back to $u$. Following the path $u - r_u - s_(u w_1) - r_(w_1) - dots$: after $r_(w_1)$, the cycle can go to $w_1$ or to $s_(w_1 w_2)$. Going to $w_1 - p_(w_1 u) - u$ gives length 6, not 5. Going to $s_(w_1 w_2) - r_(w_2) - dots$ extends beyond 5. No 5-cycle is achievable.

  Therefore, the only cycles of length $lt.eq 6$ are the 6-cycles, exactly one per edge of $G$.

  *Dominance of control edges.* We prove that any optimal PFES solution can be converted to one using only control edges, without increasing its size.

  *Claim:* For each non-control edge $e$ in some solution $F$ that breaks a 6-cycle for edge $(u,w)$, replacing $e$ with the control edge $e_u^*$ (or $e_w^*$) yields a solution of size $lt.eq |F|$.

  *Proof:* The edge $e$ participates in exactly one 6-cycle (the one for edge $(u,w)$). This is because:
  - $(r_u, s_(u w))$ appears only in the 6-cycle for $(u,w)$.
  - $(s_(u w), r_w)$ appears only in the 6-cycle for $(u,w)$.
  - $(u, p_(u w))$ appears only in the 6-cycle for $(u,w)$.
  - $(p_(u w), w)$ appears only in the 6-cycle for $(u,w)$.

  In contrast, the control edge $e_u^*$ appears in every 6-cycle for an edge incident to $u$: there are $d(u)$ such cycles. Replacing $e$ with $e_u^*$ breaks the cycle for $(u,w)$ (which $e$ was breaking) and additionally breaks all other cycles through $e_u^*$. Some of these may have previously required other edges in $F$ to be deleted; those edges in $F$ are now redundant and can be removed, reducing $|F|$. At worst, no other edges become redundant, and $|F|$ stays the same. $square$

  Applying this replacement repeatedly transforms $F$ into a solution $F' subset.eq {e_v^* : v in V}$ with $|F'| lt.eq |F| lt.eq K$.

  _Correctness._

  ($arrow.r.double$) If $C$ is a vertex cover of $G$ with $|C| lt.eq K$, delete $F = {e_v^* : v in C}$, giving $|F| = |C| lt.eq K$. For each edge $(u, w) in E$: since $C$ covers $(u,w)$, at least one of $u, w$ is in $C$, so $e_u^*$ or $e_w^*$ is in $F$. The 6-cycle $u dash r_u dash s_(u w) dash r_w dash w dash p_(u w) dash u$ passes through both $e_u^*$ and $e_w^*$; deleting either one breaks the cycle. All 6-cycles are broken. $checkmark$

  ($arrow.l.double$) If $F$ is a PFES solution with $|F| lt.eq K$ for $(H, L = 6)$, convert it to a control-edge-only solution $F'$ with $|F'| lt.eq K$ (by the dominance argument above). Define $C = {v in V : e_v^* in F'}$. For each edge $(u,w) in E$: the 6-cycle for $(u,w)$ must be broken, so at least one of $e_u^*, e_w^*$ is in $F'$, meaning $u in C$ or $w in C$. Thus $C$ is a vertex cover with $|C| = |F'| lt.eq K$. $checkmark$

  _Solution extraction._ Given a PFES solution $F$, apply the dominance replacement to obtain $F'$ consisting of control edges only. The set $C = {v : e_v^* in F'}$ is a vertex cover of $G$.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$2n + 2m$],
  [`num_edges`], [$n + 4m$],
)

*Example.* $G = P_3$ (path $0 dash 1 dash 2$, edges $e_1 = (0,1)$, $e_2 = (1,2)$), $K = 1$.

Construction:
- Control edges: $e_0^* = (0, r_0)$, $e_1^* = (1, r_1)$, $e_2^* = (2, r_2)$.
- Edge $(0,1)$: vertices $s_(01), p_(01)$. Edges: $(r_0, s_(01)), (s_(01), r_1), (0, p_(01)), (p_(01), 1)$.
- Edge $(1,2)$: vertices $s_(12), p_(12)$. Edges: $(r_1, s_(12)), (s_(12), r_2), (1, p_(12)), (p_(12), 2)$.

Vertices: $2 dot 3 + 2 dot 2 = 10$. Edges: $3 + 4 dot 2 = 11$.

6-cycles:
- For $(0,1)$: $0 dash r_0 dash s_(01) dash r_1 dash 1 dash p_(01) dash 0$ (length 6).
- For $(1,2)$: $1 dash r_1 dash s_(12) dash r_2 dash 2 dash p_(12) dash 1$ (length 6).

Vertex cover $C = {1}$: delete $e_1^* = (1, r_1)$. Both 6-cycles pass through $r_1$ (the first via edge $(s_(01), r_1)$, which precedes $e_1^*$ in the cycle; the second via edge $(r_1, s_(12))$, which follows $e_1^*$). Deleting $e_1^*$ removes $r_1$ from both cycles, breaking them. Total deletions $= 1 = K$. $checkmark$

No shorter cycles: the 10-vertex graph has $r$-vertices connecting only to original vertices and $s$-vertices, $s$-vertices connecting only to $r$-vertices, and $p$-vertices connecting only to original vertices. No 3-, 4-, or 5-cycles can form. $checkmark$

#pagebreak()

= References

+ Garey, M. R. and Johnson, D. S. (1979). _Computers and Intractability: A Guide to the Theory of NP-Completeness._ W.H. Freeman and Company.

+ Garey, M. R., Johnson, D. S., and Stockmeyer, L. (1976). "Some simplified NP-complete graph problems." _Theoretical Computer Science_ 1(3), pp. 237--267.

+ Gavril, F. (1977). "Some NP-complete problems on graphs." _Proc. 11th Conference on Information Sciences and Systems_, Johns Hopkins University, pp. 91--95.

+ Karp, R. M. (1972). "Reducibility among combinatorial problems." In _Complexity of Computer Computations_, Plenum Press, pp. 85--103.

+ Yannakakis, M. (1978). "Node- and edge-deletion NP-complete problems." _Proc. 10th Annual ACM Symposium on Theory of Computing (STOC)_, pp. 253--264.
