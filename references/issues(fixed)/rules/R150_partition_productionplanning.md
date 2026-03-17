---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Production Planning"
labels: rule
assignees: ''
---

**Source:** Partition
**Target:** Production Planning
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.4, p.243-244

## GJ Source Entry

> [SS21] PRODUCTION PLANNING
> INSTANCE: Number n E Z+ of periods, for each period i, 1 <= i <= n, a demand r_i E Z_0+, a production capacity c_i E Z_0+, a production set-up cost b_i E Z_0+, an incremental production cost coefficient p_i E Z_0+, and an inventory cost coefficient h_i E Z_0+, and an overall bound B E Z+.
> QUESTION: Do there exist production amounts x_i E Z_0+ and associated inventory levels I_i = sum_{j=1}^{i}(x_j - r_j), 1 <= i <= n, such that all x_i <= c_i, all I_i >= 0, and
>
> sum_{i=1}^{n}(p_i*x_i + h_i*I_i) + sum_{x_i > 0} b_i <= B ?
>
> Reference: [Lenstra, Rinnooy Kan, and Florian, 1978]. Transformation from PARTITION.
> Comment: Solvable in pseudo-polynomial time, but remains NP-complete even if all demands are equal, all set-up costs are equal, and all inventory costs are 0. If all capacities are equal, the problem can be solved in polynomial time [Florian and Klein, 1971]. The cited algorithms can be generalized to allow for arbitrary mono-tone non-decreasing concave cost functions, if these can be computed in polynomial time.

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

- **[Lenstra, Rinnooy Kan, and Florian, 1978]**: [`Lenstra1978c`] Jan K. Lenstra and A. H. G. Rinnooy Kan and M. Florian (1978). "Deterministic production planning: algorithms and complexity".
- **[Florian and Klein, 1971]**: [`Florian1971`] M. Florian and M. Klein (1971). "Deterministic production planning with concave costs and capacity constraints". *Management Science* 18, pp. 12–20.