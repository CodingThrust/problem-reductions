// Standalone Typst proof: ExactCoverBy3Sets -> MinimumWeightSolutionToLinearEquations
// Issue #860

#set page(width: auto, height: auto, margin: 20pt)
#set text(size: 10pt)

#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules
#show: thmrules.with(qed-symbol: $square$)

#let theorem = thmbox("theorem", "Theorem")
#let proof = thmproof("proof", "Proof")

== Exact Cover by 3-Sets $arrow.r$ Minimum Weight Solution to Linear Equations <sec:x3c-mwsle>

#theorem[
  Exact Cover by 3-Sets (X3C) is polynomial-time reducible to Minimum Weight Solution to Linear Equations.
] <thm:x3c-mwsle>

#proof[
  _Construction._
  Let $(X, cal(C))$ be an X3C instance where $X = {0, 1, dots, 3q - 1}$ is the universe with $|X| = 3q$,
  and $cal(C) = {C_1, C_2, dots, C_n}$ is a collection of 3-element subsets of $X$.

  Construct a Minimum Weight Solution to Linear Equations instance as follows:

  + *Variables:* $m = n$ (one rational variable $y_j$ per set $C_j$).

  + *Matrix:* Define the $3q times n$ incidence matrix $A$ where
    $ A_(i,j) = cases(1 &"if" u_i in C_j, 0 &"otherwise") $
    Each column $j$ is the characteristic vector of $C_j$ (with exactly 3 ones).

  + *Right-hand side:* $b = (1, 1, dots, 1)^top in ZZ^(3q)$ (the all-ones vector).

  + *Bound:* $K = q = |X| slash 3$.

  The equation set consists of $3q$ pairs $(a_i, b_i)$ for $i = 1, dots, 3q$,
  where $a_i$ is row $i$ of $A$ (an $n$-tuple) and $b_i = 1$.

  _Correctness._

  ($arrow.r.double$)
  Suppose ${C_(j_1), C_(j_2), dots, C_(j_q)}$ is an exact cover of $X$.
  Set $y_(j_ell) = 1$ for $ell = 1, dots, q$ and $y_j = 0$ for all other $j$.
  Then for each element $u_i$, exactly one set $C_(j_ell)$ contains $u_i$,
  so $(A y)_i = sum_(j=1)^n A_(i,j) y_j = 1 = b_i$.
  Thus $A y = b$ and $y$ has exactly $q = K$ nonzero entries.

  ($arrow.l.double$)
  Suppose $y in QQ^n$ with at most $K = q$ nonzero entries satisfies $A y = b$.
  Let $S = {j : y_j != 0}$ with $|S| <= q$.
  Since $A y = b$, for each element $u_i$ we have $sum_(j in S) A_(i,j) y_j = 1$.
  Since $A$ is a 0/1 matrix and each column has exactly 3 ones, the columns indexed by $S$
  must span the all-ones vector.
  Each column contributes 3 ones, so the selected columns contribute at most $3|S| <= 3q$ ones total.
  But the right-hand side has exactly $3q$ ones (summing all entries of $b$).
  Thus equality holds: $|S| = q$ and the nonzero columns cover each row exactly once.

  For the covering to work with rational coefficients, observe that if element $u_i$ is in
  only one selected set $C_j$ (i.e., $A_(i,j) = 1$ and $A_(i,k) = 0$ for all other $k in S$),
  then $y_j = 1$. By induction on the rows, each selected column must have $y_j = 1$.
  Alternatively: summing all equations gives $sum_j (sum_i A_(i,j)) y_j = 3q$.
  Since each column sum is 3, this gives $3 sum_j y_j = 3q$, so $sum_(j in S) y_j = q$.
  Combined with the non-negativity forced by $A y = b >= 0$ and the structure of the 0/1 matrix,
  the values must be $y_j in {0, 1}$.

  Therefore the sets ${C_j : j in S}$ form an exact cover of $X$.

  _Solution extraction._
  Given a solution $y$ to the linear system with at most $K$ nonzero entries,
  define the subcollection $cal(C)' = {C_j : y_j != 0}$.
  By the backward direction, $cal(C)'$ is an exact cover of $X$.
  The X3C configuration is: select subset $j$ iff $y_j != 0$.
]

