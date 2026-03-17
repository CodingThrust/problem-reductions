---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Register Sufficiency to Sequencing to Minimize Maximum Cumulative Cost"
labels: rule
assignees: ''
---

**Source:** Register Sufficiency
**Target:** Sequencing to Minimize Maximum Cumulative Cost
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.238

## GJ Source Entry

> [SS7] SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST
> INSTANCE: Set T of tasks, partial order < on T, a "cost" c(t) E Z for each t E T (if c(t) < 0, it can be viewed as a "profit"), and a constant K E Z.
> QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints and which has the property that, for every task t E T, the sum of the costs for all tasks t' with σ(t') <= σ(t) is at most K?
> Reference: [Abdel-Wahab, 1976]. Transformation from REGISTER SUFFICIENCY.
> Comment: Remains NP-complete even if c(t) E {-1,0,1} for all t E T. Can be solved in polynomial time if < is series-parallel [Abdel-Wahab and Kameda, 1978], [Monma and Sidney, 1977].

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

- **[Abdel-Wahab, 1976]**: [`Abdel-Wahab1976`] H. M. Abdel-Wahab (1976). "Scheduling with Applications to Register Allocation and Deadlock Problems". University of Waterloo.
- **[Abdel-Wahab and Kameda, 1978]**: [`Abdel-Wahab1978`] H. M. Abdel-Wahab and T. Kameda (1978). "Scheduling to minimize maximum cumulative cost subject to series-parallel precedence constraints". *Operations Research* 26, pp. 141–158.
- **[Monma and Sidney, 1977]**: [`Monma1977`] Clyde L. Monma and J. B. Sidney (1977). "A general algorithm for optimal job sequencing with series-parallel precedence constraints". School of Operations Research, Cornell University.