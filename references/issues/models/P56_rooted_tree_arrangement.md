---
name: Problem
about: Propose a new problem type
title: "[Model] RootedTreeArrangement"
labels: model
assignees: ''
---

## Motivation

ROOTED TREE ARRANGEMENT (P56) from Garey & Johnson, A1.3 GT45. An NP-complete graph arrangement problem that asks whether the vertices of a given graph can be embedded one-to-one into the nodes of a rooted tree such that every edge of the graph connects two vertices on the same root-to-leaf path, and the total edge-stretch (sum of tree distances over all graph edges) is bounded by K. This generalizes OPTIMAL LINEAR ARRANGEMENT (GT43) to tree-structured layouts. Gavril (1977) proved NP-completeness via reduction from OPTIMAL LINEAR ARRANGEMENT. The problem arises in file system layout, memory hierarchy design, and data structure optimization.

**Associated rules:**
- R099: Rooted Tree Arrangement → Rooted Tree Storage Assignment (as source)
- Reduced from OPTIMAL LINEAR ARRANGEMENT (per GJ book text, GT43 → GT45)

## Definition

**Name:** `RootedTreeArrangement`
**Canonical name:** ROOTED TREE ARRANGEMENT
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT45

**Mathematical definition:**

INSTANCE: Graph G = (V, E), positive integer K.
QUESTION: Is there a rooted tree T = (U, F), with |U| = |V|, and a one-to-one function f: V -> U such that for every edge {u,v} in E there is a simple path from the root that includes both f(u) and f(v) and such that if d(x,y) is the number of edges on the path from x to y in T, then sum_{{u,v} in E} d(f(u), f(v)) <= K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** The solution has two components: (1) a rooted tree T on n = |V| nodes, and (2) a bijection f: V -> U. Together these encode O(n) discrete choices.
- **Per-variable domain:** For the tree structure: one of the Catalan-number-many rooted labeled trees on n nodes. For the bijection: one of n! permutations.
- **Meaning:** The variable assignment encodes both the tree topology and the vertex placement. A satisfying assignment is a (tree, mapping) pair such that every graph edge maps to a pair of nodes on a common root-to-leaf path, with total distance at most K. The key constraint is the "ancestral" requirement: for each edge {u,v}, one of f(u), f(v) must be an ancestor of the other in T.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `RootedTreeArrangement`
**Variants:** graph topology (graph type parameter G)

| Field   | Type          | Description                                                |
|---------|---------------|------------------------------------------------------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) to be arranged in a tree   |
| `bound` | `usize`       | Maximum total distance K                                    |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed; edge distances are measured in the tree (hop count).
- The rooted tree T is part of the solution, not part of the instance.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** No specialized exact algorithm is known; brute-force must enumerate all rooted labeled trees on n nodes (n^(n-1) by Cayley's formula) and all n! bijections, checking the ancestral path constraint and distance bound. This gives O(n^n * n! * m) time, which is impractical even for small n.
- **Special cases:** For trees (G is a tree), the problem can be solved in polynomial time. Adolphson and Hu (1973) gave an O(n log n) algorithm for optimal linear arrangement of trees, and similar techniques apply.
- **NP-completeness:** NP-complete [Gavril, 1977a], via reduction from OPTIMAL LINEAR ARRANGEMENT (GT43).
- **References:**
  - F. Gavril (1977). "Some NP-complete problems on graphs." In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91-95. Johns Hopkins University.
  - M. R. Garey, D. S. Johnson, and L. Stockmeyer (1976). "Some simplified NP-complete graph problems." *Theoretical Computer Science* 1(3):237-267.
  - D. Adolphson and T. C. Hu (1973). "Optimal linear ordering." *SIAM Journal on Applied Mathematics* 25(3):403-423.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), positive integer K.
QUESTION: Is there a rooted tree T = (U,F), with |U| = |V|, and a one-to-one function f: V -> U such that for every edge {u,v} in E there is a simple path from the root that includes both f(u) and f(v) and such that if d(x,y) is the number of edges on the path from x to y in T, then sum_{{u,v} in E} d(f(u),f(v)) <= K?

Reference: [Gavril, 1977a]. Transformation from OPTIMAL LINEAR ARRANGEMENT.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all possible rooted trees on n nodes and all bijections f: V -> U, check ancestral path constraint and distance bound.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: For special graph classes (paths, trees), polynomial-time algorithms exist. For general graphs, the problem is NP-complete.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES, path graph can be arranged tightly):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 5 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}
- (Simple path P_6)
- Bound K = 5.

Solution: Use rooted tree T as a chain 0 -> 1 -> 2 -> 3 -> 4 -> 5 (rooted at 0), with identity mapping f(v) = v.
- Each edge {i, i+1} has d(i, i+1) = 1 in T.
- Total distance = 5 * 1 = 5 <= K = 5. ✓
- Each pair f(i), f(i+1) lies on the root-to-leaf path 0->1->2->3->4->5. ✓
- Answer: YES

**Instance 2 (YES, star graph):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 8 edges:
- Edges: {0,1}, {0,2}, {0,3}, {0,4}, {0,5}, {1,2}, {3,4}, {2,3}
- Bound K = 14.

Solution: Use rooted tree T:
```
      0
     / \
    1    3
    |    |
    2    4
         |
         5
```
Mapping: f(v) = v (identity).
- Check ancestral paths:
  - {0,1}: 0 is ancestor of 1. d = 1. ✓
  - {0,2}: 0 is ancestor of 2 (via 0->1->2). d = 2. ✓
  - {0,3}: 0 is ancestor of 3. d = 1. ✓
  - {0,4}: 0 is ancestor of 4 (via 0->3->4). d = 2. ✓
  - {0,5}: 0 is ancestor of 5 (via 0->3->4->5). d = 3. ✓
  - {1,2}: 1 is ancestor of 2. d = 1. ✓
  - {3,4}: 3 is ancestor of 4. d = 1. ✓
  - {2,3}: 2 is on path 0->1->2, 3 is on path 0->3. NOT on same root-to-leaf path! Fail!

This shows the ancestral constraint is restrictive. Edge {2,3} cannot be satisfied in this tree because 2 and 3 are in different branches. We need a different tree.

Revised tree:
```
    0 -> 1 -> 2 -> 3 -> 4 -> 5
```
(A single chain.) Now every pair of vertices is on the same root-to-leaf path.
- d(0,1) = 1, d(0,2) = 2, d(0,3) = 3, d(0,4) = 4, d(0,5) = 5
- d(1,2) = 1, d(3,4) = 1, d(2,3) = 1
- Total = 1+2+3+4+5+1+1+1 = 18. Need K >= 18.

Set K = 18. Answer: YES ✓

**Instance 3 (NO, triangle needs chain):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges forming two triangles connected by a path:
- Edges: {0,1}, {1,2}, {0,2}, {2,3}, {3,4}, {4,5}, {3,5}
- Bound K = 8.

In any rooted tree, the triangle {0,1,2} forces all three vertices onto a single root-to-leaf path, costing at least d(0,1) + d(1,2) + d(0,2) >= 1 + 1 + 2 = 4. Similarly {3,4,5} with edge {3,5} forces a chain costing at least 4. Plus edge {2,3} costs at least 1. Minimum total >= 4 + 4 + 1 = 9 > K = 8. Answer: NO.
