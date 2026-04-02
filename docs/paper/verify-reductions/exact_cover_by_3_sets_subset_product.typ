// Standalone Typst proof: ExactCoverBy3Sets -> SubsetProduct
// Issue #388

#set page(width: auto, height: auto, margin: 20pt)
#set text(size: 10pt)

#import "@preview/ctheorems:1.1.3": thmbox, thmplain, thmproof, thmrules
#show: thmrules.with(qed-symbol: $square$)

#let theorem = thmbox("theorem", "Theorem")
#let proof = thmproof("proof", "Proof")

== Exact Cover by 3-Sets $arrow.r$ Subset Product <sec:x3c-sp>

#theorem[
  Exact Cover by 3-Sets (X3C) is polynomial-time reducible to Subset Product.
] <thm:x3c-sp>

#proof[
  _Construction._
  Let $(X, cal(C))$ be an X3C instance where $X = {0, 1, dots, 3q - 1}$ is the universe with $|X| = 3q$,
  and $cal(C) = {C_1, C_2, dots, C_n}$ is a collection of 3-element subsets of $X$.

  Let $p_0 < p_1 < dots < p_(3q-1)$ be the first $3q$ prime numbers
  (i.e., $p_0 = 2, p_1 = 3, p_2 = 5, dots$).
  For each subset $C_j = {a, b, c}$ with $a < b < c$, define the size
  $ s_j = p_a dot p_b dot p_c. $
  Set the target product
  $ B = product_(i=0)^(3q-1) p_i. $

  The resulting Subset Product instance has $n$ elements with sizes $s_1, dots, s_n$ and target $B$.

  _Correctness._

  ($arrow.r.double$)
  Suppose ${C_(j_1), C_(j_2), dots, C_(j_q)}$ is an exact cover of $X$.
  Then every element of $X$ appears in exactly one selected subset, so
  $ product_(ell=1)^(q) s_(j_ell) = product_(ell=1)^(q) (p_(a_ell) dot p_(b_ell) dot p_(c_ell))
    = product_(i=0)^(3q-1) p_i = B, $
  since the union of the selected triples is exactly $X$ and they are pairwise disjoint.
  Setting $x_(j_ell) = 1$ for $ell = 1, dots, q$ and $x_j = 0$ for all other $j$
  gives a valid Subset Product solution.

  ($arrow.l.double$)
  Suppose $(x_1, dots, x_n) in {0, 1}^n$ satisfies $product_(j : x_j = 1) s_j = B$.
  Each $s_j$ is a product of exactly three distinct primes from ${p_0, dots, p_(3q-1)}$.
  By the fundamental theorem of arithmetic, $B = product_(i=0)^(3q-1) p_i$ has a unique
  prime factorization. Since each $s_j$ contributes exactly three primes, and
  $product_(j : x_j = 1) s_j = B$, the multiset union of primes from all selected subsets
  must equal the multiset ${p_0, p_1, dots, p_(3q-1)}$ (each with multiplicity 1).
  This means:
  - No prime appears more than once among selected subsets (disjointness).
  - Every prime appears at least once (completeness).
  Therefore the selected subsets form an exact cover.
  Moreover, each selected subset contributes 3 primes, and the total is $3q$,
  so exactly $q$ subsets are selected.

  _Solution extraction._
  Given a satisfying assignment $(x_1, dots, x_n)$ to the Subset Product instance,
  define $cal(C)' = {C_j : x_j = 1}$.
  By the backward direction above, $cal(C)'$ is an exact cover.
  The extraction is the identity mapping: the X3C configuration equals
  the Subset Product configuration.
]

*Overhead.*

#table(
  columns: (auto, auto),
  stroke: 0.5pt,
  [*Target metric*], [*Formula*],
  [`num_elements`], [$n$ (`num_subsets`)],
  [`target`], [$product_(i=0)^(3q-1) p_i$ (product of first $3q$ primes)],
)

Each size $s_j$ is bounded by $p_(3q-1)^3$, and the target $B$ is the primorial of $p_(3q-1)$.
Bit lengths are $O(3q log(3q))$ by the prime number theorem, so the reduction is polynomial.

*Feasible example (YES).*

Source X3C instance: $X = {0, 1, 2, 3, 4, 5, 6, 7, 8}$ (so $q = 3$), with subsets:
- $C_1 = {0, 1, 2}$, $C_2 = {3, 4, 5}$, $C_3 = {6, 7, 8}$, $C_4 = {0, 3, 6}$

Primes: $p_0 = 2, p_1 = 3, p_2 = 5, p_3 = 7, p_4 = 11, p_5 = 13, p_6 = 17, p_7 = 19, p_8 = 23$.

Sizes:
- $s_1 = p_0 dot p_1 dot p_2 = 2 dot 3 dot 5 = 30$
- $s_2 = p_3 dot p_4 dot p_5 = 7 dot 11 dot 13 = 1001$
- $s_3 = p_6 dot p_7 dot p_8 = 17 dot 19 dot 23 = 7429$
- $s_4 = p_0 dot p_3 dot p_6 = 2 dot 7 dot 17 = 238$

Target: $B = 2 dot 3 dot 5 dot 7 dot 11 dot 13 dot 17 dot 19 dot 23 = 223092870$.

Assignment $(x_1, x_2, x_3, x_4) = (1, 1, 1, 0)$:
$ s_1 dot s_2 dot s_3 = 30 dot 1001 dot 7429 = 223092870 = B #h(4pt) checkmark $

This corresponds to selecting ${C_1, C_2, C_3}$, an exact cover.

*Infeasible example (NO).*

Source X3C instance: $X = {0, 1, 2, 3, 4, 5, 6, 7, 8}$ (so $q = 3$), with subsets:
- $C_1 = {0, 1, 2}$, $C_2 = {0, 3, 4}$, $C_3 = {0, 5, 6}$, $C_4 = {3, 7, 8}$

Sizes:
- $s_1 = 2 dot 3 dot 5 = 30$
- $s_2 = 2 dot 7 dot 11 = 154$
- $s_3 = 2 dot 13 dot 17 = 442$
- $s_4 = 7 dot 19 dot 23 = 3059$

Target: $B = 223092870$.

No subset of ${30, 154, 442, 3059}$ has product $B$.
Element 0 appears in $C_1, C_2, C_3$, so selecting any two of them includes $p_0 = 2$
twice in the product, which cannot divide $B$ (where 2 appears with multiplicity 1).
At most one of $C_1, C_2, C_3$ can be selected, leaving at most 2 subsets ($<= 6$ elements),
insufficient to cover all 9 elements.
