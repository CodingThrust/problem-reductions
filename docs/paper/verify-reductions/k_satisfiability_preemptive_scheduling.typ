// Reduction proof: KSatisfiability(K3) -> PreemptiveScheduling
// Reference: Ullman (1975), "NP-complete scheduling problems"
// Garey & Johnson, Computers and Intractability, Appendix A5.2, p.240

#set page(width: 210mm, height: auto, margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.1.")
#set math.equation(numbering: "(1)")

#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules
#show: thmrules.with(qed-symbol: $square$)
#let theorem = thmbox("theorem", "Theorem", stroke: 0.5pt)
#let lemma = thmbox("lemma", "Lemma", stroke: 0.5pt)
#let proof = thmproof("proof", "Proof")

= 3-SAT $arrow.r$ Preemptive Scheduling <sec:ksat-preemptivescheduling>

== Problem Definitions

*3-SAT (KSatisfiability with $K=3$):*
Given a set of Boolean variables $x_1, dots, x_M$ and a collection of clauses $D_1, dots, D_N$, where each clause $D_j = (ell_1^j or ell_2^j or ell_3^j)$ contains exactly 3 literals, is there a truth assignment satisfying all clauses?

*Preemptive Scheduling:*
Given a set of tasks with integer processing lengths, $m$ identical processors, and precedence constraints, minimize the makespan (latest completion time). Tasks may be interrupted and resumed on any processor. The decision version asks: is there a preemptive schedule with makespan at most $D$?

== Reduction Construction (Ullman 1975)

The reduction proceeds in two stages. Stage 1 reduces 3-SAT to a _variable-capacity_ scheduling problem (Ullman's P4). Stage 2 transforms P4 into standard fixed-processor scheduling (P2). Since every non-preemptive unit-task schedule is trivially a valid preemptive schedule, the result is an instance of preemptive scheduling.

We follow Ullman's notation: $M$ = number of variables, $N$ = number of clauses ($N lt.eq 3 M$, which always holds for 3-SAT since each clause uses at most 3 of $M$ variables).

=== Stage 1: 3-SAT $arrow.r$ P4 (variable-capacity scheduling)

Given a 3-SAT instance with $M$ variables $x_1, dots, x_M$ and $N$ clauses $D_1, dots, D_N$, construct:

*Jobs (all unit-length):*
+ *Variable chains:* $x_(i,j)$ and $overline(x)_(i,j)$ for $1 lt.eq i lt.eq M$ and $0 lt.eq j lt.eq M$. These are $2 M (M+1)$ jobs.
+ *Forcing jobs:* $y_i$ and $overline(y)_i$ for $1 lt.eq i lt.eq M$. These are $2 M$ jobs.
+ *Clause jobs:* $D_(i,j)$ for $1 lt.eq i lt.eq N$ and $1 lt.eq j lt.eq 7$. These are $7 N$ jobs.

*Precedence constraints:*
+ $x_(i,j) prec x_(i,j+1)$ and $overline(x)_(i,j) prec overline(x)_(i,j+1)$ for $1 lt.eq i lt.eq M$, $0 lt.eq j < M$ (variable chains form length-$(M+1)$ paths).
+ $x_(i,i-1) prec y_i$ and $overline(x)_(i,i-1) prec overline(y)_i$ for $1 lt.eq i lt.eq M$ (forcing jobs branch off the chains at staggered positions).
+ *Clause precedences:* For each clause $D_i$, the 7 clause jobs $D_(i,1), dots, D_(i,7)$ encode the clause's literal structure. Let $D_i = {ell_1, ell_2, ell_3}$ where each $ell_k$ is either $x_(alpha_k)$ or $overline(x)_(alpha_k)$. Then let $z_(k_1), z_(k_2), z_(k_3)$ be the corresponding chain jobs at position $M$ (i.e., $x_(alpha_k, M)$ if $ell_k = x_(alpha_k)$, or $overline(x)_(alpha_k, M)$ if $ell_k = overline(x)_(alpha_k)$). We require $z_(k_p, M) prec D_(i,j)$ for certain combinations encoding the binary representations of the clause's satisfying assignments.

*Time limit:* $T = M + 3$.

*Capacity sequence* $c_0, c_1, dots, c_(M+2)$:
$ c_0 &= M, \
  c_1 &= 2M + 1, \
  c_i &= 2M + 2 quad "for" 2 lt.eq i lt.eq M, \
  c_(M+1) &= N + M + 1, \
  c_(M+2) &= 6N. $

The total number of jobs equals $sum_(i=0)^(M+2) c_i = 2M(M+1) + 2M + 7N$.

=== Stage 2: P4 $arrow.r$ P2 (fixed-capacity scheduling)

Given the P4 instance with time limit $T = M+3$, jobs $S$, and capacity sequence $(c_0, dots, c_(T-1))$, let $n = max_i c_i$ be the maximum capacity. Construct a P2 instance:

+ Set $n+1$ processors.
+ For each time step $i$ where $c_i < n$, introduce $n - c_i$ *filler jobs* $I_(i,1), dots, I_(i,n-c_i)$.
+ Add precedence: all filler jobs at time $i$ must precede all filler jobs at time $i+1$: $I_(i,j) prec I_(i+1,k)$.
+ The time limit remains $T = M+3$.

Since the filler jobs force exactly $n - c_i$ of them to execute at time $i$, the remaining $c_i$ processor slots are available for the original jobs. The P2 instance has a schedule meeting deadline $T$ if and only if the P4 instance does.

=== Embedding into Preemptive Scheduling

Since all tasks have unit length, preemption is irrelevant (a unit-length task cannot be split). The P2 instance is directly a valid preemptive scheduling instance with:
- All task lengths = 1
- Number of processors = $n + 1$ (where $n = max(c_0, dots, c_(M+2))$)
- Deadline (target makespan) = $T = M + 3$

#theorem[
  A 3-SAT instance with $M$ variables and $N$ clauses is satisfiable if and only if the constructed preemptive scheduling instance has optimal makespan at most $M + 3$.
] <thm:ksat-preemptivescheduling>

