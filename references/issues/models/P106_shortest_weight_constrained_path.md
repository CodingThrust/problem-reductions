---
name: Problem
about: Propose a new problem type
title: "[Model] ShortestWeightConstrainedPath"
labels: model
assignees: ''
---

## Motivation

SHORTEST WEIGHT-CONSTRAINED PATH (P106) from Garey & Johnson, A2 ND30. A classical NP-complete problem that asks for a simple s-t path simultaneously satisfying both a length budget and a weight budget. This bicriteria path problem arises naturally in routing with quality-of-service constraints (e.g., minimize delay subject to a bandwidth constraint). NP-complete even on acyclic graphs, but solvable in polynomial time if all weights are equal or all lengths are equal.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in the current rule set.
- **As target:** R51: PARTITION -> SHORTEST WEIGHT-CONSTRAINED PATH

## Definition

**Name:** `ShortestWeightConstrainedPath`
<!-- ⚠️ Unverified -->
**Canonical name:** SHORTEST WEIGHT-CONSTRAINED PATH (also: Weight-Constrained Shortest Path, Constrained Shortest Path, Resource-Constrained Shortest Path)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND30

**Mathematical definition:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+, and weight w(e) ∈ Z^+ for each e ∈ E, specified vertices s,t ∈ V, positive integers K,W.
QUESTION: Is there a simple path in G from s to t with total weight W or less and total length K or less?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |E| binary variables (one per edge), indicating whether the edge is included in the path.
- **Per-variable domain:** {0, 1} -- edge is excluded or included in the s-t path.
- **Meaning:** The variable assignment encodes a subset of edges. A satisfying assignment is a subset S of E such that the subgraph induced by S forms a simple path from s to t, the sum of l(e) for e in S is at most K, and the sum of w(e) for e in S is at most W.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `ShortestWeightConstrainedPath`
**Variants:** graph type (G), numeric type for lengths and weights (W)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `G` | The undirected graph G = (V, E) |
| `lengths` | `Vec<W>` | Edge length l(e) for each edge |
| `weights` | `Vec<W>` | Edge weight w(e) for each edge |
| `source` | `usize` | Index of source vertex s |
| `target` | `usize` | Index of target vertex t |
| `length_bound` | `W` | The length bound K |
| `weight_bound` | `W` | The weight bound W |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Has two simultaneous constraints (length and weight), which is what makes it NP-hard. With only one constraint it reduces to standard shortest path (polynomial).
- Generalizes to the Resource-Constrained Shortest Path Problem (RCSPP) with multiple resource constraints.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Pseudo-polynomial time O(|V| * |E| * W_max) via dynamic programming over weight values (Lagrangian relaxation approaches). For the general case, exact algorithms are exponential. FPTAS exists with (1+epsilon) approximation on the weight constraint (Hassin, 1992; Lorenz and Raz, 2001).
- **Classic algorithm:** O(n * 2^n) via subset enumeration (Held-Karp style DP adapted for bicriteria).
- **NP-completeness:** NP-complete by transformation from PARTITION (Megiddo, 1977; Garey & Johnson, ND30). Also NP-complete for directed graphs.
- **Special cases:** Polynomial-time solvable if all weights are equal or all lengths are equal (reduces to single-criterion shortest path).
- **References:**
  - N. Megiddo (1977). On the complexity of some optimization problems.
  - R. Hassin (1992). "Approximation schemes for the restricted shortest path problem." *Mathematics of Operations Research*, 17(1):36-42.
  - D.H. Lorenz, D. Raz (2001). "A simple efficient approximation scheme for the restricted shortest path problem." *Operations Research Letters*, 28(5):213-219.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+, and weight w(e) ∈ Z^+ for each e ∈ E, specified vertices s,t ∈ V, positive integers K,W.
QUESTION: Is there a simple path in G from s to t with total weight W or less and total length K or less?
Reference: [Megiddo, 1977]. Transformation from PARTITION.
Comment: Also NP-complete for directed graphs. Both problems are solvable in polynomial time if all weights are equal or all lengths are equal.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all simple s-t paths and check both constraints.
- [x] It can be solved by reducing to integer programming -- minimize total length subject to total weight <= W and path connectivity constraints.
- [x] Other: Pseudo-polynomial DP in O(|V| * |E| * W_max); FPTAS with (1+epsilon) approximation.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES -- feasible path exists):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 8 edges:
- s = 0, t = 5, K = 10, W = 8
- Edges (length, weight):
  - {0,1}: (2, 5)
  - {0,2}: (4, 1)
  - {1,3}: (3, 2)
  - {2,3}: (1, 3)
  - {2,4}: (5, 2)
  - {3,5}: (4, 3)
  - {4,5}: (2, 1)
  - {1,4}: (6, 1)

- Path 0 -> 2 -> 3 -> 5: length = 4+1+4 = 9, weight = 1+3+3 = 7. Both 9 <= 10 and 7 <= 8. YES.
- Path 0 -> 2 -> 4 -> 5: length = 4+5+2 = 11 > 10. Fails length bound.
- Path 0 -> 1 -> 3 -> 5: length = 2+3+4 = 9, weight = 5+2+3 = 10 > 8. Fails weight bound.

**Instance 2 (NO -- no feasible path):**
Same graph, K = 6, W = 4.
- Path 0->2->3->5: length=9 > 6. Fails.
- Path 0->1->3->5: length=9 > 6. Fails.
- Path 0->2->4->5: length=11 > 6. Fails.
- Path 0->1->4->5: length=2+6+2=10 > 6. Fails.
- No simple s-t path has both length <= 6 and weight <= 4.
- Answer: NO
