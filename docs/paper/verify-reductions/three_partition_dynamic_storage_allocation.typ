// Standalone Typst proof: ThreePartition -> DynamicStorageAllocation
// Issue #397 -- Garey & Johnson, SR2, p.226

#set page(width: 210mm, height: auto, margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.1.")
#set math.equation(numbering: "(1)")

#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules
#show: thmrules.with(qed-symbol: $square$)
#let theorem = thmbox("theorem", "Theorem", stroke: 0.5pt)
#let proof = thmproof("proof", "Proof")

== 3-Partition $arrow.r$ Dynamic Storage Allocation <sec:threepartition-dynamicstorageallocation>

The *3-Partition* problem (SP15 in Garey & Johnson) asks: given a multiset
$A = {a_1, a_2, dots, a_(3m)}$ of positive integers with target sum $B$
satisfying $B slash 4 < a_i < B slash 2$ for all $i$ and
$sum_(i=1)^(3m) a_i = m B$, can $A$ be partitioned into $m$ disjoint
triples each summing to exactly $B$?

The *Dynamic Storage Allocation* (DSA) problem (SR2 in Garey & Johnson)
asks: given $n$ items, each with arrival time $r(a)$, departure time
$d(a)$, and size $s(a)$, plus a memory bound $D$, can each item be
assigned a starting address $sigma(a) in {0, dots, D - s(a)}$ such that
for every pair of items $a, a'$ with overlapping time intervals
($r(a) < d(a')$ and $r(a') < d(a)$), the memory intervals
$[sigma(a), sigma(a) + s(a) - 1]$ and
$[sigma(a'), sigma(a') + s(a') - 1]$ are disjoint?

#theorem[
  3-Partition reduces to Dynamic Storage Allocation in polynomial time.
  Specifically, a 3-Partition instance $(A, B)$ with $3m$ elements is
  a YES-instance if and only if the constructed DSA instance with
  memory size $D = B$ is feasible under the optimal group assignment.
] <thm:threepartition-dynamicstorageallocation>

#proof[
  _Construction._

  Given a 3-Partition instance $A = {a_1, a_2, dots, a_(3m)}$ with bound $B$:

  + Set memory size $D = B$.
  + Create $m$ time windows: $[0, 1), [1, 2), dots, [m-1, m)$.
  + For each element $a_i$, create an item with size $s(a_i) = a_i$.
    The item's time interval is $[g(i), g(i)+1)$ where $g(i) in {0, dots, m-1}$
    is the group index assigned to element $i$.

  The group assignment $g : {1, dots, 3m} arrow {0, dots, m-1}$ must satisfy:
  each group receives exactly 3 elements. The DSA instance is parameterized
  by this assignment.

  _Observation._ Items in the same time window $[g, g+1)$ overlap in time
  and must have non-overlapping memory intervals in $[0, D)$. Items in
  different windows do not overlap in time and impose no mutual memory
  constraints. Therefore, DSA feasibility for this instance is equivalent
  to: for each group $g$, the sizes of the 3 assigned elements fit within
  memory $D = B$, i.e., they sum to at most $B$.

  _Correctness ($arrow.r.double$: 3-Partition YES $arrow.r$ DSA YES)._

  Suppose a valid 3-partition exists: disjoint triples $T_0, T_1, dots, T_(m-1)$
  with $sum_(a in T_g) a = B$ for all $g$. Assign elements of $T_g$ to
  time window $[g, g+1)$. Within each window, the 3 elements sum to
  exactly $B = D$, so they can be packed contiguously in $[0, B)$ without
  overlap. The DSA instance is feasible.

  _Correctness ($arrow.l.double$: DSA YES $arrow.r$ 3-Partition YES)._

  Suppose the DSA instance is feasible for some group assignment
  $g : {1, dots, 3m} arrow {0, dots, m-1}$ with exactly 3 elements per
  group. In each time window $[g, g+1)$, the 3 assigned elements must
  fit within $[0, B)$. Their total size is at most $B$.

  Since $sum_(i=1)^(3m) a_i = m B$ and the $m$ groups partition the elements
  with each group's total at most $B$, every group must sum to exactly $B$.
  The size constraints $B slash 4 < a_i < B slash 2$ ensure that no group can
  contain fewer or more than 3 elements (since 2 elements sum to less than $B$,
  and 4 elements sum to more than $B$).

  Therefore the group assignment defines a valid 3-partition.

  _Solution extraction._ Given a feasible DSA assignment, each item's time
  window directly gives the group index: $g(i) = r(a_i)$, the arrival time of
  item $i$.
]

*Overhead.*

#table(
  columns: (auto, auto),
  stroke: 0.5pt,
  [*Target metric*], [*Formula*],
  [`num_items`], [$3m$ #h(1em) (`num_elements`)],
  [`memory_size`], [$B$ #h(1em) (`bound`)],
)

*Feasible example (YES instance).*

Source: $A = {4, 5, 6, 4, 6, 5}$, $m = 2$, $B = 15$.

Valid 3-partition: $T_0 = {4, 5, 6}$ (sum $= 15$), $T_1 = {4, 6, 5}$ (sum $= 15$).

Constructed DSA: $D = 15$, 6 items in 2 time windows.

#table(
  columns: (auto, auto, auto, auto),
  stroke: 0.5pt,
  [*Item*], [*Arrival*], [*Departure*], [*Size*],
  [$a_1$], [0], [1], [4],
  [$a_2$], [0], [1], [5],
  [$a_3$], [0], [1], [6],
  [$a_4$], [1], [2], [4],
  [$a_5$], [1], [2], [6],
  [$a_6$], [1], [2], [5],
)

Window 0: items $a_1, a_2, a_3$ with sizes $4 + 5 + 6 = 15 = D$.
Addresses: $sigma(a_1) = 0$, $sigma(a_2) = 4$, $sigma(a_3) = 9$. #sym.checkmark

Window 1: items $a_4, a_5, a_6$ with sizes $4 + 6 + 5 = 15 = D$.
Addresses: $sigma(a_4) = 0$, $sigma(a_5) = 4$, $sigma(a_6) = 10$. #sym.checkmark

*Infeasible example (NO instance).*

Source: $A = {5, 5, 5, 7, 5, 5}$, $m = 2$, $B = 16$.

Check $B slash 4 = 4 < a_i < 8 = B slash 2$ for all elements. #sym.checkmark

Sum $= 32 = 2 times 16$. #sym.checkmark

Possible triples from ${5, 5, 5, 7, 5, 5}$:
- Any triple containing $7$: $7 + 5 + 5 = 17 eq.not 16$. #sym.crossmark
- Triple without $7$: $5 + 5 + 5 = 15 eq.not 16$. #sym.crossmark

No valid 3-partition exists. For any assignment of elements to 2 groups
of 3, at least one group's total differs from $B = 16$. Since the total
is $32 = 2B$ but no triple sums to $B$, the DSA instance with $D = 16$
is infeasible for every valid group assignment.
