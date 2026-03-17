---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-Partition to Sequencing with Release Times and Deadlines"
labels: rule
assignees: ''
canonical_source_name: '3-PARTITION'
canonical_target_name: 'SEQUENCING WITH RELEASE TIMES AND DEADLINES'
source_in_codebase: false
target_in_codebase: false
milestone: 'Garey & Johnson'
---

**Source:** 3-Partition
**Target:** Sequencing with Release Times and Deadlines
**Motivation:** Establishes that SEQUENCING WITH RELEASE TIMES AND DEADLINES is NP-complete in the strong sense by encoding a 3-PARTITION instance into a scheduling feasibility problem. The reduction creates m time "slots" of width B separated by mandatory filler tasks, so that exactly three element-tasks must fit into each slot -- mirroring the requirement that A be partitioned into m triples each summing to B. Because 3-PARTITION is strongly NP-complete, this also rules out pseudo-polynomial algorithms for the scheduling problem (unless P = NP). This is the canonical reduction cited in Garey & Johnson, Section 4.2.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A5.1, p.236; Section 4.2, pp.96-100

## GJ Source Entry

> [SS1] SEQUENCING WITH RELEASE TIMES AND DEADLINES
> INSTANCE: Set T of tasks and, for each task t E T, a length l(t) E Z+, a release time r(t) E Z_0+, and a deadline d(t) E Z+.
> QUESTION: Is there a one-processor schedule for T that satisfies the release time constraints and meets all the deadlines, i.e., a one-to-one function σ: T → Z_0+, with σ(t) > σ(t') implying σ(t) >= σ(t') + l(t'), such that, for all t E T, σ(t) >= r(t) and σ(t) + l(t) <= d(t)?
> Reference: [Garey and Johnson, 1977b]. Transformation from 3-PARTITION (see Section 4.2).
> Comment: NP-complete in the strong sense. Solvable in pseudo-polynomial time if the number of allowed values for r(t) and d(t) is bounded by a constant, but remains NP-complete (in the ordinary sense) even when each can take on only two values. If all task lengths are 1, or "preemptions" are allowed, or all release times are 0, the general problem can be solved in polynomial time, even under "precedence constraints" [Lawler, 1973], [Lageweg, Lenstra, and Rinnooy Kan, 1976]. Can also be solved in polynomial time even if release times and deadlines are allowed to be arbitrary rationals and there are precedence constraints, so long as all tasks have equal length [Carlier, 1978], [Simons, 1978], [Garey, Johnson, Simons, and Tarjan, 1978], or preemptions are allowed [Blazewicz, 1976].

## Reduction Algorithm

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**
Given a 3-PARTITION instance with set A = {a_1, ..., a_{3m}} of 3m elements, bound B, and sizes s(a_i) with B/4 < s(a_i) < B/2 and sum = mB, construct a SEQUENCING WITH RELEASE TIMES AND DEADLINES instance as follows:

1. **Element tasks:** For each a_i in A (i = 1, ..., 3m), create a task t_i with:
   - Length: l(t_i) = s(a_i)
   - Release time: r(t_i) = 0
   - Deadline: d(t_i) = (m+1)B + m (= total time horizon)

2. **Filler (separator) tasks:** For each slot boundary i = 1, ..., m-1, create a filler task f_i with:
   - Length: l(f_i) = 1
   - Release time: r(f_i) = iB + (i-1) (= the exact start of the i-th gap)
   - Deadline: d(f_i) = iB + i (= the exact end of the i-th gap)

   Each filler task has a window of exactly length 1, so it must be placed at exactly one specific time slot. This creates m "slots" of width B separated by unit-width mandatory gaps.

3. **Time horizon structure:** The total schedule spans time [0, mB + (m-1)]. The filler tasks partition the timeline into m intervals:
   - Slot 1: [0, B)
   - Gap 1: [B, B+1) -- filler f_1
   - Slot 2: [B+1, 2B+1)
   - Gap 2: [2B+1, 2B+2) -- filler f_2
   - ...
   - Slot i: [(i-1)(B+1), (i-1)(B+1)+B)
   - ...
   - Slot m: [(m-1)(B+1), m(B+1)-1)

4. **Correctness:** Since B/4 < s(a_i) < B/2, exactly 3 element tasks must fit in each slot of width B (2 tasks would leave at least B/2 unused, 4 tasks would require more than B). A feasible schedule exists iff A can be partitioned into m triples each summing to B.

5. **Solution extraction:** The element tasks occupying slot i form set A_i. The three elements in each slot sum to exactly B, giving a valid 3-partition.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = 3m = number of elements in A (= `num_elements` of source)
- m = number of groups in the 3-partition (= n/3)
- B = target sum per group

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_tasks`               | `num_elements + num_elements / 3 - 1` (= 3m + m - 1 = 4m - 1) |
| `time_horizon`            | `num_elements / 3 * (bound + 1) - 1` (= m(B+1) - 1) |

**Derivation:** 3m element tasks plus m-1 filler tasks = 4m - 1 total tasks. The time horizon is m slots of width B plus m-1 gaps of width 1 = mB + m - 1. Construction is O(m).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a 3-PARTITION instance, reduce to SEQUENCING WITH RELEASE TIMES AND DEADLINES, solve with BruteForce (try all permutations of tasks checking release time and deadline feasibility), verify the tasks in each slot form valid triples summing to B.
- Check that filler tasks are placed at their unique mandatory positions.
- Check that each slot contains exactly 3 element tasks whose lengths sum to B.
- Edge cases: test with m = 1 (single triple, 3 elements summing to B), infeasible instance (elements that cannot be partitioned into valid triples).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (3-PARTITION):**
m = 2, B = 12, A = {a_1, ..., a_6} with sizes:
- s(a_1) = 4, s(a_2) = 4, s(a_3) = 4, s(a_4) = 5, s(a_5) = 3, s(a_6) = 4
- All sizes satisfy B/4 = 3 < s(a_i) < B/2 = 6
- Total sum = 4 + 4 + 4 + 5 + 3 + 4 = 24 = 2 * 12 = mB

Valid 3-partition: A_1 = {a_1, a_4, a_5} = {4, 5, 3} (sum = 12), A_2 = {a_2, a_3, a_6} = {4, 4, 4} (sum = 12).

**Constructed SEQUENCING WITH RELEASE TIMES AND DEADLINES instance:**

Element tasks:

| Task | Length l | Release r | Deadline d | Notes     |
|------|----------|-----------|------------|-----------|
| t_1  | 4        | 0         | 25         | s(a_1)=4  |
| t_2  | 4        | 0         | 25         | s(a_2)=4  |
| t_3  | 4        | 0         | 25         | s(a_3)=4  |
| t_4  | 5        | 0         | 25         | s(a_4)=5  |
| t_5  | 3        | 0         | 25         | s(a_5)=3  |
| t_6  | 4        | 0         | 25         | s(a_6)=4  |

Filler tasks:

| Task | Length l | Release r | Deadline d | Notes                        |
|------|----------|-----------|------------|------------------------------|
| f_1  | 1        | 12        | 13         | Separates slot 1 and slot 2  |

Time horizon: m(B+1) - 1 = 2(13) - 1 = 25.
- Slot 1: [0, 12) -- width B = 12
- Gap 1: [12, 13) -- filler f_1
- Slot 2: [13, 25) -- width B = 12

**Solution (from partition A_1 = {a_1, a_4, a_5}, A_2 = {a_2, a_3, a_6}):**

Slot 1 (time [0, 12)):
- sigma(t_1) = 0, runs [0, 4) -- length 4
- sigma(t_4) = 4, runs [4, 9) -- length 5
- sigma(t_5) = 9, runs [9, 12) -- length 3
- Total = 4 + 5 + 3 = 12 = B

Gap 1: sigma(f_1) = 12, runs [12, 13)

Slot 2 (time [13, 25)):
- sigma(t_2) = 13, runs [13, 17) -- length 4
- sigma(t_3) = 17, runs [17, 21) -- length 4
- sigma(t_6) = 21, runs [21, 25) -- length 4
- Total = 4 + 4 + 4 = 12 = B

All tasks within [r, r+l] <= d. Feasible schedule.

**Solution extraction:**
Slot 1 tasks: {a_1, a_4, a_5} = {4, 5, 3} (sum = 12 = B)
Slot 2 tasks: {a_2, a_3, a_6} = {4, 4, 4} (sum = 12 = B)
Valid 3-partition.


## References

- **[Garey and Johnson, 1977b]**: [`Garey1977c`] M. R. Garey and D. S. Johnson (1977). "The rectilinear {Steiner} tree problem is {NP}-complete". *SIAM Journal on Applied Mathematics* 32, pp. 826-834.
- **[Lawler, 1973]**: [`Lawler1973`] Eugene L. Lawler (1973). "Optimal sequencing of a single machine subject to precedence constraints". *Management Science* 19, pp. 544-546.
- **[Lageweg, Lenstra, and Rinnooy Kan, 1976]**: [`Lageweg1976`] B. J. Lageweg and Jan K. Lenstra and A. H. G. Rinnooy Kan (1976). "Minimizing maximum lateness on one machine: computational experience and some applications". *Statistica Neerlandica* 30, pp. 25-41.
- **[Carlier, 1978]**: [`Carlier1978`] J. Carlier (1978). "Probl{\`e}me a une machine". Universit{\'e} de Pierre et Marie Curie.
- **[Simons, 1978]**: [`Simons1978`] Barbara Simons (1978). "A fast algorithm for single processor scheduling". In: *Proc. 19th Ann. Symp. on Foundations of Computer Science*, pp. 246-252. IEEE Computer Society.
- **[Garey, Johnson, Simons, and Tarjan, 1978]**: [`Garey1978d`] M. R. Garey and D. S. Johnson and B. B. Simons and R. E. Tarjan (1978). "Scheduling unit time tasks with arbitrary release times and deadlines".
- **[Blazewicz, 1976]**: [`Blazewicz1976`] J. Blazewicz (1976). "Scheduling dependent tasks with different arrival times to meet deadlines". In: *Modelling and Performance Evaluation of Computer Systems*. North Holland.
