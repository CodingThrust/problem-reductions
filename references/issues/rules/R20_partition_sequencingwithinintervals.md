---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Sequencing Within Intervals"
labels: rule
assignees: ''
canonical_source_name: 'PARTITION'
canonical_target_name: 'SEQUENCING WITHIN INTERVALS'
source_in_codebase: false
target_in_codebase: false
---

**Source:** Partition
**Target:** Sequencing Within Intervals
**Motivation:** The PARTITION problem asks whether n positive integers can be split into two equal-sum halves. SEQUENCING WITHIN INTERVALS asks whether n unit-processor tasks, each with a release time and a deadline, can be non-overlappingly scheduled within their windows. By inserting a single "enforcer" task whose release time equals its deadline minus one, placed exactly at the midpoint of the available time horizon, the reduction forces the schedule to be split into two independent blocks. Each block must be filled exactly, which is equivalent to finding a balanced partition. This gadget-based proof appears as Theorem 3.8 in Garey & Johnson and was one of the earliest demonstrations that single-machine preemptive scheduling can be NP-complete.
<!-- ⚠️ Unverified: AI-generated motivation -->
**Reference:** Garey & Johnson, *Computers and Intractability*, Theorem 3.8, p.70

## Reduction Algorithm

> SEQUENCING WITHIN INTERVALS
> INSTANCE: A finite set T of "tasks" and, for each t ∈ T, an integer "release time" r(t) ≥ 0, a "deadline" d(t) ∈ Z+, and a "length" l(t) ∈ Z+.
> QUESTION: Does there exist a feasible schedule for T, that is, a function σ: T → Z+ such that, for each t ∈ T, σ(t) ≥ r(t), σ(t)+l(t) ≤ d(t), and, if t' ∈ T−{t}, then either σ(t')+l(t') ≤ σ(t) or σ(t') ≥ σ(t)+l(t)? (The task t is "executed" from time σ(t) to time σ(t)+l(t), cannot start executing until time r(t), must be completed by time d(t), and its execution cannot overlap the execution of any other task t'.)
>
> Theorem 3.8 SEQUENCING WITHIN INTERVALS is NP-complete.
> Proof: We transform PARTITION to this problem. Let the finite set A and given size s(a) for each a ∈ A constitute an arbitrary instance of PARTITION, and let B = ∑_{a ∈ A} s(a).
>
> The basic units of the PARTITION instance are the individual elements a ∈ A. The local replacement for each a ∈ A is a single task t_a with r(t_a) = 0, d(t_a) = B+1, and l(t_a) = s(a). The "enforcer" is a single task t̄ with r(t̄) = [B/2], d(t̄) = [(B+1)/2], and l(t̄) = 1. Clearly, this instance can be constructed in polynomial time from the PARTITION instance.
>
> The restrictions imposed on feasible schedules by the enforcer are two-fold. First, it ensures that a feasible schedule cannot be constructed whenever B is an odd integer (in which case the desired subset for the PARTITION instance cannot exist), because then we would have r(t̄) = d(t̄), so that t̄ could not possibly be scheduled. Thus from now on, let us assume that B is even. In this case the second restriction comes to the forefront. Since B is even, r(t̄) = B/2 and d(t̄) = r(t̄) + 1, so that any feasible schedule must have σ(t̄) = B/2. This divides the time available for scheduling the remaining tasks into two separate blocks, each of total length B/2, as illustrated in Figure 3.9. Thus the scheduling problem is turned into a problem of selecting subsets, those that are scheduled before t̄ and those that are scheduled after t̄. Since the total amount of time available in the two blocks equals the total length B of the remaining tasks, it follows that each block must be filled up exactly. However, this can be done if and only if there is a subset A' ⊆ A such that
>
>     ∑_{a ∈ A'} s(a) = B/2 = ∑_{a ∈ A−A'} s(a)
>
> Thus the desired subset A' exists for the instance of PARTITION if and only if a feasible schedule exists for the corresponding instance of SEQUENCING WITHIN INTERVALS. ∎

<!-- ⚠️ Unverified: AI-generated summary below -->

**Summary:**

Let A = {a_1, ..., a_n} with s(a_i) ∈ Z⁺ be an arbitrary PARTITION instance, and let B = Σ_{i=1}^{n} s(a_i).

1. **Regular tasks:** For each a_i ∈ A, create task t_i with:
   - Release time: r(t_i) = 0
   - Deadline: d(t_i) = B + 1
   - Length: l(t_i) = s(a_i)
2. **Enforcer task:** Create one additional task t̄ with:
   - Release time: r(t̄) = ⌊B/2⌋
   - Deadline: d(t̄) = ⌈(B+1)/2⌉
   - Length: l(t̄) = 1
   - (When B is even: r(t̄) = B/2 and d(t̄) = B/2 + 1, so t̄ must start at exactly time B/2.)
   - (When B is odd: r(t̄) = d(t̄) = ⌈B/2⌉, so t̄ cannot be scheduled and the instance is infeasible, correctly reflecting that no balanced partition exists.)
3. **Correctness:** The enforcer t̄ is pinned at time slot [B/2, B/2 + 1], splitting the horizon into two blocks [0, B/2) and (B/2 + 1, B + 1). The regular tasks must fill both blocks exactly, which is equivalent to finding a balanced partition of A.
4. **Solution extraction:** Tasks scheduled in [0, B/2) form one partition A', and tasks scheduled after t̄ form A \ A'.

## Size Overhead

<!-- ⚠️ Unverified: AI-derived overhead expressions -->

**Symbols:**
- n = |A| = number of elements in PARTITION instance (`num_tasks` of source)
- B = Σ s(a_i) = total element sum

| Target metric (code name) | Polynomial (using symbols above) |
|---------------------------|----------------------------------|
| `num_tasks`               | `num_tasks + 1` (= n + 1, including the enforcer t̄) |
| `max_deadline`            | `total_sum + 1` (= B + 1)                            |

**Derivation:** n regular tasks (one per element) plus 1 enforcer task = n + 1 tasks total. All regular tasks have release time 0 and deadline B + 1. The enforcer's deadline is B/2 + 1 ≤ B + 1. Construction is O(n).

## Validation Method

<!-- ⚠️ Unverified: AI-suggested validation -->

- Closed-loop test: construct a PARTITION instance, reduce to SEQUENCING WITHIN INTERVALS, solve with BruteForce (enumerate all permutations of non-overlapping start times), verify the schedule's pre-enforcer tasks form a balanced partition.
- Check that t̄ is scheduled at exactly time B/2 (the only feasible slot), and that tasks before and after t̄ each total B/2 in length.
- Edge cases: odd total sum (expect infeasible, since r(t̄) = d(t̄)), all equal elements (multiple valid partitions), n = 2 with s(a_1) = s(a_2) (trivially feasible).

## Example

<!-- ⚠️ Unverified: AI-constructed example -->

**Source instance (PARTITION):**
A = {3, 1, 2, 4} (n = 4 elements)
Total sum B = 3 + 1 + 2 + 4 = 10 (even)
A balanced partition exists: A' = {3, 2} (sum = 5) and A \ A' = {1, 4} (sum = 5).

**Constructed SEQUENCING WITHIN INTERVALS instance:**

| Task | Release r | Deadline d | Length l | Notes                         |
|------|-----------|------------|----------|-------------------------------|
| t_1  | 0         | 11         | 3        | a_1 = 3                       |
| t_2  | 0         | 11         | 1        | a_2 = 1                       |
| t_3  | 0         | 11         | 2        | a_3 = 2                       |
| t_4  | 0         | 11         | 4        | a_4 = 4                       |
| t̄   | 5         | 6          | 1        | enforcer; B/2 = 5, d = B/2+1  |

**Solution:**
σ(t̄) = 5 (forced by r = d - 1 = 5): t̄ runs during [5, 6).

Block 1: time [0, 5) — place t_1 and t_3 (lengths 3 and 2):
- σ(t_1) = 0, runs [0, 3)
- σ(t_3) = 3, runs [3, 5)
- Total length = 5 = B/2 ✓

Block 2: time [6, 11) — place t_2 and t_4 (lengths 1 and 4):
- σ(t_2) = 6, runs [6, 7)
- σ(t_4) = 7, runs [7, 11)
- Total length = 5 = B/2 ✓

No task overlaps, and all tasks are within their [r, r+l] ≤ d windows. Feasible schedule ✓

**Solution extraction:**
Tasks in Block 1: {a_1, a_3} = {3, 2} → A' (sum = 5)
Tasks in Block 2: {a_2, a_4} = {1, 4} → A \ A' (sum = 5)
Balanced partition ✓
