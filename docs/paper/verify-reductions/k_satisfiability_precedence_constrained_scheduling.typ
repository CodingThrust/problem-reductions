// Reduction proof: KSatisfiability(K3) -> PrecedenceConstrainedScheduling
// Reference: Ullman (1975), "NP-Complete Scheduling Problems",
//            J. Computer and System Sciences 10, pp. 384-393.
//            Garey & Johnson, Computers and Intractability, A5.2, p.239.

#set page(width: auto, height: auto, margin: 15pt)
#set text(size: 10pt)

= 3-SAT $arrow.r$ Precedence Constrained Scheduling

== Problem Definitions

*3-SAT (KSatisfiability with $K=3$):*
Given a set $U = {x_1, dots, x_m}$ of Boolean variables and a collection $D_1, dots, D_n$ of clauses over $U$, where each clause $D_i = (l_1^i or l_2^i or l_3^i)$ contains exactly 3 literals, is there a truth assignment $f: U arrow {"true", "false"}$ satisfying all clauses?

*Precedence Constrained Scheduling (P2/PCS):*
Given a set $S$ of $N$ unit-length tasks, a partial order $prec$ on $S$, a number $k$ of processors, and a deadline $t$, is there a function $sigma: S arrow {0, 1, dots, t-1}$ such that:
- at most $k$ tasks are assigned to any time slot, and
- if $J prec J'$ then $sigma(J) < sigma(J')$?

*Variable-Capacity Scheduling (P4):*
Same as P2 but with slot-specific capacities: given $c_0, c_1, dots, c_(t-1)$ with $sum c_i = N$, require $|sigma^(-1)(i)| = c_i$ for each slot $i$.

== Reduction Overview

The reduction proceeds in two steps (Ullman, 1975):
1. *Lemma 2:* 3-SAT $arrow.r$ P4 (the combinatorial core)
2. *Lemma 1:* P4 $arrow.r$ P2 (mechanical padding)

== Step 1: 3-SAT $arrow.r$ P4 (Lemma 2)

Given a 3-SAT instance with $m$ variables and $n$ clauses, construct a P4 instance as follows.

*Tasks:*
- For each variable $x_i$ ($i = 1, dots, m$): a positive chain $x_(i,0), x_(i,1), dots, x_(i,m)$ and a negative chain $overline(x)_(i,0), overline(x)_(i,1), dots, overline(x)_(i,m)$ — total $2m(m+1)$ tasks.
- Indicator tasks $y_i$ and $overline(y)_i$ for $i = 1, dots, m$ — total $2m$ tasks.
- For each clause $D_i$ ($i = 1, dots, n$): seven truth-pattern tasks $D_(i,1), dots, D_(i,7)$ (one for each nonzero 3-bit pattern) — total $7n$ tasks.

*Grand total:* $2m(m+1) + 2m + 7n$ tasks.

*Time limit:* $t = m + 3$ (slots $0, 1, dots, m+2$).

*Slot capacities:*
$
c_0 &= m, \
c_1 &= 2m + 1, \
c_j &= 2m + 2 quad "for" j = 2, dots, m, \
c_(m+1) &= n + m + 1, \
c_(m+2) &= 6n.
$