== Correctness Sketch

=== Forward direction ($arrow.r$)

If the 3-SAT formula is satisfiable, assign truth values to variables. For each variable $x_i$:
- If $x_i = "true"$: execute $x_(i,0)$ at time 0 (and $overline(x)_(i,0)$ at time 1).
- If $x_i = "false"$: execute $overline(x)_(i,0)$ at time 0 (and $x_(i,0)$ at time 1).

The forcing jobs $y_i, overline(y)_i$ are then determined. At time $M + 1$, the remaining chain endpoints and forcing jobs complete. At time $M + 2$, clause jobs execute -- since the assignment satisfies every clause, for each $D_i$ at least one literal-chain endpoint was scheduled "favorably" at time 0, making the corresponding clause jobs executable by time $M + 2$. The filler jobs fill remaining processor slots at each time step.

=== Backward direction ($arrow.l$)

Given a feasible schedule with makespan $lt.eq M + 3$:
1. The capacity constraints force that at time 0, exactly one of $x_(i,0)$ or $overline(x)_(i,0)$ is executed for each variable $i$.
2. The chain structure and forcing jobs propagate this choice through times $1, dots, M$.
3. At time $M + 1$, the $N + M + 1$ capacity constraint forces exactly $N$ clause jobs to be ready, which requires each clause to have at least one satisfied literal.
4. Extract: $x_i = "true"$ if $x_(i,0)$ was executed at time 0, $x_i = "false"$ otherwise.

== Size Overhead

#table(
  columns: (auto, auto),
  stroke: 0.5pt,
  [*Target metric*], [*Formula*],
  [`num_tasks`], [$2M(M+1) + 2M + 7N + sum_(i=0)^(M+2) (n_max - c_i)$],
  [`num_processors`], [$n_max + 1$ where $n_max = max(M, 2M+2, N+M+1, 6N)$],
  [`num_precedences`], [$O(M^2 + N + F^2)$ where $F$ = total filler jobs],
  [`deadline`], [$M + 3$],
)

For small instances ($M$ variables, $N$ clauses), $n_max = max(2M+2, 6N)$ and the total number of tasks and precedences are polynomial in $M + N$.

== Example

*Source (3-SAT):* $M = 2$ variables, $N = 1$ clause: $(x_1 or x_2 or overline(x)_1)$.

Note: this clause is trivially satisfiable (any assignment with $x_1 = "true"$ or $x_2 = "true"$ works; in fact even $x_1 = "false", x_2 = "true"$ satisfies via $overline(x)_1$).

*Stage 1 (P4):*
- Variable chain jobs: $x_(1,0), x_(1,1), x_(1,2), overline(x)_(1,0), overline(x)_(1,1), overline(x)_(1,2), x_(2,0), x_(2,1), x_(2,2), overline(x)_(2,0), overline(x)_(2,1), overline(x)_(2,2)$ (12 jobs)
- Forcing jobs: $y_1, overline(y)_1, y_2, overline(y)_2$ (4 jobs)
- Clause jobs: $D_(1,1), dots, D_(1,7)$ (7 jobs)
- Total: 23 jobs
- Time limit: $T = 5$
- Capacities: $c_0 = 2, c_1 = 5, c_2 = 6, c_3 = 4, c_4 = 6$

*Stage 2 (P2):*
- $n_max = 6$, processors = 7
- Filler jobs fill gaps: 4 at time 0, 1 at time 1, 0 at time 2, 2 at time 3, 0 at time 4 = 7 filler jobs
- Total jobs: 30, deadline: 5

*Satisfying assignment:* $x_1 = "true", x_2 = "false"$ $arrow.r$ schedule exists with makespan $lt.eq 5$.

== References

- *Ullman (1975):* J. D. Ullman, "NP-complete scheduling problems," _Journal of Computer and System Sciences_ 10(3), pp. 384--393.
- *Garey & Johnson (1979):* M. R. Garey and D. S. Johnson, _Computers and Intractability: A Guide to the Theory of NP-Completeness_, Appendix A5.2, p. 240.
