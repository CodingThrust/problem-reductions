// Proposed Reduction: NAE3SAT → Set Splitting (#841)
// Verification document for issue #841
#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")
#set math.equation(numbering: "(1)")

#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules
#show: thmrules.with(qed-symbol: $square$)

#let theorem = thmbox("theorem", "Theorem", fill: rgb("#e8e8ff"))
#let proof = thmproof("proof", "Proof")

= NAE 3-SAT $arrow.r$ Set Splitting <sec:nae3sat-setsplitting>

== Problem Definitions

*Not-All-Equal 3-SAT (NAE3SAT).* Given a set of Boolean variables $V = {v_1, dots, v_n}$ and a collection of clauses $C = {c_1, dots, c_m}$ where each clause contains exactly 3 literals over $V$, determine whether there exists a truth assignment $sigma: V arrow {0, 1}$ such that every clause contains at least one true literal and at least one false literal under $sigma$.

*Set Splitting.* Given a finite universe $S$ and a collection $cal(C) = {C_1, dots, C_k}$ of subsets of $S$, determine whether there exists a partition of $S$ into two disjoint sets $S_1, S_2$ (with $S_1 union S_2 = S$ and $S_1 inter S_2 = emptyset$) such that no subset $C_i in cal(C)$ is entirely contained in $S_1$ or entirely contained in $S_2$. Equivalently, for every $C_i in cal(C)$, both $C_i inter S_1 != emptyset$ and $C_i inter S_2 != emptyset$.

== Reduction

#theorem[
  NAE 3-SAT is polynomial-time reducible to Set Splitting.
] <thm:nae3sat-setsplitting>

#proof[
  _Construction._ Let $(V, C)$ be an instance of NAE 3-SAT with variables $V = {v_1, dots, v_n}$ and clauses $C = {c_1, dots, c_m}$, where each clause $c_j$ contains exactly 3 literals.

  Construct a Set Splitting instance $(S, cal(C))$ as follows:

  + *Universe:* $S = {v_1, dots, v_n, overline(v)_1, dots, overline(v)_n}$. For each variable $v_i in V$, introduce two elements: a _positive copy_ $v_i$ and a _negative copy_ $overline(v)_i$. The universe has $|S| = 2n$ elements.

  + *Complementarity subsets:* For each variable $v_i$ ($1 <= i <= n$), add the subset ${v_i, overline(v)_i}$ to $cal(C)$. These $n$ subsets enforce that $v_i$ and $overline(v)_i$ are placed into different parts of the partition.

  + *Clause subsets:* For each clause $c_j = (ell_1, ell_2, ell_3)$ ($1 <= j <= m$), add the subset ${lambda(ell_1), lambda(ell_2), lambda(ell_3)}$ to $cal(C)$, where the literal-to-element mapping $lambda$ is defined by:
    $ lambda(ell) = cases(v_i &"if" ell = v_i, overline(v)_i &"if" ell = not v_i) $

  The resulting collection $cal(C)$ has $n + m$ subsets: $n$ complementarity subsets of size 2, and $m$ clause subsets of size 3.

  _Correctness._

  ($arrow.r.double$) Suppose the NAE 3-SAT instance is satisfiable, with satisfying assignment $sigma$. Define a partition of $S$:
  $ S_1 = {v_i : sigma(v_i) = 1} union {overline(v)_i : sigma(v_i) = 0}, quad S_2 = S without S_1 $

  We verify that $(S_1, S_2)$ is a valid set splitting:
  - _Complementarity subsets:_ For each variable $v_i$, exactly one of $sigma(v_i) = 1$ or $sigma(v_i) = 0$ holds. If $sigma(v_i) = 1$, then $v_i in S_1$ and $overline(v)_i in S_2$. If $sigma(v_i) = 0$, then $v_i in S_2$ and $overline(v)_i in S_1$. In either case, the subset ${v_i, overline(v)_i}$ is split between $S_1$ and $S_2$.
  - _Clause subsets:_ Since $sigma$ is a NAE-satisfying assignment, each clause $c_j = (ell_1, ell_2, ell_3)$ has at least one true literal and at least one false literal. A true literal $ell$ maps to an element $lambda(ell)$ placed in $S_1$ (by construction of the partition), and a false literal maps to an element in $S_2$. Therefore the clause subset ${lambda(ell_1), lambda(ell_2), lambda(ell_3)}$ has at least one element in each of $S_1$ and $S_2$.

  ($arrow.l.double$) Suppose the Set Splitting instance has a valid partition $(S_1, S_2)$. Define a truth assignment $sigma$ by:
  $ sigma(v_i) = cases(1 &"if" v_i in S_1, 0 &"if" v_i in S_2) $

  We verify that $sigma$ is a NAE-satisfying assignment:
  - _Consistency:_ For each variable $v_i$, the complementarity subset ${v_i, overline(v)_i}$ is split, so exactly one of $v_i, overline(v)_i$ is in $S_1$ and the other in $S_2$. This means the element $overline(v)_i$ represents the negation of $v_i$: if $v_i in S_1$ (i.e., $sigma(v_i) = 1$), then $overline(v)_i in S_2$; if $v_i in S_2$ (i.e., $sigma(v_i) = 0$), then $overline(v)_i in S_1$.
  - _NAE property:_ For each clause $c_j = (ell_1, ell_2, ell_3)$, the clause subset ${lambda(ell_1), lambda(ell_2), lambda(ell_3)}$ is split, so at least one element is in $S_1$ and at least one in $S_2$. An element $lambda(ell)$ is in $S_1$ if and only if the literal $ell$ evaluates to true under $sigma$. This holds because $v_i in S_1$ exactly when $sigma(v_i) = 1$, and $overline(v)_i in S_1$ exactly when $sigma(v_i) = 0$, which is when $not v_i$ evaluates to true. Therefore, at least one literal in the clause is true and at least one is false, satisfying the NAE condition.

  _Solution extraction._ Given a valid partition $(S_1, S_2)$, extract the truth assignment $sigma(v_i) = 1$ if $v_i in S_1$, and $sigma(v_i) = 0$ if $v_i in S_2$. The complementarity subsets guarantee this is well-defined, and the clause subsets guarantee the NAE property as shown above.
]

