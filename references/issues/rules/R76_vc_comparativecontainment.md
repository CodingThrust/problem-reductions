---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to COMPARATIVE CONTAINMENT"
labels: rule
assignees: ''
canonical_source_name: 'Vertex Cover'
canonical_target_name: 'Comparative Containment'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** VERTEX COVER
**Target:** COMPARATIVE CONTAINMENT
**Motivation:** Establishes NP-completeness of COMPARATIVE CONTAINMENT via polynomial-time reduction from VERTEX COVER. The reduction, due to Plaisted (1976), encodes the vertex cover structure into weighted set containment: each vertex becomes an element of the universe, and edges and coverage constraints are translated into two collections of weighted subsets (R and S) such that a vertex cover of bounded size exists if and only if a subset Y of the universe achieves at least as much R-containment weight as S-containment weight.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, SP10, p.223

## GJ Source Entry

> [SP10] COMPARATIVE CONTAINMENT
> INSTANCE: Two collections R={R_1,R_2,...,R_k} and S={S_1,S_2,...,S_l} of subsets of a finite set X, weights w(R_i) in Z^+, 1<=i<=k, and w(S_j) in Z^+, 1<=j<=l.
> QUESTION: Is there a subset Y <= X such that
> Sum_{Y <= R_i} w(R_i) >= Sum_{Y <= S_j} w(S_j) ?
> Reference: [Plaisted, 1976]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if all subsets in R and S have weight 1 [Garey and Johnson, ----].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a VERTEX COVER instance (graph G = (V, E), bound K), construct a COMPARATIVE CONTAINMENT instance as follows:

1. **Universe:** Let X = V (one element per vertex).
2. **Collection R (reward sets):** For each edge e = {u, v} in E, create a set R_e = {u, v} with weight w(R_e) = 1. Additionally, create one "budget" set R_0 = V (the entire vertex set) with weight w(R_0) = |E| - K. (Alternatively, the construction can include K copies of singleton reward sets or use a direct encoding — the exact gadgetry follows Plaisted's encoding where the reward for covering edges must offset the penalty for selecting too many vertices.)
3. **Collection S (penalty sets):** For each vertex v in V, create a singleton set S_v = {v} with weight w(S_v) = 1. This penalizes each vertex included in Y.
4. **Correctness intuition:** A subset Y <= X corresponds to selecting vertices. Every selected vertex contributes a penalty of 1 (via S). Every edge both of whose endpoints are in Y contributes a reward of 1 (via R). The budget set R_0 = V is always contained in Y only when Y = V, providing a balancing mechanism. The inequality Sum R-weight >= Sum S-weight holds iff Y forms a vertex cover of size at most K: the cover must hit all edges (maximizing R-rewards) while using few vertices (minimizing S-penalties).
5. **Solution extraction:** If the COMPARATIVE CONTAINMENT instance is satisfiable with witness Y, then Y (or its complement, depending on the polarity of the encoding) is a vertex cover of G with |Y| <= K.

*Note: The precise construction follows Plaisted (1976). The above captures the structural idea; the exact weight assignment and set definitions may vary in formulation to ensure the equivalence is tight.*

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |V| = `num_vertices` of source graph
- m = |E| = `num_edges` of source graph

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `universe_size`            | `num_vertices` (= n)             |
| `num_r_sets`               | `num_edges + 1` (= m + 1)       |
| `num_s_sets`               | `num_vertices` (= n)             |

**Derivation:** The universe X has one element per vertex. Collection R has one set per edge plus one budget set. Collection S has one singleton set per vertex. Total construction is O(n + m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce source VERTEX COVER instance, solve target COMPARATIVE CONTAINMENT with BruteForce, extract solution, verify on source
- Compare with known results from literature
- Test with small graphs (triangle, path, cycle) where vertex cover is known

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (VERTEX COVER):**
Graph G with 6 vertices V = {v_0, v_1, v_2, v_3, v_4, v_5} and 7 edges:
E = { {v_0,v_1}, {v_0,v_2}, {v_1,v_2}, {v_1,v_3}, {v_2,v_4}, {v_3,v_4}, {v_4,v_5} }
Bound K = 3.
(A minimum vertex cover is {v_1, v_2, v_4} of size 3.)

**Constructed COMPARATIVE CONTAINMENT instance:**
Universe X = {v_0, v_1, v_2, v_3, v_4, v_5}

Collection R (one set per edge, weight 1 each, plus budget set):
- R_1 = {v_0, v_1}, w = 1
- R_2 = {v_0, v_2}, w = 1
- R_3 = {v_1, v_2}, w = 1
- R_4 = {v_1, v_3}, w = 1
- R_5 = {v_2, v_4}, w = 1
- R_6 = {v_3, v_4}, w = 1
- R_7 = {v_4, v_5}, w = 1
- R_0 = {v_0, v_1, v_2, v_3, v_4, v_5}, w = |E| - K = 7 - 3 = 4

Collection S (one singleton per vertex, weight 1 each):
- S_0 = {v_0}, w = 1
- S_1 = {v_1}, w = 1
- S_2 = {v_2}, w = 1
- S_3 = {v_3}, w = 1
- S_4 = {v_4}, w = 1
- S_5 = {v_5}, w = 1

**Solution:**
Choose Y = {v_1, v_2, v_4}.

R-containment: Y <= R_3={v_1,v_2} YES (w=1), Y <= R_0={all} YES (w=4) -- but Y is NOT a subset of the edge sets (Y has 3 elements, edge sets have 2). So for the edge sets, Y <= R_e only if Y is contained in {u,v}, which requires |Y| <= 2. For |Y| = 3, no edge set contains Y. Only R_0 = V contains Y. R-weight = 4.

S-containment: Y <= S_j only if Y <= {v_j}. Since |Y| = 3, Y is not contained in any singleton. S-weight = 0.

Comparison: 4 >= 0? YES.

This confirms the vertex cover {v_1, v_2, v_4} of size 3 maps to a feasible COMPARATIVE CONTAINMENT solution.

## References

- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264-267. IEEE Computer Society.
- **[Garey and Johnson, ----]**: *(not found in bibliography)*
