// Standalone verification document: KSatisfiability(K3) -> Kernel
// Issue #882 — Chvatal (1973)

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

= 3-Satisfiability to Kernel <sec:k-satisfiability-kernel>

#theorem[
  There is a polynomial-time reduction from 3-Satisfiability (3-SAT) to the Kernel problem. Given a 3-SAT instance $phi$ with $n$ variables and $m$ clauses, the reduction constructs a directed graph $G = (V, A)$ with $|V| = 2n + 3m$ vertices and $|A| = 2n + 12m$ arcs such that $phi$ is satisfiable if and only if $G$ has a kernel.
] <thm:k-satisfiability-kernel>

#proof[
  _Construction._ Let $phi$ be a 3-SAT formula over variables $u_1, dots, u_n$ with clauses $C_1, dots, C_m$, where each clause $C_j$ is a disjunction of exactly three literals. We construct a directed graph $G = (V, A)$ in three stages.

  *Step 1 (Variable gadgets).* For each variable $u_i$ ($1 <= i <= n$), create two vertices: $x_i$ (representing the positive literal $u_i$) and $overline(x)_i$ (representing the negative literal $not u_i$). Add arcs $(x_i, overline(x)_i)$ and $(overline(x)_i, x_i)$, forming a directed 2-cycle (digon). This forces any kernel to contain exactly one of $x_i$ and $overline(x)_i$.

  *Step 2 (Clause gadgets).* For each clause $C_j$ ($1 <= j <= m$), create three auxiliary vertices $c_(j,1)$, $c_(j,2)$, $c_(j,3)$. Add arcs $(c_(j,1), c_(j,2))$, $(c_(j,2), c_(j,3))$, and $(c_(j,3), c_(j,1))$, forming a directed 3-cycle.

  *Step 3 (Connection arcs).* For each clause $C_j$ and each literal $ell_k$ ($k = 1, 2, 3$) appearing as the $k$-th literal of $C_j$, let $v$ be the vertex corresponding to $ell_k$ (that is, $v = x_i$ if $ell_k = u_i$, or $v = overline(x)_i$ if $ell_k = not u_i$). Add arcs $(c_(j,1), v)$, $(c_(j,2), v)$, and $(c_(j,3), v)$. Each clause vertex thus points to all three literal vertices of its clause.

  The total vertex count is $2n$ (variable gadgets) $+ 3m$ (clause gadgets) $= 2n + 3m$. The total arc count is $2n$ (digon arcs) $+ 3m$ (triangle arcs) $+ 9m$ (connection arcs: 3 clause vertices $times$ 3 literals $times$ 1 arc each) $= 2n + 12m$.

  _Correctness._

  ($arrow.r.double$) Suppose $phi$ has a satisfying assignment $alpha$. Define the vertex set $S$ as follows: for each variable $u_i$, include $x_i$ in $S$ if $alpha(u_i) = "true"$, and include $overline(x)_i$ if $alpha(u_i) = "false"$. We verify that $S$ is a kernel.

  _Independence:_ The only arcs between literal vertices are the digon arcs $(x_i, overline(x)_i)$ and $(overline(x)_i, x_i)$. Since $S$ contains exactly one of $x_i, overline(x)_i$ for each $i$, no arc joins two members of $S$.

  _Absorption of literal vertices:_ For each variable $u_i$, the literal vertex not in $S$ is $overline(x)_i$ (if $alpha(u_i) = "true"$) or $x_i$ (if $alpha(u_i) = "false"$). In either case, the digon arc connects this vertex to the vertex in $S$, so it is absorbed.

  _Absorption of clause vertices:_ Fix a clause $C_j$. Since $alpha$ satisfies $phi$, at least one literal $ell_k$ in $C_j$ is true under $alpha$, so the corresponding literal vertex $v$ is in $S$. Each clause vertex $c_(j,t)$ ($t = 1, 2, 3$) has an arc to $v$ (by Step 3), so every clause vertex is absorbed.

  ($arrow.l.double$) Suppose $G$ has a kernel $S$. We show that no clause vertex belongs to $S$, and then extract a satisfying assignment.

  _No clause vertex is in $S$:_ Assume for contradiction that $c_(j,1) in S$ for some $j$. By independence with the 3-cycle, $c_(j,2) , c_(j,3) in.not S$. The arcs from Step 3 give $(c_(j,1), v)$ for every literal vertex $v$ of clause $C_j$, so by independence none of these literal vertices are in $S$. But then $c_(j,2)$'s outgoing arcs go to $c_(j,3)$ (not in $S$) and to the same three literal vertices (not in $S$), so $c_(j,2)$ is not absorbed --- a contradiction. By the same argument applied to $c_(j,2)$ and $c_(j,3)$, no clause vertex belongs to $S$.

  _Variable consistency:_ Since no clause vertex is in $S$, the only vertices in $S$ are literal vertices. For each variable $u_i$, vertex $x_i$ must be absorbed: its only outgoing arc goes to $overline(x)_i$, so $overline(x)_i in S$, or vice versa. The digon structure forces exactly one of ${x_i, overline(x)_i}$ into $S$.

  _Satisfiability:_ Define $alpha(u_i) = "true"$ if $x_i in S$ and $alpha(u_i) = "false"$ if $overline(x)_i in S$. For each clause $C_j$, vertex $c_(j,1)$ is not in $S$ and must be absorbed. Its outgoing arcs go to $c_(j,2)$ (not in $S$) and to the three literal vertices of $C_j$. At least one of these literal vertices must be in $S$, meaning the corresponding literal is true under $alpha$. Hence every clause is satisfied.

  _Solution extraction._ Given a kernel $S$ of $G$, define the Boolean assignment $alpha$ by $alpha(u_i) = "true"$ if $x_i in S$ and $alpha(u_i) = "false"$ if $overline(x)_i in S$.
]

