// Reduction proof: KSatisfiability(K3) -> SimultaneousIncongruences
// Reference: Stockmeyer and Meyer (1973), "Word problems requiring exponential time"
// Garey & Johnson, Computers and Intractability, Appendix A7.1, p.249

#set page(width: auto, height: auto, margin: 15pt)
#set text(size: 10pt)

= 3-SAT $arrow.r$ Simultaneous Incongruences

== Problem Definitions

*3-SAT (KSatisfiability with $K=3$):*
Given a set $U = {x_1, dots, x_n}$ of Boolean variables and a collection $C = {C_1, dots, C_m}$ of clauses over $U$, where each clause $C_j = (l_1^j or l_2^j or l_3^j)$ contains exactly 3 literals, is there a truth assignment $tau: U arrow {0,1}$ satisfying all clauses?

*Simultaneous Incongruences:*
Given a collection ${(a_1, b_1), dots, (a_k, b_k)}$ of ordered pairs of positive integers with $1 <= a_i <= b_i$, is there a non-negative integer $x$ such that $x equiv.not a_i mod b_i$ for all $i$?

== Reduction Construction

Given a 3-SAT instance $(U, C)$ with $n$ variables and $m$ clauses, construct a Simultaneous Incongruences instance as follows.

=== Step 1: Prime Assignment

For each variable $x_i$ ($1 <= i <= n$), assign a distinct prime $p_i >= 5$.  Specifically, let $p_1, p_2, dots, p_n$ be the first $n$ primes that are $>= 5$ (i.e., $5, 7, 11, 13, dots$).

We encode the Boolean value of $x_i$ via the residue of $x$ modulo $p_i$:
- $x equiv 1 mod p_i$ encodes $x_i = "TRUE"$
- $x equiv 2 mod p_i$ encodes $x_i = "FALSE"$

=== Step 2: Forbid Invalid Residue Classes

For each variable $x_i$ and each residue $r in {3, 4, dots, p_i - 1} union {0}$, add a pair to forbid that residue class:
- For $r in {3, 4, dots, p_i - 1}$: add pair $(r, p_i)$ since $1 <= r <= p_i - 1 < p_i$.
- For $r = 0$: add pair $(p_i, p_i)$ since $p_i % p_i = 0$, so this forbids $x equiv 0 mod p_i$.

This gives $(p_i - 2)$ forbidden pairs per variable, ensuring $x mod p_i in {1, 2}$.

=== Step 3: Clause Encoding via CRT

For each clause $C_j = (l_1 or l_2 or l_3)$ over variables $x_(i_1), x_(i_2), x_(i_3)$:

The clause is violated when all three literals are simultaneously false. For each literal $l_k$:
- If $l_k = x_(i_k)$ (positive), it is false when $x equiv 2 mod p_(i_k)$.
- If $l_k = overline(x)_(i_k)$ (negative), it is false when $x equiv 1 mod p_(i_k)$.

Let $r_k$ be the "falsifying residue" for literal $l_k$:
$
r_k = cases(2 &"if" l_k = x_(i_k) "(positive literal)", 1 &"if" l_k = overline(x)_(i_k) "(negative literal)")
$

The modulus for this clause is $M_j = p_(i_1) dot p_(i_2) dot p_(i_3)$. Since $p_(i_1), p_(i_2), p_(i_3)$ are distinct primes, by the Chinese Remainder Theorem there is a unique $R_j in {0, 1, dots, M_j - 1}$ satisfying:
$
R_j equiv r_1 mod p_(i_1), quad R_j equiv r_2 mod p_(i_2), quad R_j equiv r_3 mod p_(i_3)
$

Add the pair:
- If $R_j > 0$: add $(R_j, M_j)$ (valid since $1 <= R_j < M_j$).
- If $R_j = 0$: add $(M_j, M_j)$ (valid since $M_j >= 1$, and $M_j % M_j = 0$ forbids $x equiv 0 mod M_j$).

This forbids precisely the assignment where all three literals in $C_j$ are false.

=== Size Analysis

- Variable-encoding pairs: $sum_(i=1)^n (p_i - 2)$ pairs. Since $p_i$ is the $i$-th prime $>= 5$, by the prime number theorem $p_i = O(n log n)$, so the total is $O(n^2 log n)$ in the worst case. For small $n$, this is $sum_(i=1)^n (p_i - 2)$.
- Clause pairs: $m$ pairs, one per clause.
- Total pairs: $sum_(i=1)^n (p_i - 2) + m$.

== Correctness Proof

*Claim:* The 3-SAT instance $(U, C)$ is satisfiable if and only if the Simultaneous Incongruences instance has a solution.

=== Forward direction ($arrow.r$)

Suppose $tau$ satisfies all 3-SAT clauses. Define residues:
$
r_i = cases(1 &"if" tau(x_i) = "TRUE", 2 &"if" tau(x_i) = "FALSE")
$

By the CRT (since $p_1, dots, p_n$ are distinct primes), there exists $x$ with $x equiv r_i mod p_i$ for all $i$.

