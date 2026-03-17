---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Open-Shop Scheduling"
labels: rule
assignees: ''
---

**Source:** Partition
**Target:** Open-Shop Scheduling
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.241

## GJ Source Entry

> [SS14] OPEN-SHOP SCHEDULING
> INSTANCE: Number m E Z+ of processors, set J of jobs, each job j E J consisting of m tasks t_1[j], t_2[j], ..., t_m[j] (with t_i[j] to be executed by processor i), a length l(t) E Z_0+ for each such task t, and an overall deadline D E Z+.
> QUESTION: Is there an open-shop schedule for J that meets the deadline, i.e., a collection of one-processor schedules σ_i: J → Z_0+, 1 <= i <= m, such that σ_i(j) > σ_i(k) implies σ_i(j) >= σ_i(k) + l(t_i[k]), such that for each j E J the intervals [σ_i(j), σ_i(j) + l(t_i[j])) are all disjoint, and such that σ_i(j) + l(t_i[j]) <= D for 1 <= i <= m, 1 <= j <= |J|?
> Reference: [Gonzalez and Sahni, 1976]. Transformation from PARTITION.
> Comment: Remains NP-complete if m = 3, but can be solved in polynomial time if m = 2. NP-complete in the strong sense for m arbitrary [Lenstra, 1977]. The general problem is solvable in polynomial time if "preemptive" schedules are allowed [Gonzalez and Sahni, 1976], even if two distinct release times are allowed [Cho and Sahni, 1978]. The m = 2 preemptive case can be solved in polynomial time even if arbitrary release times are allowed, and the general preemptive case with arbitrary release times and deadlines can be solved by linear programming [Cho and Sahni, 1978].

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

- **[Gonzalez and Sahni, 1976]**: [`Gonzalez1976`] T. Gonzalez and S. Sahni (1976). "Open shop scheduling to minimize finish time". *Journal of the Association for Computing Machinery* 23, pp. 665–679.
- **[Lenstra, 1977]**: [`Lenstra1977`] Jan K. Lenstra (1977). "".
- **[Cho and Sahni, 1978]**: [`Cho1978`] Y. Cho and S. Sahni (1978). "Preemptive scheduling of independent jobs with release and due times on open, flow, and job shops".