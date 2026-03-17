---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SATISFIABILITY to UNDIRECTED FLOW WITH LOWER BOUNDS"
labels: rule
assignees: ''
canonical_source_name: 'Satisfiability (SAT)'
canonical_target_name: 'Undirected Flow with Lower Bounds'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** SATISFIABILITY
**Target:** UNDIRECTED FLOW WITH LOWER BOUNDS
**Motivation:** Establishes NP-completeness of UNDIRECTED FLOW WITH LOWER BOUNDS via polynomial-time reduction from SATISFIABILITY. This is notable because directed flow with lower bounds is polynomial-time solvable, while the undirected variant with lower bounds is NP-complete even for a single commodity, even allowing non-integral flows.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND37, p.216

## GJ Source Entry

> [ND37] UNDIRECTED FLOW WITH LOWER BOUNDS
> INSTANCE: Graph G=(V,E), specified vertices s and t, capacity c(e)∈Z^+ and lower bound l(e)∈Z_0^+ for each e∈E, requirement R∈Z^+.
> QUESTION: Is there a flow function f: {(u,v),(v,u): {u,v}∈E}→Z_0^+ such that
> (1) for all {u,v}∈E, either f((u,v))=0 or f((v,u))=0,
> (2) for each e={u,v}∈E, l(e)≤max{f((u,v)),f((v,u))}≤c(e),
> (3) for each v∈V−{s,t}, flow is conserved at v, and
> (4) the net flow into t is at least R?
> Reference: [Itai, 1977]. Transformation from SATISFIABILITY.
> Comment: Problem is NP-complete in the strong sense, even if non-integral flows are allowed. Corresponding problem for directed graphs can be solved in polynomial time, even if we ask that the total flow be R or less rather than R or more [Ford and Fulkerson, 1962] (see also [Lawler, 1976a]). The analogous DIRECTED M-COMMODITY FLOW WITH LOWER BOUNDS problem is polynomially equivalent to LINEAR PROGRAMMING for all M≥2 if non-integral flows are allowed [Itai, 1977].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a SATISFIABILITY instance with n variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct an UNDIRECTED FLOW WITH LOWER BOUNDS instance as follows:

1. **Variable gadgets:** For each variable x_i, create an undirected "choice" subgraph. Two parallel edges connect node u_i to node v_i: edge e_i^T (representing x_i = TRUE) and edge e_i^F (representing x_i = FALSE). Set lower bound l = 0 and capacity c = 1 on each.

2. **Chain the variable gadgets:** Connect s to u_1, v_1 to u_2, ..., v_n to a junction node. This forces exactly one unit of flow through each variable gadget, choosing either the TRUE or FALSE edge.

3. **Clause gadgets:** For each clause C_j, introduce additional edges that must carry flow (enforced by nonzero lower bounds). The lower bound on a clause edge forces at least one unit of flow, which can only be routed if at least one literal in the clause is satisfied.

4. **Literal connections:** For each literal in a clause, add edges connecting the clause gadget to the appropriate variable gadget edge. If literal x_i appears in clause C_j, connect to the TRUE side; if ¬x_i appears, connect to the FALSE side. The lower bound on the clause edge forces flow through at least one satisfied literal path.

5. **Lower bounds enforce clause satisfaction:** Each clause edge e_{C_j} has lower bound l(e_{C_j}) = 1, meaning at least one unit of flow must traverse it. This flow can only be routed if the corresponding literal's variable assignment allows it.

6. **Requirement:** Set R appropriately (n + m or similar) to ensure both the assignment path and all clause flows are realized.

The SAT formula is satisfiable if and only if there exists a feasible flow meeting all lower bounds and the requirement R. The key insight is that undirected flow with lower bounds is hard because the lower bound constraints interact nontrivially with the undirected flow conservation, unlike in directed graphs where standard max-flow/min-cut techniques handle lower bounds.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(n + m) where n = num_variables, m = num_clauses |
| `num_edges` | O(n + m + L) where L = total literal occurrences |
| `max_capacity` | O(m) |
| `requirement` | O(n + m) |

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce source SAT instance, solve target undirected flow with lower bounds using BruteForce, extract solution, verify on source
- Compare with known results from literature
- Verify that satisfiable SAT instances yield feasible flow and unsatisfiable instances do not

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source (SAT):**
Variables: x_1, x_2, x_3, x_4
Clauses:
- C_1 = (x_1 ∨ ¬x_2 ∨ x_3)
- C_2 = (¬x_1 ∨ x_2 ∨ x_4)
- C_3 = (¬x_3 ∨ ¬x_4 ∨ x_1)

**Constructed Target (Undirected Flow with Lower Bounds):**

Vertices: s, u_1, v_1, u_2, v_2, u_3, v_3, u_4, v_4, t, clause nodes c_1, c_2, c_3, and auxiliary routing nodes.

Edges:
- Variable chain: {s, u_1}, {u_1, v_1} (TRUE path for x_1), {u_1, v_1} (FALSE path for x_1), {v_1, u_2}, ..., {v_4, t}.
- Clause edges with lower bounds: {c_j_in, c_j_out} with l = 1, c = 1 for each clause.
- Literal connection edges linking clause gadgets to variable gadgets.

Lower bounds: 0 on variable edges, 1 on clause enforcement edges.
Capacities: 1 on all edges.
Requirement R = 4 + 3 = 7.

**Solution mapping:**
Assignment x_1=TRUE, x_2=TRUE, x_3=TRUE, x_4=TRUE satisfies all clauses.
- C_1 satisfied by x_1=TRUE: flow routed through x_1's TRUE edge to clause C_1.
- C_2 satisfied by x_2=TRUE: flow routed through x_2's TRUE edge to clause C_2.
- C_3 satisfied by x_1=TRUE: flow routed through x_1's TRUE edge to clause C_3.
- Total flow: 4 (variable chain) + 3 (clause flows) = 7 = R.


## References

- **[Itai, 1977]**: [`Itai1977a`] Alon Itai (1977). "Two commodity flow". Dept. of Computer Science, Technion.
- **[Ford and Fulkerson, 1962]**: [`Ford1962`] L. R. Ford and D. R. Fulkerson (1962). "Flows in Networks". Princeton University Press, Princeton, NJ.
- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.
