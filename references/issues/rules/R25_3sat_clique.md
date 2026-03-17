---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CLIQUE"
labels: rule
assignees: ''
canonical_source_name: '3-Satisfiability (3SAT)'
canonical_target_name: 'Clique'
source_in_codebase: true
target_in_codebase: true
---

**Source:** 3SAT
**Target:** CLIQUE
**Motivation:** Establishes NP-completeness of CLIQUE via polynomial-time reduction from 3SAT. This is one of Karp's 21 NP-complete problems (1972) and one of the most widely taught reductions in computational complexity. The construction elegantly encodes satisfiability constraints into graph structure, demonstrating that finding large complete subgraphs is computationally as hard as solving Boolean satisfiability.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT19

## GJ Source Entry

> [GT19]  CLIQUE
> INSTANCE:  Graph G = (V,E), positive integer K <= |V|.
> QUESTION:  Does G contain a clique of size K or more, i.e., a subset V' <= V with |V'| >= K such that every two vertices in V' are joined by an edge in E?
>
> Reference:  [Karp, 1972]. Transformation from VERTEX COVER (see Chapter 3).
> Comment:  Solvable in polynomial time for graphs obeying any fixed degree bound d, for planar graphs, for edge graphs, for chordal graphs [Gavril, 1972], for comparability graphs [Even, Pnueli, and Lempel, 1972], for circle graphs [Gavril, 1973], and for circular arc graphs (given their representation as families of arcs) [Gavril, 1974a]. The variant in which, for a given r, 0 < r < 1, we are asked whether G contains a clique of size r|V| or more is NP-complete for any fixed value of r.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3SAT instance phi with k clauses C_1, C_2, ..., C_k, each clause C_i = (l_{i,1} OR l_{i,2} OR l_{i,3}) containing exactly 3 literals, construct a CLIQUE instance (G, k) as follows:

1. **Vertex construction:** For each clause C_i and each literal l_{i,j} in that clause, create a vertex v_{i,j}. This yields exactly 3k vertices, organized into k groups (triples) of 3, one triple per clause.

2. **Edge construction:** Connect two vertices v_{i,a} and v_{j,b} with an edge if and only if:
   - They belong to **different** clauses (i != j), AND
   - Their literals are **not contradictory** (l_{i,a} is not the negation of l_{j,b}).
   In other words, edges are absent only between vertices in the same triple and between vertices labeled with complementary literals (x and NOT x).

3. **Clique size parameter:** Set K = k (the number of clauses).

4. **Solution extraction:** Given a clique C of size k in G, construct a satisfying assignment by setting each literal corresponding to a vertex in C to TRUE. This is consistent because no two vertices in the clique have contradictory labels (such pairs have no edge, so cannot both be in a clique). Each clause is satisfied because the clique contains exactly one vertex from each triple.

**Correctness:**
- (Forward) If phi is satisfiable, pick one true literal per clause. The corresponding k vertices form a clique: they are from different triples (so edges exist between them) and their labels are consistent with a single assignment (so no contradictory pairs).
- (Backward) If G has a k-clique, no two vertices can come from the same triple (no intra-triple edges), so exactly one vertex per clause is selected. The labels on these vertices define a consistent truth assignment that satisfies all k clauses.

**Source:** Karp (1972), "Reducibility among combinatorial problems"; Sipser, *Introduction to the Theory of Computation*, Theorem 7.32; Garey & Johnson Chapter 3.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- k = number of clauses in the 3SAT formula
- n = number of variables in the 3SAT formula

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `3 * num_clauses` |
| `num_edges` | `9 * num_clauses * (num_clauses - 1) / 2 - num_complementary_pairs` |

**Derivation:** There are 3k vertices (3 per clause). The maximum number of edges is C(3k, 2) = 3k(3k-1)/2, but we remove: (a) intra-triple edges: 3 edges per triple times k triples = 3k edges, and (b) edges between contradictory literals across triples. The cross-triple pairs total 9 * C(k,2) = 9k(k-1)/2, and from those we subtract pairs with contradictory labels. In the worst case (all distinct variables), there are no contradictory pairs, giving 9k(k-1)/2 edges. In the worst case for contradictions, each variable appears in many clauses, but the number of contradictory inter-triple pairs is at most O(k^2).

**Tighter formula:** `num_edges = 9 * num_clauses * (num_clauses - 1) / 2 - contradictory_cross_pairs`, where contradictory_cross_pairs depends on the specific formula. Upper bound: `num_edges <= 9 * num_clauses * (num_clauses - 1) / 2`.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: construct a 3SAT instance, reduce to CLIQUE (G, k), solve the CLIQUE instance with BruteForce, extract the satisfying assignment from the clique vertices, verify it satisfies the original 3SAT formula.
- Forward test: generate a known-satisfiable 3SAT instance, verify the constructed graph contains a k-clique.
- Backward test: generate a known-unsatisfiable 3SAT instance, verify no k-clique exists in G.
- Size verification: check that the graph has exactly 3k vertices and that no intra-triple edges or contradictory-literal edges exist.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3SAT):**
Variables: x_1, x_2, x_3
Clauses (k = 3):
- C_1 = (x_1 OR NOT x_2 OR x_3)
- C_2 = (NOT x_1 OR x_2 OR NOT x_3)
- C_3 = (x_1 OR x_2 OR x_3)

