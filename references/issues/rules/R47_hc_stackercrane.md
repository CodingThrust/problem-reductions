---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to STACKER-CRANE"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'STACKER-CRANE'
source_in_codebase: false
target_in_codebase: false
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** STACKER-CRANE
**Motivation:** Establishes NP-completeness of STACKER-CRANE via polynomial-time reduction from HAMILTONIAN CIRCUIT. The Stacker-Crane problem generalizes the Traveling Salesman Problem to mixed graphs with mandatory directed arcs, and this reduction shows that even the unit-length case is intractable, motivating the study of approximation algorithms (e.g., the 9/5-approximation of Frederickson, Hecht, and Kim).
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND26, p.212

## GJ Source Entry

> [ND26] STACKER-CRANE
> INSTANCE: Mixed graph G=(V,A,E), length l(e)∈Z_0^+ for each e∈A∪E, bound B∈Z^+.
> QUESTION: Is there a cycle in G that includes each directed edge in A at least once, traversing such edges only in the specified direction, and that has total length no more than B?
> Reference: [Frederickson, Hecht, and Kim, 1978]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete even if all edge lengths equal 1. The analogous path problem (with or without specified endpoints) is also NP-complete.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HamiltonianCircuit instance G = (V, E) with n = |V| vertices and m = |E| edges, construct a Stacker-Crane instance on a mixed graph G' = (V', A, E') as follows:

1. **Vertex splitting:** For each vertex v_i ∈ V (i = 0, ..., n−1), create two vertices v_i^in and v_i^out. Set V' = {v_i^in, v_i^out : v_i ∈ V}, so |V'| = 2n.

2. **Directed arcs (required):** For each vertex v_i ∈ V, add a directed arc (v_i^in, v_i^out) with length 1. Set A = {(v_i^in, v_i^out) : v_i ∈ V}, so |A| = n. These arcs enforce that every vertex in the original graph is "visited."

3. **Undirected edges (optional routing):** For each edge {v_i, v_j} ∈ E, add two undirected edges: {v_i^out, v_j^in} and {v_j^out, v_i^in}, each with length 0. Set E' = {{v_i^out, v_j^in}, {v_j^out, v_i^in} : {v_i, v_j} ∈ E}, so |E'| = 2m.

4. **Bound:** Set B = n (total length budget equals the number of directed arcs, each of length 1).

**Correctness:**

- **(HC → SC):** If G has a Hamiltonian circuit v_{π(0)} → v_{π(1)} → ... → v_{π(n−1)} → v_{π(0)}, then in G' the cycle: v_{π(0)}^in → v_{π(0)}^out → v_{π(1)}^in → v_{π(1)}^out → ... → v_{π(n−1)}^in → v_{π(n−1)}^out → v_{π(0)}^in traverses all n directed arcs (cost 1 each) and n undirected edges (cost 0 each), for total length n = B.

- **(SC → HC):** If G' has a feasible Stacker-Crane cycle of total length ≤ B = n, it must traverse all n directed arcs (total cost n). Since the budget is exactly n, no additional directed arcs can be traversed, and only zero-cost undirected edges are used for routing. The undirected edges used connect consecutive vertex-splits, which corresponds to edges in the original graph G. The n vertices visited in order form a Hamiltonian circuit of G.

**Vertex count:** 2n
**Edge count (arcs + undirected):** n + 2m

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source HamiltonianCircuit instance (|V|)
- m = `num_edges` of source HamiltonianCircuit instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `2 * num_vertices` |
| `num_arcs` | `num_vertices` |
| `num_edges` | `2 * num_edges` |

**Derivation:**
- Vertices: each of the n original vertices is split into two (in/out) → 2n
- Directed arcs: one per original vertex (in → out) → n
- Undirected edges: two per original edge (one in each direction across the split) → 2m

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce a small HamiltonianCircuit instance G to Stacker-Crane G', solve G' with BruteForce, then verify that a feasible Stacker-Crane cycle (cost ≤ B) exists if and only if G has a Hamiltonian circuit.
- Test with a graph known to have a Hamiltonian circuit (e.g., a cycle C_6) and verify the SC instance has a feasible solution with cost B = 6.
- Test with a graph known to have no Hamiltonian circuit (e.g., the Petersen graph minus some edges, or a star K_{1,5}) and verify the SC instance has no feasible solution with cost B = n.
- Verify vertex and edge counts in G' match the formulas: |V'| = 2n, |A| = n, |E'| = 2m.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianCircuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges (triangular prism / prism graph):
- Edges: {0,1}, {1,2}, {2,0}, {3,4}, {4,5}, {5,3}, {0,3}, {1,4}, {2,5}
- n = 6, m = 9
- Hamiltonian circuit exists: 0 → 1 → 4 → 3 → 5 → 2 → 0

**Constructed target instance (Stacker-Crane):**
Mixed graph G' = (V', A, E'):
- V' = {0^in, 0^out, 1^in, 1^out, 2^in, 2^out, 3^in, 3^out, 4^in, 4^out, 5^in, 5^out} — 12 vertices
- A = {(0^in,0^out), (1^in,1^out), (2^in,2^out), (3^in,3^out), (4^in,4^out), (5^in,5^out)} — 6 directed arcs, each length 1
- E' = 18 undirected edges (2 per original edge), each length 0:
  - From {0,1}: {0^out,1^in}, {1^out,0^in}
  - From {1,2}: {1^out,2^in}, {2^out,1^in}
  - From {2,0}: {2^out,0^in}, {0^out,2^in}
  - From {3,4}: {3^out,4^in}, {4^out,3^in}
  - From {4,5}: {4^out,5^in}, {5^out,4^in}
  - From {5,3}: {5^out,3^in}, {3^out,5^in}
  - From {0,3}: {0^out,3^in}, {3^out,0^in}
  - From {1,4}: {1^out,4^in}, {4^out,1^in}
  - From {2,5}: {2^out,5^in}, {5^out,2^in}
- B = 6

**Solution mapping (Hamiltonian circuit 0→1→4→3→5→2→0):**
Stacker-Crane cycle:
  0^in →(arc, cost 1)→ 0^out →(edge, cost 0)→ 1^in →(arc, cost 1)→ 1^out →(edge, cost 0)→ 4^in →(arc, cost 1)→ 4^out →(edge, cost 0)→ 3^in →(arc, cost 1)→ 3^out →(edge, cost 0)→ 5^in →(arc, cost 1)→ 5^out →(edge, cost 0)→ 2^in →(arc, cost 1)→ 2^out →(edge, cost 0)→ 0^in

- Total cost: 6 × 1 (arcs) + 6 × 0 (edges) = 6 = B ✓
- All 6 directed arcs traversed in correct direction ✓
- All 12 vertices visited ✓


## References

- **[Frederickson, Hecht, and Kim, 1978]**: [`Frederickson1978`] G. N. Frederickson and M. S. Hecht and C. E. Kim (1978). "Approximation algorithms for some routing problems". *SIAM Journal on Computing* 7, pp. 178–193.
