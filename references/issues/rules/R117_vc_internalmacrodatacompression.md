---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to INTERNAL MACRO DATA COMPRESSION"
labels: rule
assignees: ''
canonical_source_name: 'VERTEX COVER'
canonical_target_name: 'INTERNAL MACRO DATA COMPRESSION'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** VERTEX COVER
**Target:** INTERNAL MACRO DATA COMPRESSION
**Motivation:** Establishes NP-completeness of INTERNAL MACRO DATA COMPRESSION via polynomial-time reduction from VERTEX COVER. This variant of macro data compression uses a single string as both dictionary and compressed output (self-referencing pointers), and is shown to be equally hard as the external variant. The reduction demonstrates that even when the dictionary is not separate but embedded within the compressed string itself, finding optimal compression remains NP-complete.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.231

## GJ Source Entry

> [SR23] INTERNAL MACRO DATA COMPRESSION
> INSTANCE: Alphabet Σ, string s E Σ*, pointer cost h E Z+, and a bound B E Z+.
> QUESTION: Is there a single string C E (Σ ∪ {p_i: 1 <= i <= |s|})* such that
>
> |C| + (h-1)*(number of occurences of pointers in C) <= B
>
> and such that there is a way of identifying pointers with substrings of C so that s can be obtained from C by using C as both compressed string and dictionary string in the manner indicated in the previous problem?
> Reference: [Storer, 1977], [Storer and Szymanski, 1978]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if h is any fixed integer 2 or greater. For other NP-complete variants (as in the previous problem), see references.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a VERTEX COVER instance (G, k) where G = (V, E) with n = |V| vertices and m = |E| edges, construct an INTERNAL MACRO DATA COMPRESSION instance as follows:

1. **Alphabet construction:** Create an alphabet Sigma encoding the graph structure, with one symbol per vertex plus structural separators. The alphabet size is O(n).

2. **Source string s construction:** Construct the string s to encode the graph G similarly to the external macro case. For each edge {u, v} in E, embed a gadget substring containing symbols for both u and v. The string is designed so that repeated vertex substrings across multiple edge gadgets can be compressed using internal pointers. The total string length is |s| = O(n + m).

3. **Compression parameters:**
   - Set pointer cost h = 2 (or any fixed constant >= 2).
   - Set the compression bound B = f(k, n, m) such that achieving |C| + (h-1) * (pointer count) <= B requires that the compressed string C contain explicit substrings for the vertex cover vertices, with other occurrences replaced by pointers referencing these substrings within C itself.

4. **Self-referencing structure:** In the internal macro scheme, C serves as its own dictionary. A vertex v in the cover appears explicitly at one position in C, and all other edge gadgets incident to v use pointers back to that position. Selecting k vertices for the cover means k explicit entries plus pointer references for all incident edges.

5. **Solution extraction:** Given a compressed string C with total cost <= B, identify which vertex substrings appear explicitly (non-pointer) in C. These vertices form a vertex cover: every edge gadget must reference at least one explicit vertex substring.

**Key invariant:** G has a vertex cover of size at most k if and only if string s can be internally compressed to total cost |C| + (h-1) * (pointer count) <= B. The internal compression cost is achievable when the "explicit" vertex entries in C correspond to a valid vertex cover.

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

**Derivation:** Similar to the external macro case. The alphabet encodes vertex identifiers. The string has one gadget per edge, giving length O(n + m). The pointer cost is fixed at 2. The bound B accounts for the single-string structure where dictionary entries are embedded within the compressed string itself, requiring slightly different accounting than the external case.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance to InternalMacroDataCompression, solve the compression problem via brute-force enumeration of compressed strings with pointers, extract the implied vertex cover, verify it is a valid cover on the original graph
- Check that the minimum internal compression cost corresponds to the minimum vertex cover size
- Test with a path graph P_6 (simple structure, VC = alternating vertices) to verify correctness on non-trivial graphs
- Compare results with the external macro variant (R116): both reductions should produce consistent answers from the same vertex cover instance

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices {0, 1, 2, 3, 4, 5} and 7 edges:
- Edges: {0,1}, {0,2}, {1,2}, {1,3}, {2,4}, {3,4}, {3,5}
- Minimum vertex cover: {1, 2, 3} of size k = 3
  - {0,1} by 1, {0,2} by 2, {1,2} by 1+2, {1,3} by 1+3, {2,4} by 2, {3,4} by 3, {3,5} by 3

**Constructed target instance (InternalMacroDataCompression):**
Using the reduction with h = 2:
- Alphabet Sigma = {v0, v1, v2, v3, v4, v5, #}
- String s encodes the 7 edges: each edge as a vertex-pair gadget
  - |s| = O(20)
- Bound B is set so that internal compression is achievable with 3 explicit vertex entries

**Solution mapping:**
- Compressed string C contains explicit substrings for vertices {1, 2, 3}
- Other occurrences of v1, v2, v3 in edge gadgets are replaced by internal pointers referencing earlier positions in C
- Vertices {0, 4, 5} appear as literals where they are not covered by pointers
- Each pointer costs h = 2, contributing (2-1) = 1 to the total per pointer occurrence
- Total cost: |C| + pointer_overhead <= B
- Extracted vertex cover: {1, 2, 3} (vertices with explicit non-pointer entries that are referenced)
- Verification: all 7 edges covered


## References

- **[Storer, 1977]**: [`Storer1977`] James A. Storer (1977). "{NP}-completeness results concerning data compression". Dept. of Electrical Engineering and Computer Science, Princeton University.
- **[Storer and Szymanski, 1978]**: [`Storer and Szymanski1978`] James A. Storer and Thomas G. Szymanski (1978). "The macro model for data compression (Extended abstract)". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 30-39. Association for Computing Machinery.
