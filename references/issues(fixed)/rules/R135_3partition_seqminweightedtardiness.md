---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Sequencing to Minimize Weighted Tardiness"
labels: rule
assignees: ''
---

**Source:** 3-Partition
**Target:** Sequencing to Minimize Weighted Tardiness
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.237-238

## GJ Source Entry

> [SS5] SEQUENCING TO MINIMIZE WEIGHTED TARDINESS
> INSTANCE: Set T of tasks, for each task t E T a length l(t) E Z+, a weight w(t) E Z+, and a deadline d(t) E Z+, and a positive integer K.
> QUESTION: Is there a one-processor schedule σ for T such that the sum, taken over all t E T satisfying σ(t) + l(t) > d(t), of (σ(t) + l(t) - d(t))*w(t) is K or less?
> Reference: [Lawler, 1977a]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense. If all weights are equal, the problem can be solved in pseudo-polynomial time [Lawler, 1977a] and is open as to ordinary NP-completeness. If all lengths are equal (with weights arbitrary), it can be solved in polynomial time by bipartite matching. If precedence constraints are added, the problem is NP-complete even with equal lengths and equal weights [Lenstra and Rinnooy Kan, 1978a]. If release times are added instead, the problem is NP-complete in the strong sense for equal task weights (see SEQUENCING WITH RELEASE TIMES AND DEADLINES), but can be solved by bipartite matching for equal lengths and arbitrary weights [Graham, Lawler, Lenstra, and Rinnooy Kan, 1978].

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

- **[Lawler, 1977a]**: [`Lawler1977a`] Eugene L. Lawler (1977). "A pseudopolynomial algorithm for sequencing jobs to minimize total tardiness". *Annals of Discrete Mathematics* 1, pp. 331–342.
- **[Lenstra and Rinnooy Kan, 1978a]**: [`Lenstra1978a`] Jan K. Lenstra and A. H. G. Rinnooy Kan (1978). "Complexity of scheduling under precedence constraints". *Operations Research* 26, pp. 22–35.
- **[Graham, Lawler, Lenstra, and Rinnooy Kan, 1978]**: [`Graham1978`] R. L. Graham and E. L. Lawler and J. K. Lenstra and A. H. G. Rinnooy Kan (1978). "Optimization and approximation in deterministic sequencing and scheduling: a survey". *Annals of Discrete Mathematics*.