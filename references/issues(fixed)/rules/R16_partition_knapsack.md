---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to KNAPSACK"
labels: rule
assignees: ''
---

**Source:** PARTITION
**Target:** KNAPSACK
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.247

## GJ Source Entry

> [MP9] KNAPSACK
> INSTANCE: Finite set U, for each u E U a size s(u) E Z+ and a value v(u) E Z+, and positive integers B and K.
> QUESTION: Is there a subset U' ⊆ U such that Σ_{u E U'} s(u) ≤ B and such that Σ_{u E U'} v(u) ≥ K?
> Reference: [Karp, 1972]. Transformation from PARTITION.
> Comment: Remains NP-complete if s(u) = v(u) for all u E U (SUBSET SUM). Can be solved in pseudo-polynomial time by dynamic programming (e.g., see [Dantzig, 1957] or [Lawler, 1976a]).

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

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Dantzig, 1957]**: [`Dantzig1957`] G. B. Dantzig (1957). "Discrete-variable extremum problems". *Operations Research* 5, pp. 266–277.
- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.