---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to CROSSWORD PUZZLE CONSTRUCTION"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** CROSSWORD PUZZLE CONSTRUCTION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.258

## GJ Source Entry

> [GP14] CROSSWORD PUZZLE CONSTRUCTION
> INSTANCE: A finite set W ⊆ Σ* of words and an n×n matrix A of 0's and 1's.
> QUESTION: Can an n×n crossword puzzle be built up from the words in W and blank squares corresponding to the 0's of A, i.e., if E is the set of pairs (i,j) such that A_{ij} = 0, is there an assignment f: E → Σ such that the letters assigned to any maximal horizontal or vertical contiguous sequence of members of E form, in order, a word of W?
> Reference: [Lewis and Papadimitriou, 1978]. Transformation from X3C.
> Comment: Remains NP-complete even if all entries in A are 0.

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

- **[Lewis and Papadimitriou, 1978]**: [`Lewis1978b`] Harry R. Lewis and Christos H. Papadimitriou (1978). "".