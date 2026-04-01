// Verification proof: SubsetSum → Partition (#973)
#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules

#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

#show: thmrules.with(qed-symbol: $square$)
#let theorem = thmbox("theorem", "Theorem", fill: rgb("#e8e8f8"))
#let proof = thmproof("proof", "Proof")

== Subset Sum $arrow.r$ Partition <sec:subsetsum-partition>

#theorem[
  Subset Sum is polynomial-time reducible to Partition.
  Given a Subset Sum instance with $n$ elements and target $T$, the
  constructed Partition instance has at most $n + 1$ elements.
] <thm:subsetsum-partition>

#proof[
  _Construction._
  Let $(S, T)$ be a Subset Sum instance where $S = {s_1, dots, s_n}$ is a
  multiset of positive integers and $T$ is the target sum. Define
  $Sigma = sum_(i=1)^n s_i$.

  Construct a Partition instance $S'$ as follows.

  + Compute the padding value $d = |Sigma - 2T|$.

  + If $d = 0$ (equivalently, $Sigma = 2T$): set $S' = S$, containing the
    same $n$ elements.

  + If $d > 0$: set $S' = S union {d}$, appending $d$ as the $(n+1)$-th
    element.

  Denote $Sigma' = sum_(a in S') a$ (the total sum of the Partition instance)
  and $H = Sigma' / 2$ (the half-sum target for Partition). We compute $H$
  for each case:
  - If $Sigma = 2T$: $Sigma' = Sigma = 2T$, so $H = T$.
  - If $Sigma > 2T$: $d = Sigma - 2T$, so $Sigma' = Sigma + d = 2(Sigma - T)$, and $H = Sigma - T$.
  - If $Sigma < 2T$: $d = 2T - Sigma$, so $Sigma' = Sigma + d = 2T$, and $H = T$.

  _Correctness._

  We prove the equivalence in three cases.

  *Case 1: $Sigma = 2T$ (no padding, $d = 0$).*

  ($arrow.r.double$) Suppose $A subset.eq S$ with $sum_(a in A) a = T$. Then
  $sum_(a in S without A) a = Sigma - T = 2T - T = T$. The partition
  $(A, S without A)$ has equal sums $T = H$.

  ($arrow.l.double$) Suppose $(P_1, P_2)$ is a balanced partition of $S'$ with
  $sum P_1 = sum P_2 = H = T$. Then $P_1 subset.eq S$ and
  $sum_(a in P_1) a = T$, so $P_1$ is a valid Subset Sum solution.

  *Case 2: $Sigma > 2T$ ($d = Sigma - 2T > 0$, $H = Sigma - T$).*

  ($arrow.r.double$) Suppose $A subset.eq S$ with $sum_(a in A) a = T$. Place
  $A union {d}$ on one side and $S without A$ on the other:
  $ sum (A union {d}) = T + (Sigma - 2T) = Sigma - T = H. $
  $ sum (S without A) = Sigma - T = H. $

  ($arrow.l.double$) Suppose $(P_1, P_2)$ is a balanced partition of $S'$
  with $sum P_1 = sum P_2 = H = Sigma - T$. The padding element $d$ lies in
  exactly one part; without loss of generality say $d in P_1$. The
  $S$-elements in $P_1$ sum to $H - d = (Sigma - T) - (Sigma - 2T) = T$.
  These elements form a valid Subset Sum solution.

  *Case 3: $Sigma < 2T$ ($d = 2T - Sigma > 0$, $H = T$).*

  ($arrow.r.double$) Suppose $A subset.eq S$ with $sum_(a in A) a = T$. Place
  $A$ on one side and $(S without A) union {d}$ on the other:
  $ sum A = T = H. $
  $ sum ((S without A) union {d}) = (Sigma - T) + (2T - Sigma) = T = H. $

  ($arrow.l.double$) Suppose $(P_1, P_2)$ is a balanced partition of $S'$
  with $sum P_1 = sum P_2 = H = T$. The padding element $d$ lies in exactly
  one part; without loss of generality say $d in P_1$. The $S$-elements in
  $P_2$ (the part without $d$) sum to $H = T$, forming a valid Subset Sum
  solution.

  *Infeasible case: $T > Sigma$.*
  No subset of $S$ can sum to $T$ because the total sum of all elements is
  only $Sigma < T$. Here $d = 2T - Sigma > Sigma$, so $Sigma' = 2T$ and
  $H = T$. Since $d > Sigma >= s_i$ for all $i$, the padding element $d$
  alone exceeds the sum of all original elements. In any partition, the side
  containing $d$ has sum at least $d = 2T - Sigma > T = H$ (because $T > Sigma$
  implies $2T - Sigma > T$). So no balanced partition exists.

  _Solution extraction._
  Given a balanced partition $(P_1, P_2)$ of $S'$:
  - If $d = 0$: either side is a valid Subset Sum solution. Return the
    indicator of $P_1$ restricted to the original elements.
  - If $Sigma > 2T$: the $S$-elements on the *same side as $d$* sum to $T$.
    Return those elements.
  - If $Sigma < 2T$: the $S$-elements on the *opposite side from $d$* sum to
    $T$. Return those elements.

  In all cases, the extraction identifies which original elements form a
  subset summing to $T$ by locating the padding element (if present) and
  selecting the appropriate side.
]

