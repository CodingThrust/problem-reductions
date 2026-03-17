---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Resource Constrained Scheduling"
labels: rule
assignees: ''
canonical_source_name: '3-PARTITION'
canonical_target_name: 'RESOURCE CONSTRAINED SCHEDULING'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3-Partition
**Target:** Resource Constrained Scheduling
**Motivation:** 3-PARTITION asks whether a multiset of 3m integers can be partitioned into m triples each summing to a target B; RESOURCE CONSTRAINED SCHEDULING asks whether unit-length tasks with resource demands can be scheduled on m processors within a deadline D while respecting resource bounds. The reduction sets m = 3 processors with a single resource (r = 1) and encodes each integer a_i as a task whose resource requirement equals a_i, forcing three tasks per time slot whose requirements sum to at most B, thereby directly encoding the 3-partition constraint. This proves RESOURCE CONSTRAINED SCHEDULING is NP-complete in the strong sense even for r = 1 and m = 3.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.2, p.239

## GJ Source Entry

> [SS10] RESOURCE CONSTRAINED SCHEDULING
> INSTANCE: Set T of tasks, each having length l(t) = 1, number m E Z+ of processors, number r E Z+ of resources, resource bounds B_i, 1 <= i <= r, resource requirement R_i(t), 0 <= R_i(t) <= B_i, for each task t and resource i, and an overall deadline D E Z+.
> QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the resource constraints, i.e., such that for all u >= 0, if S(u) is the set of all t E T for which σ(t) <= u < σ(t) + l(t), then for each resource i the sum of R_i(t) over all t E S(u) is at most B_i?
> Reference: [Garey and Johnson, 1975]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense, even if r = 1 and m = 3. Can be solved in polynomial time by matching for m = 2 and r arbitrary. If a partial order < is added, the problem becomes NP-complete in the strong sense for r = 1, m = 2, and < a "forest." If each resource requirement is restricted to be either 0 or B_i, the problem is NP-complete for m = 2, r = 1, and < arbitrary [Ullman, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Let A = {a_1, a_2, ..., a_{3m}} be a 3-PARTITION instance with target sum B, where B/4 < a_i < B/2 for all i and the total sum is mB.

1. **Tasks:** For each element a_i, create a unit-length task t_i. The task set is T = {t_1, t_2, ..., t_{3m}}.
2. **Processors:** Set the number of processors to m_proc = 3 (i.e., at most 3 tasks execute in any time slot).
3. **Resources:** Use a single resource r = 1, with resource bound B_1 = B.
4. **Resource requirements:** For each task t_i, set R_1(t_i) = a_i.
5. **Deadline:** Set D = m (there are m time slots, each accommodating up to 3 tasks).
6. **Correctness (forward):** If A can be partitioned into m triples each summing to B, then schedule the three tasks of the j-th triple in time slot j (j = 0, 1, ..., m-1). Each slot has at most 3 tasks (respects processor count) and total resource usage = B (respects resource bound).
7. **Correctness (backward):** If a feasible schedule exists, each time slot has at most 3 tasks. Since B/4 < a_i < B/2, each task uses more than B/4 of the resource, so no slot can hold 4 or more tasks (that would exceed B). Thus each slot holds exactly 3 tasks, and their resource requirements sum to at most B. Since there are m slots and 3m tasks, the total resource usage across all slots equals mB = the total sum. Hence each slot sums to exactly B, giving a valid 3-partition.
8. **Solution extraction:** Given a valid schedule sigma, the partition is: the j-th triple consists of {a_i : sigma(t_i) = j} for j = 0, 1, ..., m-1.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = 3m = number of elements in the 3-PARTITION instance (`num_elements` of source)
- B = target sum per triple
- m = n / 3 = number of triples

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| `num_tasks`                | `num_elements` (= 3m)           |
| `num_processors`           | 3                                |
| `num_resources`            | 1                                |
| `resource_bound`           | `target_sum` (= B)              |
| `deadline`                 | `num_elements / 3` (= m)        |

**Derivation:** Each element maps 1:1 to a task. The number of processors, resources, and resource bound are constants derived from the 3-PARTITION parameters. The deadline equals the number of triples. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3-PARTITION instance, reduce to RESOURCE CONSTRAINED SCHEDULING with m_proc = 3, r = 1, B_1 = B, D = m. Solve by brute-force enumeration of all assignments of 3m tasks to m time slots (with at most 3 tasks per slot). Verify the schedule corresponds to a valid 3-partition.
- Check that the constructed instance has exactly 3m tasks, 3 processors, 1 resource with bound B, and deadline m.
- Edge cases: test with a set where no valid 3-partition exists (expect infeasible); test with the smallest non-trivial case m = 1 (3 elements summing to B).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3-PARTITION):**
A = {5, 6, 9, 7, 8, 10, 11, 4, 10} (n = 9 elements, m = 3 triples)
B = (5 + 6 + 9 + 7 + 8 + 10 + 11 + 4 + 10) / 3 = 80 / 3 ... let us pick a cleaner instance.

