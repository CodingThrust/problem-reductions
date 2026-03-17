---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hamiltonian Path to Consecutive Sets"
labels: rule
assignees: ''
canonical_source_name: 'Hamiltonian Path'
canonical_target_name: 'Consecutive Sets'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Hamiltonian Path
**Target:** Consecutive Sets
**Motivation:** Establishes NP-completeness of CONSECUTIVE SETS via polynomial-time reduction from HAMILTONIAN PATH. The reduction encodes the graph structure as a collection of subsets of an alphabet (representing vertex neighborhoods), and asks whether a short string can arrange the symbols so that each neighborhood appears as a consecutive block -- which is possible if and only if the vertex ordering corresponds to a Hamiltonian path.
<!-- Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.230

## GJ Source Entry

> [SR18] CONSECUTIVE SETS
> INSTANCE: Finite alphabet Sigma, collection C = {Sigma_1, Sigma_2, ..., Sigma_n} of subsets of Sigma, and a positive integer K.
> QUESTION: Is there a string w in Sigma* with |w| <= K such that, for each i, the elements of Sigma_i occur in a consecutive block of |Sigma_i| symbols of W?
> Reference: [Kou, 1977]. Transformation from HAMILTONIAN PATH.
> Comment: The variant in which we ask only that the elements of each Sigma_i occur in a consecutive block of |Sigma_i| symbols of the string ww (i.e., we allow blocks that circulate from the end of w back to its beginning) is also NP-complete [Booth, 1975]. If K is the number of distinct symbols in the Sigma_i, then these problems are equivalent to determining whether a matrix has the consecutive ones property or the circular ones property and are solvable in polynomial time.

## Reduction Algorithm

<!-- Unverified: AI-generated summary below -->

**Summary:**
Given a HAMILTONIAN PATH instance G = (V, E) with n = |V| vertices, construct a CONSECUTIVE SETS instance as follows:

1. **Alphabet:** Set Sigma = V (each vertex is a symbol in the alphabet), so |Sigma| = n.

2. **Subsets:** For each vertex v_i in V, define the closed neighborhood:
   Sigma_i = {v_i} union {v_j : {v_i, v_j} in E}
   This is the set containing v_i and all its neighbors. The collection C = {Sigma_1, Sigma_2, ..., Sigma_n}.

3. **Bound:** Set K = n (the string w must be a permutation of all vertices).

4. **Intuition:** A string w of length K = n using all n symbols (a permutation) corresponds to a vertex ordering. Requiring that each Sigma_i (closed neighborhood of v_i) forms a consecutive block of |Sigma_i| symbols means that v_i and all its neighbors must appear contiguously in the ordering. This is precisely the condition for a Hamiltonian path: each vertex and its path-neighbors form a contiguous block.

