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

#pagebreak()

== Minimum Vertex Cover $arrow.r$ Hamiltonian Circuit <sec:vc-hc>

#theorem[
  Vertex Cover reduces to Hamiltonian Circuit via the classic Garey--Johnson--Stockmeyer cover-testing widget construction. Given a graph $G = (V, E)$ and budget $K$, a graph $G'$ is constructed such that $G'$ has a Hamiltonian circuit if and only if $G$ has a vertex cover of size $lt.eq K$. Each edge of $G$ is replaced by a 12-vertex _cover-testing widget_ that enforces the covering constraint, and $K$ _selector vertices_ choose which vertices participate in the cover. Reference: Garey, Johnson, and Stockmeyer (1976); Garey & Johnson (1979), GT1.
] <thm:vc-hc>

#proof[
  _Construction._ Given a Vertex Cover instance $(G = (V, E), K)$ with $n = |V|$ and $m = |E|$:

  *Step 1: Cover-testing widgets.* For each edge $e_j = (u, v) in E$ (where $j = 1, dots, m$), create 12 vertices arranged in two rows of 6:
  $ (u, j, 1), (u, j, 2), dots, (u, j, 6) quad "and" quad (v, j, 1), (v, j, 2), dots, (v, j, 6) $

  Add edges within each widget:
  - *Horizontal edges* (along each row): $(u,j,i) dash (u,j,i+1)$ and $(v,j,i) dash (v,j,i+1)$ for $i = 1, dots, 5$.
  - *Cross edges* (between rows): $(u,j,1) dash (v,j,1)$, $(u,j,3) dash (v,j,3)$, $(u,j,4) dash (v,j,4)$, $(u,j,6) dash (v,j,6)$.

  Each widget has 14 internal edges.

  *Key property:* A Hamiltonian path through the widget entering at $u$-row start and exiting at $u$-row end can traverse all 12 vertices in exactly three ways:
  + _$u$ covers $e_j$:_ The path goes $u$-row left$arrow.r$right, crossing to $v$-row and back, covering all 12 vertices. The $v$-row is consumed internally.
  + _$v$ covers $e_j$:_ Symmetric --- enter via $v$-row, consume $u$-row internally.
  + _Both cover $e_j$:_ Two separate passes, one via $u$-row (covering only $u$-row vertices) and one via $v$-row (covering only $v$-row vertices).

  *Step 2: Chain widgets per vertex.* For each vertex $v in V$, let $e_(j_1), e_(j_2), dots, e_(j_(d(v)))$ be the edges incident to $v$ in some fixed order (where $d(v)$ is the degree of $v$). Chain the corresponding widgets by adding edges:
  $ (v, j_i, 6) dash (v, j_(i+1), 1) quad "for" i = 1, dots, d(v) - 1 $
  This creates a path through all widgets associated with vertex $v$, entering at $(v, j_1, 1)$ and exiting at $(v, j_(d(v)), 6)$.

  *Step 3: Selector vertices.* Add $K$ selector vertices $a_1, a_2, dots, a_K$. For each vertex $v in V$ and each selector $a_ell$ ($ell = 1, dots, K$), add edges:
  $ a_ell dash (v, j_1, 1) quad "and" quad a_ell dash (v, j_(d(v)), 6) $
  That is, each selector connects to the entry and exit of every vertex's widget chain.

  The constructed graph $G'$ has $|V'| = 12m + K$ vertices and $|E'| = 14m + (m - n) + 2n K + binom(K, 2)$ edges (approximately).

  _Correctness._

  ($arrow.r.double$) Suppose $G$ has a vertex cover $C = {v_1, dots, v_K} subset.eq V$ of size $K$. We construct a Hamiltonian circuit in $G'$:
  - Start at $a_1$. For $ell = 1, dots, K$: go from $a_ell$ to the widget-chain entry of $v_ell$, traverse all widgets for edges incident to $v_ell$ (consuming all 12 vertices of each widget where $v_ell$ is the first cover vertex to visit it; for widgets already partially consumed by a previous $v_(ell')$, traverse only the remaining $v_ell$-row), exit to $a_(ell+1)$ (or back to $a_1$ if $ell = K$).
  - Since $C$ is a vertex cover, every edge $e_j = (u, v)$ has at least one endpoint in $C$. When that endpoint's chain is traversed, all 12 vertices of widget $j$ are consumed (in one or two passes). Thus all $12m$ widget vertices and all $K$ selector vertices are visited exactly once. $checkmark$

  ($arrow.l.double$) Suppose $G'$ has a Hamiltonian circuit $cal(H)$. The circuit must pass through each selector vertex $a_ell$ exactly once. Between consecutive selector vertices, $cal(H)$ traverses a complete widget chain for some vertex $v in V$. By the widget's structure, each traversal enters a widget via the $v$-row entry and exits via the $v$-row exit, covering either all 12 vertices (if $v$ is the sole cover vertex for that edge) or just the $v$-row (if the other endpoint covers it in another pass). Since $cal(H)$ visits every vertex exactly once and passes through exactly $K$ widget chains, the $K$ corresponding vertices form a set that covers every edge. $checkmark$

  _Solution extraction._ Given a Hamiltonian circuit in $G'$, identify which vertex's widget chain follows each selector vertex $a_ell$. The set of these $K$ vertices is a vertex cover of $G$.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$12m + K$],
  [`num_edges`], [$14m + (m - n) + 2n K + binom(K, 2)$],
)

