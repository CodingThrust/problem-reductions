---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to KNAPSACK"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION'
canonical_target_name: 'KNAPSACK'
source_in_codebase: false
target_in_codebase: false
---

**Source:** PARTITION
**Target:** KNAPSACK
**Motivation:** PARTITION (asking whether a multiset of integers can be split into two equal-sum halves) reduces to KNAPSACK by treating each element as an item whose weight equals its value. Because the capacity is set to half the total sum, a balanced partition exists if and only if the optimal Knapsack value equals half the total sum. This reduction is from Garey & Johnson (1979), Appendix A6, where they prove KNAPSACK NP-complete via transformation from PARTITION.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability* (1979), Appendix A6, problem MP9 — "Transformation from PARTITION"

## GJ Source Entry

> [MP9] KNAPSACK
> INSTANCE: Finite set U, for each u E U a size s(u) E Z+ and a value v(u) E Z+, and positive integers B and K.
> QUESTION: Is there a subset U' ⊆ U such that Σ_{u E U'} s(u) ≤ B and such that Σ_{u E U'} v(u) ≥ K?
> Reference: [Karp, 1972]. Transformation from PARTITION.
> Comment: Remains NP-complete if s(u) = v(u) for all u E U (SUBSET SUM). Can be solved in pseudo-polynomial time by dynamic programming (e.g., see [Dantzig, 1957] or [Lawler, 1976a]).

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Let A = {a_1, ..., a_n} with s(a_i) ∈ Z⁺ be an arbitrary PARTITION instance, and let S = Σ_{i=1}^{n} s(a_i).

Note: The codebase's `Knapsack` is an **optimization problem** (maximize total value subject to weight capacity). The reduction encodes PARTITION feasibility into the optimal value of the constructed Knapsack instance.

1. **Items:** For each element a_i ∈ A, create a Knapsack item with weight w_i = s(a_i) and value v_i = s(a_i).
2. **Capacity:** Set capacity C = ⌊S / 2⌋. (If S is odd, the PARTITION instance has no solution; the optimal Knapsack value will be < S / 2, which signals infeasibility.)
3. **Correctness:** PARTITION is feasible (a subset summing to S/2 exists) if and only if the optimal Knapsack value equals S/2. Since w_i = v_i, maximizing value subject to capacity C = S/2 is equivalent to finding a subset with sum as close to S/2 as possible. Equality holds iff a balanced partition exists.
4. **Solution extraction:** Given the optimal Knapsack configuration, the selected items (x_i = 1) form one side of the partition A', and the unselected items form A \ A'. PARTITION is feasible iff the optimal value equals S/2.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements (`num_items` of source PARTITION instance)

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_items`               | `num_items` (= n)                |

**Derivation:** Each source element maps to exactly one knapsack item (n items total). The capacity is a data parameter (set to ⌊S / 2⌋), not a size field of the Knapsack model. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to Knapsack, solve with BruteForce (`find_best`), verify the optimal value equals S/2 and the selected items form a valid balanced partition.
- Correctness check: confirm that the optimal Knapsack value equals S/2 (not merely < C), exploiting the fact that value = weight forces exact capacity usage when a balanced partition exists.
- Edge cases: test with odd total sum (optimal value < S/2, no balanced partition) and with n = 1 (no balanced partition possible).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {3, 1, 1, 2, 2, 1} (n = 6 elements)
Total sum S = 3 + 1 + 1 + 2 + 2 + 1 = 10
A balanced partition exists: A' = {3, 2} (sum = 5) and A \ A' = {1, 1, 2, 1} (sum = 5).

**Constructed Knapsack instance:**

| Item i | w_i | v_i |
|--------|-----|-----|
| 0      | 3   | 3   |
| 1      | 1   | 1   |
| 2      | 1   | 1   |
| 3      | 2   | 2   |
| 4      | 2   | 2   |
| 5      | 1   | 1   |

Capacity C = 5.

**Optimal solution:**
Select items {0, 3} (weights 3, 2).
- Total weight: 3 + 2 = 5 ≤ C = 5 ✓
- Total value: 3 + 2 = 5 = S/2 ✓ → PARTITION is feasible

**Solution extraction:**
Partition: A' = {3, 2} (sum = 5) and A \ A' = {1, 1, 2, 1} (sum = 5). Balanced ✓

## References

- **[Garey & Johnson, 1979]**: [`garey1979`] Michael R. Garey and David S. Johnson (1979). *Computers and Intractability: A Guide to the Theory of NP-Completeness*. W.H. Freeman.
- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Dantzig, 1957]**: [`Dantzig1957`] G. B. Dantzig (1957). "Discrete-variable extremum problems". *Operations Research* 5, pp. 266–277.
- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.
