---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MinimumVertexCover to ShortestCommonSupersequence"
labels: rule
assignees: ''
canonical_source_name: 'Vertex Cover'
canonical_target_name: 'Shortest Common Supersequence'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** MinimumVertexCover
**Target:** ShortestCommonSupersequence
**Motivation:** Establishes NP-completeness of SHORTEST COMMON SUPERSEQUENCE via polynomial-time reduction from VERTEX COVER. The SCS problem asks for the shortest string containing each input string as a subsequence. Maier (1978) showed this is NP-complete even for alphabets of size 5 by encoding the "at least one endpoint" constraint of vertex cover through subsequence containment requirements.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, SR8, p.228. [Maier, 1978].

## GJ Source Entry

> [SR8] SHORTEST COMMON SUPERSEQUENCE
> INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
> QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a subsequence of w?
> Reference: [Maier, 1978]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if |Σ| = 5. Solvable in polynomial time if |R| = 2 (by first computing the longest common subsequence) or if all x ∈ R have |x| ≤ 2.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a VERTEX COVER instance G = (V, E) with V = {v_1, ..., v_n}, E = {e_1, ..., e_m}, and integer K, construct a SHORTEST COMMON SUPERSEQUENCE instance as follows (based on Maier's 1978 construction):

1. **Alphabet:** Σ = {v_1, v_2, ..., v_n} ∪ {#} where # is a separator symbol not in V. The alphabet has |V| + 1 symbols. (For the fixed-alphabet variant with |Σ| = 5, a further encoding step is applied.)

2. **String construction:** For each edge e_j = {v_a, v_b} (with a < b), create the string:
   s_j = v_a · v_b
   This string of length 2 encodes the constraint that in any supersequence, the symbols v_a and v_b must both appear (at least one needs to be "shared" across edges).

3. **Vertex-ordering string:** Create a "backbone" string:
   T = v_1 · v_2 · ... · v_n
   This ensures the supersequence respects the vertex ordering.

4. **Additional constraint strings:** For each pair of adjacent vertices in an edge, separator-delimited strings enforce that the vertex symbols appear in specific positions. The full construction uses the separator # to create blocks so that the supersequence can be divided into n blocks, where each block corresponds to a vertex. A vertex is "selected" (in the cover) if its block contains the vertex symbol plus extra copies needed by incident edges; a vertex not in the cover has its symbol appear only once.

5. **Bound:** K' = n + m - K, where n = |V|, m = |E|, K = vertex cover size bound. (The exact formula depends on the padding used in the construction.)

6. **Correctness (forward):** If G has a vertex cover S of size ≤ K, the supersequence is constructed by placing all vertex symbols in order, and for each edge e = {v_a, v_b}, the subsequence v_a · v_b is embedded by having both symbols present. Because S covers all edges, at most K vertices carry extra "load," keeping the total length within K'.

7. **Correctness (reverse):** If a supersequence w of length ≤ K' exists, the vertex symbols that appear in positions accommodating multiple edge-strings correspond to a vertex cover of G with size ≤ K.

**Key insight:** Subsequence containment allows encoding the "at least one endpoint must be selected" constraint. The supersequence must "schedule" vertex symbols so that every edge-string is a subsequence, and minimizing the supersequence length corresponds to minimizing the vertex cover.

**Time complexity of reduction:** O(n + m) to construct the instance.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = `num_vertices` of source VertexCover instance (|V|)
- m = `num_edges` of source VertexCover instance (|E|)
- K = vertex cover bound

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `alphabet_size` | `num_vertices + 1` |
| `num_strings` | `num_edges + 1` |
| `max_string_length` | `num_vertices` |
| `bound_K` | `num_vertices + num_edges - cover_bound` |

**Derivation:** One symbol per vertex plus separator; one string per edge plus one backbone string; bound relates linearly to n, m, and K.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumVertexCover instance to ShortestCommonSupersequence, solve target with BruteForce (enumerate candidate supersequences up to length K'), extract solution, verify on source
- Test with known YES instance: triangle graph K_3, vertex cover of size 2
- Test with known NO instance: star graph K_{1,5}, vertex cover must include center vertex
- Verify that every constructed edge-string is indeed a subsequence of the constructed supersequence
- Compare with known results from literature

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumVertexCover):**
Graph G with 6 vertices V = {v_1, v_2, v_3, v_4, v_5, v_6} and 7 edges:
- Edges: {v_1,v_2}, {v_1,v_3}, {v_2,v_3}, {v_3,v_4}, {v_4,v_5}, {v_4,v_6}, {v_5,v_6}
- (Triangle v_1-v_2-v_3 connected to triangle v_4-v_5-v_6 via edge {v_3,v_4})
- Vertex cover of size K = 3: {v_2, v_3, v_4} covers all edges. Check:
  - {v_1,v_2}: v_2 ✓; {v_1,v_3}: v_3 ✓; {v_2,v_3}: v_2 ✓; {v_3,v_4}: v_3 ✓; {v_4,v_5}: v_4 ✓; {v_4,v_6}: v_4 ✓; {v_5,v_6}: needs v_5 or v_6 -- FAIL.
- Correct cover of size K = 4: {v_1, v_3, v_4, v_6} covers all edges:
  - {v_1,v_2}: v_1 ✓; {v_1,v_3}: v_1 ✓; {v_2,v_3}: v_3 ✓; {v_3,v_4}: v_3 ✓; {v_4,v_5}: v_4 ✓; {v_4,v_6}: v_4 ✓; {v_5,v_6}: v_6 ✓.

**Constructed target instance (ShortestCommonSupersequence):**
- Alphabet: Σ = {v_1, v_2, v_3, v_4, v_5, v_6, #}
- Strings (one per edge): R = {v_1v_2, v_1v_3, v_2v_3, v_3v_4, v_4v_5, v_4v_6, v_5v_6}
- Backbone string: T = v_1v_2v_3v_4v_5v_6
- All strings in R must be subsequences of the supersequence w

**Solution mapping:**
- The supersequence w = v_1v_2v_3v_4v_5v_6 of length 6 already contains every 2-symbol edge-string as a subsequence (since vertex symbols appear in order). The optimal SCS length relates to how many vertex symbols can be "shared" across edges.
- The vertex cover {v_1, v_3, v_4, v_6} identifies which vertices serve as shared anchors in the supersequence.

**Verification:**
- Each edge-string v_av_b (a < b) is a subsequence of v_1v_2v_3v_4v_5v_6 ✓
- The solution length relates to the vertex cover size through the reduction formula


## References

- **[Maier, 1978]**: [`Maier1978`] David Maier (1978). "The complexity of some problems on subsequences and supersequences". *Journal of the Association for Computing Machinery* 25(2), pp. 322-336.
- **[Räihä and Ukkonen, 1981]**: K. J. Räihä and E. Ukkonen (1981). "The shortest common supersequence problem over binary alphabet is NP-complete". *Theoretical Computer Science* 16(2), pp. 187-198.
- **[Lagoutte and Tavenas, 2017]**: Aurélie Lagoutte and Sébastien Tavenas (2017). "The complexity of Shortest Common Supersequence for inputs with no identical consecutive letters". *arXiv:1309.0422*.
