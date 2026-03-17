---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Sequencing to Minimize Tardy Task Weight"
labels: rule
assignees: ''
---

**Source:** Partition
**Target:** Sequencing to Minimize Tardy Task Weight
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.236-237

## GJ Source Entry

> [SS3] SEQUENCING TO MINIMIZE TARDY TASK WEIGHT
> INSTANCE: Set T of tasks, for each task t E T a length l(t) E Z+, a weight w(t) E Z+, and a deadline d(t) E Z+, and a positive integer K.
> QUESTION: Is there a one-processor schedule σ for T such that the sum of w(t), taken over all t E T for which σ(t) + l(t) > d(t), does not exceed K?
> Reference: [Karp, 1972]. Transformation from PARTITION.
> Comment: Can be solved in pseudo-polynomial time (time polynomial in |T|, sum l(t), and log sum w(t)) [Lawler and Moore, 1969]. Can be solved in polynomial time if weights are "agreeable" (i.e., w(t) < w(t') implies l(t) >= l(t')) [Lawler, 1976c].

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
- **[Lawler and Moore, 1969]**: [`Lawler1969`] Eugene L. Lawler and J. M. Moore (1969). "A functional equation and its application to resource allocation and sequencing problems". *Management Science* 16, pp. 77–84.
- **[Lawler, 1976c]**: [`Lawler1976c`] Eugene L. Lawler (1976). "Sequencing to minimize the weighted number of tardy jobs". *Revue Francaise d'Automatique, Informatique et Recherche Operationnelle, Serie Bleue* 10.5, pp. 27–33.