A = {1, 1, 1, 1, 1, 1} with B = 2 would not satisfy B/4 < a_i < B/2 (need 0.5 < 1 < 1, fails upper bound).

Let us use: A = {5, 6, 9, 8, 7, 10, 11, 4, 12, 3, 13, 2} (n = 12, m = 4, total = 90, B = 90/4 ... not integer).

Cleaner: A = {5, 7, 8, 6, 9, 10, 4, 11, 12, 3, 13, 7} — total = 95, not divisible by 4.

Standard example: m = 2, B = 20, A = {6, 7, 7, 8, 9, 11} (total = 48 ... no, need total = 2*20 = 40).
A = {6, 7, 7, 5, 8, 7} total = 40, B = 20. Check B/4=5 < a_i < B/2=10: 6,7,7,5,8,7 — a_4=5 is not > 5, fails.

A = {6, 7, 7, 6, 8, 6} total = 40, B = 20. Check: all in (5, 10), yes.
Partition: {6, 7, 7} = 20 and {6, 8, 6} = 20. Valid.

**Source instance (3-PARTITION):**
A = {6, 7, 7, 6, 8, 6} (n = 6 elements, m = 2 triples, B = 20)

**Constructed RESOURCE CONSTRAINED SCHEDULING instance:**

| Task | Length | R_1 (resource requirement) |
|------|--------|---------------------------|
| t_1  | 1      | 6                         |
| t_2  | 1      | 7                         |
| t_3  | 1      | 7                         |
| t_4  | 1      | 6                         |
| t_5  | 1      | 8                         |
| t_6  | 1      | 6                         |

Processors m_proc = 3, Resource bound B_1 = 20, Deadline D = 2.

**Solution:**
Time slot 0: {t_1, t_2, t_3} — resource usage = 6 + 7 + 7 = 20 <= 20, tasks = 3 <= 3 processors.
Time slot 1: {t_4, t_5, t_6} — resource usage = 6 + 8 + 6 = 20 <= 20, tasks = 3 <= 3 processors.
All tasks finish by time 2 = D.

**Solution extraction:**
Triple 1: {a_1, a_2, a_3} = {6, 7, 7}, sum = 20 = B.
Triple 2: {a_4, a_5, a_6} = {6, 8, 6}, sum = 20 = B. Valid 3-partition.


## References

- **[Garey and Johnson, 1975]**: [`Garey1975`] M. R. Garey and D. S. Johnson (1975). "Complexity results for multiprocessor scheduling under resource constraints". *SIAM Journal on Computing* 4, pp. 397–411.
- **[Ullman, 1976]**: [`Ullman1976`] Jeffrey D. Ullman (1976). "Complexity of sequencing problems". In: *Computer and Job/Shop Scheduling Theory*. John Wiley \& Sons.
