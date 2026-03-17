---
name: Problem
about: Propose a new problem type
title: "[Model] HamiltonianCircuit"
labels: model
assignees: ''
---

## Motivation

HAMILTONIAN CIRCUIT (P48) from Garey & Johnson, A1.3 GT37. A classical NP-complete problem (Karp, 1972) central to the theory of NP-completeness. It is a fundamental source problem for reductions to HAMILTONIAN PATH, TRAVELING SALESMAN, and many other combinatorial problems.

## Definition

**Name:** `HamiltonianCircuit`
**Canonical name:** HAMILTONIAN CIRCUIT (also: Hamiltonian Cycle)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT37

**Mathematical definition:**

INSTANCE: Graph G = (V, E).
QUESTION: Does G contain a Hamiltonian circuit, i.e., a closed path that visits every vertex exactly once?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** n = |V| binary variables (one per vertex), interpreted as a permutation; or equivalently n! orderings checked, but the canonical BFS/DP encoding uses n × 2^n states.
- **Per-variable domain:** For a permutation-encoding: position index ∈ {0, 1, ..., n−1} (which vertex occupies slot i in the circuit). For a bitmask-DP encoding: (current vertex, visited subset) pairs.
- **Meaning:** The variable assignment encodes the order in which vertices are visited. A satisfying assignment is a permutation π such that {π(i), π(i+1)} ∈ E for all i and {π(n−1), π(0)} ∈ E.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `HamiltonianCircuit`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph on which a Hamiltonian circuit is sought |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- No weight type is needed (the question is purely structural).
- Specializations: planar graphs, bipartite graphs, cubic graphs (all remain NP-complete per GJ comments).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(1.657^n) randomized time via the "Determinant Sums" Monte Carlo algorithm (Björklund, 2010/2014). For bipartite graphs this improves to O*(1.415^n) ≈ O*(√2^n). For graphs of maximum degree 3 a backtracking search achieves O*(1.251^n).
- **Classic algorithm:** O(n^2 · 2^n) deterministic dynamic programming (Held-Karp / Bellman, 1962) — this is the standard reference complexity used for the general case.
- **NP-completeness:** NP-complete (Karp, 1972, "Reducibility Among Combinatorial Problems").
- **References:**
  - R.M. Karp (1972). "Reducibility Among Combinatorial Problems." *Complexity of Computer Computations*, pp. 85–103. Plenum Press.
  - M. Held and R.M. Karp (1962). "A dynamic programming approach to sequencing problems." *Journal of the Society for Industrial and Applied Mathematics*, 10(1):196–210.
  - A. Björklund (2014). "Determinant Sums for Undirected Hamiltonicity." *SIAM Journal on Computing*, 43(1):280–299. [arXiv:1008.0541]

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E).
QUESTION: Does G contain a Hamiltonian circuit?

Reference: [Karp, 1972]. Transformation from VERTEX COVER (see Chapter 3).
Comment: Remains NP-complete (1) if G is planar, cubic, 3-connected, and has no face with fewer than 5 edges [Garey, Johnson, and Tarjan, 1976a], (2) if G is bipartite [Krishnamoorthy, 1975], (3) if G is the square of a graph [Chvátal, 1976], and (4) if a Hamiltonian path for G is given as part of the instance [Papadimitriou and Stieglitz, 1976]. Solvable in polynomial time if G has no vertex with degree exceeding 2 or if G is an edge graph (e.g., see [Liu, 1968]). The cube of a non-trivial connected graph always has a Hamiltonian circuit [Karaganis, 1968].

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all permutations of vertices and check if consecutive pairs (and wrap-around) are edges.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Held-Karp dynamic programming in O(n^2 · 2^n) time and O(n · 2^n) space; Björklund's randomized algorithm in O*(1.657^n).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (has Hamiltonian circuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}, {0,3}, {1,4}, {2,5}
- (Prism graph / triangular prism: two triangles {0,1,2} and {3,4,5} with matching edges)
- Hamiltonian circuit exists: 0 → 1 → 2 → 5 → 4 → 3 → 0
- Answer: YES

**Instance 2 (no Hamiltonian circuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {0,3}, {0,4}, {0,5}, {1,2}, {3,4}
- (Star K_{1,5} plus two extra chords — vertex 0 has degree 5, but the graph is not "balanced")
- Any Hamiltonian circuit must alternate between vertex 0 and others, but after leaving 0 to reach 1, returning requires using 0 again — impossible to visit all without revisiting 0.
- Answer: NO
