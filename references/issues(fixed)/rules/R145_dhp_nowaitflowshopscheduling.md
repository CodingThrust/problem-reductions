---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Directed Hamiltonian Path to No-Wait Flow-Shop Scheduling"
labels: rule
assignees: ''
---

**Source:** Directed Hamiltonian Path
**Target:** No-Wait Flow-Shop Scheduling
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.3, p.241-242

## GJ Source Entry

> [SS16] NO-WAIT FLOW-SHOP SCHEDULING
> INSTANCE: (Same as for FLOW-SHOP SCHEDULING).
> QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and has the property that, for each j E J and 1 <= i < m, σ_{i+1}(j) = σ_i(j) + l(t_i[j])?
> Reference: [Lenstra, Rinnooy Kan, and Brucker, 1977]. Transformation from DIRECTED HAMILTONIAN PATH.
> Comment: NP-complete in the strong sense for any fixed m >= 4 [Papadimitriou and Kanellakis, 1978]. Solvable in polynomial time for m = 2 [Gilmore and Gomory, 1964]. (However, NP-complete in the strong sense for m = 2 if jobs with no tasks on the first processor are allowed [Sahni and Cho, 1977b].) Open for fixed m = 3. If the goal is to meet a bound K on the sum, over all j E J, of σ_m(j) + l(t_m[j]), then the problem is NP-complete in the strong sense for m arbitrary [Lenstra, Rinnooy Kan, and Brucker, 1977] and open for fixed m >= 2. The analogous "no-wait" versions of OPEN-SHOP SCHEDULING and JOB-SHOP SCHEDULING are NP-complete in the strong sense for m = 2 [Sahni and Cho, 1977b].

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
- **[Papadimitriou and Kanellakis, 1978]**: [`Papadimitriou1978e`] Christos H. Papadimitriou and P. C. Kanellakis (1978). "Flowshop scheduling with limited temporary storage".
- **[Gilmore and Gomory, 1964]**: [`Gilmore1964`] P. C. Gilmore and R. E. Gomory (1964). "Sequencing a one state-variable machine: a solvable case of the traveling salesman problem". *Operations Research* 12, pp. 655–679.
- **[Sahni and Cho, 1977b]**: [`Sahni1977b`] S. Sahni and Y. Cho (1977). "Complexity of scheduling shops with no wait in process". Computer Science Dept., University of Minnesota.