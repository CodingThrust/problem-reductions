---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SUBSET SUM to INTEGER KNAPSACK"
labels: rule
assignees: ''
---

**Source:** SUBSET SUM
**Target:** INTEGER KNAPSACK
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.247

## GJ Source Entry

> [MP10] INTEGER KNAPSACK
> INSTANCE: Finite set U, for each u E U a size s(u) E Z+ and a value v(u) E Z+, and positive integers B and K.
> QUESTION: Is there an assignment of a non-negative integer c(u) to each u E U such that Σ_{u E U} c(u)·s(u) ≤ B and such that Σ_{u E U} c(u)·v(u) ≥ K?
> Reference: [Lueker, 1975]. Transformation from SUBSET SUM.
> Comment: Remains NP-complete if s(u) = v(u) for all u E U. Solvable in pseudo-polynomial time by dynamic programming. Solvable in polynomial time if |U| = 2 [Hirschberg and Wong, 1976].

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

- **[Lueker, 1975]**: [`Lueker1975`] George S. Lueker (1975). "Two {NP}-complete problems in nonnegative integer programming". Computer Science Laboratory, Princeton University.
- **[Hirschberg and Wong, 1976]**: [`Hirschberg1976`] D. S. Hirschberg and C. K. Wong (1976). "A polynomial-time algorithm for the knapsack problem with two variables". *Journal of the Association for Computing Machinery* 23, pp. 147–154.