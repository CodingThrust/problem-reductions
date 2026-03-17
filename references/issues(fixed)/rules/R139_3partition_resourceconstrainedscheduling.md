---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Resource Constrained Scheduling"
labels: rule
assignees: ''
---

**Source:** 3-Partition
**Target:** Resource Constrained Scheduling
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.239

## GJ Source Entry

> [SS10] RESOURCE CONSTRAINED SCHEDULING
> INSTANCE: Set T of tasks, each having length l(t) = 1, number m E Z+ of processors, number r E Z+ of resources, resource bounds B_i, 1 <= i <= r, resource requirement R_i(t), 0 <= R_i(t) <= B_i, for each task t and resource i, and an overall deadline D E Z+.
> QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the resource constraints, i.e., such that for all u >= 0, if S(u) is the set of all t E T for which σ(t) <= u < σ(t) + l(t), then for each resource i the sum of R_i(t) over all t E S(u) is at most B_i?
> Reference: [Garey and Johnson, 1975]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense, even if r = 1 and m = 3. Can be solved in polynomial time by matching for m = 2 and r arbitrary. If a partial order < is added, the problem becomes NP-complete in the strong sense for r = 1, m = 2, and < a "forest." If each resource requirement is restricted to be either 0 or B_i, the problem is NP-complete for m = 2, r = 1, and < arbitrary [Ullman, 1976].

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

- **[Garey and Johnson, 1975]**: [`Garey1975`] M. R. Garey and D. S. Johnson (1975). "Complexity results for multiprocessor scheduling under resource constraints". *SIAM Journal on Computing* 4, pp. 397–411.
- **[Ullman, 1976]**: [`Ullman1976`] Jeffrey D. Ullman (1976). "Complexity of sequencing problems". In: *Computer and Job/Shop Scheduling Theory*. John Wiley \& Sons.