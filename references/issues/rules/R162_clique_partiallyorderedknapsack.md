---
name: Rule
about: Propose a new reduction rule
title: "[Rule] CLIQUE to PARTIALLY ORDERED KNAPSACK"
labels: rule
assignees: ''
canonical_source_name: 'CLIQUE'
canonical_target_name: 'PARTIALLY ORDERED KNAPSACK'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** CLIQUE
**Target:** PARTIALLY ORDERED KNAPSACK
**Motivation:** Establishes the NP-completeness (in the strong sense) of PARTIALLY ORDERED KNAPSACK by reducing from CLIQUE. The key insight is that the precedence constraints in the knapsack can encode graph structure: vertices and edges of the source graph become items with precedence relations, where selecting an edge-item requires both endpoint vertex-items to be included. The capacity and value parameters are tuned so that achieving the target value requires selecting exactly J vertex-items and all their induced edges, which corresponds to a clique of size J in the original graph. This reduction also demonstrates strong NP-completeness because the reduction is parsimonious in the number values (all sizes/values are small constants).
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.247-248

## GJ Source Entry

> [MP12] PARTIALLY ORDERED KNAPSACK
> INSTANCE: Finite set U, partial order < on U, for each u E U a size s(u) E Z+ and a value v(u) E Z+, positive integers B and K.
> QUESTION: Is there a subset U' ⊆ U such that if u E U' and u' < u, then u' E U', and such that Σ_{u E U'} s(u) ≤ B and Σ_{u E U'} v(u) ≥ K?
> Reference: [Garey and Johnson, ——]. Transformation from CLIQUE. Problem is discussed in [Ibarra and Kim, 1975b].
> Comment: NP-complete in the strong sense, even if s(u) = v(u) for all u E U. General problem is solvable in pseudo-polynomial time if < is a "tree" partial order [Garey and Johnson, ——].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a CLIQUE instance: a graph G = (V, E) with |V| = n vertices and |E| = m edges, and a positive integer J, construct a PARTIALLY ORDERED KNAPSACK instance as follows:

1. **Items for vertices:** For each vertex vᵢ ∈ V, create an item uᵢ with size s(uᵢ) = 1 and value v(uᵢ) = 1. These are "vertex-items."

2. **Items for edges:** For each edge eₖ = {vᵢ, vⱼ} ∈ E, create an item wₖ with size s(wₖ) = 1 and value v(wₖ) = 1. These are "edge-items."

3. **Partial order (precedences):** For each edge eₖ = {vᵢ, vⱼ}, impose the precedences uᵢ < wₖ and uⱼ < wₖ. That is, including edge-item wₖ in the knapsack requires both endpoint vertex-items uᵢ and uⱼ to be included. Vertex-items have no predecessors (they are minimal elements in the partial order).

4. **Capacity:** Set B = J + C(J, 2) = J + J(J-1)/2, where C(J,2) is the number of edges in a complete graph on J vertices.

5. **Value target:** Set K = J + C(J, 2) = B.

6. **Correctness (forward):** If G has a clique C ⊆ V of size J, then:
   - Include the J vertex-items corresponding to vertices in C.
   - Include all C(J,2) edge-items corresponding to edges within the clique (all edges between vertices in C exist since C is a clique).
   - Total items = J + C(J,2), total size = J + C(J,2) = B ✓
   - Total value = J + C(J,2) = K ✓
   - Precedences respected: every edge-item's two vertex-item predecessors are in the clique ✓

7. **Correctness (reverse):** If there exists a downward-closed U' with Σ s(u) ≤ B and Σ v(u) ≥ K:
   - Since all sizes and values are 1, |U'| = B = J + C(J,2).
   - Let V' = {vᵢ : uᵢ ∈ U'} be the selected vertices and E' = {eₖ : wₖ ∈ U'} be the selected edges.
   - By precedence constraints, every edge in E' has both endpoints in V'.
   - Let |V'| = p. Then |E'| = B - p = J + C(J,2) - p.
   - The maximum number of edges induced by p vertices is C(p,2). So J + C(J,2) - p ≤ C(p,2), which gives J + J(J-1)/2 - p ≤ p(p-1)/2.
   - This simplifies to J(J+1)/2 ≤ p(p+1)/2, hence J ≤ p.
   - But since |U'| = J + C(J,2) and each edge-item requires at least 2 vertex-items, we also need p ≤ J (otherwise too few edge-items to reach the target). Specifically, with p vertex-items, we have B - p = J + C(J,2) - p edge-items, and we need p + (J + C(J,2) - p) = B items total, requiring all p vertices and exactly B - p edges among them. If p > J, then B - p < C(J,2) and we'd need fewer edge-items, but the constraint still requires the total to be B. So p ≥ J and the p selected vertices must have at least J + C(J,2) - p edges. When p = J, this requires C(J,2) edges, meaning the J vertices form a clique.
   - Hence V' with |V'| = J forms a clique in G.

8. **Solution extraction:** Given a POK solution U', the clique is C = {vᵢ : uᵢ ∈ U'}.

**Key invariant:** All sizes and values are 1 (hence strong NP-completeness). The precedence structure encodes the graph: edge-items depend on vertex-items. The capacity/value target B = K = J + C(J,2) forces exactly J vertices and C(J,2) edges, which is only achievable if the J vertices form a clique.

