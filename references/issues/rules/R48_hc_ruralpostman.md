---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to RURAL POSTMAN"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'RURAL POSTMAN'
source_in_codebase: false
target_in_codebase: false
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** RURAL POSTMAN
**Motivation:** Establishes NP-completeness of RURAL POSTMAN via polynomial-time reduction from HAMILTONIAN CIRCUIT. The Rural Postman problem is a fundamental arc-routing problem generalizing both the Chinese Postman Problem (polynomial when E' = E) and the Traveling Salesman Problem (when E' = ∅ with required vertices). This reduction shows that selecting which edges to traverse — even with unit lengths — is inherently intractable.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND27, p.213

## GJ Source Entry

> [ND27] RURAL POSTMAN
> INSTANCE: Graph G=(V,E), length l(e)∈Z_0^+ for each e∈E, subset E'⊆E, bound B∈Z^+.
> QUESTION: Is there a circuit in G that includes each edge in E' and that has total length no more than B?
> Reference: [Lenstra and Rinnooy Kan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete even if l(e)=1 for all e∈E, as does the corresponding problem for directed graphs.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HamiltonianCircuit instance G = (V, E) with n = |V| vertices and m = |E| edges, construct a Rural Postman instance (G', E'_req, l, B) as follows:

1. **Vertex splitting:** For each vertex v_i ∈ V (i = 0, ..., n−1), create two vertices v_i^a and v_i^b. Set V' = {v_i^a, v_i^b : v_i ∈ V}, so |V'| = 2n.

2. **Required edges (vertex-representing):** For each vertex v_i ∈ V, add an edge {v_i^a, v_i^b} with length 1. These form the required edge set: E'_req = {{v_i^a, v_i^b} : v_i ∈ V}, so |E'_req| = n. Each required edge enforces that the corresponding original vertex must be "visited."

3. **Connectivity edges (edge-representing):** For each edge {v_i, v_j} ∈ E, add edges {v_i^b, v_j^a} and {v_j^b, v_i^a}, each with length 0. These optional routing edges allow the circuit to move between vertex gadgets. Total: 2m connectivity edges.

4. **Overall graph:** G' = (V', E') where E' = E'_req ∪ E'_conn, |E'| = n + 2m.

5. **Bound:** Set B = n.

**Correctness:**

- **(HC → RPP):** If G has a Hamiltonian circuit v_{π(0)} → v_{π(1)} → ... → v_{π(n−1)} → v_{π(0)}, construct a Rural Postman circuit in G': v_{π(0)}^a → v_{π(0)}^b → v_{π(1)}^a → v_{π(1)}^b → ... → v_{π(n−1)}^a → v_{π(n−1)}^b → v_{π(0)}^a. This circuit traverses all n required edges (cost 1 each) and n zero-cost connectivity edges, for total length n = B.

- **(RPP → HC):** If G' has a circuit including all required edges with total length ≤ B = n, it traverses at least n required edges (cost n). Since the budget is exactly n, the circuit uses no additional cost-1 edges (i.e., each required edge is traversed exactly once). Between consecutive required edges, only zero-cost connectivity edges are used. The sequence of vertex-gadgets visited, in order, gives a Hamiltonian circuit in G: each vertex is visited exactly once, and consecutive vertices correspond to edges in the original graph.

**Vertex count:** 2n
**Edge count:** n + 2m

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source HamiltonianCircuit instance (|V|)
- m = `num_edges` of source HamiltonianCircuit instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `2 * num_vertices` |
| `num_edges` | `num_vertices + 2 * num_edges` |
| `num_required_edges` | `num_vertices` |

**Derivation:**
- Vertices: each of the n original vertices is split into two → 2n
- Edges: n required edges (one per vertex gadget) + 2m connectivity edges (two per original edge) → n + 2m
- Required edges: one per original vertex → n

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce a small HamiltonianCircuit instance G to Rural Postman (G', E'_req, l, B), solve G' with BruteForce, then verify that a feasible Rural Postman circuit (cost ≤ B covering all required edges) exists if and only if G has a Hamiltonian circuit.
- Test with a graph known to have a Hamiltonian circuit (e.g., cycle C_6) and verify the RPP instance has a feasible solution with cost B = 6.
- Test with a graph known to have no Hamiltonian circuit (e.g., K_{1,5} with some extra edges but no HC) and verify no RPP solution of cost ≤ B exists.
- Verify vertex and edge counts: |V'| = 2n, |E'| = n + 2m, |E'_req| = n.
- Verify that each required edge is traversed exactly once in any optimal solution (no budget for repeats).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianCircuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges (triangular prism / prism graph):
- Edges: {0,1}, {1,2}, {2,0}, {3,4}, {4,5}, {5,3}, {0,3}, {1,4}, {2,5}
- n = 6, m = 9
- Hamiltonian circuit exists: 0 → 1 → 4 → 3 → 5 → 2 → 0

**Constructed target instance (Rural Postman):**
Graph G' = (V', E'):
- V' = {0^a, 0^b, 1^a, 1^b, 2^a, 2^b, 3^a, 3^b, 4^a, 4^b, 5^a, 5^b} — 12 vertices
- Required edges E'_req (length 1 each):
  - {0^a, 0^b}, {1^a, 1^b}, {2^a, 2^b}, {3^a, 3^b}, {4^a, 4^b}, {5^a, 5^b} — 6 edges
- Connectivity edges (length 0 each):
  - From {0,1}: {0^b, 1^a}, {1^b, 0^a}
  - From {1,2}: {1^b, 2^a}, {2^b, 1^a}
  - From {2,0}: {2^b, 0^a}, {0^b, 2^a}
  - From {3,4}: {3^b, 4^a}, {4^b, 3^a}
  - From {4,5}: {4^b, 5^a}, {5^b, 4^a}
  - From {5,3}: {5^b, 3^a}, {3^b, 5^a}
  - From {0,3}: {0^b, 3^a}, {3^b, 0^a}
  - From {1,4}: {1^b, 4^a}, {4^b, 1^a}
  - From {2,5}: {2^b, 5^a}, {5^b, 2^a}
  — 18 connectivity edges
- Total: 6 + 18 = 24 edges
- B = 6

**Solution mapping (Hamiltonian circuit 0→1→4→3→5→2→0):**
Rural Postman circuit:
  0^a →(required, cost 1)→ 0^b →(conn, cost 0)→ 1^a →(required, cost 1)→ 1^b →(conn, cost 0)→ 4^a →(required, cost 1)→ 4^b →(conn, cost 0)→ 3^a →(required, cost 1)→ 3^b →(conn, cost 0)→ 5^a →(required, cost 1)→ 5^b →(conn, cost 0)→ 2^a →(required, cost 1)→ 2^b →(conn, cost 0)→ 0^a

- Total cost: 6 × 1 (required) + 6 × 0 (connectivity) = 6 = B ✓
- All 6 required edges traversed ✓
- All 12 vertices visited ✓


## References

- **[Lenstra and Rinnooy Kan, 1976]**: [`Lenstra1976`] Jan K. Lenstra and A. H. G. Rinnooy Kan (1976). "On general routing problems". *Networks* 6, pp. 273–280.
