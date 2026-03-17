---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to CONTINUOUS MULTIPLE CHOICE KNAPSACK"
labels: rule
assignees: ''
---

**Source:** PARTITION
**Target:** CONTINUOUS MULTIPLE CHOICE KNAPSACK
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.247

## GJ Source Entry

> [MP11] CONTINUOUS MULTIPLE CHOICE KNAPSACK
> INSTANCE: Finite set U, for each u E U a size s(u) E Z+ and a value v(u) E Z+, a partition of U into disjoint sets U_1,U_2,...,U_m, and positive integers B and K.
> QUESTION: Is there a choice of a unique element u_i E U_i, 1 ≤ i ≤ m, and an assignment of rational numbers r_i, 0 ≤ r_i ≤ 1, to these elements, such that Σ_{i=1}^m r_i·s(u_i) ≤ B and Σ_{i=1}^m r_i·v(u_i) ≥ K?
> Reference: [Ibaraki, 1978]. Transformation from PARTITION.
> Comment: Solvable in pseudo-polynomial time, but remains NP-complete even if |U_i| ≤ 2, 1 ≤ i ≤ m. Solvable in polynomial time by "greedy" algorithms if |U_i| = 1, 1 ≤ i ≤ m, or if we only require that the r_i ≥ 0 but place no upper bound on them. [Ibaraki, Hasegawa, Teranaka, and Iwase, 1978].

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

- **[Ibaraki, 1978]**: [`Ibaraki1978a`] Toshihide Ibaraki (1978). "Approximate algorithms for the multiple-choice continuous knapsack problem".
- **[Ibaraki, Hasegawa, Teranaka, and Iwase, 1978]**: [`Ibaraki1978b`] Toshihide Ibaraki and T. Hasegawa and K. Teranaka and J. Iwase (1978). "The multiple-choice knapsack problem". *Journal of the Operations Research Society of Japan* 21, pp. 59–94.