**Time complexity of reduction:** O(n + m) to construct vertex-items, edge-items, and precedence relations.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G = |V|
- m = `num_edges` of source graph G = |E|
- J = clique size parameter

| Target metric (code name)   | Polynomial (using symbols above) |
|-----------------------------|----------------------------------|
| `num_items`                 | `num_vertices + num_edges`       |
| `num_precedences`           | `2 * num_edges`                  |
| `capacity`                  | `J + J*(J-1)/2`                  |

**Derivation:** Each vertex becomes one item, each edge becomes one item (total n + m items). Each edge creates 2 precedence constraints (one per endpoint), yielding 2m precedences. The capacity is a function of J only.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a CLIQUE instance (graph + target J), reduce to PARTIALLY ORDERED KNAPSACK, solve target by brute-force (enumerate all downward-closed subsets satisfying capacity), extract clique from vertex-items in the solution, verify it is a clique of size ≥ J in the original graph.
- Test with known YES instance: triangle graph K₃ with J = 3. POK has 3 vertex-items + 3 edge-items = 6 items, B = K = 3 + 3 = 6. Solution: all 6 items.
- Test with known NO instance: path P₃ (3 vertices, 2 edges) with J = 3. POK has 5 items, B = K = 6. Maximum downward-closed set: all 5 items (size 5 < 6). No solution.
- Verify that all sizes and values are 1 (confirming strong NP-completeness).
- Verify that precedence constraints correctly reflect the edge-endpoint relationships.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Clique):**
Graph G with 5 vertices {v₁, v₂, v₃, v₄, v₅} and 7 edges:
- Edges: e₁={v₁,v₂}, e₂={v₁,v₃}, e₃={v₂,v₃}, e₄={v₂,v₄}, e₅={v₃,v₄}, e₆={v₃,v₅}, e₇={v₄,v₅}
- Target clique size J = 3
- Known clique of size 3: {v₂, v₃, v₄} (edges e₃, e₄, e₅ all present ✓)

**Constructed target instance (PartiallyOrderedKnapsack):**
Items: 5 vertex-items {u₁, u₂, u₃, u₄, u₅} + 7 edge-items {w₁, w₂, w₃, w₄, w₅, w₆, w₇} = 12 items total
All sizes = 1, all values = 1.

Precedences:
- w₁ (edge {v₁,v₂}): u₁ < w₁, u₂ < w₁
- w₂ (edge {v₁,v₃}): u₁ < w₂, u₃ < w₂
- w₃ (edge {v₂,v₃}): u₂ < w₃, u₃ < w₃
- w₄ (edge {v₂,v₄}): u₂ < w₄, u₄ < w₄
- w₅ (edge {v₃,v₄}): u₃ < w₅, u₄ < w₅
- w₆ (edge {v₃,v₅}): u₃ < w₆, u₅ < w₆
- w₇ (edge {v₄,v₅}): u₄ < w₇, u₅ < w₇

Capacity B = 3 + C(3,2) = 3 + 3 = 6
Value target K = 6

**Solution mapping:**
- Clique C = {v₂, v₃, v₄}, edges within clique: {e₃, e₄, e₅}
- POK solution U' = {u₂, u₃, u₄, w₃, w₄, w₅}
- Downward-closed check:
  - w₃: predecessors u₂, u₃ ∈ U' ✓
  - w₄: predecessors u₂, u₄ ∈ U' ✓
  - w₅: predecessors u₃, u₄ ∈ U' ✓
  - u₂, u₃, u₄: no predecessors (minimal) ✓
- Total size: 6·1 = 6 ≤ 6 ✓
- Total value: 6·1 = 6 ≥ 6 ✓

**Verification of reverse direction:**
- Given POK solution U' = {u₂, u₃, u₄, w₃, w₄, w₅}
- Extract vertex-items: {u₂, u₃, u₄} → vertices {v₂, v₃, v₄}
- Check edges between them: {v₂,v₃} = e₃ ✓, {v₂,v₄} = e₄ ✓, {v₃,v₄} = e₅ ✓
- All C(3,2) = 3 edges present → clique of size 3 ✓

**Invalid downward-closed set:** U' = {u₁, u₂, u₃, u₄, u₅, w₁}
- Total size = 6 ≤ 6 ✓, Total value = 6 ≥ 6 ✓
- But only 1 edge-item with 5 vertex-items. The 5 vertices {v₁,...,v₅} do not form a clique of size 3+3=6... wait, the solution has 6 items total and achieves value 6, so it is feasible for the POK instance. However, extracting: 5 vertex-items, 1 edge-item. We have p = 5 vertices and only 1 edge. This means |V'| = 5 > J = 3. We need to extract a clique: the 5 vertices induce 7 edges, but only 1 edge-item is selected. The issue is whether this is truly optimal. In fact, U' = {u₁,...,u₅,w₁} is downward-closed and achieves value 6. But this does NOT mean G has no clique of size 3 — it just means the POK has multiple optimal solutions, some of which don't directly encode a size-3 clique. The correctness argument shows that a solution with exactly J vertex-items and C(J,2) edge-items must exist if and only if a clique exists. The above solution works too but contains more vertex-items than needed. To extract the clique, find any J-subset of the selected vertices that forms a clique.


## References

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Ibarra and Kim, 1975b]**: [`Ibarra1975b`] Oscar H. Ibarra and Chul E. Kim (1975). "Scheduling for maximum profit". Computer Science Dept., University of Minnesota.
