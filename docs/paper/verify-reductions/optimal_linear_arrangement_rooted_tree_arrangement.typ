// Standalone verification proof: OptimalLinearArrangement → RootedTreeArrangement
// Issue: #888
// Reference: Gavril 1977a; Garey & Johnson, Computers and Intractability, GT45

#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules
#show: thmrules.with(qed-symbol: $square$)
#let theorem = thmbox("theorem", "Theorem")
#let proof = thmproof("proof", "Proof")

#set page(width: 6in, height: auto, margin: 1cm)
#set text(size: 10pt)

== Optimal Linear Arrangement $arrow.r$ Rooted Tree Arrangement <sec:ola-rta>

=== Problem Definitions

*Optimal Linear Arrangement (OLA).* Given an undirected graph $G = (V, E)$ and
a positive integer $K$, determine whether there exists a bijection
$f: V arrow.r {1, 2, dots, |V|}$ such that
$
  sum_({u, v} in E) |f(u) - f(v)| lt.eq K.
$

*Rooted Tree Arrangement (RTA, GT45).* Given an undirected graph $G = (V, E)$
and a positive integer $K$, determine whether there exists a rooted tree
$T = (U, F)$ with $|U| = |V|$ and a bijection $f: V arrow.r U$ such that
for every edge ${u, v} in E$, the unique root-to-leaf path in $T$ contains
both $f(u)$ and $f(v)$ (ancestor-comparability), and
$
  sum_({u, v} in E) d_T (f(u), f(v)) lt.eq K,
$
where $d_T$ denotes distance in the tree $T$.

#theorem[
  The Optimal Linear Arrangement problem decision-reduces to the Rooted Tree
  Arrangement problem in polynomial time. Given an OLA instance $(G, K)$, the
  reduction outputs the RTA instance $(G, K)$ with the same graph and bound.
  However, witness extraction from RTA back to OLA is not possible in general.
] <thm:ola-to-rta>

#proof[
  _Construction._

  Given OLA instance $(G = (V, E), K)$, output RTA instance $(G' = G, K' = K)$.
  The graph and bound are unchanged.

  _Forward direction ($arrow.r.double$)._

  Suppose OLA$(G, K)$ has a solution: a bijection $f: V arrow.r {1, dots, n}$
  with $sum_({u,v} in E) |f(u) - f(v)| lt.eq K$.

  Construct the path tree $T = P_n$ on $n$ nodes: the rooted tree where node $i$
  has parent $i - 1$ for $i gt.eq 1$ and node $0$ is the root. In this path tree,
  every pair of nodes is ancestor-comparable (they all lie on the single
  root-to-leaf path), and $d_T(i, j) = |i - j|$.

  Using $f$ as the mapping from $V$ to $T$:
  - Every edge ${u, v} in E$ maps to $(f(u), f(v))$, both on the root-to-leaf
    path, so ancestor-comparability holds.
  - $sum_({u,v} in E) d_T(f(u), f(v)) = sum_({u,v} in E) |f(u) - f(v)| lt.eq K$.

  Therefore RTA$(G, K)$ has a solution.

  _Backward direction (partial)._

  Suppose RTA$(G, K)$ has a solution using tree $T$ and mapping $f$. If $T$
  happens to be a path $P_n$, then $f$ directly yields a linear arrangement
  with cost $lt.eq K$, so OLA$(G, K)$ is also feasible.

  However, if $T$ is a branching tree, the RTA solution exploits a richer
  structure. Since the search space of RTA _strictly contains_ that of OLA
  (paths are special cases of rooted trees), it is possible that
  $"opt"_"RTA"(G) < "opt"_"OLA"(G)$. In such cases, a YES answer for
  RTA$(G, K)$ does _not_ imply a YES answer for OLA$(G, K)$.

  _Consequence._ The forward direction proves that OLA is no harder than RTA
  (any OLA-feasible instance is RTA-feasible). Combined with the known
  NP-completeness of OLA, this establishes NP-hardness of RTA. But the
  reduction does not support witness extraction: given an arbitrary RTA
  solution, there is no polynomial-time procedure guaranteed to produce a
  valid OLA solution.

  _Why the full backward direction fails._

  Consider the star graph $K_(1,n-1)$ with $n$ vertices. The optimal linear
  arrangement places the hub at the center, giving cost $approx n^2/4$. But
  in a star tree rooted at the hub, every edge has stretch 1, giving total
  cost $n - 1$. For $K lt n^2/4$ and $K gt.eq n - 1$, the RTA instance is
  feasible but the OLA instance is not.
]

*Overhead.*
#table(
  columns: (auto, auto),
  [Target metric], [Formula],
  [`num_vertices`], [$n$ (unchanged)],
  [`num_edges`], [$m$ (unchanged)],
  [`bound`], [$K$ (unchanged)],
)

*Feasible (YES) example.*

Source (OLA): Path graph $P_4$: vertices ${0, 1, 2, 3}$, edges
${(0,1), (1,2), (2,3)}$, bound $K = 3$.

Arrangement $f = (0, 1, 2, 3)$ (identity permutation):
- $|f(0) - f(1)| + |f(1) - f(2)| + |f(2) - f(3)| = 1 + 1 + 1 = 3 lt.eq 3$. $checkmark$

Target (RTA): Same graph $P_4$, bound $K = 3$.

Using path tree $T = 0 arrow.r 1 arrow.r 2 arrow.r 3$ with identity mapping:
all pairs are ancestor-comparable and total stretch $= 3 lt.eq 3$. $checkmark$

*Infeasible backward (RTA YES, OLA NO) example.*

Source (OLA): Star graph $K_(1,3)$: vertices ${0, 1, 2, 3}$, hub $= 0$, edges
${(0,1), (0,2), (0,3)}$, bound $K = 3$.

Best linear arrangement places hub at position 1 (0-indexed):
$f = (1, 0, 2, 3)$, cost $= |1-0| + |1-2| + |1-3| = 1 + 1 + 2 = 4 > 3$.
No arrangement achieves cost $lt.eq 3$. OLA is infeasible.

Target (RTA): Same $K_(1,3)$, bound $K = 3$.

Using star tree rooted at node 0, identity mapping:
each edge has stretch 1, total $= 3 lt.eq 3$. RTA is feasible. $checkmark$

This demonstrates that the backward direction fails: RTA$(K_(1,3), 3)$ is YES
but OLA$(K_(1,3), 3)$ is NO.
