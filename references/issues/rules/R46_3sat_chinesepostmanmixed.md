---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CHINESE POSTMAN FOR MIXED GRAPHS"
labels: rule
assignees: ''
canonical_source_name: '3-SATISFIABILITY'
canonical_target_name: 'CHINESE POSTMAN FOR MIXED GRAPHS'
source_in_codebase: true
target_in_codebase: false
---

**Source:** 3SAT
**Target:** CHINESE POSTMAN FOR MIXED GRAPHS
**Motivation:** Establishes NP-completeness of the CHINESE POSTMAN FOR MIXED GRAPHS (MCPP) via polynomial-time reduction from 3SAT. This is a landmark result by Papadimitriou (1976) showing that while the Chinese Postman Problem is solvable in polynomial time for purely undirected graphs (via T-join/matching) and for purely directed graphs (via min-cost flow), the mixed case where both directed arcs and undirected edges coexist is NP-complete. The result holds even when the graph is planar, has maximum degree 3, and all edge/arc lengths are 1.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND25, p.212

## GJ Source Entry

> [ND25] CHINESE POSTMAN FOR MIXED GRAPHS
> INSTANCE: Mixed graph G=(V,A,E), where A is a set of directed edges and E is a set of undirected edges on V, length l(e)∈Z_0^+ for each e∈A∪E, bound B∈Z^+.
> QUESTION: Is there a cycle in G that includes each directed and undirected edge at least once, traversing directed edges only in the specified direction, and that has total length no more than B?
> Reference: [Papadimitriou, 1976b]. Transformation from 3SAT.
> Comment: Remains NP-complete even if all edge lengths are equal, G is planar, and the maximum vertex degree is 3. Can be solved in polynomial time if either A or E is empty (i.e., if G is either a directed or an undirected graph) [Edmonds and Johnson, 1973].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance with n variables x_1, ..., x_n and m clauses C_1, ..., C_m, construct a mixed graph G = (V, A, E) with unit edge/arc lengths as follows (per Papadimitriou, 1976):

1. **Variable gadgets:** For each variable x_i, construct a gadget consisting of a cycle that can be traversed in two ways — one corresponding to x_i = TRUE and the other to x_i = FALSE. The gadget uses a mix of directed arcs and undirected edges such that:
   - The undirected edges can be traversed in either direction, representing the two truth assignments.
   - The directed arcs enforce that once a direction is chosen for the undirected edges (to form an Euler tour through the gadget), it must be consistent throughout the entire variable gadget.
   - Each variable gadget has "ports" — one for each occurrence of x_i or ¬x_i in the clauses.

2. **Clause gadgets:** For each clause C_j = (l_{j1} ∨ l_{j2} ∨ l_{j3}), construct a small subgraph that is connected to the three variable gadgets corresponding to the literals l_{j1}, l_{j2}, l_{j3}. The clause gadget is designed so that:
   - It can be traversed at minimum cost if and only if at least one of the three connected variable gadgets is set to the truth value that satisfies the literal.
   - If none of the three literals is satisfied, the clause gadget requires at least one extra edge traversal (increasing the total cost beyond the bound).

3. **Connections:** The variable gadgets and clause gadgets are connected via edges at the "ports." The direction chosen for traversing the variable gadget's undirected edges determines which literal connections can be used for "free" (without extra traversals).

4. **Edge/arc lengths:** All edges and arcs have length 1 (unit lengths). The construction works even in this restricted setting.

5. **Bound B:** Set B equal to the total number of arcs and edges in the constructed graph (i.e., the minimum possible traversal cost if the graph were Eulerian or could be made Eulerian with no extra traversals). The mixed graph is constructed so that a postman tour of cost exactly B exists if and only if the 3SAT formula is satisfiable.

6. **Correctness:**
   - **(Forward):** If the 3SAT instance is satisfiable, set each variable gadget's traversal direction according to the satisfying assignment. For each clause, at least one literal is satisfied, allowing the clause gadget to be traversed without extra cost. The total traversal cost equals B.
   - **(Reverse):** If a postman tour of cost ≤ B exists, the traversal directions of the variable gadgets encode a consistent truth assignment (due to the directed arcs enforcing consistency). Since the cost is at most B, no clause gadget requires extra traversals, meaning each clause has at least one satisfied literal.

**Key invariant:** The interplay between directed arcs (enforcing consistency of truth assignment) and undirected edges (allowing choice of traversal direction) encodes the 3SAT structure. The bound B is tight: it equals the minimum possible tour length when all clauses are satisfied.

