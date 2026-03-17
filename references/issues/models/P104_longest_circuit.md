---
name: Problem
about: Propose a new problem type
title: "[Model] LongestCircuit"
labels: model
assignees: ''
---

## Motivation

LONGEST CIRCUIT (P104) from Garey & Johnson, A2 ND28. A classical NP-complete problem useful for reductions. Asks whether a graph contains a simple circuit whose total edge length meets or exceeds a given bound K. NP-complete even with unit edge lengths (where it reduces to finding a longest simple cycle).

<!-- ⚠️ Unverified: AI-collected rule associations -->
**Associated reduction rules:**
- **As source:** None found in the current rule set.
- **As target:** R49: HAMILTONIAN CIRCUIT -> LONGEST CIRCUIT

## Definition

**Name:** `LongestCircuit`
<!-- ⚠️ Unverified -->
**Canonical name:** LONGEST CIRCUIT (also: Longest Cycle, Maximum Weight Simple Cycle)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND28

**Mathematical definition:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, positive integer K.
QUESTION: Is there a simple circuit in G of length K or more, i.e., whose edge lengths sum to at least K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |E| binary variables (one per edge), indicating whether the edge is included in the circuit.
- **Per-variable domain:** {0, 1} -- edge is excluded or included in the circuit.
- **Meaning:** The variable assignment encodes a subset of edges. A satisfying assignment is a subset S of E such that the subgraph induced by S forms a simple circuit (connected 2-regular subgraph) and the sum of l(e) for e in S is at least K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `LongestCircuit`
**Variants:** graph type (G), weight type (W)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `G` | The undirected graph G = (V, E) |
| `lengths` | `Vec<W>` | Edge length l(e) for each edge (indexed by edge index) |
| `bound` | `W` | The length bound K |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- For the optimization variant, one would maximize the circuit length (removing the bound K), making it an `OptimizationProblem` with `Direction::Maximize`.
- The unit-weight special case (l(e) = 1 for all e) is equivalent to finding the longest simple cycle by number of edges.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(1.657^n) randomized time, via reduction to Hamiltonian Cycle detection. For the unit-weight case, finding the longest cycle is equivalent to finding the largest Hamiltonian subgraph, which can be approached via Bjorklund's algebraic method. For claw-free graphs specifically, O*(1.6818^n) time with exponential space, or O*(1.8878^n) with polynomial space (van 't Hof et al., 2011).
- **Classic algorithm:** O(n^2 * 2^n) deterministic dynamic programming via Held-Karp adaptation -- enumerate subsets, tracking cycle structure.
- **NP-completeness:** NP-complete by transformation from HAMILTONIAN CIRCUIT (Garey & Johnson, ND28). Remains NP-complete with unit edge lengths.
- **References:**
  - A. Bjorklund (2014). "Determinant Sums for Undirected Hamiltonicity." *SIAM Journal on Computing*, 43(1):280-299.
  - P. van 't Hof, D. Paulusma, G.J. Woeginger (2011). "Partitioning graphs into connected parts." *Algorithmica*.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, positive integer K.
QUESTION: Is there a simple circuit in G of length K or more, i.e., whose edge lengths sum to at least K?
Reference: Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete if l(e) = 1 for all e ∈ E, as does the corresponding problem for directed circuits in directed graphs. The directed problem with all l(e) = 1 can be solved in polynomial time if G is a "tournament" [Morrow and Goodman, 1976]. The analogous directed and undirected problems, which ask for a simple circuit of length K or less, can be solved in polynomial time (e.g., see [Itai and Rodeh, 1977b]), but are NP-complete if negative lengths are allowed.

## How to solve

- [x] It can be solved by (existing) bruteforce -- enumerate all subsets of edges and check if they form a simple circuit with total length >= K.
- [ ] It can be solved by reducing to integer programming.
- [x] Other: Held-Karp-style DP in O(n^2 * 2^n); Bjorklund's randomized algorithm in O*(1.657^n).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES -- circuit of length >= K exists):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 10 edges:
- Edges with lengths: {0,1}:3, {1,2}:2, {2,3}:4, {3,4}:1, {4,5}:5, {5,0}:2, {0,3}:3, {1,4}:2, {2,5}:1, {3,5}:2
- K = 17
- Simple circuit: 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0
  - Length: 3 + 2 + 4 + 1 + 5 + 2 = 17 >= K = 17
- Answer: YES

**Instance 2 (NO -- no circuit of length >= K):**
Same graph, K = 20.
- The Hamiltonian circuit above has length 17.
- Alternative circuit: 0 -> 3 -> 2 -> 5 -> 4 -> 1 -> 0: length = 3 + 4 + 1 + 5 + 2 + 3 = 18.
- No simple circuit can have length >= 20 (the maximum over all Hamiltonian circuits is 18).
- Answer: NO
