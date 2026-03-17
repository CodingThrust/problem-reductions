---
name: Problem
about: Propose a new problem type
title: "[Model] BalancedCompleteBipartiteSubgraph"
labels: model
assignees: ''
---

## Motivation

BALANCED COMPLETE BIPARTITE SUBGRAPH (P35) from Garey & Johnson, A1.2 GT24. A classical NP-complete problem useful for reductions. Also known as the Maximum Balanced Biclique Problem (MBBP), it has applications in data mining, bioinformatics (protein-protein interaction networks), VLSI design, and document clustering.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R26 (CLIQUE -> BALANCED COMPLETE BIPARTITE SUBGRAPH) -- the NP-completeness proof by Garey and Johnson.
- **As source:** None found in the current rule set.

## Definition

**Name:** `BalancedCompleteBipartiteSubgraph`
<!-- ⚠️ Unverified -->
**Canonical name:** BALANCED COMPLETE BIPARTITE SUBGRAPH (also: Maximum Balanced Biclique, Balanced Biclique)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT24

**Mathematical definition:**

INSTANCE: Bipartite graph G = (V,E), positive integer K <= |V|.
QUESTION: Are there two disjoint subsets V1, V2 <= V such that |V1| = |V2| = K and such that u in V1, v in V2 implies that {u,v} in E?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| binary variables (one per vertex), encoding membership in V1 or V2.
  More precisely, for a bipartite graph with bipartition V = A union B where |A| = n_A and |B| = n_B, we have n_A + n_B binary variables.
- **Per-variable domain:** {0, 1, 2} where 0 = not selected, 1 = assigned to V1, 2 = assigned to V2. Alternatively, two binary indicator vectors: x_i in {0,1} for V1 membership, y_j in {0,1} for V2 membership.
- **Meaning:** x_i = 1 means vertex i from partition A is included in V1; y_j = 1 means vertex j from partition B is included in V2. A satisfying assignment has sum(x_i) = sum(y_j) = K and for every pair (i,j) with x_i = 1 and y_j = 1, the edge {i,j} exists in E.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `BalancedCompleteBipartiteSubgraph`
**Variants:** graph type parameter G (bipartite graphs only)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `BipartiteGraph` | The bipartite graph in which a balanced complete bipartite subgraph is sought |
| `k` | `usize` | The required size K of each side of the biclique |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The graph must be bipartite. The `BipartiteGraph` type encodes the bipartition.
- For the optimization variant (find the largest K), `Metric = SolutionSize<usize>` with `Direction::Maximize`.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(1.3803^n) for dense bipartite graphs, where n is the total number of vertices. This is based on a refined pivot-based branching algorithm by Chen et al. (2020), "Efficient Exact Algorithms for Maximum Balanced Biclique Search in Bipartite Graphs".
- **Brute force:** O(2^n) by trying all subsets of each partition.
- **NP-completeness:** NP-complete (Garey and Johnson, 1979, GT24). Transformation from CLIQUE.
- **Parameterized complexity:** W[1]-hard parameterized by solution size K (Lin, 2015, "The Parameterized Complexity of the k-Biclique Problem").
- **Approximation hardness:** NP-hard to approximate within a factor of n^(1-epsilon) for any epsilon > 0, assuming the Small Set Expansion Hypothesis (Manurangsi, 2017).
- **References:**
  - Garey, M.R. and Johnson, D.S. (1979). *Computers and Intractability: A Guide to the Theory of NP-Completeness*. W.H. Freeman.
  - Chen, Y. et al. (2020). "Efficient Exact Algorithms for Maximum Balanced Biclique Search in Bipartite Graphs". arXiv:2007.08836.
  - Lin, B. (2015). "The Parameterized Complexity of the k-Biclique Problem". *Journal of the ACM* 65(5).

## Extra Remark

**Full book text:**

INSTANCE: Bipartite graph G = (V,E), positive integer K <= |V|.
QUESTION: Are there two disjoint subsets V1, V2 <= V such that |V1| = |V2| = K and such that u in V1, v in V2 implies that {u,v} in E?
Reference: [Garey and Johnson, ----]. Transformation from CLIQUE.
Comment: The related problem in which the requirement "|V1| = |V2| = K" is replaced by "|V1|+|V2| = K" is solvable in polynomial time for bipartite graphs (because of the connection between matchings and independent sets in such graphs, e.g., see [Harary, 1969]), but is NP-complete for general graphs [Yannakakis, 1978b].

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all pairs of K-subsets from each partition and check completeness.
- [x] It can be solved by reducing to integer programming -- ILP formulation: binary variables for vertex selection, constraints for completeness and balance.
- [x] Other: Branch-and-bound with upper bound propagation (Chen et al., 2020); reduction to CLIQUE on a derived graph.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (has balanced biclique of size 3):**
Bipartite graph G with bipartition A = {a0, a1, a2, a3} and B = {b0, b1, b2, b3}, 10 edges:
- Edges: {a0,b0}, {a0,b1}, {a0,b2}, {a1,b0}, {a1,b1}, {a1,b3}, {a2,b0}, {a2,b2}, {a2,b3}, {a3,b1}
- K = 3
- Solution: V1 = {a0, a1, a2}, V2 = {b0, b1, b2}?
  - Check: a0-b0 YES, a0-b1 YES, a0-b2 YES, a1-b0 YES, a1-b1 YES, a1-b2? NO (not in edge set).
  - Try V1 = {a0, a1, a2}, V2 = {b0, b1, b3}?
    - a0-b0 YES, a0-b1 YES, a0-b3? NO.
  - Actually V1 = {a0, a1}, V2 = {b0, b1}: a0-b0 YES, a0-b1 YES, a1-b0 YES, a1-b1 YES. K_{2,2} biclique.
  - Maximum balanced biclique is K = 2 for this instance.
- Answer for K = 3: NO. Answer for K = 2: YES.

**Instance 2 (has balanced biclique of size 3):**
Bipartite graph G with bipartition A = {a0, a1, a2, a3} and B = {b0, b1, b2, b3}, 12 edges:
- Edges: {a0,b0}, {a0,b1}, {a0,b2}, {a1,b0}, {a1,b1}, {a1,b2}, {a2,b0}, {a2,b1}, {a2,b2}, {a3,b0}, {a3,b1}, {a3,b3}
- K = 3
- Solution: V1 = {a0, a1, a2}, V2 = {b0, b1, b2}.
  - Check all 9 pairs: a0-b0 YES, a0-b1 YES, a0-b2 YES, a1-b0 YES, a1-b1 YES, a1-b2 YES, a2-b0 YES, a2-b1 YES, a2-b2 YES.
  - Complete K_{3,3} biclique. Answer: YES.
- Note: a3 is NOT in V1 because a3-b2 is missing.
- Greedy trap: Including a3 (which has 3 edges) leads to failure since a3's neighbors {b0, b1, b3} don't fully overlap with any 3 vertices in B that also connect to a0, a1, a2.
