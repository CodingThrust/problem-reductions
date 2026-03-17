---
name: Problem
about: Propose a new problem type
title: "[Model] RuralPostman"
labels: model
assignees: ''
---

## Motivation

RURAL POSTMAN (P103) from Garey & Johnson, A2 ND27. A fundamental NP-complete arc-routing problem that generalizes the Chinese Postman Problem (polynomial when all edges are required) and is closely related to the Traveling Salesman Problem. It models the practical scenario of a delivery agent who must traverse a specified subset of roads in a network while minimizing total travel distance. The problem is a target of the following reduction:

<!-- ⚠️ Unverified: AI-collected rule associations -->
- **R48: HAMILTONIAN CIRCUIT → RURAL POSTMAN** (ND27, Lenstra and Rinnooy Kan, 1976)

## Definition

**Name:** `RuralPostman`
<!-- ⚠️ Unverified -->
**Canonical name:** RURAL POSTMAN (also: Rural Postman Problem, RPP)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND27

**Mathematical definition:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z_0^+ for each e ∈ E, subset E' ⊆ E, bound B ∈ Z^+.
QUESTION: Is there a circuit in G that includes each edge in E' and that has total length no more than B?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** The circuit is a closed walk that covers all required edges E'. For a graph with |V| = n vertices and |E'| = r required edges, the solution is a sequence of edge traversals. In a DP formulation: O(n · 2^r) states (current vertex, set of required edges covered).
- **Per-variable domain:** Each step in the sequence selects the next edge to traverse from the current vertex. In a DP formulation: (current vertex, subset of E' already traversed).
- **Meaning:** The variable assignment encodes a closed walk (circuit) in the graph. A satisfying assignment is a circuit that (1) traverses every edge in E' at least once, and (2) has total length ≤ B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `RuralPostman`
**Variants:** graph topology (graph type parameter G)

| Field | Type | Description |
|-------|------|-------------|
| `graph` | `SimpleGraph` | The undirected graph G = (V, E) |
| `edge_lengths` | `Vec<i32>` | Length l(e) for each edge e ∈ E (non-negative integers) |
| `required_edges` | `Vec<(usize, usize)>` | The subset E' ⊆ E of required edges |
| `bound` | `i32` | Upper bound B on total circuit length |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- When E' = E, the problem reduces to the Chinese Postman Problem (polynomial-time solvable via T-join / matching).
- When E' = ∅ and required vertices are specified instead, the problem reduces to the Traveling Salesman Problem.
- Remains NP-complete with unit edge lengths (l(e) = 1 for all e).
- The directed variant is also NP-complete (GJ comment).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** Dynamic programming over subsets of required edges: O(n^2 · 2^r) where r = |E'| is the number of required edges and n = |V|. The DP state tracks (current vertex, subset of required edges covered so far), with transitions via shortest paths between required-edge endpoints. When r = |E| (Chinese Postman), the problem is polynomial. For general r, the problem is strongly NP-hard.
- **Parameterized algorithm:** O(4^d · n^3) time where d = |W*| − |R| is the number of deadheading edges in an optimal solution (Sorge et al., 2012).
- **NP-completeness:** NP-complete (Lenstra and Rinnooy Kan, 1976; originally in "On general routing problems").
- **Approximation:** The RPP can be approximated within a factor of 3/2 using Christofides-type approaches for metric instances.
- **References:**
  - J. K. Lenstra and A. H. G. Rinnooy Kan (1976). "On general routing problems." *Networks* 6:273–280.
  - A. Corberán and G. Laporte (eds.) (2015). *Arc Routing: Problems, Methods, and Applications*. SIAM MOS-SIAM Series on Optimization.
  - M. Sorge, R. van Bevern, R. Niedermeier, M. Weller (2012). "A new view on Rural Postman based on Eulerian Extension and Matching." *Journal of Discrete Algorithms* 16:12–33.

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z_0^+ for each e ∈ E, subset E' ⊆ E, bound B ∈ Z^+.
QUESTION: Is there a circuit in G that includes each edge in E' and that has total length no more than B?
Reference: [Lenstra and Rinnooy Kan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete even if l(e) = 1 for all e ∈ E, as does the corresponding problem for directed graphs.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all possible closed walks covering the required edges and check feasibility + total length.
- [x] It can be solved by reducing to integer programming — standard ILP formulation with binary edge-usage variables and subtour elimination constraints.
- [x] Other: DP over subsets of required edges in O(n^2 · 2^r) time; branch-and-cut algorithms using Eulerian extension formulations; 3/2-approximation for metric instances.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (has feasible circuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 8 edges:
- Edges with lengths: {0,1}:1, {1,2}:1, {2,3}:1, {3,4}:1, {4,5}:1, {5,0}:1, {0,3}:2, {1,4}:2
- Required edges E' = {{0,1}, {2,3}, {4,5}} (3 required edges)
- B = 6
- Feasible circuit: 0 →{0,1}:1→ 1 →{1,2}:1→ 2 →{2,3}:1→ 3 →{3,4}:1→ 4 →{4,5}:1→ 5 →{5,0}:1→ 0
- Total length: 6 × 1 = 6 = B ✓
- All 3 required edges traversed ✓
- Answer: YES

**Instance 2 (no feasible circuit with given bound):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges with lengths: {0,1}:1, {1,2}:1, {2,3}:1, {3,0}:1, {3,4}:3, {4,5}:1, {5,3}:3
- Required edges E' = {{0,1}, {4,5}} (2 required edges)
- B = 4
- To traverse both {0,1} and {4,5}, the circuit must travel from the {0,1,2,3} component to vertex 4 and back. The shortest path from any endpoint of {0,1} to vertex 4 goes through vertex 3 via edge {3,4} of length 3. Minimum circuit cost: 1 (for {0,1}) + path to 4 (length ≥ 3) + 1 (for {4,5}) + path back (length ≥ 3) = 8 > 4 = B.
- Answer: NO

**Instance 3 (Chinese Postman special case, E' = E):**
Graph G with 4 vertices {0, 1, 2, 3} and 4 edges (cycle C_4):
- Edges with lengths: {0,1}:1, {1,2}:1, {2,3}:1, {3,0}:1
- Required edges E' = {{0,1}, {1,2}, {2,3}, {3,0}} (all edges required)
- B = 4
- Circuit: 0 → 1 → 2 → 3 → 0, total length 4 = B ✓
- This is the Chinese Postman special case: an Eulerian circuit exists since all vertices have even degree.
- Answer: YES
