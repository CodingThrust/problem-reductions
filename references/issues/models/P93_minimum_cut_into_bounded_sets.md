---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumCutIntoBoundedSets"
labels: model
assignees: ''
---

## Motivation

MINIMUM CUT INTO BOUNDED SETS (P93) from Garey & Johnson, A2 ND17. An NP-complete graph partitioning problem that combines the classical minimum s-t cut problem with a balance constraint on the sizes of the resulting partition sets. While minimum s-t cut without balance constraints is polynomial-time solvable via network flow, adding the requirement that both sides of the partition have bounded size makes the problem NP-complete. This problem is fundamental to VLSI layout, load balancing, and graph bisection applications.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R38b: VERTEX COVER -> MINIMUM CUT INTO BOUNDED SETS (ND17)

## Definition

**Name:** <!-- ⚠️ Unverified --> `MinimumCutIntoBoundedSets`
**Canonical name:** Minimum Cut Into Bounded Sets (also: Balanced s-t Cut, Bounded Bisection)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND17

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(e) in Z+ for each e in E, specified vertices s,t in V, positive integer B <= |V|, positive integer K.
QUESTION: Is there a partition of V into disjoint sets V1 and V2 such that s in V1, t in V2, |V1| <= B, |V2| <= B, and such that the sum of the weights of the edges from E that have one endpoint in V1 and one endpoint in V2 is no more than K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |V| binary variables (one per vertex)
- **Per-variable domain:** binary {0, 1} -- side of the partition (0 = V1, 1 = V2)
- **Meaning:** variable x_v = 0 means vertex v is in V1 (with s), x_v = 1 means v is in V2 (with t). Constraints: x_s = 0, x_t = 1, sum(1-x_v) <= B, sum(x_v) <= B, and total cut weight sum(w(e) * |x_u - x_v|) <= K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `MinimumCutIntoBoundedSets`
**Variants:** graph topology (graph type parameter G), weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected weighted graph G = (V, E) |
| `source` | `usize` | Source vertex s that must be in V1 |
| `sink` | `usize` | Sink vertex t that must be in V2 |
| `size_bound` | `usize` | Maximum size B for each partition set |
| `cut_bound` | `W` | Maximum total cut weight K |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The optimization version minimizes the cut weight subject to |V1| <= B and |V2| <= B.
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Garey, Johnson, and Stockmeyer, 1976; transformation from SIMPLE MAX CUT). Remains NP-complete for B = |V|/2 and unit edge weights (the minimum bisection problem).
- **Best known exact algorithm:** The problem generalizes minimum bisection. For minimum bisection, Cygan et al. showed it is fixed-parameter tractable (FPT) with respect to the cut size. General exact approaches use ILP formulations or branch-and-bound. Without the balance constraint, minimum s-t cut is solvable in polynomial time via max-flow (e.g., O(n^3) with push-relabel).
- **Approximation:** No polynomial-time finite approximation factor for balanced graph partition unless P = NP (Andreev and Racke, 2006). O(sqrt(log n))-approximation for balanced partition (Arora, Rao, Vazirani, 2009).
- **References:**
  - M.R. Garey, D.S. Johnson, L. Stockmeyer (1976). "Some Simplified NP-Complete Graph Problems." *Theoretical Computer Science*, 1(3):237-267.
  - M. Cygan, D. Lokshtanov, M. Pilipczuk, M. Pilipczuk, S. Saurabh (2014). "Minimum Bisection is Fixed Parameter Tractable." *STOC 2014*.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), weight w(e) in Z+ for each e in E, specified vertices s,t in V, positive integer B <= |V|, positive integer K.
QUESTION: Is there a partition of V into disjoint sets V1 and V2 such that s in V1, t in V2, |V1| <= B, |V2| <= B, and such that the sum of the weights of the edges from E that have one endpoint in V1 and one endpoint in V2 is no more than K?

Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from SIMPLE MAX CUT.
Comment: Remains NP-complete for B = |V|/2 and w(e) = 1 for all e in E. Can be solved in polynomial time for B = |V| by standard network flow techniques.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all 2^n partitions of V with s in V1 and t in V2, check size bounds and compute cut weight.
- [x] It can be solved by reducing to integer programming. Binary variable per vertex, minimize cut weight subject to s/t placement and balance constraints.
- [ ] Other: Semidefinite programming relaxation for the minimum bisection variant

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Graph G with 8 vertices {0, 1, 2, 3, 4, 5, 6, 7} and 12 edges, s=0, t=7:**
- Edges with weights:
  - {0,1}: w=2, {0,2}: w=3, {1,2}: w=1, {1,3}: w=4
  - {2,4}: w=2, {3,5}: w=1, {3,6}: w=3, {4,5}: w=2
  - {4,6}: w=1, {5,7}: w=2, {6,7}: w=3, {5,6}: w=1
- Size bound B = 5, Cut bound K = 5

**Partition V1 = {0, 1, 2, 4}, V2 = {3, 5, 6, 7}:**
- |V1| = 4 <= 5, |V2| = 4 <= 5
- s=0 in V1, t=7 in V2
- Cut edges: {1,3}(4), {2,4}-NO (both in V1), {4,5}(2), {4,6}(1)
  Wait: vertex 4 is in V1, vertex 5 is in V2 -> {4,5} is cut edge.
  Vertex 4 in V1, vertex 6 in V2 -> {4,6} is cut edge.
  Vertex 1 in V1, vertex 3 in V2 -> {1,3} is cut edge.
- Cut weight = 4 + 2 + 1 = 7 > K=5

**Better partition V1 = {0, 1, 2, 3}, V2 = {4, 5, 6, 7}:**
- |V1| = 4 <= 5, |V2| = 4 <= 5
- Cut edges: {2,4}(2), {3,5}(1), {3,6}(3) -> cut weight = 2 + 1 + 3 = 6 > K=5

**Partition V1 = {0, 1, 2, 3, 4}, V2 = {5, 6, 7}:**
- |V1| = 5 <= 5, |V2| = 3 <= 5
- Cut edges: {3,5}(1), {3,6}(3), {4,5}(2), {4,6}(1) -> cut weight = 1 + 3 + 2 + 1 = 7 > K=5

**Partition V1 = {0, 1, 2, 4, 5}, V2 = {3, 6, 7}:**
- |V1| = 5 <= 5, |V2| = 3 <= 5
- Cut edges: {1,3}(4), {3,5}-NO(3 in V2, 5 in V1)=yes: {3,5}(1), {3,6}(3 in V2, 6 in V2)=no, {4,6}(1), {5,7}(2), {5,6}(5 in V1, 6 in V2)=(1), {6,7}(both V2)=no
- Cut weight = 4 + 1 + 1 + 2 + 1 = 9

With K=6: V1={0,1,2,3}, V2={4,5,6,7} gives cut weight 6 = K. Answer: YES.
With K=5: Answer: NO (no balanced partition achieves cut weight <= 5 for this graph).
