---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to BOTTLENECK TRAVELING SALESMAN"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'BOTTLENECK TRAVELING SALESMAN'
source_in_codebase: false
target_in_codebase: false
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** BOTTLENECK TRAVELING SALESMAN
**Motivation:** Establishes NP-completeness of BOTTLENECK TRAVELING SALESMAN via polynomial-time reduction from HAMILTONIAN CIRCUIT. The reduction mirrors the classic HC → TSP reduction but uses the bottleneck (max-edge) objective instead of total length. This shows that even the min-max variant of TSP is intractable, and that restricting distances to {1, 2} does not help.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND24, p.212

## GJ Source Entry

> [ND24] BOTTLENECK TRAVELING SALESMAN
> INSTANCE: Set C of m cities, distance d(c_i,c_j)∈Z^+ for each pair of cities c_i,c_j∈C, positive integer B.
> QUESTION: Is there a tour of C whose longest edge is no longer than B, i.e., a permutation <c_{π(1)},c_{π(2)},...,c_{π(m)}> of C such that d(c_{π(i)},c_{π(i+1)})≤B for 1≤i<m and such that d(c_{π(m)},c_{π(1)})≤B?
> Reference: Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete even if d(c_i,c_j)∈{1,2} for all c_i,c_j∈C. An important special case that is solvable in polynomial time can be found in [Gilmore and Gomory, 1964].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HamiltonianCircuit instance G = (V, E) with n = |V| vertices, construct a BottleneckTravelingSalesman instance as follows:

1. **City set:** The set of cities C is exactly the vertex set V, so m = n cities.

2. **Distance matrix:** Construct the complete graph K_n on V with distances:
   - d(u, v) = 1 if {u, v} ∈ E (the edge exists in G)
   - d(u, v) = 2 if {u, v} ∉ E (the edge does not exist in G)

3. **Bottleneck bound:** Set B = 1.

4. **Correctness (forward):** If G has a Hamiltonian circuit v_1 → v_2 → ... → v_n → v_1, then every consecutive pair in the tour is an edge in G and has distance 1. The bottleneck (maximum edge weight) of this tour is 1 ≤ B = 1.

5. **Correctness (reverse):** If there is a tour with bottleneck ≤ B = 1, then every edge in the tour has distance ≤ 1. Since all distances are either 1 or 2, every tour edge must have distance exactly 1, meaning it corresponds to an edge in G. Therefore the tour visits every vertex using only edges of G — it is a Hamiltonian circuit in G.

**Key invariant:** A tour has bottleneck 1 if and only if it uses only edges of weight 1, which are exactly the edges of G. Hence a tour with bottleneck ≤ 1 exists iff G has a Hamiltonian circuit.

**Time complexity of reduction:** O(n^2) to construct the distance matrix.

**Note:** This reduction is essentially identical to the HC → TSP reduction, with the only difference being the objective (max-edge instead of sum) and the bound (B = 1 instead of B = n). The NP-completeness proof works for the same reason: the {1, 2}-distance construction forces a YES answer iff the original graph is Hamiltonian.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source HamiltonianCircuit instance (|V|)
- m = `num_edges` of source HamiltonianCircuit instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_cities` | `num_vertices` |
| `num_edges` (in complete graph) | `num_vertices * (num_vertices - 1) / 2` |
| `bound` | `1` |

**Derivation:** The Bottleneck TSP instance has the same number of cities as vertices (n). The distance matrix represents a complete graph with n(n-1)/2 entries. The bound is the constant 1.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce a small HamiltonianCircuit instance G to BottleneckTSP, solve with BruteForce (enumerate all permutations, check if max edge weight ≤ 1), verify that a solution exists iff G has a Hamiltonian circuit.
- Test with known YES instance: C_6 (6-cycle) has a Hamiltonian circuit. The Bottleneck TSP instance should have a tour with bottleneck = 1.
- Test with known NO instance: K_{2,3} plus an isolated vertex — no HC exists, so no tour with bottleneck ≤ 1 should exist (at least one distance-2 edge is forced).
- Verify all distances are in {1, 2} and the matrix is symmetric.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianCircuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}, {0,3}, {1,4}, {2,5}
- (Triangular prism / prism graph)
- Hamiltonian circuit exists: 0 → 1 → 2 → 3 → 4 → 5 → 0

**Constructed target instance (BottleneckTravelingSalesman):**
- 6 cities: {0, 1, 2, 3, 4, 5}
- Distance matrix (symmetric):
```
     0  1  2  3  4  5
  0: -  1  2  1  2  1
  1: 1  -  1  2  1  2
  2: 2  1  -  1  2  1
  3: 1  2  1  -  1  2
  4: 2  1  2  1  -  1
  5: 1  2  1  2  1  -
```
- Bottleneck bound B = 1

**Solution mapping:**
- Tour: 0 → 1 → 2 → 3 → 4 → 5 → 0
- Edge weights: d(0,1)=1, d(1,2)=1, d(2,3)=1, d(3,4)=1, d(4,5)=1, d(5,0)=1
- Bottleneck: max(1,1,1,1,1,1) = 1 ≤ B = 1 ✓
- All tour edges have distance 1, confirming they are edges in G → Hamiltonian circuit in G ✓

**Negative example (no Hamiltonian circuit):**
Graph G' with 6 vertices {0,1,2,3,4,5} and 5 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5} (path P_6, no edge {5,0})
- No Hamiltonian circuit (path cannot close into a cycle)
- Bottleneck TSP instance: d(5,0) = 2. Any tour must include at least one non-edge pair, forcing bottleneck ≥ 2 > B = 1.
- Answer: NO ✓


## References

- **[Gilmore and Gomory, 1964]**: [`Gilmore1964`] P. C. Gilmore and R. E. Gomory (1964). "Sequencing a one state-variable machine: a solvable case of the traveling salesman problem". *Operations Research* 12, pp. 655–679.
