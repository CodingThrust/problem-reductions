// NAESatisfiability -> SetSplitting: Verification Notes (Issue #841)
#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules

#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

#show link: set text(blue)
#show: thmrules.with(qed-symbol: $square$)

#let theorem = thmbox("theorem", "Theorem", fill: rgb("#e8f4f8"))
#let proof = thmproof("proof", "Proof")

#align(center)[
  #text(size: 16pt, weight: "bold")[NAE-SAT $arrow.r$ Set Splitting --- Verification Note]

  #v(0.5em)
  #text(size: 11pt)[Issue \#841: Reduction from Not-All-Equal Satisfiability to Set Splitting]

  #v(0.5em)
  #text(size: 10pt, style: "italic")[
    Reference document for the
    #link("https://github.com/CodingThrust/problem-reductions")[problem-reductions] project
  ]
]

#v(1em)

= Reduction: NAE-SAT $arrow.r$ Set Splitting

== Construction and Correctness

#theorem[
  NAE-SAT is polynomial-time reducible to Set Splitting.
  Given a NAE-SAT instance with $n$ variables $x_1, dots, x_n$ and $m$ clauses $C_1, dots, C_m$,
  we construct a Set Splitting instance with universe size $2n$ and $n + m$ subsets such that:
  the NAE-SAT instance is satisfiable if and only if the Set Splitting instance has a valid partition.
]

#proof[
  *Construction.* Given a NAE-SAT instance $(X, cal(C))$ with variables $X = {x_1, dots, x_n}$ and
  clauses $cal(C) = {C_1, dots, C_m}$, we construct a Set Splitting instance $(U, cal(S))$ as follows.

  + *Universe.* Define $U = {p_1, q_1, p_2, q_2, dots, p_n, q_n}$ with $|U| = 2n$ elements.
    Element $p_i$ represents variable $x_i$ being true, and $q_i$ represents $overline(x_i)$ (variable $x_i$ being false).

  + *Complementarity subsets.* For each variable $x_i$ ($1 <= i <= n$), create the subset
    $S_i^"comp" = {p_i, q_i}$.
    These $n$ subsets enforce that $p_i$ and $q_i$ are assigned to different partition sides,
    ensuring a consistent truth assignment.

  + *Clause subsets.* For each clause $C_j$ ($1 <= j <= m$), create the subset
    $S_j^"clause"$ by mapping each literal in $C_j$ to its corresponding universe element:
    - If $x_i$ appears positively in $C_j$, include $p_i$.
    - If $overline(x_i)$ appears in $C_j$, include $q_i$.

  + *Output.* Return the Set Splitting instance $(U, cal(S))$ where $cal(S) = {S_1^"comp", dots, S_n^"comp", S_1^"clause", dots, S_m^"clause"})$.
    The universe size is $2n$ and the number of subsets is $n + m$.

  *Correctness ($arrow.r.double$).* Suppose the NAE-SAT instance has a satisfying assignment $alpha : X arrow.r {"true", "false"}$.
  Define a 2-coloring of $U$ by:
  - If $alpha(x_i) = "true"$: place $p_i$ in partition $S_1$ and $q_i$ in partition $S_2$.
  - If $alpha(x_i) = "false"$: place $p_i$ in partition $S_2$ and $q_i$ in partition $S_1$.

  Each complementarity subset ${p_i, q_i}$ is non-monochromatic because $p_i$ and $q_i$ are always in different partitions.

  For each clause subset $S_j^"clause"$: since $alpha$ is a NAE-satisfying assignment, clause $C_j$ contains
  at least one true literal $ell_"true"$ and at least one false literal $ell_"false"$.
  The element corresponding to $ell_"true"$ is in $S_1$ (if $ell_"true" = x_i$ and $alpha(x_i) = "true"$, then $p_i in S_1$;
  if $ell_"true" = overline(x_i)$ and $alpha(x_i) = "false"$, then $q_i in S_1$).
  Similarly, the element corresponding to $ell_"false"$ is in $S_2$.
  Therefore $S_j^"clause"$ is non-monochromatic.

  *Correctness ($arrow.l.double$).* Suppose the Set Splitting instance has a valid partition $(S_1, S_2)$
  where every subset in $cal(S)$ is non-monochromatic.

  Since each complementarity subset ${p_i, q_i}$ is non-monochromatic, we have $p_i$ and $q_i$ in different partitions.
  Define $alpha(x_i) = "true"$ if $p_i in S_1$, and $alpha(x_i) = "false"$ if $p_i in S_2$.

  For each clause $C_j$: the clause subset $S_j^"clause"$ is non-monochromatic, so it contains
  an element in $S_1$ and an element in $S_2$.
  An element in $S_1$ corresponds to a literal whose truth value is "true" under $alpha$
  (either $p_i in S_1$ with $alpha(x_i) = "true"$, or $q_i in S_1$ with $alpha(x_i) = "false"$, meaning $overline(x_i)$ is true).
  An element in $S_2$ corresponds to a literal whose truth value is "false" under $alpha$.
  Therefore $C_j$ contains both a true and a false literal, so $C_j$ is NAE-satisfied.

  *Solution extraction.* Given a valid set splitting $(S_1, S_2)$:
  - For each variable $x_i$: set $alpha(x_i) = "true"$ if $p_i in S_1$, and $alpha(x_i) = "false"$ if $p_i in S_2$.
  - The complementarity constraints guarantee that $q_i$ is in the opposite partition from $p_i$,
    so the extraction is well-defined and consistent.
]

