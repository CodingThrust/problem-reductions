---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Job-Shop Scheduling"
labels: rule
assignees: ''
---

**Source:** 3-Partition
**Target:** Job-Shop Scheduling
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.242

## GJ Source Entry

> [SS18] JOB-SHOP SCHEDULING
> INSTANCE: Number m E Z+ of processors, set J of jobs, each j E J consisting of an ordered collection of tasks t_k[j], 1 <= k <= n_j, for each such task t a length l(t) E Z_0+ and a processor p(t) E {1,2,...,m}, where p(t_k[j]) ≠ p(t_{k+1}[j]) for all j E J and 1 <= k < n_j, and a deadline D E Z+.
> QUESTION: Is there a job-shop schedule for J that meets the overall deadline, i.e., a collection of one-processor schedules σ_i mapping {t: p(t) = i} into Z_0+, 1 <= i <= m, such that σ_i(t) > σ_i(t') implies σ_i(t) >= σ_i(t') + l(t), such that σ(t_{k+1}[j]) >= σ(t_k[j]) + l(t_k[j]) (where the appropriate subscripts are to be assumed on σ) for all j E J and 1 <= k < n_j, and such that for all j E J σ(t_{n_j}[j]) + l(t_{n_j}[j]) <= D (again assuming the appropriate subscript on σ)?
> Reference: [Garey, Johnson, and Sethi, 1976]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense for m = 2. Can be solved in polynomial time if m = 2 and n_j <= 2 for all j E J [Jackson, 1956]. NP-complete (in the ordinary sense) if m = 2 and n_j <= 3 for all j E J, or if m = 3 and n_j <= 2 for all j E J [Gonzalez and Sahni, 1978a]. All the above results continue to hold if "preemptive" schedules are allowed [Gonzalez and Sahni, 1978a]. If in the nonpreemptive case all tasks have the same length, the problem is NP-complete for m = 3 and open for m = 2 [Lenstra and Rinnooy Kan, 1978b].

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

- **[Garey, Johnson, and Sethi, 1976]**: [`Garey1976f`] M. R. Garey and D. S. Johnson and R. Sethi (1976). "The complexity of flowshop and jobshop scheduling". *Mathematics of Operations Research* 1, pp. 117–129.
- **[Jackson, 1956]**: [`Jackson1956`] James R. Jackson (1956). "An extension of {Johnson}'s results on job lot scheduling". *Naval Research Logistics Quarterly* 3, pp. 201–203.
- **[Gonzalez and Sahni, 1978a]**: [`Gonzalez1978b`] T. Gonzalez and S. Sahni (1978). "Flowshop and jobshop schedules: complexity and approximation". *Operations Research* 26, pp. 36–52.
- **[Lenstra and Rinnooy Kan, 1978b]**: [`Lenstra1978b`] Jan K. Lenstra and A. H. G. Rinnooy Kan (1978). "Computational complexity of discrete optimization problems". *Annals of Discrete Mathematics*.