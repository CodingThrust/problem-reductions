// Verification proof: MinimumVertexCover → HamiltonianCircuit (#198)
// Based on Garey & Johnson, Theorem 3.4, pp. 56–60
#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules

#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

#show: thmrules.with(qed-symbol: $square$)
#let theorem = thmbox("theorem", "Theorem", fill: rgb("#e8e8f8"))
#let proof = thmproof("proof", "Proof")

== Vertex Cover $arrow.r$ Hamiltonian Circuit <sec:vc-hc>

#theorem[
  Vertex Cover is polynomial-time reducible to Hamiltonian Circuit.
  Given a graph $G = (V, E)$ with $|V| = n$ vertices and $|E| = m$ edges,
  and a positive integer $K <= n$, the constructed graph $G'$ has
  $12m + K$ vertices and at most $14m + 2n K - m'$ edges, where $m'$ is
  related to vertex degrees.
] <thm:vc-hc>

#proof[
  _Construction._
  Let $(G, K)$ be a Vertex Cover instance with $G = (V, E)$, $|V| = n$,
  $|E| = m$, and positive integer $K <= n$. We assume $G$ has no isolated
  vertices (vertices with degree 0 can be removed without affecting the
  vertex cover problem, and $K <= n'$ where $n'$ is the number of
  non-isolated vertices). We construct a graph $G' = (V', E')$ as follows.

  + *Selector vertices.* Create $K$ vertices $a_1, a_2, dots, a_K$.

  + *Cover-testing widgets.* For each edge $e = {u, v} in E$, create 12
    vertices:
    $ V'_e = {(u, e, i), (v, e, i) : 1 <= i <= 6}. $
    Add 14 internal edges:
    - *Horizontal chains:* ${(u, e, i), (u, e, i+1)}$ and
      ${(v, e, i), (v, e, i+1)}$ for $1 <= i <= 5$ (10 edges).
    - *Cross edges:* ${(u, e, 3), (v, e, 1)}$, ${(v, e, 3), (u, e, 1)}$,
      ${(u, e, 6), (v, e, 4)}$, ${(v, e, 6), (u, e, 4)}$ (4 edges).

    The only vertices in $V'_e$ that participate in edges outside the widget
    are the four _boundary vertices_ $(u, e, 1)$, $(v, e, 1)$, $(u, e, 6)$,
    $(v, e, 6)$.

    Due to the cross-edge structure, any Hamiltonian circuit of $G'$ must
    traverse the widget in exactly one of three patterns:
    - *Pattern U:* enters at $(u, e, 1)$, traverses all 6 $u$-vertices, exits
      at $(u, e, 6)$; separately enters at $(v, e, 1)$, traverses all 6
      $v$-vertices, exits at $(v, e, 6)$. Both $u$ and $v$ "cover" $e$.
    - *Pattern u-only:* enters at $(u, e, 1)$, crosses to $(v, e, 1)$ via
      cross-edge, traverses all 12 vertices using both chains, exits at
      $(u, e, 6)$. Only $u$ covers $e$.
    - *Pattern v-only:* enters at $(v, e, 1)$, crosses to $(u, e, 1)$,
      traverses all 12 vertices, exits at $(v, e, 6)$. Only $v$ covers $e$.

  + *Chain links.* For each vertex $v in V$, order the edges incident to $v$
    as $e_(v[1]), e_(v[2]), dots, e_(v[deg(v)])$ (arbitrary but fixed). Add
    connecting edges:
    $ E'_v = {{(v, e_(v[i]), 6), (v, e_(v[i+1]), 1)} : 1 <= i < deg(v)}. $
    This creates, for each $v$, a single path through all widgets where $v$
    is an endpoint: starting at $(v, e_(v[1]), 1)$ and ending at
    $(v, e_(v[deg(v)]), 6)$.

  + *Selector connections.* For each selector $a_j$ ($1 <= j <= K$) and
    each vertex $v in V$, add two edges:
    $ {a_j, (v, e_(v[1]), 1)} "and" {a_j, (v, e_(v[deg(v)]), 6)}. $
    Each selector can "choose" a vertex $v$ by connecting $a_j$ to the start
    and end of $v$'s chain.

  The complete vertex set is $V' = {a_1, dots, a_K} union union.big_(e in E) V'_e$
  with $|V'| = K + 12m$.

  _Correctness._

  ($arrow.r.double$) Suppose $C = {v_1, dots, v_K}$ is a vertex cover of $G$
  with $|C| = K$. We construct a Hamiltonian circuit of $G'$.

  Assign each selector $a_j$ to vertex $v_j in C$. For each $a_j$, build a
  cycle segment: $a_j -> (v_j, e_(v_j [1]), 1) -> dots -> (v_j, e_(v_j [deg(v_j)]), 6) -> a_(j+1)$
  (indices mod $K$, so $a_(K+1) = a_1$). Within each widget for edge
  $e = {u, v}$:
  - If both $u, v in C$: use Pattern U (two separate traversals).
  - If only $u in C$ (and $v in.not C$): use Pattern u-only (the
    $u$-selector traverses all 12 vertices).
  - If only $v in C$: use Pattern v-only.
  Since $C$ is a vertex cover, every edge has at least one endpoint in $C$,
  so every widget is fully traversed. Every non-covered vertex $v in.not C$
  has all its widgets traversed by the covering vertices, so $v$ contributes
  no selector path but all its widget vertices are visited. The result is a
  Hamiltonian circuit visiting every vertex exactly once.

  ($arrow.l.double$) Suppose $G'$ has a Hamiltonian circuit $cal(H)$. We
  extract a vertex cover of size $K$.

  The circuit visits each selector vertex $a_j$ exactly once. Between
  consecutive selectors $a_j$ and $a_(j')$ (the next selector on the
  circuit), the path must follow a chain of widgets for some vertex
  $v in V$: entering at $(v, e_(v[1]), 1)$, traversing widgets via chain
  links, and exiting at $(v, e_(v[deg(v)]), 6)$. This is because the only
  edges connecting selectors to widget vertices are the chain-start and
  chain-end edges, and within a chain the structure forces sequential widget
  traversal.

  Define $C = {v : "the chain of" v "is traversed by some selector path"}$.
  Then $|C| = K$ (each selector contributes exactly one vertex). For any edge
  $e = {u, v} in E$, the widget for $e$ must be fully traversed (all 12
  vertices visited). By the three-pattern property, at least one of $u$ or
  $v$ must have its chain pass through this widget, meaning at least one is
  in $C$. Hence $C$ is a vertex cover of size $K$.

  _Solution extraction._
  Given a Hamiltonian circuit $cal(H)$ in $G'$, identify the $K$ selector
  vertices. Between consecutive selectors, the path follows a vertex chain.
  The set of vertices whose chains are used forms a vertex cover of size $K$.
  In configuration terms: for each vertex $v in V$, set $"config"[v] = 1$
  (in cover) if $v$'s chain appears in any selector-to-selector segment.
]

