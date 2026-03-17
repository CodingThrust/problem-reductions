---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hamiltonian Circuit to Traveling Salesman Polytope Non-Adjacency"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'TRAVELING SALESMAN POLYTOPE NON-ADJACENCY'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** TRAVELING SALESMAN POLYTOPE NON-ADJACENCY
**Motivation:** Establishes NP-completeness of determining vertex non-adjacency on the TSP polytope, demonstrating that even the local geometric structure of the TSP polytope is computationally intractable -- a fundamental barrier to simplex-based LP approaches for solving TSP.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.246

## GJ Source Entry

> [MP8] TRAVELING SALESMAN POLYTOPE NON-ADJACENCY
> INSTANCE: Graph G = (V, E), two Hamiltonian circuits C and C' for G.
> QUESTION: Do C and C' correspond to non-adjacent vertices of the "traveling salesman polytope" for G?
> Reference: [Papadimitriou, 1978a]. Transformation from 3SAT.
> Comment: Result also holds for the "non-symmetric" case where G is a directed graph and C and C' are directed Hamiltonian circuits. Analogous polytope non-adjacency problems for graph matching and CLIQUE can be solved in polynomial time [Chvatal, 1975].

**Note on reduction chain:** GJ states the transformation is from 3SAT. The rule file title says "Hamiltonian Circuit to TSP Polytope Non-Adjacency" because Hamiltonian Circuit is an intermediate problem in the standard reduction chain (3SAT -> ... -> HC -> TSP Polytope Non-Adjacency). Papadimitriou's original paper proves NP-completeness via a construction that may go through 3SAT or directly encode satisfiability. The rule file captures the direct relationship described by GJ's reference structure.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a Hamiltonian Circuit instance G = (V, E) with |V| = n, construct a TSP Polytope Non-Adjacency instance (G', C, C') as follows:

1. **Embed G into a complete graph:** Construct the complete graph K_n on the same vertex set V. All edges of G are present in K_n, plus additional edges not in G.

2. **Construct two specific Hamiltonian circuits:** Papadimitriou's construction builds a graph G' and two specific tours C and C' in G' such that:
   - C and C' are both valid Hamiltonian circuits in G'.
   - The symmetric difference of C and C' encodes the structure of the original Hamiltonian Circuit instance G.
   - C and C' are non-adjacent on the TSP polytope of G' if and only if the original graph G has a Hamiltonian circuit.

3. **Encoding via symmetric difference:** The key idea is that two tours C and C' on the TSP polytope are non-adjacent if and only if their midpoint (chi_C + chi_{C'})/2 can be written as a convex combination of characteristic vectors of other tours. A sufficient condition is that the 4-regular multigraph formed by the symmetric difference of C and C' can be decomposed into two other Hamiltonian tours T1, T2 (i.e., admits a Hamiltonian decomposition).

4. **Graph construction details:**
   - Start with the source graph G = (V, E).
   - Construct G' by augmenting G with gadget vertices and edges that encode the graph structure.
   - Define C as a "canonical" Hamiltonian circuit through the gadgets.
   - Define C' as a modified circuit that differs from C on edges corresponding to the structure of G.
   - The symmetric difference of C and C' forms a 4-regular multigraph whose Hamiltonian decomposability is equivalent to the existence of a Hamiltonian circuit in G.

