---
name: Problem
about: Propose a new problem type
title: "[Model] OptimalLinearArrangement"
labels: model
assignees: ''
---

## Motivation

OPTIMAL LINEAR ARRANGEMENT (P53) from Garey & Johnson, A1.3 GT42. A classical NP-complete graph optimization problem that asks for a vertex ordering on a line that minimizes the total edge length. It is a fundamental problem in VLSI design, graph drawing, and sparse matrix computations. It serves as a source problem for reductions to CONSECUTIVE ONES MATRIX AUGMENTATION (R110) and SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME (R134), and is itself a target of a reduction from SIMPLE MAX CUT (R278).

**Associated rules (as source):**
- R110: Optimal Linear Arrangement -> Consecutive Ones Matrix Augmentation
- R134: Optimal Linear Arrangement -> Sequencing to Minimize Weighted Completion Time
- R271: Optimal Linear Arrangement -> Directed Optimal Linear Arrangement
- R272: Optimal Linear Arrangement -> Interval Graph Completion
- R273: Optimal Linear Arrangement -> Rooted Tree Arrangement

**Associated rules (as target):**
- R278: Simple Max Cut -> Optimal Linear Arrangement

<!-- ⚠️ Unverified: AI-updated associated rules list -->

## Definition

**Name:** `OptimalLinearArrangement`
**Canonical name:** OPTIMAL LINEAR ARRANGEMENT
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT42

**Mathematical definition:**

INSTANCE: Graph G = (V, E), positive integer K.
QUESTION: Is there a one-to-one function f: V -> {1, 2, ..., |V|} such that sum_{{u,v} in E} |f(u) - f(v)| <= K?

## Variables

<!-- Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| variables, one per vertex, representing the position in the linear ordering.
- **Per-variable domain:** Each variable takes a value in {1, 2, ..., n}, subject to the constraint that all values are distinct (i.e., the assignment is a permutation).
- **Meaning:** Variable x_v = f(v) gives the position of vertex v on the line. A satisfying assignment is a permutation f such that sum_{{u,v} in E} |f(u) - f(v)| <= K.

## Schema (data type)

<!-- Unverified: AI-designed schema -->
**Type name:** `OptimalLinearArrangement`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) |
| `bound` | `usize` | The positive integer K (upper bound on total edge length) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed; edges are unweighted and the objective is purely a function of vertex positions.
- The optimization variant (minimize sum of |f(u)-f(v)|) is an `OptimizationProblem` with `Direction::Minimize`.

## Complexity

<!-- Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(2^n) time and O*(2^n) space using dynamic programming over subsets (analogous to Held-Karp for TSP), where n = |V|. Can also be solved in O*(4^n) time with polynomial space.
- **NP-completeness:** NP-complete [Garey, Johnson, and Stockmeyer, 1976]. Remains NP-complete if G is bipartite [Even and Shiloach, 1975].
- **Polynomial-time special cases:** Solvable in polynomial time if G is a tree [Shiloach, 1976], [Gol'dberg and Klipker, 1976]. The tree algorithm runs in O(n^{2.2}) time.
- **Approximation:** O(log n)-approximation via balanced separators. O(sqrt(log n) * log log n)-approximation for general graphs [Feige and Lee, 2007].
- **References:**
  - M. R. Garey, D. S. Johnson, and L. Stockmeyer (1976). "Some simplified NP-complete graph problems." *Theoretical Computer Science*, 1(3):237-267.
  - S. Even and Y. Shiloach (1975). "NP-completeness of several arrangement problems." Dept. of Computer Science, Technion.
  - Y. Shiloach (1976). "A minimum linear arrangement algorithm for undirected trees." *SIAM Journal on Computing*, 8(1):15-32.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), positive integer K.
QUESTION: Is there a one-to-one function f: V -> {1,2,...,|V|} such that sum_{{u,v} in E} |f(u)-f(v)| <= K?

Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from SIMPLE MAX CUT.
Comment: Remains NP-complete if G is bipartite [Even and Shiloach, 1975]. Solvable in polynomial time if G is a tree [Shiloach, 1976], [Gol'dberg and Klipker, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all n! permutations of vertices and compute the total edge length for each.
- [x] It can be solved by reducing to integer programming -- formulate as an ILP with binary variables x_{v,p} indicating vertex v is at position p, and minimize sum of |f(u)-f(v)| over edges.
- [x] Other: Held-Karp-style DP in O*(2^n) time; branch-and-bound with cutting planes for moderate-size instances; O(log n)-approximation via balanced separators.

## Example Instance

<!-- Unverified: AI-constructed example -->

**Instance 1 (YES instance, non-trivial):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {0,3}, {2,5}
- Bound K = 11
- Arrangement: f(0)=1, f(1)=2, f(2)=3, f(3)=4, f(4)=5, f(5)=6
- Cost: |1-2| + |2-3| + |3-4| + |4-5| + |5-6| + |1-4| + |3-6| = 1+1+1+1+1+3+3 = 11 <= 11
- Answer: YES

**Instance 2 (YES instance, better arrangement exists):**
Same graph as Instance 1, but K = 9:
- Arrangement: f(0)=1, f(3)=2, f(2)=3, f(1)=4, f(5)=5, f(4)=6
- Cost: |1-4| + |4-3| + |3-2| + |2-6| + |6-5| + |1-2| + |3-5| = 3+1+1+4+1+1+2 = 13 > 9
- Try: f(0)=1, f(1)=2, f(2)=4, f(3)=3, f(5)=5, f(4)=6
- Cost: |1-2| + |2-4| + |4-3| + |3-6| + |6-5| + |1-3| + |4-5| = 1+2+1+3+1+2+1 = 11 > 9
- Answer: NO (optimal arrangement cost = 11 for this graph, so K=9 is infeasible)

**Instance 3 (path graph, polynomial case):**
Graph G with 6 vertices, path 0-1-2-3-4-5 (5 edges):
- K = 5
- Identity arrangement: cost = 5 (each edge has length 1)
- Answer: YES (this is optimal for a path)
