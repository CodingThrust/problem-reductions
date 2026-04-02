// Reduction proof: KSatisfiability(K3) -> CyclicOrdering
// Reference: Galil & Megiddo (1977), "Cyclic ordering is NP-complete"
// Theoretical Computer Science 5(2), pp. 179-182.

#set page(width: auto, height: auto, margin: 15pt)
#set text(size: 10pt)

= 3-SAT $arrow.r$ Cyclic Ordering

== Problem Definitions

*3-SAT (KSatisfiability with $K=3$):*
Given a set $U = {u_1, dots, u_r, overline(u)_1, dots, overline(u)_r}$ of Boolean literals and a collection of $p$ clauses $C_nu = x_nu or y_nu or z_nu$ ($nu = 1, dots, p$) where each literal ${x_nu, y_nu, z_nu} subset U$, is there a truth assignment $S subset.eq U$ (containing exactly one of $u_tau, overline(u)_tau$ for each $tau$) that satisfies all clauses?

*Cyclic Ordering:*
Given a finite set $T$ and a collection $Delta$ of cyclically ordered triples (COTs) of elements from $T$, does there exist a cyclic ordering of $T$ from which every COT in $Delta$ is derived? A COT $a b c$ means $a, b, c$ appear in that cyclic order.

== Reduction Construction (Galil & Megiddo 1977)

Given a 3-SAT instance with $r$ variables and $p$ clauses, construct a Cyclic Ordering instance as follows.

*Variable elements:* For each variable $u_tau$ ($tau = 1, dots, r$), create three elements $alpha_tau, beta_tau, gamma_tau$. The set $A = {alpha_1, beta_1, gamma_1, dots, alpha_r, beta_r, gamma_r}$ has $3r$ elements.

*Variable COTs:* With $u_tau$ we associate the COT $alpha_tau beta_tau gamma_tau$, and with $overline(u)_tau$ we associate the reverse COT $alpha_tau gamma_tau beta_tau$. These two orientations encode the truth value of $u_tau$: the COT of the _true_ literal is NOT derived from the cyclic ordering (it is in $S$), while the COT of the _false_ literal IS derived.

*Clause gadget:* For each clause $C_nu = x_nu or y_nu or z_nu$, let $a b c$, $d e f$, $g h i$ be the COTs associated with literals $x_nu$, $y_nu$, $z_nu$ respectively (each is a triple of elements from $A$). Introduce 5 fresh auxiliary elements $j_nu, k_nu, l_nu, m_nu, n_nu$ and add 10 COTs:

$
Delta^0_nu = {a c j, #h(0.3em) b j k, #h(0.3em) c k l, #h(0.3em) d f j, #h(0.3em) e j l, #h(0.3em) f l m, #h(0.3em) g i k, #h(0.3em) h k m, #h(0.3em) i m n, #h(0.3em) n m l}
$

*Total size:*
- $|T| = 3r + 5p$ elements
- $|Delta| = 10p$ COTs

== Correctness Proof

*Claim (Theorem 3 of Galil & Megiddo):* The 3-SAT instance is satisfiable if and only if $Delta_1^0 union dots union Delta_p^0$ is consistent.

=== Forward direction ($arrow.r$)

Suppose $S subset U$ is a satisfying assignment. For each clause $C_nu$, at least one literal is in $S$, so $S sect {x_nu, y_nu, z_nu} eq.not emptyset$.

By Lemma 1, when $S sect {x, y, z} eq.not emptyset$, the clause gadget $Delta^0_nu$ (together with the variable COTs determined by $S$) is consistent. The paper provides explicit cyclic orderings for all 7 cases:

#table(
  columns: (auto, auto, auto),
  align: center,
  table.header[$S sect {x,y,z}$][$Delta$][Cyclic ordering],
  ${x}$, $Delta^0 union {a c b, d f e, g h i}$, $a c k m b d e f j l n g i h$,
  ${y}$, $Delta^0 union {a b c, d f e, g h i}$, $a b c j k d m f l n e g h i$,
  ${z}$, $Delta^0 union {a b c, d e f, g i h}$, $a b c d e f j k l n g i m h$,
  ${x,y}$, $Delta^0 union {a c b, d f e, g h i}$, $a c k m b d f e j l n g i h$,
  ${x,z}$, $Delta^0 union {a c b, d e f, g i h}$, $a c k m b d e f j l n g i h$,
  ${y,z}$, $Delta^0 union {a b c, d f e, g i h}$, $a b c j k d m f l n e g i h$,
  ${x,y,z}$, $Delta^0 union {a c b, d f e, g i h}$, $a c b j k d m f l n e g i h$,
)

