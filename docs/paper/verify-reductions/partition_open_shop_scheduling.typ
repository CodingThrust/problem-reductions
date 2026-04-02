// Standalone Typst proof: Partition -> Open Shop Scheduling
// Issue #481 -- Gonzalez & Sahni (1976)

#set page(width: 210mm, height: auto, margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.1.")
#set math.equation(numbering: "(1)")

#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules
#show: thmrules.with(qed-symbol: $square$)
#let theorem = thmbox("theorem", "Theorem", stroke: 0.5pt)
#let proof = thmproof("proof", "Proof")

== Partition $arrow.r$ Open Shop Scheduling <sec:partition-openshopscheduling>

Let $A = {a_1, a_2, dots, a_k}$ be a multiset of positive integers with total
sum $S = sum_(j=1)^k a_j$.  Define the half-sum $Q = S slash 2$.  The
*Partition* problem asks whether there exists a subset $A' subset.eq A$ with
$sum_(a in A') a = Q$.  (If $S$ is odd, the answer is trivially NO.)

The *Open Shop Scheduling* problem with $m$ machines and $n$ jobs seeks a
non-preemptive schedule minimising the makespan (latest completion time).
Each job $j$ has one task per machine $i$ with processing time $p_(j,i)$.
Constraints: (1) each machine processes at most one task at a time; (2) each
job occupies at most one machine at a time.

#theorem[
  Partition reduces to Open Shop Scheduling with 3 machines in polynomial time.
  Specifically, a Partition instance $(A, S)$ is a YES-instance if and only if
  the constructed Open Shop Scheduling instance has optimal makespan at most $3Q$.
] <thm:partition-openshopscheduling>

#proof[
  _Construction._

  Given a Partition instance $A = {a_1, dots, a_k}$ with total sum $S$ and
  half-sum $Q = S slash 2$:

  + Set the number of machines to $m = 3$.
  + For each element $a_j$ ($j = 1, dots, k$), create *element job* $J_j$ with
    processing times $p_(j,1) = p_(j,2) = p_(j,3) = a_j$ (identical on all three
    machines).
  + Create one *special job* $J_(k+1)$ with processing times
    $p_(k+1,1) = p_(k+1,2) = p_(k+1,3) = Q$.
  + The constructed instance has $n = k + 1$ jobs and $m = 3$ machines.
    The deadline (target makespan) is $D = 3Q$.

  _Correctness ($arrow.r.double$: Partition YES $arrow.r$ makespan $<= 3Q$)._

  Suppose a balanced partition exists: $A' subset.eq A$ with
  $sum_(a in A') a = Q$ and $sum_(a in A backslash A') a = Q$.
  Denote the index sets $I_1 = {j : a_j in A'}$ and $I_2 = {j : a_j in.not A'}$.

  Schedule the special job $J_(k+1)$ on the three machines consecutively:
  - Machine 1: task of $J_(k+1)$ runs during $[0, Q)$.
  - Machine 2: task of $J_(k+1)$ runs during $[Q, 2Q)$.
  - Machine 3: task of $J_(k+1)$ runs during $[2Q, 3Q)$.

  The tasks of $J_(k+1)$ occupy disjoint time intervals, satisfying the job
  constraint.  Each machine has two idle blocks:
  - Machine 1 is idle during $[Q, 2Q)$ and $[2Q, 3Q)$.
  - Machine 2 is idle during $[0, Q)$ and $[2Q, 3Q)$.
  - Machine 3 is idle during $[0, Q)$ and $[Q, 2Q)$.

  Use a *rotated* assignment to ensure no two tasks of the same element job
  overlap in time.  Order the jobs in $I_1$ as $j_1, j_2, dots$ and define
  cumulative offsets $c_0 = 0$, $c_l = sum_(r=1)^l a_(j_r)$.  Assign:
  - Machine 1: $[Q + c_(l-1), thin Q + c_l)$
  - Machine 2: $[2Q + c_(l-1), thin 2Q + c_l)$
  - Machine 3: $[c_(l-1), thin c_l)$

  Since $c_(|I_1|) = Q$, these intervals fit within the idle blocks.  Each
  $I_1$-job has its three tasks in three distinct time blocks ($[0,Q)$,
  $[Q,2Q)$, $[2Q,3Q)$), so no job-overlap violations occur.

  Similarly, order the jobs in $I_2$ as $j'_1, j'_2, dots$ with cumulative
  offsets $c'_0 = 0$, $c'_l = sum_(r=1)^l a_(j'_r)$.  Assign:
  - Machine 1: $[2Q + c'_(l-1), thin 2Q + c'_l)$
  - Machine 2: $[c'_(l-1), thin c'_l)$
  - Machine 3: $[Q + c'_(l-1), thin Q + c'_l)$

  Each $I_2$-job also occupies three distinct time blocks.  The machine
  constraint is satisfied because within each time block on each machine,
  either $I_1$-jobs, $I_2$-jobs, or the special job are packed (never
  overlapping).  All tasks complete by time $3Q = D$.

  _Correctness ($arrow.l.double$: makespan $<= 3Q$ $arrow.r$ Partition YES)._

  Suppose a schedule with makespan at most $3Q$ exists.  The special job
  $J_(k+1)$ requires $Q$ time units on each of the 3 machines, and its tasks
  must be non-overlapping (job constraint).  Therefore $J_(k+1)$ alone needs
  at least $3Q$ elapsed time.  Since the makespan is at most $3Q$, the three
  tasks of $J_(k+1)$ must occupy three disjoint intervals of length $Q$ that
  together tile $[0, 3Q)$ exactly.

  On each machine, the remaining idle time is $3Q - Q = 2Q$, split into
  exactly two contiguous blocks of length $Q$.  The total processing time of
  element jobs on any single machine is $sum_(j=1)^k a_j = S = 2Q$.  These
  element jobs must fill the two idle blocks of length $Q$ exactly (zero slack).

  Consider machine 1.  Let $B_1$ and $B_2$ be the two idle blocks (each of
  length $Q$).  The element jobs scheduled in $B_1$ have total processing time
  $Q$, and those in $B_2$ also total $Q$.  The set of elements corresponding to
  jobs in $B_1$ forms a subset summing to $Q$, which is a valid partition.

  _Solution extraction._

  Given a feasible schedule (makespan $<= 3Q$), identify the special job's
  task on machine 1.  The element jobs in one of the two idle blocks on machine
  1 form a subset summing to $Q$.  Map those indices back to the Partition
  instance: set $x_j = 0$ for elements in that subset and $x_j = 1$ for the
  rest.
]

