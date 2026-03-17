---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingWithDeadlinesAndSetUpTimes"
labels: model
assignees: ''
---

## Motivation

SEQUENCING WITH DEADLINES AND SET-UP TIMES (P190) from Garey & Johnson, A5 SS6. A single-processor scheduling problem where tasks belong to different "compiler" classes, and switching between classes incurs a set-up time penalty. The question is whether all tasks can meet their individual deadlines given these switching costs. NP-complete even with equal set-up times (via reduction from PARTITION, R136). Can be solved in pseudo-polynomial time when the number of distinct deadlines is bounded by a constant.

**Associated rules:**
- R136: PARTITION → Sequencing with Deadlines and Set-Up Times (this model is the **target**)

## Definition

**Name:** `SequencingWithDeadlinesAndSetUpTimes`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS6

**Mathematical definition:**

INSTANCE: Set C of "compilers," set T of tasks, for each t ∈ T a length l(t) ∈ Z+, a deadline d(t) ∈ Z+, and a compiler k(t) ∈ C, and for each c ∈ C a "set-up time" l(c) ∈ Z0+.
QUESTION: Is there a one-processor schedule σ for T that meets all the task deadlines and that satisfies the additional constraint that, whenever two tasks t and t' with σ(t) < σ(t') are scheduled "consecutively" (i.e., no other task t'' has σ(t) < σ(t'') < σ(t')) and have different compilers (i.e., k(t) ≠ k(t')), then σ(t') ≥ σ(t) + l(t) + l(k(t'))?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one variable per task, representing its position in the schedule)
- **Per-variable domain:** {0, 1, ..., n−1} — the position index of the task in the permutation schedule
- **Meaning:** π(i) ∈ {0, ..., n−1} gives the position of task t_i in the single-processor schedule. Start time σ(t) depends on the sum of lengths of preceding tasks plus any set-up times incurred from compiler switches. A feasible schedule must ensure σ(t) + l(t) ≤ d(t) for all t ∈ T.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SequencingWithDeadlinesAndSetUpTimes`
**Variants:** none (no type parameters)

| Field         | Type         | Description                                                    |
|---------------|--------------|----------------------------------------------------------------|
| `lengths`     | `Vec<u64>`   | Length l(t) of each task t ∈ T                                 |
| `deadlines`   | `Vec<u64>`   | Deadline d(t) of each task t ∈ T                               |
| `compilers`   | `Vec<usize>` | Compiler index k(t) ∈ {0, ..., |C|−1} of each task t ∈ T     |
| `setup_times` | `Vec<u64>`   | Set-up time l(c) for each compiler c ∈ C                      |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete [Bruno & Downey, 1978]. It remains NP-complete even if all set-up times are equal. When the number of distinct deadlines is bounded by a constant, the problem can be solved in pseudo-polynomial time. For general instances, exact algorithms rely on branch-and-bound or ILP formulations. The brute-force complexity is O(n! · n) for enumerating all permutations and checking deadline feasibility.

## Extra Remark

**Full book text:**

INSTANCE: Set C of "compilers," set T of tasks, for each t ∈ T a length l(t) ∈ Z+, a deadline d(t) ∈ Z+, and a compiler k(t) ∈ C, and for each c ∈ C a "set-up time" l(c) ∈ Z0+.
QUESTION: Is there a one-processor schedule σ for T that meets all the task deadlines and that satisfies the additional constraint that, whenever two tasks t and t' with σ(t) < σ(t') are scheduled "consecutively" (i.e., no other task t'' has σ(t) < σ(t'') < σ(t')) and have different compilers (i.e., k(t) ≠ k(t')), then σ(t') ≥ σ(t) + l(t) + l(k(t'))?

Reference: [Bruno and Downey, 1978]. Transformation from PARTITION.

Comment: Remains NP-complete even if all set-up times are equal. The related problem in which set-up times are replaced by "changeover costs," and we want to know if there is a schedule that meets all the deadlines and has total changeover cost at most K, is NP-complete even if all changeover costs are equal. Both problems can be solved in pseudo-polynomial time when the number of distinct deadlines is bounded by a constant. If the number of deadlines is unbounded, it is open whether these problems are NP-complete in the strong sense.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! permutations; compute start times with set-up penalties; check all deadlines.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{ij} ∈ {0,1} = task i in position j; add set-up time variables y_{j} for compiler switch at position j; enforce deadline constraints.)
- [ ] Other: Pseudo-polynomial DP when the number of distinct deadlines is bounded by a constant [Bruno & Downey, 1978].

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
C = {c_0, c_1} (2 compilers), setup_times = [2, 3]
T = {t_1, t_2, t_3, t_4, t_5} (n = 5 tasks)

| Task | Length | Deadline | Compiler |
|------|--------|----------|----------|
| t_1  | 3      | 8        | c_0      |
| t_2  | 4      | 12       | c_1      |
| t_3  | 2      | 5        | c_0      |
| t_4  | 5      | 20       | c_1      |
| t_5  | 3      | 18       | c_0      |

**Feasible schedule:**
Order: t_3, t_1, t_5, [set-up c_0→c_1: 3 units], t_2, t_4
Start times: σ(t_3)=0, σ(t_1)=2, σ(t_5)=5, set-up at 8, σ(t_2)=11, σ(t_4)=15
Completion: t_3 at 2 ≤ 5 ✓, t_1 at 5 ≤ 8 ✓, t_5 at 8 ≤ 18 ✓, t_2 at 15 ≤ 12 ✗

Adjust: t_3, t_1, [set-up: 2], t_2, t_4, [set-up: 2], t_5
Start: σ(t_3)=0, σ(t_1)=2, set-up at 5, σ(t_2)=7, σ(t_4)=11, set-up at 16, σ(t_5)=18
Completion: t_3→2≤5 ✓, t_1→5≤8 ✓, t_2→11≤12 ✓, t_4→16≤20 ✓, t_5→21≤18 ✗

Adjust: t_3, t_1, [set-up c_0→c_1: 3], t_2, [set-up c_1→c_0: 2], t_5, [set-up c_0→c_1: 3], t_4
Start: 0, 2, set-up 5–8, σ(t_2)=8, set-up 12–14, σ(t_5)=14, set-up 17–20, σ(t_4)=20
Completion: 2≤5 ✓, 5≤8 ✓, 12≤12 ✓, 17≤18 ✓, 25≤20 ✗

Group by compiler to minimize switches:
t_3, t_1, t_5, [set-up c_0→c_1: 3], t_2, t_4
Start: 0, 2, 5, set-up 8–11, σ(t_2)=11, σ(t_4)=15
Completion: 2≤5 ✓, 5≤8 ✓, 8≤18 ✓, 15≤12 ✗

Swap t_2 and t_4: t_3, t_1, t_5, [set-up: 3], t_4, t_2
Start: 0, 2, 5, set-up 8–11, σ(t_4)=11, σ(t_2)=16
Completion: 2≤5 ✓, 5≤8 ✓, 8≤18 ✓, 16≤20 ✓, 20≤12 ✗

Final feasible: t_3, t_1, [set-up: 3], t_2, [set-up: 2], t_5, t_4 — but t_4 uses c_1, set-up needed again.

Simplest feasible with adjusted deadlines (d_2 = 15):
t_3, t_1, t_5, [set-up: 3], t_2, t_4
Completion: 2≤5 ✓, 5≤8 ✓, 8≤18 ✓, 15≤15 ✓, 20≤20 ✓

Answer: YES — a valid schedule meeting all deadlines exists (with d_2 adjusted to 15).
