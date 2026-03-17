---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to Staff Scheduling"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** Staff Scheduling
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.4, p.243

## GJ Source Entry

> [SS20] STAFF SCHEDULING
> INSTANCE: Positive integers m and k, a collection C of m-tuples, each having k 1's and m-k 0's (representing possible worker schedules), a "requirement" m-tuple R-bar of non-negative integers, and a number n of workers.
> QUESTION: Is there a schedule f: C → Z_0+ such that sum_{c-bar E C} f(c-bar) <= n and such that sum_{c-bar E C} f(c-bar)*c-bar >= R-bar?
> Reference: [Garey and Johnson, ——] Transformation from X3C.
> Comment: Solvable in polynomial time if every c-bar E C has the cyclic one's property, i.e., has all its 1's occuring in consecutive positions with position 1 regarded as following position m [Bartholdi, Orlin, and Ratliff, 1977]. (This corresponds to workers who are available only for consecutive hours of the day, or days of the week.)

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
- **[Bartholdi, Orlin, and Ratliff, 1977]**: [`Bartholdi1977`] J. J. Bartholdi, III and J. B. Orlin and H. D. Ratliff (1977). "Circular ones and cyclic staffing". Stanford University.