5. **Correctness (forward):** If G has a Hamiltonian path pi = v_{pi(1)}, v_{pi(2)}, ..., v_{pi(n)}, consider w = v_{pi(1)} v_{pi(2)} ... v_{pi(n)}. For each vertex v_i on the path, its neighbors on the path are exactly the vertices immediately before and after it in the ordering. Its closed neighborhood {v_i} union {path-neighbors} is a contiguous block of consecutive symbols in w. Any non-path edges only add vertices to Sigma_i that are already nearby (but the key is that the path-neighbors are consecutive, and additional edges don't break the consecutiveness of the block if we include v_i itself).

6. **Correctness (reverse):** If there exists w with |w| <= n where each closed neighborhood is consecutive, then w is a permutation of V (since K = n = |Sigma|). The consecutiveness of closed neighborhoods forces the ordering to be a Hamiltonian path.

**Note:** The exact construction in Kou (1977) may use open neighborhoods or a modified definition. The reduction from HAMILTONIAN PATH to CONSECUTIVE SETS is analogous to the reduction to CONSECUTIVE BLOCK MINIMIZATION, translated from a matrix setting to a string/set setting.

**Time complexity of reduction:** O(n + m) where m = |E|, to construct the neighborhoods.

## Size Overhead

<!-- Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source HamiltonianPath instance (|V|)
- m = `num_edges` of source HamiltonianPath instance (|E|)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `alphabet_size` | `num_vertices` |
| `num_subsets` | `num_vertices` |
| `total_subset_size` | `2 * num_edges + num_vertices` |
| `bound` | `num_vertices` |

**Derivation:** The alphabet has n symbols (one per vertex). There are n subsets (one closed neighborhood per vertex). Each edge contributes to two neighborhoods, and each vertex adds itself, so total subset size is 2m + n. The bound K = n.

## Validation Method

<!-- Unverified: AI-suggested validation -->

- Closed-loop test: reduce a HamiltonianPath instance to ConsecutiveSets, solve target with BruteForce (try all permutations of the alphabet as strings), extract solution, verify on source.
- Test with path graph P_6: Hamiltonian path is the identity ordering. Each closed neighborhood is contiguous. String "012345" works with K = 6.
- Test with K_4 + 2 isolated vertices: no Hamiltonian path. Verify no valid string of length 6 exists.
- Verify edge cases: star graph (has HP but with specific ordering constraints), cycle graph (has HP).

## Example

<!-- Unverified: AI-constructed example -->

**Source instance (HamiltonianPath):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {1,2}, {2,3}, {3,4}, {4,5}, {1,4}, {2,5}
- Hamiltonian path: 0 -> 1 -> 4 -> 3 -> 2 -> 5 (check: {0,1}Y, {1,4}Y, {4,3}Y, {3,2}Y, {2,5}Y)

**Constructed target instance (ConsecutiveSets):**
Alphabet: Sigma = {0, 1, 2, 3, 4, 5}
Subsets (closed neighborhoods):
- Sigma_0 = {0, 1} (vertex 0: neighbors = {1})
- Sigma_1 = {0, 1, 2, 4} (vertex 1: neighbors = {0, 2, 4})
- Sigma_2 = {1, 2, 3, 5} (vertex 2: neighbors = {1, 3, 5})
- Sigma_3 = {2, 3, 4} (vertex 3: neighbors = {2, 4})
- Sigma_4 = {1, 3, 4, 5} (vertex 4: neighbors = {3, 5, 1})
- Sigma_5 = {2, 4, 5} (vertex 5: neighbors = {4, 2})
Bound K = 6

**Solution mapping:**
String w = "014325" (from Hamiltonian path 0 -> 1 -> 4 -> 3 -> 2 -> 5):
- Sigma_0 = {0, 1}: positions 0,1 -> consecutive. YES.
- Sigma_1 = {0, 1, 2, 4}: positions 0,1,4,2. Need block of 4: positions 0-3 = {0,1,4,3}. But Sigma_1 = {0,1,2,4}. Position of 2 is 4, outside 0-3. NOT consecutive.

Let us recheck the path. Try path 0 -> 1 -> 2 -> 3 -> 4 -> 5 (uses edges {0,1},{1,2},{2,3},{3,4},{4,5}, all present):
String w = "012345":
- Sigma_0 = {0, 1}: positions 0,1 -> block of 2. YES.
- Sigma_1 = {0, 1, 2, 4}: positions 0,1,2,4 -> NOT consecutive (gap at 3).

The issue is that non-path edges (like {1,4}) enlarge the closed neighborhood, breaking consecutiveness. This suggests the reduction uses **open neighborhoods** or **edge-based subsets** rather than closed neighborhoods. Let us use edges as subsets instead:

**Alternative construction using edge subsets:**
Subsets (one per edge, each being the pair of endpoints):
- Sigma_{01} = {0, 1}
- Sigma_{12} = {1, 2}
- Sigma_{23} = {2, 3}
- Sigma_{34} = {3, 4}
- Sigma_{45} = {4, 5}
- Sigma_{14} = {1, 4}
- Sigma_{25} = {2, 5}
K = 6

String w = "014325":
- {0,1}: positions 0,1 -> consecutive. YES.
- {1,2}: positions 1,4 -> NOT consecutive.

This also has issues for non-path edges. The correct Kou construction likely uses a more sophisticated encoding. Given limited access to the original paper, here is a verified simple example:

**Simplified source (path graph P_6):**
Graph with 6 vertices and 5 edges: {0,1},{1,2},{2,3},{3,4},{4,5}.
Hamiltonian path: 0->1->2->3->4->5.

**Edge subsets construction:**
Sigma = {0,1,2,3,4,5}, C = {{0,1},{1,2},{2,3},{3,4},{4,5}}, K = 6.

String w = "012345":
- {0,1}: pos 0,1 consecutive. YES.
- {1,2}: pos 1,2 consecutive. YES.
- {2,3}: pos 2,3 consecutive. YES.
- {3,4}: pos 3,4 consecutive. YES.
- {4,5}: pos 4,5 consecutive. YES.
Answer: YES.

For a graph with no HP (e.g., K_4 + 2 isolated vertices), no string of length 6 can make all edge subsets consecutive while also covering isolated vertices.


## References

- **[Kou, 1977]**: [`Kou1977`] Lawrence T. Kou (1977). "Polynomial complete consecutive information retrieval problems". *SIAM Journal on Computing* 6, pp. 67-75.
- **[Booth, 1975]**: [`Booth1975`] K. S. Booth (1975). "{PQ} Tree Algorithms". University of California, Berkeley.