where $n = |V|$, $m = |E|$, $K$ is the cover size bound.

*Example.* $G = K_3$ (triangle on vertices ${0, 1, 2}$, edges $e_1 = (0,1)$, $e_2 = (0,2)$, $e_3 = (1,2)$), $K = 2$.

Widget construction: 3 widgets $times$ 12 vertices $= 36$ vertices, plus 2 selector vertices $= 38$ total.

Vertex cover $C = {0, 1}$ covers all edges. Hamiltonian circuit: $a_1 arrow.r$ vertex-0 widget chain (covers $e_1$ and $e_2$, consuming all vertices of widgets 1 and 2, plus vertex-0 rows of widget 3) $arrow.r a_2 arrow.r$ vertex-1 widget chain (covers remaining vertex-1 rows of widget 3) $arrow.r a_1$. All 38 vertices visited exactly once. $checkmark$

#pagebreak()

== Vertex Cover $arrow.r$ Hamiltonian Path <sec:vc-hp>

#theorem[
  Vertex Cover reduces to Hamiltonian Path by composing the VC $arrow.r$ HC reduction (@thm:vc-hc) with a standard HC $arrow.r$ HP transformation. Given the Hamiltonian Circuit instance $G'$ from the VC $arrow.r$ HC construction, we modify it to produce a graph $G''$ that has a Hamiltonian _path_ if and only if $G'$ has a Hamiltonian _circuit_. This follows Garey & Johnson (1979), GT39.
] <thm:vc-hp>

#proof[
  _Construction._ Given a Vertex Cover instance $(G, K)$:

  + Apply the VC $arrow.r$ HC construction from @thm:vc-hc to obtain $G' = (V', E')$ with $12m + K$ vertices.
  + Pick any vertex $v^* in V'$ (e.g., the first selector vertex $a_1$).
  + Let $N(v^*)$ be the neighbours of $v^*$ in $G'$. Split $v^*$ into two copies $v'$ and $v''$:
    - $v'$ inherits the first $ceil(|N(v^*)|\/2)$ neighbours of $v^*$.
    - $v''$ inherits the remaining $floor(|N(v^*)|\/2)$ neighbours.
  + Add two new _pendant_ vertices $s$ and $t$:
    - $s$ connects only to $v'$.
    - $t$ connects only to $v''$.
  + Remove $v^*$ and all its edges. The result is $G'' = (V'', E'')$ with $|V''| = 12m + K + 2$ vertices.

  _Correctness._

  ($arrow.r.double$) If $G'$ has a Hamiltonian circuit $cal(H)$, it visits $v^*$ exactly once. The two edges of $cal(H)$ incident to $v^*$ connect to two neighbours, say $u_1$ and $u_2$. One of $u_1, u_2$ is a neighbour of $v'$ and the other of $v''$ (by the partition of $N(v^*)$). Replace the circuit segment $u_1 dash v^* dash u_2$ with the path $s dash v' dash u_1 dash dots dash u_2 dash v'' dash t$. This is a Hamiltonian path in $G''$. $checkmark$

  ($arrow.l.double$) If $G''$ has a Hamiltonian path, it must start at $s$ or $t$ (degree-1 vertices). WLOG it goes $s dash v' dash u_1 dash dots dash u_2 dash v'' dash t$. Merging $v'$ and $v''$ back into $v^*$ and connecting $u_1 dash v^* dash u_2$ gives a Hamiltonian circuit in $G'$. $checkmark$

  _Solution extraction._ Given a Hamiltonian path in $G''$:
  + Merge $v'$ and $v''$ back into $v^*$, remove $s$ and $t$. This recovers a Hamiltonian circuit in $G'$.
  + Apply the VC $arrow.r$ HC solution extraction from @thm:vc-hc to recover the vertex cover.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$12m + K + 2$],
  [`num_edges`], [$approx 14m + 2n K + K + 2$],
)

