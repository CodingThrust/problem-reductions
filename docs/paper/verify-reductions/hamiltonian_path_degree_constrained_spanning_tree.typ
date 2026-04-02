// Verification proof: HamiltonianPath -> DegreeConstrainedSpanningTree
// Issue: #911
// Reference: Garey & Johnson, Computers and Intractability, ND1, p.206

= Hamiltonian Path $arrow.r$ Degree-Constrained Spanning Tree

== Problem Definitions

*Hamiltonian Path.* Given an undirected graph $G = (V, E)$, determine whether $G$
contains a simple path that visits every vertex exactly once.

*Degree-Constrained Spanning Tree (ND1).* Given an undirected graph $G = (V, E)$ and a
positive integer $K <= |V|$, determine whether $G$ has a spanning tree in which every
vertex has degree at most $K$.

== Reduction

Given a Hamiltonian Path instance $G = (V, E)$ with $n = |V|$ vertices:

+ Set the target graph $G' = G$ (unchanged).
+ Set the degree bound $K = 2$.
+ Output $"DegreeConstrainedSpanningTree"(G', K)$.

== Correctness Proof

We show that $G$ has a Hamiltonian path if and only if $G$ has a spanning tree with
maximum vertex degree at most 2.

=== Forward ($G$ has a Hamiltonian path $arrow.r.double$ degree-2 spanning tree exists)

Let $P = v_0, v_1, dots, v_(n-1)$ be a Hamiltonian path in $G$. The path edges
$T = {{v_0, v_1}, {v_1, v_2}, dots, {v_(n-2), v_(n-1)}}$ form a spanning tree:

- *Spanning:* $P$ visits all $n$ vertices, so $V(T) = V$.
- *Tree:* $|T| = n - 1$ edges and $T$ is connected (it is a path), so $T$ is a tree.
- *Degree bound:* Each interior vertex $v_i$ ($0 < i < n-1$) has degree exactly 2 in $T$
  (edges to $v_(i-1)$ and $v_(i+1)$). Each endpoint ($v_0$ and $v_(n-1)$) has degree 1.
  Thus $max "deg"(T) <= 2 = K$. #sym.checkmark

=== Backward (degree-2 spanning tree exists $arrow.r.double$ $G$ has a Hamiltonian path)

Let $T$ be a spanning tree of $G$ with maximum degree at most 2. We claim $T$ is a
Hamiltonian path.

A connected acyclic graph (tree) on $n$ vertices in which every vertex has degree at
most 2 must be a simple path:

- A tree with $n$ vertices has exactly $n - 1$ edges.
- If every vertex has degree $<= 2$, the tree has no branching (a branch point would
  require degree $>= 3$).
- A connected graph with no branching and no cycles is a simple path.

Since $T$ spans all $n$ vertices, $T$ is a Hamiltonian path in $G$. #sym.checkmark

=== Infeasible Instances

If $G$ has no Hamiltonian path, then no spanning subgraph of $G$ that is a simple path
on all vertices exists. Equivalently, no spanning tree with maximum degree $<= 2$ exists,
because any such tree would be a Hamiltonian path (as shown above). #sym.checkmark

== Solution Extraction

*Source representation:* A Hamiltonian path is a permutation $(v_0, v_1, dots, v_(n-1))$
of $V$ such that ${v_i, v_(i+1)} in E$ for all $0 <= i < n - 1$.

*Target representation:* A configuration is a binary vector $c in {0, 1}^(|E|)$ where
$c_j = 1$ means edge $e_j$ is selected for the spanning tree.

*Extraction:* Given a target solution $c$ (edge selection for a degree-2 spanning tree):
+ Collect the selected edges $T = {e_j : c_j = 1}$.
+ Build the adjacency structure of $T$.
+ Find an endpoint (vertex with degree 1 in $T$). If $n = 1$, return $(0)$.
+ Walk the path from the endpoint, outputting the vertex sequence.

The resulting permutation is a valid Hamiltonian path in $G$.

== Overhead

$ "num_vertices"_"target" &= "num_vertices"_"source" \
  "num_edges"_"target" &= "num_edges"_"source" $

The graph is passed through unchanged; the degree bound $K = 2$ is a constant parameter.

== YES Example

*Source:* $G$ with $V = {0, 1, 2, 3, 4}$ and
$E = {{0,1}, {0,3}, {1,2}, {1,3}, {2,3}, {2,4}, {3,4}}$.

Hamiltonian path: $0 arrow 1 arrow 2 arrow 4 arrow 3$.
Check: ${0,1} in E$, ${1,2} in E$, ${2,4} in E$, ${4,3} in E$. #sym.checkmark

*Target:* $G' = G$, $K = 2$.

Spanning tree edges: ${0,1}, {1,2}, {2,4}, {4,3}$ (same as path edges).

Degree check: $"deg"(0) = 1, "deg"(1) = 2, "deg"(2) = 2, "deg"(3) = 1, "deg"(4) = 2$.
Maximum degree $= 2 <= K = 2$. #sym.checkmark

== NO Example

*Source:* $G' = K_(1,4)$ plus edge ${1, 2}$. Vertices ${0, 1, 2, 3, 4}$,
edges $= {{0,1}, {0,2}, {0,3}, {0,4}, {1,2}}$.

No Hamiltonian path exists: vertices 3 and 4 each connect only to vertex 0,
so any spanning path must use both edges ${0,3}$ and ${0,4}$, giving vertex 0
degree $>= 2$ in the path. But vertex 0 must also connect to vertex 1 or 2
(since $G'$ has no other edges reaching 3 or 4), requiring degree $>= 3$ at
vertex 0 -- impossible in a path.

*Target:* $G' = G$, $K = 2$. Any spanning tree must include edges ${0,3}$ and
${0,4}$ (since 3 and 4 are pendant vertices). Together with a third edge
incident to 0 for connectivity to vertices 1 and 2, vertex 0 gets degree $>= 3 > K$.
No degree-2 spanning tree exists. #sym.checkmark
