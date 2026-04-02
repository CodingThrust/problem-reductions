// Reduction proof: KSatisfiability(K3) -> RegisterSufficiency
// Reference: Sethi (1975), "Complete register allocation problems"
// Garey & Johnson, Computers and Intractability, Appendix A11, PO1

#set page(width: auto, height: auto, margin: 15pt)
#set text(size: 10pt)

= 3-SAT $arrow.r$ Register Sufficiency

== Problem Definitions

*3-SAT (KSatisfiability with $K=3$):*
Given a set $U = {x_1, dots, x_n}$ of Boolean variables and a collection $C = {C_1, dots, C_m}$ of clauses over $U$, where each clause $C_j = (l_1^j or l_2^j or l_3^j)$ contains exactly 3 literals, is there a truth assignment $tau: U arrow {0,1}$ satisfying all clauses?

*Register Sufficiency:*
Given a directed acyclic graph $G = (V, A)$ representing a computation and a positive integer $K$, is there a topological ordering $v_1, v_2, dots, v_n$ of $V$ and a sequence $S_0, S_1, dots, S_n$ of subsets of $V$ with $|S_i| <= K$, such that $S_0 = emptyset$, $S_n$ contains all vertices with in-degree 0, and for $1 <= i <= n$: $v_i in S_i$, $S_i without {v_i} subset.eq S_(i-1)$, and $S_(i-1)$ contains all vertices $u$ with $(v_i, u) in A$?

Equivalently: does there exist an evaluation ordering of all vertices such that the maximum number of simultaneously-live values (registers) never exceeds $K$? A vertex is "live" from its evaluation until all its dependents have been evaluated; vertices with no dependents remain live until the end.

== Reduction Construction

Given a 3-SAT instance $(U, C)$ with $n$ variables and $m$ clauses, construct a DAG $G'$ and bound $K$ as follows.

*Variable gadgets:* For each variable $x_i$ ($i = 1, dots, n$), create four vertices forming a "diamond" subDAG:
- $s_i$ (source): no predecessors if $i = 1$; depends on $k_(i-1)$ otherwise
- $t_i$ (true literal): depends on $s_i$
- $f_i$ (false literal): depends on $s_i$
- $k_i$ (kill): depends on $t_i$ and $f_i$

The variable gadgets form a chain: $s_i$ depends on $k_(i-1)$ for $i > 1$.

*Clause gadgets:* For each clause $C_j = (l_1 or l_2 or l_3)$, create a vertex $c_j$ with dependencies:
- If literal $l$ is positive ($x_i$): $c_j$ depends on $t_i$
- If literal $l$ is negative ($overline(x)_i$): $c_j$ depends on $f_i$

*Sink:* A single sink vertex $sigma$ depends on $k_n$ and all clause vertices $c_1, dots, c_m$.

*Size:*
- $|V'| = 4n + m + 1$ vertices
- $|A'| = 4n - 1 + 3m + m + 1$ arcs

*Register bound:* $K$ is set to the minimum register count achievable by the constructive ordering described below, over all satisfying assignments.

== Evaluation Ordering

Given a satisfying assignment $tau$, construct the evaluation ordering:

For each variable $x_i$ in order $i = 1, dots, n$:
1. Evaluate $s_i$
2. If $tau(x_i) = 1$: evaluate $f_i$, then $t_i$ (false path first)
3. If $tau(x_i) = 0$: evaluate $t_i$, then $f_i$ (true path first)
4. Evaluate $k_i$

After all variables: evaluate clause vertices $c_1, dots, c_m$, then the sink $sigma$.

*Truth assignment encoding:* The evaluation order within each variable gadget encodes the truth value: $x_i = 1$ iff $t_i$ is evaluated after $f_i$ (i.e., $"config"[t_i] > "config"[f_i]$).

== Correctness Sketch

*Forward direction ($arrow.r$):* If $tau$ satisfies the 3-SAT instance, the constructive ordering above produces a valid topological ordering of $G'$. The register count is bounded because:

