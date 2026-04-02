// Verification proof: MinimumVertexCover -> PartialFeedbackEdgeSet
// Issue: #894
// Reference: Garey & Johnson, Computers and Intractability, GT9;
//            Yannakakis 1978b / 1981 (edge-deletion NP-completeness)

= Minimum Vertex Cover $arrow.r$ Partial Feedback Edge Set

== Problem Definitions

*Minimum Vertex Cover.* Given an undirected graph $G = (V, E)$ with $|V| = n$
and $|E| = m$, and a positive integer $k <= n$, determine whether there exists a
subset $S subset.eq V$ with $|S| <= k$ such that every edge in $E$ has at least
one endpoint in $S$.

*Partial Feedback Edge Set (GT9).* Given an undirected graph $G' = (V', E')$,
positive integers $K <= |E'|$ and $L >= 3$, determine whether there exists a
subset $E'' subset.eq E'$ with $|E''| <= K$ such that $E''$ contains at least one
edge from every cycle in $G'$ of length at most $L$.

== Reduction (for fixed even $L >= 6$)

Given a Vertex Cover instance $(G = (V, E), k)$ and a fixed *even* cycle-length
bound $L >= 6$, construct a Partial Feedback Edge Set instance $(G', K' = k, L)$.

The constructed graph $G'$ uses _hub vertices_ -- the original vertices and edges
of $G$ do NOT appear in $G'$.

+ *Hub vertices.* For each vertex $v in V$, create two hub vertices $h_v^1$ and
  $h_v^2$, with a _hub edge_ $(h_v^1, h_v^2)$. This is the "activation edge"
  for vertex $v$; removing it conceptually "selects $v$ for the cover."

+ *Cycle gadgets.* Let $p = q = (L - 4) slash 2 >= 1$. For each edge
  $e = (u, v) in E$, create $L - 4$ private intermediate vertices: $p$ forward
  intermediates $f_1^e, dots, f_p^e$ and $q$ return intermediates
  $r_1^e, dots, r_q^e$. Add edges to form an $L$-cycle:
  $
  C_e: quad h_u^1 - h_u^2 - f_1^e - dots - f_p^e - h_v^1 - h_v^2 - r_1^e - dots - r_q^e - h_u^1.
  $

+ *Parameters.* Set $K' = k$ and keep cycle-length bound $L$.

=== Size Overhead

$
  "num_vertices"' &= 2n + m(L - 4) \
  "num_edges"' &= n + m(L - 2) \
  K' &= k
$

where $n = |V|$, $m = |E|$. The $n$ hub edges plus $m(L - 3)$ path edges (forward
and return combined) give $n + m(L - 2)$ total edges, since each gadget also
shares two hub edges already counted.

== Correctness Proof

=== Forward Direction ($"VC" => "PFES"$)

Let $S subset.eq V$ with $|S| <= k$ be a vertex cover of $G$.

Define $E'' = {(h_v^1, h_v^2) : v in S}$. Then $|E''| = |S| <= k = K'$.

For any gadget cycle $C_e$ (for edge $e = (u, v) in E$), since $S$ is a vertex
cover, at least one of $u, v$ belongs to $S$. WLOG $u in S$. Then
$(h_u^1, h_u^2) in E''$ and this edge lies on $C_e$. Hence $E''$ hits $C_e$.

Since every cycle of length $<= L$ in $G'$ is a gadget cycle (see below), $E''$
hits all such cycles. #sym.checkmark

=== Backward Direction ($"PFES" => "VC"$)

Let $E'' subset.eq E'$ with $|E''| <= K' = k$ hit every cycle of length $<= L$.

*Claim.* $E''$ can be transformed into a set $E'''$ of hub edges only, with
$|E'''| <= |E''|$.

_Proof._ Consider an edge $f in E''$ that is _not_ a hub edge. Then $f$ is an
intermediate edge lying in exactly one gadget cycle $C_e$:
- Every intermediate vertex has degree 2, so any intermediate edge belongs to
  exactly one cycle.

Replace $f$ with the hub edge $(h_u^1, h_u^2)$ (or $(h_v^1, h_v^2)$ if the
former is already in $E''$). This hits $C_e$ and additionally hits all other
gadget cycles passing through that hub edge. The replacement does not increase
$|E''|$.

