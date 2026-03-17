---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to LONGEST CIRCUIT"
labels: rule
assignees: ''
canonical_source_name: 'Hamiltonian Circuit'
canonical_target_name: 'Longest Circuit'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** LONGEST CIRCUIT
**Motivation:** Establishes NP-completeness of LONGEST CIRCUIT via polynomial-time reduction from HAMILTONIAN CIRCUIT. The reduction is trivial: assign unit weight to every edge and set the length bound K = |V|, so a simple circuit of length at least K exists if and only if a Hamiltonian circuit exists.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND28, p.213

## GJ Source Entry

> [ND28] LONGEST CIRCUIT
> INSTANCE: Graph G=(V,E), length l(e)∈Z^+ for each e∈E, positive integer K.
> QUESTION: Is there a simple circuit in G of length K or more, i.e., whose edge lengths sum to at least K?
> Reference: Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete if l(e)=1 for all e∈E, as does the corresponding problem for directed circuits in directed graphs. The directed problem with all l(e)=1 can be solved in polynomial time if G is a "tournament" [Morrow and Goodman, 1976]. The analogous directed and undirected problems, which ask for a simple circuit of length K or less, can be solved in polynomial time (e.g., see [Itai and Rodeh, 1977b]), but are NP-complete if negative lengths are allowed.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HAMILTONIAN CIRCUIT instance G = (V, E) with n = |V| vertices, construct a LONGEST CIRCUIT instance as follows:

1. **Graph:** Use the same graph G' = G = (V, E).

2. **Edge lengths:** For every edge e in E, set the length l(e) = 1 (unit weights).

3. **Bound:** Set K = n (the number of vertices).

4. **Correctness (forward):** If G has a Hamiltonian circuit C visiting all n vertices, then C is a simple circuit in G' with exactly n edges, each of length 1, so the total length is n = K. Thus the LONGEST CIRCUIT instance is a YES instance.

5. **Correctness (reverse):** If G' has a simple circuit of length >= K = n with unit-weight edges, then the circuit has at least n edges. Since the circuit is simple (no repeated vertices) and the graph has only n vertices, a simple circuit can have at most n edges. Therefore the circuit has exactly n edges and visits all n vertices -- it is a Hamiltonian circuit in G.

**Key invariant:** With unit weights, a simple circuit of length >= n exists if and only if the circuit visits all n vertices, i.e., it is Hamiltonian.

**Time complexity of reduction:** O(|E|) to assign unit weights.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source HamiltonianCircuit instance (|V|)
- m = `num_edges` of source HamiltonianCircuit instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_edges` |
| `bound` | `num_vertices` |

**Derivation:** The graph is unchanged. The bound K equals the number of vertices n. Each edge simply gets a unit length assigned.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a HamiltonianCircuit instance to LongestCircuit, solve target with BruteForce, extract solution, verify on source
- Test with known YES instance: the Petersen graph is Hamiltonian; the longest circuit instance with K = 10 should be satisfiable
- Test with known NO instance: the Petersen graph minus one vertex is not Hamiltonian; verify that no circuit of length >= 9 exists in that subgraph with the same construction
- Compare with known results from literature

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianCircuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}, {0,3}, {1,4}, {2,5}
- (Triangular prism / prism graph)
- Hamiltonian circuit exists: 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0

**Constructed target instance (LongestCircuit):**
- Same graph G' = G with 6 vertices and 9 edges
- Edge lengths: l(e) = 1 for all 9 edges
- Bound K = 6

**Solution mapping:**
- LongestCircuit solution: simple circuit 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 0
- Circuit length: 6 edges x 1 = 6 >= K = 6
- This circuit visits all 6 vertices exactly once, forming a Hamiltonian circuit in G

**Verification:**
- Forward: HC 0->1->2->3->4->5->0 maps to circuit of length 6 = K
- Reverse: any simple circuit of length >= 6 with 6 vertices must visit all vertices -> Hamiltonian circuit


## References

- **[Morrow and Goodman, 1976]**: [`Morrow1976`] C. Morrow and S. Goodman (1976). "An efficient algorithm for finding a longest cycle in a tournament". In: *Proceedings of the 7th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 453-462. Utilitas Mathematica Publishing.
- **[Itai and Rodeh, 1977b]**: [`Itai1977c`] Alon Itai and Michael Rodeh (1977). "Some matching problems". In: *Automata, Languages, and Programming*. Springer.