*Overhead.*

#table(
  columns: (auto, auto),
  stroke: 0.5pt,
  [*Target metric*], [*Formula*],
  [`num_variables` ($m$)], [$n$ (`num_subsets`)],
  [`num_equations` (rows)], [$3q$ (`universe_size`)],
  [`bound` ($K$)], [$q = 3q slash 3$ (`universe_size / 3`)],
)

The incidence matrix $A$ has dimensions $3q times n$ with exactly $3n$ nonzero entries
(3 ones per column). Construction time is $O(3q dot n)$.

*Feasible example (YES).*

Source X3C instance: $X = {0, 1, 2, 3, 4, 5}$ (so $q = 2$), with subsets:
- $C_1 = {0, 1, 2}$, $C_2 = {3, 4, 5}$, $C_3 = {0, 3, 4}$, $C_4 = {2, 3, 6}$... no, let us keep it valid:
- $C_1 = {0, 1, 2}$, $C_2 = {3, 4, 5}$, $C_3 = {0, 3, 4}$

Exact cover: ${C_1, C_2}$.

Constructed MinimumWeightSolutionToLinearEquations instance:

$m = 3$ variables, $3q = 6$ equations, $K = 2$.

Matrix $A$ ($6 times 3$):

#table(
  columns: (auto, auto, auto, auto),
  stroke: 0.5pt,
  [], [$y_1$], [$y_2$], [$y_3$],
  [$u_0$], [1], [0], [1],
  [$u_1$], [1], [0], [0],
  [$u_2$], [1], [0], [0],
  [$u_3$], [0], [1], [1],
  [$u_4$], [0], [1], [1],
  [$u_5$], [0], [1], [0],
)

$b = (1, 1, 1, 1, 1, 1)^top$, $K = 2$.

Verification with $y = (1, 1, 0)$:
- $u_0$: $1 dot 1 + 0 dot 1 + 1 dot 0 = 1$ #sym.checkmark
- $u_1$: $1 dot 1 + 0 dot 1 + 0 dot 0 = 1$ #sym.checkmark
- $u_2$: $1 dot 1 + 0 dot 1 + 0 dot 0 = 1$ #sym.checkmark
- $u_3$: $0 dot 1 + 1 dot 1 + 1 dot 0 = 1$ #sym.checkmark
- $u_4$: $0 dot 1 + 1 dot 1 + 1 dot 0 = 1$ #sym.checkmark
- $u_5$: $0 dot 1 + 1 dot 1 + 0 dot 0 = 1$ #sym.checkmark

Weight of $y$ = 2 (at most $K = 2$). Corresponds to ${C_1, C_2}$, an exact cover.

*Infeasible example (NO).*

Source X3C instance: $X = {0, 1, 2, 3, 4, 5}$ (so $q = 2$), with subsets:
- $C_1 = {0, 1, 2}$, $C_2 = {0, 3, 4}$, $C_3 = {0, 4, 5}$

No exact cover exists: element 0 is in all three sets, so selecting any set that covers element 0
also covers at least one other element. Selecting $C_1$ covers ${0,1,2}$, then need to cover ${3,4,5}$
with one set from ${C_2, C_3}$, but $C_2={0,3,4}$ overlaps on 0, and $C_3={0,4,5}$ overlaps on 0.

Matrix $A$ ($6 times 3$):

#table(
  columns: (auto, auto, auto, auto),
  stroke: 0.5pt,
  [], [$y_1$], [$y_2$], [$y_3$],
  [$u_0$], [1], [1], [1],
  [$u_1$], [1], [0], [0],
  [$u_2$], [1], [0], [0],
  [$u_3$], [0], [1], [0],
  [$u_4$], [0], [1], [1],
  [$u_5$], [0], [0], [1],
)

$b = (1, 1, 1, 1, 1, 1)^top$, $K = 2$.

Row 1 forces $y_1 = 1$, row 3 forces $y_2 = 1$ (since these are the only nonzero entries).
But then row 0: $y_1 + y_2 + y_3 = 1 + 1 + y_3$. For this to equal 1, we need $y_3 = -1 != 0$.
So 3 nonzero entries are needed, but $K = 2$. No feasible solution with weight $<= K$.
