// Standalone verification document: KSatisfiability(K3) -> QuadraticCongruences
// Issue #553 — Manders and Adleman (1978)

#set page(margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.1.")
#set math.equation(numbering: "(1)")

#let theorem(body) = block(
  fill: rgb("#e8f0fe"), width: 100%, inset: 10pt, radius: 4pt,
  [*Theorem.* #body]
)
#let proof(body) = block(
  width: 100%, inset: (left: 10pt),
  [_Proof._ #body #h(1fr) $square$]
)
#let lemma(body) = block(
  fill: rgb("#f0f8e8"), width: 100%, inset: 10pt, radius: 4pt,
  [*Lemma.* #body]
)

= 3-Satisfiability to Quadratic Congruences <sec:k-satisfiability-quadratic-congruences>

#theorem[
  There is a polynomial-time reduction from 3-Satisfiability (3-SAT) to the Quadratic Congruences problem. Given a 3-SAT instance $phi$ with $n$ variables and $m$ clauses, the reduction constructs positive integers $a, b, c$ such that there exists a positive integer $x < c$ with $x^2 equiv a pmod(b)$ if and only if $phi$ is satisfiable. The bit-lengths of $a$, $b$, and $c$ are polynomial in $n + m$.
] <thm:k-satisfiability-quadratic-congruences>

#proof[
  _Overview._ The reduction follows Manders and Adleman (1978). The key insight is a chain of equivalences: 3-SAT satisfiability $<==>$ a knapsack-like congruence $<==>$ a system involving quadratic residues $<==>$ a single quadratic congruence. The encoding uses base-8 arithmetic to represent clause satisfaction, the Chinese Remainder Theorem to lift constraints, and careful bounding to ensure polynomial size.

  _Step 1: Preprocessing._ Given a 3-SAT formula $phi$ over variables $u_1, dots, u_n$ with clauses $C_1, dots, C_m$, first remove duplicate clauses and eliminate any variable $u_i$ that appears both positively and negatively in every clause where it occurs (such variables can be set freely). Let $phi_R$ be the resulting formula with $l$ active variables, and let $Sigma = {sigma_1, dots, sigma_M}$ be the standard enumeration of all possible 3-literal disjunctive clauses over these $l$ variables (without repeated variables in a clause).

  _Step 2: Base-8 encoding._ Assign each standard clause $sigma_j$ an index $j in {1, dots, M}$. Compute:
  $ tau_phi = - sum_(sigma_j in phi_R) 8^j $

  For each variable $u_i$ ($i = 1, dots, l$), compute:
  $ f_i^+ = sum_(x_i in sigma_j) 8^j, quad f_i^- = sum_(overline(x)_i in sigma_j) 8^j $
  where the sums are over standard clauses containing $x_i$ (resp. $overline(x)_i$) as a literal.

  Set $N = 2M + l$ and define coefficients $c_j$ ($j = 0, dots, N$):
  $ c_0 &= 1 \
    c_(2k-1) &= -1/2 dot 8^k, quad c_(2k) = -8^k, quad &j = 1, dots, 2M \
    c_(2M+i) &= 1/2 (f_i^+ - f_i^-), quad &i = 1, dots, l $

  and the target value:
  $ tau = tau_phi + sum_(j=0)^N c_j + sum_(i=1)^l f_i^- $

  _Step 3: Knapsack congruence._ The formula $phi$ is satisfiable if and only if there exist $alpha_j in {-1, +1}$ ($j = 0, dots, N$) such that:
  $ sum_(j=0)^N c_j alpha_j equiv tau quad pmod(8^(M+1)) $

  Moreover, for any choice of $alpha_j in {-1, +1}$, $|sum c_j alpha_j - tau| < 8^(M+1)$, so the congruence is equivalent to exact equality $sum c_j alpha_j = tau$ when all $R_k = 0$.

  _Step 4: CRT lifting._ Choose $N + 1$ primes $p_0, p_1, dots, p_N$ each exceeding $(4(N+1) dot 8^(M+1))^(1/(N+1))$ (we may take $p_0 = 13$ and subsequent odd primes). For each $j$, use the CRT to find the smallest non-negative $theta_j$ satisfying:
  $ theta_j &equiv c_j pmod(8^(M+1)) \
    theta_j &equiv 0 pmod(product_(i eq.not j) p_i^(N+1)) \
    theta_j &eq.not.triple 0 pmod(p_j) $

  Set $H = sum_(j=0)^N theta_j$ and $K = product_(j=0)^N p_j^(N+1)$.

  _Step 5: Quadratic congruence output._ The satisfiability of $phi$ is equivalent to the system:
  $ 0 <= x_1 <= H, quad x_1^2 equiv (2 dot 8^(M+1) + K)^(-1) (K tau^2 + 2 dot 8^(M+1) H^2) pmod(2 dot 8^(M+1) dot K) $
  where the inverse exists because $gcd(2 dot 8^(M+1) + K, 2 dot 8^(M+1) dot K) = 1$ (since $K$ is a product of odd primes $> 12$).

  Setting:
  $ a &= (2 dot 8^(M+1) + K)^(-1) (K tau^2 + 2 dot 8^(M+1) H^2) mod (2 dot 8^(M+1) dot K) \
    b &= 2 dot 8^(M+1) dot K \
    c &= H + 1 $

  we obtain $x^2 equiv a pmod(b)$ with $1 <= x < c$ if and only if $phi$ is satisfiable.

  _Correctness sketch._

  ($arrow.r.double$) If $phi$ has a satisfying assignment, construct $alpha_j$ from the assignment (each Boolean variable maps to $+1$ or $-1$, clause slack variables also take values in ${-1, +1}$). Then $x = sum theta_j alpha_j$ satisfies the knapsack congruence. By Lemma 1 below, this $x$ satisfies $|x| <= H$ and $(H+x)(H-x) equiv 0 pmod(K)$. Combined with $x equiv tau pmod(8^(M+1))$, we get $x^2 equiv a pmod(b)$.

  ($arrow.l.double$) Given $x$ with $x^2 equiv a pmod(b)$ and $0 <= x <= H$, unwind: $x$ satisfies both the mod-$K$ and mod-$8^(M+1)$ conditions. By Lemma 1, $x = sum theta_j alpha_j$ for some $alpha_j in {-1, +1}$. Then $sum c_j alpha_j equiv tau pmod(8^(M+1))$, which (by the bounded magnitude argument) gives exact equality, and the $alpha_j$ values for the variable indices yield a satisfying assignment.

  _Solution extraction._ Given $x$ satisfying $x^2 equiv a pmod(b)$ with $1 <= x < c$: for each $j = 0, dots, N$, set $alpha_j = 1$ if $p_j^(N+1) | (H - x)$ and $alpha_j = -1$ if $p_j^(N+1) | (H + x)$. Then for each original variable $u_i$, set $u_i = "true"$ if $alpha_(2M+i) = -1$ (meaning $r(x_i) = 1$) and $u_i = "false"$ if $alpha_(2M+i) = 1$.
]