== Overhead

#table(
  columns: (auto, auto),
  align: (left, left),
  table.header([*Target metric*], [*Formula*]),
  [`universe_size`], [$2 dot n$],
  [`num_subsets`], [$n + m$],
)

where $n$ = number of variables (`num_vars`) and $m$ = number of clauses (`num_clauses`) in the NAE 3-SAT instance.

*Derivation.* The universe contains one positive and one negative element per variable, giving $2n$ elements. The collection contains $n$ complementarity subsets (one per variable) and $m$ clause subsets (one per clause), totaling $n + m$ subsets. Each complementarity subset has size 2, and each clause subset has size 3.

== Feasible Example (YES Instance)

*Source (NAE 3-SAT):* $n = 4$ variables ${v_1, v_2, v_3, v_4}$, $m = 3$ clauses:
$ c_1 &= (v_1, v_2, v_3) \
  c_2 &= (not v_1, v_3, v_4) \
  c_3 &= (v_2, not v_3, not v_4) $

*Target (Set Splitting):* Universe $S = {v_1, v_2, v_3, v_4, overline(v)_1, overline(v)_2, overline(v)_3, overline(v)_4}$ ($|S| = 8 = 2 dot 4$), collection with $4 + 3 = 7$ subsets:
- Complementarity: ${v_1, overline(v)_1}$, ${v_2, overline(v)_2}$, ${v_3, overline(v)_3}$, ${v_4, overline(v)_4}$
- Clause subsets: ${v_1, v_2, v_3}$, ${overline(v)_1, v_3, v_4}$, ${v_2, overline(v)_3, overline(v)_4}$

*Satisfying assignment:* $sigma = (v_1 = 1, v_2 = 0, v_3 = 1, v_4 = 0)$.

*NAE check:*
- $c_1 = (1, 0, 1)$: has true and false $checkmark$
- $c_2 = (0, 1, 0)$: has true and false $checkmark$
- $c_3 = (0, 0, 1)$: has true and false $checkmark$

*Extracted partition:*
$ S_1 = {v_1, overline(v)_2, v_3, overline(v)_4}, quad S_2 = {overline(v)_1, v_2, overline(v)_3, v_4} $

