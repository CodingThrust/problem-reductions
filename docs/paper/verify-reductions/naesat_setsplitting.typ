// Verification proof: NAESatisfiability → SetSplitting (#841)
#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules

#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.1")

#show: thmrules.with(qed-symbol: $square$)
#let theorem = thmbox("theorem", "Theorem", fill: rgb("#e8e8f8"))
#let proof = thmproof("proof", "Proof")

== NAE Satisfiability $arrow.r$ Set Splitting <sec:naesat-setsplitting>

#theorem[
  NAE Satisfiability is polynomial-time reducible to Set Splitting.
  Given a NAE-SAT instance with $n$ variables and $m$ clauses, the
  constructed Set Splitting instance has universe size $2n$ and $m + n$
  subsets.
] <thm:naesat-setsplitting>

#proof[
  _Construction._
  Let $phi$ be a NAE-SAT instance with variables $V = {v_1, dots, v_n}$ and
  clauses $C = {c_1, dots, c_m}$, where each clause $c_j$ contains at least
  two literals drawn from ${v_1, overline(v)_1, dots, v_n, overline(v)_n}$.

  Construct a Set Splitting instance $(S, cal(C))$ as follows.

  + Define the universe $S = {v_1, overline(v)_1, v_2, overline(v)_2, dots, v_n, overline(v)_n}$,
    so $|S| = 2n$. Element $v_i$ represents the positive literal of
    variable $i$, and element $overline(v)_i$ represents its negation.

  + For each variable $v_i$ ($1 <= i <= n$), add a _complementarity subset_
    $P_i = {v_i, overline(v)_i}$ to the collection $cal(C)$.

  + For each clause $c_j = (ell_(j,1), dots, ell_(j,k_j))$ ($1 <= j <= m$),
    add the _clause subset_ $Q_j = {ell_(j,1), dots, ell_(j,k_j)}$ to the
    collection $cal(C)$, where each literal $ell_(j,t)$ is identified with
    the corresponding element in $S$: a positive literal $v_i$ maps to
    element $v_i$, and a negative literal $overline(v)_i$ maps to
    element $overline(v)_i$.

  The collection is $cal(C) = {P_1, dots, P_n, Q_1, dots, Q_m}$, giving
  $|cal(C)| = n + m$ subsets in total.

  _Correctness._

  ($arrow.r.double$) Suppose $alpha: V -> {"true", "false"}$ is a NAE-satisfying
  assignment for $phi$. Define a partition $(S_1, S_2)$ of $S$ by:
  $ S_1 = {v_i : alpha(v_i) = "true"} union {overline(v)_i : alpha(v_i) = "false"}, $
  $ S_2 = {v_i : alpha(v_i) = "false"} union {overline(v)_i : alpha(v_i) = "true"}. $
  We verify that every subset in $cal(C)$ is split:
  - _Complementarity subsets:_ For each $P_i = {v_i, overline(v)_i}$,
    exactly one of $v_i, overline(v)_i$ is in $S_1$ and the other is in $S_2$
    by construction. Hence $P_i$ is split.
  - _Clause subsets:_ For each $Q_j$, since $alpha$ is NAE-satisfying,
    clause $c_j$ contains at least one true literal $ell_t$ and at least one
    false literal $ell_f$. The element corresponding to a true literal is in
    $S_1$: for a positive literal $v_i$, truth means $alpha(v_i) = "true"$,
    so $v_i in S_1$; for a negative literal $overline(v)_i$, truth means
    $alpha(v_i) = "false"$, so $overline(v)_i in S_1$. By the same
    reasoning, the element corresponding to a false literal is in $S_2$.
    Hence $Q_j$ has at least one element in each part and is split.

  ($arrow.l.double$) Suppose $(S_1, S_2)$ is a valid set splitting for
  $(S, cal(C))$. Define the assignment $alpha$ by:
  $ alpha(v_i) = cases("true" &"if" v_i in S_1, "false" &"if" v_i in S_2.) $
  This is well-defined because the complementarity subset $P_i = {v_i, overline(v)_i}$
  must be split, so exactly one of $v_i, overline(v)_i$ is in $S_1$ and the
  other is in $S_2$. In particular, $overline(v)_i in S_1$ if and only if
  $alpha(v_i) = "false"$.

  We verify that $alpha$ is NAE-satisfying. Consider any clause
  $c_j = (ell_(j,1), dots, ell_(j,k_j))$ with corresponding subset $Q_j$.
  Since $Q_j$ is split, there exist elements $ell_a in Q_j sect S_1$ and
  $ell_b in Q_j sect S_2$.
  - If $ell_a = v_i$ for some $i$, then $v_i in S_1$, so
    $alpha(v_i) = "true"$, and the literal $v_i$ evaluates to true.
  - If $ell_a = overline(v)_i$ for some $i$, then $overline(v)_i in S_1$.
    Since $P_i$ is split, $v_i in S_2$, so $alpha(v_i) = "false"$, and the
    literal $overline(v)_i$ evaluates to true.
  By the same reasoning applied to $ell_b in S_2$, literal $ell_b$ evaluates
  to false. Hence clause $c_j$ has both a true and a false literal,
  satisfying the NAE condition.

  _Solution extraction._
  Given a set splitting $(S_1, S_2)$, extract the NAE-satisfying assignment
  by setting $alpha(v_i) = "true"$ if and only if $v_i in S_1$.
  In the binary encoding used by the codebase, the universe has $2n$
  elements indexed $0, 1, dots, 2n - 1$, where element $2(i - 1)$
  represents $v_i$ and element $2(i - 1) + 1$ represents $overline(v)_i$.
  A configuration assigns each element to part 0 ($S_1$) or part 1 ($S_2$).
  Variable $i$ is set to true when element $2(i - 1)$ is in part 0.
]

