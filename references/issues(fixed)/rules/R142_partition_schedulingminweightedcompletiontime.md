---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Scheduling to Minimize Weighted Completion Time"
labels: rule
assignees: ''
---

**Source:** Partition
**Target:** Scheduling to Minimize Weighted Completion Time
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.240-241

## GJ Source Entry

> [SS13] SCHEDULING TO MINIMIZE WEIGHTED COMPLETION TIME
> INSTANCE: Set T of tasks, number m E Z+ of processors, for each task t E T a length l(t) E Z+ and a weight w(t) E Z+, and a positive integer K.
> QUESTION: Is there an m-processor schedule σ for T such that the sum, over all t E T, of (σ(t) + l(t))*w(t) is no more than K?
> Reference: [Lenstra, Rinnooy Kan, and Brucker, 1977]. Transformation from PARTITION.
> Comment: Remains NP-complete for m = 2, and is NP-complete in the strong sense for m arbitrary [Lageweg and Lenstra, 1977]. The problem is solvable in pseudo-polynomial time for fixed m. These results continue to hold if "preemptive" schedules are allowed [McNaughton, 1959]. Can be solved in polynomial time if all lengths are equal (by matching techniques). If instead all weights are equal, it can be solved in polynomial time even for "different speed" processors [Conway, Maxwell, and Miller, 1967] and for "unrelated" processors [Horn, 1973], [Bruno, Coffman, and Sethi, 1974]. The "preemptive" case for different speed processors also can be solved in polynomial time [Gonzalez, 1977]. If precedence constraints are allowed, the original problem is NP-complete in the strong sense even if all weights are equal, m = 2, and the partial order is either an "in-tree" or an "out-tree" [Sethi, 1977a]. If resources are allowed, the same subcases men-tioned under RESOURCE CONSTRAINED SCHEDULING are NP-complete, even for equal weights [Blazewicz, 1977a].

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

- **[Lenstra, Rinnooy Kan, and Brucker, 1977]**: [`Lenstra1977a`] Jan K. Lenstra and A. H. G. Rinnooy Kan and Peter Brucker (1977). "Complexity of machine scheduling problems". *Annals of Discrete Mathematics* 1, pp. 343–362.
- **[Lageweg and Lenstra, 1977]**: [`Lageweg1977`] B. J. Lageweg and Jan K. Lenstra (1977). "".
- **[McNaughton, 1959]**: [`McNaughton1959`] Robert McNaughton (1959). "Scheduling with deadlines and loss functions". *Management Science* 6, pp. 1–12.
- **[Conway, Maxwell, and Miller, 1967]**: [`Conway1967`] R. W. Conway and W. L. Maxwell and L. W. Miller (1967). "Theory of Scheduling". Addison-Wesley, Reading, MA.
- **[Horn, 1973]**: [`Horn1973`] William A. Horn (1973). "Minimizing average flow time with parallel machines". *Operations Research* 21, pp. 846–847.
- **[Bruno, Coffman, and Sethi, 1974]**: [`Bruno1974`] J. Bruno and E. G. Coffman, Jr and R. Sethi (1974). "Scheduling independent tasks to reduce mean finishing time". *Communications of the ACM* 17, pp. 382–387.
- **[Gonzalez, 1977]**: [`Gonzalez1977`] T. Gonzalez (1977). "Optimal mean finish time preemptive schedules". Computer Science Dept., Pennsylvania State University.
- **[Sethi, 1977a]**: [`Sethi1977a`] R. Sethi (1977). "On the complexity of mean flow time scheduling". *Mathematics of Operations Research* 2, pp. 320–330.
- **[Blazewicz, 1977a]**: [`Blazewicz1977a`] J. Blazewicz (1977). "Mean flow time scheduling under resource constraints". Technical University of Poznan.