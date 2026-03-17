---
name: Rule
about: Propose a new reduction rule
title: "[Rule] FEEDBACK EDGE SET to GROUPING BY SWAPPING"
labels: rule
assignees: ''
canonical_source_name: 'FEEDBACK EDGE SET'
canonical_target_name: 'GROUPING BY SWAPPING'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** FEEDBACK EDGE SET
**Target:** GROUPING BY SWAPPING
**Motivation:** Establishes NP-completeness of GROUPING BY SWAPPING via polynomial-time reduction from FEEDBACK EDGE SET. This shows that the problem of sorting a string into grouped blocks (where all occurrences of each symbol are contiguous) using a minimum number of adjacent transpositions is computationally hard, connecting graph cycle structure to string rearrangement complexity.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.231

## GJ Source Entry

> [SR21] GROUPING BY SWAPPING
> INSTANCE: Finite alphabet Σ, string x E Σ*, and a positive integer K.
> QUESTION: Is there a sequence of K or fewer adjacent symbol interchanges that converts x into a string y in which all occurrences of each symbol a E Σ are in a single block, i.e., y has no subsequences of the form aba for a,b E Σ and a ≠ b?
> Reference: [Howell, 1977]. Transformation from FEEDBACK EDGE SET.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a FEEDBACK EDGE SET instance (G, K) where G = (V, E) is an undirected graph and K is a budget for edge removal to make G acyclic, construct a GROUPING BY SWAPPING instance as follows:

1. **Alphabet construction:** Create an alphabet Sigma with one symbol for each vertex v in V. That is, |Sigma| = |V|.

2. **String construction:** Construct the string x from the graph G by encoding the edge structure. For each edge {u, v} in E, the symbols u and v must be interleaved in x so that grouping them requires adjacent swaps. The string is constructed by traversing the edges and creating a sequence where vertices sharing an edge have their symbols interleaved -- specifically, for each cycle in G, the symbols of the cycle's vertices appear in an order that requires swaps proportional to the cycle length to unscramble.

3. **Budget parameter:** Set the swap budget K' to be a function of K and the graph structure. The key insight is that each edge in a feedback edge set corresponds to a "crossing" in the string that must be resolved by a swap. Removing an edge from a cycle in G corresponds to performing swaps to separate the interleaved occurrences of the corresponding vertex symbols.

4. **Solution extraction:** Given a sequence of at most K' adjacent swaps that groups the string, identify which "crossings" were resolved. The edges corresponding to these crossings form a feedback edge set of size at most K in G.

**Key invariant:** G has a feedback edge set of size at most K if and only if the string x can be grouped (all occurrences of each symbol contiguous) using at most K' adjacent transpositions. Cycles in G correspond to interleaving patterns in x that require swaps to resolve, and breaking each cycle requires resolving at least one crossing.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |V| = number of vertices in G
- m = |E| = number of edges in G

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `alphabet_size` | n |
| `string_length` | O(m + n) |
| `budget` | polynomial in K, n, m |

**Derivation:** The alphabet has one symbol per vertex. Each edge contributes a constant number of symbol occurrences to the string, so the string length is O(m + n). The budget K' is derived from K and the graph structure, maintaining the correspondence between feedback edges and swap operations needed to resolve interleaving patterns.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a Feedback Edge Set instance to GroupingBySwapping, solve the grouping problem via brute-force enumeration of swap sequences, extract the implied feedback edge set, verify it makes the original graph acyclic
- Check that the minimum number of swaps to group the string corresponds to the minimum feedback edge set size
- Test with a graph containing multiple independent cycles (each cycle requires at least one feedback edge) to verify the budget is correctly computed
- Verify with a tree (acyclic graph) that zero swaps are needed (string is already groupable or trivially groupable)

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (Feedback Edge Set):**
Graph G with 6 vertices {a, b, c, d, e, f} and 7 edges:
- Edges: {a,b}, {b,c}, {c,a}, {c,d}, {d,e}, {e,f}, {f,d}
- Two triangles: (a,b,c) and (d,e,f), connected by edge {c,d}
- Minimum feedback edge set size: K = 2 (remove one edge from each triangle, e.g., {c,a} and {f,d})

**Constructed target instance (GroupingBySwapping):**
Using the reduction:
- Alphabet Sigma = {a, b, c, d, e, f}
- String x is constructed from the graph structure. The triangles create interleaving patterns:
  - Triangle (a,b,c): symbols a, b, c are interleaved, e.g., subsequence "abcabc"
  - Triangle (d,e,f): symbols d, e, f are interleaved, e.g., subsequence "defdef"
  - Edge {c,d} links the two groups
- The resulting string x might look like: "a b c a b c d e f d e f" with careful interleaving of shared edges
- Budget K' is set based on K=2 and the encoding

**Solution mapping:**
- A minimum swap sequence groups the string by resolving exactly 2 interleaving crossings
- These crossings correspond to feedback edges {c,a} and {f,d}
- Removing {c,a} from triangle (a,b,c) and {f,d} from triangle (d,e,f) makes G acyclic
- The resulting graph is a tree/forest, confirming a valid feedback edge set of size 2

**Note:** The exact string encoding depends on Howell's 1977 construction, which carefully maps cycle structure to symbol interleaving patterns.


## References

- **[Howell, 1977]**: [`Howell1977`] Thomas D. Howell (1977). "Grouping by swapping is {NP}-complete".