1. *Variable-encoding pairs:* For each variable $x_i$, $x mod p_i in {1, 2}$, so $x$ avoids all forbidden residues ${0, 3, 4, dots, p_i - 1}$.

2. *Clause pairs:* For each clause $C_j$, since $tau$ satisfies $C_j$, at least one literal is true. Thus the assignment $(x mod p_(i_1), x mod p_(i_2), x mod p_(i_3))$ differs from the all-false residue triple $(r_1, r_2, r_3)$, meaning $x equiv.not R_j mod M_j$. Hence $x$ avoids the forbidden clause residue.

Therefore $x$ satisfies all incongruences. $square$

=== Backward direction ($arrow.l$)

Suppose $x$ satisfies all incongruences. The variable-encoding pairs force $x mod p_i in {1, 2}$ for each $i$. Define:
$
tau(x_i) = cases("TRUE" &"if" x mod p_i = 1, "FALSE" &"if" x mod p_i = 2)
$

For each clause $C_j = (l_1 or l_2 or l_3)$: the clause pair forbids $x equiv R_j mod M_j$. Since $x equiv.not R_j mod M_j$, the residue triple $(x mod p_(i_1), x mod p_(i_2), x mod p_(i_3)) != (r_1, r_2, r_3)$ (the all-false triple). Therefore at least one literal evaluates to true under $tau$, and the clause is satisfied. $square$

== Solution Extraction

Given $x$ satisfying all incongruences, for each variable $x_i$:
$
tau(x_i) = cases("TRUE" &"if" x mod p_i = 1, "FALSE" &"if" x mod p_i = 2)
$

== YES Example

*Source (3-SAT):* $n = 2$, $m = 2$ clauses:
- $C_1 = (x_1 or x_2 or x_1)$  — note: variable repetition is avoided by using $n >= 3$ in practice.

Let us use a proper example with $n = 3$:
- $C_1 = (x_1 or x_2 or x_3)$

*Construction:*

Primes: $p_1 = 5, p_2 = 7, p_3 = 11$.

Variable-encoding pairs:
- $x_1$ ($p_1 = 5$): forbid residues $0, 3, 4$ $arrow.r$ pairs $(5, 5), (3, 5), (4, 5)$
- $x_2$ ($p_2 = 7$): forbid residues $0, 3, 4, 5, 6$ $arrow.r$ pairs $(7, 7), (3, 7), (4, 7), (5, 7), (6, 7)$
- $x_3$ ($p_3 = 11$): forbid residues $0, 3, 4, 5, 6, 7, 8, 9, 10$ $arrow.r$ pairs $(11, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11), (8, 11), (9, 11), (10, 11)$

Clause pair for $C_1 = (x_1 or x_2 or x_3)$: all-false means $x_1 = x_2 = x_3 = "FALSE"$, i.e., $x equiv 2 mod 5, x equiv 2 mod 7, x equiv 2 mod 11$. By CRT: $x equiv 2 mod 385$. Add pair $(2, 385)$.

Total: $3 + 5 + 9 + 1 = 18$ pairs.

*Verification:*

Setting $x_1 = "TRUE"$ gives $x equiv 1 mod 5, x equiv 1 mod 7, x equiv 1 mod 11$, i.e., $x = 1$ (by CRT, $x equiv 1 mod 385$).

Check $x = 1$:
- Variable pairs: $1 mod 5 = 1$ (not $0,3,4$) #sym.checkmark, $1 mod 7 = 1$ (not $0,3,4,5,6$) #sym.checkmark, $1 mod 11 = 1$ (not $0,3,...,10$) #sym.checkmark
- Clause pair: $1 mod 385 = 1 != 2$ #sym.checkmark

Extract: $tau(x_1) = "TRUE"$ (1 mod 5 = 1), $tau(x_2) = "TRUE"$ (1 mod 7 = 1), $tau(x_3) = "TRUE"$ (1 mod 11 = 1). Clause $(x_1 or x_2 or x_3)$ is satisfied. #sym.checkmark

== NO Example

*Source (3-SAT):* $n = 3$, $m = 8$ — all 8 sign patterns on variables $x_1, x_2, x_3$:

$(x_1 or x_2 or x_3)$, $(overline(x)_1 or overline(x)_2 or overline(x)_3)$, $(x_1 or overline(x)_2 or x_3)$, $(overline(x)_1 or x_2 or overline(x)_3)$, $(x_1 or x_2 or overline(x)_3)$, $(overline(x)_1 or overline(x)_2 or x_3)$, $(overline(x)_1 or x_2 or x_3)$, $(x_1 or overline(x)_2 or overline(x)_3)$.

This is unsatisfiable (every assignment falsifies at least one clause). The 8 clause pairs forbid all 8 possible residue triples for $(x mod 5, x mod 7, x mod 11) in {1, 2}^3$, so together with the variable-encoding pairs, no valid $x$ exists in the Simultaneous Incongruences instance.
