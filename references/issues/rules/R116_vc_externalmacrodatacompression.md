---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to EXTERNAL MACRO DATA COMPRESSION"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'EXTERNAL MACRO DATA COMPRESSION'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** VERTEX COVER
**Target:** EXTERNAL MACRO DATA COMPRESSION
**Motivation:** Establishes NP-completeness of EXTERNAL MACRO DATA COMPRESSION via polynomial-time reduction from VERTEX COVER. This shows that the problem of optimally compressing a string using an external dictionary with pointers is computationally hard, connecting graph covering problems to data compression theory. The result is foundational to the Storer-Szymanski macro model of data compression, demonstrating that finding optimal textual substitution schemes is inherently intractable.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.231

## GJ Source Entry

> [SR22] EXTERNAL MACRO DATA COMPRESSION
> INSTANCE: Alphabet Σ, string s E Σ*, pointer cost h E Z+, and a bound B E Z+.
> QUESTION: Are there strings D (dictionary string) and C (compressed string) in (Σ ∪ {p_i: 1 <= i <= |s|})*, where the symbols p_i are "pointers," such that
>
> |D| + |C| + (h-1)*(number of occurrences of pointers in D and C) <= B
>
> and such that there is a way of identifying pointers with substrings of D so that S can be obtained from C by repeatedly replacing pointers in C by their corresponding substrings in D?
> Reference: [Storer, 1977], [Storer and Szymanski, 1978]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if h is any fixed integer 2 or greater. Many variants, including those in which D can contain no pointers and/or no pointers can refer to overlapping strings, are also NP-complete. If the alphabet size is fixed at 3 or greater, and the pointer cost is [h*log|s|], the problem is also NP-complete. For further variants, including the case of "original pointers," see references.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a VERTEX COVER instance (G, k) where G = (V, E) with n = |V| vertices and m = |E| edges, construct an EXTERNAL MACRO DATA COMPRESSION instance as follows:

1. **Alphabet construction:** Create an alphabet Sigma with symbols encoding the graph structure. Use one symbol per vertex and additional structural symbols. The alphabet size is O(n).

2. **Source string s construction:** Construct the string s to encode the graph G. For each edge {u, v} in E, embed a "gadget substring" in s that contains the symbols for both u and v. The string s is designed so that shared substrings between edge gadgets correspond to shared vertices. The total string length is |s| = O(n + m).

3. **Dictionary and compression parameters:**
   - Set pointer cost h = 2 (or any fixed constant >= 2).
   - Set the compression bound B = f(k, n, m) for a polynomial function f such that achieving compression of s to total cost <= B requires a dictionary D that captures the structure of at least n - k vertices.

4. **Vertex cover correspondence:** Each vertex v in a vertex cover contributes a "dictionary entry" -- a substring of D that can be referenced by pointers in C. Selecting v for the cover means the edge gadgets incident to v can use pointers to v's dictionary entry, reducing the total compressed size. A vertex cover of size k allows compression to cost <= B because:
   - Each covered edge has at least one endpoint in the dictionary
   - Pointers to dictionary entries replace repeated vertex-substrings in the compressed string

5. **Solution extraction:** Given strings D and C with total cost <= B, identify which vertex substrings appear in D. These vertices form a vertex cover: every edge gadget must have at least one endpoint in D to achieve sufficient compression.

**Key invariant:** G has a vertex cover of size at most k if and only if string s can be compressed with total cost |D| + |C| + (h-1) * (pointer count) <= B using an external dictionary scheme.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source graph G
- m = `num_edges` of source graph G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `alphabet_size` | O(num_vertices) |
| `string_length` | O(num_vertices + num_edges) |
| `pointer_cost` | 2 (constant) |
| `bound` | polynomial in num_vertices, num_edges, k |

**Derivation:** The alphabet encodes vertex identifiers. The string has one gadget per edge plus vertex separators, giving length O(n + m). The pointer cost is fixed at 2 (or any constant >= 2). The compression bound B is set to be achievable exactly when a vertex cover of size k exists, balancing dictionary size against pointer overhead.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance to ExternalMacroDataCompression, solve the compression problem via brute-force enumeration of dictionary/compressed string pairs, extract the implied vertex cover, verify it is a valid cover on the original graph
- Check that the minimum compression cost corresponds to the minimum vertex cover size
- Test with a star graph K_{1,5} (vertex cover = center vertex alone) to verify that the dictionary contains the center's substring
- Verify that a graph with no edges yields a trivially compressible string (no dictionary needed)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {3,5}
- Minimum vertex cover: {1, 2, 3} of size k = 3
  - {0,1} by 1, {0,2} by 2, {1,2} by 1+2, {1,3} by 1+3, {2,4} by 2, {3,4} by 3, {3,5} by 3

**Constructed target instance (ExternalMacroDataCompression):**
Using the reduction with h = 2:
- Alphabet Sigma = {v0, v1, v2, v3, v4, v5, #} (vertex symbols + separator)
- String s encodes the 7 edges as gadget substrings:
  - s = "v0v1 # v0v2 # v1v2 # v1v3 # v2v4 # v3v4 # v3v5" (conceptually, with appropriate encoding)
  - |s| = O(7 * 2 + 6) = O(20)
- Bound B is set so that compression is achievable with dictionary containing 3 vertex entries

**Solution mapping:**
- Dictionary D contains entries for vertices {1, 2, 3}: D = "v1 v2 v3"
- Compressed string C replaces occurrences of v1, v2, v3 with pointers to D
- Each pointer costs h = 2, so pointer overhead = (2-1) * (number of pointers)
- With 3 vertices in dictionary, all 7 edges have at least one endpoint referenced
- Total cost: |D| + |C| + pointer_overhead <= B
- Extracted vertex cover: {1, 2, 3} (the vertices whose substrings appear in D)
- Verification: all 7 edges covered by {1, 2, 3}


## References

- **[Storer, 1977]**: [`Storer1977`] James A. Storer (1977). "{NP}-completeness results concerning data compression". Dept. of Electrical Engineering and Computer Science, Princeton University.
- **[Storer and Szymanski, 1978]**: [`Storer and Szymanski1978`] James A. Storer and Thomas G. Szymanski (1978). "The macro model for data compression (Extended abstract)". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 30-39. Association for Computing Machinery.
