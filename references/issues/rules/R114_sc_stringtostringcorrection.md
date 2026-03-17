---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SET COVERING to STRING-TO-STRING CORRECTION"
labels: rule
assignees: ''
canonical_source_name: 'SET COVERING'
canonical_target_name: 'STRING-TO-STRING CORRECTION'
source_in_codebase: true
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** SET COVERING
**Target:** STRING-TO-STRING CORRECTION
**Motivation:** Establishes NP-completeness of STRING-TO-STRING CORRECTION (with deletion and adjacent-symbol interchange only) via polynomial-time reduction from SET COVERING. This reduction, due to Wagner (1975), shows that the restricted edit distance problem with only swap and delete operations is computationally hard, even though the problem becomes polynomial-time solvable when additional operations (insert, change) are allowed or when only swaps are permitted.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.230

## GJ Source Entry

> [SR20] STRING-TO-STRING CORRECTION
> INSTANCE: Finite alphabet Σ, two strings x,y E Σ*, and a positive integer K.
> QUESTION: Is there a way to derive the string y from the string x by a sequence of K or fewer operations of single symbol deletion or adjacent symbol interchange?
> Reference: [Wagner, 1975]. Transformation from SET COVERING.
> Comment: Solvable in polynomial time if the operation set is expanded to include the operations of changing a single character and of inserting a single character, even if interchanges are not allowed (e.g., see [Wagner and Fischer, 1974]), or if the only operation is adjacent symbol interchange [Wagner, 1975]. See reference for related results for cases in which different operations can have different costs.

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a SET COVERING instance (S, C, K) where S is a universe of m elements, C = {C_1, ..., C_n} is a collection of n subsets of S, and K is a budget, construct a STRING-TO-STRING CORRECTION instance as follows:

1. **Alphabet construction:** Create a finite alphabet Sigma with one distinct symbol for each element of S plus additional separator/marker symbols. Specifically, use symbols a_1, ..., a_m for the m universe elements, plus additional structural symbols to encode the covering structure. The alphabet size is O(m + n).

2. **Source string x construction:** Construct the source string x that encodes the structure of the set covering instance. For each subset C_j in C, create a "block" in the string containing the symbols corresponding to elements in C_j, arranged so that selecting subset C_j corresponds to performing a bounded number of swap and delete operations on that block. Blocks are separated by marker symbols. The source string has length O(m * n).

3. **Target string y construction:** Construct the target string y that represents the "goal" configuration, where the elements are grouped/ordered in a way that can only be achieved from x by selecting at most K subsets worth of edit operations.

4. **Budget parameter:** Set the edit distance bound K' = f(K, m, n) for some polynomial function f that ensures K or fewer subsets can cover S if and only if K' or fewer swap/delete operations transform x into y.

5. **Solution extraction:** Given a sequence of at most K' edit operations transforming x to y, decode which subsets were effectively "selected" by examining which blocks were modified, recovering a set cover of size at most K.

**Key invariant:** A set cover of S using at most K subsets from C exists if and only if string y can be derived from string x using at most K' swap and delete operations.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- m = number of universe elements in S
- n = number of subsets in C (i.e., `num_sets`)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `alphabet_size` | O(m + n) |
| `string_length_x` | O(m * n) |
| `string_length_y` | O(m * n) |
| `budget` | polynomial in K, m, n |

**Derivation:** The alphabet must have enough distinct symbols to encode each universe element and structural separators. Each subset contributes a block to the source string proportional to the number of elements it contains, giving total string length polynomial in m and n. The target string has comparable length. The exact polynomial form depends on the specific encoding details in Wagner's 1975 construction.

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: reduce a MinimumSetCovering instance to StringToStringCorrection, solve the target with brute-force enumeration of edit operation sequences, extract the implied set cover, verify it is a valid cover on the original instance
- Check that the minimum edit distance equals the budget threshold exactly when a minimum set cover of the required size exists
- Test with a set covering instance where greedy fails (e.g., elements covered by overlapping subsets requiring non-obvious selection)
- Verify polynomial blow-up: string lengths and alphabet size should be polynomial in the original instance size

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (MinimumSetCovering):**
Universe S = {1, 2, 3, 4, 5, 6}, Collection C:
- C_1 = {1, 2, 3}
- C_2 = {2, 4, 5}
- C_3 = {3, 5, 6}
- C_4 = {1, 4, 6}
Budget K = 2

Minimum set cover: {C_1, C_3} = {1,2,3} ∪ {3,5,6} = {1,2,3,5,6} -- does not cover 4.
Try: {C_2, C_4} = {2,4,5} ∪ {1,4,6} = {1,2,4,5,6} -- does not cover 3.
Try: {C_1, C_2} = {1,2,3} ∪ {2,4,5} = {1,2,3,4,5} -- does not cover 6.
No cover of size 2 exists. A cover of size 3 is needed, e.g., {C_1, C_2, C_3}.

**Constructed target instance (StringToStringCorrection):**
Using the reduction, construct:
- Alphabet Sigma with symbols {a, b, c, d, e, f, #, $} (one per element plus separators)
- Source string x encodes the subset structure with separator-delimited blocks
- Target string y encodes the desired grouped configuration
- Budget K' computed from K=2 and the instance parameters

**Solution mapping:**
- Since no set cover of size 2 exists, the edit distance from x to y exceeds K', confirming the answer is NO for both instances
- Increasing K to 3 would yield a valid set cover {C_1, C_2, C_3}, and correspondingly the edit distance from x to y would be at most K'(3)

**Note:** The exact string constructions depend on Wagner's specific encoding, which maps subset selection to sequences of adjacent swaps and deletions in a carefully designed string pair.


## References

- **[Wagner, 1975]**: [`Wagner1975`] Robert A. Wagner (1975). "On the complexity of the extended string-to-string correction problem". In: *Proc. 7th Ann. ACM Symp. on Theory of Computing*, pp. 218-223. Association for Computing Machinery.
- **[Wagner and Fischer, 1974]**: [`Wagner and Fischer1974`] Robert A. Wagner and Michael J. Fischer (1974). "The string-to-string correction problem". *Journal of the Association for Computing Machinery* 21, pp. 168-173.
