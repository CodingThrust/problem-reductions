---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Production Planning"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION'
canonical_target_name: 'PRODUCTION PLANNING'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** Partition
**Target:** Production Planning
**Motivation:** PARTITION asks whether a multiset of integers can be split into two equal-sum halves; PRODUCTION PLANNING asks whether production amounts can be scheduled across periods to meet demands while respecting capacities and staying within a total cost bound (including set-up, production, and inventory costs). Lenstra, Rinnooy Kan, and Florian (1978) showed NP-completeness of production planning via reduction from PARTITION, establishing that even simplified lot-sizing problems with set-up costs are computationally intractable. This is a foundational hardness result for operations research and supply chain optimization.
<!-- ⚠️ Unverified: AI-generated motivation -->
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

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a PARTITION instance with multiset A = {a_1, ..., a_n} where B_total = sum a_i, construct a PRODUCTION PLANNING instance as follows:

1. **Periods:** Set 2 periods (n_periods = 2).
2. **Demands:** r_1 = B_total / 2, r_2 = B_total / 2. (If B_total is odd, output a trivially infeasible instance.)
3. **Capacities:** For each element a_i, create a "production source" that can contribute up to a_i units. More precisely, use n production periods (one per element), with c_i = a_i, and arrange demands so that the total production in the first group of periods must equal B_total/2.

Alternatively (using the simplified form noted in the GJ comment: equal demands, equal set-up costs, zero inventory costs):

1. **Periods:** n periods (one per element of A).
2. **Demands:** r_i = a_i for each period i. Each period demands exactly its element's value.
3. **Capacities:** c_i = B_total (large enough that capacity is not binding in any single period).
4. **Costs:** Set all production costs p_i = 0, all inventory costs h_i = 0, and set-up costs b_i chosen so that the total cost depends on which periods have nonzero production. The bound B is chosen so that a feasible plan exists iff production can be consolidated in a way that corresponds to a balanced partition.
5. **Correctness:** A balanced partition A' with sum = B_total/2 corresponds to a production plan where items in A' are produced in one batch and items in A\A' in another, meeting all demands with inventory never going negative, and total cost <= B. Conversely, any feasible plan within cost bound B implies a balanced partition.
6. **Solution extraction:** From a feasible production plan, identify which periods have positive production; the corresponding elements form one half of the partition.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements in the PARTITION instance

| Target metric (code name)   | Polynomial (using symbols above) |
|-----------------------------|----------------------------------|
| `num_periods`               | n                                |
| `max_demand`                | max(a_i)                         |
| `max_capacity`              | B_total (= sum of all a_i)      |

**Derivation:** Each element maps to one production period. Demands and capacities are derived directly from element values. The cost bound B is polynomial in the input size. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to PRODUCTION PLANNING, solve by brute-force enumeration of all feasible production vectors (x_1, ..., x_n) with x_i <= c_i and all I_i >= 0, check if any achieves total cost <= B, verify the answer matches PARTITION solvability.
- Check that the instance has n periods with demands, capacities, and costs matching the construction.
- Edge cases: odd total sum (infeasible partition, expect infeasible production plan), all elements equal (trivial partition), single element (n=1, infeasible unless a_1 = 0).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {3, 5, 2, 4, 6} (n = 5 elements)
B_total = 3 + 5 + 2 + 4 + 6 = 20
Target: split into two subsets each summing to 10.
Balanced partition: A' = {4, 6} (sum = 10), A\A' = {3, 5, 2} (sum = 10). ✓

**Constructed PRODUCTION PLANNING instance:**
n = 5 periods, with equal set-up costs and zero inventory costs (per GJ simplified form).

| Period i | Demand r_i | Capacity c_i | Set-up cost b_i | Production cost p_i | Inventory cost h_i |
|----------|-----------|-------------|----------------|--------------------|--------------------|
| 1        | 3         | 20          | 1              | 0                  | 0                  |
| 2        | 5         | 20          | 1              | 0                  | 0                  |
| 3        | 2         | 20          | 1              | 0                  | 0                  |
| 4        | 4         | 20          | 1              | 0                  | 0                  |
| 5        | 6         | 20          | 1              | 0                  | 0                  |

Cost bound B = 2 (at most 2 set-up costs, since p_i = h_i = 0, total cost = number of periods with x_i > 0).

**Solution:**
Produce in period 1: x_1 = 10 (covers demands for periods 1-3: r_1+r_2+r_3 = 3+5+2 = 10)
Produce in period 4: x_4 = 10 (covers demands for periods 4-5: r_4+r_5 = 4+6 = 10)
x_2 = x_3 = x_5 = 0.

Inventory levels: I_1 = 10-3 = 7, I_2 = 7-5 = 2, I_3 = 2-2 = 0, I_4 = 0+10-4 = 6, I_5 = 6-6 = 0. All >= 0 ✓
Total cost = p*x + h*I + set-ups = 0 + 0 + 2 = 2 <= B = 2 ✓

**Solution extraction:**
Periods with production: {1, 4}. The demands met from period 1's batch correspond to elements {3, 5, 2} (sum = 10), and from period 4's batch to {4, 6} (sum = 10). Balanced partition ✓


## References

- **[Lenstra, Rinnooy Kan, and Florian, 1978]**: [`Lenstra1978c`] Jan K. Lenstra and A. H. G. Rinnooy Kan and M. Florian (1978). "Deterministic production planning: algorithms and complexity".
- **[Florian and Klein, 1971]**: [`Florian1971`] M. Florian and M. Klein (1971). "Deterministic production planning with concave costs and capacity constraints". *Management Science* 18, pp. 12–20.
