// Standalone verification proof: ThreeDimensionalMatching -> Numerical3DimensionalMatching
// Issue #390 -- 3-DIMENSIONAL MATCHING to NUMERICAL 3-DIMENSIONAL MATCHING
// Reference: Garey & Johnson, SP16, p.224
// Status: BLOCKED -- No known direct reduction; see analysis below.

#set page(width: 210mm, height: auto, margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.1.")
#set math.equation(numbering: "(1)")

#let theorem(body) = block(
  width: 100%, inset: 10pt, fill: rgb("#e8f0fe"), radius: 4pt,
  [*Theorem.* #body]
)
#let proof(body) = block(
  width: 100%, inset: (left: 10pt),
  [*Proof.* #body #h(1fr) $square$]
)

= Three-Dimensional Matching $arrow.r$ Numerical 3-Dimensional Matching <sec:threedimensionalmatching-numerical3dimensionalmatching>

== Problem Definitions

*Three-Dimensional Matching (3DM).* Given disjoint sets $W = {0, dots, q-1}$, $X = {0, dots, q-1}$, $Y = {0, dots, q-1}$ and a collection $M = {t_0, dots, t_(m-1)}$ of triples where $t_j = (w_j, x_j, y_j)$ with $w_j in W$, $x_j in X$, $y_j in Y$, determine whether there exists a subcollection $M' subset.eq M$ with $|M'| = q$ such that every element of $W union X union Y$ appears in exactly one triple of $M'$.

*Numerical 3-Dimensional Matching (N3DM).* Given disjoint sets $W'$, $X'$, $Y'$ each with $n$ elements, a positive integer size $s(a)$ for every element $a in W' union X' union Y'$ satisfying $B slash 4 < s(a) < B slash 2$, and a bound $B$ such that the total sum equals $n B$, determine whether $W' union X' union Y'$ can be partitioned into $n$ triples, each containing one element from $W'$, one from $X'$, and one from $Y'$, with each triple summing to exactly $B$.

== Impossibility of Direct Additive Reduction

#theorem[
  No polynomial-time reduction from 3DM to N3DM exists using a simple additive encoding where sizes of individual elements depend only on their coordinates, with a constant per-group bound $B$.
]

#proof[
  _Setup._ Consider a hypothetical reduction that creates an N3DM instance with $n = q$ groups, where each group corresponds to a W-element. The configuration $(sigma, tau)$ assigns X-element $sigma(w)$ and Y-element $tau(w)$ to group $w$, forming the triple $(w, sigma(w), tau(w))$.

  _Separability requirement._ For the reduction to be correct, we need:
  $ s_W (w) + s_X (sigma(w)) + s_Y (tau(w)) = B quad forall w in {0, dots, q-1} $
  if and only if $(w, sigma(w), tau(w)) in M$ for all $w$. This requires the indicator function $I(w, x, y) = [(w, x, y) in M]$ to be representable as a constant level set of an additively separable function $f(w) + g(x) + h(y) = B$.

  _Counterexample._ Consider $q = 2$ and $M = {(0, 0, 0), (0, 1, 1), (1, 0, 1), (1, 1, 0)}$ (all triples where $w + x + y equiv 0 mod 2$). Suppose $f, g, h, B$ exist such that $f(w) + g(x) + h(y) = B$ iff $(w, x, y) in M$.

  From $(0,0,0) in M$: $f(0) + g(0) + h(0) = B$.

  From $(0,1,1) in M$: $f(0) + g(1) + h(1) = B$.

  From $(0,0,1) in.not M$: $f(0) + g(0) + h(1) != B$.

  From the first two: $g(0) + h(0) = g(1) + h(1)$, so $g(0) - g(1) = h(1) - h(0) = delta$ for some $delta != 0$ (otherwise $(0,0,1)$ would also give $B$).

  From $(1,0,1) in M$: $f(1) + g(0) + h(1) = B$. Combined with $(0,0,0) in M$: $f(1) - f(0) = h(0) - h(1) = -delta$.

  Now check $(1,1,1) in.not M$: $f(1) + g(1) + h(1) = f(0) - delta + g(0) - delta + h(0) + delta = B - delta != B$ since $delta != 0$. Consistent.

  Check $(1,0,0) in.not M$: $f(1) + g(0) + h(0) = f(0) - delta + g(0) + h(0) = B - delta != B$. Consistent.

  Check $(0,1,0) in.not M$: $f(0) + g(1) + h(0) = f(0) + g(0) - delta + h(0) = B - delta != B$. Consistent.

  Check $(1,1,0) in M$: $f(1) + g(1) + h(0) = (f(0) - delta) + (g(0) - delta) + h(0) = B - 2 delta$. For this to equal $B$: $delta = 0$, contradicting $delta != 0$.

  Therefore no $f, g, h, B$ exist for this $M$. The indicator function of $M$ is not representable as a constant level set of an additively separable function.

  _Generalization to $n = m$ groups._ With $m$ groups (one per triple), the coordinate-complement construction can enforce X-coverage and Y-coverage through competition for shared real elements. However, W-coverage (requiring the active triples to cover each W-element exactly once) requires distinguishing groups by their W-coordinate within the per-group sum. Since $B$ is a single constant shared by all groups, and the W-coordinate varies across groups, no additive encoding can enforce W-distinctness among the active groups.

  _Counterexample for $n = m$._ Let $q = 2$, $M = {(0,0,0), (0,1,1)}$. The 3DM instance is infeasible because W-element 1 is uncovered. Using the coordinate-complement construction with $m = 2$ groups:
  - $s_W (0) = P + D dot (q - 0) + (q - 0) = P + 2D + 2$
  - $s_W (1) = P + D dot (q - 1) + (q - 1) = P + D + 1$
  - $s_X (0) = P$, $s_X (1) = P + D$
  - $s_Y (0) = P$, $s_Y (1) = P + 1$
  - $B = 3P + D q + q = 3P + 2D + 2$

  With $sigma = (0, 1)$, $tau = (0, 1)$ (identity):
  - Group 0: $s_W (0) + s_X (0) + s_Y (0) = (P + 2D + 2) + P + P = 3P + 2D + 2 = B$
  - Group 1: $s_W (1) + s_X (1) + s_Y (1) = (P + D + 1) + (P + D) + (P + 1) = 3P + 2D + 2 = B$

  The N3DM instance is feasible, but the 3DM instance is infeasible (W-element 1 uncovered). The reduction is incorrect.
]

