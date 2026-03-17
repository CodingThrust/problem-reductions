---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Optimal Linear Arrangement to Sequencing to Minimize Weighted Completion Time"
labels: rule
assignees: ''
---

**Source:** Optimal Linear Arrangement
**Target:** Sequencing to Minimize Weighted Completion Time
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.237

## GJ Source Entry

> [SS4] SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME
> INSTANCE: Set T of tasks, partial order < on T, for each task t E T a length l(t) E Z+ and a weight w(t) E Z+, and a positive integer K.
> QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints and for which the sum, over all t E T, of (σ(t) + l(t))*w(t) is K or less?
> Reference: [Lawler, 1978]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
> Comment: NP-complete in the strong sense and remains so even if all task lengths are 1 or all task weights are 1. Can be solved in polynomial time for < a "forest" [Horn, 1972], [Adolphson and Hu, 1973], [Garey, 1973], [Sidney, 1975] or if < is "series-parallel" or "generalized series-parallel" [Knuth, 1973], [Lawler, 1978], [Adolphson, 1977], [Monma and Sidney, 1977]. If the partial order < is replaced by individual task deadlines, the resulting problem is NP-complete in the strong sense [Lenstra, 1977], but can be solved in polynomial time if all task weights are equal [Smith, 1956]. If there are individual task release times instead of deadlines, the resulting problem is NP-complete in the strong sense, even if all task weights are 1 [Lenstra, Rinnooy Kan, and Brucker, 1977]. The "preemptive" version of this latter problem is NP-complete in the strong sense [Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1978], but is solvable in polynomial time if all weights are equal [Graham, Lawler, Lenstra, and Rinnooy Kan, 1978].

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

- **[Lawler, 1978]**: [`Lawler1978a`] Eugene L. Lawler (1978). "Sequencing jobs to minimize total weighted completion time subject to precedence constraints". *Annals of Discrete Mathematics* 2, pp. 75–90.
- **[Horn, 1972]**: [`Horn1972`] William A. Horn (1972). "Single-machine job sequencing with treelike precedence ordering and linear delay penalties". *SIAM Journal on Applied Mathematics* 23, pp. 189–202.
- **[Adolphson and Hu, 1973]**: [`Adolphson1973`] D. Adolphson and T. C. Hu (1973). "Optimal linear ordering". *SIAM Journal on Applied Mathematics* 25, pp. 403–423.
- **[Garey, 1973]**: [`Garey1973`] M. R. Garey (1973). "Optimal task sequencing with precedence constraints". *Discrete Mathematics* 4, pp. 37–56.
- **[Sidney, 1975]**: [`Sidney1975`] Jeffrey B. Sidney (1975). "Decomposition algorithms for single-machine sequencing with precedence relations and deferral costs". *Operations Research* 23, pp. 283–298.
- **[Knuth, 1973]**: [`Knuth1973`] Donald E. Knuth (1973). "Private communication".
- **[Adolphson, 1977]**: [`Adolphson1977`] D. Adolphson (1977). "Single machine job sequencing with precedence constraints". *SIAM Journal on Computing* 6, pp. 40–54.
- **[Monma and Sidney, 1977]**: [`Monma1977`] Clyde L. Monma and J. B. Sidney (1977). "A general algorithm for optimal job sequencing with series-parallel precedence constraints". School of Operations Research, Cornell University.
- **[Lenstra, 1977]**: [`Lenstra1977`] Jan K. Lenstra (1977). "".
- **[Smith, 1956]**: [`Smith1956`] Wayne E. Smith (1956). "Various optimizers for single-state production". *Naval Research Logistics Quarterly* 3, pp. 59–66.
- **[Lenstra, Rinnooy Kan, and Brucker, 1977]**: [`Lenstra1977a`] Jan K. Lenstra and A. H. G. Rinnooy Kan and Peter Brucker (1977). "Complexity of machine scheduling problems". *Annals of Discrete Mathematics* 1, pp. 343–362.
- **[Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1978]**: [`Labetoulle and Lawler and Lenstra and Rinnooy Kan1978`] Jacques Labetoulle and Eugene L. Lawler and Jan K. Lenstra and A. H. G. Rinnooy Kan (1978). "Preemptive scheduling of uniform machines".
- **[Graham, Lawler, Lenstra, and Rinnooy Kan, 1978]**: [`Graham1978`] R. L. Graham and E. L. Lawler and J. K. Lenstra and A. H. G. Rinnooy Kan (1978). "Optimization and approximation in deterministic sequencing and scheduling: a survey". *Annals of Discrete Mathematics*.