*Overhead.*

#table(
  columns: (auto, auto),
  stroke: 0.5pt,
  align: (left, left),
  [*Target field*], [*Expression*],
  [`universe_size`], [$2 dot.c$ `num_vars`],
  [`num_subsets`], [`num_vars` $+$ `num_clauses`],
)

#pagebreak()

== Worked Example: YES Instance

Consider a NAE-SAT instance with $n = 3$ variables ${x_1, x_2, x_3}$ and $m = 2$ clauses:
$ C_1 = (x_1, x_2, overline(x_3)), quad C_2 = (overline(x_1), x_3, x_2) $

*Step 1: Universe.* $U = {p_1, q_1, p_2, q_2, p_3, q_3}$, indexed as elements $0, 1, 2, 3, 4, 5$.

*Step 2: Complementarity subsets.*
$ S_1^"comp" = {p_1, q_1} = {0, 1}, quad S_2^"comp" = {p_2, q_2} = {2, 3}, quad S_3^"comp" = {p_3, q_3} = {4, 5} $

*Step 3: Clause subsets.*
- $C_1 = (x_1, x_2, overline(x_3))$: map $x_1 arrow.r p_1 = 0$, $x_2 arrow.r p_2 = 2$, $overline(x_3) arrow.r q_3 = 5$.
  So $S_1^"clause" = {0, 2, 5}$.
- $C_2 = (overline(x_1), x_3, x_2)$: map $overline(x_1) arrow.r q_1 = 1$, $x_3 arrow.r p_3 = 4$, $x_2 arrow.r p_2 = 2$.
  So $S_2^"clause" = {1, 4, 2} = {1, 2, 4}$.

*Resulting Set Splitting instance:*
- Universe size: $6$
- Subsets: ${0, 1}, {2, 3}, {4, 5}, {0, 2, 5}, {1, 2, 4}$

*NAE-satisfying assignment:* $alpha = (x_1 = top, x_2 = top, x_3 = top)$.
- $C_1 = (x_1, x_2, overline(x_3))$: values $(top, top, bot)$. Has both $top$ and $bot$. $checkmark$
- $C_2 = (overline(x_1), x_3, x_2)$: values $(bot, top, top)$. Has both $top$ and $bot$. $checkmark$

*Partition:* $S_1 = {p_1, p_2, p_3} = {0, 2, 4}$ (true literals), $S_2 = {q_1, q_2, q_3} = {1, 3, 5}$ (false literals).

Config vector: $[0, 1, 0, 1, 0, 1]$ (element $i$: 0 $=$ $S_1$, 1 $=$ $S_2$).

