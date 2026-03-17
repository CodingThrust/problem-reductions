---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to TRAVELING SALESMAN"
labels: rule
assignees: ''
canonical_source_name: 'HAMILTONIAN CIRCUIT'
canonical_target_name: 'TRAVELING SALESMAN'
source_in_codebase: false
target_in_codebase: true
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** TRAVELING SALESMAN
**Motivation:** Establishes NP-completeness of TRAVELING SALESMAN via one of the most classical and widely-taught polynomial-time reductions in complexity theory. The reduction is remarkably simple: embed the graph into a complete weighted graph with distances 1 (for existing edges) and 2 (for non-edges), then ask whether a tour of total length n exists. This is a textbook example appearing in virtually every algorithms course.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND22, p.211

## GJ Source Entry

> [ND22] TRAVELING SALESMAN
> INSTANCE: Set C of m cities, distance d(c_i,c_j)∈Z^+ for each pair of cities c_i,c_j∈C, positive integer B.
> QUESTION: Is there a tour of C having length B or less, i.e., a permutation <c_{π(1)},c_{π(2)},...,c_{π(m)}> of C such that
> (∑_{i=1}^{m-1} d(c_{π(i)},c_{π(i+1)})) + d(c_{π(m)},c_{π(1)}) ≤ B ?
> Reference: Transformation from HAMILTONIAN CIRCUIT.
> Comment: Remains NP-complete even if d(c_i,c_j)∈{1,2} for all c_i,c_j∈C. Special cases that can be solved in polynomial time are discussed in [Gilmore and Gomory, 1964], [Garfinkel, 1977], and [Syslo, 1973]. The variant in which we ask for a tour with "mean arrival time" of B or less is also NP-complete [Sahni and Gonzalez, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HamiltonianCircuit instance G = (V, E) with n = |V| vertices, construct a TravelingSalesman instance as follows:

1. **City set:** The set of cities C is exactly the vertex set V, so m = n cities.

2. **Distance matrix:** Construct the complete graph K_n on V with distances:
   - d(u, v) = 1 if {u, v} ∈ E (the edge exists in G)
   - d(u, v) = 2 if {u, v} ∉ E (the edge does not exist in G)

3. **Bound:** Set B = n (the number of vertices/cities).

4. **Correctness (forward):** If G has a Hamiltonian circuit v_1 → v_2 → ... → v_n → v_1, then this same ordering gives a tour of length n × 1 = n = B, since every consecutive pair {v_i, v_{i+1}} and {v_n, v_1} is an edge in G and thus has distance 1.

5. **Correctness (reverse):** If there is a tour of length ≤ B = n, then since the tour visits all n cities and each edge in the tour has distance ≥ 1, the total length is ≥ n. For the total to be exactly n, every edge in the tour must have distance 1, meaning every consecutive pair is an edge in G. Therefore, the tour is a Hamiltonian circuit in G.

**Key invariant:** A tour has length exactly n if and only if it uses only edges of weight 1, which correspond exactly to the edges of G. Hence a tour of length ≤ n exists iff G has a Hamiltonian circuit.

**Time complexity of reduction:** O(n^2) to construct the distance matrix.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source HamiltonianCircuit instance (|V|)
- m = `num_edges` of source HamiltonianCircuit instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_cities` | `num_vertices` |
| `num_edges` (in complete graph) | `num_vertices * (num_vertices - 1) / 2` |
| `bound` | `num_vertices` |

**Derivation:** The TSP instance has the same number of cities as vertices (n). The distance matrix represents a complete graph with n(n-1)/2 entries. The bound is simply n.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->
- Closed-loop test: reduce a small HamiltonianCircuit instance G to TravelingSalesman, solve the TSP with BruteForce, extract the tour, verify that if a tour of length n exists then all edges in the tour are in G (forming a Hamiltonian circuit), and vice versa.
- Test with known YES instance: C_6 (6-cycle) has a Hamiltonian circuit. The TSP instance should have a tour of length 6.
- Test with known NO instance: a graph with an isolated vertex has no HC. The TSP instance should have no tour of length n (since the isolated vertex forces at least one distance-2 edge, giving total ≥ n+1).
- Verify distance matrix: all entries are 1 or 2, diagonal is 0, matrix is symmetric.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianCircuit):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 9 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {5,0}, {0,3}, {1,4}, {2,5}
- (Triangular prism / prism graph)
- Hamiltonian circuit exists: 0 → 1 → 2 → 3 → 4 → 5 → 0

**Constructed target instance (TravelingSalesman):**
- 6 cities: {0, 1, 2, 3, 4, 5}
- Distance matrix (symmetric, d(i,j)):
```
     0  1  2  3  4  5
  0: -  1  2  1  2  1
  1: 1  -  1  2  1  2
  2: 2  1  -  1  2  1
  3: 1  2  1  -  1  2
  4: 2  1  2  1  -  1
  5: 1  2  1  2  1  -
```
- Bound B = 6
- Edges in G get distance 1: {0,1}=1, {1,2}=1, {2,3}=1, {3,4}=1, {4,5}=1, {5,0}=1, {0,3}=1, {1,4}=1, {2,5}=1
- Non-edges get distance 2: {0,2}=2, {0,4}=2, {1,3}=2, {1,5}=2, {2,4}=2, {3,5}=2

**Solution mapping:**
- TSP optimal tour: 0 → 1 → 2 → 3 → 4 → 5 → 0
- Tour length: d(0,1) + d(1,2) + d(2,3) + d(3,4) + d(4,5) + d(5,0) = 1+1+1+1+1+1 = 6 = B ✓
- All tour edges have distance 1, so all are edges in G → this is a Hamiltonian circuit in G ✓

**Alternative tour:** 0 → 3 → 2 → 5 → 4 → 1 → 0
- d(0,3)=1, d(3,2)=1, d(2,5)=1, d(5,4)=1, d(4,1)=1, d(1,0)=1 → total = 6 = B ✓
- Also a valid Hamiltonian circuit in G ✓


## References

- **[Gilmore and Gomory, 1964]**: [`Gilmore1964`] P. C. Gilmore and R. E. Gomory (1964). "Sequencing a one state-variable machine: a solvable case of the traveling salesman problem". *Operations Research* 12, pp. 655–679.
- **[Garfinkel, 1977]**: [`Garfinkel1977`] R. S. Garfinkel (1977). "Minimizing wallpaper waste, {Part} 1: a class of traveling salesman problems". *Operations Research* 25, pp. 741–751.
- **[Syslo, 1973]**: [`Syslo1973`] Maciej M. Syslo (1973). "A new solvable case of the traveling salesman problem". *Mathematical Programming* 4, pp. 347–348.
- **[Sahni and Gonzalez, 1976]**: [`Gonzalez1976`] T. Gonzalez and S. Sahni (1976). "Open shop scheduling to minimize finish time". *Journal of the Association for Computing Machinery* 23, pp. 665–679.