Since the auxiliary element sets $B_nu = {j_nu, k_nu, l_nu, m_nu, n_nu}$ are pairwise disjoint and disjoint from $A$, the per-clause orderings combine into a global cyclic ordering.

=== Backward direction ($arrow.l$)

Suppose $Delta_1^0 union dots union Delta_p^0$ is consistent and $C$ is the cyclic ordering. Define $S = {x in U : "COT of" x "is NOT derived from" C}$. Then $u_tau in S arrow.l.r.double overline(u)_tau in.not S$.

By the contrapositive of Lemma 1: if $S sect {x_nu, y_nu, z_nu} = emptyset$ then $Delta^0_nu$ is _inconsistent_. The proof proceeds by a chain-of-implications argument showing that when all three literal COTs are derived (i.e., no literal is in $S$), the 10 gadget COTs plus the three forward COTs together require both $n m l$ and $l m n$ to be derived from $C$, which is impossible. Contradiction.

Therefore $S sect {x_nu, y_nu, z_nu} eq.not emptyset$ for every clause, and $S$ is a satisfying assignment. $square$

== Solution Extraction

Given a consistent cyclic ordering $C$ (represented as a permutation $f$), determine for each variable $tau$:
- $u_tau = "TRUE"$ if the COT $alpha_tau beta_tau gamma_tau$ is *not* derived from $C$ (i.e., $f(alpha_tau), f(beta_tau), f(gamma_tau)$ are NOT in cyclic order)
- $u_tau = "FALSE"$ if the COT $alpha_tau beta_tau gamma_tau$ IS derived from $C$

== Gadget Property (Computationally Verified)

The core correctness of the reduction rests on a single combinatorial fact, which we verified by exhaustive backtracking over all $14!/(14) = 13!$ permutations of 14 local elements:

*For any truth assignment to the 3 literal variables of a clause:*
- If at least one literal is TRUE, the 10 COTs of $Delta^0$ plus the 3 variable ordering constraints are simultaneously satisfiable.
- If all three literals are FALSE, they are NOT simultaneously satisfiable.

This was verified for all $2^3 = 8$ truth patterns.

== Example

*Source (3-SAT):* $r = 3$ variables, $p = 1$ clause: $(x_1 or x_2 or x_3)$

*Elements:* $alpha_1, beta_1, gamma_1, alpha_2, beta_2, gamma_2, alpha_3, beta_3, gamma_3$ (9 variable elements) + $j, k, l, m, n$ (5 auxiliary) = 14 total

*10 COTs ($Delta^0$):*
$
& (alpha_1, gamma_1, j), quad (beta_1, j, k), quad (gamma_1, k, l), \
& (alpha_2, gamma_2, j), quad (beta_2, j, l), quad (gamma_2, l, m), \
& (alpha_3, gamma_3, k), quad (beta_3, k, m), quad (gamma_3, m, n), quad (n, m, l)
$

*Satisfying assignment:* $x_1 = "FALSE", x_2 = "FALSE", x_3 = "TRUE"$ satisfies the clause. The backtracking solver finds a valid cyclic ordering of all 14 elements satisfying all 10 COTs.

*Extraction:* From the cyclic ordering, $(alpha_3, beta_3, gamma_3)$ is NOT in cyclic order $arrow.r x_3 = "TRUE"$, while $(alpha_1, beta_1, gamma_1)$ and $(alpha_2, beta_2, gamma_2)$ ARE in cyclic order $arrow.r x_1 = x_2 = "FALSE"$.

== References

- *[Galil and Megiddo, 1977]:* Z. Galil and N. Megiddo. "Cyclic ordering is NP-complete." _Theoretical Computer Science_ 5(2), pp. 179--182.
- *[Garey and Johnson, 1979]:* M. R. Garey and D. S. Johnson. _Computers and Intractability: A Guide to the Theory of NP-Completeness._ W. H. Freeman, pp. 225 (MS2).
