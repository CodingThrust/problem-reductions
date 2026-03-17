---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH BETWEEN TWO VERTICES to LONGEST PATH"
labels: rule
assignees: ''
canonical_source_name: 'Hamiltonian Path Between Two Vertices'
canonical_target_name: 'Longest Path'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** HAMILTONIAN PATH BETWEEN TWO VERTICES
**Target:** LONGEST PATH
**Motivation:** Establishes NP-completeness of LONGEST PATH via polynomial-time reduction from HAMILTONIAN PATH BETWEEN TWO VERTICES. The reduction is direct: assign unit length to every edge and set K = |V| - 1, so a simple s-t path of length at least K exists if and only if a Hamiltonian s-t path exists.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND29, p.213

## GJ Source Entry

> [ND29] LONGEST PATH
> INSTANCE: Graph G=(V,E), length l(e)∈Z^+ for each e∈E, positive integer K, specified vertices s,t∈V.
> QUESTION: Is there a simple path in G from s to t of length K or more, i.e., whose edge lengths sum to at least K?
> Reference: Transformation from HAMILTONIAN PATH BETWEEN TWO VERTICES.
> Comment: Remains NP-complete if l(e)=1 for all e∈E, as does the corresponding problem for directed paths in directed graphs. The general problem can be solved in polynomial time for acyclic digraphs, e.g., see [Lawler, 1976a]. The analogous directed and undirected "shortest path" problems can be solved for arbitrary graphs in polynomial time (e.g., see [Lawler, 1976a]), but are NP-complete if negative lengths are allowed.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a HAMILTONIAN PATH BETWEEN TWO VERTICES instance (G = (V, E), s, t) with n = |V| vertices, construct a LONGEST PATH instance as follows:

1. **Graph:** Use the same graph G' = G = (V, E).

2. **Edge lengths:** For every edge e in E, set the length l(e) = 1 (unit weights).

3. **Specified vertices:** Use the same source s and target t.

4. **Bound:** Set K = n - 1 (the number of edges in a Hamiltonian path on n vertices).

5. **Correctness (forward):** If G has a Hamiltonian path from s to t, this path traverses n - 1 edges, each of length 1, for a total length of n - 1 = K. Thus the LONGEST PATH instance is a YES instance.

6. **Correctness (reverse):** If G' has a simple path from s to t of length >= K = n - 1, then the path contains at least n - 1 edges. A simple path on n vertices uses at most n - 1 edges. Therefore the path uses exactly n - 1 edges and visits all n vertices -- it is a Hamiltonian path from s to t in G.

**Key invariant:** With unit weights, a simple s-t path of length >= n-1 exists if and only if it visits all n vertices, i.e., it is a Hamiltonian path.

**Time complexity of reduction:** O(|E|) to assign unit weights.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source instance (|V|)
- m = `num_edges` of source instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `num_vertices` |
| `num_edges` | `num_edges` |
| `bound` | `num_vertices - 1` |

**Derivation:** The graph is unchanged. The bound K = n - 1. Each edge gets a unit length assigned. The source and target vertices s, t are preserved.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a HamiltonianPathBetweenTwoVertices instance to LongestPath, solve target with BruteForce, extract solution, verify on source
- Test with known YES instance: a 6-vertex path graph (0-1-2-3-4-5) with s=0, t=5 has a Hamiltonian path; the LONGEST PATH instance should be satisfiable with K=5
- Test with known NO instance: a graph where s and t are in different components has no s-t path at all
- Compare with known results from literature

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (HamiltonianPathBetweenTwoVertices):**
Graph G with 7 vertices {0, 1, 2, 3, 4, 5, 6} and 10 edges:
- Edges: {0,1}, {0,2}, {1,3}, {2,3}, {2,4}, {3,5}, {4,5}, {4,6}, {5,6}, {1,6}
- s = 0, t = 6
- Hamiltonian path exists: 0 -> 1 -> 3 -> 2 -> 4 -> 5 -> 6 (visits all 7 vertices)

**Constructed target instance (LongestPath):**
- Same graph G' = G with 7 vertices and 10 edges
- Edge lengths: l(e) = 1 for all 10 edges
- s = 0, t = 6
- Bound K = 7 - 1 = 6

**Solution mapping:**
- LongestPath solution: path 0 -> 1 -> 3 -> 2 -> 4 -> 5 -> 6
- Path length: 6 edges x 1 = 6 >= K = 6
- This path visits all 7 vertices from s=0 to t=6, forming a Hamiltonian path in G

**Verification:**
- Forward: Hamiltonian path 0->1->3->2->4->5->6 has 6 edges of length 1, total = 6 = K
- Reverse: any simple path of length >= 6 on 7 vertices must use 6 edges and visit all 7 vertices -> Hamiltonian path


## References

- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.