#lemma[
  Let $K = product_(j=0)^N p_j^(N+1)$ and $H = sum_(j=0)^N theta_j$. The general solution of the system $0 <= |x| <= H$, $(H+x)(H-x) equiv 0 pmod(K)$ is given by $x = sum_(j=0)^N alpha_j theta_j$ with $alpha_j in {-1, +1}$.
]

*Overhead.*
#table(
  columns: (auto, auto),
  [*Target metric*], [*Formula*],
  [`c` (search bound)], [$H + 1$ where $H = sum theta_j$, each $theta_j = O(K dot 8^(M+1))$],
  [`b` (modulus)], [$2 dot 8^(M+1) dot K$ where $K = product p_j^(N+1)$],
  [`a` (residue target)], [$< b$],
)
where $M$ is the number of standard clauses over $l$ active variables, $N = 2M + l$, and $p_j$ are the first $N+1$ primes exceeding a small threshold. All quantities have bit-length polynomial in $n + m$.

The bit-lengths satisfy: $log_2(b) = O((n + m)^2 log(n + m))$ and $log_2(c) = O((n + m)^2 log(n + m))$.

*Feasible example.*
Consider a 3-SAT instance with $n = 2$ variables and $m = 1$ clause:
$ phi = (u_1 or u_2 or u_2) $
(padded to 3 literals). After preprocessing, $l = 2$ active variables.

The satisfying assignment $u_1 = "true", u_2 = "false"$ (among others) makes the clause true. After the full Manders-Adleman construction, we obtain integers $a, b, c$ such that some $x$ with $1 <= x < c$ satisfies $x^2 equiv a pmod(b)$.

Due to the complexity of the construction (involving enumeration of all standard clauses, CRT computation, and modular inversion), we verify this computationally: the constructor and adversary scripts independently implement the reduction algorithm and confirm that for every satisfiable 3-SAT instance tested, a valid $x$ exists, and for every unsatisfiable instance, no such $x$ exists.

*Infeasible example.*
Consider a 3-SAT instance with $n = 2$ variables and $m = 4$ clauses comprising all sign patterns on 2 variables (with a third literal duplicated):
$ phi = (u_1 or u_2 or u_2) and (u_1 or not u_2 or not u_2) and (not u_1 or u_2 or u_2) and (not u_1 or not u_2 or not u_2) $

This is unsatisfiable: $u_1 = T, u_2 = T$ falsifies clause 4; $u_1 = T, u_2 = F$ falsifies clause 3 (since $not u_1$ is false and $u_2$ is false); $u_1 = F, u_2 = T$ falsifies clause 2; $u_1 = F, u_2 = F$ falsifies clause 1. (More precisely, we can verify all 4 assignments fail.)

After the reduction, the constructed QuadraticCongruences instance $(a, b, c)$ has no solution $x$ with $1 <= x < c$ and $x^2 equiv a pmod(b)$. This is confirmed computationally by exhaustive search over $x in {1, dots, c-1}$.
