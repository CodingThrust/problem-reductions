---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to BICONNECTIVITY AUGMENTATION"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'BICONNECTIVITY AUGMENTATION'
source_in_codebase: false
target_in_codebase: false
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** BICONNECTIVITY AUGMENTATION
**Motivation:** Establishes NP-completeness of the weighted BICONNECTIVITY AUGMENTATION problem via polynomial-time reduction from HAMILTONIAN CIRCUIT. The key insight of Eswaran and Tarjan (1976) is that finding a Hamiltonian cycle in a graph G is equivalent to finding a minimum-weight set of edges (from the complete graph on V with appropriate weights) that makes the edgeless graph biconnected, where edges of G have weight 1 and non-edges have weight 2.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND18, p.210

## GJ Source Entry

> [ND18] BICONNECTIVITY AUGMENTATION
> INSTANCE: Graph G=(V,E), weight w({u,v}) in Z^+ for each unordered pair {u,v} of vertices from V, positive integer B.
> QUESTION: Is there a set E' of unordered pairs of vertices from V such that sum_{e in E'} w(e) <= B and such that the graph G'=(V,E union E') is biconnected, i.e., cannot be disconnected by removing a single vertex?
> Reference: [Eswaran and Tarjan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: The related problem in which G' must be bridge-connected, i.e., cannot be disconnected by removing a single edge, is also NP-complete. Both problems remain NP-complete if all weights are either 1 or 2 and E is empty. Both can be solved in polynomial time if all weights are equal.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HamiltonianCircuit instance G = (V, E) with n = |V| vertices, construct a BiconnectivityAugmentation instance (G_empty, w, B) as follows:

1. **Initial graph:** Start with the edgeless graph G_empty = (V, empty set). The initial graph has no edges at all.

2. **Weight function:** For each unordered pair {u, v} of vertices from V, define the weight:
   - w({u, v}) = 1 if {u, v} in E (the edge exists in the original graph G)
   - w({u, v}) = 2 if {u, v} not in E (the edge does not exist in G)

3. **Budget parameter:** Set B = n (the number of vertices).

4. **Correctness (forward direction):** If G has a Hamiltonian circuit C visiting all n vertices, then E' = C (the set of n edges forming the circuit) makes G_empty union E' = C, which is a cycle on all n vertices. A cycle on n >= 3 vertices is biconnected (removing any single vertex leaves a path, which is connected). The total weight of E' is n * 1 = n = B since all edges of C are edges of G (weight 1 each).

5. **Correctness (reverse direction):** If there exists E' with sum(w(e)) <= B = n such that (V, E') is biconnected, then:
   - A biconnected graph on n vertices requires at least n edges.
   - Since each edge has weight >= 1 and the budget is n, E' has exactly n edges each of weight 1.
   - All edges in E' must be edges of G (since non-edges have weight 2, and using even one would require total weight >= n + 1 > B).
   - A biconnected graph on n vertices with exactly n edges is a Hamiltonian cycle.
   - Therefore E' is a Hamiltonian circuit of G.

6. **Solution extraction:** The set E' of added edges IS the Hamiltonian circuit of G.

**Key insight:** A biconnected graph on n vertices with exactly n edges must be a single cycle visiting all vertices (a Hamiltonian cycle). This is because a biconnected graph is 2-connected, and the only 2-connected graph with exactly n edges on n vertices is a cycle.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source HamiltonianCircuit instance (|V|)
- m = `num_edges` of source HamiltonianCircuit instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` (initial) | `0` |
| `num_potential_edges` | `num_vertices * (num_vertices - 1) / 2` |

**Derivation:**
- Vertices: same vertex set, no changes -> n
- Initial edges: 0 (edgeless graph)
- Potential edges (complete graph): n(n-1)/2 pairs, each with weight 1 or 2
- Budget: B = n

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a HamiltonianCircuit instance G to BiconnectivityAugmentation (G_empty, w, B=n), solve target with BruteForce (enumerate subsets of edges with total weight <= n, check biconnectivity), extract Hamiltonian circuit from the solution edges, verify it visits all vertices exactly once
- Test with a graph known to have a Hamiltonian circuit (e.g., complete graph K_n, cycle graph C_n) and verify the augmentation solution uses exactly n weight-1 edges
- Test with a graph known to NOT have a Hamiltonian circuit (e.g., Petersen graph for n=10 with modifications) and verify no valid augmentation exists within budget n
- Verify that any solution set E' with weight n forms a cycle on all n vertices

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianCircuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}, {0,3}, {1,4}, {2,5}
- (Prism graph / triangular prism)
- Known Hamiltonian circuit: 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0

**Constructed target instance (BiconnectivityAugmentation):**

Initial graph: edgeless on vertices {0, 1, 2, 3, 4, 5} (no edges).

Weight function for all 15 unordered pairs:
- Weight 1 (edges of G): {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}, {0,3}, {1,4}, {2,5}
- Weight 2 (non-edges of G): {0,2}, {0,4}, {1,3}, {1,5}, {2,4}, {3,5}

Budget: B = 6

**Solution mapping:**
- Choose E' = {{0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}} (the Hamiltonian circuit edges)
- Total weight = 6 * 1 = 6 = B
- Graph (V, E') forms the cycle 0-1-2-3-4-5-0 which is biconnected (removing any one vertex leaves a path on 5 vertices, which is connected)
- Extracted Hamiltonian circuit: 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0

**Verification that no cheaper solution exists:**
- A biconnected graph on 6 vertices needs >= 6 edges
- Each edge costs >= 1, so minimum cost >= 6 = B
- Any solution with cost 6 must use exactly 6 edges, each of weight 1 (from G)
- The only biconnected graph with 6 vertices and 6 edges is a 6-cycle = Hamiltonian circuit


## References

- **[Eswaran and Tarjan, 1976]**: [`Eswaran and Tarjan1976`] K. P. Eswaran and R. E. Tarjan (1976). "Augmentation problems". *SIAM Journal on Computing* 5, pp. 653-665.
