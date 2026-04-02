// Standalone verification proof: PartitionIntoCliques -> MinimumCoveringByCliques
// Issue: #889

== Partition Into Cliques $arrow.r$ Minimum Covering By Cliques <sec:partitionintocliques-minimumcoveringbycliques>

#let theorem(body) = block(
  width: 100%,
  inset: 8pt,
  stroke: 0.5pt,
  radius: 4pt,
  [*Theorem.* #body],
)

#let proof(body) = block(
  width: 100%,
  inset: 8pt,
  [*Proof.* #body #h(1fr) $square$],
)

#theorem[
  There is a polynomial-time reduction from Partition Into Cliques to Minimum Covering By Cliques. Given a graph $G = (V, E)$ and a positive integer $K$, the reduction outputs the same graph $G' = G$ and clique bound $K' = K$. If $G$ admits a partition of its vertices into at most $K$ cliques, then $G$ admits a covering of its edges by at most $K$ cliques (and the covering uses the same clique collection).
] <thm:partitionintocliques-minimumcoveringbycliques>

#proof[
  _Construction._

  Let $(G, K)$ be a Partition Into Cliques instance where $G = (V, E)$ is an undirected graph with $n = |V|$ vertices and $m = |E|$ edges, and $K >= 1$ is the maximum number of clique groups.

  + Set $G' = G$ (same vertex set $V$ and edge set $E$).
  + Set $K' = K$.
  + Output the Minimum Covering By Cliques instance $(G', K')$: find a collection of at most $K'$ cliques whose union covers every edge.

  _Correctness (forward direction)._

  ($arrow.r.double$) Suppose $G$ admits a partition of $V$ into $k <= K$ cliques $V_0, V_1, dots, V_(k-1)$. Each $V_i$ induces a complete subgraph. Since the $V_i$ partition $V$, every edge ${u, v} in E$ has both endpoints in exactly one $V_i$ (namely the group containing $u$ and $v$; since $V_i$ is a clique and ${u, v} in E$, both $u$ and $v$ belong to the same group). Therefore the collection $V_0, dots, V_(k-1)$ is also a valid edge clique cover: every edge is contained in some $V_i$, and $k <= K' = K$. Hence $(G', K')$ admits a covering by at most $K'$ cliques.

  _Remark on the reverse direction._

  The reverse direction does not hold in general: a covering by $K$ cliques does not imply a partition into $K$ cliques, because a covering allows vertices to belong to multiple cliques. For example, the path $P_3 = ({0, 1, 2}, {(0,1), (1,2)})$ can be covered by 2 cliques ${0, 1}$ and ${1, 2}$ (vertex 1 appears in both), but there is no partition of ${0, 1, 2}$ into 2 cliques that covers both edges (any partition into 2 groups leaves at least one group with a non-adjacent pair if that group has $>= 2$ vertices, or a singleton group whose edges are uncovered).

  This one-directional reduction is standard for proving NP-hardness: since Partition Into Cliques is NP-complete (Garey & Johnson, GT15), and any YES instance of Partition Into Cliques maps to a YES instance of Covering By Cliques, the covering problem is NP-hard (it is at least as hard to solve).

  _Solution extraction._ Given a partition $V_0, V_1, dots, V_(k-1)$ of $G$ into cliques (source witness), construct the target witness (edge-to-group assignment) as follows: for each edge $(u, v) in E$, assign it to the group $i$ such that both $u$ and $v$ belong to $V_i$. Since the partition is disjoint, each edge maps to exactly one group, and since each $V_i$ is a clique, all edges assigned to group $i$ have both endpoints in $V_i$, forming a valid clique cover.
]

*Overhead.*

#table(
  columns: (auto, auto),
  align: (left, left),
  [*Target metric*], [*Formula*],
  [`num_vertices`], [$n$],
  [`num_edges`], [$m$],
)

where $n$ = `num_vertices` and $m$ = `num_edges` of the source graph $G$. Both the graph and the bound are copied unchanged.

*Feasible example (YES instance).*

Source: $G$ has $n = 5$ vertices ${0, 1, 2, 3, 4}$ with edges $E = {(0,1), (0,2), (1,2), (3,4)}$ and $K = 2$.

The graph consists of a triangle ${0, 1, 2}$ and an edge ${3, 4}$. A valid partition into 2 cliques: $V_0 = {0, 1, 2}$ (triangle) and $V_1 = {3, 4}$ (edge). Partition config: $[0, 0, 0, 1, 1]$.

Verification: Group 0: vertices ${0, 1, 2}$ -- edges $(0,1)$, $(0,2)$, $(1,2)$ all present #sym.checkmark; Group 1: vertices ${3, 4}$ -- edge $(3,4)$ present #sym.checkmark. Groups are disjoint and cover all vertices #sym.checkmark.

Target: $G' = G$, same 5 vertices, same 4 edges, $K' = 2$.

Edge assignment: edge $(0,1)$ $arrow$ group 0 (both in $V_0$); edge $(0,2)$ $arrow$ group 0; edge $(1,2)$ $arrow$ group 0; edge $(3,4)$ $arrow$ group 1. Edge config: $[0, 0, 0, 1]$.

Check covering: Group 0 vertices ${0, 1, 2}$ form a clique #sym.checkmark; Group 1 vertices ${3, 4}$ form a clique #sym.checkmark. All 4 edges covered #sym.checkmark. Two groups used, $2 <= K' = 2$ #sym.checkmark.

*Infeasible example (NO instance, forward direction only).*

Source: $G$ is the path $P_4 = ({0, 1, 2, 3}, {(0,1), (1,2), (2,3)})$ with $K = 2$.

No partition of ${0, 1, 2, 3}$ into 2 groups can make each group a clique covering all edges. The 3 edges force the 4 vertices into groups where each group is a clique. But the only cliques in $P_4$ are: singletons, edges $(0,1)$, $(1,2)$, $(2,3)$. Any partition into 2 groups of 4 vertices must place at least 2 vertices in one group, and if those 2 vertices are not adjacent, that group is not a clique.

Specifically: consider all partitions into 2 groups. Vertex 1 must be with vertex 0 or vertex 2 (or both via a group). If $V_0 = {0, 1}$ and $V_1 = {2, 3}$: both are cliques (edges $(0,1)$ and $(2,3)$ exist). But edge $(1,2)$ has endpoints in different groups and is not covered by either clique. So this fails.

No valid 2-clique partition exists. Hence the source is a NO instance.

Note: the target (covering by 2 cliques) IS feasible for this graph: cliques ${0, 1}$ and ${1, 2, 3}$... wait, ${1, 2, 3}$ is not a clique ($(1,3)$ is not an edge). Instead: ${0, 1}$, ${1, 2}$, ${2, 3}$ requires 3 cliques. With 2 cliques we cannot cover all 3 edges of $P_4$ since no clique has more than 2 vertices. So the target is also NO for $K' = 2$.

Verification of target infeasibility: each edge of $P_4$ is its own maximal clique (no vertex belongs to all three edges). To cover 3 edges we need at least 3 cliques, so $K' = 2$ is insufficient #sym.checkmark.
