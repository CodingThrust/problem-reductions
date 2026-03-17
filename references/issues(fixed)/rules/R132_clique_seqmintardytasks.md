---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Clique to Sequencing to Minimize Tardy Tasks"
labels: rule
assignees: ''
---

**Source:** Clique
**Target:** Sequencing to Minimize Tardy Tasks
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.236

## GJ Source Entry

> [SS2] SEQUENCING TO MINIMIZE TARDY TASKS
> INSTANCE: Set T of tasks, partial order < on T, for each task t E T a length l(t) E Z+ and a deadline d(t) E Z+, and a positive integer K <= |T|.
> QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints, i.e., such that t < t' implies σ(t) + l(t) < σ(t'), and such that there are at most K tasks t E T for which σ(t) + l(t) > d(t)?
> Reference: [Garey and Johnson, 1976c]. Transformation from CLIQUE (see Section 3.2.3).
> Comment: Remains NP-complete even if all task lengths are 1 and < consists only of "chains" (each task has at most one immediate predecessor and at most one immediate successor) [Lenstra, 1977]. The general problem can be solved in polynomial time if K = 0 [Lawler, 1973], or if < is empty [Moore, 1968] [Sidney, 1973]. The < empty case remains polynomially solvable if "agreeable" release times (i.e., r(t) < r(t') implies d(t) <= d(t')) are added [Kise, Ibaraki, and Mine, 1978], but is NP-complete for arbitrary release times (see previous problem).

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

- **[Garey and Johnson, 1976c]**: [`Garey1976c`] M. R. Garey and D. S. Johnson (1976). "The complexity of near-optimal graph coloring". *Journal of the Association for Computing Machinery* 23, pp. 43–49.
- **[Lenstra, 1977]**: [`Lenstra1977`] Jan K. Lenstra (1977). "".
- **[Lawler, 1973]**: [`Lawler1973`] Eugene L. Lawler (1973). "Optimal sequencing of a single machine subject to precedence constraints". *Management Science* 19, pp. 544–546.
- **[Moore, 1968]**: [`Moore1968`] J. M. Moore (1968). "An $n$ job, one machine sequencing algorithm for minimizing the number of late jobs". *Management Science* 15, pp. 102–109.
- **[Sidney, 1973]**: [`Sidney1973`] Jeffrey B. Sidney (1973). "An extension of {Moore}'s due date algorithm". In: *Symposium on the Theory of Scheduling and its Applications*. Springer.
- **[Kise, Ibaraki, and Mine, 1978]**: [`Kise1978`] Hiroshi Kise and Toshihide Ibaraki and Hisashi Mine (1978). "A solvable case of the one-machine scheduling problem with ready and due times". *Operations Research* 26, pp. 121–126.