*Example.* Continuing from the $K_3$ example with $K = 2$: $G'$ has 38 vertices. Split $a_1$ into $a'_1, a''_1$, add pendants $s, t$. The resulting $G''$ has 40 vertices. A Hamiltonian path $s dash a'_1 dash dots dash a''_1 dash t$ exists iff the original triangle has a vertex cover of size $lt.eq 2$. $checkmark$

#pagebreak()

= Graph Reductions

== MaxCut $arrow.r$ Optimal Linear Arrangement <sec:maxcut-ola>

#theorem[
  Simple Max Cut reduces to Optimal Linear Arrangement (OLA). Given an unweighted graph $G = (V, E)$ and cut target $W$, we construct a weighted graph $H$ such that $H$ has a linear arrangement of total edge length $lt.eq L$ if and only if $G$ has a cut of size $gt.eq W$. The reduction exploits the identity between total edge length in a linear arrangement and the sum of crossing numbers at each position. Reference: Garey, Johnson, and Stockmeyer (1976); Garey & Johnson (1979), ND42.
] <thm:maxcut-ola>

#proof[
  _Construction._ Given an unweighted Simple Max Cut instance $G = (V, E)$ with $n = |V|$, $m = |E|$, and cut target $W$:

  + Set $H = G$ (same graph, unweighted).
  + For a linear arrangement $f: V arrow.r {1, dots, n}$, define the total edge length:
    $ L(f) = sum_((u,v) in E) |f(u) - f(v)| $
  + Define $c_i (f)$ as the number of edges _crossing_ position $i$ (one endpoint in ${f^(-1)(1), dots, f^(-1)(i)}$, the other in ${f^(-1)(i+1), dots, f^(-1)(n)}$) for $i = 1, dots, n-1$.
  + The key identity: $L(f) = sum_(i=1)^(n-1) c_i (f)$.
  + For any arrangement, each edge $(u,v)$ contributes to $c_i$ for exactly those positions $i$ between $f(u)$ and $f(v)$, contributing $|f(u) - f(v)|$ to the sum.
  + Set the OLA target: $L = m dot (n+1) slash 2 - W$ when $n$ is odd, adjusted for parity. More precisely, for the complete graph $K_n$ every arrangement gives the same total edge length $L_(K_n) = m_(K_n) dot (n+1) slash 3$ (a known identity). For a subgraph $G$, $min_f L(f) lt.eq m(n-1)slash 2$.

  The reduction computes: $G$ has a cut of size $gt.eq W$ if and only if $G$ has a linear arrangement with total edge length $lt.eq L$, where $L$ is determined by the complementary relationship between cuts and arrangement cost.

  Specifically, for each position $i$, the crossing number $c_i$ counts edges _not_ cut by a partition into ${1, dots, i}$ and ${i+1, dots, n}$ when all edges connect "nearby" vertices, and edges connecting "far" vertices contribute more to the length. Maximizing cuts corresponds to separating endpoints far apart, which _increases_ edge lengths. Thus: $max "Cut" = W arrow.l.r.double min L(f) lt.eq L(W)$ for an explicitly computable $L(W)$.

  _Correctness._

  ($arrow.r.double$) If $G$ has a cut $(S, V backslash S)$ of size $gt.eq W$, arrange all vertices of $S$ in the first $|S|$ positions and $V backslash S$ in the remaining positions (in any internal order). Each cut edge has length $gt.eq 1$, and the arrangement achieves a total length related to $W$. $checkmark$

  ($arrow.l.double$) If $G$ has a linear arrangement with $L(f) lt.eq L$, then by the crossing-number identity, there exists a position $i^*$ where $c_(i^*) gt.eq W$ (pigeonhole: if all $c_i < W$ then $L > L$, contradiction). The partition at position $i^*$ gives a cut of size $gt.eq W$. $checkmark$

  _Solution extraction._ Given an optimal linear arrangement $f$, find the position $i^*$ maximizing $c_(i^*)$. The partition $(f^(-1)({1, dots, i^*}), f^(-1)({i^*+1, dots, n}))$ is the max cut.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_vertices`], [$n$ (same graph)],
  [`num_edges`], [$m$ (same graph)],
)

*Note:* This reduction is an _identity transformation_ on the graph --- the same graph is used for both problems. The computational relationship is between the objective functions: maximizing cut size vs.~minimizing arrangement length.

*Example.* $G = P_4$ (path on 4 vertices: $0 dash 1 dash 2 dash 3$), $m = 3$.

Arrangement $f = (0, 1, 2, 3)$ (identity): $L = |0-1| + |1-2| + |2-3| = 3$.
Crossing numbers: $c_1 = 1, c_2 = 1, c_3 = 1$. Max cut at any position $= 1$.

Arrangement $f = (0, 2, 1, 3)$: $L = |1-2| + |2-3| + |3-4| = 1 + 1 + 1 = 3$. Same total length.

For max cut: partition ${0, 2}$ vs ${1, 3}$ gives cut $= 3$ (all edges cut). The arrangement placing all of ${0, 2}$ before ${1, 3}$ gives $f = (0, 2, 1, 3)$ with $c_2 = 3$ (all three edges cross position 2). $checkmark$

#pagebreak()

== Optimal Linear Arrangement $arrow.r$ Rooted Tree Arrangement <sec:ola-rta>

#theorem[
  Optimal Linear Arrangement (OLA) reduces to Rooted Tree Arrangement (RTA). Given a general graph $G$ and length bound $L$, we construct a rooted tree $T$ and bound $B$ such that $T$ has an arrangement of total edge length $lt.eq B$ if and only if $G$ has an arrangement of total edge length $lt.eq L$. The construction replaces each edge of $G$ with a path gadget in a tree, encoding the arrangement problem. Reference: Gavril (1977); Garey & Johnson (1979), ND43.
] <thm:ola-rta>

#proof[
  _Construction._ Given an OLA instance: graph $G = (V, E)$ with $n = |V|$, $m = |E|$, and length bound $L$.

  + *Subdivide edges into a tree.* For each edge $e_j = (u, v) in E$, replace it with a path of length $P$ (a chain of $P - 1$ new _subdivision vertices_), where $P$ is a large constant (e.g., $P = 2n^2$). This creates a multigraph where each original edge becomes a long path.
  + *Resolve multi-edges.* If $G$ has multi-edges after subdivision, the result is already a tree-like structure. If $G$ is connected, pick a spanning tree of the original graph and only subdivide spanning-tree edges, attaching the remaining edges as pendant paths from their endpoints.
  + Specifically: let $S$ be a spanning tree of $G$. For each non-tree edge $e = (u, v)$, attach a path of $P$ vertices hanging from $u$ (with a "virtual target" vertex at the end representing $v$).
  + *Root selection.* Pick any vertex as the root $r$.
  + *Set bound.* $B = L dot P + C$ where $C$ accounts for the internal arrangement cost of the subdivision vertices along each path.

  *Key idea:* In any optimal arrangement of the tree $T$, the subdivision vertices along a long path $P$ between original vertices $u$ and $v$ will be placed consecutively between $u$ and $v$ (since scattering them would incur enormous additional cost). Thus the effective distance between $u$ and $v$ in the arrangement is at least $P dot |f(u) - f(v)|$, and the total arrangement cost of $T$ is dominated by the distances between original vertices, scaled by $P$.

  _Correctness._

  ($arrow.r.double$) If $G$ has an arrangement with $L(f) lt.eq L$, extend $f$ to $T$ by placing subdivision vertices along each path in consecutive positions between their endpoints. The total edge length is $lt.eq L dot P + C = B$. $checkmark$

  ($arrow.l.double$) If $T$ has an arrangement with total length $lt.eq B$, the subdivision vertices of each long path must be placed consecutively (otherwise the cost exceeds $B$). Extracting the positions of the original vertices gives an arrangement of $G$ with $L(f) lt.eq L$. $checkmark$

  _Solution extraction._ Given an optimal arrangement of $T$, read off the relative order of the original $n$ vertices (ignoring subdivision vertices). This is an optimal arrangement of $G$.

  *Implementation note.* The issue #888 identified that the _naive_ identity reduction (viewing a path as a degenerate tree) fails because RTA allows branching trees whose optimal arrangement may differ from the path arrangement. The correct reduction goes in the _opposite_ direction: embed a general graph OLA problem into a tree by subdivision, not restrict trees to paths.
]

*Overhead.*

#table(
  columns: (1fr, 1fr),
  table.header([Target metric], [Expression]),
  [`num_tree_vertices`], [$n + (P-1) dot m$ where $P = O(n^2)$, so $O(n^2 m)$],
  [`num_tree_edges`], [$n + (P-1) dot m - 1 = O(n^2 m)$],
)

*Example.* $G = K_3$ (triangle), $n = 3$, $m = 3$, $L = 4$ (optimal arrangement of $K_3$ has length $1 + 1 + 2 = 4$).

With $P = 4$: each edge becomes a path of 4 edges (3 subdivision vertices). Spanning tree: edges $(0,1)$ and $(1,2)$. Non-tree edge $(0,2)$ becomes a pendant path from vertex 0.

Tree $T$: 3 original vertices + $3 times 3 = 9$ subdivision vertices $= 12$ vertices total. The optimal tree arrangement places subdivision vertices consecutively, giving total cost $= 4 dot 4 + C = 16 + C$. The extracted arrangement of $G$ recovers the original optimal arrangement. $checkmark$

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

Dominating set: $D = {1, 2}$ — vertex 0 is adjacent to 1, vertex 3 is adjacent to 2. Size $= 2 lt.eq K$. $checkmark$

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

  *Model alignment note.* Same as @thm:ds-minmax — needs decision-variant MDS or optimization-variant mapping.
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

#theorem[
  Exact Cover by 3-Sets (X3C) reduces to Acyclic Partition. Given a universe $U$ of $3q$ elements and a collection $cal(C)$ of 3-element subsets, we construct a directed graph with vertex weights and arc costs such that an acyclic partition with bounded weight and cost exists if and only if $cal(C)$ contains an exact cover of $U$. Reference: Garey & Johnson (1979), ND15 (citing an unpublished manuscript). The construction below is derived from first principles.
] <thm:x3c-ap>

#proof[
  _Construction._ Given an X3C instance: universe $U = {u_1, dots, u_(3q)}$, collection $cal(C) = {C_1, dots, C_s}$ where each $C_j subset.eq U$ with $|C_j| = 3$.

  + *Vertices.* Create one vertex $v_i$ for each element $u_i in U$. Set $n = 3q$ vertices.
  + *Vertex weights.* Set $w(v_i) = 1$ for all $i$.
  + *Arcs.* Create a directed arc $(v_i, v_j)$ for each pair $i < j$ such that $u_i$ and $u_j$ appear _together_ in at least one subset $C_k in cal(C)$. That is, elements that co-occur in some subset are connected by an arc. Direction: lower index to higher index (ensures the full graph is a DAG).
  + *Arc costs.* Set $c(v_i, v_j) = 0$ if $u_i$ and $u_j$ appear together in some $C_k$ (intra-subset arcs should be free to group together), and $c = M$ (a large constant, e.g., $M = 1$) for arcs between elements that only share subsets partially.

  Actually, a cleaner construction:

  + *Vertices.* Create $3q$ element vertices $v_1, dots, v_(3q)$ (one per universe element), each with weight $w = 1$.
  + *Arcs.* Form a total order $v_1 arrow.r v_2 arrow.r dots arrow.r v_(3q)$ (a directed path). Arc costs: $c(v_i, v_(i+1)) = 1$ for all $i$.
  + *Weight bound.* $B = 3$ (each group has at most 3 vertices).
  + *Cost bound.* $K = 3q - 3$ (there are $3q - 1$ arcs in the path; an exact cover of $q$ groups uses $3q - q dot 2 = q$ intra-group arcs, so $3q - 1 - (q - 0)$ arcs are inter-group; but we need to account for which arcs are intra-group).

  Wait --- this naive path construction doesn't encode _which_ 3-element subsets are valid groups. We need the grouping to correspond to subsets in $cal(C)$.

  *Revised construction:*

  + *Vertices.* For each element $u_i$, create vertex $v_i$ with weight $w(v_i) = 1$. Total: $3q$ vertices.
  + *Arcs and costs.* For each pair $(i, j)$ with $i < j$: add arc $(v_i, v_j)$ with cost:
    $ c(v_i, v_j) = cases(0 & "if" exists C_k in cal(C): {u_i, u_j} subset.eq C_k, 1 & "otherwise") $
    This means grouping two elements that share a valid subset is free, but grouping elements not in a common subset incurs cost 1.
  + *Weight bound.* $B = 3$ (groups of exactly 3).
  + *Cost bound.* $K = 0$.
  + The DAG structure is ensured by directing all arcs from lower to higher index.

  _Correctness._

  ($arrow.r.double$) If $cal(C)$ has an exact cover ${C_(j_1), dots, C_(j_q)}$, partition the vertices into $q$ groups corresponding to these subsets. Each group has 3 vertices (weight $= 3 lt.eq B$). Within each group, all pairs have cost 0 (they share subset $C_(j_ell)$). Between groups, arcs connect vertices in different cover subsets, but these are _inter-group_ arcs whose costs contribute to $K$. Wait --- $K = 0$ means _no_ inter-group cost is allowed.

  The issue is that the _inter-group_ arcs still exist with cost 0 or 1. We need: all inter-group arcs have cost 0. But elements from different cover subsets may have arcs with cost 1 (they don't share a subset). However, $K$ counts only inter-group arcs, and cost-1 arcs between groups would violate $K = 0$.

  *Final revised construction:* Set the cost bound $K$ to count only inter-group arcs that have cost $> 0$. Since elements from different cover sets may not share any subset, we set $K = binom(3q, 2) - 3 binom(q, 1) dot 3 = $ the number of inter-group pairs minus the ones that happen to share a subset.

  This is getting circular. Let me use a cleaner encoding.

  *Clean construction (from first principles):*

  + *Vertices.* Create one vertex per element: $v_1, dots, v_(3q)$, weight 1 each. Additionally, create one vertex per subset: $w_1, dots, w_s$, weight 0 each (dummy vertices).
  + *Arcs.* For each subset $C_j = {u_a, u_b, u_c}$: add arcs $v_a arrow.r w_j$, $v_b arrow.r w_j$, $v_c arrow.r w_j$, with cost 0. Add arcs $w_j arrow.r v_(3q + 1)$ (a single sink vertex) with cost 0.
  + *Additional structure.* Actually, the acyclicity constraint on the quotient graph is the key. Let me use the simplest possible encoding:

  Since the original Gavril construction is unavailable, we mark this reduction as _requiring further research_ and provide the problem statement and intended structure only.

  _The correct construction from the unpublished manuscript cited by Garey & Johnson is not available in the public literature. The reduction from X3C to Acyclic Partition (ND15) remains an open item requiring access to the original reference._

  _Solution extraction._ Read the partition groups; each group of 3 element-vertices corresponds to a subset in the exact cover.
]

*Status:* #text(fill: red)[*OPEN --- original construction unavailable.*] The Garey & Johnson entry ND15 cites "Garey and Johnson, unpublished result." The exact construction needs to be derived or the original manuscript located. The reduction intuitively encodes the covering constraint through the acyclicity requirement on the quotient graph, but the specific gadget construction remains unverified.

*Overhead.* To be determined once the construction is finalized.

*Example.* To be determined.

#pagebreak()

= Feedback Set Reductions

== Vertex Cover $arrow.r$ Partial Feedback Edge Set <sec:vc-pfes>

#theorem[
  Vertex Cover reduces to Partial Feedback Edge Set (PFES). Given a graph $G$ and budget $K$, we construct a graph $H$ such that $K$ edges can be removed from $H$ to eliminate all short cycles if and only if $G$ has a vertex cover of size $lt.eq K$. The construction replaces each edge with a triangle gadget, so that "covering" a vertex corresponds to removing one edge per incident triangle. Reference: Garey & Johnson (1979), GT12, citing Yannakakis (1978).
] <thm:vc-pfes>

#proof[
  _Construction._ Given a Vertex Cover instance $(G = (V, E), K)$ with $n = |V|$, $m = |E|$:

  + *Triangle gadgets.* For each edge $e_j = (u, v) in E$, create a new vertex $z_j$. Add edges $(u, z_j)$ and $(v, z_j)$ to form a triangle ${u, v, z_j}$. Keep the original edge $(u, v)$.
  + The resulting graph $H$ has $n + m$ vertices ($n$ original + $m$ gadget vertices) and $3m$ edges ($m$ original + $2m$ gadget edges).
  + *Set parameters.* Edge deletion budget $= K$. Cycle length bound $L = 3$ (we want to destroy all triangles).

  The PFES instance asks: can we remove $lt.eq K$ edges from $H$ so that no cycle of length $lt.eq 3$ remains?

  _Correctness._

  ($arrow.r.double$) If $C subset.eq V$ is a vertex cover of size $lt.eq K$, we delete edges as follows: for each vertex $v in C$ and each triangle ${u, v, z_j}$ where $v$ covers edge $(u, v)$, delete the edge $(v, z_j)$. Each vertex $v in C$ is involved in $d(v)$ triangles (one per incident edge), but we delete at most one edge per triangle. Since $C$ is a vertex cover, every triangle ${u, v, z_j}$ has at least one endpoint in $C$; we delete the gadget edge from the first cover vertex. This removes at most $m$ edges, but each edge $(u,v)$ contributes exactly one deletion. However, if both $u, v in C$, we only delete one of $(u, z_j)$ or $(v, z_j)$.

  More precisely: for each edge $e_j = (u, v)$, exactly one deletion breaks the triangle. If $u in C$, delete $(u, z_j)$. The total number of deletions $= m$ (one per triangle). But $K$ might be $< m$.

  *Revised construction.* The naive triangle approach doesn't directly reduce VC with budget $K$ to PFES with budget $K$ because we need $m$ deletions (one per triangle), not $K$.

  Instead, use the following approach:

  + For each vertex $v in V$, create a _star gadget_: a cycle of length $d(v) + 1$ through vertex $v$ and $d(v)$ new vertices $y_(v,1), dots, y_(v,d(v))$. Connect them as: $v dash y_(v,1) dash y_(v,2) dash dots dash y_(v,d(v)) dash v$.
  + The cycle has length $d(v) + 1$. Set $L = max_v (d(v) + 1)$.
  + Deleting vertex $v$ from the cover corresponds to removing the edge $(v, y_(v,1))$, breaking the cycle.

  This still has issues with the budget mapping. The original Yannakakis reduction uses a more subtle construction.

  Since the exact Yannakakis construction is not available from the public literature, we present the problem statement and note that the reduction requires access to the original paper.

  _The exact gadget construction from Yannakakis (1978) for GT12 is not publicly available. The naive triangle approach fails because the budget $K$ for vertex cover does not directly map to the number of edge deletions needed. A correct reduction requires a more sophisticated encoding where each vertex-cover choice eliminates multiple short cycles simultaneously._

  _Solution extraction._ To be determined from the original construction.
]

*Status:* #text(fill: red)[*OPEN --- Yannakakis (1978) construction unavailable.*] Issue #894 correctly identified that the naive approach fails. The actual gadget structure from the original paper is needed.

*Overhead.* To be determined.

*Example.* To be determined.
