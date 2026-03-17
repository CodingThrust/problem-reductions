---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Flow-Shop Scheduling"
labels: rule
assignees: ''
---

**Source:** 3-Partition
**Target:** Flow-Shop Scheduling
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.241

## GJ Source Entry

> [SS15] FLOW-SHOP SCHEDULING
> INSTANCE: Number m E Z+ of processors, set J of jobs, each job j E J consisting of m tasks t_1[j], t_2[j], ..., t_m[j], a length l(t) E Z_0+ for each such task t, and an overall deadline D E Z+.
> QUESTION: Is there a flow-shop schedule for J that meets the overall deadline, where such a schedule is identical to an open-shop schedule with the additional constraint that, for each j E J and 1 <= i < m, σ_{i+1}(j) >= σ_i(j) + l(t_i[j])?
> Reference: [Garey, Johnson, and Sethi, 1976]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense for m = 3. Solvable in polynomial time for m = 2 [Johnson, 1954]. The same results hold if "preemptive" schedules are allowed [Gonzalez and Sahni, 1978a], although if release times are added in this case, the problem is NP-complete in the strong sense, even for m = 2 [Cho and Sahni, 1978]. If the goal is to meet a bound K on the sum, over all j E J, of σ_m(j) + l(t_m[j]), then the non-preemptive problem is NP-complete in the strong sense even if m = 2 [Garey, Johnson, and Sethi, 1976].

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
- **[Johnson, 1954]**: [`Johnson1954`] Selmer M. Johnson (1954). "Optimal two- and three-stage production schedules with setup times included". *Naval Research Logistics Quarterly* 1, pp. 61–68.
- **[Gonzalez and Sahni, 1978a]**: [`Gonzalez1978b`] T. Gonzalez and S. Sahni (1978). "Flowshop and jobshop schedules: complexity and approximation". *Operations Research* 26, pp. 36–52.
- **[Cho and Sahni, 1978]**: [`Cho1978`] Y. Cho and S. Sahni (1978). "Preemptive scheduling of independent jobs with release and due times on open, flow, and job shops".