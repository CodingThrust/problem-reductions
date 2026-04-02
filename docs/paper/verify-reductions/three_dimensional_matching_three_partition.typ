// Verification proof: ThreeDimensionalMatching → ThreePartition
// Issue: #389
// Reference: Garey & Johnson, Computers and Intractability, SP15, p.224
// Chain: 3DM → ABCD-Partition → 4-Partition → 3-Partition
// (Garey & Johnson 1975; Wikipedia reconstruction)

= Three-Dimensional Matching $arrow.r$ 3-Partition

== Problem Definitions

*Three-Dimensional Matching (3DM, SP1).* Given disjoint sets
$W = {w_0, dots, w_(q-1)}$, $X = {x_0, dots, x_(q-1)}$,
$Y = {y_0, dots, y_(q-1)}$, each of size $q$, and a set $M$ of $t$
triples $(w_i, x_j, y_k)$ with $w_i in W$, $x_j in X$, $y_k in Y$,
determine whether there exists a subset $M' subset.eq M$ with
$|M'| = q$ such that no two triples in $M'$ agree in any coordinate.

*3-Partition (SP15).* Given $3m$ positive integers
$s_1, dots, s_(3m)$ with $B slash 4 < s_i < B slash 2$ for all $i$
and $sum s_i = m B$, determine whether the integers can be partitioned
into $m$ triples that each sum to $B$.

== Reduction Overview

The reduction composes three classical steps from Garey & Johnson (1975, 1979):

+ *3DM $arrow.r$ ABCD-Partition:* encode matching constraints into four
  numerically-typed sets.
+ *ABCD-Partition $arrow.r$ 4-Partition:* use modular tagging to remove
  set labels while preserving the one-from-each requirement.
+ *4-Partition $arrow.r$ 3-Partition:* introduce pairing and filler
  gadgets that split each 4-group into two 3-groups.

Each step runs in polynomial time; the composition is polynomial.

== Step 1: 3DM $arrow.r$ ABCD-Partition

Let $r := 32 q$.

For each triple $m_l = (w_(a_l), x_(b_l), y_(c_l))$ in $M$
($l = 0, dots, t-1$), create four elements:

$ u_l &= 10 r^4 - c_l r^3 - b_l r^2 - a_l r \
  w^l_(a_l) &= cases(
    10 r^4 + a_l r quad & "if first occurrence of" w_(a_l),
    11 r^4 + a_l r & "otherwise (dummy)"
  ) \
  x^l_(b_l) &= cases(
    10 r^4 + b_l r^2 & "if first occurrence of" x_(b_l),
    11 r^4 + b_l r^2 & "otherwise (dummy)"
  ) \
  y^l_(c_l) &= cases(
    10 r^4 + c_l r^3 & "if first occurrence of" y_(c_l),
    8 r^4 + c_l r^3 & "otherwise (dummy)"
  ) $

Target: $T_1 = 40 r^4$.

*Correctness.* A "real" triple (using first-occurrence elements) sums to
$(10 + 10 + 10 + 10) r^4 = 40 r^4 = T_1$ (the $r$, $r^2$, $r^3$
terms cancel). A "dummy" triple sums to
$(10 + 11 + 11 + 8) r^4 = 40 r^4 = T_1$. Any mixed combination fails
because the lower-order terms do not cancel (since $r = 32 q > 3 q$
prevents carries).

A valid ABCD-partition exists iff a perfect 3DM matching exists: real
triples cover each vertex exactly once.

== Step 2: ABCD-Partition $arrow.r$ 4-Partition

Given $4 t$ elements in sets $A, B, C, D$ with target $T_1$:

$ a'_l = 16 a_l + 1, quad b'_l = 16 b_l + 2, quad
  c'_l = 16 c_l + 4, quad d'_l = 16 d_l + 8 $

Target: $T_2 = 16 T_1 + 15$.

Since each element's residue mod 16 is unique to its source set
(1, 2, 4, 8), any 4-set summing to $T_2 equiv 15 pmod(16)$ must
contain exactly one element from each original set.

== Step 3: 4-Partition $arrow.r$ 3-Partition

Let the $4 t$ elements from Step 2 be $a_1, dots, a_(4 t)$ with target
$T_2$.

Create:

+ *Regular elements* ($4 t$ total): $w_i = 4(5 T_2 + a_i) + 1$.
+ *Pairing elements* ($4 t (4 t - 1)$ total): for each pair $(i, j)$
  with $i != j$:
  $ u_(i j) = 4(6 T_2 - a_i - a_j) + 2, quad
    u'_(i j) = 4(5 T_2 + a_i + a_j) + 2 $
+ *Filler elements* ($8 t^2 - 3 t$ total): each of size
  $f = 4 dot 5 T_2 = 20 T_2$.

Total: $24 t^2 - 3 t = 3(8 t^2 - t)$ elements in $m_3 = 8 t^2 - t$
groups.

Target: $B = 64 T_2 + 4$.

All element sizes lie in $(B slash 4, B slash 2)$.

*Correctness.*
- _Forward:_ each 4-group ${a_i, a_j, a_k, a_l}$ with sum $T_2$
  yields 3-groups ${w_i, w_j, u_(i j)}$ and ${w_k, w_l, u'_(i j)}$,
  each summing to $B$. Remaining pairs $(u_(k l), u'_(k l))$ pair with
  fillers.
- _Backward:_ residue mod 4 forces each 3-set to be either
  (2 regular + 1 pairing) or (2 pairing + 1 filler). Filler groups force
  $u_(i j) + u'_(i j) = 44 T_2 + 4$, recovering the original 4-partition
  structure.

== Solution Extraction

Given a 3-Partition solution, reverse the three steps:

+ Identify filler groups (contain a filler element); their paired
  $u, u'$ elements reveal the original $(i, j)$ pairs.
+ The remaining 3-sets contain two regular elements $w_i, w_j$ plus one
  pairing element $u_(i j)$. Group the four regular elements of each
  pair of 3-sets into a 4-set.
+ Undo the modular tagging to recover the ABCD-partition sets.
+ Each "real" ABCD-group corresponds to a triple in the matching;
  read off the matching from the $u_l$ elements (decode $a_l, b_l, c_l$
  from the lower-order terms).

== Overhead

#table(
  columns: (auto, auto),
  [Target metric], [Formula],
  [`num_elements`], [$24 t^2 - 3 t$ where $t = |M|$],
  [`num_groups`], [$8 t^2 - t$],
  [`bound`], [$64(16 dot 40 r^4 + 15) + 4$ where $r = 32 q$],
)

== YES Example

*Source:* $q = 2$, $M = {(0, 0, 1), (1, 1, 0), (0, 1, 1), (1, 0, 0)}$
($t = 4$ triples).

Matching: ${(0, 0, 1), (1, 1, 0)}$ covers $W = {0, 1}$, $X = {0, 1}$,
$Y = {0, 1}$ exactly. #sym.checkmark

The reduction produces a 3-Partition instance with
$24 dot 16 - 12 = 372$ elements in $124$ groups.
The 3-Partition instance is feasible (by forward construction from the
matching). #sym.checkmark

== NO Example

*Source:* $q = 2$, $M = {(0, 0, 0), (0, 1, 0), (1, 0, 0)}$ ($t = 3$).

No perfect matching exists: $y_1$ is never covered.

The reduction produces a 3-Partition instance with
$24 dot 9 - 9 = 207$ elements in $69$ groups.
The 3-Partition instance is infeasible. #sym.checkmark
