---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Subset Sum to Capacity Assignment"
labels: rule
assignees: ''
---

**Source:** Subset Sum
**Target:** Capacity Assignment
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.1, p.227

## GJ Source Entry

> [SR7] CAPACITY ASSIGNMENT
> INSTANCE: Set C of communication links, set M ⊆ Z+ of capacities, cost function g: C×M → Z+, delay penalty function d: C×M → Z+ such that, for all c E C and i < j E M, g(c,i) <= g(c,j) and d(c,i) >= d(c,j), and positive integers K and J.
> QUESTION: Is there an assignment σ: C → M such that the total cost sum_{c E C} g(c,σ(c)) does not exceed K and such that the total delay penalty sum_{c E C} d(c,σ(c)) does not exceed J?
> Reference: [Van Sickle and Chandy, 1977]. Transformation from SUBSET SUM.
> Comment: Solvable in pseudo-polynomial time.

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

- **[Van Sickle and Chandy, 1977]**: [`van Sickle and Chandy1977`] Larry van Sickle and K. Mani Chandy (1977). "The complexity of computer network design problems".