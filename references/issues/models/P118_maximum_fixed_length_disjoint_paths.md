---
name: Problem
about: Propose a new problem type
title: "[Model] MaximumFixedLengthDisjointPaths"
labels: model
assignees: ''
---

## Motivation

MAXIMUM FIXED-LENGTH DISJOINT PATHS (P118) from Garey & Johnson, A2 ND42. A classical NP-complete problem useful for reductions. It is a stricter variant of the length-bounded disjoint paths problem (ND41), requiring each path to have exactly K edges rather than at most K. This arises in uniform-latency network routing where all connections must have the same delay.

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in current rule set.
- **As target:** R63 (3SAT to MAXIMUM FIXED-LENGTH DISJOINT PATHS)

## Definition

**Name:** <!-- ⚠️ Unverified --> `MaximumFixedLengthDisjointPaths`
**Canonical name:** Maximum Fixed-Length Disjoint Paths (also: Exact-Length Vertex-Disjoint Paths)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND42

**Mathematical definition:**

INSTANCE: Graph G = (V,E), specified vertices s and t, positive integers J,K <= |V|.
QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, each involving exactly K edges?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** J * |V| binary variables (for each of J paths, one variable per vertex indicating inclusion).
- **Per-variable domain:** binary {0, 1} — whether vertex v is on path j.
- **Meaning:** The variable assignment encodes J candidate paths from s to t. Each path must have exactly K edges (not fewer, not more). The paths must be vertex-disjoint at internal vertices (s and t are shared). The metric is `bool`: True if J such paths exist, False otherwise.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `MaximumFixedLengthDisjointPaths`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) |
| `source` | `usize` | The source vertex s |
| `sink` | `usize` | The sink vertex t |
| `num_paths_required` | `usize` | J — minimum number of disjoint paths required |
| `fixed_length` | `usize` | K — exact number of edges each path must have |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- Key getter methods: `num_vertices()` (= |V|), `num_edges()` (= |E|), `num_paths_required()` (= J), `fixed_length()` (= K).
- The key difference from ND41 is "exactly K" vs "at most K" — this makes the problem NP-complete starting at K >= 4 instead of K >= 5.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete for fixed K >= 4 (Itai, Perl, and Shiloach, 1982). Solvable in polynomial time for K <= 3.
- **Best known exact algorithm:** Brute force enumeration of all possible path combinations of exactly K edges. Worst case exponential in |V|.
- **Complexity string (for general K >= 4):** `"2^num_vertices"` (brute force)
- **Special cases:**
  - K <= 3: polynomial time
  - Edge-disjoint version: NP-complete for K >= 4, polynomial for K <= 2, open for K = 3
  - Directed version: same results, except arc-disjoint version is polynomial for K <= 3, open for K = 4
- **Comparison with ND41 (length-bounded):** The exact-length constraint makes the problem NP-complete one step earlier (K >= 4 vs K >= 5).
- **References:**
  - A. Itai, Y. Perl, Y. Shiloach (1982). "The complexity of finding maximum disjoint paths with length constraints." *Networks*, 12(3):277-286.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), specified vertices s and t, positive integers J,K <= |V|.
QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, each involving exactly K edges?
Reference: [Itai, Perl, and Shiloach, 1977]. Transformation from 3SAT.
Comment: Remains NP-complete for fixed K >= 4. Solvable in polynomial time for K <= 3. Corresponding problem for edge-disjoint paths is NP-complete for fixed K >= 4, polynomially solvable for K <= 2, and open for K = 3. The same results hold for directed graphs and directed paths, except that the arc-disjoint version is polynomially solvable for K <= 3 and open for K = 4.

## How to solve

- [x] It can be solved by (existing) bruteforce. Enumerate all sets of J vertex-disjoint s-t paths of exactly K edges.
- [x] It can be solved by reducing to integer programming. Multi-commodity flow formulation with binary edge variables per path, flow conservation, vertex-disjointness, and exact path-length constraints (sum of edges per path = K).
- [x] Other: For K <= 3, polynomial-time algorithms exist.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES — disjoint paths of exact length exist):**
Graph G with 10 vertices {0, 1, 2, 3, 4, 5, 6, 7, 8, 9} and 14 edges:
- Edges: {0,1}, {1,2}, {2,9}, {0,3}, {3,4}, {4,9}, {0,5}, {5,6}, {6,9}, {0,7}, {7,8}, {8,9}, {1,4}, {5,8}
- s = 0, t = 9, J = 3, K = 3

Three vertex-disjoint s-t paths of exactly 3 edges:
- P_1: 0 → 1 → 2 → 9 (exactly 3 edges)
- P_2: 0 → 3 → 4 → 9 (exactly 3 edges)
- P_3: 0 → 5 → 6 → 9 (exactly 3 edges)
- Internal vertex sets: {1,2}, {3,4}, {5,6} — pairwise disjoint ✓
- All paths have exactly 3 edges ✓
- Answer: YES

**Instance 2 (NO — paths exist but not of exact length):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {1,5}, {0,2}, {2,3}, {3,5}, {0,4}, {4,5}
- s = 0, t = 5, J = 2, K = 3

Paths from 0 to 5:
- 0 → 1 → 5 (length 2, not exactly 3)
- 0 → 2 → 3 → 5 (length 3 ✓)
- 0 → 4 → 5 (length 2, not exactly 3)
- Only one path of exactly 3 edges exists; J = 2 requires two such paths.
- Answer: NO