*Overhead.*
#table(
  columns: (auto, auto),
  [*Target metric*], [*Formula*],
  [`num_vertices`], [$2 dot n + 3 dot m$],
  [`num_arcs`], [$2 dot n + 12 dot m$],
)
where $n$ = `num_vars` and $m$ = `num_clauses` of the source 3-SAT instance.

*Feasible example.*
Consider a 3-SAT instance with $n = 3$ variables and $m = 2$ clauses:
$ phi = (u_1 or u_2 or u_3) and (not u_1 or not u_2 or u_3) $

The reduction constructs a directed graph with $2 dot 3 + 3 dot 2 = 12$ vertices and $2 dot 3 + 12 dot 2 = 30$ arcs.

Vertices: $x_1, overline(x)_1, x_2, overline(x)_2, x_3, overline(x)_3$ (literal vertices, indices 0--5) and $c_(1,1), c_(1,2), c_(1,3), c_(2,1), c_(2,2), c_(2,3)$ (clause vertices, indices 6--11).

Variable digon arcs: $(x_1, overline(x)_1), (overline(x)_1, x_1), (x_2, overline(x)_2), (overline(x)_2, x_2), (x_3, overline(x)_3), (overline(x)_3, x_3)$.

Clause 1 triangle: $(c_(1,1), c_(1,2)), (c_(1,2), c_(1,3)), (c_(1,3), c_(1,1))$.

Clause 1 connections ($u_1 or u_2 or u_3$, literal vertices $x_1, x_2, x_3$):
$(c_(1,1), x_1), (c_(1,2), x_1), (c_(1,3), x_1)$,
$(c_(1,1), x_2), (c_(1,2), x_2), (c_(1,3), x_2)$,
$(c_(1,1), x_3), (c_(1,2), x_3), (c_(1,3), x_3)$.

Clause 2 triangle: $(c_(2,1), c_(2,2)), (c_(2,2), c_(2,3)), (c_(2,3), c_(2,1))$.

Clause 2 connections ($not u_1 or not u_2 or u_3$, literal vertices $overline(x)_1, overline(x)_2, x_3$):
$(c_(2,1), overline(x)_1), (c_(2,2), overline(x)_1), (c_(2,3), overline(x)_1)$,
$(c_(2,1), overline(x)_2), (c_(2,2), overline(x)_2), (c_(2,3), overline(x)_2)$,
$(c_(2,1), x_3), (c_(2,2), x_3), (c_(2,3), x_3)$.

The satisfying assignment $alpha(u_1) = "true", alpha(u_2) = "false", alpha(u_3) = "true"$ yields kernel $S = {x_1, overline(x)_2, x_3}$ (indices ${0, 3, 4}$).

Verification:
- Independence: no arc between vertices 0, 3, 4. Digon arcs connect $(0,1), (2,3), (4,5)$; none link two members of $S$.
- Absorption of $overline(x)_1$ (index 1): arc $(1, 0)$, and $0 in S$. Absorbed.
- Absorption of $x_2$ (index 2): arc $(2, 3)$, and $3 in S$. Absorbed.
- Absorption of $overline(x)_3$ (index 5): arc $(5, 4)$, and $4 in S$. Absorbed.
- Absorption of $c_(1,t)$ ($t = 1, 2, 3$): each has arc to $x_1$ (index 0) $in S$. Absorbed.
- Absorption of $c_(2,t)$ ($t = 1, 2, 3$): each has arc to $overline(x)_2$ (index 3) $in S$ and to $x_3$ (index 4) $in S$. Absorbed.

*Infeasible example.*
Consider a 3-SAT instance with $n = 3$ variables and $m = 8$ clauses comprising all $2^3 = 8$ sign patterns on 3 variables:
$ phi = (u_1 or u_2 or u_3) and (u_1 or u_2 or not u_3) and (u_1 or not u_2 or u_3) and (u_1 or not u_2 or not u_3) $
$ and (not u_1 or u_2 or u_3) and (not u_1 or u_2 or not u_3) and (not u_1 or not u_2 or u_3) and (not u_1 or not u_2 or not u_3) $

This formula is unsatisfiable because each of the $2^3 = 8$ possible truth assignments falsifies exactly one clause. For any assignment $alpha$, the clause whose literals are all negations of $alpha$ is falsified: if $alpha = (T, T, T)$ then clause 8 ($(not u_1 or not u_2 or not u_3)$) is false; if $alpha = (F, F, F)$ then clause 1 ($(u_1 or u_2 or u_3)$) is false; and so on for each of the 8 assignments.

The reduction constructs a directed graph with $2 dot 3 + 3 dot 8 = 30$ vertices and $2 dot 3 + 12 dot 8 = 102$ arcs.

In any kernel $S$ of $G$, exactly one of ${x_i, overline(x)_i}$ is selected for each $i$, corresponding to a truth assignment (as proved above). The clause gadgets enforce that each clause is satisfied. Since no satisfying assignment exists for this formula, $G$ has no kernel.

Explicit check for $alpha = (T, T, T)$: kernel candidate $S = {x_1, x_2, x_3}$ (indices ${0, 2, 4}$). Clause 8 is $(not u_1 or not u_2 or not u_3)$ with literal vertices $overline(x)_1, overline(x)_2, overline(x)_3$ (indices 1, 3, 5). The first clause-8 vertex $c_(8,1)$ (index 27) has outgoing arcs to $c_(8,2)$ (index 28, not in $S$) and to vertices 1, 3, 5 (none in $S$). Thus $c_(8,1)$ is not absorbed, so $S$ is not a kernel.
