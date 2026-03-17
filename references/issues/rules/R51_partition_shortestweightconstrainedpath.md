---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to SHORTEST WEIGHT-CONSTRAINED PATH"
labels: rule
assignees: ''
canonical_source_name: 'Partition'
canonical_target_name: 'Shortest Weight-Constrained Path'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** PARTITION
**Target:** SHORTEST WEIGHT-CONSTRAINED PATH
**Motivation:** Establishes NP-completeness of SHORTEST WEIGHT-CONSTRAINED PATH via polynomial-time reduction from PARTITION. The reduction encodes the subset-sum structure of PARTITION into a layered graph where choosing "upper" or "lower" arcs at each layer corresponds to including or excluding an element, with edge lengths and weights set so that a feasible constrained path exists if and only if a balanced partition exists.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, ND30, p.214

## GJ Source Entry

> [ND30] SHORTEST WEIGHT-CONSTRAINED PATH
> INSTANCE: Graph G=(V,E), length l(e)∈Z^+, and weight w(e)∈Z^+ for each e∈E, specified vertices s,t∈V, positive integers K,W.
> QUESTION: Is there a simple path in G from s to t with total weight W or less and total length K or less?
> Reference: [Megiddo, 1977]. Transformation from PARTITION.
> Comment: Also NP-complete for directed graphs. Both problems are solvable in polynomial time if all weights are equal or all lengths are equal.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a PARTITION instance with multiset A = {a_1, a_2, ..., a_n} of positive integers with total sum S = sum(a_i), construct a SHORTEST WEIGHT-CONSTRAINED PATH instance as follows:

1. **Layered graph construction:** Create a directed acyclic graph with n + 1 layers of vertices. Let v_0 = s (source) and v_n = t (target). At each layer i (for i = 1, ..., n-1), create two intermediate vertices: u_i (upper) and d_i (lower). The source s and sink t are single vertices.

2. **Edges at layer i (for i = 1, ..., n):** From the previous layer's vertex/vertices to the next layer's vertex/vertices, add two parallel paths:
   - **"Include a_i" arc:** Set length l = a_i and weight w = 0 (or vice versa).
   - **"Exclude a_i" arc:** Set length l = 0 and weight w = a_i (or vice versa).

   More precisely, construct a chain of n + 1 nodes v_0, v_1, ..., v_n with s = v_0 and t = v_n. Between v_{i-1} and v_i, add two parallel edges:
   - Edge e_i^1 with length l(e_i^1) = a_i and weight w(e_i^1) = 0.
   - Edge e_i^2 with length l(e_i^2) = 0 and weight w(e_i^2) = a_i.

3. **Bounds:** Set K = S/2 (length budget) and W = S/2 (weight budget), where S = sum of all a_i.

4. **Correctness (forward):** If A can be partitioned into A_1 and A_2 with sum(A_1) = sum(A_2) = S/2, then for each a_i in A_1 take the edge with length a_i and weight 0, and for each a_i in A_2 take the edge with length 0 and weight a_i. The total length is sum(A_1) = S/2 = K and total weight is sum(A_2) = S/2 = W.

5. **Correctness (reverse):** If a path from s to t has total length <= K = S/2 and total weight <= W = S/2, then since every a_i contributes either to the total length or total weight (and the sum of all contributions is S), we must have total length = S/2 and total weight = S/2. The set of indices using the length-edge form a partition with sum S/2.

**Key invariant:** Each element a_i contributes its value to exactly one of the two budgets (length or weight). A feasible path exists iff the total can be split evenly.

**Time complexity of reduction:** O(n) to construct the graph.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements in the PARTITION instance
- S = sum of all elements

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_vertices` | `n + 1` |
| `num_edges` | `2 * n` |
| `length_bound` (K) | `S / 2` |
| `weight_bound` (W) | `S / 2` |

**Derivation:** The layered graph has n + 1 vertices (one per layer) and 2n edges (two parallel edges per layer). The bounds are both S/2.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a PARTITION instance to ShortestWeightConstrainedPath, solve target with BruteForce, extract solution, verify on source
- Test with known YES instance: A = {1, 2, 3, 4, 5, 5} with S = 20, S/2 = 10. Partition {1, 4, 5} and {2, 3, 5} both sum to 10.
- Test with known NO instance: A = {1, 2, 3, 7} with S = 13 (odd, so no balanced partition).
- Compare with known results from literature

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {2, 3, 4, 5, 6, 4} with S = 24, S/2 = 12.
A valid partition: A_1 = {2, 4, 6} (sum = 12), A_2 = {3, 5, 4} (sum = 12).

**Constructed target instance (ShortestWeightConstrainedPath):**
- 7 vertices: v_0 = s, v_1, v_2, v_3, v_4, v_5, v_6 = t
- 12 edges (2 per layer):

| Layer i | a_i | Edge e_i^1 (length, weight) | Edge e_i^2 (length, weight) |
|---------|-----|-----------------------------|-----------------------------|
| 1 | 2 | (2, 0) | (0, 2) |
| 2 | 3 | (3, 0) | (0, 3) |
| 3 | 4 | (4, 0) | (0, 4) |
| 4 | 5 | (5, 0) | (0, 5) |
| 5 | 6 | (6, 0) | (0, 6) |
| 6 | 4 | (4, 0) | (0, 4) |

- K = 12 (length bound), W = 12 (weight bound)

**Solution mapping:**
- Partition A_1 = {2, 4, 6} (indices 1, 3, 5): use e_i^1 (length edge) at layers 1, 3, 5
- Partition A_2 = {3, 5, 4} (indices 2, 4, 6): use e_i^2 (weight edge) at layers 2, 4, 6
- Path: v_0 --e_1^1--> v_1 --e_2^2--> v_2 --e_3^1--> v_3 --e_4^2--> v_4 --e_5^1--> v_5 --e_6^2--> v_6
- Total length: 2 + 0 + 4 + 0 + 6 + 0 = 12 = K
- Total weight: 0 + 3 + 0 + 5 + 0 + 4 = 12 = W
- Both bounds met, confirming YES


## References

- **[Megiddo, 1977]**: [`Megiddo1977`] Nimrod Megiddo (1977). "Combinatorial optimization with rational objective functions". *Mathematics of Operations Research* 4(4), pp. 414-424.