**Construction size:** The mixed graph has O(n + m) vertices and O(n + m) edges/arcs (polynomial in the input size).

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_variables` of source 3SAT instance
- m = `num_clauses` of source 3SAT instance
- L = total number of literal occurrences across all clauses (≤ 3m)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | O(n + m) — linear in the formula size |
| `num_arcs` | O(L + n) — arcs in variable gadgets plus connections |
| `num_edges` | O(L + n) — undirected edges in variable and clause gadgets |
| `bound` | `num_arcs + num_edges` (unit-length case) |

**Derivation:** Each variable gadget contributes O(degree(x_i)) vertices and edges/arcs, where degree is the number of clause occurrences. Each clause gadget adds O(1) vertices and edges. The total is O(sum of degrees + m) = O(L + m) = O(L) since L ≥ m. With unit lengths, B = |A| + |E| (traverse each exactly once if possible).

**Note:** The exact constants depend on the specific gadget design from Papadimitriou (1976). The construction in the original paper achieves planarity and max degree 3, which constrains the gadget design.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce a small 3SAT instance to MCPP, enumerate all possible Euler tours or postman tours on the mixed graph, verify that a tour of cost ≤ B exists iff the formula is satisfiable.
- Test with a known satisfiable instance: (x_1 ∨ x_2 ∨ x_3) with the trivial satisfying assignment x_1 = TRUE. The MCPP instance should have a postman tour of cost B.
- Test with a known unsatisfiable instance: (x_1 ∨ x_2) ∧ (¬x_1 ∨ ¬x_2) ∧ (x_1 ∨ ¬x_2) ∧ (¬x_1 ∨ x_2) — unsatisfiable (requires x_1 = x_2 = TRUE and x_1 = x_2 = FALSE simultaneously). Pad to 3SAT and verify no tour of cost ≤ B exists.
- Verify graph properties: planarity, max degree 3 (if using the restricted construction), unit lengths.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
3 variables {x_1, x_2, x_3} and 3 clauses:
- C_1 = (x_1 ∨ ¬x_2 ∨ x_3)
- C_2 = (¬x_1 ∨ x_2 ∨ ¬x_3)
- C_3 = (x_1 ∨ x_2 ∨ x_3)
- Satisfying assignment: x_1 = TRUE, x_2 = TRUE, x_3 = TRUE (satisfies C_1 via x_1, C_2 via x_2, C_3 via all three)

**Constructed target instance (ChinesePostmanForMixedGraphs) — schematic:**
Mixed graph G = (V, A, E) with unit lengths:

*Variable gadgets (schematic for x_1 with 2 occurrences as positive literal, 1 as negative):*
- Vertices: v_{1,1}, v_{1,2}, v_{1,3}, v_{1,4}, v_{1,5}, v_{1,6}
- Arcs (directed): (v_{1,1} → v_{1,2}), (v_{1,3} → v_{1,4}), (v_{1,5} → v_{1,6}) — enforce consistency
- Edges (undirected): {v_{1,2}, v_{1,3}}, {v_{1,4}, v_{1,5}}, {v_{1,6}, v_{1,1}} — allow choice of direction
- Traversing undirected edges "clockwise" encodes x_1 = TRUE; "counterclockwise" encodes x_1 = FALSE.
- Port vertices connect to clause gadgets: v_{1,2} links to C_1 (positive), v_{1,4} links to C_3 (positive), v_{1,6} links to C_2 (negative).

*Clause gadgets (schematic for C_1 = (x_1 ∨ ¬x_2 ∨ x_3)):*
- Small subgraph with 3 connection vertices, one per literal port.
- If at least one literal's variable gadget is traversed in the "satisfying" direction, the clause gadget can be Euler-toured at base cost. Otherwise, an extra traversal (cost +1) is forced.

*Total construction:*
- Approximately 6×3 = 18 vertices for variable gadgets + 3×O(1) vertices for clause gadgets ≈ 24 vertices
- Approximately 9 arcs + 9 edges for variable gadgets + clause connections ≈ 30 arcs/edges total
- Bound B = 30 (one traversal per arc/edge)

**Solution mapping:**
- Satisfying assignment: x_1 = T, x_2 = T, x_3 = T
- Variable gadget x_1: traverse undirected edges clockwise → encodes TRUE
- Variable gadget x_2: traverse undirected edges clockwise → encodes TRUE
- Variable gadget x_3: traverse undirected edges clockwise → encodes TRUE
- Each clause gadget has at least one satisfied literal → no extra traversals needed
- Postman tour cost = B = 30 ✓
- Answer: YES (satisfiable ↔ postman tour within bound)


## References

- **[Papadimitriou, 1976b]**: [`Papadimitriou1976b`] Christos H. Papadimitriou (1976). "On the complexity of edge traversing". *Journal of the Association for Computing Machinery* 23, pp. 544–554.
- **[Edmonds and Johnson, 1973]**: [`Edmonds1973`] J. Edmonds and E. L. Johnson (1973). "Matching, {Euler} tours, and the {Chinese} postman". *Mathematical Programming* 5, pp. 88–124.