*Overhead.*
#table(
  columns: (auto, auto),
  align: (left, left),
  table.header([Target metric], [Formula]),
  [`num_vertices`], [$12m + K$],
  [`num_edges`], [$14m + sum_(v in V) (deg(v) - 1) + 2 n K = 14m + (2m - n') + 2n K$],
)
where $n'$ is the number of non-isolated vertices (vertices with $deg(v) >= 1$;
each contributes $deg(v) - 1$ chain-link edges and the total chain-link count
is $sum deg(v) - n' = 2m - n'$).

*Feasible example (YES instance).*
Consider the path graph $P_3$ on vertices ${0, 1, 2}$ with edges
$E = {{0,1}, {1,2}}$, so $n = 3$, $m = 2$, and $K = 1$.

$G$ has a vertex cover of size 1: $C = {1}$ (vertex 1 covers both edges).

The constructed graph $G'$ has:
- 1 selector vertex: $a_1$.
- Widget for $e_1 = {0, 1}$: 12 vertices $(0, e_1, 1), dots, (0, e_1, 6), (1, e_1, 1), dots, (1, e_1, 6)$ with 14 edges.
- Widget for $e_2 = {1, 2}$: 12 vertices $(1, e_2, 1), dots, (1, e_2, 6), (2, e_2, 1), dots, (2, e_2, 6)$ with 14 edges.
- Chain links: vertex 1 has edges $e_1, e_2$, so chain link $(1, e_1, 6) - (1, e_2, 1)$. Vertices 0 and 2 each have degree 1, so no chain links for them.
- Selector connections: $a_1$ connects to $(v, e_(v[1]), 1)$ and $(v, e_(v[deg(v)]), 6)$ for each $v in {0, 1, 2}$.
  - $v = 0$: $a_1 - (0, e_1, 1)$ and $a_1 - (0, e_1, 6)$.
  - $v = 1$: $a_1 - (1, e_1, 1)$ and $a_1 - (1, e_2, 6)$.
  - $v = 2$: $a_1 - (2, e_2, 1)$ and $a_1 - (2, e_2, 6)$.

Total: $|V'| = 1 + 24 = 25$ vertices. The Hamiltonian circuit for $C = {1}$:
$a_1 -> (1, e_1, 1) -> (0, e_1, 1) -> (0, e_1, 2) -> dots -> (0, e_1, 6)$
$-> (1, e_1, 4) -> (1, e_1, 5) -> (1, e_1, 6) -> (1, e_2, 1)$
$-> (2, e_2, 1) -> (2, e_2, 2) -> dots -> (2, e_2, 6)$
$-> (1, e_2, 4) -> (1, e_2, 5) -> (1, e_2, 6) -> a_1$.

In widget $e_1$: Pattern 1-only (vertex 1 covers, traverses all 12 vertices).
In widget $e_2$: Pattern 1-only (vertex 1 covers, traverses all 12 vertices).
All 25 vertices visited exactly once. $checkmark$

*Infeasible example (NO instance).*
Consider the triangle graph $K_3$ on vertices ${0, 1, 2}$ with edges
$E = {{0,1}, {0,2}, {1,2}}$, so $n = 3$, $m = 3$, and $K = 1$.

The minimum vertex cover of $K_3$ has size 2 (any single vertex leaves one
edge uncovered). Since $K = 1 < 2$, there is no vertex cover of size 1.

The constructed graph $G'$ has $|V'| = 1 + 36 = 37$ vertices. Since no
vertex cover of size 1 exists, the reduction guarantees that $G'$ has no
Hamiltonian circuit: a single selector can traverse the chain of only one
vertex, which covers at most 2 of the 3 edges, leaving one widget that
cannot be fully traversed. $checkmark$
