// Standalone verification proof: SetSplitting -> Betweenness
// Issue #842 -- SET SPLITTING to BETWEENNESS
// Reference: Garey & Johnson, MS1; Opatrny, 1979

#set page(width: 210mm, height: auto, margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.1.")
#set math.equation(numbering: "(1)")

// Theorem/proof environments (self-contained, no external package)
#let theorem(body) = block(
  width: 100%, inset: 10pt, fill: rgb("#e8f0fe"), radius: 4pt,
  [*Theorem.* #body]
)
#let proof(body) = block(
  width: 100%, inset: (left: 10pt),
  [*Proof.* #body #h(1fr) $square$]
)

= Set Splitting $arrow.r$ Betweenness <sec:setsplitting-betweenness>

== Problem Definitions

*Set Splitting.* Given a finite universe $U = {0, dots, n-1}$ and a collection $cal(C) = {S_1, dots, S_m}$ of subsets of $U$ (each of size at least 2), determine whether there exists a 2-coloring $chi: U arrow {0,1}$ such that every subset in $cal(C)$ is non-monochromatic, i.e., contains elements of both colors.

*Betweenness.* Given a finite set $A$ of elements and a collection $cal(T)$ of ordered triples $(a, b, c)$ of distinct elements from $A$, determine whether there exists a one-to-one function $f: A arrow {1, 2, dots, |A|}$ such that for each $(a,b,c) in cal(T)$, either $f(a) < f(b) < f(c)$ or $f(c) < f(b) < f(a)$ (i.e., $b$ is between $a$ and $c$).

== Reduction

#theorem[
  Set Splitting is polynomial-time reducible to Betweenness.
]

