---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Numerical 3-Dimensional Matching to Two-Processor Flow-Shop with Bounded Buffer"
labels: rule
assignees: ''
---

**Source:** Numerical 3-Dimensional Matching
**Target:** Two-Processor Flow-Shop with Bounded Buffer
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.242

## GJ Source Entry

> [SS17] TWO-PROCESSOR FLOW-SHOP WITH BOUNDED BUFFER
> INSTANCE: (Same as for FLOW-SHOP SCHEDULING with m = 2, with the addition of a "buffer bound" B E Z_0+.)
> QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and such that, for all u >= 0, the number of jobs j E J for which both σ_1(j) + l(t_1[j]) <= u and σ_2(j) > u does not exceed B?
> Reference: [Papadimitriou and Kanellakis, 1978]. Transformation from NUMERICAL 3-DIMENSIONAL MATCHING.
> Comment: NP-complete in the strong sense for any fixed B, 1 <= B < ∞. Solvable in polynomial time if B = 0 [Gilmore and Gomory, 1964] or if B >= |J| - 1 [Johnson, 1954].

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

- **[Papadimitriou and Kanellakis, 1978]**: [`Papadimitriou1978e`] Christos H. Papadimitriou and P. C. Kanellakis (1978). "Flowshop scheduling with limited temporary storage".
- **[Gilmore and Gomory, 1964]**: [`Gilmore1964`] P. C. Gilmore and R. E. Gomory (1964). "Sequencing a one state-variable machine: a solvable case of the traveling salesman problem". *Operations Research* 12, pp. 655–679.
- **[Johnson, 1954]**: [`Johnson1954`] Selmer M. Johnson (1954). "Optimal two- and three-stage production schedules with setup times included". *Naval Research Logistics Quarterly* 1, pp. 61–68.