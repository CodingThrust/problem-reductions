---
name: Problem
about: Propose a new problem type
title: "[Model] StackerCrane"
labels: model
assignees: ''
---

## Motivation

STACKER-CRANE (P102) from Garey & Johnson, A2 ND26. A classical NP-complete problem on mixed graphs (containing both directed arcs and undirected edges). It generalizes the Traveling Salesman Problem to settings where certain traversals must follow a prescribed direction, modeling pickup-and-delivery routing scenarios. The problem is a target of the following reduction:

<!-- ⚠️ Unverified: AI-collected rule associations -->
- **R47: HAMILTONIAN CIRCUIT → STACKER-CRANE** (ND26, Frederickson, Hecht, and Kim, 1978)

## Definition

**Name:** `StackerCrane`
<!-- ⚠️ Unverified -->
**Canonical name:** STACKER-CRANE (also: Stacker Crane Problem, SCP)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND26

**Mathematical definition:**

INSTANCE: Mixed graph G = (V,A,E), length l(e) ∈ Z_0^+ for each e ∈ A ∪ E, bound B ∈ Z^+.
QUESTION: Is there a cycle in G that includes each directed edge in A at least once, traversing such edges only in the specified direction, and that has total length no more than B?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** The cycle is a sequence of edges/arcs. For a mixed graph with |V| = n vertices, |A| = a directed arcs, and |E| = m undirected edges, the solution can be encoded as a sequence of at most n + a + m edge traversals. Alternatively, in a permutation/bitmask-DP encoding: O(n · 2^n) states.
- **Per-variable domain:** Each step in the sequence selects the next edge/arc to traverse from the current vertex. In a DP formulation: (current vertex, set of arcs already covered).
- **Meaning:** The variable assignment encodes a closed walk (cycle) in the mixed graph. A satisfying assignment is a cycle that (1) traverses every directed arc in A at least once, respecting arc directions, and (2) has total length ≤ B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `StackerCrane`
**Variants:** graph topology (mixed graph with directed arcs and undirected edges)

| Field | Type | Description |
|-------|------|-------------|
| `vertices` | `usize` | Number of vertices |V| |
| `arcs` | `Vec<(usize, usize)>` | Directed arcs A: list of (from, to) pairs |
| `edges` | `Vec<(usize, usize)>` | Undirected edges E: list of {u, v} pairs |
| `arc_lengths` | `Vec<i32>` | Length of each directed arc (non-negative) |
| `edge_lengths` | `Vec<i32>` | Length of each undirected edge (non-negative) |
| `bound` | `i32` | Upper bound B on total cycle length |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The mixed graph structure distinguishes it from pure undirected (TSP, Chinese Postman) or pure directed problems.
- Specializations: remains NP-complete even with all edge lengths equal to 1 (GJ comment). Also NP-hard on trees (Frederickson and Guan).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** As a generalization of TSP, the Stacker-Crane problem can be solved by Held-Karp-style dynamic programming in O(n^2 · 2^n) time and O(n · 2^n) space, where n = |V|. The DP state is (current vertex, set of arcs covered so far).
- **NP-completeness:** NP-complete (Frederickson, Hecht, and Kim, 1978, via transformation from Hamiltonian Circuit).
- **Approximation:** 9/5-approximation algorithm based on the Christofides algorithm (Frederickson, Hecht, and Kim, 1978).
- **Special cases:** Polynomial-time solvable when the underlying graph is a path. NP-hard on trees (Frederickson and Guan, 1991).
- **References:**
  - G. N. Frederickson, M. S. Hecht, C. E. Kim (1978). "Approximation algorithms for some routing problems." *SIAM Journal on Computing* 7(2):178–193.
  - G. N. Frederickson (1991). "Approximation algorithms for some postman problems." *JACM* 26:538–554.

## Extra Remark

**Full book text:**

INSTANCE: Mixed graph G = (V,A,E), length l(e) ∈ Z_0^+ for each e ∈ A ∪ E, bound B ∈ Z^+.
QUESTION: Is there a cycle in G that includes each directed edge in A at least once, traversing such edges only in the specified direction, and that has total length no more than B?
Reference: [Frederickson, Hecht, and Kim, 1978]. Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete even if all edge lengths equal 1. The analogous path problem (with or without specified endpoints) is also NP-complete.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all possible closed walks covering the required arcs and check feasibility + total length.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Held-Karp-style DP in O(n^2 · 2^n) time tracking (current vertex, covered arcs). The 9/5-approximation of Frederickson et al. is practical for larger instances.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (has feasible cycle):**
Mixed graph G with 6 vertices {0, 1, 2, 3, 4, 5}:
- Directed arcs A: (0,1), (2,3), (4,5) — 3 arcs, each length 2
- Undirected edges E: {1,2}, {3,4}, {5,0} — 3 edges, each length 1
- B = 9
- Feasible cycle: 0 →(arc,2)→ 1 →(edge,1)→ 2 →(arc,2)→ 3 →(edge,1)→ 4 →(arc,2)→ 5 →(edge,1)→ 0
- Total length: 3×2 + 3×1 = 9 = B ✓
- All 3 directed arcs traversed in correct direction ✓
- Answer: YES

**Instance 2 (no feasible cycle):**
Mixed graph G with 6 vertices {0, 1, 2, 3, 4, 5}:
- Directed arcs A: (0,1), (2,3), (4,5) — 3 arcs, each length 2
- Undirected edges E: {1,2}, {3,4} — 2 edges, each length 1 (no edge connecting 5 back to 0)
- B = 9
- No closed walk exists that covers all arcs: vertex 5 has no outgoing connection back to 0 or any other vertex.
- Answer: NO
