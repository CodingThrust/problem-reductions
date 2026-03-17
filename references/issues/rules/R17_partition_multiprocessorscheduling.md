---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Multiprocessor Scheduling"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION'
canonical_target_name: 'MULTIPROCESSOR SCHEDULING'
source_in_codebase: false
target_in_codebase: false
---

**Source:** Partition
**Target:** Multiprocessor Scheduling
**Motivation:** PARTITION asks whether a multiset of integers can be split into two equal-sum halves; MULTIPROCESSOR SCHEDULING asks whether n tasks can be divided across m processors all finishing by deadline D. Setting m = 2 and D = half the total task length turns the scheduling problem into exactly PARTITION, establishing that scheduling is NP-complete even with only two processors. This is the canonical restriction-based reduction cited in Garey & Johnson, Section 3.2.1.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1 item (7), p.65

## Reduction Algorithm

> (7) MULTIPROCESSOR SCHEDULING
> INSTANCE: A finite set A of "tasks," a "length" l(a) ∈ Z+ for each a ∈ A, a number m ∈ Z+ of "processors," and a "deadline" D ∈ Z+.
> QUESTION: Is there a partition A = A_1 ∪ A_2 ∪ ⋯ ∪ A_m of A into m disjoint sets such that
>
> max { ∑_{a ∈ A_i} l(a) : 1 ≤ i ≤ m } ≤ D ?
>
> Proof: Restrict to PARTITION by allowing only instances in which m = 2 and D = ½∑_{a ∈ A} l(a).

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Let A = {a_1, ..., a_n} with s(a_i) ∈ Z⁺ be an arbitrary PARTITION instance, and let B_total = Σ_{i=1}^{n} s(a_i).

1. **Tasks:** For each element a_i ∈ A, create a scheduling task t_i with length l(t_i) = s(a_i). The task set is T = {t_1, ..., t_n}.
2. **Processors and deadline:** Set m = 2 processors and deadline D = B_total / 2. (If B_total is odd, there is no balanced partition; output any trivially infeasible instance, e.g., D = 0.)
3. **Correctness:** A balanced partition A' ∪ (A \ A') with each part summing to B_total / 2 exists if and only if assigning {t_i : a_i ∈ A'} to processor 1 and the rest to processor 2 satisfies max load ≤ D.
4. **Solution extraction:** Given a valid schedule σ (assignment of tasks to processors), the partition is A' = {a_i : t_i assigned to processor 1} and A \ A' = {a_i : t_i assigned to processor 2}.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = number of elements in PARTITION instance (`num_tasks` of source)
- S = Σ s(a_i) = total sum of element sizes

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_tasks`               | `num_tasks` (= n)                |
| `num_processors`          | 2                                |
| `deadline`                | `total_sum / 2`                  |

**Derivation:** Each element of A maps directly to one task of the same length. Only two processors are needed (a constant). The deadline is fixed at half the total task length. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to MULTIPROCESSOR SCHEDULING with m=2, D=S/2, solve with BruteForce by trying all 2^n assignments, verify the max-load assignment corresponds to a valid balanced partition.
- Check that the constructed instance has exactly n tasks, m = 2 processors, and D = S/2.
- Edge cases: test with odd total sum (expect infeasible: D would not be an integer), n = 1 (infeasible, single task cannot meet deadline D = s(a_1)/2 < l(t_1)).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {4, 5, 3, 2, 6} (n = 5 elements)
Total sum B_total = 4 + 5 + 3 + 2 + 6 = 20
A balanced partition exists: A' = {4, 6} (sum = 10) and A \ A' = {5, 3, 2} (sum = 10).

**Constructed MULTIPROCESSOR SCHEDULING instance:**

| Task t_i | Length l(t_i) |
|----------|--------------|
| t_1      | 4            |
| t_2      | 5            |
| t_3      | 3            |
| t_4      | 2            |
| t_5      | 6            |

Number of processors m = 2, Deadline D = 10.

**Solution:**
Assign {t_1, t_5} (lengths 4, 6) to processor 1: total load = 10 ≤ D ✓
Assign {t_2, t_3, t_4} (lengths 5, 3, 2) to processor 2: total load = 10 ≤ D ✓
Max load = max(10, 10) = 10 ≤ D = 10 ✓

**Solution extraction:**
Partition: A' = {a_1, a_5} = {4, 6} (sum = 10) and A \ A' = {a_2, a_3, a_4} = {5, 3, 2} (sum = 10). Balanced ✓