*Precedences:*
+ *Variable chains:* $x_(i,j) prec x_(i,j+1)$ and $overline(x)_(i,j) prec overline(x)_(i,j+1)$ for all $i, j$.
+ *Indicator connections:* $x_(i,i-1) prec y_i$ and $overline(x)_(i,i-1) prec overline(y)_i$.
+ *Clause gadgets:* For clause $D_i$ with literals $z_(k_1), z_(k_2), z_(k_3)$ and truth-pattern task $D_(i,j)$ where $j = a_1 dot 4 + a_2 dot 2 + a_3$ in binary:
  - If $a_p = 1$: $z_(k_p, m) prec D_(i,j)$ (the literal's chain-end task)
  - If $a_p = 0$: $overline(z)_(k_p, m) prec D_(i,j)$ (the complement's chain-end task)

== Correctness Proof (Sketch)

=== Variable Assignment Encoding

The tight slot capacities force a specific structure:

- *Slot 0* holds exactly $m$ tasks. The only tasks with no predecessors and whose chains are long enough to fill subsequent slots are $x_(i,0)$ and $overline(x)_(i,0)$. Exactly one of each pair occupies slot 0.

- *Interpretation:* $x_i = "true"$ iff $x_(i,0)$ is in slot 0.

=== Key Invariant

Ullman proves that in any valid P4 schedule:
- Exactly one of $x_(i,0)$ and $overline(x)_(i,0)$ is at time 0 (with the other at time 1).
- The remaining chain tasks and indicators are determined by this choice.
- At time $m+1$, exactly $n$ of the $D$ tasks can be scheduled — specifically, for each clause $D_i$, at most one $D_(i,j)$ fits.

=== Forward Direction ($arrow.r$)

Given a satisfying assignment $f$:
- Place $x_(i,0)$ at time 0 if $f(x_i) = "true"$, otherwise $overline(x)_(i,0)$ at time 0.
- Chain tasks and indicators fill deterministically.
- For each clause $D_i$, at least one $D_(i,j)$ (corresponding to the truth pattern matching $f$) has all predecessors completed by time $m$, so it can be placed at time $m+1$.

=== Backward Direction ($arrow.l$)

Given a feasible P4 schedule:
- The capacity constraint forces exactly one of each variable pair into slot 0.
- Define $f(x_i) = "true"$ iff $x_(i,0)$ is at time 0.
- Since $n$ of the $D$ tasks must be at time $m+1$ and at most one per clause fits, each clause has a matching truth pattern — hence $f$ satisfies all clauses. $square$

== Step 2: P4 $arrow.r$ P2 (Lemma 1)

Given a P4 instance with $N$ tasks, time limit $t$, and capacities $c_0, dots, c_(t-1)$:

- Introduce padding jobs $I_(i,j)$ for $0 <= i < t$ and $0 <= j < N - c_i$.
- Chain all padding: $I_(i,j) prec I_(i+1,k)$ for all valid $i, j, k$.
- Set $k = N + 1$ processors and deadline $t$.

In any P2 solution, exactly $N + 1 - c_i$ padding jobs occupy slot $i$, leaving exactly $c_i$ slots for original jobs. Thus P2 and P4 have the same feasible solutions for the original jobs.

== Size Overhead

| Metric | Expression |
|--------|-----------|
| P4 tasks | $2m(m+1) + 2m + 7n$ |
| P4 time slots | $m + 3$ |
| P2 tasks (after Lemma 1) | $(m + 3)(2m^2 + 4m + 7n + 1)$ |
| P2 processors | $2m^2 + 4m + 7n + 1$ |
| P2 deadline | $m + 3$ |

== Example

*Source (3-SAT):* $m = 3$ variables, clause: $(x_1 or x_2 or x_3)$

*P4 instance:* 37 tasks, 6 time slots, capacities $(3, 7, 8, 8, 5, 6)$.

*Satisfying assignment:* $x_1 = "true", x_2 = "true", x_3 = "true"$

*Schedule (slot assignments):*
- Slot 0: $x_(1,0), x_(2,0), x_(3,0)$ (all positive chain starts)
- Slot 1: $x_(1,1), x_(2,1), x_(3,1), overline(x)_(1,0), overline(x)_(2,0), overline(x)_(3,0), y_1$
- Slot 2: $x_(1,2), x_(2,2), x_(3,2), overline(x)_(1,1), overline(x)_(2,1), overline(x)_(3,1), y_2, overline(y)_1$
- Slot 3: $x_(1,3), x_(2,3), x_(3,3), overline(x)_(1,2), overline(x)_(2,2), overline(x)_(3,2), y_3, overline(y)_2$
- Slot 4: $overline(x)_(1,3), overline(x)_(2,3), overline(x)_(3,3), overline(y)_3, D_(1,7)$
- Slot 5: $D_(1,1), D_(1,2), D_(1,3), D_(1,4), D_(1,5), D_(1,6)$

*Solution extraction:* $x_(i,0)$ at slot 0 $arrow.r.double x_i = "true"$ for all $i$. Check: $("true" or "true" or "true") = "true"$. $checkmark$

== References

- *[Ullman, 1975]* Jeffrey D. Ullman. "NP-complete scheduling problems". _Journal of Computer and System Sciences_ 10, pp. 384--393.
- *[Garey & Johnson, 1979]* M. R. Garey and D. S. Johnson. _Computers and Intractability: A Guide to the Theory of NP-Completeness_, W. H. Freeman, pp. 236--239.
