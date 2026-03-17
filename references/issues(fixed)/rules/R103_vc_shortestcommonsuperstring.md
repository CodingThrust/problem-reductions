---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover (for cubic graphs) to Shortest Common Superstring"
labels: rule
assignees: ''
---

**Source:** Vertex Cover (for cubic graphs)
**Target:** Shortest Common Superstring
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.228

## GJ Source Entry

> [SR9] SHORTEST COMMON SUPERSTRING
> INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
> QUESTION: Is there a string w E Σ* with |w| <= K such that each string x E R is a substring of w, i.e., w = w_0*x*w_1 where each w_i E Σ*?
> Reference: [Maier and Storer, 1977]. Transformation from VERTEX COVER for cubic graphs.
> Comment: Remains NP-complete even if |Σ| = 2 or if all x E R have |x| <= 8 and contain no repeated symbols. Solvable in polynomial time if all x E R have |x| <= 2.

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

- **[Maier and Storer, 1977]**: [`Maier1977a`] David Maier and James A. Storer (1977). "A note on the complexity of the superstring problem". Computer Science Laboratory, Princeton University.