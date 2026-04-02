// Standalone Typst proof: Partition -> Production Planning
// Issue #488 -- Lenstra, Rinnooy Kan & Florian (1978)

#set page(width: 210mm, height: auto, margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.1.")
#set math.equation(numbering: "(1)")

#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules
#show: thmrules.with(qed-symbol: $square$)
#let theorem = thmbox("theorem", "Theorem", stroke: 0.5pt)
#let proof = thmproof("proof", "Proof")

== Partition $arrow.r$ Production Planning <sec:partition-productionplanning>

Let $A = {a_1, a_2, dots, a_n}$ be a multiset of positive integers with total
sum $S = sum_(i=1)^n a_i$.  Define the half-sum $Q = S slash 2$.  The
*Partition* problem asks whether there exists a subset $A' subset.eq A$ with
$sum_(a in A') a = Q$.  (If $S$ is odd, the answer is trivially NO.)

The *Production Planning* problem asks: given $n$ periods, each with demand
$r_i$, production capacity $c_i$, set-up cost $b_i$ (incurred whenever
$x_i > 0$), per-unit production cost $p_i$, and per-unit inventory cost $h_i$,
and an overall cost bound $B$, do there exist production amounts
$x_i in {0, 1, dots, c_i}$ such that the inventory levels
$I_i = sum_(j=1)^i (x_j - r_j) >= 0$ for all $i$, and the total cost
$ sum_(i=1)^n (p_i dot x_i + h_i dot I_i) + sum_(x_i > 0) b_i <= B ? $

#theorem[
  Partition reduces to Production Planning in polynomial time.
  Specifically, a Partition instance $(A, S)$ is a YES-instance if and only if
  the constructed Production Planning instance is feasible.
] <thm:partition-productionplanning>

#proof[
  _Construction._

  Given a Partition instance $A = {a_1, dots, a_n}$ with total sum $S$ and
  half-sum $Q = S slash 2$.  If $S$ is odd, output a trivially infeasible
  Production Planning instance (e.g., one period with demand 1, capacity 0,
  and $B = 0$).  Otherwise, construct $n + 1$ periods:

  + For each element $a_i$ ($i = 1, dots, n$), create *element period* $i$ with:
    - Demand $r_i = 0$ (no demand in element periods).
    - Capacity $c_i = a_i$.
    - Set-up cost $b_i = a_i$.
    - Production cost $p_i = 0$.
    - Inventory cost $h_i = 0$.

  + Create one *demand period* $n + 1$ with:
    - Demand $r_(n+1) = Q$.
    - Capacity $c_(n+1) = 0$ (no production allowed).
    - Set-up cost $b_(n+1) = 0$.
    - Production cost $p_(n+1) = 0$.
    - Inventory cost $h_(n+1) = 0$.

  + Set the cost bound $B = Q$.

  The constructed instance has $n + 1$ periods.

  _Correctness ($arrow.r.double$: Partition YES $arrow.r$ Production Planning feasible)._

  Suppose a balanced partition exists: $A' subset.eq A$ with
  $sum_(a in A') a = Q$.  Let $I_1 = {i : a_i in A'}$.

  Set $x_i = a_i$ for $i in I_1$ and $x_i = 0$ for $i in.not I_1$ (among the
  element periods), and $x_(n+1) = 0$.

  *Inventory check:* For each element period $i$ ($1 <= i <= n$),
  $I_i = sum_(j=1)^i x_j >= 0$ since all $x_j >= 0$ and all $r_j = 0$.
  At the demand period: $I_(n+1) = sum_(j=1)^n x_j - Q = Q - Q = 0 >= 0$.

  *Cost check:* All production costs $p_i = 0$ and inventory costs $h_i = 0$,
  so only set-up costs matter.  The set-up cost is incurred for each period
  where $x_i > 0$, i.e., for $i in I_1$:
  $ "Total cost" = sum_(i in I_1) b_i = sum_(i in I_1) a_i = Q = B. $

  The plan is feasible.

  _Correctness ($arrow.l.double$: Production Planning feasible $arrow.r$ Partition YES)._

  Suppose a feasible production plan exists with cost at most $B = Q$.

  Let $J = {i in {1, dots, n} : x_i > 0}$ be the active element periods.

  *Setup cost bound:* The total cost includes $sum_(i in J) b_i = sum_(i in J) a_i$.
  Since all other cost terms ($p_i dot x_i$ and $h_i dot I_i$) are zero
  (because $p_i = h_i = 0$ for all periods), we have:
  $ sum_(i in J) a_i <= Q. $ <eq:setup-bound>

  *Demand satisfaction:* At the demand period $n + 1$, the inventory
  $I_(n+1) = sum_(j=1)^n x_j - Q >= 0$, so:
  $ sum_(j=1)^n x_j >= Q. $ <eq:demand>

  *Capacity constraint:* For each active period $i in J$, $0 < x_i <= c_i = a_i$.
  Therefore:
  $ sum_(j=1)^n x_j = sum_(i in J) x_i <= sum_(i in J) a_i <= Q, $ <eq:capacity>

  where the last inequality is @eq:setup-bound.

  Combining @eq:demand and @eq:capacity:
  $ Q <= sum_(j=1)^n x_j <= sum_(i in J) a_i <= Q. $

  All inequalities are equalities.  In particular, $sum_(i in J) a_i = Q$, so
  $J$ indexes a subset of $A$ that sums to $Q$.  This is a valid partition.

  _Solution extraction._

  Given a feasible production plan, the set of active element periods
  ${i : x_i > 0}$ corresponds to a partition subset summing to $Q$.
  Set the Partition solution to $x_i^"src" = 1$ if $x_i > 0$ (element in
  second subset), and $x_i^"src" = 0$ otherwise.
]