*Overhead.*
#table(
  columns: (auto, auto),
  align: (left, left),
  table.header([Target metric], [Formula]),
  [`universe_size`], [$2n$ where $n =$ `num_vars`],
  [`num_subsets`], [$n + m$ where $m =$ `num_clauses`],
)

*Feasible example (YES instance).*
Consider the NAE-SAT instance with $n = 4$ variables and $m = 4$ clauses:
$ c_1 = (v_1, v_2, v_3), quad c_2 = (overline(v)_1, v_3, v_4), quad c_3 = (v_2, overline(v)_3, overline(v)_4), quad c_4 = (v_1, overline(v)_2, v_4). $

The constructed Set Splitting instance has universe size $2 dot 4 = 8$ and
$4 + 4 = 8$ subsets:
- Complementarity: $P_1 = {v_1, overline(v)_1}$, $P_2 = {v_2, overline(v)_2}$,
  $P_3 = {v_3, overline(v)_3}$, $P_4 = {v_4, overline(v)_4}$.
- Clause: $Q_1 = {v_1, v_2, v_3}$, $Q_2 = {overline(v)_1, v_3, v_4}$,
  $Q_3 = {v_2, overline(v)_3, overline(v)_4}$, $Q_4 = {v_1, overline(v)_2, v_4}$.

Using 0-indexed elements: $v_i$ is element $2(i-1)$ and $overline(v)_i$ is
element $2(i-1)+1$, so $v_1 = 0, overline(v)_1 = 1, v_2 = 2, overline(v)_2 = 3,
v_3 = 4, overline(v)_3 = 5, v_4 = 6, overline(v)_4 = 7$.

Subsets (0-indexed): ${0,1}, {2,3}, {4,5}, {6,7}, {0,2,4}, {1,4,6}, {2,5,7}, {0,3,6}$.

The assignment $alpha = (v_1 = "true", v_2 = "false", v_3 = "true", v_4 = "false")$ is NAE-satisfying:
- $c_1 = (v_1, v_2, v_3) = ("T", "F", "T")$: not all equal $checkmark$
- $c_2 = (overline(v)_1, v_3, v_4) = ("F", "T", "F")$: not all equal $checkmark$
- $c_3 = (v_2, overline(v)_3, overline(v)_4) = ("F", "F", "T")$: not all equal $checkmark$
- $c_4 = (v_1, overline(v)_2, v_4) = ("T", "T", "F")$: not all equal $checkmark$

