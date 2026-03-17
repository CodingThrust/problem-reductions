---
name: Problem
about: Propose a new problem type
title: "[Model] MaximumLengthBoundedDisjointPaths"
labels: model
assignees: ''
---

## Motivation

MAXIMUM LENGTH-BOUNDED DISJOINT PATHS (P117) from Garey & Johnson, A2 ND41. A classical NP-complete problem useful for reductions. It models the routing problem where multiple connections must share a network between the same source-sink pair, with quality-of-service constraints on path length. Applications include telecommunications routing and VLSI design.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in current rule set.
- **As target:** R62 (3SAT to MAXIMUM LENGTH-BOUNDED DISJOINT PATHS)

## Definition

**Name:** <!-- ⚠️ Unverified --> `MaximumLengthBoundedDisjointPaths`
**Canonical name:** Maximum Length-Bounded Disjoint Paths (also: Length-Constrained Vertex-Disjoint Paths)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND41

**Mathematical definition:**

INSTANCE: Graph G = (V,E), specified vertices s and t, positive integers J,K <= |V|.
QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, none involving more than K edges?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** J * |V| binary variables (for each of J paths, one variable per vertex indicating inclusion), or equivalently encode as a selection of J vertex-disjoint s-t paths.
- **Per-variable domain:** binary {0, 1} — whether vertex v is on path j.
- **Meaning:** The variable assignment encodes J candidate paths from s to t. Each path has at most K edges. The paths must be vertex-disjoint (except at s and t, which are shared). The metric is `bool`: True if J such paths exist, False otherwise.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `MaximumLengthBoundedDisjointPaths`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) |
| `source` | `usize` | The source vertex s |
| `sink` | `usize` | The sink vertex t |
| `num_paths_required` | `usize` | J — minimum number of disjoint paths required |
| `max_length` | `usize` | K — maximum number of edges per path |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|), `num_paths_required()` (= J), `max_length()` (= K).
- Note: s and t are shared by all paths; the vertex-disjointness applies to internal vertices only.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete for all fixed K >= 5 (Itai, Perl, and Shiloach, 1982). Solvable in polynomial time for K <= 4.
- **Best known exact algorithm:** For fixed K, brute force enumeration of all possible path combinations. For general K, this is equivalent to a constrained multi-commodity flow problem. Worst case exponential in |V|.
- **Parameterized complexity:** FPT when parameterized by J + K on certain graph classes. On general graphs, W[1]-hard parameterized by various combinations of parameters.
- **Complexity string (for general K >= 5):** `"2^num_vertices"` (brute force)
- **Special cases:**
  - K <= 4: polynomial time
  - No length constraint (K = |V|): polynomial time by standard network flow (Menger's theorem)
  - Edge-disjoint version: NP-complete for K >= 5, polynomial for K <= 3, open for K = 4
- **References:**
  - A. Itai, Y. Perl, Y. Shiloach (1982). "The complexity of finding maximum disjoint paths with length constraints." *Networks*, 12(3):277-286.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), specified vertices s and t, positive integers J,K <= |V|.
QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, none involving more than K edges?
Reference: [Itai, Perl, and Shiloach, 1977]. Transformation from 3SAT.
Comment: Remains NP-complete for all fixed K >= 5. Solvable in polynomial time for K <= 4. Problem where paths need only be edge-disjoint is NP-complete for all fixed K >= 5, polynomially solvable for K <= 3, and open for K = 4. The same results hold if G is a directed graph and the paths must be directed paths. The problem of finding the maximum number of disjoint paths from s to t, under no length constraint, is solvable in polynomial time by standard network flow techniques in both the vertex-disjoint and edge-disjoint cases.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all sets of J vertex-disjoint s-t paths of length <= K.
- [x] It can be solved by reducing to integer programming. Multi-commodity flow formulation with binary edge variables per path, flow conservation, vertex-disjointness, and path-length constraints.
- [x] Other: For K <= 4, polynomial-time algorithms exist. For unbounded K, standard maximum flow / Menger's theorem gives the answer in polynomial time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — disjoint paths exist):**
Graph G with 8 vertices {0, 1, 2, 3, 4, 5, 6, 7} and 12 edges:
- Edges: {0,1}, {0,2}, {0,3}, {1,4}, {2,5}, {3,6}, {4,7}, {5,7}, {6,7}, {1,5}, {2,6}, {3,4}
- s = 0, t = 7, J = 3, K = 3

Three vertex-disjoint s-t paths of length <= 3:
- P_1: 0 → 1 → 4 → 7 (length 3)
- P_2: 0 → 2 → 5 → 7 (length 3)
- P_3: 0 → 3 → 6 → 7 (length 3)
- All internal vertices {1,4}, {2,5}, {3,6} are pairwise disjoint ✓
- All paths have exactly 3 edges (<= K = 3) ✓
- Answer: YES

**Instance 2 (NO — not enough short disjoint paths):**
Same graph as above but with J = 3, K = 2:
- Any path of length 2 from 0 to 7 must go 0 → v → 7 where v is a common neighbor of 0 and 7.
- Neighbors of 0: {1, 2, 3}. Neighbors of 7: {4, 5, 6}.
- No vertex is a common neighbor of both 0 and 7, so no path of length 2 exists.
- Answer: NO (cannot find even J = 1 path of length <= 2)