*Overhead.*

#table(
  columns: (auto, auto),
  stroke: 0.5pt,
  [*Target metric*], [*Formula*],
  [`num_periods`], [$n + 1$ #h(1em) (`num_elements + 1`)],
  [`max_capacity`], [$max(a_i)$ #h(1em) (`max(sizes)`)],
  [`cost_bound`], [$Q = S slash 2$ #h(1em) (`total_sum / 2`)],
)

*Feasible example (YES instance).*

Source: $A = {3, 1, 1, 2, 2, 1}$, $n = 6$, $S = 10$, $Q = 5$.
Balanced partition: ${a_1, a_4} = {3, 2}$ (sum $= 5$) and ${a_2, a_3, a_5, a_6} = {1, 1, 2, 1}$ (sum $= 5$).

Constructed instance: $n + 1 = 7$ periods, cost bound $B = 5$.

#table(
  columns: (auto, auto, auto, auto, auto, auto),
  stroke: 0.5pt,
  [*Period*], [*$r_i$*], [*$c_i$*], [*$b_i$*], [*$p_i$*], [*$h_i$*],
  [1 (elem $a_1=3$)], [0], [3], [3], [0], [0],
  [2 (elem $a_2=1$)], [0], [1], [1], [0], [0],
  [3 (elem $a_3=1$)], [0], [1], [1], [0], [0],
  [4 (elem $a_4=2$)], [0], [2], [2], [0], [0],
  [5 (elem $a_5=2$)], [0], [2], [2], [0], [0],
  [6 (elem $a_6=1$)], [0], [1], [1], [0], [0],
  [7 (demand)], [$Q = 5$], [0], [0], [0], [0],
)

Solution: activate elements in $I_1 = {1, 4}$: produce $x_1 = 3$, $x_4 = 2$,
all others $= 0$.

Inventory levels: $I_1 = 3$, $I_2 = 3$, $I_3 = 3$, $I_4 = 5$, $I_5 = 5$,
$I_6 = 5$, $I_7 = 5 - 5 = 0$.  All $>= 0$ #sym.checkmark

Total cost $= b_1 + b_4 = 3 + 2 = 5 = B$ #sym.checkmark

*Infeasible example (NO instance).*

Source: $A = {1, 1, 1, 5}$, $n = 4$, $S = 8$, $Q = 4$.
The achievable subset sums are ${0, 1, 2, 3, 5, 6, 7, 8}$.  No subset sums to
$4$, so no balanced partition exists.

Constructed instance: $n + 1 = 5$ periods, cost bound $B = 4$.

#table(
  columns: (auto, auto, auto, auto, auto, auto),
  stroke: 0.5pt,
  [*Period*], [*$r_i$*], [*$c_i$*], [*$b_i$*], [*$p_i$*], [*$h_i$*],
  [1 (elem $a_1=1$)], [0], [1], [1], [0], [0],
  [2 (elem $a_2=1$)], [0], [1], [1], [0], [0],
  [3 (elem $a_3=1$)], [0], [1], [1], [0], [0],
  [4 (elem $a_4=5$)], [0], [5], [5], [0], [0],
  [5 (demand)], [$Q = 4$], [0], [0], [0], [0],
)

Any feasible plan needs $sum_(i in J) a_i <= 4$ (setup cost bound) and
$sum_(i in J) x_i >= 4$ (demand satisfaction), with $x_i <= a_i = c_i$.
These force $sum_(i in J) a_i >= 4$, hence $sum_(i in J) a_i = 4$.
But no subset of ${1, 1, 1, 5}$ sums to $4$, so no feasible plan exists.
