---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeWeightedTardiness"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE WEIGHTED TARDINESS (P189) from Garey & Johnson, A5 SS5. A classical NP-complete scheduling problem: given a set of tasks with lengths, weights, and deadlines on a single processor, minimize the total weighted tardiness. It is NP-complete in the strong sense (via reduction from 3-PARTITION, R135), ruling out pseudo-polynomial time algorithms. Special cases are tractable: equal weights admit a pseudo-polynomial algorithm [Lawler, 1977a], and equal lengths can be solved in polynomial time by bipartite matching.

**Associated rules:**
- R135: 3-PARTITION → Sequencing to Minimize Weighted Tardiness (this model is the **target**)

## Definition

**Name:** `SequencingToMinimizeWeightedTardiness`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS5

**Mathematical definition:**

INSTANCE: Set T of tasks, for each task t ∈ T a length l(t) ∈ Z+, a weight w(t) ∈ Z+, and a deadline d(t) ∈ Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule σ for T such that the sum, taken over all t ∈ T satisfying σ(t) + l(t) > d(t), of (σ(t) + l(t) - d(t))·w(t) is K or less?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one variable per task, representing position in the schedule)
- **Per-variable domain:** {0, 1, ..., n−1} — the position index of the task in the permutation schedule
- **Meaning:** π(i) ∈ {0, ..., n−1} gives the position of task t_i in the single-processor schedule. The schedule is a permutation of tasks. Start time σ(t) is determined by the sum of lengths of all tasks scheduled before t. The objective is to find a permutation minimizing total weighted tardiness, subject to the bound K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SequencingToMinimizeWeightedTardiness`
**Variants:** none (no type parameters; all values are positive integers)

| Field      | Type       | Description                                                 |
|------------|------------|-------------------------------------------------------------|
| `lengths`  | `Vec<u64>` | Length l(t) of each task t ∈ T                              |
| `weights`  | `Vec<u64>` | Weight w(t) of each task t ∈ T                              |
| `deadlines`| `Vec<u64>` | Deadline d(t) of each task t ∈ T                            |
| `bound`    | `u64`      | Upper bound K on total weighted tardiness                   |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem 1||Σw_jT_j is NP-hard in the strong sense [Lawler, 1977a; Garey & Johnson, 1979]. For the decision version, no pseudo-polynomial time algorithm exists unless P = NP. Branch-and-bound algorithms can solve instances with up to 40–50 jobs [Potts & Van Wassenhove, 1985]. More recent exact approaches based on successive sublimation dynamic programming handle up to 100–500 jobs in practice [Tanaka et al., 2009]. The worst-case brute-force complexity is O(n! · n) for enumerating all permutations.

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, for each task t ∈ T a length l(t) ∈ Z+, a weight w(t) ∈ Z+, and a deadline d(t) ∈ Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule σ for T such that the sum, taken over all t ∈ T satisfying σ(t) + l(t) > d(t), of (σ(t) + l(t) - d(t))·w(t) is K or less?

Reference: [Lawler, 1977a]. Transformation from 3-PARTITION.

Comment: NP-complete in the strong sense. If all weights are equal, the problem can be solved in pseudo-polynomial time [Lawler, 1977a] and is open as to ordinary NP-completeness. If all lengths are equal (with weights arbitrary), it can be solved in polynomial time by bipartite matching. If precedence constraints are added, the problem is NP-complete even with equal lengths and equal weights [Lenstra and Rinnooy Kan, 1978a]. If release times are added instead, the problem is NP-complete in the strong sense for equal task weights (see SEQUENCING WITH RELEASE TIMES AND DEADLINES), but can be solved by bipartite matching for equal lengths and arbitrary weights [Graham, Lawler, Lenstra, and Rinnooy Kan, 1978].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! permutations of tasks; compute total weighted tardiness for each; check if any has total ≤ K.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{ij} ∈ {0,1} = task i in position j; enforce permutation constraints; compute start times and tardiness as linear functions of x; minimize total weighted tardiness subject to ≤ K.)
- [ ] Other: Branch-and-bound [Potts & Van Wassenhove, 1985], SSDP [Tanaka et al., 2009].

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5} (n = 5 tasks)
Lengths: l = [3, 4, 2, 5, 3]
Weights: w = [2, 3, 1, 4, 2]
Deadlines: d = [5, 8, 4, 15, 10]
Bound K = 10.

**Schedule (one feasible permutation):**
Order: t_3, t_1, t_2, t_5, t_4
Start times: σ(t_3)=0, σ(t_1)=2, σ(t_2)=5, σ(t_5)=9, σ(t_4)=12
Completion times: 2, 5, 9, 12, 17
Tardiness: max(0, 2−4)=0, max(0, 5−5)=0, max(0, 9−8)=1, max(0, 12−10)=2, max(0, 17−15)=2
Weighted tardiness: 0·1 + 0·2 + 1·3 + 2·2 + 2·4 = 0 + 0 + 3 + 4 + 8 = 15 > K.

Try: t_3, t_1, t_5, t_2, t_4
Start: 0, 2, 5, 8, 12. Completion: 2, 5, 8, 12, 17.
Tardiness: max(0,2−4)=0, max(0,5−5)=0, max(0,8−10)=0, max(0,12−8)=4, max(0,17−15)=2.
WT: 0·1 + 0·2 + 0·2 + 4·3 + 2·4 = 12 + 8 = 20 > K.

Try: t_3, t_1, t_2, t_4, t_5
Start: 0, 2, 5, 9, 14. Completion: 2, 5, 9, 14, 17.
Tardiness: 0, 0, 1, 0, 7. WT: 0 + 0 + 3 + 0 + 14 = 17 > K.

Try: t_1, t_3, t_5, t_4, t_2
Start: 0, 3, 5, 8, 13. Completion: 3, 5, 8, 13, 17.
Tardiness: 0, 1, 0, 0, 9. WT: 0 + 1 + 0 + 0 + 27 = 28 > K.

Try: t_3, t_1, t_5, t_4, t_2
Start: 0, 2, 5, 8, 13. Completion: 2, 5, 8, 13, 17.
Tardiness: 0, 0, 0, 0, 9. WT: 0 + 0 + 0 + 0 + 27 = 27 > K.

Adjusting K = 15 for feasibility with the first schedule:
Answer: YES with K = 15, schedule t_3, t_1, t_2, t_5, t_4 achieves total weighted tardiness = 15 ≤ K ✓.