*Verification of each subset:*
- ${0, 1}$: element 0 in $S_1$, element 1 in $S_2$. Non-monochromatic. $checkmark$
- ${2, 3}$: element 2 in $S_1$, element 3 in $S_2$. Non-monochromatic. $checkmark$
- ${4, 5}$: element 4 in $S_1$, element 5 in $S_2$. Non-monochromatic. $checkmark$
- ${0, 2, 5}$ (clause $C_1$): elements 0, 2 in $S_1$, element 5 in $S_2$. Non-monochromatic. $checkmark$
- ${1, 2, 4}$ (clause $C_2$): element 1 in $S_2$, elements 2, 4 in $S_1$. Non-monochromatic. $checkmark$

All 5 subsets are non-monochromatic. Valid set splitting. $checkmark$

#pagebreak()

== Worked Example: NO Instance

Consider a NAE-SAT instance with $n = 3$ variables ${x_1, x_2, x_3}$ and all $m = 8$ possible 3-literal clauses:
$ C_1 &= (x_1, x_2, x_3), quad & C_2 &= (x_1, x_2, overline(x_3)), \
  C_3 &= (x_1, overline(x_2), x_3), quad & C_4 &= (x_1, overline(x_2), overline(x_3)), \
  C_5 &= (overline(x_1), x_2, x_3), quad & C_6 &= (overline(x_1), x_2, overline(x_3)), \
  C_7 &= (overline(x_1), overline(x_2), x_3), quad & C_8 &= (overline(x_1), overline(x_2), overline(x_3)) $

*NAE-unsatisfiability.* For any assignment $alpha$ to 3 Boolean variables, consider the clause whose
literals match $alpha$ exactly: if $alpha = ("true", "true", "false")$, then $C_2 = (x_1, x_2, overline(x_3))$
evaluates to all-true, violating NAE. Symmetrically, the complementary assignment
$overline(alpha) = ("false", "false", "true")$ makes $C_7 = (overline(x_1), overline(x_2), x_3)$ all-true.
Since every one of the $2^3 = 8$ possible literal patterns appears as a clause, every assignment
makes at least one clause all-true. Therefore this instance is NAE-unsatisfiable.

*Reduction output.* Universe: $U = {p_1, q_1, p_2, q_2, p_3, q_3}$ (size 6). Subsets ($3 + 8 = 11$ total):

Complementarity: ${0, 1}, {2, 3}, {4, 5}$.

Clause subsets:
- $C_1 = (x_1, x_2, x_3) arrow.r {0, 2, 4}$ (all $p$'s)
- $C_2 = (x_1, x_2, overline(x_3)) arrow.r {0, 2, 5}$
- $C_3 = (x_1, overline(x_2), x_3) arrow.r {0, 3, 4}$
- $C_4 = (x_1, overline(x_2), overline(x_3)) arrow.r {0, 3, 5}$
- $C_5 = (overline(x_1), x_2, x_3) arrow.r {1, 2, 4}$
- $C_6 = (overline(x_1), x_2, overline(x_3)) arrow.r {1, 2, 5}$
- $C_7 = (overline(x_1), overline(x_2), x_3) arrow.r {1, 3, 4}$
- $C_8 = (overline(x_1), overline(x_2), overline(x_3)) arrow.r {1, 3, 5}$ (all $q$'s)

*No valid set splitting exists.* Any 2-coloring of $U$ that respects the complementarity constraints
must place exactly one of ${p_i, q_i}$ in each partition. This determines a truth assignment
$alpha(x_i) = "true"$ iff $p_i in S_1$. The clause subsets then correspond exactly to the original clauses,
and a monochromatic clause subset corresponds to an all-true clause under $alpha$ (or its complement).
Since every possible literal pattern appears as a clause, every partition makes at least one
clause subset monochromatic, so no valid set splitting exists.

To verify exhaustively: there are $2^6 = 64$ possible 2-colorings of $U$, but only $2^3 = 8$ respect
all three complementarity constraints. Each of these 8 colorings corresponds to an assignment $alpha$.
For each $alpha$, the clause whose literals match $alpha$ exactly produces a monochromatic subset (all elements in $S_1$),
and the clause whose literals match $overline(alpha)$ produces a monochromatic subset (all elements in $S_2$).
Therefore no valid splitting exists. $checkmark$