#proof[
  _Construction._ Given a Set Splitting instance with universe $U = {0, dots, n-1}$ and collection $cal(C) = {S_1, dots, S_m}$ of subsets (each of size $>=$ 2), construct a Betweenness instance in three stages.

  *Stage 1: Normalize to size-3 subsets.* First, transform the Set Splitting instance so that every subset has size exactly 2 or 3, preserving feasibility. Process each subset $S_j$ with $|S_j| >= 4$ as follows. Let $S_j = {s_1, dots, s_k}$ with $k >= 4$.

  For each decomposition step, introduce a pair of fresh auxiliary universe elements $(y^+, y^-)$ with a complementarity subset ${y^+, y^-}$ (forcing $chi(y^+) != chi(y^-)$). Replace $S_j$ by:
  $ "NAE"(s_1, s_2, y^+) quad "and" quad "NAE"(y^-, s_3, dots, s_k) $
  That is, create subset ${s_1, s_2, y^+}$ of size 3 and subset ${y^-, s_3, dots, s_k}$ of size $k - 1$. Recurse on the second subset until it has size $<=$ 3. This yields $k - 3$ auxiliary pairs and $k - 3$ complementarity subsets plus $k - 2$ subsets of size 2 or 3 (replacing the original subset).

  After normalization, we have universe size $n' = n + 2 sum_j max(0, |S_j| - 3)$ and all subsets have size 2 or 3.

  *Stage 2: Build the Betweenness instance.* Let $p$ be a distinguished _pole_ element. The elements of the Betweenness instance are:
  $ A = {a_0, dots, a_(n'-1), p} $
  where $a_i$ represents universe element $i$. The 2-coloring is encoded by position relative to the pole: $chi(i) = 0$ if $a_i$ is to the left of $p$ in the ordering, and $chi(i) = 1$ if $a_i$ is to the right of $p$.

  *Size-2 subsets.* For each size-2 subset ${u, v}$, add the betweenness triple:
  $ (a_u, p, a_v) $
  This forces $p$ between $a_u$ and $a_v$, ensuring $u$ and $v$ are on opposite sides of $p$ and hence receive different colors.

  *Size-3 subsets.* For each size-3 subset ${u, v, w}$, introduce a fresh auxiliary element $d$ (not in $U$) and add two betweenness triples:
  $ (a_u, d, a_v) quad "and" quad (d, p, a_w) $
  The first triple forces $d$ between $a_u$ and $a_v$. The second forces $p$ between $d$ and $a_w$. Together, these are satisfiable if and only if ${u, v, w}$ is non-monochromatic.

  *Stage 3: Output.* The Betweenness instance has:
  - $|A| = n' + 1 + D$ elements, where $D$ is the number of size-3 subsets (each contributing one auxiliary $d$), and
  - $|cal(T)|$ = (number of size-2 subsets) + 2 $times$ (number of size-3 subsets) triples.

  _Gadget correctness for size-3 subsets._ We show that the two triples $(a_u, d, a_v)$ and $(d, p, a_w)$ are simultaneously satisfiable in a linear ordering if and only if ${u, v, w}$ is not monochromatic with respect to $p$.

  ($arrow.r.double$) Suppose ${u, v, w}$ is non-monochromatic: at least one element is on each side of $p$. We consider cases.

  _Case 1: $w$ is on a different side from at least one of $u, v$._ Without loss of generality, suppose $a_u < p$ and $a_w > p$. Place $d$ between $a_u$ and $a_v$. If $a_v < p$: choose $d$ with $a_u < d < a_v$ (or $a_v < d < a_u$), then $d < p < a_w$ so $(d, p, a_w)$ holds. If $a_v > p$: choose $d$ between $a_u$ and $a_v$ with $d > p$ (possible since $a_v > p > a_u$), then $a_w > p$ and $d > p$, and we need $p$ between $d$ and $a_w$. If $d < a_w$, choose $d$ close to $p$ from the right; then we need $a_w > p > d$... but $d > p$ contradicts this. Instead, choose $d$ just above $a_u$ (so $d < p$). Then $d < p < a_w$. And $a_u < d < a_v$ holds since $a_u < d < p < a_v$. Both triples satisfied.

  _Case 2: $u$ and $v$ are on different sides of $p$, $w$ on either side._ Say $a_u < p < a_v$. Place $d$ between $a_u$ and $a_v$. If $a_w < p$: place $d > p$ (so $a_u < p < d < a_v$). Then $a_w < p < d$, so $p$ is between $a_w$ and $d$: $(d, p, a_w)$ holds. If $a_w > p$: place $d < p$ (so $a_u < d < p < a_v$). Then $d < p < a_w$, so $(d, p, a_w)$ holds.

  ($arrow.l.double$) Suppose ${u, v, w}$ is monochromatic: all three on the same side of $p$. Say all $a_u, a_v, a_w < p$ (the case where all are $> p$ is symmetric). Triple $(a_u, d, a_v)$ forces $d$ between $a_u$ and $a_v$, so $d < p$. Triple $(d, p, a_w)$ requires $p$ between $d$ and $a_w$. But $d < p$ and $a_w < p$, so both are on the same side of $p$, and $p$ cannot be between them. Contradiction.

  _Correctness of the full reduction._

  ($arrow.r.double$) Suppose $chi$ is a valid 2-coloring for the (normalized) Set Splitting instance. Build a linear ordering as follows. Let $L = {a_i : chi(i) = 0}$ and $R = {a_i : chi(i) = 1}$. Order all elements of $L$ to the left of $p$ and all elements of $R$ to the right of $p$. For each size-2 subset ${u,v}$: since $chi(u) != chi(v)$, $a_u$ and $a_v$ are on opposite sides of $p$, so $(a_u, p, a_v)$ is satisfied. For each size-3 subset ${u,v,w}$: by the gadget correctness (forward direction), we can place auxiliary $d$ to satisfy both triples.

  ($arrow.l.double$) Suppose a linear ordering of $A$ satisfies all betweenness triples. For size-2 subsets, $(a_u, p, a_v)$ forces $u$ and $v$ to be on opposite sides of $p$, hence non-monochromatic. For size-3 subsets, by the gadget correctness (backward direction), ${u,v,w}$ is non-monochromatic. Thus the coloring $chi(i) = 0$ if $a_i$ is left of $p$, $chi(i) = 1$ if right of $p$, is a valid set splitting. By the correctness of the Stage 1 decomposition, this yields a valid splitting of the original instance.

  _Solution extraction._ Given a valid linear ordering $f$ of the Betweenness instance, extract the Set Splitting coloring as:
  $ chi(i) = cases(0 &"if" f(a_i) < f(p), 1 &"if" f(a_i) > f(p)) $
  for each original universe element $i in {0, dots, n-1}$.
]

*Overhead.*

#table(
  columns: (auto, auto),
  table.header([*Target metric*], [*Formula*]),
  [`num_elements`], [$n' + 1 + D$ where $n'$ is the expanded universe size and $D$ is the number of size-3 subsets],
  [`num_triples`], [number of size-2 subsets $+ 2 times$ number of size-3 subsets],
)

