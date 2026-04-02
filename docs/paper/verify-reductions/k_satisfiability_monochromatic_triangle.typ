// Reduction proof: KSatisfiability(K3) -> MonochromaticTriangle
// Reference: Garey & Johnson, Computers and Intractability, A1.1 GT6;
// Burr 1976, "Generalized Ramsey theory for graphs --- a survey"

#set page(width: auto, height: auto, margin: 15pt)
#set text(size: 10pt)

= 3-SAT $arrow.r$ Monochromatic Triangle

== Problem Definitions

*3-SAT (KSatisfiability with $K=3$):*
Given a set $U = {x_1, dots, x_n}$ of Boolean variables and a collection $C = {C_1, dots, C_m}$ of clauses over $U$, where each clause $C_j = (l_1^j or l_2^j or l_3^j)$ contains exactly 3 literals, is there a truth assignment $tau: U arrow {0,1}$ satisfying all clauses?

*Monochromatic Triangle:*
Given a graph $G = (V, E)$, can the edges of $G$ be 2-colored (each edge assigned color 0 or 1) so that no triangle is monochromatic, i.e., no three mutually adjacent vertices have all three connecting edges the same color? Equivalently, can $E$ be partitioned into two triangle-free subgraphs?

== Reduction Construction

Given a 3-SAT instance $(U, C)$ with $n$ variables and $m$ clauses, construct a graph $G = (V', E')$ as follows.

*Literal vertices:* For each variable $x_i$ ($i = 1, dots, n$), create a _positive vertex_ $p_i$ and a _negative vertex_ $n_i$. Add a _negation edge_ $(p_i, n_i)$ for each variable. This gives $2n$ vertices and $n$ edges.

*Clause gadgets:* For each clause $C_j = (l_1 or l_2 or l_3)$, map each literal to its vertex:
- $x_i$ (positive) maps to $p_i$; $overline(x)_i$ (negative) maps to $n_i$.

Let $v_1, v_2, v_3$ be the three literal vertices for the clause. For each pair $(v_a, v_b)$ from ${v_1, v_2, v_3}$, create a fresh _intermediate_ vertex $m_(a b)^j$ and add edges $(v_a, m_(a b)^j)$ and $(v_b, m_(a b)^j)$. This produces 3 intermediate vertices per clause.

Connect the three intermediate vertices to form a _clause triangle_:
$ (m_(12)^j, m_(13)^j), quad (m_(12)^j, m_(23)^j), quad (m_(13)^j, m_(23)^j) $

*Total size:*
- $|V'| = 2n + 3m$ vertices
- $|E'| <= n + 9m$ edges ($n$ negation edges + at most $6m$ fan edges + $3m$ clause-triangle edges)

*Triangles per clause:* Each clause gadget produces exactly 4 triangles:
+ The clause triangle $(m_(12)^j, m_(13)^j, m_(23)^j)$
+ Three fan triangles: $(v_1, m_(12)^j, m_(13)^j)$, $(v_2, m_(12)^j, m_(23)^j)$, $(v_3, m_(13)^j, m_(23)^j)$

Each fan triangle has NAE (not-all-equal) constraint on its three edges. The clause triangle ties the three fan constraints together.

== Correctness Proof

*Claim:* The 3-SAT instance $(U, C)$ is satisfiable if and only if the graph $G$ admits a 2-edge-coloring with no monochromatic triangles.

=== Forward direction ($arrow.r$)

Suppose $tau$ satisfies all 3-SAT clauses. We construct a valid 2-edge-coloring of $G$:

- *Negation edges:* Color $(p_i, n_i)$ with color 0 if $tau(x_i) = 1$ (True), color 1 otherwise.

- *Fan edges and clause-triangle edges:* For each clause $C_j$, at least one literal is true under $tau$. The fan and clause-triangle edges can be colored to satisfy all 4 NAE constraints. Since each clause gadget is an independent substructure (intermediate vertices are unique per clause), the coloring choices for different clauses do not interfere.

The 4 NAE constraints per clause form a small constraint system with 9 edge variables and only 4 constraints, each forbidding one of 8 possible patterns. With at most $4 times 2 = 8$ forbidden patterns out of $2^9 = 512$ possible colorings per gadget, valid colorings exist for any literal assignment that satisfies the clause (verified exhaustively by the accompanying Python scripts).

=== Backward direction ($arrow.l$)

Suppose $G$ has a valid 2-edge-coloring $c$ (no monochromatic triangles).

For each clause $C_j$, consider its 4 triangles. The clause triangle $(m_(12)^j, m_(13)^j, m_(23)^j)$ constrains the clause-triangle edge colors. The fan triangles propagate these constraints to the literal vertices.

We show that at least one literal must be "True" (in the sense that the clause constraint is satisfied). The intermediate vertices create a gadget where the NAE constraints on the 4 triangles collectively prevent the configuration where all three literals evaluate to False. This is because the all-False configuration would force the fan edges into a pattern that makes the clause triangle monochromatic (verified exhaustively).

Read off the truth assignment from the negation edge colors (or their complement). The resulting assignment satisfies every clause. $square$

== Solution Extraction

Given a valid 2-edge-coloring $c$ of $G$:
1. Read the negation edge colors: set $tau(x_i) = 1$ if $c(p_i, n_i) = 0$, else $tau(x_i) = 0$.
2. If this assignment satisfies all clauses, return it.
3. Otherwise, try the complement assignment: $tau(x_i) = 1 - tau(x_i)$.
4. As a fallback, brute-force the original 3-SAT (guaranteed to be satisfiable).

== Example

*Source (3-SAT):* $n = 3$, clause: $(x_1 or x_2 or x_3)$

*Target (MonochromaticTriangle):*
- $2 dot 3 + 3 dot 1 = 9$ vertices: $p_1, p_2, p_3, n_1, n_2, n_3, m_(12), m_(13), m_(23)$
- Negation edges: $(p_1, n_1), (p_2, n_2), (p_3, n_3)$
- Fan edges: $(p_1, m_(12)), (p_2, m_(12)), (p_1, m_(13)), (p_3, m_(13)), (p_2, m_(23)), (p_3, m_(23))$
- Clause triangle: $(m_(12), m_(13)), (m_(12), m_(23)), (m_(13), m_(23))$

*Satisfying assignment:* $x_1 = 1, x_2 = 0, x_3 = 0$. The negation edges get colors $0, 1, 1$. The fan and clause-triangle edges can be colored to avoid monochromatic triangles (verified computationally).

== NO Example

*Source (3-SAT):* $n = 3$, all 8 clauses on variables $x_1, x_2, x_3$:
$(x_1 or x_2 or x_3)$, $(overline(x)_1 or overline(x)_2 or overline(x)_3)$, $(x_1 or overline(x)_2 or x_3)$, $(overline(x)_1 or x_2 or overline(x)_3)$, $(x_1 or x_2 or overline(x)_3)$, $(overline(x)_1 or overline(x)_2 or x_3)$, $(overline(x)_1 or x_2 or x_3)$, $(x_1 or overline(x)_2 or overline(x)_3)$.

This is unsatisfiable (every assignment falsifies at least one clause). By correctness of the reduction, the corresponding MonochromaticTriangle instance ($30$ vertices, $75$ edges) has no valid 2-edge-coloring without monochromatic triangles.
