---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Sequencing to Minimize Weighted Tardiness"
labels: rule
assignees: ''
canonical_source_name: '3-PARTITION'
canonical_target_name: 'SEQUENCING TO MINIMIZE WEIGHTED TARDINESS'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3-Partition
**Target:** Sequencing to Minimize Weighted Tardiness
**Motivation:** 3-PARTITION is a strongly NP-complete number-partitioning problem; SEQUENCING TO MINIMIZE WEIGHTED TARDINESS asks whether tasks can be scheduled on a single processor so that total weighted tardiness stays within a bound K. By encoding the 3-PARTITION bins as deadline-separated time slots and setting task weights to enforce exact packing, the reduction establishes that weighted tardiness minimization is NP-complete in the strong sense — ruling out any pseudo-polynomial time algorithm unless P = NP.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.237-238

## GJ Source Entry

> [SS5] SEQUENCING TO MINIMIZE WEIGHTED TARDINESS
> INSTANCE: Set T of tasks, for each task t E T a length l(t) E Z+, a weight w(t) E Z+, and a deadline d(t) E Z+, and a positive integer K.
> QUESTION: Is there a one-processor schedule σ for T such that the sum, taken over all t E T satisfying σ(t) + l(t) > d(t), of (σ(t) + l(t) - d(t))*w(t) is K or less?
> Reference: [Lawler, 1977a]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense. If all weights are equal, the problem can be solved in pseudo-polynomial time [Lawler, 1977a] and is open as to ordinary NP-completeness. If all lengths are equal (with weights arbitrary), it can be solved in polynomial time by bipartite matching. If precedence constraints are added, the problem is NP-complete even with equal lengths and equal weights [Lenstra and Rinnooy Kan, 1978a]. If release times are added instead, the problem is NP-complete in the strong sense for equal task weights (see SEQUENCING WITH RELEASE TIMES AND DEADLINES), but can be solved by bipartite matching for equal lengths and arbitrary weights [Graham, Lawler, Lenstra, and Rinnooy Kan, 1978].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Given a 3-PARTITION instance: a set A = {a_1, ..., a_{3m}} of 3m positive integers with bound B such that B/4 < a_i < B/2 for all i and Σa_i = mB, construct a SEQUENCING TO MINIMIZE WEIGHTED TARDINESS instance as follows.

1. **Partition tasks:** For each element a_i ∈ A, create a task t_i with length l(t_i) = a_i and weight w(t_i) = W (a sufficiently large weight, e.g., W = (mB)^2 + 1). Set deadline d(t_i) = jB for the j-th group, assigned so that exactly three tasks must fit in each time slot of length B.

2. **Separator tasks:** Create m − 1 separator (or "filler") tasks s_1, ..., s_{m−1}, each with length l(s_j) = 0 (or a negligible length), very large weight to force them into specific positions, and deadlines that partition the time horizon into m consecutive slots of length B each: [0, B], [B, 2B], ..., [(m−1)B, mB].

3. **Deadlines and weights enforce grouping:** Each partition task t_i has its deadline set so that it must be completed within one of the m time slots. The large weight W ensures that any tardiness incurs a cost exceeding the budget K, effectively forcing zero tardiness for all tasks. Set K = 0 (or a value that allows no tardiness at all).

4. **Correctness:** Because B/4 < a_i < B/2, exactly three elements must sum to B in each slot. A 3-partition of A exists if and only if all tasks can be scheduled with zero total weighted tardiness (i.e., meeting all deadlines).

5. **Solution extraction:** Given a feasible schedule σ with total weighted tardiness ≤ K = 0, the tasks scheduled in the j-th time slot [(j−1)B, jB] correspond to a triple of elements summing to exactly B.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- m = number of triples in the 3-PARTITION instance (source has 3m elements)
- B = bin capacity

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_tasks`                | `3 * m + (m - 1)` = `4 * m - 1`  |
| `max_length`               | B (maximum task length)           |
| `max_deadline`             | `m * B`                           |
| `max_weight`               | O((mB)^2)                         |
| `bound_K`                  | 0                                 |

**Derivation:** The 3m element tasks come directly from the 3-PARTITION elements. Up to m − 1 separator tasks enforce the time-slot boundaries. All deadlines are multiples of B up to mB. Construction is O(m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3-PARTITION instance with 3m = 6 elements (m = 2), reduce to SEQUENCING TO MINIMIZE WEIGHTED TARDINESS, enumerate all permutations of tasks on the single processor, verify that a zero-tardiness schedule exists iff the 3-PARTITION instance is feasible.
- Check that the constructed instance has 4m − 1 tasks (approximately), all deadlines are multiples of B, and K = 0.
- Edge cases: test with an infeasible 3-PARTITION instance (expect no zero-tardiness schedule), and with m = 1 (three elements summing to B, trivial case).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3-PARTITION):**
A = {5, 6, 7, 8, 9, 10}, B = 15, m = 2
Check: B/4 = 3.75 < a_i < B/2 = 7.5 — note that 8, 9, 10 violate the upper bound, so adjust:
A = {4, 5, 6, 4, 5, 6}, B = 10, m = 2
Check: B/4 = 2.5 < a_i < B/2 = 5 — 5 and 6 violate. Adjust again:
A = {3, 3, 4, 3, 3, 4}, B = 10, m = 2
Check: 2.5 < {3,3,4,3,3,4} < 5 ✓, Σ = 20 = 2 × 10 ✓

Feasible 3-partition: {3, 3, 4} and {3, 3, 4}, each summing to 10.

**Constructed SEQUENCING TO MINIMIZE WEIGHTED TARDINESS instance:**

| Task | Length | Weight | Deadline |
|------|--------|--------|----------|
| t_1  | 3      | 1000   | 10       |
| t_2  | 3      | 1000   | 10       |
| t_3  | 4      | 1000   | 10       |
| t_4  | 3      | 1000   | 20       |
| t_5  | 3      | 1000   | 20       |
| t_6  | 4      | 1000   | 20       |
| s_1  | 0      | 10000  | 10       |

K = 0 (total weighted tardiness bound).

**Solution:**
Schedule in slot [0, 10]: t_1 (0–3), t_2 (3–6), t_3 (6–10), s_1 at time 10.
Schedule in slot [10, 20]: t_4 (10–13), t_5 (13–16), t_6 (16–20).
All tasks meet their deadlines. Total weighted tardiness = 0 ≤ K ✓

**Solution extraction:**
Triple 1: {a_1, a_2, a_3} = {3, 3, 4}, sum = 10 = B ✓
Triple 2: {a_4, a_5, a_6} = {3, 3, 4}, sum = 10 = B ✓


## References

- **[Lawler, 1977a]**: [`Lawler1977a`] Eugene L. Lawler (1977). "A pseudopolynomial algorithm for sequencing jobs to minimize total tardiness". *Annals of Discrete Mathematics* 1, pp. 331–342.
- **[Lenstra and Rinnooy Kan, 1978a]**: [`Lenstra1978a`] Jan K. Lenstra and A. H. G. Rinnooy Kan (1978). "Complexity of scheduling under precedence constraints". *Operations Research* 26, pp. 22–35.
- **[Graham, Lawler, Lenstra, and Rinnooy Kan, 1978]**: [`Graham1978`] R. L. Graham and E. L. Lawler and J. K. Lenstra and A. H. G. Rinnooy Kan (1978). "Optimization and approximation in deterministic sequencing and scheduling: a survey". *Annals of Discrete Mathematics*.
