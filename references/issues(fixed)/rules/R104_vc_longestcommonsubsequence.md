---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Longest Common Subsequence"
labels: rule
assignees: ''
---

**Source:** Vertex Cover
**Target:** Longest Common Subsequence
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.228

## GJ Source Entry

> [SR10] LONGEST COMMON SUBSEQUENCE
> INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
> QUESTION: Is there a string w E Σ* with |w| >= K such that w is a subsequence of each x E R?
> Reference: [Maier, 1978]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if |Σ| = 2. Solvable in polynomial time for any fixed K or for fixed |R| (by dynamic programming, e.g., see [Wagner and Fischer, 1974]). The analogous LONGEST COMMON SUBSTRING problem is trivially solvable in polynomial time.

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Maier, 1978]**: [`Maier1978`] David Maier (1978). "The complexity of some problems on subsequences and supersequences". *Journal of the Association for Computing Machinery* 25, pp. 322–336.
- **[Wagner and Fischer, 1974]**: [`Wagner and Fischer1974`] Robert A. Wagner and Michael J. Fischer (1974). "The string-to-string correction problem". *Journal of the Association for Computing Machinery* 21, pp. 168–173.