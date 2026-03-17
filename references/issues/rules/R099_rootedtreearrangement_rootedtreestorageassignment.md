---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Rooted Tree Arrangement to Rooted Tree Storage Assignment"
labels: rule
assignees: ''
canonical_source_name: 'ROOTED TREE ARRANGEMENT'
canonical_target_name: 'ROOTED TREE STORAGE ASSIGNMENT'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Rooted Tree Arrangement
**Target:** Rooted Tree Storage Assignment
**Motivation:** Establishes NP-completeness of ROOTED TREE STORAGE ASSIGNMENT by reduction from ROOTED TREE ARRANGEMENT. The key idea is that an optimal embedding of a graph into a rooted tree (minimizing total edge-stretch) can be re-encoded as a storage assignment problem: each edge {u,v} of the source graph becomes a "requirement set" {f(u), f(v)} that must lie on a directed path in a rooted tree, and the cost of extending subsets to form directed paths corresponds to the edge distances in the tree arrangement. Gavril (1977) showed this transformation is polynomial and preserves the YES/NO answer.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.1 [SR5], p.227

## GJ Source Entry

> [SR5] ROOTED TREE STORAGE ASSIGNMENT
> INSTANCE: Finite set X, collection C = {X_1, X_2, ..., X_n} of subsets of X, positive integer K.
> QUESTION: Is there a collection C' = {X_1', X_2', ..., X_n'} of subsets of X such that X_i ⊆ X_i' for 1 ≤ i ≤ n, such that ∑_{i=1}^{n} |X_i' − X_i| ≤ K, and such that there is a directed rooted tree T = (X, A) in which the elements of each X_i', 1 ≤ i ≤ n, form a directed path?
> Reference: [Gavril, 1977a]. Transformation from ROOTED TREE ARRANGEMENT.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a ROOTED TREE ARRANGEMENT instance: graph G = (V, E) with |V| = n vertices and positive integer K, construct a ROOTED TREE STORAGE ASSIGNMENT instance as follows:

1. **Universe:** Set X = V (the vertex set of G). The universe has |X| = n elements.

2. **Collection of subsets:** For each edge {u, v} ∈ E, create a subset X_e = {u, v} containing exactly the two endpoints of the edge. The collection is C = {X_e : e ∈ E}, with |C| = |E| subsets, each of size 2.

3. **Bound:** Set K' = K − |E|. (Each required subset has size 2, so forming a directed path through both endpoints requires at least 1 additional element if they are not adjacent in the tree, or 0 if they are parent-child. The total extension cost corresponds to the total edge distance minus the minimum |E| cost of traversing each edge.)

4. **Correctness (forward):** If there exists a rooted tree T = (U, F) with |U| = n and a one-to-one mapping f: V → U such that (a) for every edge {u,v} ∈ E, f(u) and f(v) lie on a common root-to-leaf path in T, and (b) ∑_{{u,v} ∈ E} d(f(u), f(v)) ≤ K, then we can construct extended subsets X_e' for each edge e = {u,v} by taking all tree nodes on the path from f(u) to f(v) in T. The total extension cost ∑ |X_e' − X_e| = ∑ (d(f(u),f(v)) − 1) = (∑ d(f(u),f(v))) − |E| ≤ K − |E| = K'.

5. **Correctness (reverse):** If there exists a valid storage assignment with extended subsets forming directed paths in some rooted tree T = (X, A) with total extension cost ≤ K', then the same tree T with the identity mapping gives a rooted tree arrangement with total distance ∑ d(u,v) ≤ K' + |E| = K.

6. **Solution extraction:** Given a valid storage assignment (a rooted tree T and extended subsets), the rooted tree arrangement is the same tree T with the identity embedding f(v) = v, and the arrangement cost is K' + |E|.

**Key invariant:** The extension cost for a single edge subset {u,v} equals d(u,v) − 1 in the rooted tree (number of intermediate nodes on the path), so total extension cost = total arrangement cost − |E|.

