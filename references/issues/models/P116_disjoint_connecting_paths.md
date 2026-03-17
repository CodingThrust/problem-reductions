---
name: Problem
about: Propose a new problem type
title: "[Model] DisjointConnectingPaths"
labels: model
assignees: ''
---

## Motivation

DISJOINT CONNECTING PATHS (P116) from Garey & Johnson, A2 ND40. A classical NP-complete problem useful for reductions. It models the fundamental routing/interconnection problem: given a set of terminal pairs in a network, can all pairs be simultaneously connected by vertex-disjoint paths? This has applications in VLSI design, network routing, and wire layout.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in current rule set.
- **As target:** R61 (3SAT to DISJOINT CONNECTING PATHS)

## Definition

**Name:** <!-- ⚠️ Unverified --> `DisjointConnectingPaths`
**Canonical name:** Disjoint Connecting Paths (also: Vertex-Disjoint Paths Problem, Node-Disjoint Paths, Interconnection Problem)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND40

**Mathematical definition:**

INSTANCE: Graph G = (V,E), collection of disjoint vertex pairs (s_1,t_1),(s_2,t_2),...,(s_k,t_k).
QUESTION: Does G contain k mutually vertex-disjoint paths, one connecting s_i and t_i for each i, 1 <= i <= k?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |E| binary variables (one per edge), or equivalently, for each terminal pair i, choose a path P_i from s_i to t_i. The configuration encodes which edges are used by each path.
- **Per-variable domain:** binary {0, 1} — whether edge e is used by any of the k paths.
- **Meaning:** The variable assignment encodes a set of k paths. For a valid solution, the paths must be vertex-disjoint (no two paths share an internal vertex) and path P_i must connect s_i to t_i. The metric is `bool`: True if k vertex-disjoint paths exist, False otherwise.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `DisjointConnectingPaths`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) |
| `terminal_pairs` | `Vec<(usize, usize)>` | The k disjoint terminal pairs (s_i, t_i) |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|), `num_pairs()` (= k).
- The terminal vertices must be pairwise disjoint (no vertex appears in more than one pair).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete (Knuth 1974, Karp 1975, Lynch 1974; transformation from 3SAT). Remains NP-complete for planar graphs (Lynch 1975).
- **Fixed-parameter tractability:** For fixed k (number of terminal pairs), the problem is solvable in polynomial time:
  - O(n^3) by Robertson and Seymour's graph minor algorithm (1995), as part of their Graph Minors series.
  - O(n^2) by Kawarabayashi, Kobayashi, and Reed (2012), improving the cubic bound.
  - However, the constant factor is enormous (tower of exponentials in k), making these algorithms impractical.
- **Best known exact algorithm (general k):** Brute force enumeration of all possible path combinations. For each terminal pair, enumerate paths and check vertex-disjointness. Worst case exponential in |V|.
- **Complexity string (for general k):** `"2^num_vertices"` (brute force over all possible path selections)
- **References:**
  - N. Robertson and P.D. Seymour (1995). "Graph Minors. XIII. The Disjoint Paths Problem." *Journal of Combinatorial Theory, Series B*, 63(1):65-110. Polynomial algorithm for fixed k.
  - K. Kawarabayashi, Y. Kobayashi, B. Reed (2012). "The disjoint paths problem in quadratic time." *Journal of Combinatorial Theory, Series B*, 102(2):424-435.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), collection of disjoint vertex pairs (s_1,t_1),(s_2,t_2),...,(s_k,t_k).
QUESTION: Does G contain k mutually vertex-disjoint paths, one connecting s_i and t_i for each i, 1 <= i <= k?
Reference: [Knuth, 1974c], [Karp, 1975a], [Lynch, 1974]. Transformation from 3SAT.
Comment: Remains NP-complete for planar graphs [Lynch, 1974], [Lynch, 1975]. Complexity is open for any fixed k >= 2, but can be solved in polynomial time if k = 2 and G is planar or chordal [Perl and Shiloach, 1978]. (A polynomial time algorithm for the general 2 path problem has been announced in [Shiloach, 1978]). The directed version of this problem is also NP-complete in general and solvable in polynomial time when k = 2 and G is planar or acyclic [Perl and Shiloach, 1978].

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all possible assignments of paths for each terminal pair and check vertex-disjointness.
- [x] It can be solved by reducing to integer programming. Introduce binary variables for edge usage per path, add flow conservation constraints, and vertex-disjointness constraints.
- [x] Other: For fixed k, Robertson-Seymour O(n^3) or Kawarabayashi-Kobayashi-Reed O(n^2). For k = 2, polynomial algorithms exist (Shiloach 1978).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — disjoint paths exist):**
Graph G with 8 vertices {0, 1, 2, 3, 4, 5, 6, 7} and 10 edges:
- Edges: {0,1}, {0,2}, {1,3}, {2,3}, {3,4}, {4,5}, {4,6}, {5,7}, {6,7}, {2,6}
- Terminal pairs: (0, 7), (2, 5)

Vertex-disjoint paths:
- Path P_1: 0 → 1 → 3 → 4 → 5 → 7 (connecting 0 to 7)
- Path P_2: 2 → 6 → 4... wait, vertex 4 is used by P_1.
- Revised paths:
  - P_1: 0 → 2 → 3 → 4 → 5 → 7 (connecting 0 to 7)
  - P_2: ... vertex 2 is now used by P_1.
  - P_1: 0 → 1 → 3 → 4 → 6 → 7 (connecting 0 to 7)
  - P_2: 2 → 3... vertex 3 used. Need: 2 → 6... vertex 6 used by P_1.
- Better graph: add edge {2,5}:
  - P_1: 0 → 1 → 3 → 4 → 6 → 7
  - P_2: 2 → 5 (direct)
  - Vertex-disjoint ✓
- Answer: YES

**Instance 2 (NO — no disjoint paths):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 6 edges:
- Edges: {0,2}, {1,2}, {2,3}, {3,4}, {3,5}
- Terminal pairs: (0, 4), (1, 5)
- Vertex 2 and vertex 3 are cut vertices; any path from 0 to 4 must pass through 2 and 3, and any path from 1 to 5 must also pass through 2 and 3. Since both paths must share vertices 2 and 3, no vertex-disjoint solution exists.
- Answer: NO
