---
name: Problem
about: Propose a new problem type
title: "[Model] SteinerTreeInGraphs"
labels: model
assignees: ''
---

## Motivation

STEINER TREE IN GRAPHS (P88) from Garey & Johnson, A2 ND12. A classical NP-complete problem (Karp, 1972) central to network design, VLSI layout, and phylogenetic tree reconstruction. Given a graph with weighted edges and a subset of required terminal vertices, the problem asks for the minimum-weight tree connecting all terminals. It generalizes the minimum spanning tree problem (where all vertices are terminals) and is a key source for the reduction to NETWORK RELIABILITY (ND20).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated rules:**
- R41: STEINER TREE IN GRAPHS -> NETWORK RELIABILITY (ND20)

## Definition

**Name:** <!-- ⚠️ Unverified --> `SteinerTreeInGraphs`
**Canonical name:** Steiner Tree in Graphs (also: Steiner Minimum Tree, Steiner Network Problem)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND12

**Mathematical definition:**

INSTANCE: Graph G = (V,E), a weight w(e) in Z0+ for each e in E, a subset R <= V, and a positive integer bound B.
QUESTION: Is there a subtree of G that includes all the vertices of R and such that the sum of the weights of the edges in the subtree is no more than B?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |E| binary variables (one per edge)
- **Per-variable domain:** binary {0, 1} -- whether edge e is included in the Steiner tree
- **Meaning:** variable x_e = 1 if edge e is selected for the tree. A valid assignment forms a connected subtree spanning all terminals R with total weight sum(w(e) * x_e) <= B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SteinerTreeInGraphs`
**Variants:** graph topology (graph type parameter G), weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected weighted graph G = (V, E) |
| `terminals` | `Vec<usize>` | The required terminal vertices R <= V |
| `bound` | `W` | The weight bound B for the decision version |

**Notes:**
- This is a minimization problem: `Metric = SolutionSize<W>`, implementing `OptimizationProblem` with `Direction::Minimize`.
- The optimization version minimizes the total edge weight of the Steiner tree.
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|), `num_terminals()` (= |R|).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Karp, 1972; transformation from EXACT COVER BY 3-SETS).
- **Best known exact algorithm:** The Dreyfus-Wagner dynamic programming algorithm runs in O(3^k * n + 2^k * n^2 + n^3) time and O(2^k * n) space, where k = |R| is the number of terminals and n = |V|. This was improved by Fuchs, Kern, and Wang (2007) to O*(2.684^k). Bjorklund, Husfeldt, Kaski, and Koivisto (2007) achieved O*(2^k) time using subset convolution over the Mobius algebra, and Nederlof (2009) gave an O*(2^k)-time polynomial-space algorithm using Mobius inversion.
- **In terms of n (all vertices):** O*(1.36^n) by combining DP techniques, improving on the trivial O*(1.62^n) enumeration.
- **Special cases:** Polynomial-time solvable when |R| = |V| (minimum spanning tree) or |R| <= 2 (shortest path).
- **References:**
  - R.M. Karp (1972). "Reducibility Among Combinatorial Problems." *Complexity of Computer Computations*, pp. 85-103. Plenum Press.
  - S.E. Dreyfus, R.A. Wagner (1971). "The Steiner Problem in Graphs." *Networks*, 1(3):195-207.
  - B. Fuchs, W. Kern, X. Wang (2007). "Speeding up the Dreyfus-Wagner Algorithm for Minimum Steiner Trees." *Mathematical Methods of Operations Research*, 66(1):117-125.
  - J. Nederlof (2009). "Fast Polynomial-Space Algorithms Using Mobius Inversion." *ICALP 2009*, LNCS 5555, pp. 713-725.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), a weight w(e) in Z0+ for each e in E, a subset R <= V, and a positive integer bound B.
QUESTION: Is there a subtree of G that includes all the vertices of R and such that the sum of the weights of the edges in the subtree is no more than B?

Reference: [Karp, 1972]. Transformation from EXACT COVER BY 3-SETS.
Comment: Remains NP-complete if all edge weights are equal, even if G is a bipartite graph having no edges joining two vertices in R or two vertices in V-R [Berlekamp, 1976] or G is planar [Garey and Johnson, 1977a].

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all subsets of edges containing a spanning tree of the terminals R, check connectivity and compute weight.
- [x] It can be solved by reducing to integer programming. Binary variable per edge, minimize total weight subject to connectivity constraints (subtour elimination or flow-based formulation).
- [x] Other: Dreyfus-Wagner DP in O(3^k * n + 2^k * n^2) time; Nederlof O*(2^k) polynomial-space algorithm.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Graph G with 8 vertices {0, 1, 2, 3, 4, 5, 6, 7} and 12 edges:**
- Terminals R = {0, 3, 5, 7} (k = 4)
- Edges with weights:
  - {0,1}: w=2, {0,2}: w=3, {1,2}: w=1, {1,3}: w=4
  - {2,4}: w=2, {3,4}: w=3, {3,5}: w=5, {4,5}: w=1
  - {4,6}: w=2, {5,6}: w=3, {5,7}: w=4, {6,7}: w=1

**Optimal Steiner tree:**
- Edges: {0,1}(2) + {1,2}(1) + {2,4}(2) + {4,5}(1) + {4,6}(2) + {6,7}(1) + {3,4}(3)
- Total weight: 2 + 1 + 2 + 1 + 2 + 1 + 3 = 12
- Steiner vertices used: {1, 2, 4, 6} (non-terminal vertices included to reduce total weight)
- All terminals {0, 3, 5, 7} are connected in the subtree
- Budget B = 12: answer is YES
- Budget B = 11: answer is NO