*Splitting check:*
- ${v_1, overline(v)_1}$: $v_1 in S_1, overline(v)_1 in S_2$ $checkmark$
- ${v_2, overline(v)_2}$: $overline(v)_2 in S_1, v_2 in S_2$ $checkmark$
- ${v_3, overline(v)_3}$: $v_3 in S_1, overline(v)_3 in S_2$ $checkmark$
- ${v_4, overline(v)_4}$: $overline(v)_4 in S_1, v_4 in S_2$ $checkmark$
- ${v_1, v_2, v_3}$: $v_1, v_3 in S_1$ and $v_2 in S_2$ $checkmark$
- ${overline(v)_1, v_3, v_4}$: $v_3 in S_1$ and $overline(v)_1, v_4 in S_2$ $checkmark$
- ${v_2, overline(v)_3, overline(v)_4}$: $overline(v)_4 in S_1$ and $v_2, overline(v)_3 in S_2$ $checkmark$

== Infeasible Example (NO Instance)

*Source (NAE 3-SAT):* $n = 3$ variables ${v_1, v_2, v_3}$, $m = 8$ clauses consisting of all $2^3$ possible sign patterns over 3 variables:
$ c_1 &= (v_1, v_2, v_3) \
  c_2 &= (not v_1, v_2, v_3) \
  c_3 &= (v_1, not v_2, v_3) \
  c_4 &= (v_1, v_2, not v_3) \
  c_5 &= (not v_1, not v_2, v_3) \
  c_6 &= (not v_1, v_2, not v_3) \
  c_7 &= (v_1, not v_2, not v_3) \
  c_8 &= (not v_1, not v_2, not v_3) $

*Why this is NAE-unsatisfiable:* For any assignment $sigma in {0,1}^3$, one of the eight clauses has all three literals evaluating to true (the clause whose sign pattern matches $sigma$), violating the NAE condition. Concretely:
- $sigma = (1,1,1)$: $c_1 = (1,1,1)$ all true
- $sigma = (0,1,1)$: $c_2 = (1,1,1)$ all true
- $sigma = (1,0,1)$: $c_3 = (1,1,1)$ all true
- $sigma = (1,1,0)$: $c_4 = (1,1,1)$ all true
- $sigma = (0,0,1)$: $c_5 = (1,1,1)$ all true
- $sigma = (0,1,0)$: $c_6 = (1,1,1)$ all true
- $sigma = (1,0,0)$: $c_7 = (1,1,1)$ all true
- $sigma = (0,0,0)$: $c_8 = (1,1,1)$ all true

No assignment satisfies NAE for all 8 clauses simultaneously.

*Target (Set Splitting):* Universe $S = {v_1, v_2, v_3, overline(v)_1, overline(v)_2, overline(v)_3}$ ($|S| = 6 = 2 dot 3$), collection with $3 + 8 = 11$ subsets:
- Complementarity: ${v_1, overline(v)_1}$, ${v_2, overline(v)_2}$, ${v_3, overline(v)_3}$
- Clause subsets: ${v_1, v_2, v_3}$, ${overline(v)_1, v_2, v_3}$, ${v_1, overline(v)_2, v_3}$, ${v_1, v_2, overline(v)_3}$, ${overline(v)_1, overline(v)_2, v_3}$, ${overline(v)_1, v_2, overline(v)_3}$, ${v_1, overline(v)_2, overline(v)_3}$, ${overline(v)_1, overline(v)_2, overline(v)_3}$

Since the NAE 3-SAT instance is unsatisfiable, the Set Splitting instance has no valid partition. Any partition $(S_1, S_2)$ that respects the complementarity subsets corresponds to a truth assignment $sigma$, and for that $sigma$, one clause subset maps to three elements all in $S_1$ (the clause matching $sigma$), making it monochromatic.

== Reference

L. Lovász (1973). "Coverings and colorings of hypergraphs." In: _Proceedings of the 4th Southeastern Conference on Combinatorics, Graph Theory, and Computing_, pp. 3–12.

M. R. Garey and D. S. Johnson (1979). _Computers and Intractability: A Guide to the Theory of NP-Completeness_, W. H. Freeman. Problem SP4, p. 221.