**Constructed target instance (CLIQUE):**
Vertices (3k = 9):
- Triple 1 (C_1): v_{1,1}=x_1, v_{1,2}=NOT_x_2, v_{1,3}=x_3
- Triple 2 (C_2): v_{2,1}=NOT_x_1, v_{2,2}=x_2, v_{2,3}=NOT_x_3
- Triple 3 (C_3): v_{3,1}=x_1, v_{3,2}=x_2, v_{3,3}=x_3

Edges (connect cross-triple non-contradictory pairs):
- v_{1,1}(x_1) -- v_{2,2}(x_2): YES (different triples, not contradictory)
- v_{1,1}(x_1) -- v_{2,3}(NOT_x_3): YES
- v_{1,1}(x_1) -- v_{2,1}(NOT_x_1): NO (contradictory: x_1 vs NOT x_1)
- v_{1,1}(x_1) -- v_{3,1}(x_1): YES
- v_{1,1}(x_1) -- v_{3,2}(x_2): YES
- v_{1,1}(x_1) -- v_{3,3}(x_3): YES
- v_{1,2}(NOT_x_2) -- v_{2,1}(NOT_x_1): YES
- v_{1,2}(NOT_x_2) -- v_{2,2}(x_2): NO (contradictory: NOT x_2 vs x_2)
- v_{1,2}(NOT_x_2) -- v_{2,3}(NOT_x_3): YES
- v_{1,2}(NOT_x_2) -- v_{3,1}(x_1): YES
- v_{1,2}(NOT_x_2) -- v_{3,2}(x_2): NO (contradictory)
- v_{1,2}(NOT_x_2) -- v_{3,3}(x_3): YES
- v_{1,3}(x_3) -- v_{2,1}(NOT_x_1): YES
- v_{1,3}(x_3) -- v_{2,2}(x_2): YES
- v_{1,3}(x_3) -- v_{2,3}(NOT_x_3): NO (contradictory: x_3 vs NOT x_3)
- v_{1,3}(x_3) -- v_{3,1}(x_1): YES
- v_{1,3}(x_3) -- v_{3,2}(x_2): YES
- v_{1,3}(x_3) -- v_{3,3}(x_3): YES
- v_{2,1}(NOT_x_1) -- v_{3,1}(x_1): NO (contradictory)
- v_{2,1}(NOT_x_1) -- v_{3,2}(x_2): YES
- v_{2,1}(NOT_x_1) -- v_{3,3}(x_3): YES
- v_{2,2}(x_2) -- v_{3,1}(x_1): YES
- v_{2,2}(x_2) -- v_{3,2}(x_2): YES
- v_{2,2}(x_2) -- v_{3,3}(x_3): YES
- v_{2,3}(NOT_x_3) -- v_{3,1}(x_1): YES
- v_{2,3}(NOT_x_3) -- v_{3,2}(x_2): YES
- v_{2,3}(NOT_x_3) -- v_{3,3}(x_3): NO (contradictory)

Clique size: K = 3

Total edges: 18 (out of 27 possible cross-triple pairs, 5 contradictory pairs removed, plus 0 intra-triple edges = 9*3 - 9 = 18... Let's recount: 27 cross-triple pairs - 5 contradictory = 22 edges)

Corrected edge count: 27 - 5 = 22 edges.

**Solution mapping:**
- 3-clique in G: {v_{1,1}(x_1), v_{2,2}(x_2), v_{3,3}(x_3)}
  - Check edges: v_{1,1}--v_{2,2} (YES), v_{1,1}--v_{3,3} (YES), v_{2,2}--v_{3,3} (YES). Valid clique.
- Extracted assignment: x_1 = TRUE, x_2 = TRUE, x_3 = TRUE
- Verification against 3SAT:
  - C_1 = (TRUE OR NOT TRUE OR TRUE) = (T OR F OR T) = TRUE
  - C_2 = (NOT TRUE OR TRUE OR NOT TRUE) = (F OR T OR F) = TRUE
  - C_3 = (TRUE OR TRUE OR TRUE) = TRUE
- All clauses satisfied.

**Greedy trap:** A greedy approach might pick v_{1,2}(NOT_x_2) first since it has many edges, then v_{2,1}(NOT_x_1), but v_{1,2}--v_{2,1} is an edge, and continuing to triple 3, v_{3,3}(x_3) connects to both. However, {v_{1,2}, v_{2,1}, v_{3,3}} is also a valid 3-clique yielding assignment x_1=F, x_2=F, x_3=T which also satisfies the formula. The real trap would be in an unsatisfiable formula where no k-clique exists.


## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Gavril, 1972]**: [`Gavril1972`] F. Gavril (1972). "Algorithms for minimum coloring, maximum clique, minimum covering by cliques, and maximum independent set of a chordal graph". *SIAM Journal on Computing* 1, pp. 180-187.
- **[Even, Pnueli, and Lempel, 1972]**: [`Even1972`] S. Even and A. Pnueli and A. Lempel (1972). "Permutation graphs and transitive graphs". *Journal of the Association for Computing Machinery* 19, pp. 400-410.
- **[Gavril, 1973]**: [`Gavril1973`] F. Gavril (1973). "Algorithms for a maximum clique and a maximum independent set of a circle graph". *Networks* 3, pp. 261-273.
- **[Gavril, 1974a]**: [`Gavril1974a`] F. Gavril (1974). "Algorithms on circular-arc graphs". *Networks* 4, pp. 357-369.