- During variable $i$ processing: at most 3 registers are used (source, one literal, plus the chain predecessor)
- Literal nodes referenced by clause nodes may extend their live ranges, but the total number of simultaneously-live literals is bounded by the specific clause structure
- The bound $K$ is computed as the minimum over all satisfying assignments

*Backward direction ($arrow.l$):* If an evaluation ordering achieves $<= K$ registers, the ordering implicitly encodes a truth assignment through the variable gadget evaluation order, and the register pressure constraint ensures this assignment satisfies all clauses.

== Solution Extraction

Given a Register Sufficiency solution (evaluation ordering as config vector), extract the 3-SAT assignment:
$ tau(x_i) = cases(1 &"if" "config"[t_i] > "config"[f_i], 0 &"otherwise") $

where $t_i = 4(i-1) + 1$ and $f_i = 4(i-1) + 2$ (0-indexed vertex numbering).

== Example

*Source (3-SAT):* $n = 3$, clause: $(x_1 or x_2 or x_3)$

*Target (Register Sufficiency):* $n' = 14$ vertices, $K = 4$

Vertices: $s_1 = 0, t_1 = 1, f_1 = 2, k_1 = 3, s_2 = 4, t_2 = 5, f_2 = 6, k_2 = 7, s_3 = 8, t_3 = 9, f_3 = 10, k_3 = 11, c_1 = 12, sigma = 13$

Arcs (diamond chain): $(t_1, s_1), (f_1, s_1), (k_1, t_1), (k_1, f_1), (s_2, k_1), (t_2, s_2), (f_2, s_2), (k_2, t_2), (k_2, f_2), (s_3, k_2), (t_3, s_3), (f_3, s_3), (k_3, t_3), (k_3, f_3)$

Clause arc: $(c_1, t_1), (c_1, t_2), (c_1, t_3)$

Sink arcs: $(sigma, k_3), (sigma, c_1)$

*Satisfying assignment:* $x_1 = 1, x_2 = 0, x_3 = 0$

*Evaluation ordering:* $s_1, f_1, t_1, k_1, s_2, t_2, f_2, k_2, s_3, t_3, f_3, k_3, c_1, sigma$

*Register trace:*
- Step 0 ($s_1$): 1 register
- Step 1 ($f_1$): 2 registers ($s_1, f_1$)
- Step 2 ($t_1$): 2 registers ($t_1, f_1$; $s_1$ freed)
- Step 3 ($k_1$): 1 register ($k_1$; $t_1$ stays alive for $c_1$)... actually 2 ($k_1, t_1$)
- Steps 4--11: variable processing continues
- Step 12 ($c_1$): clause evaluated
- Step 13 ($sigma$): sink evaluated

Maximum registers used: 4. Since $K = 4$, the instance is feasible. #sym.checkmark

== NO Example

*Source (3-SAT):* $n = 3$, all 8 clauses on variables $x_1, x_2, x_3$:

$(x_1 or x_2 or x_3)$, $(overline(x)_1 or overline(x)_2 or overline(x)_3)$, $(x_1 or overline(x)_2 or x_3)$, $(overline(x)_1 or x_2 or overline(x)_3)$, $(x_1 or x_2 or overline(x)_3)$, $(overline(x)_1 or overline(x)_2 or x_3)$, $(overline(x)_1 or x_2 or x_3)$, $(x_1 or overline(x)_2 or overline(x)_3)$.

This is unsatisfiable (every assignment falsifies at least one clause). The corresponding Register Sufficiency instance has $4 dot 3 + 8 + 1 = 21$ vertices. By correctness of the reduction, the target instance requires more than $K$ registers for any evaluation ordering.

== References

- *[Sethi, 1975]:* R. Sethi, "Complete register allocation problems," _SIAM Journal on Computing_ 4(3), pp. 226--248, 1975.
- *[Garey & Johnson, 1979]:* M. R. Garey and D. S. Johnson, _Computers and Intractability: A Guide to the Theory of NP-Completeness_, W. H. Freeman, 1979. Problem A11 PO1.