**Time complexity of reduction:** O(|E|) to construct the subsets.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G (|V|)
- m = `num_edges` of source graph G (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `universe_size`            | `num_vertices` (= n)             |
| `num_subsets`              | `num_edges` (= m)                |

**Derivation:** The universe X is exactly the vertex set V, so |X| = n. Each edge becomes one 2-element subset, giving |C| = m subsets. The bound K' = K − m is derived from the source bound.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a ROOTED TREE ARRANGEMENT instance (small graph G, bound K), reduce to ROOTED TREE STORAGE ASSIGNMENT, solve target by brute-force (enumerate all rooted trees on n nodes and all ways to extend subsets to paths), verify the solution maps back to a valid tree arrangement.
- Test with a path graph P_6 (6 vertices, 5 edges): the optimal rooted tree arrangement should embed the path into a rooted path tree with total distance 5 (each edge has distance 1). The storage assignment with K' = 5 − 5 = 0 means no extensions needed, which is correct since the path itself is already a rooted tree where each edge forms a directed path.
- Test with a star graph K_{1,5} (6 vertices, 5 edges, center vertex 0): embed into a star rooted tree at 0; total distance = 5, K = 5, K' = 0.
- Test unsatisfiable case: complete graph K_4 with very small K; verify no valid storage assignment exists.

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (RootedTreeArrangement):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {1,2}, {2,3}, {0,4}, {4,5}, {1,3}, {0,2}
- Bound K = 12.

Consider a rooted tree T with root 0:
```
       0
      / \
     1   4
    / \    \
   2   3    5
```
- d(0,1) = 1, d(1,2) = 1, d(2,3) = 2, d(0,4) = 1, d(4,5) = 1, d(1,3) = 1, d(0,2) = 2
- Total distance = 1+1+2+1+1+1+2 = 9 ≤ K = 12 ✓

**Constructed target instance (RootedTreeStorageAssignment):**
- Universe X = {0, 1, 2, 3, 4, 5}, |X| = 6
- Collection C (one subset per edge):
  - X_1 = {0, 1}, X_2 = {1, 2}, X_3 = {2, 3}, X_4 = {0, 4}, X_5 = {4, 5}, X_6 = {1, 3}, X_7 = {0, 2}
- |C| = 7 subsets
- Bound K' = K − |E| = 12 − 7 = 5

**Solution mapping (using the same rooted tree T):**
- X_1' = {0, 1}: path 0→1, no extension needed. |X_1' − X_1| = 0.
- X_2' = {1, 2}: path 1→2, no extension needed. |X_2' − X_2| = 0.
- X_3' = {1, 2, 3}: path 1→2→..., but 2 and 3 are siblings, not on same path!

Re-consider the tree. Let us use:
```
       0
      /|\
     1  2  4
     |  |   \
     3  .    5
```
Wait — let me use a chain tree for simpler paths:
```
    0 → 1 → 2 → 3
    |
    4 → 5
```
- d(0,1) = 1, d(1,2) = 1, d(2,3) = 1, d(0,4) = 1, d(4,5) = 1
- d(1,3) = 2 (path 1→2→3, both on same root-to-leaf path ✓)
- d(0,2) = 2 (path 0→1→2, both on same root-to-leaf path ✓)
- Total distance = 1+1+1+1+1+2+2 = 9 ≤ K = 12 ✓

Extended subsets:
- X_1' = {0, 1}: path 0→1. Extension cost = 0.
- X_2' = {1, 2}: path 1→2. Extension cost = 0.
- X_3' = {2, 3}: path 2→3. Extension cost = 0.
- X_4' = {0, 4}: path 0→4. Extension cost = 0.
- X_5' = {4, 5}: path 4→5. Extension cost = 0.
- X_6' = {1, 2, 3}: path 1→2→3, extending {1,3} by adding {2}. Extension cost = 1.
- X_7' = {0, 1, 2}: path 0→1→2, extending {0,2} by adding {1}. Extension cost = 1.
- Total extension cost = 0+0+0+0+0+1+1 = 2 ≤ K' = 5 ✓

**Verification:**
- Total arrangement distance = total extension cost + |E| = 2 + 7 = 9 ≤ K = 12 ✓
- Each extended subset forms a directed path in the rooted tree ✓
- Each original subset is contained in its extension ✓


## References

- **[Gavril, 1977a]**: [`Gavril1977a`] F. Gavril (1977). "Some NP-complete problems on graphs". In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91-95. Johns Hopkins University.
- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976`] M. R. Garey, D. S. Johnson, and L. Stockmeyer (1976). "Some simplified NP-complete graph problems." *Theoretical Computer Science* 1(3), pp. 237-267.