*Overhead.*

#table(
  columns: (auto, auto),
  stroke: 0.5pt,
  [*Target metric*], [*Formula*],
  [`num_jobs`], [$k + 1$ #h(1em) (`num_elements + 1`)],
  [`num_machines`], [$3$],
  [`max processing time`], [$Q = S slash 2$ #h(1em) (`total_sum / 2`)],
)

*Feasible example (YES instance).*

Source: $A = {3, 1, 1, 2, 2, 1}$, $k = 6$, $S = 10$, $Q = 5$.
Balanced partition: ${3, 2} = {a_1, a_4}$ (sum $= 5$) and ${1, 1, 2, 1} = {a_2, a_3, a_5, a_6}$ (sum $= 5$).

Constructed instance: $m = 3$ machines, $n = 7$ jobs, deadline $D = 15$.

#table(
  columns: (auto, auto, auto, auto),
  stroke: 0.5pt,
  [*Job*], [*$p_(j,1)$*], [*$p_(j,2)$*], [*$p_(j,3)$*],
  [$J_1$], [3], [3], [3],
  [$J_2$], [1], [1], [1],
  [$J_3$], [1], [1], [1],
  [$J_4$], [2], [2], [2],
  [$J_5$], [2], [2], [2],
  [$J_6$], [1], [1], [1],
  [$J_7$ (special)], [5], [5], [5],
)

Schedule with makespan $= 15$:

Special job $J_7$: machine 1 in $[0, 5)$, machine 2 in $[5, 10)$, machine 3
in $[10, 15)$.

$I_1 = {1, 4}$ (elements $3, 2$, sum $= 5$):
- $J_1$: machine 1 in $[5, 8)$, machine 2 in $[10, 13)$, machine 3 in $[0, 3)$.
- $J_4$: machine 1 in $[8, 10)$, machine 2 in $[13, 15)$, machine 3 in $[3, 5)$.

$I_2 = {2, 3, 5, 6}$ (elements $1, 1, 2, 1$, sum $= 5$):
- $J_2$: machine 1 in $[10, 11)$, machine 2 in $[0, 1)$, machine 3 in $[5, 6)$.
- $J_3$: machine 1 in $[11, 12)$, machine 2 in $[1, 2)$, machine 3 in $[6, 7)$.
- $J_5$: machine 1 in $[12, 14)$, machine 2 in $[2, 4)$, machine 3 in $[7, 9)$.
- $J_6$: machine 1 in $[14, 15)$, machine 2 in $[4, 5)$, machine 3 in $[9, 10)$.

Verification: each machine has total load $2Q + Q = 3Q = 15$.  Each element
job's three tasks are in three distinct time blocks, so no job-overlap
violations.  Makespan $= 15 = 3Q = D$.

*Infeasible example (NO instance).*

Source: $A = {1, 1, 1, 5}$, $k = 4$, $S = 8$, $Q = 4$.
The achievable subset sums are $0, 1, 2, 3, 5, 6, 7, 8$.  No subset sums to
$4$: ${5} = 5 eq.not 4$; ${1,1,1} = 3 eq.not 4$; ${1,5} = 6 eq.not 4$;
${1,1,5} = 7 eq.not 4$.  All other subsets are complements of these.

Constructed instance: $m = 3$, $n = 5$ jobs, deadline $D = 12$.

#table(
  columns: (auto, auto, auto, auto),
  stroke: 0.5pt,
  [*Job*], [*$p_(j,1)$*], [*$p_(j,2)$*], [*$p_(j,3)$*],
  [$J_1$], [1], [1], [1],
  [$J_2$], [1], [1], [1],
  [$J_3$], [1], [1], [1],
  [$J_4$], [5], [5], [5],
  [$J_5$ (special)], [4], [4], [4],
)

The special job $J_5$ requires $3 times 4 = 12$ total time, which equals the
deadline $D = 12$.  Total work across all jobs and machines is
$3 times (8 + 4) = 36$, and total capacity is $3 times 12 = 36$, so the
schedule must have zero idle time.

The special job partitions $[0, 12)$ into one block of 4 per machine and two
idle blocks of 4 each.  The element jobs must fill each idle block exactly.
On any machine, each idle block has length 4, and the element jobs filling it
must sum to 4.  But no subset of ${1, 1, 1, 5}$ sums to 4.  Therefore no
feasible schedule with makespan $<= 12$ exists, and the optimal makespan is
strictly greater than 12.
