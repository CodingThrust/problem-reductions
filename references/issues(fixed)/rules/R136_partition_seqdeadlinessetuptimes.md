---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Sequencing with Deadlines and Set-Up Times"
labels: rule
assignees: ''
---

**Source:** Partition
**Target:** Sequencing with Deadlines and Set-Up Times
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.238

## GJ Source Entry

> [SS6] SEQUENCING WITH DEADLINES AND SET-UP TIMES
> INSTANCE: Set C of "compilers," set T of tasks, for each t E T a length l(t) E Z+, a deadline d(t) E Z+, and a compiler k(t) E C, and for each c E C a "set-up time" l(c) E Z_0+.
> QUESTION: Is there a one-processor schedule σ for T that meets all the task deadlines and that satisfies the additional constraint that, whenever two tasks t and t' with σ(t) < σ(t') are scheduled "consecutively" (i.e., no other task t'' has σ(t) < σ(t'') < σ(t')) and have different compilers (i.e., k(t) ≠ k(t')), then σ(t') >= σ(t) + l(t) + l(k(t'))?
> Reference: [Bruno and Downey, 1978]. Transformation from PARTITION.
> Comment: Remains NP-complete even if all set-up times are equal. The related problem in which set-up times are replaced by "changeover costs," and we want to know if there is a schedule that meets all the deadlines and has total changeover cost at most K, is NP-complete even if all changeover costs are equal. Both problems can be solved in pseudo-polynomial time when the number of distinct deadlines is bounded by a constant. If the number of deadlines is unbounded, it is open whether these problems are NP-complete in the strong sense.

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

- **[Bruno and Downey, 1978]**: [`Bruno1978`] J. Bruno and P. Downey (1978). "Complexity of task scheduling with deadlines, set-up times and changeover costs". *SIAM Journal on Computing*.