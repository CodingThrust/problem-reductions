---
name: Rule
about: Propose a new reduction rule
title: "[Rule] CLIQUE to PARTIALLY ORDERED KNAPSACK"
labels: rule
assignees: ''
---

**Source:** CLIQUE
**Target:** PARTIALLY ORDERED KNAPSACK
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.247-248

## GJ Source Entry

> [MP12] PARTIALLY ORDERED KNAPSACK
> INSTANCE: Finite set U, partial order < on U, for each u E U a size s(u) E Z+ and a value v(u) E Z+, positive integers B and K.
> QUESTION: Is there a subset U' ⊆ U such that if u E U' and u' < u, then u' E U', and such that Σ_{u E U'} s(u) ≤ B and Σ_{u E U'} v(u) ≥ K?
> Reference: [Garey and Johnson, ——]. Transformation from CLIQUE. Problem is discussed in [Ibarra and Kim, 1975b].
> Comment: NP-complete in the strong sense, even if s(u) = v(u) for all u E U. General problem is solvable in pseudo-polynomial time if < is a "tree" partial order [Garey and Johnson, ——].

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

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Ibarra and Kim, 1975b]**: [`Ibarra1975b`] Oscar H. Ibarra and Chul E. Kim (1975). "Scheduling for maximum profit". Computer Science Dept., University of Minnesota.