For the common case where all subsets have size $<=$ 3 (no decomposition needed), the overhead simplifies to:
#table(
  columns: (auto, auto),
  table.header([*Target metric*], [*Formula*]),
  [`num_elements`], [$n + 1 + D$ where $D$ = number of size-3 subsets],
  [`num_triples`], [(number of size-2 subsets) $+ 2 D$],
)

== Feasible Example (YES Instance)

Consider the Set Splitting instance with universe $U = {0, 1, 2, 3, 4}$ ($n = 5$) and subsets:
$ S_1 = {0, 1, 2}, quad S_2 = {2, 3, 4}, quad S_3 = {0, 3, 4}, quad S_4 = {1, 2, 3} $

All subsets have size 3, so no decomposition is needed.

*Reduction output.* Elements: $A = {a_0, a_1, a_2, a_3, a_4, p, d_1, d_2, d_3, d_4}$ (10 elements). Betweenness triples (using gadget $(a_u, d, a_v), (d, p, a_w)$ for each subset):
- $S_1 = {0, 1, 2}$: $(a_0, d_1, a_1)$ and $(d_1, p, a_2)$
- $S_2 = {2, 3, 4}$: $(a_2, d_2, a_3)$ and $(d_2, p, a_4)$
- $S_3 = {0, 3, 4}$: $(a_0, d_3, a_3)$ and $(d_3, p, a_4)$
- $S_4 = {1, 2, 3}$: $(a_1, d_4, a_2)$ and $(d_4, p, a_3)$

Total: 8 triples.

*Solution.* The coloring $chi = (1, 0, 1, 0, 0)$ (i.e., $S_1 = {1, 3, 4}$ in color 0, $S_2 = {0, 2}$ in color 1) splits all subsets:
- $S_1 = {0, 1, 2}$: colors $(1, 0, 1)$ -- non-monochromatic.
- $S_2 = {2, 3, 4}$: colors $(1, 0, 0)$ -- non-monochromatic.
- $S_3 = {0, 3, 4}$: colors $(1, 0, 0)$ -- non-monochromatic.
- $S_4 = {1, 2, 3}$: colors $(0, 1, 0)$ -- non-monochromatic.

*Ordering.* Place elements with color 0 left of $p$ and color 1 right: $a_1, a_3, a_4 < p < a_0, a_2$. A specific ordering: $a_3, a_4, a_1, d_1, p, d_4, d_2, d_3, a_0, a_2$, which satisfies all 8 betweenness triples.

*Extraction:* $chi(i) = 0$ if $f(a_i) < f(p)$, else $chi(i) = 1$. Gives $(1, 0, 1, 0, 0)$, matching the original coloring.

== Infeasible Example (NO Instance)

Consider the Set Splitting instance with $n = 3$ elements and 4 subsets:
$ S_1 = {0, 1}, quad S_2 = {1, 2}, quad S_3 = {0, 2}, quad S_4 = {0, 1, 2} $

*Why no valid splitting exists.* Size-2 subsets force: $chi(0) != chi(1)$ (from $S_1$), $chi(1) != chi(2)$ (from $S_2$), $chi(0) != chi(2)$ (from $S_3$). But $chi(0) != chi(1)$ and $chi(1) != chi(2)$ imply $chi(0) = chi(2)$ (Boolean), contradicting $chi(0) != chi(2)$.

*Reduction output.* Elements: $A = {a_0, a_1, a_2, p, d_4}$ (5 elements). Triples:
- $S_1 = {0, 1}$: $(a_0, p, a_1)$
- $S_2 = {1, 2}$: $(a_1, p, a_2)$
- $S_3 = {0, 2}$: $(a_0, p, a_2)$
- $S_4 = {0, 1, 2}$: $(a_0, d_4, a_1)$ and $(d_4, p, a_2)$

Total: 5 triples.

*Why the Betweenness instance is infeasible.* The first three triples require $p$ between each pair of $a_0, a_1, a_2$. The triple $(a_0, p, a_1)$ forces $a_0$ and $a_1$ on opposite sides of $p$; $(a_1, p, a_2)$ forces $a_1$ and $a_2$ on opposite sides; $(a_0, p, a_2)$ forces $a_0$ and $a_2$ on opposite sides. WLOG $a_0 < p < a_1$. Then $a_2$ must be on the opposite side of $p$ from $a_1$, so $a_2 < p$. But $(a_0, p, a_2)$ requires them on opposite sides, and both $a_0, a_2 < p$. Contradiction.
