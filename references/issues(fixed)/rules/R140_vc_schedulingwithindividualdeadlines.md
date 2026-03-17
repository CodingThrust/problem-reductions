---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Scheduling with Individual Deadlines"
labels: rule
assignees: ''
---

**Source:** Vertex Cover
**Target:** Scheduling with Individual Deadlines
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.239-240

## GJ Source Entry

> [SS11] SCHEDULING WITH INDIVIDUAL DEADLINES
> INSTANCE: Set T of tasks, each having length l(t) = 1, number m E Z+ of processors, partial order < on T, and for each task t E T a deadline d(t) E Z+.
> QUESTION: Is there an m-processor schedule σ for T that obeys the precedence constraints and meets all the deadlines, i.e., σ(t) + l(t) <= d(t) for all t E T?
> Reference: [Brucker, Garey, and Johnson, 1977]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if < is an "out-tree" partial order (no task has more than one immediate predecessor), but can be solved in polynomial time if < is an "in-tree" partial order (no task has more than one immediate successor). Solvable in polynomial time if m = 2 and < is arbitrary [Garey and Johnson, 1976c], even if individual release times are included [Garey and Johnson, 1977b]. For < empty, can be solved in polynomial time by matching for m arbitrary, even with release times and with a single resource having 0-1 valued requirements [Blazewicz, 1977b], [Blazewicz, 1978].

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

- **[Brucker, Garey, and Johnson, 1977]**: [`Brucker1977`] P. Brucker and M. R. Garey and D. S. Johnson (1977). "Scheduling equal-length tasks under treelike precedence constraints to minimize maximum lateness". *Mathematics of Operations Research* 2, pp. 275–284.
- **[Garey and Johnson, 1976c]**: [`Garey1976c`] M. R. Garey and D. S. Johnson (1976). "The complexity of near-optimal graph coloring". *Journal of the Association for Computing Machinery* 23, pp. 43–49.
- **[Garey and Johnson, 1977b]**: [`Garey1977c`] M. R. Garey and D. S. Johnson (1977). "The rectilinear {Steiner} tree problem is {NP}-complete". *SIAM Journal on Applied Mathematics* 32, pp. 826–834.
- **[Blazewicz, 1977b]**: [`Blazewicz1977b`] J. Blazewicz (1977). "Scheduling with deadlines and resource constraints". Technical University of Poznan.
- **[Blazewicz, 1978]**: [`Blazewicz1978`] J. Blazewicz (1978). "Deadline scheduling of tasks with ready times and resource constraints".