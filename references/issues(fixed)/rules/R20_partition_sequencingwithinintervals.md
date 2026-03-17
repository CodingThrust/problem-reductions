---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Sequencing Within Intervals"
labels: rule
assignees: ''
---

**Source:** Partition
**Target:** Sequencing Within Intervals
**Motivation:** (TBD)
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

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)
