// Verification proof: SubsetSum → IntegerExpressionMembership
// Issue: #569
// Reference: Stockmeyer and Meyer (1973); Garey & Johnson, Appendix A7.3, p.253

= Subset Sum $arrow.r$ Integer Expression Membership

== Problem Definitions

*Subset Sum (SP13).* Given a finite set $S = {s_1, dots, s_n}$ of positive integers and
a target $B in bb(Z)^+$, determine whether there exists a subset $A' subset.eq S$ such that
$sum_(a in A') a = B$.

*Integer Expression Membership (AN18).* Given an integer expression $e$ over
the operations $union$ (set union) and $+$ (Minkowski sum), where atoms are positive
integers, and a positive integer $K$, determine whether $K in op("eval")(e)$.

The Minkowski sum of two sets is $F + G = {m + n : m in F, n in G}$.

== Reduction

Given a Subset Sum instance $(S, B)$ with $S = {s_1, dots, s_n}$:

+ For each element $s_i$, construct a "choice" expression
  $ c_i = (1 union (s_i + 1)) $
  representing the set ${1, s_i + 1}$. The atom $1$ encodes "skip this element"
  and the atom $s_i + 1$ encodes "select this element" (shifted by $1$ to keep
  all atoms positive).

+ Build the overall expression as the Minkowski-sum chain
  $ e = c_1 + c_2 + dots.c + c_n. $

+ Set the target $K = B + n$.

The resulting Integer Expression Membership instance is $(e, K)$.

== Correctness Proof

=== Forward ($"YES source" arrow.r "YES target"$)

Suppose $A' subset.eq S$ satisfies $sum_(a in A') a = B$. Define the choice for
each union node:
$ d_i = cases(s_i + 1 &"if" s_i in A', 1 &"otherwise".) $

Then
$ sum_(i=1)^n d_i
  = sum_(s_i in A') (s_i + 1) + sum_(s_i in.not A') 1
  = sum_(s_i in A') s_i + |A'| + (n - |A'|)
  = B + n = K. $
So $K in op("eval")(e)$. #sym.checkmark

=== Backward ($"YES target" arrow.r "YES source"$)

Suppose $K = B + n in op("eval")(e)$. Then there exist choices $d_i in {1, s_i + 1}$
for each $i$ with $sum d_i = B + n$. Let $A' = {s_i : d_i = s_i + 1}$ and
$k = |A'|$. Then
$ sum d_i = sum_(s_i in A') (s_i + 1) + (n - k) dot 1
  = sum_(s_i in A') s_i + k + n - k
  = sum_(s_i in A') s_i + n. $
Setting this equal to $B + n$ gives $sum_(s_i in A') s_i = B$. #sym.checkmark

=== Infeasible Instances

If no subset of $S$ sums to $B$, then for every choice $d_i in {1, s_i + 1}$,
the sum $sum d_i eq.not B + n$ (by the backward argument in contrapositive).
Hence $K in.not op("eval")(e)$. #sym.checkmark

== Solution Extraction

Given that $K in op("eval")(e)$ via union choices $(d_1, dots, d_n)$ (in DFS order,
one per union node), extract a Subset Sum solution:
$ x_i = cases(1 &"if" d_i = 1 " (right branch chosen, i.e., atom " s_i + 1 ")", 0 &"if" d_i = 0 " (left branch chosen, i.e., atom 1)".) $

In the IntegerExpressionMembership configuration encoding, each union node has
binary variable: $0 =$ left branch (atom $1$, skip), $1 =$ right branch
(atom $s_i + 1$, select). So the SubsetSum config is exactly the
IntegerExpressionMembership config.

== Overhead

The expression tree has $n$ union nodes, $2n$ atoms, and $n - 1$ sum nodes
(for $n >= 2$), giving a total tree size of $4n - 1$ nodes.

$ "expression_size" &= 4 dot "num_elements" - 1 quad (n >= 2) \
  "num_union_nodes" &= "num_elements" \
  "num_atoms" &= 2 dot "num_elements" \
  "target" &= B + "num_elements" $

== YES Example

*Source:* $S = {3, 5, 7}$, $B = 8$ ($n = 3$). Subset ${3, 5}$ sums to $8$.

*Constructed expression:*
$ e = (1 union 4) + (1 union 6) + (1 union 8), quad K = 8 + 3 = 11. $

*Set represented by $e$:*
All sums $d_1 + d_2 + d_3$ with $d_i in {1, s_i + 1}$:
${3, 6, 8, 10, 11, 13, 15, 18}$.

$K = 11 in op("eval")(e)$ via $d = (4, 6, 1)$, i.e., config $= (1, 1, 0)$.

*Extract:* $x = (1, 1, 0)$ $arrow.r$ select ${3, 5}$, sum $= 8 = B$. #sym.checkmark

== NO Example

*Source:* $S = {3, 7, 11}$, $B = 5$ ($n = 3$). No subset sums to $5$.

*Constructed expression:*
$ e = (1 union 4) + (1 union 8) + (1 union 12), quad K = 5 + 3 = 8. $

*Set represented by $e$:*
${3, 6, 10, 13, 14, 17, 21, 24}$.

$K = 8 in.not op("eval")(e)$. #sym.checkmark