*Overhead.*
#table(
  columns: (auto, auto),
  align: (left, left),
  table.header([Target metric], [Formula]),
  [`num_elements`], [$n + 1$ (worst case); $n$ when $Sigma = 2T$],
)

*Feasible example (YES instance).*
Consider the Subset Sum instance with elements $S = {3, 5, 7, 1, 4}$ ($n = 5$)
and target $T = 8$.

$Sigma = 3 + 5 + 7 + 1 + 4 = 20$, and $2T = 16$, so $Sigma > 2T$. The padding
value is $d = Sigma - 2T = 20 - 16 = 4$.

Constructed Partition instance: $S' = {3, 5, 7, 1, 4, 4}$ ($n + 1 = 6$ elements).
$Sigma' = 24$, $H = 12$.

The subset $A = {3, 5}$ satisfies $3 + 5 = 8 = T$.

Partition solution (Case 2, $Sigma > 2T$): place $A union {d} = {3, 5, 4_"pad"}$
on one side (sum $= 8 + 4 = 12 = H$) and ${7, 1, 4}$ on the other
(sum $= 7 + 1 + 4 = 12 = H$).

In binary config over $S' = {3, 5, 7, 1, 4, 4_"pad"}$:
config $= [0, 0, 1, 1, 1, 0]$ (where 0 = side with $A union {d}$,
1 = other side).

Extraction: padding element (index 5) is on side 0. Since $Sigma > 2T$,
the $S$-elements on the same side as padding are ${s_1 = 3, s_2 = 5}$, which
sum to $8 = T$. $checkmark$

Verification:
- Side 0: $3 + 5 + 4_"pad" = 12$ $checkmark$
- Side 1: $7 + 1 + 4 = 12$ $checkmark$
- Extracted subset: ${3, 5}$, sum $= 8 = T$ $checkmark$

*Infeasible example (NO instance).*
Consider the Subset Sum instance with elements $S = {3, 7, 11}$ ($n = 3$) and
target $T = 5$.

$Sigma = 3 + 7 + 11 = 21$, and $2T = 10$, so $Sigma > 2T$. The padding value
is $d = Sigma - 2T = 21 - 10 = 11$.

Constructed Partition instance: $S' = {3, 7, 11, 11}$ ($n + 1 = 4$ elements).
$Sigma' = 32$, $H = 16$.

No subset of $S = {3, 7, 11}$ sums to $T = 5$: the possible subset sums are
$0, 3, 7, 10, 11, 14, 18, 21$, none of which equals 5.

For the Partition instance, the half-sum target is $H = 16$. There are
$2^4 = 16$ possible binary assignments over 4 elements. The achievable
subset sums of ${3, 7, 11, 11}$ are:
$0, 3, 7, 10, 11, 14, 18, 21, 22, 25, 29, 32$, none of which equals 16.
(Here 11 appears twice, but the two copies are distinguishable as original
vs padding.) Since no subset sums to $H = 16$, no balanced partition
exists. $checkmark$
