// Verification proof: Partition → KthLargestMTuple
// Issue: #395
// Reference: Garey & Johnson, Computers and Intractability, SP21, p.225
// Original: Johnson and Mizoguchi (1978)

= Partition $arrow.r$ Kth Largest $m$-Tuple

== Problem Definitions

*Partition (SP12).* Given a finite set $A = {a_1, dots, a_n}$ with sizes
$s(a_i) in bb(Z)^+$ and total $S = sum_(i=1)^n s(a_i)$, determine whether
there exists a subset $A' subset.eq A$ such that
$sum_(a in A') s(a) = S slash 2$.

*Kth Largest $m$-Tuple (SP21).* Given sets $X_1, X_2, dots, X_m subset.eq bb(Z)^+$,
a size function $s: union.big X_i arrow bb(Z)^+$, and positive integers $K$ and $B$,
determine whether there are $K$ or more distinct $m$-tuples
$(x_1, dots, x_m) in X_1 times dots times X_m$ for which
$sum_(i=1)^m s(x_i) gt.eq B$.

== Reduction

Given a Partition instance $A = {a_1, dots, a_n}$ with sizes $s(a_i)$ and
total $S = sum s(a_i)$:

+ *Sets:* For each $i = 1, dots, n$, define $X_i = {0^*, s(a_i)}$ where $0^*$
  is a distinguished placeholder with size $0$.
  (In the code model, sizes must be positive, so we use index-based selection:
  each set has two elements and we track which is "include" vs "exclude".)
+ *Bound:* Set $B = ceil(S slash 2)$.
+ *Threshold:* Compute $C = |{(x_1, dots, x_n) in X_1 times dots times X_n : sum x_i > S slash 2}|$
  (the count of tuples with sum strictly exceeding half). Set $K = C + 1$.

*Note.* Computing $C$ requires enumerating or counting subsets, making this
a *Turing reduction* (polynomial-time with oracle access), not a standard
many-one reduction. The (*) in GJ indicates the target problem is not known
to be in NP.

== Correctness Proof

Each $m$-tuple $(x_1, dots, x_n) in X_1 times dots times X_n$ corresponds
bijectively to a subset $A' subset.eq A$ via $a_i in A' iff x_i = s(a_i)$.
The tuple sum $sum x_i = sum_(a_i in A') s(a_i)$.

=== Forward: YES Partition $arrow.r$ YES KthLargestMTuple

Suppose $A' subset.eq A$ satisfies $sum_(a in A') s(a) = S slash 2$.

The tuples with sum $gt.eq B = ceil(S slash 2)$ are:
- All tuples corresponding to subsets with sum $> S slash 2$ (there are $C$ of these).
- All tuples corresponding to subsets with sum $= S slash 2$ (at least 1, namely $A'$).

So the count of qualifying tuples is $gt.eq C + 1 = K$, and the answer is YES.

When $S$ is even, subsets summing to exactly $S slash 2$ exist (the partition),
and $B = S slash 2$. The qualifying count is $C + P$ where $P gt.eq 1$ is the
number of balanced partitions. Since $C + P gt.eq C + 1 = K$, the answer is YES.

=== Backward: YES KthLargestMTuple $arrow.r$ YES Partition

Suppose there are $gt.eq K = C + 1$ tuples with sum $gt.eq B$.

By construction, there are exactly $C$ tuples with sum $> S slash 2$.
Since there are $gt.eq C + 1$ tuples with sum $gt.eq B gt.eq S slash 2$,
at least one tuple has sum $gt.eq B$ but $lt.eq S slash 2$... this is
impossible unless $B = S slash 2$ (which happens when $S$ is even).

More precisely: when $S$ is even, $B = S slash 2$, and the tuples with sum
$gt.eq B$ include those with sum $= S slash 2$ and those with sum $> S slash 2$.
Since $C$ counts only strict-greater, having $gt.eq C + 1$ qualifying tuples
means at least one tuple has sum exactly $S slash 2$, i.e., a balanced partition exists.

When $S$ is odd, $B = ceil(S slash 2) = (S+1) slash 2$. No integer subset sum
can equal $S slash 2$ (not an integer). The tuples with sum $gt.eq (S+1) slash 2$
are exactly those with sum $> S slash 2$ (since sums are integers). So the count
of qualifying tuples equals $C$, and $K = C + 1 > C$ means the answer is NO.
This is consistent since odd-sum Partition instances are always NO.

=== Infeasible Instances

If $S$ is odd, no balanced partition exists. We have $B = (S+1) slash 2$ and
the qualifying count is exactly $C$ (tuples with integer sum $gt.eq (S+1) slash 2$
are the same as those with sum $> S slash 2$). Since $K = C + 1 > C$, the
KthLargestMTuple answer is NO, matching the Partition answer.

If $S$ is even but no subset sums to $S slash 2$, then all qualifying tuples have
sum strictly $> S slash 2$, so the count is exactly $C$. Again $K = C + 1 > C$
yields NO.

== Solution Extraction

This is a Turing reduction: we do not extract a Partition solution from a
KthLargestMTuple answer. The KthLargestMTuple problem returns a YES/NO count
comparison, not a witness. The reduction preserves feasibility (YES/NO).

== Overhead

$ m &= n = "num_elements" \
  "num_sets" &= "num_elements" \
  "total_set_sizes" &= 2 dot "num_elements" \
  "total_tuples" &= 2^"num_elements" $

Each element maps to a 2-element set. The total tuple space is $2^n$, which is
exponential — but the *description* of the target instance is polynomial ($O(n)$).

== YES Example

*Source:* $A = {3, 1, 1, 2, 2, 1}$, $S = 10$, half-sum $= 5$.

Balanced partition exists: $A' = {a_1, a_4} = {3, 2}$ with sum $5$.

*Target:*
- $X_1 = {0, 3}$, $X_2 = {0, 1}$, $X_3 = {0, 1}$, $X_4 = {0, 2}$, $X_5 = {0, 2}$, $X_6 = {0, 1}$
- $B = 5$
- $C = 27$ (subsets with sum $> 5$), $K = 28$

Qualifying tuples (sum $gt.eq 5$): $27 + 10 = 37 gt.eq 28$ $arrow.r$ YES. #sym.checkmark

== NO Example

*Source:* $A = {5, 3, 3}$, $S = 11$ (odd, no partition possible).

*Target:*
- $X_1 = {0, 5}$, $X_2 = {0, 3}$, $X_3 = {0, 3}$
- $B = ceil(11 slash 2) = 6$
- Subsets with sum $> 5.5$ (equivalently sum $gt.eq 6$): ${5,3_a} = 8$, ${5,3_b} = 8$, ${5,3_a,3_b} = 11$, ${3_a,3_b} = 6$ $arrow.r$ $C = 4$
- $K = 5$

Qualifying tuples (sum $gt.eq 6$): exactly $4 < 5$ $arrow.r$ NO. #sym.checkmark