== Standard Reduction Chain

The NP-completeness of N3DM is established through the following chain of reductions, as described in Garey and Johnson (1979):

$ sans("3DM") arrow.r sans("4-PARTITION") arrow.r sans("3-PARTITION") $

N3DM is a special case of both 3-Partition and 3DM. Its NP-completeness follows from 3-Partition, which is proved NP-complete via the above chain.

The reduction from 3DM to 4-Partition uses the construction from the Garey and Johnson compendium:
- Choose $r = 32q$ where $q$ is the 3DM universe size.
- For each triple $t = (w_i, x_j, y_k)$: create element $u_t = 10 r^4 - k r^3 - j r^2 - i r$.
- For each W-element $w_i$: one "real" copy with size $10 r^4 + i r$ and multiple "dummy" copies with size $11 r^4 + i r$.
- For each X-element $x_j$: real $10 r^4 + j r^2$, dummy $11 r^4 + j r^2$.
- For each Y-element $y_k$: real $10 r^4 + k r^3$, dummy $8 r^4 + k r^3$.
- Target $T = 40 r^4$.

Each valid 4-partition group combines one triple-element with one element from each coordinate set. A valid 4-partition exists if and only if the 3DM instance has a matching.

Adapting this to N3DM (3 elements per group, tripartite structure) requires merging two of the four roles into one N3DM set, which creates size-bound violations ($B slash 4 < s < B slash 2$). This is why the standard approach first reduces to 3-Partition (an unconstrained version) and then observes that N3DM is a special case.

== Conclusion

A direct, single-step polynomial reduction from 3DM to N3DM using additive numerical encoding does not exist. The issue claim of a direct reduction (G\&J SP16, p.224) refers to the NP-completeness proof chain, not a single-step transformation. The standard proof of N3DM's NP-completeness proceeds through 4-Partition and 3-Partition.

For the codebase implementation of this reduction rule, one would need to either:
1. Implement the composed 3DM $arrow.r$ 4-Partition $arrow.r$ 3-Partition $arrow.r$ N3DM chain.
2. Find an alternative NP-completeness proof for N3DM that provides a cleaner single-step reduction from a different source problem (e.g., the linear reduction from NAE-SAT by Caracciolo, Fichera, and Sportiello, 2006).

== Feasible Example (for partial coordinate-complement construction)

Consider the 3DM instance with $q = 3$, $W = X = Y = {0, 1, 2}$, and $m = 5$ triples:
$ t_0 = (0, 1, 2), quad t_1 = (1, 0, 1), quad t_2 = (2, 2, 0), quad t_3 = (0, 0, 0), quad t_4 = (1, 2, 2) $

*Valid matching.* $M' = {t_0, t_1, t_2}$: covers $W = {0, 1, 2}$, $X = {1, 0, 2}$, $Y = {2, 1, 0}$.

Using the coordinate-complement encoding with $D = 4$, $P = 128$, $B = 399$:
- $s_W (0) = 128 + 4 dot 2 + 1 = 137$, $s_X (1) = 128 + 4 = 132$, $s_Y (2) = 130$. Sum $= 399 = B$.
- $s_W (1) = 128 + 4 dot 3 + 2 = 142$, $s_X (0) = 128$, $s_Y (1) = 129$. Sum $= 399 = B$.
- $s_W (2) = 128 + 4 dot 1 + 3 = 135$, $s_X (2) = 128 + 8 = 136$, $s_Y (0) = 128$. Sum $= 399 = B$.

The partial construction correctly verifies X-coverage and Y-coverage. W-coverage is satisfied in this case but is not guaranteed in general.

== Infeasible Example

Consider the 3DM instance with $q = 2$, $M = {(0, 0, 0), (0, 1, 1)}$.

*Why no valid matching exists.* Both triples have $w_j = 0$. W-element 1 cannot be covered by any triple in $M$.

*Coordinate-complement construction failure.* The N3DM instance has $B = 308$ and the identity permutation achieves all sums equal to $B$, making the N3DM instance feasible despite the 3DM instance being infeasible. This demonstrates the W-coverage enforcement gap in the direct additive construction.