The corresponding partition is:
$S_1 = {v_1, overline(v)_2, v_3, overline(v)_4} = {0, 3, 4, 7}$,
$S_2 = {overline(v)_1, v_2, overline(v)_3, v_4} = {1, 2, 5, 6}$.

Verification that every subset is split:
- $P_1 = {0, 1}$: $0 in S_1$, $1 in S_2$ $checkmark$
- $P_2 = {2, 3}$: $3 in S_1$, $2 in S_2$ $checkmark$
- $P_3 = {4, 5}$: $4 in S_1$, $5 in S_2$ $checkmark$
- $P_4 = {6, 7}$: $7 in S_1$, $6 in S_2$ $checkmark$
- $Q_1 = {0, 2, 4}$: $0 in S_1$, $2 in S_2$ $checkmark$
- $Q_2 = {1, 4, 6}$: $4 in S_1$, $1 in S_2$ $checkmark$
- $Q_3 = {2, 5, 7}$: $7 in S_1$, $2 in S_2$ $checkmark$
- $Q_4 = {0, 3, 6}$: $0 in S_1$, $6 in S_2$ $checkmark$

*Infeasible example (NO instance).*
Consider the NAE-SAT instance with $n = 3$ variables and $m = 4$ clauses:
$ c_1 = (v_1, v_2, v_3), quad c_2 = (v_1, v_2, overline(v)_3), quad c_3 = (v_1, overline(v)_2, v_3), quad c_4 = (v_1, overline(v)_2, overline(v)_3). $

The constructed Set Splitting instance has universe size $2 dot 3 = 6$ and
$3 + 4 = 7$ subsets:
- Complementarity: $P_1 = {v_1, overline(v)_1}$, $P_2 = {v_2, overline(v)_2}$,
  $P_3 = {v_3, overline(v)_3}$.
- Clause: $Q_1 = {v_1, v_2, v_3}$, $Q_2 = {v_1, v_2, overline(v)_3}$,
  $Q_3 = {v_1, overline(v)_2, v_3}$, $Q_4 = {v_1, overline(v)_2, overline(v)_3}$.

Using 0-indexed elements: $v_1 = 0, overline(v)_1 = 1, v_2 = 2, overline(v)_2 = 3,
v_3 = 4, overline(v)_3 = 5$.

Subsets (0-indexed): ${0,1}, {2,3}, {4,5}, {0,2,4}, {0,2,5}, {0,3,4}, {0,3,5}$.

This instance is unsatisfiable. The complementarity subsets force a consistent
assignment. Each of the $2^3 = 8$ possible assignments violates at least one
clause:
- $alpha = ("T","T","T")$: $c_1 = ("T","T","T")$ — all equal, fails.
- $alpha = ("T","T","F")$: $c_2 = ("T","T","T")$ — all equal, fails.
- $alpha = ("T","F","T")$: $c_3 = ("T","T","T")$ — all equal, fails.
- $alpha = ("T","F","F")$: $c_4 = ("T","T","T")$ — all equal, fails.
- $alpha = ("F","T","T")$: $c_1 = ("F","T","T")$ NAE $checkmark$, $c_2 = ("F","T","F")$ NAE $checkmark$, $c_3 = ("F","F","T")$ NAE $checkmark$, $c_4 = ("F","F","F")$ — all equal, fails.
- $alpha = ("F","T","F")$: $c_4 = ("F","F","T")$ NAE $checkmark$, $c_1 = ("F","T","F")$ NAE $checkmark$, $c_2 = ("F","T","T")$ NAE $checkmark$, $c_3 = ("F","F","F")$ — all equal, fails.
- $alpha = ("F","F","T")$: $c_3 = ("F","T","T")$ NAE $checkmark$, $c_1 = ("F","F","T")$ NAE $checkmark$, $c_2 = ("F","F","F")$ — all equal, fails.
- $alpha = ("F","F","F")$: $c_1 = ("F","F","F")$ — all equal, fails.

Since no assignment is NAE-satisfying, the corresponding Set Splitting instance
also has no valid partition: by the backward direction of the proof, any valid
partition would yield a NAE-satisfying assignment, which does not exist.