5. **Solution extraction:** If C and C' are non-adjacent (witnessed by tours T1, T2 decomposing the symmetric difference), the edges of T1 or T2 that correspond to original edges of G encode a Hamiltonian circuit in G. Conversely, a Hamiltonian circuit in G can be lifted to a Hamiltonian decomposition of the symmetric difference multigraph, proving non-adjacency.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of the source graph G (|V|)
- m = `num_edges` of the source graph G (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | O(n + m) -- original vertices plus gadget vertices for edge encoding |
| `num_edges` | O(n^2) -- G' is dense (possibly complete on the augmented vertex set) |
| `circuit1_length` | O(n + m) -- C visits all vertices of G' |
| `circuit2_length` | O(n + m) -- C' visits all vertices of G' |

**Derivation:**
- Papadimitriou's construction adds gadget vertices proportional to the number of edges in G to encode the graph structure within the tour difference.
- The resulting graph G' has O(n + m) vertices.
- Both constructed circuits C and C' are Hamiltonian in G', so their lengths equal the number of vertices in G'.
- The number of edges in G' is at most O((n + m)^2) but typically O(n^2 + m) depending on the specific gadget construction.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- **Closed-loop test:** Start from a Hamiltonian Circuit instance G; apply R159 to construct (G', C, C'); determine non-adjacency of C and C' on the TSP polytope of G' by brute-force enumeration of all Hamiltonian tours in G' and checking for a decomposition witness; verify the answer matches whether G has a Hamiltonian circuit.
- **Forward mapping:** Given a Hamiltonian circuit H in G, construct the two witnessing tours T1, T2 that decompose the symmetric difference of C and C', confirming non-adjacency.
- **Backward mapping:** Given a witness (T1, T2) for non-adjacency, extract the corresponding Hamiltonian circuit in G from the edges of T1 (or T2) that map back to edges of G.
- **Size verification:** Check that |V(G')| and |E(G')| match the overhead expressions.
- **Negative instance:** Test with the Petersen graph (no Hamiltonian circuit) and verify that C and C' are adjacent on the TSP polytope (no decomposition witness exists).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Hamiltonian Circuit):**

Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges (prism graph):
- Edges: {0,1}, {1,2}, {2,0}, {3,4}, {4,5}, {5,3}, {0,3}, {1,4}, {2,5}
- G has a Hamiltonian circuit: H = 0 -> 1 -> 4 -> 3 -> 5 -> 2 -> 0

**Constructed target instance (TSP Polytope Non-Adjacency):**

After applying Papadimitriou's construction, we obtain:
- Graph G' with O(6 + 9) = O(15) vertices (6 original + gadget vertices for each edge).
- Two specific Hamiltonian circuits C and C' in G' whose symmetric difference encodes G's structure.

**Verification using K_6 (simplified illustration):**

For a cleaner illustration, consider the problem directly on K_6. Take:
- C: 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0
  Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}
- C': 0 -> 2 -> 4 -> 1 -> 3 -> 5 -> 0
  Edges: {0,2}, {2,4}, {4,1}, {1,3}, {3,5}, {5,0}

Symmetric difference: {0,1}, {1,2}, {2,3}, {3,4}, {4,5} from C; {0,2}, {2,4}, {4,1}, {1,3}, {3,5} from C' (excluding the common edge {5,0}).

The symmetric difference edges form a 4-regular multigraph on vertices {0,1,2,3,4,5} (each vertex touched by exactly 4 of the symmetric difference edges, since each vertex had degree 2 in C and degree 2 in C', and the common edge contributes to two vertices).

If this multigraph decomposes into two Hamiltonian tours:
- T1: 0 -> 1 -> 3 -> 2 -> 4 -> 5 -> 0 (edges: {0,1}, {1,3}, {3,2}, {2,4}, {4,5}, {5,0})
- T2: 0 -> 2 -> 1 -> 4 -> 3 -> 5 -> 0 (edges: {0,2}, {2,1}, {1,4}, {4,3}, {3,5}, {5,0})

Then chi_C + chi_{C'} = chi_{T1} + chi_{T2}, proving C and C' are non-adjacent on the TSP polytope of K_6.

**Solution mapping:**
- Non-adjacency witness (T1, T2) exists => the original graph G has a Hamiltonian circuit.
- The Hamiltonian circuit in G can be read from the tour structure: edges of T1 (or T2) restricted to G give a Hamiltonian circuit.


## References

- **[Papadimitriou, 1978a]**: C. H. Papadimitriou (1978). "The adjacency relation on the traveling salesman polytope is NP-Complete." *Mathematical Programming* 14, pp. 312-324.
- **[Chvatal, 1975]**: V. Chvatal (1975). "On certain polytopes associated with graphs." *Journal of Combinatorial Theory (B)* 18, pp. 138-154.
