---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Sequencing with Release Times and Deadlines"
labels: rule
assignees: ''
---

**Source:** 3-Partition
**Target:** Sequencing with Release Times and Deadlines
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.236

## GJ Source Entry

> [SS1] SEQUENCING WITH RELEASE TIMES AND DEADLINES
> INSTANCE: Set T of tasks and, for each task t E T, a length l(t) E Z+, a release time r(t) E Z_0+, and a deadline d(t) E Z+.
> QUESTION: Is there a one-processor schedule for T that satisfies the release time constraints and meets all the deadlines, i.e., a one-to-one function σ: T → Z_0+, with σ(t) > σ(t') implying σ(t) >= σ(t') + l(t'), such that, for all t E T, σ(t) >= r(t) and σ(t) + l(t) <= d(t)?
> Reference: [Garey and Johnson, 1977b]. Transformation from 3-PARTITION (see Section 4.2).
> Comment: NP-complete in the strong sense. Solvable in pseudo-polynomial time if the number of allowed values for r(t) and d(t) is bounded by a constant, but remains NP-complete (in the ordinary sense) even when each can take on only two values. If all task lengths are 1, or "preemptions" are allowed, or all release times are 0, the general problem can be solved in polynomial time, even under "precedence constraints" [Lawler, 1973], [Lageweg, Lenstra, and Rinnooy Kan, 1976]. Can also be solved in polynomial time even if release times and deadlines are allowed to be arbitrary rationals and there are precedence constraints, so long as all tasks have equal length [Carlier, 1978], [Simons, 1978], [Garey, Johnson, Simons, and Tarjan, 1978], or preemptions are allowed [Blazewicz, 1976].

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

- **[Garey and Johnson, 1977b]**: [`Garey1977c`] M. R. Garey and D. S. Johnson (1977). "The rectilinear {Steiner} tree problem is {NP}-complete". *SIAM Journal on Applied Mathematics* 32, pp. 826–834.
- **[Lawler, 1973]**: [`Lawler1973`] Eugene L. Lawler (1973). "Optimal sequencing of a single machine subject to precedence constraints". *Management Science* 19, pp. 544–546.
- **[Lageweg, Lenstra, and Rinnooy Kan, 1976]**: [`Lageweg1976`] B. J. Lageweg and Jan K. Lenstra and A. H. G. Rinnooy Kan (1976). "Minimizing maximum lateness on one machine: computational experience and some applications". *Statistica Neerlandica* 30, pp. 25–41.
- **[Carlier, 1978]**: [`Carlier1978`] J. Carlier (1978). "Probl{\`e}me a une machine". Universit{\'e} de Pierre et Marie Curie.
- **[Simons, 1978]**: [`Simons1978`] Barbara Simons (1978). "A fast algorithm for single processor scheduling". In: *Proc. 19th Ann. Symp. on Foundations of Computer Science*, pp. 246–252. IEEE Computer Society.
- **[Garey, Johnson, Simons, and Tarjan, 1978]**: [`Garey1978d`] M. R. Garey and D. S. Johnson and B. B. Simons and R. E. Tarjan (1978). "Scheduling unit time tasks with arbitrary release times and deadlines".
- **[Blazewicz, 1976]**: [`Blazewicz1976`] J. Blazewicz (1976). "Scheduling dependent tasks with different arrival times to meet deadlines". In: *Modelling and Performance Evaluation of Computer Systems*. North Holland.