After processing all non-hub edges, define $S = {v in V : (h_v^1, h_v^2) in E'''}$.
Then $|S| <= |E'''| <= k$, and for every $e = (u, v) in E$, cycle $C_e$ is hit
by a hub edge of $u$ or $v$, so $S$ is a vertex cover. #sym.checkmark

=== No Spurious Short Cycles (even $L >= 6$)

We verify that $G'$ has no cycles of length $<= L$ besides the gadget cycles.

Each intermediate vertex has degree exactly 2. Hub vertex $h_v^1$ connects to
$h_v^2$ (hub edge) and to the endpoints of return paths whose target is $v$
plus the endpoints of forward paths whose target is $v$. Similarly for $h_v^2$.

A non-gadget cycle must traverse parts of at least two distinct gadget paths.
Each gadget sub-path (forward or return) has length $p + 1 = (L - 2) slash 2$.
Since the minimum non-gadget cycle uses at least 3 such sub-paths (alternating
through hub vertices), its length is at least $3 dot (L - 2) slash 2$.

For even $L >= 6$: $3(L - 2) slash 2 >= 3 dot 2 = 6 > L$ requires
$3(L-2) > 2L$, i.e., $L > 6$. For $L = 6$: three sub-paths of length 2 each
give a cycle of length 6, but such a cycle would need to traverse 3 hub edges
as well, giving total length $3 dot 2 + 3 = 9 > 6$. #sym.checkmark

More precisely, each "step" in a non-gadget cycle traverses a sub-path of
length $(L - 2) slash 2$ plus a hub edge, for a step cost of $(L - 2) slash 2 + 1 = L slash 2$.
A non-gadget cycle needs at least 3 steps: minimum length $= 3 L slash 2 > L$.
#sym.checkmark

*Remark:* For odd $L$, the asymmetric split $p != q$ can create spurious
$L$-cycles through hub vertices. The symmetric $p = q$ split requires even $L$.
For $L = 3, 4, 5$, more sophisticated gadgets from Yannakakis (1978b/1981) are
needed.

== Solution Extraction

Given a PFES solution $c in {0, 1}^(|E'|)$ (where $c_j = 1$ means edge $j$ is
removed):

+ Identify hub edges. For each vertex $v$, let $a_v$ be the index of edge
  $(h_v^1, h_v^2)$ in $E'$.
+ If $c_(a_v) = 1$, mark $v$ as in the cover.
+ For any gadget cycle $C_e$ ($e = (u, v)$) not already hit by a hub edge,
  add $u$ (or $v$) to the cover.

The result is a vertex cover of $G$ with size $<= K' = k$.

== YES Example ($L = 6$)

*Source:* $G = (V, E)$ with $V = {0, 1, 2, 3}$, $E = {(0,1), (1,2), (2,3)}$
(path $P_4$), $k = 2$.

Vertex cover: $S = {1, 2}$ (covers all three edges).

*Target ($L = 6$, $p = q = 1$):*

- Hub vertices: $h_0^1=0, h_0^2=1, h_1^1=2, h_1^2=3, h_2^1=4, h_2^2=5, h_3^1=6, h_3^2=7$.
- Hub edges: $(0,1), (2,3), (4,5), (6,7)$ -- 4 edges.
- Gadget for $(0,1)$: forward $3 -> 8 -> 2$, return $3 -> 9 -> 0$.
  $C_((0,1)): 0 - 1 - 8 - 2 - 3 - 9 - 0$ (6 edges). #sym.checkmark
- Gadget for $(1,2)$: forward $3 -> 10 -> 4$, return $5 -> 11 -> 2$.
  $C_((1,2)): 2 - 3 - 10 - 4 - 5 - 11 - 2$ (6 edges). #sym.checkmark
- Gadget for $(2,3)$: forward $5 -> 12 -> 6$, return $7 -> 13 -> 4$.
  $C_((2,3)): 4 - 5 - 12 - 6 - 7 - 13 - 4$ (6 edges). #sym.checkmark

Total: 14 vertices, 16 edges, $K' = 2$.

Remove hub edges $(2,3)$ and $(4,5)$ (for vertices 1 and 2):
- $C_((0,1))$ hit by $(2,3)$. #sym.checkmark
- $C_((1,2))$ hit by $(2,3)$ and $(4,5)$. #sym.checkmark
- $C_((2,3))$ hit by $(4,5)$. #sym.checkmark

== NO Example ($L = 6$)

*Source:* $G = K_3$ (triangle ${0, 1, 2}$), $k = 1$.

No vertex cover of size 1 exists (minimum is 2).

*Target:* 12 vertices, 15 edges, $K' = 1$.

3 gadget cycles. Each hub edge appears in exactly 2 of 3 cycles (each vertex
in $K_3$ has degree 2). With budget 1, removing any one hub edge hits at most 2
cycles. Since there are 3 cycles, the instance is infeasible. #sym.checkmark

== References

- Garey, M. R. and Johnson, D. S. (1979). _Computers and Intractability_. Problem GT9.
- Yannakakis, M. (1978b). "Node- and edge-deletion NP-complete problems."
  _STOC '78_, pp. 253--264.
- Yannakakis, M. (1981). "Edge-Deletion Problems." _SIAM J. Comput._ 10(2):297--309.
