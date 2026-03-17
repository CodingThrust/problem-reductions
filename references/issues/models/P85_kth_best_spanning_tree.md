---
name: Problem
about: Propose a new problem type
title: "[Model] KthBestSpanningTree"
labels: model
assignees: ''
---

## Motivation

Kth BEST SPANNING TREE (P85) from Garey & Johnson, A2 ND9. An NP-hard problem (marked with (*) in G&J, meaning it is not known to be in NP) that asks whether a weighted graph has K distinct spanning trees each with total weight at most B. The problem generalizes minimum spanning tree enumeration and has connections to network reliability and sensitivity analysis. It can be solved in pseudo-polynomial time via Lawler's K-best enumeration procedure (1972).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As target:** R30 (HAMILTONIAN PATH -> KTH BEST SPANNING TREE) -- Turing reduction establishing NP-hardness (Johnson and Kashdan, 1976).
- **As source:** None found in the current rule set.

## Definition

**Name:** `KthBestSpanningTree`
<!-- ⚠️ Unverified -->
**Canonical name:** K^th BEST SPANNING TREE (also: K Minimum Spanning Trees, Multiple Minimum Weight Spanning Trees)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND9

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(e) in Z_0^+ for each e in E, positive integers K and B.
QUESTION: Are there K distinct spanning trees for G, each having total weight B or less?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** m = |E| binary variables per spanning tree, indicating edge inclusion. Since we need K spanning trees, total variables are K * m, though the decision problem only asks for existence.
- **Per-variable domain:** {0, 1} for each edge: 1 if the edge is included in a particular spanning tree, 0 otherwise.
- **Meaning:** Each spanning tree is represented by a subset of exactly n-1 edges from E that form a connected acyclic subgraph. The weight of a spanning tree is the sum of weights of its edges. A satisfying assignment provides K distinct such subsets, each with total weight at most B. Two spanning trees are "distinct" if they differ in at least one edge.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `KthBestSpanningTree<W>`
**Variants:** graph type parameter G, weight type W

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected weighted graph G |
| `weights` | `Vec<W>` | Edge weights w(e) for each edge in E, non-negative integers |
| `k` | `usize` | The number K of distinct spanning trees required |
| `b` | `W` | The weight bound B; each spanning tree must have total weight <= B |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- Weight type W should implement `WeightElement` (typically `i32` or `usize` for non-negative integer weights).
- The problem is NOT known to be in NP (certificate may require listing K spanning trees, which could be exponential in size).
- For fixed K, the problem is polynomial-time solvable.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:**
  - For fixed K: Polynomial time. Lawler's procedure (1972) finds the K best solutions in O(K * n * c(n)) time, where c(n) is the cost of finding one optimal spanning tree (O(m log n) or O(m * alpha(n)) with the best MST algorithms). This gives O(K * n * m * alpha(n)).
  - Eppstein (1992) improved this to O(m * alpha(n) + K * n^(1/2)) for unweighted graphs and O(m * log(m/n) + min(K * n, K^(1/2) * m)) for weighted graphs.
  - For variable K (part of input): NP-hard (Turing reduction from Hamiltonian Path; Johnson and Kashdan, 1976).
- **Pseudo-polynomial time:** The problem can be solved in time polynomial in |V|, K, log B, and max{log w(e)} (Lawler, 1972).
- **Related enumeration:** The problem of counting ALL spanning trees of weight <= B is #P-complete. However, the unweighted enumeration problem (counting all spanning trees) is polynomial via Kirchhoff's matrix tree theorem.
- **NP-hardness status:** NP-hard but not known to be in NP (marked (*) in G&J).
- **References:**
  - Johnson, D.B. and Kashdan, S.D. (1976). "Lower bounds for selection in X+Y and other multisets". Penn State CS Dept.
  - Lawler, E.L. (1972). "A procedure for computing the K best solutions to discrete optimization problems". *Management Science* 18, pp. 401-405.
  - Eppstein, D. (1992). "Finding the k Smallest Spanning Trees". *BIT Numerical Mathematics* 32, pp. 237-248.
  - Harary, F. and Palmer, E.M. (1973). "Graphical Enumeration". Academic Press.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), weight w(e) in Z0+ for each e in E, positive integers K and B.
QUESTION: Are there K distinct spanning trees for G, each having total weight B or less?

Reference: [Johnson and Kashdan, 1976]. Turing reduction from HAMILTONIAN PATH.
Comment: Not known to be in NP. Can be solved in pseudo-polynomial time (polynomial in |V|, K, log B, max {log w(e): e in E}) [Lawler, 1972], and hence in polynomial time for any fixed value of K. The corresponding enumeration problem is #P-complete. However, the unweighted case of the enumeration problem is solvable in polynomial time (e.g., see [Harary and Palmer, 1973]).

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all possible spanning trees (subsets of n-1 edges forming a tree), count those with weight <= B, check if count >= K.
- [x] It can be solved by reducing to integer programming -- ILP with K sets of binary edge variables, tree constraints (flow-based or subtour elimination), weight constraints, and distinctness constraints.
- [x] Other: Lawler's K-best enumeration (1972): iteratively find the next-best spanning tree by partitioning the solution space. Eppstein's improved algorithm (1992) for faster K-best spanning tree enumeration.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES):**
Graph G with 5 vertices {0, 1, 2, 3, 4} and 8 edges with weights:
- {0,1}: w=2, {0,2}: w=3, {1,2}: w=1, {1,3}: w=4, {2,3}: w=2, {2,4}: w=5, {3,4}: w=3, {0,4}: w=6
- K = 3, B = 12

Spanning trees (need 4 edges each, total weight <= 12):
1. MST: {1,2}(1) + {2,3}(2) + {0,1}(2) + {3,4}(3) = weight 8. Edges: {0,1},{1,2},{2,3},{3,4}.
2. 2nd best: Replace {2,3}(2) with {1,3}(4): {0,1}(2) + {1,2}(1) + {1,3}(4) + {3,4}(3) = weight 10. Edges: {0,1},{1,2},{1,3},{3,4}.
3. 3rd best: Replace {0,1}(2) with {0,2}(3) in MST: {0,2}(3) + {1,2}(1) + {2,3}(2) + {3,4}(3) = weight 9. Edges: {0,2},{1,2},{2,3},{3,4}.

All three have weight <= 12. Are they distinct? Tree 1 has edges {0,1},{1,2},{2,3},{3,4}. Tree 3 has edges {0,2},{1,2},{2,3},{3,4}. Different (edge {0,1} vs {0,2}). Tree 2 has {0,1},{1,2},{1,3},{3,4}. Different from both.

Answer: YES (3 distinct spanning trees with weight <= 12 exist).

**Instance 2 (NO):**
Graph G with 4 vertices {0, 1, 2, 3} forming a path: {0,1}: w=1, {1,2}: w=1, {2,3}: w=1.
- This graph is already a tree (only 3 edges = n-1).
- K = 2, B = 3.
- Only ONE spanning tree exists (the graph itself, weight 3).
- Answer: NO (need 2 distinct spanning trees, but only 1 exists).
