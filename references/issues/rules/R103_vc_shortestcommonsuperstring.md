---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover (for cubic graphs) to Shortest Common Superstring"
labels: rule
assignees: ''
canonical_source_name: 'Vertex Cover'
canonical_target_name: 'Shortest Common Superstring'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Vertex Cover (for cubic graphs)
**Target:** Shortest Common Superstring
**Motivation:** Establishes NP-completeness of SHORTEST COMMON SUPERSTRING via polynomial-time reduction from VERTEX COVER restricted to cubic (3-regular) graphs. The Shortest Common Superstring problem asks for the shortest string containing each input string as a contiguous substring. This problem is fundamental in bioinformatics (genome assembly) and data compression. Maier and Storer (1977) showed it is NP-complete even over a binary alphabet, using cubic graph structure to control the overlaps between constructed strings.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, SR9, p.228. [Maier and Storer, 1977].

## GJ Source Entry

> [SR9] SHORTEST COMMON SUPERSTRING
> INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
> QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a substring of w, i.e., w = w_0 x w_1 where each w_i ∈ Σ*?
> Reference: [Maier and Storer, 1977]. Transformation from VERTEX COVER for cubic graphs.
> Comment: Remains NP-complete even if |Σ| = 2 or if all x ∈ R have |x| ≤ 8 and contain no repeated symbols. Solvable in polynomial time if all x ∈ R have |x| ≤ 2.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a VERTEX COVER instance on a cubic graph G = (V, E) with V = {v_1, ..., v_n}, E = {e_1, ..., e_m} (where m = 3n/2 since G is cubic), and integer K, construct a SHORTEST COMMON SUPERSTRING instance as follows (based on Gallant, Maier, and Storer 1980):

1. **Alphabet:** Σ = {0, 1} (binary alphabet). The cubic graph restriction is essential because each vertex has exactly 3 incident edges, which controls the structure of the strings.

2. **Vertex encoding:** Assign each vertex v_i a unique binary codeword c_i of length L = ⌈log₂(n)⌉ + O(1). Each codeword is padded so that no codeword is a substring of another.

3. **Edge-string construction:** For each edge e_j = {v_a, v_b}, construct a string s_j that concatenates the codewords of v_a and v_b with specific separator patterns. The strings are designed so that:
   - Two edge-strings can overlap (share a substring) only if they share an endpoint vertex.
   - The amount of overlap when sharing a vertex is exactly the length of that vertex's codeword.

4. **Key property (cubic structure):** Since each vertex has exactly 3 incident edges, each codeword appears as a suffix of exactly 3 edge-strings and as a prefix of exactly 3 edge-strings. Selecting a vertex for the cover allows its 3 incident edge-strings to be "chained" together, saving the codeword length in the superstring.

5. **Bound:** K' = (total length of all edge-strings) - (savings from overlap) = ∑|s_j| - K · L, where the savings per vertex in the cover is L (one codeword shared across its incident edges).

6. **Correctness (forward):** If G has a vertex cover S of size ≤ K, order the edge-strings so that edges sharing a vertex in S overlap on that vertex's codeword. The total superstring length = ∑|s_j| - (overlap savings) ≤ K'.

7. **Correctness (reverse):** If a superstring of length ≤ K' exists, the overlapping positions identify at least K vertices, and these vertices form a vertex cover (since every edge-string must appear, requiring at least one endpoint's codeword to participate in an overlap).

**Key insight:** Substring containment (contiguous) is stricter than subsequence containment. The cubic graph structure ensures exactly 3 strings per vertex, making the overlap structure precise and predictable.

**Time complexity of reduction:** O(n + m) to construct the instance.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source VertexCover instance (|V|)
- m = `num_edges` = 3n/2 (cubic graph)

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `alphabet_size` | `2` |
| `num_strings` | `num_edges` (= 3 * num_vertices / 2) |
| `max_string_length` | O(log(num_vertices)) |
| `total_string_length` | O(num_edges * log(num_vertices)) |
| `bound_K` | O(num_edges * log(num_vertices)) |

**Derivation:** Binary alphabet; one string per edge; string length is O(log n) for the vertex codewords plus separators; bound is total length minus savings from cover.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance (on a cubic graph) to ShortestCommonSuperstring, solve target with BruteForce, extract solution, verify on source
- Test with known YES instance: Petersen graph (cubic, 10 vertices, 15 edges), vertex cover of size 6
- Test with known NO instance: cubic graph where minimum vertex cover exceeds K
- Verify that every constructed edge-string appears as a contiguous substring of the superstring
- Check that the binary alphabet constraint is satisfied (|Σ| = 2)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover on cubic graph):**
Prism graph (triangular prism) with 6 vertices V = {v_1, v_2, v_3, v_4, v_5, v_6} and 9 edges:
- Triangle 1: {v_1,v_2}, {v_2,v_3}, {v_1,v_3}
- Triangle 2: {v_4,v_5}, {v_5,v_6}, {v_4,v_6}
- Connecting: {v_1,v_4}, {v_2,v_5}, {v_3,v_6}
- Each vertex has degree 3 (cubic). K = 4.
- Vertex cover {v_1, v_3, v_5, v_4} of size 4 covers all edges:
  - {v_1,v_2}: v_1 ✓; {v_2,v_3}: v_3 ✓; {v_1,v_3}: v_1 ✓; {v_4,v_5}: v_4 ✓; {v_5,v_6}: v_5 ✓; {v_4,v_6}: v_4 ✓; {v_1,v_4}: v_1 ✓; {v_2,v_5}: v_5 ✓; {v_3,v_6}: v_3 ✓.

**Constructed target instance (ShortestCommonSuperstring):**
- Alphabet: Σ = {0, 1}
- Vertex codewords (length 3): c_1=000, c_2=001, c_3=010, c_4=011, c_5=100, c_6=101
- Edge-strings constructed by concatenating endpoint codewords with separators. For example:
  - s({v_1,v_2}) = 000#001 (using # as conceptual separator, encoded in binary)
  - Each string has length O(log n)
- Bound K' derived from total string length minus K·L savings

**Solution mapping:**
- Vertex cover {v_1, v_3, v_5, v_4} allows 4 vertices' codewords to be shared in overlaps
- The 4 selected vertices' codewords serve as overlap anchors, chaining their incident edge-strings
- Total superstring length achieves bound K'

**Verification:**
- Forward: cover of size 4 yields savings of 4·L in the superstring
- Reverse: any superstring within the bound implies sufficient overlaps to identify a cover of size ≤ K


## References

- **[Maier and Storer, 1977]**: [`Maier1977a`] David Maier and James A. Storer (1977). "A note on the complexity of the superstring problem". Computer Science Laboratory, Princeton University. Technical Report 233.
- **[Gallant, Maier, and Storer, 1980]**: [`Gallant1980`] John Gallant, David Maier, and James A. Storer (1980). "On finding minimal length superstrings". *Journal of Computer and System Sciences* 20(1), pp. 50-58.
