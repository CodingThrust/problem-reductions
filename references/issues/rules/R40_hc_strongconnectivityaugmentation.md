---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to STRONG CONNECTIVITY AUGMENTATION"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'STRONG CONNECTIVITY AUGMENTATION'
source_in_codebase: false
target_in_codebase: false
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** STRONG CONNECTIVITY AUGMENTATION
**Motivation:** Establishes NP-completeness of the weighted STRONG CONNECTIVITY AUGMENTATION problem via polynomial-time reduction from HAMILTONIAN CIRCUIT. The key insight of Eswaran and Tarjan (1976) is analogous to the biconnectivity reduction: finding a Hamiltonian cycle in an undirected graph G is equivalent to finding a minimum-weight set of directed arcs that makes the arc-less digraph strongly connected, where arcs corresponding to edges of G have weight 1 and other arcs have weight 2.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND19, p.211

## GJ Source Entry

> [ND19] STRONG CONNECTIVITY AUGMENTATION
> INSTANCE: Directed graph G=(V,A), weight w(u,v) in Z^+ for each ordered pair (u,v) in V x V, positive integer B.
> QUESTION: Is there a set A' of ordered pairs of vertices from V such that sum_{a in A'} w(a) <= B and such that the graph G'=(V,A union A') is strongly connected?
> Reference: [Eswaran and Tarjan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete if all weights are either 1 or 2 and A is empty. Can be solved in polynomial time if all weights are equal.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HamiltonianCircuit instance G = (V, E) with n = |V| vertices (undirected), construct a StrongConnectivityAugmentation instance (G_empty, w, B) as follows:

1. **Initial digraph:** Start with the arc-less directed graph G_empty = (V, empty set). No arcs at all.

2. **Weight function:** For each ordered pair (u, v) of distinct vertices from V, define the weight:
   - w(u, v) = 1 if {u, v} in E (the undirected edge exists in G)
   - w(u, v) = 2 if {u, v} not in E (the undirected edge does not exist in G)

   Note: both (u, v) and (v, u) get the same weight since the original graph is undirected.

3. **Budget parameter:** Set B = n (the number of vertices).

4. **Correctness (forward direction):** If G has a Hamiltonian circuit C = v_1 -> v_2 -> ... -> v_n -> v_1, orient it as a directed cycle: arcs (v_1, v_2), (v_2, v_3), ..., (v_n, v_1). This gives n directed arcs. The resulting digraph is a directed cycle on all n vertices, which is strongly connected. Each arc corresponds to an edge of G, so each has weight 1. Total weight = n = B.

5. **Correctness (reverse direction):** If there exists A' with sum(w(a)) <= B = n such that (V, A') is strongly connected, then:
   - A strongly connected digraph on n vertices requires at least n arcs.
   - Each arc has weight >= 1, so with budget n, there are exactly n arcs each of weight 1.
   - All arcs in A' correspond to edges of G (non-edges have weight 2).
   - A strongly connected digraph on n vertices with exactly n arcs must be a directed Hamiltonian cycle (a single directed cycle visiting every vertex exactly once).
   - The underlying undirected edges of A' form a Hamiltonian circuit of G.

6. **Solution extraction:** Take the set of arcs A', ignore orientations to get undirected edges. These edges form a Hamiltonian circuit of G.

**Key insight:** A strongly connected digraph on n vertices with exactly n arcs must be a single directed cycle (since every vertex needs in-degree >= 1 and out-degree >= 1, and n arcs among n vertices with these constraints forces a single cycle). This directed cycle, when ignoring orientations, gives a Hamiltonian circuit.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source HamiltonianCircuit instance (|V|)
- m = `num_edges` of source HamiltonianCircuit instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_arcs` (initial) | `0` |
| `num_potential_arcs` | `num_vertices * (num_vertices - 1)` |

**Derivation:**
- Vertices: same vertex set, no changes -> n
- Initial arcs: 0 (arc-less digraph)
- Potential arcs (all ordered pairs): n(n-1) pairs, each with weight 1 or 2
- Budget: B = n

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a HamiltonianCircuit instance G to StrongConnectivityAugmentation (G_empty, w, B=n), solve target with BruteForce (enumerate subsets of arcs with total weight <= n, check strong connectivity), extract Hamiltonian circuit by ignoring arc orientations, verify it visits all vertices exactly once
- Test with a graph known to have a Hamiltonian circuit (e.g., cycle C_n, complete graph K_n) and verify the augmentation solution uses exactly n weight-1 arcs
- Test with a graph known to NOT have a Hamiltonian circuit and verify no valid augmentation within budget n exists
- Verify that any solution arc set A' with weight n forms a directed cycle on all n vertices

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianCircuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}, {0,3}, {1,4}, {2,5}
- (Prism graph / triangular prism)
- Known Hamiltonian circuit: 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0

**Constructed target instance (StrongConnectivityAugmentation):**

Initial digraph: arc-less on vertices {0, 1, 2, 3, 4, 5} (no arcs).

Weight function for all 30 ordered pairs (u, v) where u != v:
- Weight 1 (ordered pairs where {u,v} in E): (0,1),(1,0), (1,2),(2,1), (2,3),(3,2), (3,4),(4,3), (4,5),(5,4), (5,0),(0,5), (0,3),(3,0), (1,4),(4,1), (2,5),(5,2) -- 18 ordered pairs
- Weight 2 (ordered pairs where {u,v} not in E): (0,2),(2,0), (0,4),(4,0), (1,3),(3,1), (1,5),(5,1), (2,4),(4,2), (3,5),(5,3) -- 12 ordered pairs

Budget: B = 6

**Solution mapping:**
- Choose A' = {(0,1), (1,2), (2,3), (3,4), (4,5), (5,0)} (directed Hamiltonian cycle)
- Total weight = 6 * 1 = 6 = B
- Digraph (V, A') is the directed cycle 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0, which is strongly connected
- Extracted Hamiltonian circuit (undirected): {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}

**Verification that no cheaper solution exists:**
- A strongly connected digraph on 6 vertices needs >= 6 arcs
- Each arc costs >= 1, so minimum cost >= 6 = B
- Any solution with cost 6 must use exactly 6 arcs of weight 1 (edges of G)
- The only strongly connected digraph with 6 vertices and 6 arcs is a directed 6-cycle = Hamiltonian circuit


## References

- **[Eswaran and Tarjan, 1976]**: [`Eswaran and Tarjan1976`] K. P. Eswaran and R. E. Tarjan (1976). "Augmentation problems". *SIAM Journal on Computing* 5, pp. 653-665.
