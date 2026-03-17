---
name: Problem
about: Propose a new problem type
title: "[Model] HamiltonianPath"
labels: model
assignees: ''
---

## Motivation

HAMILTONIAN PATH (P50) from Garey & Johnson, A1.3 GT39. A classical NP-complete decision problem closely related to HAMILTONIAN CIRCUIT. It is used as a source problem in reductions to TSP and other path/tour-finding problems, and arises naturally when the circuit closing-edge constraint is dropped.

## Definition

**Name:** `HamiltonianPath`
**Canonical name:** HAMILTONIAN PATH
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT39

**Mathematical definition:**

INSTANCE: Graph G = (V, E).
QUESTION: Does G contain a Hamiltonian path, i.e., a simple path that visits every vertex exactly once?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| binary variables, interpreted as a permutation of vertices (like HC but without a closing edge requirement).
- **Per-variable domain:** For a permutation-encoding: position index ∈ {0, 1, ..., n−1}. For bitmask-DP: (current vertex, visited subset) pairs over n · 2^n states.
- **Meaning:** The variable assignment encodes the visitation order. A satisfying assignment is a permutation π such that {π(i), π(i+1)} ∈ E for all i = 0, ..., n−2. No wrap-around edge is required (contrast with HC).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `HamiltonianPath`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph on which a Hamiltonian path is sought |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed.
- Variant: HAMILTONIAN PATH BETWEEN TWO POINTS adds specified endpoints u, v as part of the instance (also NP-complete per GJ).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(1.657^n) randomized time via Björklund's "Determinant Sums" Monte Carlo algorithm (2010/2014), same as for Hamiltonian Circuit. For bipartite graphs: O*(1.415^n) ≈ O*(√2^n). For graphs of maximum degree 3: O*(1.251^n) via backtracking.
- **Classic algorithm:** O(n^2 · 2^n) deterministic dynamic programming (Bellman 1962, Held-Karp 1962) — solve by checking all (start vertex, subset) pairs.
- **NP-completeness:** NP-complete (Karp, 1972; and via GJ Section 3.1.4 reduction from VC).
- **References:**
  - R.M. Karp (1972). "Reducibility Among Combinatorial Problems." *Complexity of Computer Computations*, pp. 85–103. Plenum Press.
  - M. Held and R.M. Karp (1962). "A dynamic programming approach to sequencing problems." *Journal of the Society for Industrial and Applied Mathematics*, 10(1):196–210.
  - A. Björklund (2014). "Determinant Sums for Undirected Hamiltonicity." *SIAM Journal on Computing*, 43(1):280–299. [arXiv:1008.0541]

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E).
QUESTION: Does G contain a Hamiltonian path?

Reference: Transformation from VERTEX COVER (see Chapter 3).
Comment: Remains NP-complete under restrictions (1) and (2) for HAMILTONIAN CIRCUIT and is polynomially solvable under the same restrictions as HC. Corresponding DIRECTED HAMILTONIAN PATH problem is also NP-complete, and the comments for DIRECTED HC apply to it as well. The variants in which either the starting point or the ending point or both are specified in the instance are also NP-complete. DIRECTED HAMILTONIAN PATH can be solved in polynomial time for acyclic digraphs, e.g., see [Lawler, 1976a].

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all permutations of vertices and check if consecutive pairs are edges (no wrap-around check needed vs HC).
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Held-Karp DP in O(n^2 · 2^n) time; Björklund's randomized algorithm in O*(1.657^n). Also reducible to Hamiltonian Circuit (add a new vertex adjacent to all others, then check HC on the augmented graph).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (has Hamiltonian path but no Hamiltonian circuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 5 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}
- (Simple path graph P_6)
- Hamiltonian path: 0 → 1 → 2 → 3 → 4 → 5 ✓
- No Hamiltonian circuit (no edge {5,0}) ✗
- Answer: YES

**Instance 2 (has Hamiltonian path, non-trivial):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 8 edges:
- Edges: {0,1}, {0,2}, {1,3}, {2,3}, {3,4}, {3,5}, {4,2}, {5,1}
- Hamiltonian path: 0 → 2 → 4 → 3 → 1 → 5 — check: {0,2}✓, {2,4}✓, {4,3}✓, {3,1}✓, {1,5}✓
- Answer: YES

**Instance 3 (no Hamiltonian path):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 6 edges:
- Edges: {0,1}, {0,2}, {0,3}, {1,2}, {1,3}, {2,3}
- (K_4 on vertices {0,1,2,3} plus two isolated vertices {4,5})
- Vertices 4 and 5 are isolated — no path can reach them
- Answer: NO
